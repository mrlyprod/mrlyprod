import collections
import os
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

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


def repo(start):
    done = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=start, capture_output=True, text=True)
    return done.stdout.strip() if done.returncode == 0 else REPO


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
    root = repo(sys.argv[1] if len(sys.argv) > 1 else ".")
    names = [name for name in git(root, "ls-files", "-z", "--cached", "--others", "--exclude-standard").split("\0") if name]
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
    lines = [f"{os.path.basename(root).upper()}: {kept} files, {human(total)}", "", "LANGUAGES"]
    for language, (files, tally) in sorted(languages.items(), key=lambda row: -row[1][1]):
        lines.append(f"  {language:<12} {files:>6} files {tally:>9} lines")
    lines += ["", "FOLDERS"]
    for folder, size in folders.most_common():
        lines.append(f"  {folder:<24} {human(size)}")
    lines += ["", "ACTIVITY"]
    months = collections.Counter(git(root, "log", "--format=%cd", "--date=format:%Y-%m").split())
    lines.append(f"  {sum(months.values())} commits, the last on {git(root, 'log', '-1', '--format=%cs').strip()}")
    for month in sorted(months, reverse=True)[:12]:
        lines.append(f"  {month}  {months[month]}")
    text = "\n".join(lines)
    print(text)
    if root == REPO:
        with open(os.path.join(REPO, "STATS.md"), "w") as handle:
            handle.write(f"# STATS\n\n```text\n{text}\n```\n")


if __name__ == "__main__":
    main()
