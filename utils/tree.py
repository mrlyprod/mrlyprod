import os
import subprocess
from config import ROOT

OUTPUT = os.path.join(ROOT, "TREE.md")

STUBS = {"sites/web/fixtures", "pkgs/rs/mrlycli/tests/shots"}

def tracked():
    out = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, text=True)
    return [p for p in out.stdout.split("\0") if p]

def tree_of(paths):
    root = {}
    for path in paths:
        stub = next((s for s in STUBS if path.startswith(s + "/")), None)
        parts = (stub or path).split("/")
        node = root
        for part in parts[:-1]:
            node = node.setdefault(part, {})
        node.setdefault(parts[-1], {} if stub else None)
    return root

def render(node, prefix=""):
    dirs = sorted(k for k, v in node.items() if v is not None)
    files = sorted(k for k, v in node.items() if v is None)
    items = dirs + files
    lines = []
    for i, name in enumerate(items):
        last = i == len(items) - 1
        lines.append(f"{prefix}{'└── ' if last else '├── '}{name}")
        child = node[name]
        if child:
            lines.extend(render(child, prefix + ("    " if last else "│   ")))
    return lines

def generate():
    lines = render(tree_of(tracked()))
    content = "# MrlyTree\n\n```\nmrlyprod\n" + "\n".join(lines) + "\n```\n"
    with open(OUTPUT, "w") as f:
        f.write(content)
    print(f"Wrote {OUTPUT}")

if __name__ == "__main__":
    generate()
