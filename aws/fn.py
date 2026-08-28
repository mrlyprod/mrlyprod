import os
import sys
import zipfile

from common import ROOT_DIR

LAMBDAS_DIR = os.path.join(ROOT_DIR, "lambdas")
BUILD_DIR = os.path.join(ROOT_DIR, "data", "fn")

# DESIRED

RUNTIME = "python3.13"
ARCHITECTURE = "arm64"
HANDLER = "handler.handler"
RETRIES = 0
CONCURRENCY = 1

LOGGING = {"LogFormat": "JSON", "ApplicationLogLevel": "INFO", "SystemLogLevel": "INFO"}

FUNCTIONS = {
    "game": {
        "name": "mrlygame",
        "source": "mrlygame",
        "timeout": 900,
        "memory": 3008,
        "storage": 2048,
        "layers": ["MRLYGAME_LAYER_ARN", "FFMPEG_LAYER_ARN"],
        "env": ["MRLYGAME_BUCKET"],
    },
}

def deployed_name(key):
    return FUNCTIONS[key]["name"]

def desired_config(target, role, layers, variables):
    return {
        "Role": role,
        "Handler": HANDLER,
        "Runtime": RUNTIME,
        "Timeout": target["timeout"],
        "MemorySize": target["memory"],
        "Layers": layers,
        "Environment": {"Variables": variables},
        "EphemeralStorage": {"Size": target["storage"]},
        "LoggingConfig": LOGGING,
    }

# ARN

_account = None

def account_id():
    global _account
    if _account is None:
        _account = sts_client.get_caller_identity()["Account"]
    return _account

def function_arn(key):
    region = lambda_client.meta.region_name
    return f"arn:aws:lambda:{region}:{account_id()}:function:{deployed_name(key)}"

# ENV

def resolve(target):
    keys = ["ROLE_ARN", *target["layers"], *target["env"]]
    absent = [key for key in keys if not os.environ.get(key)]
    if absent:
        print(f"fn (add to .env: {', '.join(absent)})")
        return None
    layers = [os.environ[key] for key in target["layers"]]
    variables = {key: os.environ[key] for key in target["env"]}
    return os.environ["ROLE_ARN"], layers, variables

# PACKAGE

SKIP_DIRS = {"__pycache__", ".git"}

def bundle(target):
    source = os.path.join(LAMBDAS_DIR, target["source"])
    if not os.path.isdir(source):
        print(f"fn (no source: {source})")
        return None
    os.makedirs(BUILD_DIR, exist_ok=True)
    zip_path = os.path.join(BUILD_DIR, f"{target['name']}.zip")
    keys = []
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
        for root, dirs, files in os.walk(source):
            dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
            for name in sorted(files):
                if name.startswith(".") or name.endswith(".pyc"): continue
                path = os.path.join(root, name)
                key = os.path.relpath(path, source).replace(os.path.sep, "/")
                zf.write(path, key)
                keys.append(key)
    if not any(key == "handler.py" for key in keys):
        print(f"fn (no handler.py in {source})")
        return None
    return zip_path, keys

# STATE

def exists(name):
    try:
        lambda_client.get_function(FunctionName=name)
        return True
    except lambda_client.exceptions.ResourceNotFoundException:
        return False

def retry_attempts(name):
    try:
        return lambda_client.get_function_event_invoke_config(FunctionName=name)["MaximumRetryAttempts"]
    except lambda_client.exceptions.ResourceNotFoundException:
        return 2

# COMMANDS

def functions():
    paginator = lambda_client.get_paginator("list_functions")
    found = 0
    for page in paginator.paginate():
        for function in page["Functions"]:
            arch = ",".join(function.get("Architectures", []))
            print(f"{function['FunctionName']:<20} {function['Runtime']:<12} {arch:<6} {function['MemorySize']:>5} MB  {function['Timeout']:>4}s")
            found += 1
    if not found:
        print("no functions")

def package(key):
    target = FUNCTIONS[key]
    got = bundle(target)
    if not got: return
    zip_path, keys = got
    print(f"{target['name']}: {zip_path} ({os.path.getsize(zip_path) / 1024:.0f} KB)")
    for name in keys:
        print(f"  {name}")

def deploy(key):
    target = FUNCTIONS[key]
    env = resolve(target)
    if not env: return
    role, layers, variables = env
    got = bundle(target)
    if not got: return
    zip_path, keys = got
    with open(zip_path, "rb") as f:
        code = f.read()
    name = target["name"]
    config = desired_config(target, role, layers, variables)
    print(f"{name}: {len(code) / 1024:.0f} KB, {len(keys)} files, {len(layers)} layers")
    if exists(name):
        lambda_client.update_function_code(FunctionName=name, ZipFile=code, Publish=True)
        lambda_client.get_waiter("function_updated").wait(FunctionName=name)
        lambda_client.update_function_configuration(FunctionName=name, **config)
        lambda_client.get_waiter("function_updated").wait(FunctionName=name)
        print("  code and configuration: updated")
    else:
        lambda_client.create_function(
            FunctionName=name,
            Code={"ZipFile": code},
            Publish=True,
            Architectures=[ARCHITECTURE],
            **config,
        )
        lambda_client.get_waiter("function_active").wait(FunctionName=name)
        print("  function: created")
    lambda_client.put_function_event_invoke_config(FunctionName=name, MaximumRetryAttempts=RETRIES)
    lambda_client.put_function_concurrency(FunctionName=name, ReservedConcurrentExecutions=CONCURRENCY)
    print(f"  retries {RETRIES}, reserved concurrency {CONCURRENCY}")

def show(key):
    target = FUNCTIONS[key]
    name = target["name"]
    if not exists(name):
        print(f"{name}: not deployed")
        return
    got = lambda_client.get_function(FunctionName=name)
    config = got["Configuration"]
    reserved = got.get("Concurrency", {}).get("ReservedConcurrentExecutions", "unreserved")
    print(f"{name}: {config['State']}, {config['Runtime']}, {','.join(config.get('Architectures', []))}")
    print(f"  memory {config['MemorySize']} MB, timeout {config['Timeout']}s, storage {config.get('EphemeralStorage', {}).get('Size', 512)} MB")
    print(f"  retries {retry_attempts(name)}, reserved concurrency {reserved}")
    print(f"  code {config['CodeSize'] / 1024:.0f} KB, updated {config['LastModified']}")
    for layer in config.get("Layers", []):
        print(f"  layer {layer['Arn'].rsplit(':', 2)[-2]} version {layer['Arn'].rsplit(':', 1)[-1]}")
    for variable in sorted(config.get("Environment", {}).get("Variables", {})):
        print(f"  env {variable}")

def drop(key):
    name = deployed_name(key)
    if not exists(name):
        print(f"{name}: not deployed")
        return
    sure = input(f"delete the function {name}? type the name to confirm: ")
    if sure != name:
        print("aborted")
        return
    lambda_client.delete_function(FunctionName=name)
    print(f"deleted {name}")

# TERMINAL

def help():
    commands = [
        ("list", "list every deployed function with runtime, memory, timeout"),
        ("package <target>", "zip the source folder into data/fn, no aws"),
        ("deploy <target>", f"create or update code and config, retries {RETRIES}, concurrency {CONCURRENCY}"),
        ("show <target>", "state, sizes, layers, retries, concurrency, env keys"),
        ("drop <target>", "delete the function (confirmed)"),
    ]
    width = max(len(name) for name, _ in commands)
    print(f"fn <command> <{'|'.join(FUNCTIONS)}>")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["list"]: functions()
        case ["package", key] if key in FUNCTIONS: package(key)
        case ["deploy", key] if key in FUNCTIONS: deploy(key)
        case ["show", key] if key in FUNCTIONS: show(key)
        case ["drop", key] if key in FUNCTIONS: drop(key)
        case _: help()

# AWS

import common

lambda_client = common.lambda_client()
sts_client = common.sts_client()

if __name__ == "__main__":
    terminal()
