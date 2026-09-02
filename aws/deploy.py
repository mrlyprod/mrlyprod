import os

from common import (
    DIST_DIR, SITE_BUCKET, SITE_DIR,
    aws, gate, say, shell, site_id, stream, verb,
)

# RULES

TARGET = f"s3://{SITE_BUCKET}"
IMMUTABLE = "public, max-age=31536000, immutable"
REVALIDATE = "public, max-age=0, must-revalidate"
HASHED = ["lib-*.js", "lib-*.css", "*.wasm", "*-????????.*"]
SKIPPED = ["_shots/*", ".DS_Store"]
TYPES = [
    ("*.wasm", "application/wasm", IMMUTABLE),
    ("*.svg", "image/svg+xml", REVALIDATE),
    ("*.xml", "application/xml", REVALIDATE),
    ("*.webmanifest", "application/manifest+json", REVALIDATE),
    ("*.txt", "text/plain", REVALIDATE),
]

def need_dist():
    if not os.path.isdir(DIST_DIR):
        raise SystemExit(f"refuse: {DIST_DIR} is missing, run build first")

def need_distribution():
    dist = site_id()
    if not dist:
        raise SystemExit("refuse: no mrlyprod.org distribution, run site.py distribution first")
    return dist

# BUILD

def build():
    stream("bun", "run", "build", cwd=SITE_DIR)
    say(f"built into {DIST_DIR}")

# SYNC

def sync_hashed(extra):
    args = ["aws", "s3", "sync", DIST_DIR, TARGET, "--delete",
            "--cache-control", IMMUTABLE, "--exclude", "*"]
    for pattern in HASHED: args += ["--include", pattern]
    for pattern in SKIPPED: args += ["--exclude", pattern]
    return shell(*args, *extra)

def sync_rest(extra):
    args = ["aws", "s3", "sync", DIST_DIR, TARGET, "--delete",
            "--cache-control", REVALIDATE]
    for pattern in HASHED + SKIPPED: args += ["--exclude", pattern]
    return shell(*args, *extra)

def retype():
    for pattern, kind, cache in TYPES:
        shell("aws", "s3", "cp", TARGET + "/", TARGET + "/", "--recursive",
              "--exclude", "*", "--include", pattern,
              "--content-type", kind, "--cache-control", cache,
              "--metadata-directive", "REPLACE")
        say(f"content-type {kind} on {pattern}")

# VERBS

def plan():
    need_dist()
    say(f"PLAN push {DIST_DIR} to {TARGET}")
    say(f"  pass 1 {IMMUTABLE}")
    say(sync_hashed(["--dryrun"]).rstrip())
    say(f"  pass 2 {REVALIDATE}")
    say(sync_rest(["--dryrun"]).rstrip())
    say("  then content-type fixups and one /* invalidation")

def push():
    dist = need_distribution()
    if not gate("push", [
        f"bun run build in {SITE_DIR}",
        f"pass 1 hashed assets to {TARGET} with {IMMUTABLE}",
        f"pass 2 everything else to {TARGET} with {REVALIDATE}",
        "both passes carry --delete so stale keys go",
        "content-type fixups for wasm, svg, xml, webmanifest, txt",
        f"invalidate /* on {dist}",
    ]): return
    build()
    say(sync_hashed([]).rstrip())
    say(sync_rest([]).rstrip())
    retype()
    data = aws("cloudfront", "create-invalidation", "--distribution-id", dist, "--paths", "/*")
    say(f"invalidation {data['Invalidation']['Id']} {data['Invalidation']['Status']}")

def status():
    dist = need_distribution()
    data = aws("cloudfront", "list-invalidations", "--distribution-id", dist)
    items = (data.get("InvalidationList") or {}).get("Items") or []
    if not items:
        say(f"{dist} has no invalidations")
        return
    last = items[0]
    full = aws("cloudfront", "get-invalidation", "--distribution-id", dist, "--id", last["Id"])
    paths = full["Invalidation"]["InvalidationBatch"]["Paths"]["Items"]
    say(f"{dist} {last['Id']} {last['Status']} {last['CreateTime']} {' '.join(paths)}")

# MAIN

VERBS = {"build": build, "plan": plan, "push": push, "status": status}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
