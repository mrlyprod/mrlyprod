import json
import os
import shutil
import subprocess
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor

CORE_DIR = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.dirname(CORE_DIR)
LOCK = threading.Lock()

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

def step(label, cmd, env, out):
    if out is None:
        return run(label, cmd, True, env)
    start = time.monotonic()
    merged = None if env is None else {**os.environ, **env}
    try:
        result = subprocess.run(cmd, cwd=DIR, env=merged, capture_output=True, text=True)
    except OSError as err:
        out.append(f"  {label:<10}{'failed':>7} ({err})")
        return False
    secs = time.monotonic() - start
    if result.returncode == 0:
        out.append(f"  {label:<10}{secs:>6.1f}s")
        return True
    out.append(f"  {label:<10}{'failed':>7} ({secs:.1f}s)")
    out.extend((result.stdout + result.stderr).strip().splitlines()[-20:])
    return False

def chain(steps, grounded, out):
    for label, cmd, env in steps:
        if grounded.is_set():
            return False
        if not step(label, cmd, env, out):
            grounded.set()
            return False
    return True

def landed(fn, grounded):
    out = []
    try:
        ok = fn(out)
    except BaseException as err:
        grounded.set()
        out.append(f"  {err!r}")
        ok = False
    if out:
        with LOCK:
            print("\n".join(out), flush=True)
    return ok

def changed():
    paths = set()
    status = subprocess.run(["git", "status", "--porcelain"], cwd=DIR, capture_output=True, text=True)
    paths.update(line[3:].strip().split(" -> ")[-1].strip('"') for line in status.stdout.splitlines())
    ahead = subprocess.run(["git", "diff", "--name-only", "@{u}...HEAD"], cwd=DIR, capture_output=True, text=True)
    if ahead.returncode != 0:
        ahead = subprocess.run(["git", "diff", "--name-only", "origin/main...HEAD"], cwd=DIR, capture_output=True, text=True)
    if ahead.returncode == 0:
        paths.update(line.strip() for line in ahead.stdout.splitlines() if line.strip())
    return sorted(paths)

def touches(paths, *prefixes):
    return any(path.startswith(prefix) for path in paths for prefix in prefixes)

def bootstrapped():
    if os.path.isdir(os.path.join(DIR, "node_modules")):
        return True
    print("run bun install")
    return False

# TEST

def test(loud=False, record=False):
    paths = changed()
    config = touches(paths, "utils/test.py", "utils/layers.py", "utils/doors.py", "utils/license.py")
    ts = config or touches(paths, "package.json", "bun.lock", "tsconfig.base.json")
    rust = config or touches(paths, "pkgs/", "Cargo.toml")
    sites = ts or touches(paths, "sites/git/", "pkgs/js/mrlyui")
    net = ts or touches(paths, "sites/net/", "pkgs/js/mrlyui")
    bot = ts or touches(paths, "sites/bot/", "pkgs/js/mrlyui")
    ui = ts or touches(paths, "pkgs/js/mrlyui")
    web = ts or touches(paths, "sites/web", "pkgs/js/mrlyui", "pkgs/js/mrlygpu")
    if (rust or ts or sites or net or bot or ui or web) and not bootstrapped():
        return False
    print("mrlytest")
    if not run("fmt", ["cargo", "fmt"], loud):
        return False
    if rust and record:
        pins = [
            ("record", ["cargo", "run", "-p", "mrlyweb", "--example", "fixtures"]),
            ("bake", ["cargo", "run", "-p", "mrlyweb", "--example", "bake"]),
            ("frames", ["cargo", "run", "-p", "mrlycli", "--", "frame", "--record"]),
            ("shots", ["cargo", "run", "-p", "mrlycli", "--", "shot", "--record"]),
        ]
        for label, cmd in pins:
            if not run(label, cmd, loud):
                return False
    grounded = threading.Event()
    wasm_ready = threading.Event()
    wasm = ("wasm", ["wasm-pack", "build", "pkgs/js/web", "--target", "web", "--out-name", "mrlyweb"], None)
    lanes = []
    if rust:
        head = []
        if shutil.which("cargo-nextest"):
            head += [("test", ["cargo", "nextest", "run", "--workspace"], None), ("doc", ["cargo", "test", "--doc", "--workspace"], None)]
        else:
            head += [("test", ["cargo", "test", "--workspace"], None)]
        head += [
            ("clippy", ["cargo", "clippy", "--workspace", "--all-targets", "--", "-D", "warnings"], None),
            ("layers", ["uv", "run", "python", "utils/layers.py"], None),
            ("doors", ["uv", "run", "python", "utils/doors.py"], None),
            ("license", ["uv", "run", "python", "utils/license.py"], None),
            wasm,
        ]
        tail = [
            ("maturin", ["uv", "run", "maturin", "develop", "--manifest-path", "pkgs/py/web/Cargo.toml", "--profile", "gate"], None),
            ("pytest", ["uv", "run", "python", "-m", "pytest", "pkgs/py/web/tests", "-q"], None),
        ]
        def rust_lane(out):
            if not chain(head, grounded, out):
                return False
            wasm_ready.set()
            return chain(tail, grounded, out)
        lanes.append(rust_lane)
    if ts:
        shot_steps = [("tsc shot", ["bunx", "tsc", "--noEmit", "--project", "utils"], None)]
        lanes.append(lambda out: chain(shot_steps, grounded, out))
    if sites:
        git_steps = [
            ("tsc git", ["bun", "run", "--cwd", "sites/git", "check"], None),
            ("vite git", ["bun", "run", "--cwd", "sites/git", "site"], {"MRLY_OUT": "../../data/git/check"}),
            ("data git", ["bun", "run", "--cwd", "sites/git", "data"], {"MRLY_OUT": "../../data/git/check"}),
            ("links git", ["bun", "run", "--cwd", "sites/git", "links"], None),
        ]
        lanes.append(lambda out: chain(git_steps, grounded, out))
    if net:
        net_steps = [
            ("tsc net", ["bun", "run", "--cwd", "sites/net", "check"], None),
            ("vite net", ["bun", "run", "--cwd", "sites/net", "site"], {"MRLY_OUT": "../../data/net/check"}),
            ("data net", ["bun", "run", "--cwd", "sites/net", "data"], {"MRLY_OUT": "../../data/net/check"}),
            ("links net", ["bun", "run", "--cwd", "sites/net", "links"], None),
        ]
        lanes.append(lambda out: chain(net_steps, grounded, out))
    if bot:
        bot_steps = [
            ("tsc bot", ["bun", "run", "--cwd", "sites/bot", "check"], None),
            ("vite bot", ["bun", "run", "--cwd", "sites/bot", "site"], {"MRLY_OUT": "../../data/bot/check"}),
        ]
        lanes.append(lambda out: chain(bot_steps, grounded, out))
    if ui:
        ui_steps = [
            ("tsc ui", ["bun", "run", "--cwd", "pkgs/js/mrlyui", "check"], None),
            ("vite ui", ["bun", "run", "--cwd", "pkgs/js/mrlyui", "site"], {"MRLY_OUT": "../../../../data/ui/check"}),
        ]
        lanes.append(lambda out: chain(ui_steps, grounded, out))
    if rust or web:
        web_steps = [
            ("tsc web", ["bun", "run", "--cwd", "sites/web", "check"], None),
            ("verify", ["bun", "run", "sites/web/verify.ts"], None),
            ("site web", ["bun", "run", "--cwd", "sites/web", "site"], None),
        ]
        def web_lane(out):
            if rust:
                while not wasm_ready.wait(0.2):
                    if grounded.is_set():
                        return False
            elif not os.path.exists(os.path.join(DIR, "pkgs", "js", "web", "pkg", "mrlyweb.d.ts")):
                if not chain([wasm], grounded, out):
                    return False
            return chain(web_steps, grounded, out)
        lanes.append(web_lane)
    if loud:
        for fn in lanes:
            if not fn(None):
                return False
    elif lanes:
        with ThreadPoolExecutor(len(lanes)) as pool:
            futures = [pool.submit(landed, fn, grounded) for fn in lanes]
            try:
                oks = [future.result() for future in futures]
            except BaseException:
                grounded.set()
                raise
        if not all(oks):
            return False
    if not rust and not sites and not net and not bot and not ui and not web:
        print("mrlyprod (no source changed, gates skipped)")
    if not run("tree", ["uv", "run", "python", "utils/tree.py"], loud):
        return False
    if not run("stats", ["uv", "run", "python", "utils/stats.py"], loud):
        return False
    print("mrlyprod (green)")
    return True

# FAST

CHECKS = [
    ("tsc git", "sites/git", ("sites/git/", "pkgs/js/mrlyui")),
    ("tsc net", "sites/net", ("sites/net/", "pkgs/js/mrlyui")),
    ("tsc bot", "sites/bot", ("sites/bot/", "pkgs/js/mrlyui")),
    ("tsc ui", "pkgs/js/mrlyui", ("pkgs/js/mrlyui",)),
    ("tsc web", "sites/web", ("sites/web", "pkgs/js/mrlyui", "pkgs/js/mrlygpu")),
]

def crates_of(paths):
    found = set()
    for path in paths:
        parts = path.split("/")
        if path.startswith("pkgs/rs/apps/") and len(parts) > 3:
            found.add(parts[3])
        elif path.startswith("pkgs/rs/") and len(parts) > 2:
            found.add(parts[2])
        elif path.startswith("pkgs/js/web"):
            found.add("mrlyweb-js")
        elif path.startswith("pkgs/py/web"):
            found.add("mrlyweb-py")
    return found

def dependents(seeds):
    meta = subprocess.run(["cargo", "metadata", "--format-version", "1", "--no-deps"], cwd=DIR, capture_output=True, text=True)
    if meta.returncode != 0:
        print(meta.stderr.strip())
        return None
    packages = json.loads(meta.stdout)["packages"]
    names = {p["name"] for p in packages}
    edges = {p["name"]: {d["name"] for d in p["dependencies"] if d["name"] in names} for p in packages}
    crates = seeds & names
    grown = True
    while grown:
        grown = False
        for name, deps in edges.items():
            if name not in crates and deps & crates:
                crates.add(name)
                grown = True
    return sorted(crates)

def fast(loud=False):
    paths = changed()
    crates = crates_of(paths)
    projects = [(label, proj) for label, proj, prefixes in CHECKS if touches(paths, *prefixes)]
    if projects and not bootstrapped():
        return False
    print("mrlytest (fast)")
    if not run("fmt", ["cargo", "fmt"], loud):
        return False
    if crates:
        crates = dependents(crates)
        if crates is None:
            return False
    if crates:
        flags = [flag for name in crates for flag in ("-p", name)]
        runner = ["cargo", "nextest", "run"] if shutil.which("cargo-nextest") else ["cargo", "test"]
        if not run("test", runner + flags, loud):
            return False
        if not run("clippy", ["cargo", "clippy", *flags, "--all-targets", "--", "-D", "warnings"], loud):
            return False
    for label, proj in projects:
        if not run(label, ["bun", "run", "--cwd", proj, "check"], loud):
            return False
    if not crates and not projects:
        print("mrlyprod (no source changed, gates skipped)")
        return True
    print("mrlyprod (green)")
    return True

# TERMINAL

def help():
    commands = [
        ("(no args)", "gate what changed, rebuild, regenerate tree + stats"),
        ("loud", "the same, streaming each command's output"),
        ("fast", "fmt + targeted tests, clippy, tsc for what changed; no rebuilds"),
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
        case ["fast"]: sys.exit(0 if fast() else 1)
        case ["fast", "loud"] | ["loud", "fast"]: sys.exit(0 if fast(loud=True) else 1)
        case ["record"]: sys.exit(0 if test(record=True) else 1)
        case ["record", "loud"] | ["loud", "record"]: sys.exit(0 if test(loud=True, record=True) else 1)
        case _:
            help()
            sys.exit(2)

if __name__ == "__main__":
    terminal()
