import json
import os
import shutil
import subprocess
import sys

SEED = 7

# CLIENTS

def client():
    import boto3
    return boto3.client("s3")

# PRESS

def press(emit, home, seed):
    subprocess.run(emit + ["emit", home, str(seed)], check=True)
    from video import assemble
    assemble(home)
    with open(os.path.join(home, "quest.json")) as f:
        return json.load(f)["key"]

# HANDLER

def handler(event, context):
    seed = int(event.get("seed", SEED))
    binary = os.environ.get("MRLYGAME_BIN", "/opt/bin/mrlygame")
    bucket = os.environ["MRLYGAME_BUCKET"]
    home = os.path.join("/tmp", f"quest-{seed}")
    key = press([binary], home, seed)
    s3 = client()
    s3.upload_file(os.path.join(home, f"{key}.mp4"), bucket, f"{key}/{key}.mp4")
    s3.upload_file(os.path.join(home, "quest.json"), bucket, f"{key}/quest.json")
    return {"key": key, "seed": seed, "video": f"{key}/{key}.mp4", "manifest": f"{key}/quest.json"}

# LOCAL

def root():
    here = os.path.dirname(os.path.abspath(__file__))
    return os.path.dirname(os.path.dirname(here))

def emitter():
    binary = os.environ.get("MRLYGAME_BIN")
    if binary:
        return [binary]
    return ["cargo", "run", "-q", "--release", "-p", "mrlygame",
            "--manifest-path", os.path.join(root(), "Cargo.toml"), "--"]

def local(seed):
    home = os.path.join(root(), "data", "mrlygame", f"quest-{seed}")
    shutil.rmtree(home, ignore_errors=True)
    os.makedirs(home)
    key = press(emitter(), home, seed)
    print(f"Pressed: {os.path.join(home, f'{key}.mp4')}")
    return key

# TERMINAL

def help():
    commands = [
        ("local", f"press one quest into data/mrlygame/ (seed {SEED})"),
        ("local <seed>", "press one quest into data/mrlygame/ under that seed"),
    ]
    width = max(len(name) for name, _ in commands)
    print("mrlygame lambda")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["local"]:
            local(SEED)
        case ["local", seed]:
            local(int(seed))
        case _:
            help()
            sys.exit(2)

if __name__ == "__main__":
    terminal()
