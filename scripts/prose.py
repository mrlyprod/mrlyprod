import os
import re
import subprocess
import sys

WIDTH = 120
LINES = 10
PAGE_LINES = 5
PAGE_SECTIONS = 1
PAGE = re.compile(r"(^|/)(site/pages|sites/[^/]+/pages)/")
HELD = re.compile(r"(^|/)(local)/")
FRONT = re.compile(r"\A---\n.*?\n---\n", re.S)
HEAD = re.compile(r"^(#+)(\s|$)")
FENCE = re.compile(r"^\s*(```|~~~)")


def tracked():
    args = ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*.md", "*.mdx"]
    names = subprocess.run(args, capture_output=True, text=True, check=True).stdout.split("\0")
    return sorted({name for name in names if name and os.path.isfile(name)})


def exempt(name, skips):
    if PAGE.search(name):
        return False
    if HELD.search(name):
        return True
    return any(name == skip or name.startswith(skip) for skip in skips)


def problems(name):
    with open(name) as handle:
        text = handle.read()
    page = bool(PAGE.search(name))
    found = FRONT.match(text)
    start = found.group(0).count("\n") if found else 0
    fenced = False
    sections = 0
    here = 0
    total = 0
    for at, line in enumerate(text.split("\n"), 1):
        if len(line) > WIDTH:
            yield f"{name}:{at}: {len(line)} chars, the limit is {WIDTH}"
        if at <= start or not line.strip():
            continue
        if FENCE.match(line):
            fenced = not fenced
        head = None if fenced else HEAD.match(line)
        if head:
            if len(head.group(1)) > 2:
                yield f"{name}:{at}: only # and ## headings"
            if len(head.group(1)) == 2:
                sections += 1
            here = 0
            continue
        here += 1
        total += 1
        if here == LINES + 1:
            yield f"{name}:{at}: a section holds at most {LINES} lines"
    if page and sections > PAGE_SECTIONS:
        yield f"{name}: {sections} sections, a site page holds at most {PAGE_SECTIONS}"
    if page and total > PAGE_LINES:
        yield f"{name}: {total} lines, a site page holds at most {PAGE_LINES}"


def main():
    names = [name for name in tracked() if not exempt(name, sys.argv[1:])]
    bad = [one for name in names for one in problems(name)]
    for one in bad:
        print(one)
    print(f"prose: {len(names)} files, {len(bad)} problems")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
