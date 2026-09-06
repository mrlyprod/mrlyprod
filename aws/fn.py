import base64
import json
import os
import zipfile

from common import (
    ACCOUNT, AWS_DIR, BUN_LAYER, DEV_BUCKET, NET_FUNCTION, PROD_BUCKET, REGION,
    REPO, ROLE_NAME, SITE_BUCKET, aws, gate, maybe, say, shell, stream, verb,
)

# DESIRED

RUNTIME = "provided.al2"
ARCH = "arm64"
HANDLER = "handler.fetch"
TIMEOUT = 900
MEMORY = 3008
STORAGE = 2048
RETRIES = 0
CONCURRENCY = 1

SOURCE = os.path.join(AWS_DIR, "net.ts")
BUILD_DIR = os.path.join(REPO, "data", "fn")
BUNDLE = os.path.join(BUILD_DIR, "handler.js")
ZIP_PATH = os.path.join(BUILD_DIR, f"{NET_FUNCTION}.zip")
LOG_GROUP = f"/aws/lambda/{NET_FUNCTION}"

def role_arn():
    return f"arn:aws:iam::{ACCOUNT}:role/{ROLE_NAME}"

def environment():
    return {"Variables": {
        "MRLYNET_BUCKET": SITE_BUCKET,
        "MRLYDEV_BUCKET": DEV_BUCKET,
        "MRLYPROD_BUCKET": PROD_BUCKET,
    }}

def config_args():
    return [
        "--role", role_arn(),
        "--handler", HANDLER,
        "--runtime", RUNTIME,
        "--timeout", TIMEOUT,
        "--memory-size", MEMORY,
        "--ephemeral-storage", json.dumps({"Size": STORAGE}),
        "--layers", BUN_LAYER,
        "--environment", json.dumps(environment()),
    ]

# GUARDS

def need_layer():
    if not BUN_LAYER:
        raise SystemExit(
            "refuse: BUN_LAYER is empty, publish it once with "
            "layers.py publish --yes and paste the arn into common.py")

def need_role():
    if not maybe("iam", "get-role", "--role-name", ROLE_NAME):
        raise SystemExit(f"refuse: role {ROLE_NAME} is missing, run iam.py role --yes first")

def need_zip():
    if not os.path.exists(ZIP_PATH):
        raise SystemExit(f"refuse: {ZIP_PATH} is missing, run fn.py bundle first")
    return "fileb://" + ZIP_PATH

def live():
    return bool(maybe("lambda", "get-function", "--function-name", NET_FUNCTION, region=REGION))

# VERBS

def bundle():
    os.makedirs(BUILD_DIR, exist_ok=True)
    shell("bun", "build", SOURCE, "--target=bun", "--outfile", BUNDLE, cwd=REPO)
    with zipfile.ZipFile(ZIP_PATH, "w", zipfile.ZIP_DEFLATED) as archive:
        archive.write(BUNDLE, "handler.js")
    say(f"{ZIP_PATH} {os.path.getsize(ZIP_PATH) / 1024:.0f} KB from {os.path.getsize(BUNDLE) / 1024:.0f} KB of js")

def deploy():
    need_layer()
    need_role()
    there = live()
    if not gate("deploy", [
        f"{'update' if there else 'create'} function {NET_FUNCTION} in {REGION}",
        f"{RUNTIME} on {ARCH} with layer {BUN_LAYER}",
        f"handler {HANDLER}, role {ROLE_NAME}",
        f"timeout {TIMEOUT}s, memory {MEMORY} MB, ephemeral storage {STORAGE} MB",
        f"async retries {RETRIES}, reserved concurrency {CONCURRENCY}",
        f"code from {ZIP_PATH}, rebuilt first",
    ]): return
    bundle()
    code = need_zip()
    if there:
        aws("lambda", "update-function-code", "--function-name", NET_FUNCTION,
            "--zip-file", code, "--architectures", ARCH, "--publish", region=REGION)
        aws("lambda", "wait", "function-updated", "--function-name", NET_FUNCTION, region=REGION)
        aws("lambda", "update-function-configuration", "--function-name", NET_FUNCTION,
            *config_args(), region=REGION)
        aws("lambda", "wait", "function-updated", "--function-name", NET_FUNCTION, region=REGION)
        say(f"{NET_FUNCTION} code and configuration updated")
    else:
        aws("lambda", "create-function", "--function-name", NET_FUNCTION,
            "--zip-file", code, "--architectures", ARCH, "--publish",
            *config_args(), region=REGION)
        aws("lambda", "wait", "function-active", "--function-name", NET_FUNCTION, region=REGION)
        say(f"{NET_FUNCTION} created")
    aws("lambda", "put-function-event-invoke-config", "--function-name", NET_FUNCTION,
        "--maximum-retry-attempts", RETRIES, region=REGION)
    aws("lambda", "put-function-concurrency", "--function-name", NET_FUNCTION,
        "--reserved-concurrent-executions", CONCURRENCY, region=REGION)
    say(f"retries {RETRIES}, reserved concurrency {CONCURRENCY}")

def invoke():
    if not gate("invoke", [
        f"synchronous invoke of {NET_FUNCTION} in {REGION}",
        "a no-op run answers in under a second",
        "otherwise it builds the site and uploads what changed",
    ]): return
    os.makedirs(BUILD_DIR, exist_ok=True)
    out = os.path.join(BUILD_DIR, "invoke.json")
    text = shell("aws", "lambda", "invoke", "--function-name", NET_FUNCTION,
                 "--cli-read-timeout", "0", "--log-type", "Tail",
                 "--cli-binary-format", "raw-in-base64-out",
                 "--payload", json.dumps({"source": "manual"}),
                 "--region", REGION, "--output", "json", out)
    data = json.loads(text or "{}")
    say(f"status {data.get('StatusCode')} {data.get('FunctionError') or 'ok'}")
    for line in base64.b64decode(data.get("LogResult", "")).decode().splitlines():
        say(line)
    with open(out) as handle:
        say(handle.read().strip())

def logs():
    stream("aws", "logs", "tail", LOG_GROUP, "--since", "1h", "--format", "short", "--region", REGION)

# MAIN

VERBS = {"bundle": bundle, "deploy": deploy, "invoke": invoke, "logs": logs}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
