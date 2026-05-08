#!/usr/bin/env python3
"""Post-process openapi-generator's reqwest-trait Rust output.

Fixes a known template bug where multipart file params marked
`required: true` end up typed as PathBuf / Vec<PathBuf> but are wrapped
in `if let Some(ref path) = NAME { ... }` — which doesn't compile because
NAME is not an Option.

We rewrite the broken pattern depending on whether NAME is single-file
(`PathBuf`) or multi-file (`Vec<PathBuf>`) by looking at the surrounding
Params struct.
"""
import re
import sys
from pathlib import Path

API_DIR = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("sdk/src/apis")

PARAM_FIELD = re.compile(r"^\s+pub (\w+):\s*(Vec<std::path::PathBuf>|std::path::PathBuf)\s*[,}]?\s*$", re.M)
BROKEN = re.compile(
    r"        if let Some\(ref path\) = (\w+) \{\n"
    r"            local_var_form = local_var_form\.file\(\"(\w+)\", path\.as_os_str\(\)\)\.await\?;\n"
    r"        \}\n"
)

def field_types(text: str) -> dict[str, str]:
    """Return {field_name: type_string} for all PathBuf fields in any Params struct."""
    return {m.group(1): m.group(2) for m in PARAM_FIELD.finditer(text)}

def fix_file(p: Path) -> bool:
    text = p.read_text()
    types = field_types(text)
    changed = False

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
        changed = True
        print(f"  fixed {n} multipart blocks in {p.name}")
    return changed

def main() -> None:
    for p in sorted(API_DIR.glob("*.rs")):
        fix_file(p)

if __name__ == "__main__":
    main()
