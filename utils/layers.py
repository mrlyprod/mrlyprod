import os
import re
import sys

SRC = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "pkgs",
    "mrlyrs",
    "mrlyapps",
    "src",
)

PATH = re.compile(r"crate::([a-z_][a-z0-9_]*)(?:::(\{[^}]*\}|[A-Za-z_][A-Za-z0-9_]*))?")
SUPER = re.compile(r"super::([a-z_][a-z0-9_]*)")

def files():
    out = []
    for root, _, names in os.walk(SRC):
        for name in names:
            if name.endswith(".rs"):
                out.append(os.path.join(root, name))
    return sorted(out)

def targets(segment):
    if segment is None:
        return [None]
    if segment.startswith("{"):
        return [s.strip().split("::")[0].split(" ")[0] for s in segment[1:-1].split(",") if s.strip()]
    return [segment]

def main():
    categories = {
        d: set(os.listdir(os.path.join(SRC, d)))
        for d in os.listdir(SRC)
        if os.path.isdir(os.path.join(SRC, d))
    }
    violations = []
    count = 0
    scanned = files()
    for path in scanned:
        rel = os.path.relpath(path, SRC)
        parts = rel.split(os.sep)
        if len(parts) < 3:
            continue
        cat, app = parts[0], parts[1]
        with open(path) as f:
            for lineno, line in enumerate(f, 1):
                for m in PATH.finditer(line):
                    tcat = m.group(1)
                    if tcat not in categories:
                        continue
                    for tapp in targets(m.group(2)):
                        count += 1
                        if tapp is None or tapp[0].isupper():
                            violations.append(f"{rel}:{lineno}: {m.group(0)} ({cat}/{app} reaches into {tcat})")
                        elif (tcat, tapp) != (cat, app):
                            violations.append(f"{rel}:{lineno}: {m.group(0)} ({cat}/{app} -> {tcat}/{tapp})")
                if parts[2] == "mod.rs":
                    for m in SUPER.finditer(line):
                        sibling = m.group(1)
                        if sibling in categories.get(cat, set()) and sibling != app:
                            violations.append(f"{rel}:{lineno}: {m.group(0)} ({cat}/{app} -> {cat}/{sibling})")
    if violations:
        for v in violations:
            print(v)
        sys.exit(1)
    print(f"layers (clean, {len(scanned)} files, {count} refs)")

if __name__ == "__main__":
    main()
