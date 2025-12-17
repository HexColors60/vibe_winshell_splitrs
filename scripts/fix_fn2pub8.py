# fix_sysinfo_system_and_write_debug.py
# Fix two common SplitRS refactor breakages:
# 1) `System` accidentally refers to `std::alloc::System` instead of `sysinfo::System`
# 2) `writeln!(file, ...)` fails because `std::io::Write` trait isn't imported
#
# Usage:
#   cargo build 2> bug04.txt
#   python fix_sysinfo_system_and_write_debug.py bug04.txt

from __future__ import annotations
import re
import sys
from pathlib import Path

DRY_RUN = False

def dbg(s: str):
    print(s, flush=True)

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

RE_ERROR_LOC = re.compile(r"-->\s+([^\s:]+):(\d+):(\d+)")
RE_STD_ALLOC_SYSTEM = re.compile(r"^\s*use\s+std::alloc::System\s*;\s*$")
RE_SYSINFO_SYSTEM = re.compile(r"^\s*use\s+sysinfo::System\s*;\s*$")
RE_USE_WRITE = re.compile(r"^\s*use\s+std::io::Write\s*;\s*$")

def has_write_macros(src: str) -> bool:
    return ("writeln!(" in src) or ("write!(" in src)

def insert_use_after_outer_attrs_and_docs(lines: list[str], use_stmt: str) -> tuple[list[str], bool]:
    # already exists?
    if any(l.strip() == use_stmt for l in lines):
        return lines, False

    insert_at = 0
    for i, l in enumerate(lines):
        s = l.strip()
        # module docs or crate attrs
        if s.startswith("//!") or s.startswith("#!"):
            insert_at = i + 1
            continue
        # leading empty lines
        if s == "":
            insert_at = i + 1
            continue
        # existing use lines cluster
        if s.startswith("use ") or s.startswith("pub use ") or s.startswith("extern crate"):
            insert_at = i + 1
            continue
        break

    lines.insert(insert_at, use_stmt)
    return lines, True

def patch_file(file: Path) -> bool:
    src = read(file)
    lines = src.splitlines()
    changed = False

    # (1) Replace `use std::alloc::System;` -> `use sysinfo::System;`
    for i, l in enumerate(lines):
        if RE_STD_ALLOC_SYSTEM.match(l):
            # if sysinfo::System already present, just remove alloc one
            if any(RE_SYSINFO_SYSTEM.match(x) for x in lines):
                dbg(f"  [PATCH] {file}:{i+1} remove redundant `use std::alloc::System;` (sysinfo::System already imported)")
                dbg(f"          OLD: {lines[i]}")
                if not DRY_RUN:
                    backup(file)
                    lines.pop(i)
                    changed = True
                return True if DRY_RUN else True  # early exit after modifying list
            else:
                dbg(f"  [PATCH] {file}:{i+1} replace std::alloc::System -> sysinfo::System")
                dbg(f"          OLD: {lines[i]}")
                dbg(f"          NEW: use sysinfo::System;")
                if not DRY_RUN:
                    backup(file)
                    lines[i] = "use sysinfo::System;"
                    changed = True
                break

    # (2) Add `use std::io::Write;` if needed
    if has_write_macros(src) and not any(RE_USE_WRITE.match(x) for x in lines):
        dbg(f"  [PATCH] {file} add `use std::io::Write;` (writeln!/write! detected)")
        new_lines, did = insert_use_after_outer_attrs_and_docs(lines, "use std::io::Write;")
        if did:
            lines = new_lines
            changed = True

    if changed and not DRY_RUN:
        write(file, "\n".join(lines) + "\n")
        dbg(f"  [WRITE] {file} updated")
    elif changed and DRY_RUN:
        dbg(f"  [DRYRUN] would update {file}")
    else:
        dbg(f"  [NOCHANGE] {file}")

    return changed

def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: python fix_sysinfo_system_and_write_debug.py bug04.txt")
        return 2

    log = Path(sys.argv[1])
    if not log.exists():
        print(f"[ERROR] log not found: {log}")
        return 2

    dbg(f"[INFO] parsing {log}")
    log_lines = read(log).splitlines()

    files = []
    for line in log_lines:
        m = RE_ERROR_LOC.search(line)
        if m:
            files.append(m.group(1))

    # unique in order
    seen = set()
    uniq = []
    for f in files:
        if f not in seen:
            seen.add(f)
            uniq.append(f)

    dbg(f"[INFO] files referenced by errors: {len(uniq)}")

    applied = 0
    for f in uniq:
        dbg("\n--------------------------------------------")
        dbg(f"[FILE] {f}")
        rp = resolve_path(f)
        if not rp:
            dbg("  [FAIL] not found")
            continue
        dbg(f"  [OK] resolved to {rp}")
        if patch_file(rp):
            applied += 1

    dbg("\n============================================")
    dbg(f"[DONE] files={len(uniq)} patched_files={applied} dry_run={DRY_RUN}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
