import io
import json
import os
import shutil
import subprocess
import sys
import time
import zipfile

from common import ROOT_DIR, save_env

BUILD_DIR = os.path.join(ROOT_DIR, "data", "layers")

# DESIRED

RUNTIME = "python3.13"
ARCHITECTURE = "arm64"
IMPLEMENTATION = "cp"
PYTHON_VERSION = "3.13"
PLATFORM = "manylinux_2_17_aarch64"

PACKAGES = {
    "boto3": "boto3",
    "hfhub": "huggingface_hub",
    "numpy": "numpy",
    "pillow": "Pillow",
    "requests": "requests",
}

MODULES = {
    "boto3": "boto3",
    "hfhub": "huggingface_hub",
    "numpy": "numpy",
    "pillow": "PIL",
    "requests": "requests",
}

FFMPEG_URL = "https://johnvansickle.com/ffmpeg/builds/ffmpeg-git-arm64-static.tar.xz"

def arn_key(name):
    return f"{name.upper()}_LAYER_ARN"

# ZIP

EXEC_MODE = 0o755
READ_MODE = 0o644
REGULAR = 0o100000

def zip_dir(source, zip_path):
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
        for root, dirs, files in os.walk(source):
            dirs[:] = [d for d in dirs if d != "__pycache__"]
            for name in sorted(files):
                path = os.path.join(root, name)
                key = os.path.relpath(path, source).replace(os.path.sep, "/")
                info = zipfile.ZipInfo.from_file(path, key)
                info.compress_type = zipfile.ZIP_DEFLATED
                mode = EXEC_MODE if key.startswith("bin/") or os.access(path, os.X_OK) else READ_MODE
                info.external_attr = (REGULAR | mode) << 16
                with open(path, "rb") as f:
                    zf.writestr(info, f.read())
    return zip_path

def run(cmd):
    result = subprocess.run(cmd)
    if result.returncode != 0:
        print(f"layers (step failed: {' '.join(cmd)})")
        sys.exit(1)

def clean(name):
    os.makedirs(BUILD_DIR, exist_ok=True)
    build = os.path.join(BUILD_DIR, name)
    zip_path = os.path.join(BUILD_DIR, f"{name}.zip")
    if os.path.exists(build): shutil.rmtree(build)
    if os.path.exists(zip_path): os.remove(zip_path)
    return build, zip_path

def report(name, zip_path):
    print(f"{name}: {zip_path} ({os.path.getsize(zip_path) / 1_000_000:.1f} MB)")

# BUILD

def build_package(name):
    build, zip_path = clean(name)
    site = os.path.join(build, "python")
    os.makedirs(site, exist_ok=True)
    print(f"{name}: installing {PACKAGES[name]} for {PLATFORM}, python {PYTHON_VERSION}")
    run([
        sys.executable, "-m", "pip", "install", PACKAGES[name],
        "--implementation", IMPLEMENTATION,
        "--python-version", PYTHON_VERSION,
        "--platform", PLATFORM,
        "--only-binary=:all:",
        "-t", site,
    ])
    zip_dir(build, zip_path)
    shutil.rmtree(build)
    report(name, zip_path)

def build_ffmpeg():
    name = "ffmpeg"
    build, zip_path = clean(name)
    os.makedirs(os.path.join(build, "bin"), exist_ok=True)
    tarball = os.path.join(BUILD_DIR, "ffmpeg.tar.xz")
    print(f"{name}: downloading the static {ARCHITECTURE} build")
    run(["curl", "-L", "-o", tarball, FFMPEG_URL])
    run(["tar", "-xf", tarball, "-C", BUILD_DIR])
    unpacked = [d for d in sorted(os.listdir(BUILD_DIR)) if d.startswith("ffmpeg-") and "arm64" in d and os.path.isdir(os.path.join(BUILD_DIR, d))]
    if not unpacked:
        print("layers (no unpacked ffmpeg directory)")
        sys.exit(1)
    source = os.path.join(BUILD_DIR, unpacked[0])
    binary = os.path.join(build, "bin", name)
    shutil.move(os.path.join(source, name), binary)
    os.chmod(binary, EXEC_MODE)
    os.remove(tarball)
    shutil.rmtree(source)
    zip_dir(build, zip_path)
    shutil.rmtree(build)
    report(name, zip_path)

def build_bin(name, path):
    if not os.path.isfile(path):
        print(f"layers (no binary: {path})")
        return
    build, zip_path = clean(name)
    os.makedirs(os.path.join(build, "bin"), exist_ok=True)
    binary = os.path.join(build, "bin", os.path.basename(path))
    shutil.copyfile(path, binary)
    os.chmod(binary, EXEC_MODE)
    zip_dir(build, zip_path)
    shutil.rmtree(build)
    report(name, zip_path)

# PROBE

PROBE_NAME = "mrlyprobe"
PROBE_TIMEOUT = 30
PROBE_MEMORY = 512
PROBE_SETTLE = 5

PROBE_SOURCE = """import subprocess

def handler(event, context):
    results = {}
    for name in event.get("modules", []):
        try:
            results[name] = getattr(__import__(name), "__version__", "ok")
        except Exception as error:
            results[name] = f"error: {error}"
    if event.get("ffmpeg"):
        try:
            out = subprocess.check_output(["/opt/bin/ffmpeg", "-version"], stderr=subprocess.STDOUT)
            results["ffmpeg"] = out.decode().splitlines()[0]
        except Exception as error:
            results["ffmpeg"] = f"error: {error}"
    return {"results": results}
"""

def probe_zip():
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w", zipfile.ZIP_DEFLATED) as zf:
        zf.writestr("handler.py", PROBE_SOURCE)
    return buffer.getvalue()

# COMMANDS

def build(name, path=None):
    if name == "ffmpeg":
        build_ffmpeg()
        return
    if name in PACKAGES:
        build_package(name)
        return
    if path:
        build_bin(name, path)
        return
    print(f"layers (no recipe for {name}, pass a binary path to make a bin layer)")

def publish(name):
    zip_path = os.path.join(BUILD_DIR, f"{name}.zip")
    if not os.path.isfile(zip_path):
        print(f"layers (no zip: build {name} first)")
        return
    with open(zip_path, "rb") as f:
        content = f.read()
    response = lambda_client.publish_layer_version(
        LayerName=name,
        Content={"ZipFile": content},
        CompatibleRuntimes=[RUNTIME],
        CompatibleArchitectures=[ARCHITECTURE],
    )
    arn = response["LayerVersionArn"]
    print(f"{name}: version {response['Version']} published, {len(content) / 1_000_000:.1f} MB")
    save_env(arn_key(name), arn)
    print(f"  {arn_key(name)}={arn}")

def layers():
    paginator = lambda_client.get_paginator("list_layers")
    found = 0
    for page in paginator.paginate(CompatibleArchitecture=ARCHITECTURE, CompatibleRuntime=RUNTIME):
        for layer in page["Layers"]:
            latest = layer["LatestMatchingVersion"]
            pinned = os.environ.get(arn_key(layer["LayerName"]))
            state = "pinned" if pinned == latest["LayerVersionArn"] else "not pinned in .env"
            print(f"{layer['LayerName']:<12} version {latest['Version']:<4} {state}")
            print(f"  {latest['LayerVersionArn']}")
            found += 1
    if not found:
        print("no layers")

def probe(names):
    role = os.environ.get("ROLE_ARN")
    if not role:
        print("layers (add to .env: ROLE_ARN)")
        return
    unknown = [name for name in names if name not in PACKAGES and name != "ffmpeg"]
    if unknown:
        print(f"layers (no recipe for {', '.join(unknown)})")
        return
    names = names or [name for name in (*PACKAGES, "ffmpeg") if os.environ.get(arn_key(name))]
    if not names:
        print("layers (nothing to probe: publish a layer first)")
        return
    absent = [arn_key(name) for name in names if not os.environ.get(arn_key(name))]
    if absent:
        print(f"layers (add to .env: {', '.join(absent)})")
        return
    arns = [os.environ[arn_key(name)] for name in names]
    try:
        lambda_client.delete_function(FunctionName=PROBE_NAME)
        print(f"{PROBE_NAME}: stale copy deleted")
        time.sleep(PROBE_SETTLE)
    except lambda_client.exceptions.ResourceNotFoundException:
        pass
    print(f"{PROBE_NAME}: {', '.join(names)}")
    lambda_client.create_function(
        FunctionName=PROBE_NAME,
        Runtime=RUNTIME,
        Role=role,
        Handler="handler.handler",
        Code={"ZipFile": probe_zip()},
        Timeout=PROBE_TIMEOUT,
        MemorySize=PROBE_MEMORY,
        Layers=arns,
        Architectures=[ARCHITECTURE],
    )
    lambda_client.get_waiter("function_active").wait(FunctionName=PROBE_NAME)
    event = {"modules": [MODULES[name] for name in names if name in MODULES], "ffmpeg": "ffmpeg" in names}
    response = lambda_client.invoke(
        FunctionName=PROBE_NAME,
        InvocationType="RequestResponse",
        Payload=json.dumps(event).encode(),
    )
    payload = json.loads(response["Payload"].read())
    for key, value in payload.get("results", {}).items():
        print(f"  {key}: {value}")
    if "results" not in payload:
        print(f"  unexpected response: {payload}")
    lambda_client.delete_function(FunctionName=PROBE_NAME)
    print(f"{PROBE_NAME}: deleted")

# TERMINAL

def help():
    commands = [
        ("build <layer>", f"pip a wheel layer for {ARCHITECTURE} into data/layers, no aws"),
        ("build ffmpeg", "download the static arm64 ffmpeg into bin/ with the exec bit set"),
        ("build <name> <path>", "zip a local binary into bin/ as its own layer"),
        ("publish <layer>", "publish the built zip and write the version arn to .env"),
        ("list", "published layers, latest version, whether .env pins it"),
        ("probe [layer...]", "run one throwaway function with the layers attached, then delete it"),
    ]
    width = max(len(name) for name, _ in commands)
    print(f"layers (recipes: {', '.join(PACKAGES)}, ffmpeg)")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["build", name]: build(name)
        case ["build", name, path]: build(name, path)
        case ["publish", name]: publish(name)
        case ["list"]: layers()
        case ["probe", *names]: probe(names)
        case _: help()

# AWS

import common

lambda_client = common.lambda_client()

if __name__ == "__main__":
    terminal()
