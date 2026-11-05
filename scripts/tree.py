import os
import subprocess
import sys


def tracked(root):
    done = subprocess.run(["git", "ls-files", "-z"], cwd=root, capture_output=True, text=True)
    if done.returncode != 0:
        sys.exit(done.stderr.strip() or f"{root} is not a git repo")
    return [name for name in done.stdout.split("\0") if name and os.path.lexists(os.path.join(root, name))]


def insert(node, parts):
    if len(parts) == 1:
        node.setdefault("", []).append(parts[0])
    else:
        insert(node.setdefault(parts[0], {}), parts[1:])


def draw(node, prefix=""):
    dirs = sorted(key for key in node if key)
    entries = [(name, True) for name in dirs] + [(name, False) for name in sorted(node.get("", []))]
    for i, (name, is_dir) in enumerate(entries):
        last = i + 1 == len(entries)
        print(f"{prefix}{'└──' if last else '├──'} {name}{'/' if is_dir else ''}")
        if is_dir:
            draw(node[name], prefix + ("    " if last else "│   "))


def main():
    root = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else ".")
    names = tracked(root)
    tree = {}
    for name in names:
        insert(tree, name.split("/"))
    print(f"{os.path.basename(root)}/  {len(names)} files")
    draw(tree)


if __name__ == "__main__":
    main()
