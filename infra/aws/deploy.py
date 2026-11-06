import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
import threading
import time

from common import SITES, site_env
from lib import HERE, REPO, SHELL_ENV, aws, head, maybe, slug

LAYERS = os.path.join(HERE, "layers.py")
FN = os.path.join(HERE, "fn.py")
SITE = os.path.join(HERE, "site.py")

BUILDERS = {
    "mrly": {
        "function": "mrlynet-site",
        "layer": "mrlynet-node",
        "repo": REPO,
        "slug": slug(REPO),
    },
}

ENDED = re.compile(
    r"\b(?:done|unchanged|seen) [0-9a-f]{7}\b|Task timed out|^REPORT RequestId"
)
QUEUE_WAIT = 900
FLUSH = 40
POLL = 3


def rel(path):
    return os.path.relpath(path, HERE) if path.startswith("/") else path


def shown(cmd):
    return " ".join("python" if word == sys.executable else rel(word) for word in cmd)


def step(title, cmd, cwd, environ):
    print(f"== {title}")
    if subprocess.run(cmd, cwd=cwd, env=environ, check=False).returncode != 0:
        raise SystemExit(f"deploy stopped at {title}")


def layer(builder):
    check = [sys.executable, LAYERS, "check", builder["layer"]]
    if subprocess.run(check, cwd=HERE, env=SHELL_ENV, check=False).returncode == 0:
        return
    for title, tool, verb, name in [
        ("build", LAYERS, "build", builder["layer"]),
        ("publish", LAYERS, "publish", builder["layer"]),
        ("deploy", FN, "deploy", builder["function"]),
    ]:
        step(
            f"{title} {name}",
            [sys.executable, tool, verb, name, "--yes"],
            HERE,
            SHELL_ENV,
        )


LINE = re.compile(r'^(INFO|WARN|ERROR)\s+RequestId: \S+\s+"?(.*?)"?$')
SKIP = ("INIT_START", "END RequestId", "INIT_REPORT")


def tidy(message):
    if message.startswith(SKIP):
        return None
    if message.startswith("START RequestId"):
        return "START"
    if message.startswith("REPORT RequestId"):
        return "REPORT " + " ".join(part.strip() for part in message.split("\t")[1:])
    if message.startswith("{"):
        try:
            record = json.loads(message)
            return record.get("message") or json.dumps(record.get("record", record))
        except ValueError:
            return message
    found = LINE.match(message)
    return found.group(2) if found else message


class Tail:
    def __init__(self, name, sha):
        self.group = f"/aws/lambda/{name}"
        self.start = int(time.time() * 1000) - 2000
        self.printed = set()
        self.pinned = re.compile(rf"\b(?:done|unchanged|seen) {sha[:7]}\b")
        self.ended = threading.Event()
        self.landed = threading.Event()
        self.halt = threading.Event()
        self.thread = threading.Thread(target=self.loop, daemon=True)
        self.thread.start()

    def fetch(self):
        data = maybe(
            "logs",
            "filter-log-events",
            "--log-group-name",
            self.group,
            "--start-time",
            self.start,
        )
        for event in (data or {}).get("events", []):
            if event["eventId"] in self.printed:
                continue
            self.printed.add(event["eventId"])
            self.show(event)

    def show(self, event):
        stamp = time.strftime("%H:%M:%S", time.gmtime(event["timestamp"] / 1000))
        message = tidy(event["message"].rstrip())
        if message is None:
            return
        print(f"   {stamp} {message}", flush=True)
        if ENDED.search(message):
            self.ended.set()
        if self.pinned.search(message):
            self.landed.set()

    def loop(self):
        while not self.halt.is_set():
            self.fetch()
            self.halt.wait(POLL)

    def stop(self):
        self.ended.wait(FLUSH)
        self.halt.set()
        self.thread.join(timeout=POLL + 5)
        self.fetch()


def answer(text):
    try:
        body = json.loads(text)
    except ValueError:
        return text.strip()
    if isinstance(body, dict) and "body" in body:
        return str(body["body"]).strip()
    return str(body).strip()


BUSY = "TooManyRequestsException"


def call(name, out, kind, payload):
    try:
        reply = aws(
            "lambda",
            "invoke",
            "--function-name",
            name,
            "--payload",
            json.dumps(payload),
            "--invocation-type",
            kind,
            "--cli-binary-format",
            "raw-in-base64-out",
            "--cli-read-timeout",
            "0",
            out,
        )
        return reply or {}, ""
    except RuntimeError as error:
        return None, str(error)


def invoke(name, repo, slug):
    handle, out = tempfile.mkstemp(suffix=".json")
    os.close(handle)
    sha = head(repo)
    payload = {"source": "manual", "repo": slug, "sha": sha}
    print(f"   pinned {slug} at {sha[:7]}")
    tail = Tail(name, sha)
    try:
        reply, failed = call(name, out, "RequestResponse", payload)
        if failed and BUSY in failed:
            reply, failed = call(name, os.devnull, "Event", payload)
            if not failed:
                print(
                    f"   {name} is busy, queued behind the running build, tailing until it lands"
                )
                landed = tail.landed.wait(QUEUE_WAIT)
                return (
                    landed,
                    f"{name} at {sha[:7]}: queued run {'landed' if landed else 'did not land'}",
                )
        if failed:
            lines = failed.strip().splitlines()
            return (
                False,
                f"{name}: failed {lines[-1] if lines else 'aws lambda invoke'}",
            )
        error = f"{reply['FunctionError']} " if reply.get("FunctionError") else ""
        return not error, f"{name} at {sha[:7]}: {error}{answer(open(out).read())}"
    finally:
        tail.stop()
        os.remove(out)


def wake(builder):
    landed, text = invoke(builder["function"], builder["repo"], builder["slug"])
    print(text)
    if not landed:
        raise SystemExit(
            "deploy stopped at wake: function and headers stay as they were"
        )


def run(title, cmd, cwd=HERE, environ=SHELL_ENV, note=""):
    return {"title": title, "cmd": cmd, "cwd": cwd, "env": environ, "note": note}


def act(title, work, note):
    return {"title": title, "work": work, "note": note}


def edge(name, site):
    return [
        run(
            f"function {name}",
            [sys.executable, SITE, "function", name, "--yes"],
            note=site["router"],
        ),
        run(
            f"headers {name}",
            [sys.executable, SITE, "headers", name, "--yes"],
            note=site["headers"],
        ),
    ]


def pushed(builder):
    repo, slug = builder["repo"], builder["slug"]
    dirty = subprocess.run(
        ["git", "status", "--porcelain"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=False,
    ).stdout.strip()
    ahead = subprocess.run(
        ["git", "rev-list", "@{u}..HEAD"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=False,
    )
    if dirty or ahead.returncode != 0 or ahead.stdout.strip():
        raise SystemExit(
            f"refuse: {slug} has work GitHub lacks; Lambda builds the pushed head, so push first"
        )


def order(name):
    site = SITES[name]
    builder = BUILDERS.get(name)
    if not builder:
        push = run(
            f"push {name}",
            ["bun", "run", "push"],
            cwd=site["root"],
            environ=site_env(site),
            note=f"MRLY_ROOT {rel(site['dir'])}, bucket {site['bucket']}",
        )
        return [push] + edge(name, site)
    steps = [
        act(
            f"pushed {builder['slug']}",
            lambda: pushed(builder),
            "refuse unless GitHub holds every change; deploy never pushes",
        )
    ]
    steps.append(
        act(
            f"layer {builder['layer']}",
            lambda: layer(builder),
            f"layers.py check {builder['layer']}; stale: build, publish, fn.py deploy {builder['function']}",
        )
    )
    steps.append(
        act(
            f"wake {builder['function']}",
            lambda: wake(builder),
            f"lambda invoke {builder['function']} at the pushed head, bucket {site['bucket']}",
        )
    )
    return steps + edge(name, site)


def dry(number, one):
    line = f"{number}. {one['title']}"
    if "cmd" in one:
        line += f": {shown(one['cmd'])}"
        if one["cwd"] != HERE:
            line += f" in {rel(one['cwd'])}"
    if one["note"]:
        line += f" ({one['note']})"
    print(line)


def main():
    parser = argparse.ArgumentParser(
        prog="deploy",
        description="Ship a site. mrly: the layer check (stale: layers.py build and publish, fn.py deploy), a refusal unless its source repo is fully pushed, the wake of mrlynet-site at the pushed head with its log tailed live, then site.py function and headers. org and com: bun run push in the site's root, then site.py function and headers; no Lambda, no commit.",
    )
    parser.add_argument("site", choices=list(SITES))
    parser.add_argument(
        "--dry", action="store_true", help="print every step and run nothing"
    )
    args = parser.parse_args()
    for number, one in enumerate(order(args.site), 1):
        if args.dry:
            dry(number, one)
        elif "work" in one:
            print(f"== {one['title']}")
            one["work"]()
        else:
            step(one["title"], one["cmd"], one["cwd"], one["env"])


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        raise SystemExit("\ndeploy stopped")
