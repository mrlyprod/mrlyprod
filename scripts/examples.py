import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)
RS = os.path.join(REPO, "packages", "rs")
CAP = 16 * 1024
MOST = 15
KINDS = (".svg", ".txt")


def folders():
    for crate in sorted(os.listdir(RS)):
        folder = os.path.join(RS, crate, "examples")
        if os.path.isdir(folder):
            yield crate, folder


def run(crate, stem):
    done = subprocess.run(
        ["cargo", "run", "-q", "-p", f"mrly-{crate}", "--example", stem],
        cwd=REPO,
        capture_output=True,
        text=True,
    )
    return done.returncode, done.stdout, done.stderr


def read(path):
    if not os.path.exists(path):
        return None
    with open(path) as f:
        return f.read()


def one(crate, folder, stem, check):
    code, out, err = run(crate, stem)
    name = f"{crate}/examples/{stem}"
    if code != 0:
        return [f"{name} exited {code}: {err.strip()[-200:]}"]
    if not out.strip():
        return [f"{name} printed nothing"]
    if len(out.encode()) > CAP:
        return [f"{name} printed {len(out.encode())} bytes, over {CAP}"]
    kind = ".svg" if out.lstrip().startswith("<svg") else ".txt"
    path = os.path.join(folder, stem + kind)
    if read(path) == out:
        return []
    if check:
        return [f"{name}{kind} is stale"]
    with open(path, "w") as f:
        f.write(out)
    other = os.path.join(folder, stem + (".txt" if kind == ".svg" else ".svg"))
    if os.path.exists(other):
        os.remove(other)
    return []


def main():
    words = sys.argv[1:]
    check = "--check" in words
    only = {word for word in words if word != "--check"}
    bad = []
    for crate, folder in folders():
        if only and crate not in only:
            continue
        names = os.listdir(folder)
        stems = sorted(name[:-3] for name in names if name.endswith(".rs"))
        for name in names:
            stem, kind = os.path.splitext(name)
            if kind in KINDS and stem not in stems:
                bad.append(f"{crate}/examples/{name} has no example")
        if len(stems) > MOST:
            bad.append(f"{crate} has {len(stems)} examples, over {MOST}")
        for stem in stems:
            bad.extend(one(crate, folder, stem, check))
    for line in bad:
        print(line)
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
