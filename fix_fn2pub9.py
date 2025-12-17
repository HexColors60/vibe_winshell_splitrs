# fix_e0624_methods_debug.py
# Fix Rust E0624: private method/associated function by making the defining `fn` public.
# Focuses on "method `...` is private" cases, but also works for associated functions.
#
# Usage:
#   cargo build 2> bug05.txt
#   python fix_e0624_methods_debug.py bug05.txt

from __future__ import annotations
import re
import sys
from pathlib import Path

# ===== CONFIG =====
VIS = "pub(crate)"   # recommended; change to "pub" if you want
DRY_RUN = False
WINDOW = 40          # lines around defined-here line to search
GLOBAL_FALLBACK = True

def dbg(s: str) -> None:
    print(s, flush=True)

def read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8", errors="replace")

def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8", newline="")

def backup_once(p: Path) -> None:
    bak = p.with_suffix(p.suffix + ".bak")
    if not bak.exists():
        bak.write_text(read_text(p), encoding="utf-8", newline="")

RE_E0624 = re.compile(
    r"^error\[E0624\]:\s+(?:method|associated function)\s+`([^`]+)`\s+is private"
)
RE_DEFINED_HERE = re.compile(r"^\s*:::\s+(.+)\s*$")

def parse_path_line_col(raw: str):
    s = raw.strip().lstrip("|").strip()
    parts = s.rsplit(":", 2)  # handles Windows drives
    if len(parts) != 3:
        return None
    path_str, line_str, col_str = parts
    try:
        return Path(path_str), int(line_str), int(col_str), s
    except ValueError:
        return None

def resolve_path(p: Path) -> Path | None:
    if p.exists():
        return p
    p2 = Path(str(p).replace("/", "\\"))
    if p2.exists():
        return p2
    if not p.is_absolute():
        cand = Path.cwd() / p
        if cand.exists():
            return cand
        cand2 = Path(str(cand).replace("/", "\\"))
        if cand2.exists():
            return cand2
    return None

def pat_fn(name: str) -> re.Pattern:
    # allow existing pub(...) + async + unsafe before fn
    return re.compile(
        rf"^(\s*)(?:pub(\s*\([^)]+\))?\s+)?(?:(async)\s+)?(?:(unsafe)\s+)?fn\s+{re.escape(name)}\s*\("
    )

def already_pub(line: str, name: str) -> bool:
    return re.match(
        rf"^\s*pub(\s*\([^)]+\))?\s+(?:async\s+)?(?:unsafe\s+)?fn\s+{re.escape(name)}\s*\(",
        line,
    ) is not None

def rewrite_to_pub(line: str, name: str) -> str | None:
    m = pat_fn(name).match(line)
    if not m:
        return None
    if already_pub(line, name):
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
    return f"{indent}{VIS} {mid}fn {remainder}"

def patch_file(file_path: Path, line_no: int, name: str) -> bool:
    lines = read_text(file_path).splitlines()
    idx = line_no - 1

    dbg(f"  [CHECK] file={file_path} target_line={line_no} name={name}")

    start = max(0, idx - WINDOW)
    end = min(len(lines), idx + WINDOW + 1)

    # near-line search first
    for i in range(start, end):
        if pat_fn(name).match(lines[i]):
            if already_pub(lines[i], name):
                dbg(f"  [FOUND] line {i+1} already public: {lines[i]}")
                return False
            new_line = rewrite_to_pub(lines[i], name)
            if new_line:
                dbg(f"  [FOUND] line {i+1} in window")
                dbg(f"          OLD: {lines[i]}")
                dbg(f"          NEW: {new_line}")
                if DRY_RUN:
                    dbg("  [DRYRUN] not writing")
                    return True
                backup_once(file_path)
                lines[i] = new_line
                write_text(file_path, "\n".join(lines) + "\n")
                dbg("  [WRITE] patched")
                return True

    dbg("  [NOT FOUND] in window")

    if GLOBAL_FALLBACK:
        dbg("  [FALLBACK] global search...")
        for i in range(len(lines)):
            if pat_fn(name).match(lines[i]):
                if already_pub(lines[i], name):
                    dbg(f"  [FOUND] global line {i+1} already public: {lines[i]}")
                    return False
                new_line = rewrite_to_pub(lines[i], name)
                if new_line:
                    dbg(f"  [FOUND] global line {i+1}")
                    dbg(f"          OLD: {lines[i]}")
                    dbg(f"          NEW: {new_line}")
                    if DRY_RUN:
                        dbg("  [DRYRUN] not writing")
                        return True
                    backup_once(file_path)
                    lines[i] = new_line
                    write_text(file_path, "\n".join(lines) + "\n")
                    dbg("  [WRITE] patched")
                    return True
        dbg("  [NOT FOUND] global too")

    return False

def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: python fix_e0624_methods_debug.py bug05.txt")
        return 2

    log_path = Path(sys.argv[1])
    if not log_path.exists():
        print(f"[ERROR] log not found: {log_path}")
        return 2

    dbg(f"[INFO] parsing {log_path} (CWD={Path.cwd()})")
    log_lines = read_text(log_path).splitlines()

    targets = []
    current_name = None

    for line in log_lines:
        m = RE_E0624.match(line)
        if m:
            current_name = m.group(1)
            dbg(f"[PARSE] E0624 name='{current_name}'")
            continue

        if current_name is not None:
            m2 = RE_DEFINED_HERE.match(line)
            if m2:
                parsed = parse_path_line_col(m2.group(1))
                if not parsed:
                    dbg(f"[WARN] cannot parse defined-here: {line}")
                    current_name = None
                    continue
                p, lno, cno, raw = parsed
                dbg(f"[PARSE] defined-here raw='{raw}' path='{p}' line={lno} col={cno}")
                targets.append((current_name, p, lno, cno, raw))
                current_name = None

    dbg(f"[INFO] targets parsed: {len(targets)}")
    if not targets:
        dbg("[INFO] no E0624 targets found")
        return 0

    seen = set()
    changed = 0
    for idx, (name, p, lno, _cno, raw) in enumerate(targets, start=1):
        dbg("\n============================================================")
        dbg(f"[TARGET {idx}] name='{name}' defined_at='{raw}'")

        rp = resolve_path(p)
        if rp is None:
            dbg(f"[FILE] NOT FOUND: {p}")
            continue
        dbg(f"[FILE] FOUND: {rp}")

        key = (str(rp).lower(), name)
        if key in seen:
            dbg("[SKIP] duplicate (same file+name)")
            continue
        seen.add(key)

        if patch_file(rp, lno, name):
            changed += 1

    dbg("\n============================================================")
    dbg(f"[DONE] targets={len(seen)} changed={changed} vis='{VIS}' dry_run={DRY_RUN}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
