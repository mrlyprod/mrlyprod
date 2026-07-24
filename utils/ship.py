import os
import subprocess
import sys

CORE_DIR = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.dirname(CORE_DIR)

sys.path.insert(0, CORE_DIR)

from git import push

# HELPERS

def run(cmd):
    result = subprocess.run(cmd, cwd=DIR, capture_output=True, text=True)
    if result.returncode != 0:
        tail = (result.stdout + result.stderr).strip().splitlines()[-20:]
        print(f"mrlyprod (step failed: {' '.join(cmd)})")
        print("\n".join(tail))
    return result.returncode == 0

def changed():
    result = subprocess.run(["git", "status", "--porcelain"], cwd=DIR, capture_output=True, text=True)
    return [line[3:].strip().strip('"') for line in result.stdout.splitlines()]

def touches(paths, *prefixes):
    return any(path.startswith(prefix) for path in paths for prefix in prefixes)

# SHIP

def ship(fast=False):
    paths = changed()
    rust = touches(paths, "pkgs/", "Cargo.toml")
    web = touches(paths, "apps/web")
    if not fast:
        if rust:
            steps = [
                ["cargo", "run", "-p", "mrlynet", "--example", "fixtures"],
                ["cargo", "test", "--workspace"],
                ["uv", "run", "python", "utils/layers.py"],
                ["wasm-pack", "build", "pkgs/mrlyjs", "--target", "web"],
            ]
            for step in steps:
                if not run(step):
                    return
        if rust or web:
            steps = [
                ["bunx", "tsc", "--noEmit", "--project", "apps/web"],
                ["bun", "run", "apps/web/verify.ts"],
            ]
            for step in steps:
                if not run(step):
                    return
        if not rust and not web:
            print("mrlyprod (no source changed, gates skipped)")
        if not run(["uv", "run", "python", "utils/tree.py"]):
            return
    push(fast=True)

# TERMINAL

def help():
    commands = [
        ("(no args)", "gate what changed, rebuild, push"),
        ("fast", "push, no gates"),
    ]
    width = max(len(name) for name, _ in commands)
    print("mrlyship")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case []: ship()
        case ["fast"]: ship(fast=True)
        case _: help()

if __name__ == "__main__":
    terminal()
