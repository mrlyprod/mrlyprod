import os
import subprocess
import sys
import time

CORE_DIR = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.dirname(CORE_DIR)

# HELPERS

def run(label, cmd, loud=False, env=None):
    start = time.monotonic()
    merged = None if env is None else {**os.environ, **env}
    if loud:
        print(f"  {label}", flush=True)
        result = subprocess.run(cmd, cwd=DIR, env=merged)
    else:
        print(f"  {label:<10}", end="", flush=True)
        result = subprocess.run(cmd, cwd=DIR, env=merged, capture_output=True, text=True)
    secs = time.monotonic() - start
    head = f"  {label:<10}" if loud else ""
    if result.returncode == 0:
        print(f"{head}{secs:>6.1f}s")
        return True
    print(f"{head}{'failed':>7} ({secs:.1f}s)")
    if not loud:
        tail = (result.stdout + result.stderr).strip().splitlines()[-20:]
        print("\n".join(tail))
    return False

def changed():
    paths = set()
    status = subprocess.run(["git", "status", "--porcelain"], cwd=DIR, capture_output=True, text=True)
    paths.update(line[3:].strip().strip('"') for line in status.stdout.splitlines())
    ahead = subprocess.run(["git", "diff", "--name-only", "@{u}...HEAD"], cwd=DIR, capture_output=True, text=True)
    if ahead.returncode == 0:
        paths.update(line.strip() for line in ahead.stdout.splitlines() if line.strip())
    return sorted(paths)

def touches(paths, *prefixes):
    return any(path.startswith(prefix) for path in paths for prefix in prefixes)

# TEST

def test(loud=False, record=False):
    paths = changed()
    config = touches(paths, "utils/")
    ts = config or touches(paths, "package.json", "bun.lock", "tsconfig.base.json")
    rust = config or touches(paths, "pkgs/", "apps/cli", "Cargo.toml")
    sites = ts or touches(paths, "apps/git/", "pkgs/mrlyui", "pkgs/mrlygpu")
    net = ts or touches(paths, "apps/net/", "pkgs/mrlycss", "pkgs/mrlydom")
    ui = ts or touches(paths, "pkgs/mrlyui")
    web = ts or touches(paths, "apps/web")
    print("mrlytest")
    if not run("fmt", ["cargo", "fmt"], loud):
        return False
    if rust:
        if record and not run("record", ["cargo", "run", "-p", "mrlyweb", "--example", "fixtures"], loud):
            return False
        if record and not run("bake", ["cargo", "run", "-p", "mrlyweb", "--example", "bake"], loud):
            return False
        if record and not run("frames", ["cargo", "run", "-p", "mrlycli", "--", "frame", "--record"], loud):
            return False
        steps = [
            ("test", ["cargo", "test", "--workspace"]),
            ("layers", ["uv", "run", "python", "utils/layers.py"]),
            ("doors", ["uv", "run", "python", "utils/doors.py"]),
            ("wasm", ["wasm-pack", "build", "pkgs/mrlyjs/web", "--target", "web"]),
            ("maturin", ["uv", "run", "maturin", "develop", "--manifest-path", "pkgs/mrlypy/web/Cargo.toml", "--release"]),
            ("pytest", ["uv", "run", "python", "-m", "pytest", "pkgs/mrlypy/web/tests", "-q"]),
        ]
        for label, cmd in steps:
            if not run(label, cmd, loud):
                return False
    if ts:
        if not run("tsc shot", ["bunx", "tsc", "--noEmit", "--project", "utils"], loud):
            return False
    if sites:
        steps = [
            ("tsc git", ["bun", "run", "--cwd", "apps/git", "check"], None),
            ("vite git", ["bun", "run", "--cwd", "apps/git", "site"], {"MRLY_OUT": "../../data/git/check"}),
            ("data git", ["bun", "run", "--cwd", "apps/git", "data"], {"MRLY_OUT": "../../data/git/check"}),
            ("links git", ["bun", "run", "--cwd", "apps/git", "links"], None),
        ]
        for label, cmd, env in steps:
            if not run(label, cmd, loud, env):
                return False
    if net:
        steps = [
            ("tsc net", ["bun", "run", "--cwd", "apps/net", "check"], None),
            ("vite net", ["bun", "run", "--cwd", "apps/net", "site"], {"MRLY_OUT": "../../data/net/check"}),
            ("data net", ["bun", "run", "--cwd", "apps/net", "data"], {"MRLY_OUT": "../../data/net/check"}),
            ("links net", ["bun", "run", "--cwd", "apps/net", "links"], None),
        ]
        for label, cmd, env in steps:
            if not run(label, cmd, loud, env):
                return False
    if ui:
        steps = [
            ("tsc ui", ["bun", "run", "--cwd", "pkgs/mrlyui", "check"], None),
            ("vite ui", ["bun", "run", "--cwd", "pkgs/mrlyui", "site"], {"MRLY_OUT": "../../../data/ui/check"}),
        ]
        for label, cmd, env in steps:
            if not run(label, cmd, loud, env):
                return False
    if rust or web:
        if not os.path.exists(os.path.join(DIR, "pkgs", "mrlyjs", "web", "pkg", "mrlyjs.d.ts")):
            if not run("wasm", ["wasm-pack", "build", "pkgs/mrlyjs/web", "--target", "web"], loud):
                return False
        steps = [
            ("tsc web", ["bun", "run", "--cwd", "apps/web", "check"]),
            ("verify", ["bun", "run", "apps/web/verify.ts"]),
        ]
        for label, cmd in steps:
            if not run(label, cmd, loud):
                return False
    if not rust and not sites and not net and not ui and not web:
        print("mrlyprod (no source changed, gates skipped)")
    if not run("tree", ["uv", "run", "python", "utils/tree.py"], loud):
        return False
    if not run("stats", ["uv", "run", "python", "utils/stats.py"], loud):
        return False
    print("mrlyprod (green)")
    return True

# TERMINAL

def help():
    commands = [
        ("(no args)", "gate what changed, rebuild, regenerate tree + stats"),
        ("loud", "the same, streaming each command's output"),
        ("record", "re-pin the golden fixtures first, then gate; only after a deliberate vocabulary change"),
    ]
    width = max(len(name) for name, _ in commands)
    print("mrlytest")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case []: sys.exit(0 if test() else 1)
        case ["loud"]: sys.exit(0 if test(loud=True) else 1)
        case ["record"]: sys.exit(0 if test(record=True) else 1)
        case ["record", "loud"] | ["loud", "record"]: sys.exit(0 if test(loud=True, record=True) else 1)
        case _: help()

if __name__ == "__main__":
    terminal()
