import os
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def git(start, *args):
    return subprocess.run(["git", *args], cwd=start, capture_output=True, text=True)


def repo(start):
    done = git(start, "rev-parse", "--show-toplevel")
    return done.stdout.strip() if done.returncode == 0 else REPO


def tracked(root):
    done = git(root, "ls-files", "-z", "--cached", "--others", "--exclude-standard")
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
        yield f"{prefix}{'└──' if last else '├──'} {name}{'/' if is_dir else ''}"
        if is_dir:
            yield from draw(node[name], prefix + ("    " if last else "│   "))


def main():
    root = repo(sys.argv[1] if len(sys.argv) > 1 else ".")
    names = tracked(root)
    tree = {}
    for name in names:
        insert(tree, name.split("/"))
    text = "\n".join([f"{os.path.basename(root)}/  {len(names)} files", *draw(tree)])
    print(text)
    if root == REPO:
        with open(os.path.join(REPO, "TREE.md"), "w") as handle:
            handle.write(f"# TREE\n\n```text\n{text}\n```\n")


if __name__ == "__main__":
    main()
