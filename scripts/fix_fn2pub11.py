# fix_e0753_inner_doc_debug.py
# Fix Rust error[E0753]: inner doc comments (//!) placed after items.
# Strategy:
#   - If a file contains `//!` not at the very top (before any item),
#     convert those `//!` lines to normal `//` comments.
#
# Safe for SplitRS output.
# Win10 friendly, with debug output.

from __future__ import annotations
import re
import sys
from pathlib import Path

DRY_RUN = False

def dbg(msg: str):
    print(msg, flush=True)

def read(p: Path) -> str:
    return p.read_text(encoding="utf-8", errors="replace")

def write(p: Path, s: str):
    p.write_text(s, encoding="utf-8", newline="")

def backup(p: Path):
    bak = p.with_suffix(p.suffix + ".bak")
    if not bak.exists():
        bak.write_text(read(p), encoding="utf-8", newline="")

# match file location lines
RE_ERROR_LOC = re.compile(r"-->\s+([^\s:]+):(\d+):(\d+)")

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

def fix_file(file: Path) -> bool:
    src = read(file)
    lines = src.splitlines()
    changed = False

    # find first "item" line index
    # item = use / mod / struct / enum / impl / fn / const / type / trait / extern crate
    item_re = re.compile(
        r"^\s*(use|pub\s+use|mod|pub\s+mod|struct|enum|impl|fn|pub\s+fn|const|type|trait|extern\s+crate)\b"
    )

    first_item_idx = None
    for i, l in enumerate(lines):
        if item_re.match(l):
            first_item_idx = i
            break

    if first_item_idx is None:
        # no items, safe to keep //!
        return False

    for i in range(first_item_idx, len(lines)):
        if lines[i].lstrip().startswith("//!"):
            dbg(f"  [PATCH] {file}:{i+1} convert `//!` -> `//`")
            dbg(f"          OLD: {lines[i]}")
            lines[i] = lines[i].replace("//!", "//", 1)
            dbg(f"          NEW: {lines[i]}")
            changed = True

    if changed:
        if DRY_RUN:
            dbg(f"  [DRYRUN] would update {file}")
            return True
        backup(file)
        write(file, "\n".join(lines) + "\n")
        dbg(f"  [WRITE] updated {file}")
        return True

    dbg(f"  [NOCHANGE] {file}")
    return False

def main():
    if len(sys.argv) < 2:
        print("Usage: python fix_e0753_inner_doc_debug.py bug06.txt")
        return 2

    log = Path(sys.argv[1])
    if not log.exists():
        print(f"[ERROR] log not found: {log}")
        return 2

    dbg(f"[INFO] parsing {log}")
    files = []
    for line in read(log).splitlines():
        m = RE_ERROR_LOC.search(line)
        if m:
            files.append(m.group(1))

    # unique preserve order
    seen = set()
    uniq = []
    for f in files:
        if f not in seen:
            seen.add(f)
            uniq.append(f)

    dbg(f"[INFO] files to check: {len(uniq)}")

    fixed = 0
    for f in uniq:
        dbg("\n--------------------------------------------")
        dbg(f"[FILE] {f}")
        rp = resolve_path(f)
        if not rp:
            dbg("  [FAIL] file not found")
            continue
        dbg(f"  [OK] resolved to {rp}")
        if fix_file(rp):
            fixed += 1

    dbg("\n============================================")
    dbg(f"[DONE] files={len(uniq)} fixed={fixed} dry_run={DRY_RUN}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
