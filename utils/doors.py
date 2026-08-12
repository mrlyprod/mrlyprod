import os
import re
import sys

DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOORS = {"boot", "list", "call", "read"}
WASM_SRC = os.path.join(DIR, "pkgs", "js", "web", "src")
PY_SRC = os.path.join(DIR, "pkgs", "py", "web", "src")
FACE_SRC = os.path.join(DIR, "sites", "web")
DEAD = [os.path.join(DIR, "pkgs", "js", "math"), os.path.join(DIR, "pkgs", "py", "math")]

EXPORT = re.compile(r"^#\[(wasm_bindgen|pyfunction)")
FN = re.compile(r"^(?:pub )?fn ([a-z_][a-z0-9_]*)")
SUBMODULE = re.compile(r"add_submodule")
WIRE = re.compile(r"from \"[^\"]*pkgs/js/web/pkg/mrlyweb\.js\"")

def sources(root, suffixes):
    out = []
    for base, _, names in os.walk(root):
        if "node_modules" in base or os.sep + "gen" in base:
            continue
        for name in names:
            if name.endswith(suffixes):
                out.append(os.path.join(base, name))
    return sorted(out)

def exported(root):
    found = []
    for path in sources(root, (".rs",)):
        rel = os.path.relpath(path, DIR)
        with open(path) as f:
            lines = f.readlines()
        for i, line in enumerate(lines):
            if not EXPORT.match(line.strip()):
                continue
            for after in lines[i + 1 : i + 4]:
                m = FN.match(after.strip())
                if m:
                    found.append((rel, i + 1, m.group(1)))
                    break
    return found

def main():
    violations = []
    for path in DEAD:
        if os.path.exists(path):
            violations.append(f"{os.path.relpath(path, DIR)}: the math island must not exist")
    surfaces = {}
    for name, root in (("mrlyweb-js", WASM_SRC), ("mrlyweb-py", PY_SRC)):
        names = []
        for rel, lineno, fn in exported(root):
            names.append(fn)
            if fn not in DOORS:
                violations.append(f"{rel}:{lineno}: {fn} (the wire has four doors: boot, list, call, read)")
        surfaces[name] = names
    for path in sources(PY_SRC, (".rs",)):
        rel = os.path.relpath(path, DIR)
        with open(path) as f:
            for lineno, line in enumerate(f, 1):
                if SUBMODULE.search(line):
                    violations.append(f"{rel}:{lineno}: add_submodule (the wire registers no side modules)")
    wired = []
    for path in sources(FACE_SRC, (".ts", ".tsx")):
        rel = os.path.relpath(path, DIR)
        with open(path) as f:
            for lineno, line in enumerate(f, 1):
                if WIRE.search(line):
                    wired.append(rel)
                    if not rel.endswith(os.path.join("src", "kernel.ts")):
                        violations.append(f"{rel}:{lineno}: wasm import (the face touches the wire only in kernel.ts)")
    if violations:
        for v in violations:
            print(v)
        sys.exit(1)
    js = " ".join(sorted(set(surfaces["mrlyweb-js"])))
    py = " ".join(sorted(set(surfaces["mrlyweb-py"])))
    print(f"doors (mrlyweb-js: {js}; mrlyweb-py: {py}; wire imports: {len(set(wired))} file)")

if __name__ == "__main__":
    main()
