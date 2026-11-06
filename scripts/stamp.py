import os
import re
import sys
import tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)
RS = os.path.join(REPO, "packages", "rs")
ROOT = os.path.join(REPO, "Cargo.toml")
PATCH = os.path.join(REPO, "research", ".cargo", "config.toml")
INHERIT = [
    "version",
    "edition",
    "license",
    "repository",
    "homepage",
    "rust-version",
    "keywords",
    "categories",
]
STRING = re.compile(r'"(?:\\.|[^"\\\n])*"')
COMMENT = re.compile(r"//[^\n]*")
SIBLING = re.compile(r"\bmrly_([a-z][a-z0-9]*)\b(?!_)")
USES_SERDE = re.compile(r"\bserde::|\buse serde\b|#\[serde\(|derive\([^)]*\b(?:Serialize|Deserialize)\b")
SECTION = re.compile(r"^\[.*\]\s*$", re.MULTILINE)


def crates():
    return sorted(
        name for name in os.listdir(RS) if os.path.isfile(os.path.join(RS, name, "src", "lib.rs")) or os.path.isfile(os.path.join(RS, name, "src", "main.rs"))
    )


def sources(folder):
    found = []
    for base, _, names in os.walk(folder):
        found.extend(os.path.join(base, name) for name in names if name.endswith(".rs"))
    return sorted(found)


def scan(folder, known):
    siblings, serde = set(), False
    for path in sources(folder):
        with open(path) as f:
            text = COMMENT.sub("", f.read())
        serde = serde or bool(USES_SERDE.search(text))
        siblings.update(name for name in SIBLING.findall(STRING.sub('""', text)) if name in known)
    return siblings, serde


def split(text):
    marks = list(SECTION.finditer(text))
    head = text[: marks[0].start()] if marks else text
    parts = []
    for at, mark in enumerate(marks):
        end = marks[at + 1].start() if at + 1 < len(marks) else len(text)
        parts.append((mark.group().strip(), text[mark.end() : end].strip("\n").splitlines()))
    return head, parts


def derived(line):
    key = line.split("=")[0].strip().split(".")[0]
    return key.startswith("mrly-") or key == "serde"


def table(lines):
    return sorted(lines, key=lambda line: line.split("=")[0].strip())


def manifest(name, known, text):
    _, parts = split(text)
    held = dict(parts)
    order = [title for title, _ in parts]
    folder = os.path.join(RS, name)
    siblings, serde = scan(os.path.join(folder, "src"), known)
    siblings.discard(name)
    wanted = {f"mrly-{one}.workspace = true" for one in siblings}
    if serde:
        wanted.add("serde.workspace = true")
    extra = set()
    for sub in ("examples", "tests", "benches"):
        more, more_serde = scan(os.path.join(folder, sub), known)
        more.discard(name)
        extra |= {f"mrly-{one}.workspace = true" for one in more}
        if more_serde:
            extra.add("serde.workspace = true")
    extra -= wanted

    fixed = {f"{key}.workspace = true" for key in INHERIT}
    kept = [
        line
        for line in held.get("[package]", [])
        if line.strip() and not line.startswith("name") and line not in fixed and line.split("=")[0].strip() not in INHERIT
    ]
    package = [f'name = "mrly-{name}"', *[f"{key}.workspace = true" for key in INHERIT], *kept]

    deps = [line for line in held.get("[dependencies]", []) if line.strip() and not derived(line)]
    dev = [line for line in held.get("[dev-dependencies]", []) if line.strip() and not derived(line)]
    built = {
        "[package]": package,
        "[dependencies]": table([*wanted, *deps]),
        "[dev-dependencies]": table([*extra, *dev]),
    }
    if "[lints.rust]" not in held:
        built["[lints]"] = ["workspace = true"]
    rest = [title for title in order if title not in built]
    out = []
    for title in ["[package]", "[lints]", "[dependencies]", "[dev-dependencies]", *rest]:
        lines = built.get(title, held.get(title, []))
        if lines:
            out.append(title + "\n" + "\n".join(lines) + "\n")
    return "\n".join(out)


def root(known, text):
    version = tomllib.loads(text)["workspace"]["package"]["version"]
    members = 'members = ["packages/rs/*"]'
    text = re.sub(r"members = \[[^\]]*\]", members, text, count=1)
    head, tail = text.split("[workspace.dependencies]\n", 1)
    others = [line for line in tail.splitlines() if line.strip() and not line.startswith("mrly-")]
    rows = [f'mrly-{name} = {{ path = "packages/rs/{name}", version = "{version}" }}' for name in known]
    return head + "[workspace.dependencies]\n" + "\n".join([*rows, *others]) + "\n"


def patch(known, text):
    head = text.split("[patch.crates-io]")[0]
    rows = [f'mrly-{name} = {{ path = "../packages/rs/{name}" }}' for name in known if not hidden(name)]
    return head + "[patch.crates-io]\n" + "\n".join(rows) + "\n"


def hidden(name):
    return name in ("wire", "bindgen")


def read(path):
    if not os.path.exists(path):
        return ""
    with open(path) as f:
        return f.read()


def plan():
    known = crates()
    files = {ROOT: root(known, read(ROOT)), PATCH: patch(known, read(PATCH))}
    for name in known:
        if hidden(name):
            continue
        path = os.path.join(RS, name, "Cargo.toml")
        files[path] = manifest(name, set(known), read(path))
    return files


def main():
    check = "--check" in sys.argv[1:]
    stale = []
    for path, text in plan().items():
        if read(path) == text:
            continue
        stale.append(os.path.relpath(path, REPO))
        if not check:
            with open(path, "w") as f:
                f.write(text)
    if check and stale:
        print("stale, run uv run scripts/stamp.py:")
        for path in stale:
            print(f"  {path}")
        sys.exit(1)
    if stale:
        print(f"stamped {len(stale)} files")


if __name__ == "__main__":
    main()
