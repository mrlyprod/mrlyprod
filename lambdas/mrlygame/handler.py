import json
import os
import secrets
import shutil
import subprocess
import sys

from video import assemble, poster, read_quest, span

PREFIX = "videos"
CHUNK = 100
SEED = 7
IMMUTABLE = "public, max-age=31536000, immutable"
FRESH = "no-cache"

# CLIENTS

def client():
    import boto3
    return boto3.client("s3")

# PRESS

def press(emit, home, seed):
    shutil.rmtree(home, ignore_errors=True)
    os.makedirs(home)
    subprocess.run(emit + ["emit", home, str(seed)], check=True)
    assemble(home)
    poster(home)
    return read_quest(home)

# ENTRY

def entry(quest):
    name = quest["name"]
    return {
        "name": name,
        "seed": quest["seed"],
        "video": f"{PREFIX}/v/{name}.mp4",
        "poster": f"{PREFIX}/p/{name}.jpg",
        "duration": round(span(quest["manifest"]), 3),
        "frames": sum(quest["manifest"]["segments"]),
    }

# OBJECTS

def body(value):
    return (json.dumps(value, indent=2) + "\n").encode()

def read_json(s3, bucket, key):
    try:
        return json.loads(s3.get_object(Bucket=bucket, Key=key)["Body"].read())
    except s3.exceptions.NoSuchKey:
        return None

def write_json(s3, bucket, key, value):
    s3.put_object(
        Bucket=bucket,
        Key=key,
        Body=body(value),
        ContentType="application/json",
        CacheControl=FRESH,
    )

def write_file(s3, bucket, path, key, kind):
    extra = {"ContentType": kind, "CacheControl": IMMUTABLE}
    s3.upload_file(path, bucket, key, ExtraArgs=extra)

# INDEX

def chunk_key(number):
    return f"{PREFIX}/chunk-{number:04d}.json"

def read_index(s3, bucket):
    plan = read_json(s3, bucket, f"{PREFIX}/index.json")
    if not plan:
        return [], []
    chunks = [read_json(s3, bucket, key) for key in plan["chunks"]]
    if any(chunk is None for chunk in chunks):
        raise RuntimeError(f"{PREFIX}/index.json names a chunk that is not there")
    return [row for chunk in chunks for row in chunk], chunks

def merge(old, fresh):
    names = {row["name"] for row in fresh}
    return fresh + [row for row in old if row["name"] not in names]

def sliced(rows):
    return [rows[start:start + CHUNK] for start in range(0, max(len(rows), 1), CHUNK)]

def write_index(s3, bucket, rows, standing):
    chunks = sliced(rows)
    for number in reversed(range(len(chunks))):
        if number < len(standing) and standing[number] == chunks[number]:
            continue
        write_json(s3, bucket, chunk_key(number), chunks[number])
    write_json(s3, bucket, f"{PREFIX}/index.json", {
        "count": len(rows),
        "chunk": CHUNK,
        "chunks": [chunk_key(number) for number in range(len(chunks))],
    })

# PUBLISH

def publish(s3, bucket, home, quest):
    row = entry(quest)
    mp4 = os.path.join(home, f"{quest['key']}.mp4")
    jpg = os.path.join(home, f"{quest['name']}.jpg")
    write_file(s3, bucket, mp4, row["video"], "video/mp4")
    write_file(s3, bucket, jpg, row["poster"], "image/jpeg")
    rows, standing = read_index(s3, bucket)
    rows = merge(rows, [row])
    write_index(s3, bucket, rows, standing)
    return {**row, "count": len(rows)}

# HANDLER

def seed_of(event):
    given = event.get("seed")
    return int(given) if given is not None else secrets.randbits(64)

def handler(event, context):
    seed = seed_of(event or {})
    binary = os.environ.get("MRLYGAME_BIN", "/opt/bin/mrlygame")
    bucket = os.environ["MRLYGAME_BUCKET"]
    home = os.path.join("/tmp", f"quest-{seed}")
    try:
        return publish(client(), bucket, home, press([binary], home, seed))
    finally:
        shutil.rmtree(home, ignore_errors=True)

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
    quest = press(emitter(), home, seed)
    print(json.dumps(entry(quest), indent=2))
    return quest

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
