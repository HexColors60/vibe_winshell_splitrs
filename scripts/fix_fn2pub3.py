# fix_priv_methods_debug.py
# Windows 10 friendly + DEBUG output for Rust E0624 private methods.
#
# Usage:
#   cargo build 2> bug01.txt
#   python fix_priv_methods_debug.py bug01.txt
#
# What it does:
# - Parses rustc output for blocks:
#     error[E0624]: method `NAME` is private
#     ...
#       ::: PATH:LINE:COL
#       LINE |     fn NAME(...)
# - For each target, it tries to locate the file (relative to CWD and absolute),
#   then searches around LINE (and globally fallback) for a function definition:
#     fn NAME(
#     async fn NAME(
#     unsafe fn NAME(
#     pub ... fn NAME(  (already public)
# - If found and not already public, it changes it to:
#     pub fn NAME(
#   (or pub(super)/pub(crate) if configured)
#
# Creates .bak backups (once) for modified files.
#
# Notes:
# - Run from project root (where Cargo.toml is) for relative paths to resolve.

from __future__ import annotations

import re
import sys
from pathlib import Path
from collections import defaultdict

# ===== CONFIG =====
VIS = "pub"          # change to "pub(super)" or "pub(crate)" if you want
DRY_RUN = False      # True = don't write files, only show what would change
WINDOW = 20          # search +- WINDOW lines around the reported line
GLOBAL_FALLBACK = True

MAKE_PREFIX = f"{VIS} "

# ===== REGEX =====
RE_E0624 = re.compile(r"^error\[E0624\]: method `([^`]+)` is private")
RE_DEFINED_HERE_LINE = re.compile(r"^\s*:::\s+(.+)\s*$")

def debug(msg: str) -> None:
    print(msg)

def read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8", errors="replace")

def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8", newline="")

def make_backup_once(p: Path) -> None:
    bak = p.with_suffix(p.suffix + ".bak")
    if not bak.exists():
        bak.write_text(read_text(p), encoding="utf-8", newline="")

def parse_path_line_col(s: str):
    """
    Robust parse for:
      src\\ws\\file.rs:147:5
      R:\\tmp\\proj\\src\\ws\\file.rs:147:5
    Split from right by ':' to avoid drive-letter issues.
    """
    raw = s.strip()
    raw = raw.lstrip("|").strip()

    parts = raw.rsplit(":", 2)
    if len(parts) != 3:
        return None

    path_str, line_str, col_str = parts
    try:
        line_no = int(line_str)
        col_no = int(col_str)
    except ValueError:
        return None

    return Path(path_str), line_no, col_no, raw

def resolve_file_path(p: Path) -> Path | None:
    """
    Try to resolve file path in a Windows-friendly way.
    1) If p exists as-is, use it.
    2) If it's relative, try CWD / p
    3) Try normalizing slashes
    """
    # as-is
    if p.exists():
        return p

    # try relative to CWD
    if not p.is_absolute():
        cand = (Path.cwd() / p)
        if cand.exists():
            return cand

        # Sometimes log contains forward slashes; normalize
        cand2 = Path(str(cand).replace("/", "\\"))
        if cand2.exists():
            return cand2

    # If absolute but slashes weird
    cand3 = Path(str(p).replace("/", "\\"))
    if cand3.exists():
        return cand3

    return None

def is_already_public(line: str, func_name: str) -> bool:
    return re.match(rf"^\s*pub(\s*\([^)]+\))?\s+(async\s+)?(unsafe\s+)?fn\s+{re.escape(func_name)}\s*\(",
                    line) is not None

def build_fn_patterns(func_name: str):
    """
    Match function definition lines (start of signature).
    We accept:
      fn name(
      async fn name(
      unsafe fn name(
      async unsafe fn name(
      unsafe async fn name(
    with optional visibility already present.
    """
    # capture indentation in group 1
    base = rf"^(\s*)(?:pub(\s*\([^)]+\))?\s+)?(?:(async)\s+)?(?:(unsafe)\s+)?fn\s+{re.escape(func_name)}\s*\("
    return re.compile(base)

def patch_line_to_pub(line: str, func_name: str) -> str | None:
    """
    If line defines `func_name` and isn't already public, rewrite leading portion to:
      <indent>pub [async ] [unsafe ] fn func_name(
    preserving async/unsafe order as found.
    """
    pat = build_fn_patterns(func_name)
    m = pat.match(line)
    if not m:
        return None

    if is_already_public(line, func_name):
        return None  # no change needed

    indent = m.group(1)
    has_async = m.group(3) is not None
    has_unsafe = m.group(4) is not None

    mid = ""
    if has_async:
        mid += "async "
    if has_unsafe:
        mid += "unsafe "

    # Now we need to replace everything up to "fn <name>(" with our desired prefix.
    # Simplest: rebuild the start from scratch and then append the remainder after the name '('
    # Find the index where "fn <name>(" begins in the current line
    # We'll just find the first occurrence of "fn" and then of "<name>(".
    # Safer: find the position of the function name and slice from there.
    name_pos = line.find(func_name)
    if name_pos < 0:
        return None

    # remainder includes from func_name to end, so it keeps generics etc.
    remainder = line[name_pos:]
    new_line = f"{indent}{MAKE_PREFIX}{mid}fn {remainder}"
    return new_line

def patch_function(file_path: Path, line_no_1based: int, func_name: str) -> bool:
    """
    Search around line_no for fn definition; patch to pub.
    Returns True if changed.
    Debug prints FOUND/NOT FOUND and shows snippet line.
    """
    src_text = read_text(file_path)
    lines = src_text.splitlines()
    idx = line_no_1based - 1

    debug(f"  [CHECK] File: {file_path}  (lines={len(lines)})  target line={line_no_1based}  func={func_name}")

    # search window
    start = max(0, idx - WINDOW)
    end = min(len(lines), idx + WINDOW + 1)

    # 1) window search
    for i in range(start, end):
        new_line = patch_line_to_pub(lines[i], func_name)
        if new_line is not None:
            debug(f"  [FOUND] near line {i+1}:")
            debug(f"          OLD: {lines[i]}")
            debug(f"          NEW: {new_line}")
            if DRY_RUN:
                debug("  [DRYRUN] Not writing changes.")
                return True
            make_backup_once(file_path)
            lines[i] = new_line
            write_text(file_path, "\n".join(lines) + "\n")
            debug("  [WRITE] patched.")
            return True

        # also detect if already public
        if build_fn_patterns(func_name).match(lines[i]) and is_already_public(lines[i], func_name):
            debug(f"  [FOUND] near line {i+1} but already public:")
            debug(f"          LINE: {lines[i]}")
            return False

    debug("  [NOT FOUND] in window search.")

    # 2) global fallback
    if GLOBAL_FALLBACK:
        debug("  [FALLBACK] global search...")
        for i in range(len(lines)):
            new_line = patch_line_to_pub(lines[i], func_name)
            if new_line is not None:
                debug(f"  [FOUND] global at line {i+1}:")
                debug(f"          OLD: {lines[i]}")
                debug(f"          NEW: {new_line}")
                if DRY_RUN:
                    debug("  [DRYRUN] Not writing changes.")
                    return True
                make_backup_once(file_path)
                lines[i] = new_line
                write_text(file_path, "\n".join(lines) + "\n")
                debug("  [WRITE] patched.")
                return True

            if build_fn_patterns(func_name).match(lines[i]) and is_already_public(lines[i], func_name):
                debug(f"  [FOUND] global at line {i+1} but already public:")
                debug(f"          LINE: {lines[i]}")
                return False

        debug("  [NOT FOUND] global search failed too.")

    return False

def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: python fix_priv_methods_debug.py bug01.txt")
        return 2

    log_path = Path(sys.argv[1])
    if not log_path.exists():
        print(f"[ERROR] Log file not found: {log_path}")
        return 2

    log_lines = read_text(log_path).splitlines()

    # Collect targets: list of (method_name, def_path_raw, def_line, def_col)
    targets = []
    current_method = None

    debug(f"[INFO] Parsing log: {log_path}")
    for line in log_lines:
        m = RE_E0624.match(line)
        if m:
            current_method = m.group(1)
            debug(f"[PARSE] E0624 method: {current_method}")
            continue

        if current_method is not None:
            m2 = RE_DEFINED_HERE_LINE.match(line)
            if m2:
                parsed = parse_path_line_col(m2.group(1))
                if parsed is None:
                    debug(f"[WARN] Could not parse 'defined here' line: {line}")
                    current_method = None
                    continue
                p, lno, cno, raw = parsed
                debug(f"[PARSE] defined-here: raw='{raw}'  path='{p}'  line={lno} col={cno}")
                targets.append((current_method, p, lno, cno, raw))
                current_method = None

    if not targets:
        debug("[INFO] No E0624 blocks parsed. Nothing to do.")
        return 0

    debug(f"[INFO] Total E0624 targets parsed: {len(targets)}")
    changed = 0
    processed = 0

    # Dedup by (resolved_file, line, method) to avoid repeated edits
    seen = set()

    for method, p, lno, cno, raw in targets:
        processed += 1
        debug("\n============================================================")
        debug(f"[TARGET {processed}] method='{method}'  defined at '{raw}'")

        resolved = resolve_file_path(p)
        if resolved is None:
            debug(f"[FILE] NOT FOUND on disk for path='{p}'")
            debug("       HINT: run this script from project root where Cargo.toml is,")
            debug("             or your log contains paths that don't match your CWD.")
            continue

        debug(f"[FILE] FOUND: {resolved}")

        key = (str(resolved).lower(), lno, method)
        if key in seen:
            debug("[SKIP] duplicate target.")
            continue
        seen.add(key)

        ok = patch_function(resolved, lno, method)
        if ok:
            changed += 1

    debug("\n============================================================")
    debug(f"[DONE] Parsed targets={len(targets)}, attempted={len(seen)}, changed={changed}, visibility='{VIS}', dry_run={DRY_RUN}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
