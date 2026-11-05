import collections
import os
import subprocess
import sys

LANGUAGES = {
    "rs": "Rust",
    "py": "Python",
    "ts": "TypeScript",
    "tsx": "TypeScript",
    "js": "JavaScript",
    "jsx": "JavaScript",
    "mjs": "JavaScript",
    "md": "Markdown",
    "toml": "TOML",
    "json": "JSON",
    "jsonc": "JSON",
    "css": "CSS",
    "html": "HTML",
    "sh": "Shell",
    "zsh": "Shell",
    "yml": "YAML",
    "yaml": "YAML",
    "svg": "SVG",
    "lean": "Lean",
}


def git(root, *args):
    done = subprocess.run(["git", *args], cwd=root, capture_output=True, text=True)
    if done.returncode != 0:
        sys.exit(done.stderr.strip() or f"{root} is not a git repo")
    return done.stdout


def human(size):
    for limit, unit in ((1 << 30, "GB"), (1 << 20, "MB")):
        if size >= limit:
            return f"{size / limit:.1f} {unit}"
    return f"{size >> 10} KB" if size >= 1 << 10 else f"{size} B"


def count(path):
    try:
        with open(path, errors="ignore") as handle:
            return sum(1 for _ in handle)
    except OSError:
        return 0


def main():
    root = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else ".")
    names = [name for name in git(root, "ls-files", "-z").split("\0") if name]
    languages = collections.defaultdict(lambda: [0, 0])
    folders = collections.Counter()
    total = 0
    kept = 0
    for name in names:
        path = os.path.join(root, name)
        if not os.path.isfile(path) or os.path.islink(path):
            continue
        kept += 1
        size = os.path.getsize(path)
        total += size
        folders[name.split("/")[0] + "/" if "/" in name else "."] += size
        language = LANGUAGES.get(name.rsplit(".", 1)[-1])
        if language:
            languages[language][0] += 1
            languages[language][1] += count(path)
    print(f"{os.path.basename(root).upper()}: {kept} files, {human(total)}")
    print("\nLANGUAGES")
    for language, (files, lines) in sorted(languages.items(), key=lambda row: -row[1][1]):
        print(f"  {language:<12} {files:>6} files {lines:>9} lines")
    print("\nFOLDERS")
    for folder, size in folders.most_common():
        print(f"  {folder:<24} {human(size)}")
    print("\nACTIVITY")
    months = collections.Counter(git(root, "log", "--format=%cd", "--date=format:%Y-%m").split())
    print(f"  {sum(months.values())} commits, the last on {git(root, 'log', '-1', '--format=%cs').strip()}")
    for month in sorted(months, reverse=True)[:12]:
        print(f"  {month}  {months[month]}")


if __name__ == "__main__":
    main()
