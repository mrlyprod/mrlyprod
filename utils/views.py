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

RAW = re.compile(r"\b(?:Node|Role|Pick)::|\bui::|\btokens::|\bjson!")
KIT = re.compile(r"\bkit::")

def files():
    out = []
    for root, _, names in os.walk(SRC):
        if "view.rs" in names:
            out.append(os.path.join(root, "view.rs"))
    return sorted(out)

def main():
    violations = []
    count = 0
    scanned = files()
    for path in scanned:
        rel = os.path.relpath(path, SRC)
        with open(path) as f:
            for lineno, line in enumerate(f, 1):
                count += len(KIT.findall(line))
                for m in RAW.finditer(line):
                    violations.append(f"{rel}:{lineno}: {m.group(0)} (views compose the kit only)")
    if violations:
        for v in violations:
            print(v)
        sys.exit(1)
    print(f"views (clean, {len(scanned)} files, {count} parts)")

if __name__ == "__main__":
    main()
