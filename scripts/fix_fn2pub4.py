# fix_e0624_assoc_debug.py
# Fix Rust E0624 private methods / associated functions with DEBUG output.
# Handles:
#   error[E0624]: method `name` is private
#   error[E0624]: associated function `name` is private
# ... and uses the "private ... defined here" location to patch the defining file.
#
# Usage:
#   cargo build 2> bug01.txt
#   python fix_e0624_assoc_debug.py bug01.txt
#
# Run from project root (Cargo.toml folder) so relative paths like src\... resolve.

from __future__ import annotations

import re
import sys
from pathlib import Path

# ================== CONFIG ==================
VIS = "pub"          # change to "pub(super)" or "pub(crate)" if you want
DRY_RUN = False      # True: don't modify files
WINDOW = 25          # search +-WINDOW lines around defined-here line
GLOBAL_FALLBACK = True

# ============================================
MAKE_PREFIX = f"{VIS} "

def dbg(msg: str) -> None:
    print(msg, flush=True)

def read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8", errors="replace")

def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8", newline="")

def backup_once(p: Path) -> None:
    bak = p.with_suffix(p.suffix + ".bak")
    if not bak.exists():
        bak.write_text(read_text(p), encoding="utf-8", newline="")

# ---- Parse error lines ----
# Supports both:
#   error[E0624]: method `foo` is private
#   error[E0624]: associated function `foo` is private
RE_E0624_NAME = re.compile(
    r"^error\[E0624\]:\s+(?:method|associated function)\s+`([^`]+)`\s+is private"
)

# "defined here" marker line (cargo uses :::)
RE_DEFINED_HERE = re.compile(r"^\s*:::\s+(.+)\s*$")

def parse_path_line_col(raw: str):
    """
    Parse 'src\\ws\\file.rs:201:5' or 'R:\\...\\file.rs:201:5'
    robustly by splitting from the right.
    """
    s = raw.strip().lstrip("|").strip()
    parts = s.rsplit(":", 2)
    if len(parts) != 3:
        return None
    p, l, c = parts
    try:
        return Path(p), int(l), int(c), s
    except ValueError:
        return None

def resolve_path(p: Path) -> Path | None:
    """
    Resolve relative paths from CWD, normalize slashes.
    """
    # as-is
    if p.exists():
        return p

    # normalize slashes
    p_norm = Path(str(p).replace("/", "\\"))
    if p_norm.exists():
        return p_norm

    # try CWD / relative
    if not p.is_absolute():
        cand = Path.cwd() / p
        if cand.exists():
            return cand
        cand2 = Path(str(cand).replace("/", "\\"))
        if cand2.exists():
            return cand2

    return None

def already_public(line: str, name: str) -> bool:
    return re.match(
        rf"^\s*pub(\s*\([^)]+\))?\s+(?:async\s+)?(?:unsafe\s+)?fn\s+{re.escape(name)}\s*\(",
        line,
    ) is not None

def fn_line_matcher(name: str) -> re.Pattern:
    # indentation group(1), optional pub group(2..), optional async/unsafe
    return re.compile(
        rf"^(\s*)(?:pub(\s*\([^)]+\))?\s+)?(?:(async)\s+)?(?:(unsafe)\s+)?fn\s+{re.escape(name)}\s*\("
    )

def rewrite_to_pub(line: str, name: str) -> str | None:
    """
    If this line is a fn definition for `name` and not already public,
    rewrite to start with 'pub ' (or configured VIS), preserving async/unsafe.
    """
    pat = fn_line_matcher(name)
    m = pat.match(line)
    if not m:
        return None
    if already_public(line, name):
        return None

    indent = m.group(1)
    has_async = m.group(3) is not None
    has_unsafe = m.group(4) is not None

    mid = ""
    if has_async:
        mid += "async "
    if has_unsafe:
        mid += "unsafe "

    pos = line.find(name)
    if pos < 0:
        return None
    remainder = line[pos:]  # keep "name(...", generics, etc.
    return f"{indent}{MAKE_PREFIX}{mid}fn {remainder}"

def patch_one(file_path: Path, defined_line: int, name: str) -> bool:
    """
    Try patch around defined_line; fallback global search.
    Prints FOUND/NOT FOUND debug.
    """
    text = read_text(file_path)
    lines = text.splitlines()
    idx = defined_line - 1

    dbg(f"  [CHECK] file={file_path} lines={len(lines)} target_line={defined_line} name={name}")

    start = max(0, idx - WINDOW)
    end = min(len(lines), idx + WINDOW + 1)

    # window search
    for i in range(start, end):
        if fn_line_matcher(name).match(lines[i]):
            if already_public(lines[i], name):
                dbg(f"  [FOUND] line {i+1} but already public:")
                dbg(f"          {lines[i]}")
                return False

            new_line = rewrite_to_pub(lines[i], name)
            if new_line is not None:
                dbg(f"  [FOUND] line {i+1} in window:")
                dbg(f"          OLD: {lines[i]}")
                dbg(f"          NEW: {new_line}")
                if DRY_RUN:
                    dbg("  [DRYRUN] not writing.")
                    return True
                backup_once(file_path)
                lines[i] = new_line
                write_text(file_path, "\n".join(lines) + "\n")
                dbg("  [WRITE] patched.")
                return True

    dbg("  [NOT FOUND] in window search.")

    # global fallback
    if GLOBAL_FALLBACK:
        dbg("  [FALLBACK] global search...")
        for i in range(len(lines)):
            if fn_line_matcher(name).match(lines[i]):
                if already_public(lines[i], name):
                    dbg(f"  [FOUND] global line {i+1} but already public:")
                    dbg(f"          {lines[i]}")
                    return False

                new_line = rewrite_to_pub(lines[i], name)
                if new_line is not None:
                    dbg(f"  [FOUND] global line {i+1}:")
                    dbg(f"          OLD: {lines[i]}")
                    dbg(f"          NEW: {new_line}")
                    if DRY_RUN:
                        dbg("  [DRYRUN] not writing.")
                        return True
                    backup_once(file_path)
                    lines[i] = new_line
                    write_text(file_path, "\n".join(lines) + "\n")
                    dbg("  [WRITE] patched.")
                    return True

        dbg("  [NOT FOUND] global search failed too.")

    return False

def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: python fix_e0624_assoc_debug.py bug01.txt")
        return 2

    log_path = Path(sys.argv[1])
    if not log_path.exists():
        print(f"[ERROR] log not found: {log_path}")
        return 2

    dbg(f"[INFO] Start parse: {log_path}")
    log_lines = read_text(log_path).splitlines()

    targets = []  # list of (name, def_path, def_line, def_col, raw)
    current_name = None

    for line in log_lines:
        m = RE_E0624_NAME.match(line)
        if m:
            current_name = m.group(1)
            dbg(f"[PARSE] E0624 name='{current_name}' from: {line.strip()}")
            continue

        if current_name is not None:
            m2 = RE_DEFINED_HERE.match(line)
            if m2:
                parsed = parse_path_line_col(m2.group(1))
                if parsed is None:
                    dbg(f"[WARN] cannot parse defined-here line: {line}")
                    current_name = None
                    continue
                p, lno, cno, raw = parsed
                dbg(f"[PARSE] defined-here raw='{raw}' path='{p}' line={lno} col={cno}")
                targets.append((current_name, p, lno, cno, raw))
                current_name = None

    dbg(f"[INFO] Parsed targets: {len(targets)}")
    if not targets:
        dbg("[INFO] No targets found. (Did you redirect stderr: cargo build 2> bug01.txt ?)")
        return 0

    seen = set()
    changed = 0
    idx = 0

    for name, p, lno, cno, raw in targets:
        idx += 1
        dbg("\n============================================================")
        dbg(f"[TARGET {idx}] name='{name}' defined_at='{raw}'")
        rp = resolve_path(p)
        if rp is None:
            dbg(f"[FILE] NOT FOUND: '{p}' (CWD={Path.cwd()})")
            continue
        dbg(f"[FILE] FOUND: {rp}")

        key = (str(rp).lower(), lno, name)
        if key in seen:
            dbg("[SKIP] duplicate target")
            continue
        seen.add(key)

        ok = patch_one(rp, lno, name)
        if ok:
            changed += 1

    dbg("\n============================================================")
    dbg(f"[DONE] attempted={len(seen)} changed={changed} vis='{VIS}' dry_run={DRY_RUN}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
