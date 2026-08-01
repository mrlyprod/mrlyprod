import os
import random
import shutil
import subprocess
import sys

# CONFIG

SLUG = "mrlyprod/mrlyprod"
DESC = "MrlyProd; or, Marley Productions"
VISIBILITY = "public"

# HELPERS

DIR = os.path.dirname(os.path.abspath(__file__))
NAME = os.path.basename(DIR)

def run(cmd, cwd=None):
    try:
        return subprocess.run(cmd, cwd=cwd or DIR, capture_output=True, text=True)
    except FileNotFoundError:
        return subprocess.CompletedProcess(cmd, returncode=1, stdout="", stderr=f"{cmd[0]} not found")

def hex_key(k=8):
    return "".join(random.choices("0123456789abcdef", k=k))

# STATUS

def status():
    if not os.path.exists(os.path.join(DIR, ".git")):
        print(f"{NAME} (no git, run setup)")
        return
    result = run(["git", "status", "--porcelain"])
    changes = len(result.stdout.strip().splitlines()) if result.stdout.strip() else 0
    if changes:
        print(f"{NAME} ({changes} changes)")
    else:
        print(f"{NAME} (clean)")

# PUSH

def push():
    if not os.path.exists(os.path.join(DIR, ".git")):
        print(f"{NAME} (no git, run setup)")
        return
    run(["git", "add", "-A"])
    result = run(["git", "status", "--porcelain"])
    if not result.stdout.strip():
        print(f"{NAME} (no changes)")
        return
    key = hex_key()
    run(["git", "commit", "-m", key])
    result = run(["git", "push", "-u", "origin", "main"])
    if result.returncode == 0:
        print(f"{NAME} (pushed {key})")
    else:
        print(f"{NAME} (failed: {result.stderr.strip()})")

# PUBLISH

def publish():
    git_dir = os.path.join(DIR, ".git")
    if os.path.exists(git_dir):
        shutil.rmtree(git_dir)
    run(["git", "init"])
    run(["git", "add", "-A"])
    key = hex_key()
    run(["git", "commit", "-m", key])
    run(["git", "branch", "-M", "main"])
    run(["git", "remote", "add", "origin", f"https://github.com/{SLUG}.git"])
    result = run(["git", "push", "--force", "origin", "main"])
    if result.returncode == 0:
        print(f"{NAME} (published {key})")
    else:
        print(f"{NAME} (failed: {result.stderr.strip()})")

# SETUP

def setup():
    if not os.path.exists(os.path.join(DIR, ".git")):
        run(["git", "init"])
        run(["git", "branch", "-M", "main"])
    result = run(["gh", "repo", "view", SLUG])
    if result.returncode != 0:
        result = run(["gh", "repo", "create", SLUG, f"--{VISIBILITY}"])
        if result.returncode != 0:
            print(f"{NAME} (failed: {result.stderr.strip()})")
            return
        print(f"{NAME} ({SLUG} created)")
    run(["git", "remote", "remove", "origin"])
    run(["git", "remote", "add", "origin", f"https://github.com/{SLUG}.git"])
    print(f"{NAME} (ready)")

# ADMIN

def desc():
    if not DESC:
        print(f"{NAME} (no DESC set)")
        return
    result = run(["gh", "repo", "edit", SLUG, f"--description={DESC}"])
    if result.returncode == 0:
        print(f"{NAME} (description set)")
    else:
        print(f"{NAME} (failed: {result.stderr.strip()})")

def lockdown():
    result = run([
        "gh", "repo", "edit", SLUG,
        "--enable-issues=false",
        "--enable-wiki=false",
        "--enable-discussions=false",
        "--enable-projects=false",
    ])
    if result.returncode == 0:
        print(f"{NAME} (locked)")
    else:
        print(f"{NAME} (failed: {result.stderr.strip()})")

def set_visibility(visibility):
    result = run(["gh", "repo", "edit", SLUG, f"--visibility={visibility}", "--accept-visibility-change-consequences"])
    if result.returncode == 0:
        print(f"{NAME} ({visibility})")
    else:
        print(f"{NAME} (failed: {result.stderr.strip()})")

def public():
    set_visibility("public")

def private():
    set_visibility("private")

def wipe():
    answer = input(f"Type '{SLUG}' to delete the GitHub repo: ")
    if answer != SLUG:
        print("Cancelled.")
        return
    result = run(["gh", "repo", "delete", SLUG, "--yes"])
    if result.returncode == 0:
        print(f"{NAME} ({SLUG} deleted)")
        git_dir = os.path.join(DIR, ".git")
        if os.path.exists(git_dir):
            shutil.rmtree(git_dir)
            print(".git removed.")
    else:
        print(f"{NAME} (failed: {result.stderr.strip()})")

# TERMINAL

def help():
    commands = [
        ("status", "show working-tree change count"),
        ("push", "stage all, commit, push to origin/main"),
        ("publish", "wipe history, force-push a fresh main"),
        ("setup", "init git and create the GitHub repo if missing"),
        ("desc", "set the repo description"),
        ("lockdown", "disable issues, wiki, discussions, projects"),
        ("public", "make the repo public"),
        ("private", "make the repo private"),
        ("wipe", "delete the GitHub repo (after confirm) and local .git"),
    ]
    width = max(len(name) for name, _ in commands)
    print(NAME)
    print()
    for name, blurb in commands:
        print(f"  {name:<{width}}  {blurb}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["status"]: status()
        case ["push"]: push()
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
