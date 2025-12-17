# fix_e0616_fields_debug.py
# Fix Rust error[E0616]: private struct fields used from other modules.
# It makes the specific referenced fields public in the struct definition.
#
# Usage:
#   cargo build 2> bug03.txt
#   python fix_e0616_fields_debug.py bug03.txt
#
# Run from project root.

from __future__ import annotations
import re
import sys
from pathlib import Path

# ===== CONFIG =====
VIS = "pub"          # change to "pub(crate)" or "pub(super)" if you prefer
DRY_RUN = False
WINDOW_STRUCT_SCAN = 4000  # max lines to scan for struct blocks per file

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
    cand2 = Path.cwd() / Path(p.replace("/", "\\"))
    if cand2.exists():
        return cand2
    return None

# Example:
# error[E0616]: field `programs` of struct `types::AppConfig` is private
RE_E0616 = re.compile(
    r"^error\[E0616\]: field `([^`]+)` of struct `([^`]+)` is private"
)

# Example:
#   --> src\ws\functions.rs:25:15
RE_ERROR_LOC = re.compile(r"-->\s+([^\s:]+):(\d+):(\d+)")

def guess_struct_name(full: str) -> str:
    # "types::AppConfig" -> "AppConfig"
    return full.split("::")[-1]

def candidate_type_files(full: str) -> list[Path]:
    # Heuristic: if struct path contains "types::", try src/ws/types.rs first.
    files = []
    if "types::" in full or full.endswith("::types::AppConfig"):
        files.append(Path("src/ws/types.rs"))
        files.append(Path("src\\ws\\types.rs"))
    # Also try common places
    files.append(Path("src/ws/types.rs"))
    files.append(Path("src/ws/mod.rs"))
    files.append(Path("src/main.rs"))
    return files

def find_struct_block_in_file(file: Path, struct_name: str):
    """
    Find `struct <Name> { ... }` or `pub struct <Name> { ... }` block in file.
    Returns (start_idx, end_idx, lines) where indexes are inclusive line indexes of the block.
    """
    lines = read(file).splitlines()
    pat = re.compile(rf"^\s*(pub\s+)?struct\s+{re.escape(struct_name)}\b")
    start = None
    brace_depth = 0
    in_block = False

    for i, l in enumerate(lines):
        if start is None:
            if pat.search(l):
                start = i
                # find first '{' from this line onward
                if "{" in l:
                    in_block = True
                    brace_depth = l.count("{") - l.count("}")
                    if brace_depth <= 0:
                        return None  # weird one-line struct?
                continue
        else:
            if not in_block:
                if "{" in l:
                    in_block = True
                    brace_depth = l.count("{") - l.count("}")
                continue
            else:
                brace_depth += l.count("{") - l.count("}")
                if brace_depth == 0:
                    end = i
                    return start, end, lines
        if i > WINDOW_STRUCT_SCAN:
            break
    return None

def patch_field_in_struct(file: Path, struct_name: str, field_name: str) -> bool:
    """
    Make the field line `field_name: Type` into `pub field_name: Type` within the struct block.
    """
    block = find_struct_block_in_file(file, struct_name)
    if not block:
        dbg(f"  [NOT FOUND] struct {struct_name} in {file}")
        return False

    start, end, lines = block
    dbg(f"  [FOUND] struct {struct_name} block in {file} lines {start+1}-{end+1}")

    field_pat = re.compile(rf"^(\s*)(pub\s+)?{re.escape(field_name)}\s*:\s*")
    changed = False

    for i in range(start, end + 1):
        m = field_pat.match(lines[i])
        if not m:
            continue

        indent = m.group(1)
        has_pub = m.group(2) is not None
        if has_pub:
            dbg(f"  [SKIP] field already public at line {i+1}: {lines[i]}")
            return False

        # Replace only the beginning "field_name:" with "pub field_name:"
        new_line = field_pat.sub(rf"{indent}{VIS} {field_name}: ", lines[i], count=1)
        dbg(f"  [PATCH] {file}:{i+1}")
        dbg(f"          OLD: {lines[i]}")
        dbg(f"          NEW: {new_line}")

        if DRY_RUN:
            return True

        backup(file)
        lines[i] = new_line
        write(file, "\n".join(lines) + "\n")
        changed = True
        return True

    dbg(f"  [NOT FOUND] field '{field_name}' inside struct {struct_name} block in {file}")
    return False

def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: python fix_e0616_fields_debug.py bug03.txt")
        return 2

    log = Path(sys.argv[1])
    if not log.exists():
        print(f"[ERROR] log not found: {log}")
        return 2

    dbg(f"[INFO] parsing {log}")
    log_lines = read(log).splitlines()

    # Collect E0616 items
    targets = []  # (field, struct_full, err_file)
    last_err_file = None

    for line in log_lines:
        mloc = RE_ERROR_LOC.search(line)
        if mloc:
            last_err_file = mloc.group(1)
            continue

        m = RE_E0616.match(line)
        if m:
            field = m.group(1)
            struct_full = m.group(2)
            targets.append((field, struct_full, last_err_file))
            dbg(f"[PARSE] E0616 field='{field}' struct='{struct_full}' (used at {last_err_file})")

    dbg(f"[INFO] targets parsed: {len(targets)}")
    if not targets:
        dbg("[INFO] no E0616 targets found")
        return 0

    # For each target, try to locate struct definition file and patch it
    applied = 0
    seen = set()

    for idx, (field, struct_full, used_file) in enumerate(targets, 1):
        dbg("\n============================================================")
        struct_name = guess_struct_name(struct_full)
        dbg(f"[TARGET {idx}] struct='{struct_full}' field='{field}' used_at='{used_file}'")

        key = (struct_full, field)
        if key in seen:
            dbg("[SKIP] duplicate")
            continue
        seen.add(key)

        # 1) Try likely files
        tried = []
        success = False
        for cand in candidate_type_files(struct_full):
            rp = resolve_path(str(cand))
            if not rp:
                continue
            tried.append(rp)
            dbg(f"[TRY] {rp}")
            if patch_field_in_struct(rp, struct_name, field):
                applied += 1
                success = True
                break

        # 2) If not found, do a small search across src/ws/*.rs and src/*.rs
        if not success:
            dbg("[FALLBACK] scanning src/ws and src for struct definition...")
            candidates = list(Path("src/ws").glob("*.rs")) + list(Path("src").glob("*.rs"))
            for f in candidates:
                tried.append(f)
                if patch_field_in_struct(f, struct_name, field):
                    applied += 1
                    success = True
                    break

        if not success:
            dbg("[FAIL] could not patch. Files tried:")
            for t in tried[:25]:
                dbg(f"       - {t}")
            if len(tried) > 25:
                dbg(f"       ... +{len(tried)-25} more")

    dbg("\n============================================================")
    dbg(f"[DONE] unique_targets={len(seen)} applied={applied} vis='{VIS}' dry_run={DRY_RUN}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
