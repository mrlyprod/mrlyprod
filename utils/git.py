import os
import shutil
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from config import hex_key, run

DIR = os.path.expanduser("~/Developer/mrlyprod")
SLUG = "mrlyprod/mrlyprod"
DESC = "MrlyProd; or, Marley Productions"

# STATUS

def status():
    git_dir = os.path.join(DIR, ".git")
    if not os.path.exists(DIR):
        print("mrlyprod (missing)")
        return
    if not os.path.exists(git_dir):
        print("mrlyprod (no git)")
        return
    result = run(["git", "status", "--porcelain"], cwd=DIR)
    changes = len(result.stdout.strip().splitlines()) if result.stdout.strip() else 0
    if changes:
        print(f"mrlyprod ({changes} changes)")
    else:
        print("mrlyprod (clean)")

# CHECKS

CHECKS = [
    ["cargo", "test", "-p", "mrly"],
    ["uv", "run", "python", "utils/layers.py"],
    ["bunx", "tsc", "--noEmit", "--project", "apps/web"],
]

def checks():
    for cmd in CHECKS:
        result = run(cmd, cwd=DIR)
        if result.returncode != 0:
            tail = (result.stdout + result.stderr).strip().splitlines()[-20:]
            print(f"mrlyprod (check failed: {' '.join(cmd)})")
            print("\n".join(tail))
            return False
    return True

# PUSH

def push(fast=False):
    if not os.path.exists(DIR):
        print("mrlyprod (missing)")
        return
    if not fast and not checks():
        return
    run(["git", "add", "-A"], cwd=DIR)
    result = run(["git", "status", "--porcelain"], cwd=DIR)
    if not result.stdout.strip():
        print("mrlyprod (no changes)")
        return
    key = hex_key()
    run(["git", "commit", "-m", key], cwd=DIR)
    result = run(["git", "push", "-u", "origin", "main"], cwd=DIR)
    if result.returncode == 0:
        print(f"mrlyprod (pushed {key})")
    else:
        print(f"mrlyprod (failed: {result.stderr.strip()})")

# PUBLISH

def publish():
    if not os.path.exists(DIR):
        print("mrlyprod (missing)")
        return
    git_dir = os.path.join(DIR, ".git")
    if os.path.exists(git_dir):
        shutil.rmtree(git_dir)
    run(["git", "init"], cwd=DIR)
    run(["git", "add", "-A"], cwd=DIR)
    key = hex_key()
    run(["git", "commit", "-m", key], cwd=DIR)
    run(["git", "branch", "-M", "main"], cwd=DIR)
    run(["git", "remote", "add", "origin", f"https://github.com/{SLUG}.git"], cwd=DIR)
    result = run(["git", "push", "--force", "origin", "main"], cwd=DIR)
    if result.returncode == 0:
        print(f"mrlyprod (published {key})")
    else:
        print(f"mrlyprod (failed: {result.stderr.strip()})")

# SETUP

def setup():
    result = run(["gh", "repo", "view", SLUG])
    if result.returncode == 0:
        print(f"{SLUG} (exists)")
        return
    result = run(["gh", "repo", "create", SLUG, "--public"])
    if result.returncode == 0:
        print(f"{SLUG} (created)")
    else:
        print(f"{SLUG} (failed: {result.stderr.strip()})")

# DESCRIPTIONS

def desc():
    result = run(["gh", "repo", "edit", SLUG, f"--description={DESC}"])
    if result.returncode == 0:
        print(f"{SLUG} (description updated)")
    else:
        print(f"{SLUG} (failed: {result.stderr.strip()})")

# LOCKDOWN

def lockdown():
    result = run([
        "gh", "repo", "edit", SLUG,
        "--enable-issues=false",
        "--enable-wiki=false",
        "--enable-discussions=false",
        "--enable-projects=false",
    ])
    if result.returncode == 0:
        print(f"{SLUG} (locked)")
    else:
        print(f"{SLUG} (failed: {result.stderr.strip()})")

# VISIBILITY

def set_visibility(visibility):
    result = run(["gh", "repo", "edit", SLUG, f"--visibility={visibility}", "--accept-visibility-change-consequences"])
    if result.returncode == 0:
        print(f"{SLUG} ({visibility})")
    else:
        print(f"{SLUG} (failed: {result.stderr.strip()})")

def public():
    set_visibility("public")

def private():
    set_visibility("private")

# WIPE

def wipe():
    answer = input(f"Type '{SLUG}' to delete repo: ")
    if answer != SLUG:
        print("Cancelled.")
        return
    result = run(["gh", "repo", "delete", SLUG, "--yes"])
    if result.returncode == 0:
        print(f"{SLUG} (deleted)")
        git_dir = os.path.join(DIR, ".git")
        if os.path.exists(git_dir):
            shutil.rmtree(git_dir)
            print(".git removed.")
    else:
        print(f"Failed: {result.stderr.strip()}")

# TERMINAL

def help():
    commands = [
        ("status", "show working-tree change count"),
        ("push", "run the checks, stage all, commit, push to origin/main"),
        ("push fast", "push without the checks"),
        ("publish", "reset history, force-push a fresh main"),
        ("setup", "create the GitHub repo if missing"),
        ("desc", "set the repo description"),
        ("lockdown", "disable issues, wiki, discussions, projects"),
        ("public", "make the repo public"),
        ("private", "make the repo private"),
        ("wipe", "delete the repo (after confirm) and local .git"),
    ]
    width = max(len(name) for name, _ in commands)
    print("mrlygit")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["status"]: status()
        case ["push"]: push()
        case ["push", "fast"]: push(fast=True)
        case ["publish"]: publish()
        case ["setup"]: setup()
        case ["desc"]: desc()
        case ["lockdown"]: lockdown()
        case ["public"]: public()
        case ["private"]: private()
        case ["wipe"]: wipe()
        case _: help()

if __name__ == "__main__":
    terminal()
