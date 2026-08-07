import os
from config import ROOT
from ignore import is_ignored

OUTPUT = os.path.join(ROOT, "TREE.md")

STUBS = {"apps/web/fixtures"}

def build_tree(directory, prefix=""):
    entries = sorted(os.listdir(directory))
    entries = [e for e in entries if not is_ignored(os.path.relpath(os.path.join(directory, e), ROOT))]
    dirs = [e for e in entries if os.path.isdir(os.path.join(directory, e))]
    files = [e for e in entries if not os.path.isdir(os.path.join(directory, e))]
    items = dirs + files
    lines = []
    for i, name in enumerate(items):
        is_last = i == len(items) - 1
        connector = "└── " if is_last else "├── "
        lines.append(f"{prefix}{connector}{name}")
        path = os.path.join(directory, name)
        if os.path.isdir(path) and os.path.relpath(path, ROOT) not in STUBS:
            extension = "    " if is_last else "│   "
            lines.extend(build_tree(path, prefix + extension))
    return lines

def generate():
    lines = build_tree(ROOT)
    content = "# MrlyTree\n\n```\nmrlyprod\n"
    for line in lines:
        content += line + "\n"
    content += "```\n"
    with open(OUTPUT, "w") as f:
        f.write(content)
    print(f"Wrote {OUTPUT}")

if __name__ == "__main__":
    generate()