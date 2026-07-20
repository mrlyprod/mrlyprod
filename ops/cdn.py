import hashlib
import os
import sys

from deploy import content_type, invalidate_cloudfront, put_file, s3_client

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CDN_DIR = os.path.join(ROOT_DIR, "cdn")

# HASH

def local_md5(local_path):
    h = hashlib.md5()
    with open(local_path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()

def remote_map(bucket):
    remote = {}
    paginator = s3_client.get_paginator("list_objects_v2")
    for page in paginator.paginate(Bucket=bucket):
        for obj in page.get("Contents", []):
            remote[obj["Key"]] = obj["ETag"].strip('"')
    return remote

# SYNC

def sync():
    bucket = os.environ.get("MRLYCDN_BUCKET")
    distribution_id = os.environ.get("MRLY_ID")
    missing = [key for key, value in (("MRLYCDN_BUCKET", bucket), ("MRLY_ID", distribution_id)) if not value]
    if missing:
        print(f"cdn (add to .env: {', '.join(missing)})")
        return
    if not os.path.exists(CDN_DIR):
        print("cdn (nothing to sync: cdn/ is empty)")
        return
    remote = remote_map(bucket)
    changed = []
    unchanged = 0
    for root, _, files in os.walk(CDN_DIR):
        for name in files:
            if name == ".DS_Store": continue
            local_path = os.path.join(root, name)
            key = "cdn/" + os.path.relpath(local_path, CDN_DIR).replace(os.path.sep, "/")
            ct = "text/markdown" if name.endswith(".md") else content_type(local_path)
            if remote.get(key) == local_md5(local_path):
                unchanged += 1
                continue
            put_file(local_path, key, bucket, content_type=ct, cache_control="max-age=300")
            changed.append(key)
            print(f"Uploaded: {key}")
    if changed:
        invalidate_cloudfront(distribution_id, [f"/{key}" for key in changed])
    print(f"cdn: {len(changed)} uploaded, {unchanged} unchanged")

# TERMINAL

def help():
    commands = [
        ("sync", "mirror cdn/ into S3 and invalidate changed keys"),
    ]
    width = max(len(name) for name, _ in commands)
    print("cdn")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case [] | ["sync"]: sync()
        case _: help()

if __name__ == "__main__":
    terminal()
