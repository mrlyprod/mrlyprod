import json
import os

from common import SITES, gate, pick, say, site_env, verb
from lib import maybe, need, shell, stream


def rules(key):
    with open(os.path.join(SITES[key]["dir"], "site.json")) as handle:
        block = json.load(handle).get("push")
    if not block:
        raise SystemExit(f"refuse: {key} keeps no push block in site.json")
    return block


def guarded(key):
    block = rules(key)
    return [
        f"every object under {block['prefix']}{one} is left alone"
        for one in block["guard"]
    ]


def where(key):
    store = rules(key)["store"]
    return need(store["bucket"]), store["prefix"]


def grab(key, name):
    store, prefix = where(key)
    try:
        return shell("aws", "s3", "cp", f"s3://{store}/{prefix}/{name}", "-")
    except RuntimeError:
        return ""


def routes(key):
    path = os.path.join(SITES[key]["dir"], "dist", "routes.json")
    if not os.path.exists(path):
        return None
    with open(path) as handle:
        data = json.load(handle)
    return len(data["rows"] if isinstance(data, dict) else data)


def build():
    key = pick(SITES, "site")
    site = SITES[key]
    if not gate(
        f"build {key}", [f"bun run build in {site['root']}, MRLY_ROOT {site['dir']}"]
    ):
        return
    stream("bun", "run", "build", cwd=site["root"], environ=site_env(site))


def plan():
    site = SITES[pick(SITES, "site")]
    stream("bun", "run", "push", "--dry", cwd=site["root"], environ=site_env(site))


def push():
    key = pick(SITES, "site")
    site = SITES[key]
    steps = [
        f"bun run push in {site['root']}, MRLY_ROOT {site['dir']}: build against the remote manifest, upload what changed, delete what went"
    ]
    steps += guarded(key)
    if not gate(f"push {key}", steps):
        return
    stream("bun", "run", "push", cwd=site["root"], environ=site_env(site))


def status():
    key = pick(SITES, "site")
    store, prefix = where(key)
    head = grab(key, "head").strip().split("\n")[0]
    say(f"head {head}" if head else "head none")
    text = grab(key, "manifest.json")
    if not text:
        say("manifest none")
        return
    data = json.loads(text)
    meta = (
        maybe(
            "s3api",
            "head-object",
            "--bucket",
            store,
            "--key",
            f"{prefix}/manifest.json",
        )
        or {}
    )
    say(f"manifest {len(data)} rows, written {meta.get('LastModified', 'unknown')}")
    local = routes(key)
    say(
        f"routes {local} in dist/routes.json"
        if local is not None
        else "routes none, run build"
    )
    say(f"proof         curl -I https://{SITES[key]['apex']}/")


VERBS = {
    "build": (
        build,
        "<site>",
        "bun run build in the site's root, MRLY_ROOT at its dir",
    ),
    "plan": (
        plan,
        "<site>",
        "bun run push --dry in the site's root: what would upload and what would go",
    ),
    "push": (
        push,
        "<site>",
        "bun run push: build against the remote manifest, upload what changed; a removed file other than a page stays a week",
    ),
    "status": (
        status,
        "<site>",
        "read-only: the pushed head, the remote manifest and the local routes",
    ),
}

ABOUT = "A site's content from this Mac, from the root and dir of its SITES row in common.py and the push block of its site.json. A mutating verb prints a PLAN; --yes applies it."

if __name__ == "__main__":
    verb(VERBS, ABOUT)()
