import json

from common import (
    DIST_DIR, SITE_DIR,
    env, maybe, say, shell, stream, verb,
)

# RULES

DEV_BUCKET = env("MRLYDEV_BUCKET", "mrlydev")
BUILD = "build/net"

def grab(key):
    try:
        return shell("aws", "s3", "cp", f"s3://{DEV_BUCKET}/{key}", "-")
    except RuntimeError:
        return ""

# BUILD

def build():
    stream("bun", "run", "build", cwd=SITE_DIR)
    say(f"built into {DIST_DIR}")

# PLAN

def plan():
    stream("bun", "run", "push", "--dry", cwd=SITE_DIR)

# STATUS

def status():
    head = grab(f"{BUILD}/head").strip().split("\n")[0]
    say(f"head {head}" if head else "head none")
    text = grab(f"{BUILD}/manifest.json")
    if not text:
        say("manifest none")
        return
    data = json.loads(text)
    routes = [key for key in data if not key.startswith("@")]
    meta = maybe("s3api", "head-object", "--bucket", DEV_BUCKET, "--key", f"{BUILD}/manifest.json") or {}
    say(f"manifest {len(routes)} routes, {len(data) - len(routes)} assets, written {meta.get('LastModified', 'unknown')}")

# MAIN

VERBS = {"build": build, "plan": plan, "status": status}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
