#!/usr/bin/env python3
"""Post-process openapi-generator's reqwest-trait Rust output.

Two patches:

1. Multipart bug: the template emits `if let Some(ref path) = NAME { ... }`
   for required multipart file params, but NAME is the bare PathBuf /
   Vec<PathBuf> so the code doesn't compile. Rewrite to direct/iterating
   form.file() calls.

2. Drop the hardcoded `_api` suffix on per-tag accessors and module paths
   so users write `client.videos().get(...)` instead of
   `client.videos_api().get(...)`. Renames module files
   (`videos_api.rs` -> `videos.rs`) and every `*_api` identifier in
   apis/mod.rs.
"""
import re
import sys
from pathlib import Path

API_DIR = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("sdk/src/apis")

# ---- (1) multipart fix ----
PARAM_FIELD = re.compile(r"^\s+pub (\w+):\s*(Vec<std::path::PathBuf>|std::path::PathBuf)\s*[,}]?\s*$", re.M)
BROKEN = re.compile(
    r"        if let Some\(ref path\) = (\w+) \{\n"
    r"            local_var_form = local_var_form\.file\(\"(\w+)\", path\.as_os_str\(\)\)\.await\?;\n"
    r"        \}\n"
)

def field_types(text: str) -> dict[str, str]:
    return {m.group(1): m.group(2) for m in PARAM_FIELD.finditer(text)}

def fix_multipart(p: Path) -> int:
    text = p.read_text()
    types = field_types(text)

    def replace(m: re.Match) -> str:
        var, form_name = m.group(1), m.group(2)
        ty = types.get(var)
        if ty == "Vec<std::path::PathBuf>":
            return (
                f"        for path in &{var} {{\n"
                f"            local_var_form = local_var_form.file(\"{form_name}\", path.as_os_str()).await?;\n"
                f"        }}\n"
            )
        if ty == "std::path::PathBuf":
            return f"        local_var_form = local_var_form.file(\"{form_name}\", {var}.as_os_str()).await?;\n"
        return m.group(0)

    new_text, n = BROKEN.subn(replace, text)
    if n > 0:
        p.write_text(new_text)
    return n

# ---- (2) drop _api suffix ----
def drop_api_suffix(api_dir: Path) -> None:
    renames = {f.stem: f.stem[:-4] for f in api_dir.glob("*_api.rs")}
    if not renames:
        return

    for f in api_dir.glob("*.rs"):
        text = f.read_text()
        new_text = text
        for old, new in renames.items():
            new_text = re.sub(rf"\b{old}\b", new, new_text)
        if new_text != text:
            f.write_text(new_text)

    for old, new in renames.items():
        src = api_dir / f"{old}.rs"
        dst = api_dir / f"{new}.rs"
        if src.exists():
            src.rename(dst)

def main() -> None:
    for p in sorted(API_DIR.glob("*.rs")):
        n = fix_multipart(p)
        if n:
            print(f"  fixed {n} multipart blocks in {p.name}")

    drop_api_suffix(API_DIR)
    print(f"  dropped _api suffix on per-tag modules and accessors")

if __name__ == "__main__":
    main()
