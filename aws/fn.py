import base64
import json
import os
import sys
import time
import zipfile

from common import FUNCTIONS, gate, given, pick, role_arn, say, verb
from lib import (
    DATA_DIR,
    HERE,
    aws,
    env,
    maybe,
    need,
    save_env,
    shell,
    stream,
    tmpfile,
)

FILE_MODE = 0o644 << 16
SKIP = ("__pycache__", ".pyc", ".DS_Store")


def build_dir(key):
    path = os.path.join(DATA_DIR, "functions", key)
    os.makedirs(path, exist_ok=True)
    return path


def package_path(key):
    return os.path.join(build_dir(key), f"{FUNCTIONS[key]['name']}.zip")


def put(archive, arc, data):
    if any(mark in arc for mark in SKIP):
        return 0
    info = zipfile.ZipInfo(arc, (1980, 1, 1, 0, 0, 0))
    info.external_attr = FILE_MODE
    info.compress_type = zipfile.ZIP_DEFLATED
    archive.writestr(info, data)
    return 1


def add(archive, full, arc):
    with open(full, "rb") as handle:
        return put(archive, arc, handle.read())


def package_zip(archive, sources):
    missing = [
        os.path.relpath(source, HERE)
        for source, _ in sources
        if not os.path.exists(source)
    ]
    if missing:
        raise SystemExit("refuse: missing " + " ".join(missing))
    count = 0
    for source, arc in sources:
        if os.path.isfile(source):
            count += add(archive, source, arc)
            continue
        for folder, _, files in sorted(os.walk(source)):
            for file in sorted(files):
                full = os.path.join(folder, file)
                count += add(archive, full, f"{arc}/{os.path.relpath(full, source)}")
    return count


def bun_root(source):
    return os.path.dirname(os.path.dirname(source))


def package_bun(archive, source, key):
    bundle = os.path.abspath(os.path.join(build_dir(key), "handler.js"))
    shell(
        "bun",
        "build",
        source,
        "--target=bun",
        "--outfile",
        bundle,
        cwd=bun_root(source),
    )
    return add(archive, bundle, "handler.js")


def package(key):
    kind, source = FUNCTIONS[key]["package"]
    path = package_path(key)
    if os.path.exists(path):
        os.remove(path)
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as archive:
        count = (
            package_bun(archive, source, key)
            if kind == "bun"
            else package_zip(archive, source)
        )
    say(f"packaged {count} files into {path} ({os.path.getsize(path) / 1e6:.1f} MB)")
    return path


def describe_package(spec):
    kind, source = spec["package"]
    if kind == "bun":
        return f"bun build {os.path.relpath(source, HERE)} from {os.path.relpath(bun_root(source), HERE)}/ into one handler.js"
    return "zip " + ", ".join(arc for _, arc in source)


def configuration(name):
    return maybe("lambda", "get-function-configuration", "--function-name", name)


def wanted(spec):
    wants = spec["env"]
    return (
        list(wants.items())
        if isinstance(wants, dict)
        else [(key, key) for key in wants]
    )


def describe_env(spec):
    return [
        name if name == key else f"{name}<-{key}" for name, key in wanted(spec)
    ] + list(spec["extra"])


def variables(spec):
    data = {name: need(key) for name, key in wanted(spec)}
    data.update(spec["extra"])
    return data


def log_group(name):
    return f"/aws/lambda/{name}"


def remember(spec):
    found = configuration(spec["name"])
    if found:
        save_env(spec["arn"], found["FunctionArn"], repair=True)


def config_args(spec, variables_path):
    return [
        "--role",
        role_arn(spec["role"]),
        "--handler",
        spec["handler"],
        "--runtime",
        spec["runtime"],
        "--memory-size",
        spec["memory"],
        "--timeout",
        spec["timeout"],
        "--ephemeral-storage",
        f"Size={spec['storage']}",
        "--environment",
        f"file://{variables_path}",
        *(
            ["--layers", *[need(key) for key in spec["layers"]]]
            if spec["layers"]
            else []
        ),
        "--logging-config",
        f"LogFormat={spec['logs']}"
        + (
            ",ApplicationLogLevel=INFO,SystemLogLevel=INFO"
            if spec["logs"] == "JSON"
            else ""
        ),
    ]


def settle(spec, path, exists):
    variables_path = tmpfile(json.dumps({"Variables": variables(spec)}), ".json")
    try:
        if exists:
            aws(
                "lambda",
                "update-function-code",
                "--function-name",
                spec["name"],
                "--zip-file",
                f"fileb://{path}",
            )
            aws(
                "lambda", "wait", "function-updated-v2", "--function-name", spec["name"]
            )
            aws(
                "lambda",
                "update-function-configuration",
                "--function-name",
                spec["name"],
                *config_args(spec, variables_path),
            )
            aws(
                "lambda", "wait", "function-updated-v2", "--function-name", spec["name"]
            )
            remember(spec)
        else:
            data = aws(
                "lambda",
                "create-function",
                "--function-name",
                spec["name"],
                "--architectures",
                spec["arch"],
                "--zip-file",
                f"fileb://{path}",
                *config_args(spec, variables_path),
            )
            aws("lambda", "wait", "function-active-v2", "--function-name", spec["name"])
            save_env(spec["arn"], data["FunctionArn"], repair=True)
    finally:
        os.remove(variables_path)


def guards(spec):
    aws(
        "lambda",
        "put-function-event-invoke-config",
        "--function-name",
        spec["name"],
        "--maximum-retry-attempts",
        spec["retries"],
    )
    aws(
        "lambda",
        "put-function-concurrency",
        "--function-name",
        spec["name"],
        "--reserved-concurrent-executions",
        spec["concurrency"],
    )
    maybe("logs", "create-log-group", "--log-group-name", log_group(spec["name"]))
    aws(
        "logs",
        "put-retention-policy",
        "--log-group-name",
        log_group(spec["name"]),
        "--retention-in-days",
        spec["retention"],
    )


def deploy():
    key = pick(FUNCTIONS, "function")
    spec = FUNCTIONS[key]
    if not maybe("iam", "get-role", "--role-name", spec["role"]):
        raise SystemExit(
            f"refuse: role {spec['role']} is missing, run iam.py role {spec['role']} --yes first"
        )
    exists = configuration(spec["name"]) is not None
    if not gate(
        f"fn deploy {key}",
        [
            describe_package(spec),
            f"{'update' if exists else 'create'} {spec['name']} on {spec['runtime']} {spec['arch']}, handler {spec['handler']}, role {spec['role']}",
            f"memory {spec['memory']} MB, timeout {spec['timeout']} s, storage {spec['storage']} MB",
            f"layers {' '.join(spec['layers'])}, {spec['logs']} logs",
            f"env {' '.join(describe_env(spec))}",
            f"retries {spec['retries']}, reserved concurrency {spec['concurrency']}, log retention {spec['retention']} days",
        ],
    ):
        return
    path = package(key)
    settle(spec, path, exists)
    guards(spec)
    say(f"deployed {spec['name']}")


def delete():
    name = given("function name")
    if any(spec["name"] == name for spec in FUNCTIONS.values()):
        raise SystemExit(
            f"refuse: {name} is a registry row, drop it from common.py first"
        )
    found = configuration(name)
    if not found:
        raise SystemExit(f"refuse: {name} does not exist")
    if not gate(
        f"fn delete {name}",
        [
            f"delete function {name} on {found['Runtime']}, last modified {found['LastModified']}",
            f"delete log group {log_group(name)}",
        ],
    ):
        return
    aws("lambda", "delete-function", "--function-name", name)
    maybe("logs", "delete-log-group", "--log-group-name", log_group(name))
    say(f"deleted {name}")


def show():
    for key, spec in FUNCTIONS.items():
        found = configuration(spec["name"])
        if not found:
            say(f"{key:<20} {spec['name']:<28} none yet")
            continue
        say(
            f"{key:<20} {spec['name']:<28} {found['State']} {found['MemorySize']} MB {found['Timeout']} s"
        )
        say(
            f"{'':<20} {found['Runtime']} {found['Architectures']} {found['LastModified']}"
        )


def config():
    spec = FUNCTIONS[pick(FUNCTIONS, "function")]
    found = configuration(spec["name"])
    if not found:
        raise SystemExit(f"refuse: {spec['name']} does not exist")
    found.get("Environment", {}).pop("Variables", None)
    say(json.dumps(found, indent=2))


def logs():
    spec = FUNCTIONS[pick(FUNCTIONS, "function")]
    stream(
        "aws",
        "logs",
        "tail",
        log_group(spec["name"]),
        "--since",
        "1h",
        "--format",
        "short",
    )


ERROR_PATTERN = '?ERROR ?WARNING ?"Task timed out" ?"Runtime exited" ?Traceback'


def errors():
    spec = FUNCTIONS[pick(FUNCTIONS, "function")]
    hours = (
        float(sys.argv[3])
        if len(sys.argv) > 3 and not sys.argv[3].startswith("--")
        else 24
    )
    start = int((time.time() - hours * 3600) * 1000)
    found = (
        aws(
            "logs",
            "filter-log-events",
            "--log-group-name",
            log_group(spec["name"]),
            "--start-time",
            start,
            "--filter-pattern",
            ERROR_PATTERN,
            "--query",
            "events[].message",
        )
        or []
    )
    for line in found:
        try:
            record = json.loads(line)
        except ValueError:
            say(line.strip())
            continue
        if "message" in record:
            say(
                f"{record.get('timestamp', '')} {record.get('level', '')} {record['message']}"
            )
            for frame in record.get("stackTrace") or []:
                say(f"    {frame.strip()}")
        elif (
            record.get("type") == "platform.report"
            and record["record"].get("status") != "success"
        ):
            say(
                f"{record['time']} REPORT {record['record']['status']} {record['record']['metrics']['durationMs']} ms"
            )
        else:
            say(line.strip())
    say(f"{len(found)} matches in the last {hours:g} h")
    if found:
        raise SystemExit(1)


def invoke():
    key = pick(FUNCTIONS, "function")
    spec = FUNCTIONS[key]
    if not gate(
        f"fn invoke {key}",
        [f"invoke {spec['name']} once with {json.dumps(spec['payload'])}"],
    ):
        return
    out = tmpfile("", ".json")
    try:
        data = aws(
            "lambda",
            "invoke",
            "--function-name",
            spec["name"],
            "--payload",
            json.dumps(spec["payload"]),
            "--cli-binary-format",
            "raw-in-base64-out",
            "--cli-read-timeout",
            "0",
            "--log-type",
            "Tail",
            out,
        )
        say(f"status {data['StatusCode']} {data.get('FunctionError', 'ok')}")
        say(base64.b64decode(data["LogResult"]).decode())
        with open(out) as handle:
            say(handle.read())
    finally:
        os.remove(out)


def wake():
    spec = FUNCTIONS[pick(FUNCTIONS, "function")]
    payload = dict(spec["payload"])
    words = list(sys.argv[3:])
    if "--repo" in words:
        index = words.index("--repo")
        slug = words[index + 1] if index + 1 < len(words) else ""
        if not slug or slug.startswith("--"):
            raise SystemExit("refuse: --repo needs an owner/slug")
        payload["repo"] = slug
        del words[index : index + 2]
    sha = next((word for word in words if not word.startswith("--")), "")
    if sha:
        payload["sha"] = sha
    if "--force" in words:
        payload["force"] = True
    if env("DRY") == "1":
        say(f"dry {spec['name']} {json.dumps(payload)}")
        return
    aws(
        "lambda",
        "invoke",
        "--function-name",
        spec["name"],
        "--invocation-type",
        "Event",
        "--payload",
        json.dumps(payload),
        "--cli-binary-format",
        "raw-in-base64-out",
        os.devnull,
    )
    say(f"woke {spec['name']} with {json.dumps(payload)}")


def package_verb():
    package(pick(FUNCTIONS, "function"))


VERBS = {
    "list": (show, "", "each function with state, memory and timeout"),
    "package": (package_verb, "<fn>", "zip the function into data/functions/<fn>/"),
    "deploy": (
        deploy,
        "<fn>",
        "package, create or update code and configuration, then the guards: retries 0, reserved concurrency 1, log retention 14 days",
    ),
    "delete": (
        delete,
        "<name>",
        "drop a function and its log group by literal name; refuses a name still in the registry",
    ),
    "invoke": (
        invoke,
        "<fn>",
        "one synchronous run with the function's payload, then the tail log",
    ),
    "wake": (
        wake,
        "<fn> [--repo <slug>] [sha] [--force]",
        "one asynchronous invoke, no gate; a sha counts only beside --repo; --force repaints every route; DRY=1 prints the payload",
    ),
    "config": (config, "<fn>", "the live configuration without the env"),
    "logs": (logs, "<fn>", "the last hour of logs"),
    "errors": (
        errors,
        "<fn> [hours]",
        "every ERROR, WARNING, timeout and traceback of the last 24 h (or hours), with the stack; exit 1 on any",
    ),
}

ABOUT = "Lambda functions from FUNCTIONS in common.py. A mutating verb prints a PLAN; --yes applies it. Order for a new one: iam.py role, layers.py build and publish, fn.py deploy, fn.py invoke, schedules.py sync and enable."

if __name__ == "__main__":
    verb(VERBS, ABOUT)()
