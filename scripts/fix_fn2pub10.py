# fix_splitrs.py
# Usage:
#   python fix_splitrs.py bug01.txt --root .            (dry run, only debug)
#   python fix_splitrs.py bug01.txt --root . --apply    (apply changes)
#
# What it fixes (best-effort):
# - E0624: private method / private associated function -> make it pub(crate)
# - "help: consider importing ..." -> auto-insert use crate::ws::Xxx;
# - egui copied_text removed -> replace ui.output_mut(|o| o.copied_text = X) with ui.ctx().copy_text(X)
#
# Notes:
# - This is intentionally conservative. It only edits the definition file shown by rustc for E0624.
# - For imports, it only inserts the exact suggestion rustc printed.

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Optional, List, Tuple


@dataclass
class Edit:
    path: Path
    kind: str
    detail: str


E0624_HEAD = re.compile(
    r"error\[E0624\]: (method|associated function) `(?P<name>[^`]+)` is private"
)
# Example:
#   ::: src\ws\processmanagerapp_restart_as_admin_group.rs:44:5
E0624_DEF = re.compile(r":::\s+(?P<file>.*\.rs):(?P<line>\d+):(?P<col>\d+)")

# Example:
# help: consider importing this enum through its public re-export
#   |
# 7 + use crate::ws::FilepaneCommand;
IMPORT_SUGG = re.compile(r"^\s*\d+\s*\+\s*use\s+(?P<use_stmt>crate::ws::[A-Za-z0-9_]+)\s*;\s*$")

# Example:
#   --> src\ws\functions.rs:32:35
ERR_AT = re.compile(r"^\s*-->\s+(?P<file>.*\.rs):(?P<line>\d+):(?P<col>\d+)\s*$")

# copied_text patterns (a few common shapes)
COPIED_TEXT_RE = re.compile(
    r"""ui\.output_mut\(\s*\|o\|\s*o\.copied_text\s*=\s*(?P<expr>[^;]+?)\s*\);\s*""",
    re.MULTILINE,
)

# function signature line: "fn name(" possibly with spaces/indent
def make_fn_sig_re(name: str) -> re.Pattern:
    # Keep it simple: start of line, optional indent, "fn <name>"
    return re.compile(rf"^(?P<indent>\s*)fn\s+{re.escape(name)}\b", re.MULTILINE)


def read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8", errors="replace")


def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8", newline="\n")


def debug(msg: str) -> None:
    print(msg, flush=True)


def patch_make_pub_crate(fn_file: Path, fn_name: str, apply: bool) -> Tuple[bool, int]:
    """
    Replace the first occurrence of:
        <indent>fn <name>
    with:
        <indent>pub(crate) fn <name>
    """
    if not fn_file.exists():
        debug(f"[E0624] DEF FILE NOT FOUND: {fn_file}")
        return False, 0

    src = read_text(fn_file)
    pat = make_fn_sig_re(fn_name)
    m = pat.search(src)
    if not m:
        debug(f"[E0624] NOT FOUND signature in {fn_file} for fn `{fn_name}`")
        return False, 0

    indent = m.group("indent")
    repl = f"{indent}pub(crate) fn {fn_name}"
    new_src, n = pat.subn(repl, src, count=1)

    debug(f"[E0624] FOUND {fn_file}: make `{fn_name}` pub(crate) (replacements={n})")
    if apply and n > 0:
        write_text(fn_file, new_src)
    return True, n


def insert_use_stmt(target_file: Path, use_stmt: str, apply: bool) -> bool:
    """
    Insert `use <use_stmt>;` near the top, after existing `use ...;` block if present,
    otherwise after module docs / attributes.
    """
    if not target_file.exists():
        debug(f"[IMPORT] TARGET FILE NOT FOUND: {target_file}")
        return False

    src = read_text(target_file)

    line_to_add = f"use {use_stmt};"
    if line_to_add in src:
        debug(f"[IMPORT] already present in {target_file}: {line_to_add}")
        return True

    lines = src.splitlines()

    # Find insertion point:
    # Skip leading shebang (rare), inner attrs (#![...]), outer attrs #[...], and doc comments //! ...
    i = 0
    while i < len(lines):
        l = lines[i].strip()
        if l.startswith("//!") or l.startswith("#![") or l.startswith("#[") or l == "":
            i += 1
            continue
        break

    # Also, if there is a block of `use ...;` at the top, insert after it.
    j = i
    while j < len(lines):
        l = lines[j].strip()
        if l.startswith("use ") and l.endswith(";"):
            j += 1
            continue
        break

    insert_at = j if j > i else i
    debug(f"[IMPORT] inserting into {target_file} at line {insert_at+1}: {line_to_add}")

    new_lines = lines[:insert_at] + [line_to_add] + lines[insert_at:]
    new_src = "\n".join(new_lines) + ("\n" if src.endswith("\n") else "")

    if apply:
        write_text(target_file, new_src)
    return True


def patch_copied_text(root: Path, apply: bool) -> List[Edit]:
    edits: List[Edit] = []
    for rs in root.rglob("*.rs"):
        src = read_text(rs)
        if "copied_text" not in src:
            continue

        def _repl(m: re.Match) -> str:
            expr = m.group("expr").strip()
            return f"ui.ctx().copy_text({expr});"

        new_src, n = COPIED_TEXT_RE.subn(_repl, src)
        if n > 0:
            debug(f"[copied_text] {rs}: replaced {n} occurrence(s)")
            edits.append(Edit(rs, "copied_text", f"replaced {n}"))
            if apply:
                write_text(rs, new_src)
        else:
            debug(f"[copied_text] {rs}: contains copied_text but pattern not matched (manual check)")
    return edits


def parse_bug_file(bug_path: Path) -> Tuple[List[Tuple[str, Path]], List[Tuple[Path, str]]]:
    """
    Returns:
      - e0624_items: list of (fn_name, def_file)
      - import_items: list of (target_file, use_stmt)
    """
    text = read_text(bug_path).splitlines()

    e0624_items: List[Tuple[str, Path]] = []
    import_items: List[Tuple[Path, str]] = []

    cur_err_file: Optional[Path] = None

    i = 0
    while i < len(text):
        line = text[i]

        # Track current error file from "--> path:line:col"
        m_at = ERR_AT.match(line)
        if m_at:
            cur_err_file = Path(m_at.group("file"))
            debug(f"[parse] error location file = {cur_err_file}")
            i += 1
            continue

        # E0624 header
        m = E0624_HEAD.search(line)
        if m:
            fn_name = m.group("name")
            debug(f"[parse] E0624 found: fn=`{fn_name}`")
            # Search forward for the definition "::: file:line:col"
            j = i + 1
            def_file = None
            while j < min(i + 40, len(text)):
                md = E0624_DEF.search(text[j])
                if md:
                    def_file = Path(md.group("file"))
                    break
                j += 1
            if def_file is not None:
                debug(f"[parse] E0624 def file = {def_file}")
                e0624_items.append((fn_name, def_file))
            else:
                debug(f"[parse] E0624 def file NOT FOUND for fn=`{fn_name}` (need manual)")
            i += 1
            continue

        # Import suggestion
        ms = IMPORT_SUGG.match(line)
        if ms and cur_err_file is not None:
            use_stmt = ms.group("use_stmt")
            debug(f"[parse] IMPORT suggestion: {use_stmt} into {cur_err_file}")
            import_items.append((cur_err_file, use_stmt))
            i += 1
            continue

        i += 1

    return e0624_items, import_items


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("bugfile", type=str, help="bugxx.txt (stderr from cargo build)")
    ap.add_argument("--root", type=str, default=".", help="crate root (folder containing src/)")
    ap.add_argument("--apply", action="store_true", help="apply changes (otherwise dry run)")
    args = ap.parse_args()

    bugfile = Path(args.bugfile)
    root = Path(args.root)
    apply = args.apply

    debug(f"== fix_splitrs ==")
    debug(f"bugfile = {bugfile}")
    debug(f"root    = {root.resolve()}")
    debug(f"apply   = {apply}")
    debug("")

    if not bugfile.exists():
        debug(f"BUGFILE NOT FOUND: {bugfile}")
        return

    # 1) Parse bug file
    e0624_items, import_items = parse_bug_file(bugfile)
    debug("")
    debug(f"[summary] E0624 items   = {len(e0624_items)}")
    debug(f"[summary] import items = {len(import_items)}")
    debug("")

    # 2) Apply E0624 patches
    for fn_name, def_file in e0624_items:
        # def_file is often relative; resolve against root if needed
        df = def_file
        if not df.is_absolute():
            df = root / df
        patch_make_pub_crate(df, fn_name, apply)

    # 3) Apply import insertions
    for target_file, use_stmt in import_items:
        tf = target_file
        if not tf.is_absolute():
            tf = root / tf
        insert_use_stmt(tf, use_stmt, apply)

    # 4) Fix copied_text across project
    debug("")
    patch_copied_text(root, apply)

    debug("")
    debug("== done ==")
    if not apply:
        debug("Dry-run only. Re-run with --apply to write changes.")


if __name__ == "__main__":
    main()

