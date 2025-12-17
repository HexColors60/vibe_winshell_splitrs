# fix_missing_use_debug.py
# Auto add `use crate::...;` lines suggested by rustc help messages.
# Handles E0433 / E0412 undeclared type errors.
# Win10 friendly, heavy debug output.

from __future__ import annotations
import re
import sys
from pathlib import Path

DRY_RUN = False  # True = only print, do not modify files

def dbg(msg: str):
    print(msg, flush=True)

# error location
RE_ERROR_FILE = re.compile(r"-->\s+([^\s:]+):(\d+):(\d+)")
# rustc help line
RE_HELP_USE = re.compile(r"\+\s+(use\s+.+?;)\s*$")

def read(p: Path) -> str:
    return p.read_text(encoding="utf-8", errors="replace")

def write(p: Path, s: str):
    p.write_text(s, encoding="utf-8", newline="")

def backup(p: Path):
    bak = p.with_suffix(p.suffix + ".bak")
    if not bak.exists():
        bak.write_text(read(p), encoding="utf-8", newline="")

def resolve_path(p: str) -> Path | None:
    path = Path(p)
    if path.exists():
        return path
    p2 = Path(p.replace("/", "\\"))
    if p2.exists():
        return p2
    cand = Path.cwd() / path
    if cand.exists():
        return cand
    return None

def insert_use(file: Path, use_line: str) -> bool:
    src = read(file)
    lines = src.splitlines()

    # already exists?
    if any(l.strip() == use_line for l in lines):
        dbg(f"  [SKIP] already has: {use_line}")
        return False

    # find insertion point:
    # after mod / crate attributes / existing use lines
    insert_at = 0
    for i, l in enumerate(lines):
        s = l.strip()
        if (
            s.startswith("#!") or
            s.startswith("use ") or
            s.startswith("pub use ") or
            s.startswith("extern crate")
        ):
            insert_at = i + 1
            continue
        if s == "":
            insert_at = i + 1
            continue
        break

    dbg(f"  [INSERT] line {insert_at+1}: {use_line}")

    if DRY_RUN:
        return True

    backup(file)
    lines.insert(insert_at, use_line)
    write(file, "\n".join(lines) + "\n")
    return True

def main():
    if len(sys.argv) < 2:
        print("Usage: python fix_missing_use_debug.py bug02.txt")
        return 2

    log = Path(sys.argv[1])
    if not log.exists():
        print(f"[ERROR] log not found: {log}")
        return 2

    dbg(f"[INFO] parsing {log}")
    lines = read(log).splitlines()

    current_file = None
    fixes = []

    for i, line in enumerate(lines):
        m = RE_ERROR_FILE.search(line)
        if m:
            current_file = m.group(1)
            dbg(f"[ERROR] file={current_file}")
            continue

        if current_file:
            m2 = RE_HELP_USE.search(line)
            if m2:
                use_stmt = m2.group(1)
                fixes.append((current_file, use_stmt))
                dbg(f"[HELP] found suggestion: {use_stmt}")
                current_file = None

    dbg(f"[INFO] total fixes found: {len(fixes)}")

    applied = 0
    for idx, (file_str, use_stmt) in enumerate(fixes, 1):
        dbg("\n--------------------------------------------")
        dbg(f"[FIX {idx}] file={file_str}")
        rp = resolve_path(file_str)
        if not rp:
            dbg(f"  [FAIL] file not found")
            continue

        dbg(f"  [OK] resolved to {rp}")
        if insert_use(rp, use_stmt):
            applied += 1

    dbg("\n============================================")
    dbg(f"[DONE] fixes={len(fixes)} applied={applied} dry_run={DRY_RUN}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
