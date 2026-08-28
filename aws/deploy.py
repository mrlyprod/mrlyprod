import hashlib
import mimetypes
import os
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
import uuid
from concurrent.futures import ThreadPoolExecutor, as_completed

from common import ROOT_DIR

NET_DIST_DIR = os.path.join(ROOT_DIR, "data", "net", "dist")
GIT_DIST_DIR = os.path.join(ROOT_DIR, "data", "git", "dist")
WEB_DIST_DIR = os.path.join(ROOT_DIR, "data", "web", "dist")
BOT_DIST_DIR = os.path.join(ROOT_DIR, "data", "bot", "dist")
CDN_DIR = os.path.join(ROOT_DIR, "data", "cdn")
CDN_PREFIXES = ["fonts", "licenses"]
RELEASE_DIR = os.path.join(ROOT_DIR, "data", "release")
CLI_BIN = "mrly"
CLI_TARGETS = [
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
]

# BUILD

def run(cmd, env=None, cwd=None):
    result = subprocess.run(cmd, cwd=cwd or ROOT_DIR, env=env)
    if result.returncode != 0:
        print(f"deploy (step failed: {' '.join(cmd)})")
        sys.exit(1)

def build_net():
    run(["bun", "install"])
    print("Generating the landing site...")
    if os.path.exists(NET_DIST_DIR): shutil.rmtree(NET_DIST_DIR)
    run(["bun", "run", "--cwd", "sites/net", "build"])
    run(["bun", "run", "--cwd", "sites/net", "links"])
    summarize(NET_DIST_DIR)

def build_git():
    print("Generating the projection...")
    run(["bun", "install"])
    if os.path.exists(GIT_DIST_DIR): shutil.rmtree(GIT_DIST_DIR)
    run(["bun", "run", "--cwd", "sites/git", "site"])
    run(["bun", "run", "--cwd", "sites/git", "data"])
    summarize(GIT_DIST_DIR)

def build_web():
    print("Building wasm (release)...")
    run(["wasm-pack", "build", "pkgs/js/web", "--target", "web", "--release", "--out-name", "mrlyweb"], env={**os.environ, "MRLY_RELEASE": "1"})
    print("Generating the face...")
    run(["bun", "install"])
    if os.path.exists(WEB_DIST_DIR): shutil.rmtree(WEB_DIST_DIR)
    run(["bun", "run", "--cwd", "sites/web", "site"])
    summarize(WEB_DIST_DIR)

def build_bot():
    print("Generating the notebook...")
    run(["bun", "install"])
    if os.path.exists(BOT_DIST_DIR): shutil.rmtree(BOT_DIST_DIR)
    run(["bun", "run", "--cwd", "sites/bot", "site"])
    summarize(BOT_DIST_DIR)

def summarize(dist_dir):
    file_count = 0
    total_bytes = 0
    for root, _, files in os.walk(dist_dir):
        for name in files:
            file_count += 1
            total_bytes += os.path.getsize(os.path.join(root, name))
    print(f"dist: {file_count} files, {total_bytes / 1024:.1f} KB total")

# CACHE

NO_CACHE_NAMES = {"robots.txt", "sitemap.xml", "tree.json", "manifest.json", "site.json", "notes.json", "boot.js", "install.sh", "sw.js"}
HASHED_RE = re.compile(r"[-.][0-9a-zA-Z_-]{8}\.[^.]+$")
RAW_PREFIX = "raw/"

IMMUTABLE_CACHE = "public, max-age=31536000, immutable"
DEFAULT_CACHE = "max-age=86400"
NO_CACHE = "no-cache"

def cache_control(key):
    if key.startswith(RAW_PREFIX): return NO_CACHE
    name = key.rsplit("/", 1)[-1]
    if name.endswith(".html") or name.endswith(".md") or name in NO_CACHE_NAMES: return NO_CACHE
    if HASHED_RE.search(name): return IMMUTABLE_CACHE
    return DEFAULT_CACHE

CONTENT_TYPES = {
    ".html": "text/html",
    ".md": "text/plain; charset=utf-8",
    ".css": "text/css",
    ".js": "application/javascript",
    ".mjs": "application/javascript",
    ".json": "application/json",
    ".svg": "image/svg+xml",
    ".png": "image/png",
    ".ico": "image/x-icon",
    ".webp": "image/webp",
    ".woff2": "font/woff2",
    ".woff": "font/woff",
    ".ttf": "font/ttf",
    ".xml": "application/xml",
    ".txt": "text/plain",
    ".sh": "text/x-shellscript",
    ".wasm": "application/wasm",
}

RAW_TYPES = {
    ".gif": "image/gif",
    ".ico": "image/x-icon",
    ".jpeg": "image/jpeg",
    ".jpg": "image/jpeg",
    ".pdf": "application/pdf",
    ".png": "image/png",
    ".svg": "image/svg+xml",
    ".ttf": "font/ttf",
    ".wasm": "application/wasm",
    ".webp": "image/webp",
    ".woff": "font/woff",
    ".woff2": "font/woff2",
}

INLINE_TYPE = "text/plain; charset=utf-8"

def content_type(local_path, key=""):
    ext = os.path.splitext(local_path)[1]
    if key.startswith(RAW_PREFIX): return RAW_TYPES.get(ext.lower()) or INLINE_TYPE
    return CONTENT_TYPES.get(ext) or mimetypes.guess_type(local_path)[0] or "application/octet-stream"

# ARCHIVE

ARCHIVE_NAME = "mrlyprod"
TARBALL = f"{ARCHIVE_NAME}.tar.gz"
SIDECAR = f"{TARBALL}.sha256"
GZIP_TYPE = "application/gzip"
SIDECAR_TYPE = "text/plain; charset=utf-8"

def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()

def head_key():
    result = subprocess.run(["git", "log", "-1", "--format=%h"], cwd=ROOT_DIR, capture_output=True, text=True)
    return result.stdout.strip() or "head"

def push_tarball(bucket):
    with tempfile.TemporaryDirectory() as out_dir:
        tarball = os.path.join(out_dir, TARBALL)
        result = subprocess.run(["git", "archive", "--format=tar.gz", f"--prefix={ARCHIVE_NAME}/", "-o", tarball, "HEAD"], cwd=ROOT_DIR, capture_output=True, text=True)
        if result.returncode != 0:
            print(f"deploy (archive failed: {result.stderr.strip()})")
            sys.exit(1)
        sidecar = os.path.join(out_dir, SIDECAR)
        with open(sidecar, "w") as f:
            f.write(f"{sha256_file(tarball)}  {TARBALL}\n")
        era = f"{ARCHIVE_NAME}-{head_key()}.tar.gz"
        put_file(tarball, TARBALL, bucket, content_type=GZIP_TYPE, cache_control=NO_CACHE)
        put_file(sidecar, SIDECAR, bucket, content_type=SIDECAR_TYPE, cache_control=NO_CACHE)
        put_file(tarball, era, bucket, content_type=GZIP_TYPE, cache_control=IMMUTABLE_CACHE)
        print(f"deploy (tarball {TARBALL}, {SIDECAR}, {era}: {os.path.getsize(tarball) / 1024:.0f} KB)")

# PUSH

def push_net():
    bucket = os.environ.get("MRLYNET_BUCKET")
    distribution_id = os.environ.get("MRLYNET_ID")
    missing = [key for key, value in (("MRLYNET_BUCKET", bucket), ("MRLYNET_ID", distribution_id)) if not value]
    if missing:
        print(f"deploy (add to .env: {', '.join(missing)})")
        sys.exit(1)
    build_net()
    plan = plan_dist(NET_DIST_DIR)
    for key in NET_CONTENT_KEYS:
        plan.pop(key, None)
    print(f"Syncing s3://{bucket} from '{NET_DIST_DIR}'...")
    invalidate_if_changed(distribution_id, reconcile(bucket, plan, bucket_etags(bucket)))

def push_git():
    bucket = os.environ.get("MRLYGIT_BUCKET")
    distribution_id = os.environ.get("MRLYGIT_ID")
    missing = [key for key, value in (("MRLYGIT_BUCKET", bucket), ("MRLYGIT_ID", distribution_id)) if not value]
    if missing:
        print(f"deploy (add to .env: {', '.join(missing)})")
        sys.exit(1)
    build_git()
    plan = plan_shell()
    print(f"Syncing s3://{bucket} from '{GIT_DIST_DIR}'...")
    invalidate_if_changed(distribution_id, reconcile(bucket, plan, bucket_etags(bucket)))

def push_web():
    bucket = os.environ.get("MRLYWEB_BUCKET")
    distribution_id = os.environ.get("MRLYWEB_ID")
    missing = [key for key, value in (("MRLYWEB_BUCKET", bucket), ("MRLYWEB_ID", distribution_id)) if not value]
    if missing:
        print(f"deploy (add to .env: {', '.join(missing)})")
        sys.exit(1)
    build_web()
    invalidate_if_changed(distribution_id, sync(bucket, WEB_DIST_DIR))

def push_bot():
    bucket = os.environ.get("MRLYBOT_BUCKET")
    distribution_id = os.environ.get("MRLYBOT_ID")
    missing = [key for key, value in (("MRLYBOT_BUCKET", bucket), ("MRLYBOT_ID", distribution_id)) if not value]
    if missing:
        print(f"deploy (add to .env: {', '.join(missing)})")
        sys.exit(1)
    build_bot()
    plan = plan_dist(BOT_DIST_DIR)
    for key in BOT_CONTENT_KEYS:
        plan.pop(key, None)
    remote = {key: etag for key, etag in bucket_etags(bucket).items() if key not in BOT_CONTENT_KEYS}
    print(f"Syncing s3://{bucket} from '{BOT_DIST_DIR}'...")
    invalidate_if_changed(distribution_id, reconcile(bucket, plan, remote))

def push_cdn():
    bucket = os.environ.get("MRLYCDN_BUCKET")
    distribution_id = os.environ.get("MRLYCDN_ID")
    missing = [key for key, value in (("MRLYCDN_BUCKET", bucket), ("MRLYCDN_ID", distribution_id)) if not value]
    if missing:
        print(f"deploy (add to .env: {', '.join(missing)})")
        sys.exit(1)
    if not os.path.isdir(CDN_DIR):
        print("deploy (no data/cdn: run utils/brand.py first)")
        sys.exit(1)
    build_git()
    print("Generating the site manifest...")
    run(["bun", "run", "--cwd", "sites/net", "data"])
    changed = sync_cdn(bucket)
    push_tarball(bucket)
    invalidate_cloudfront(distribution_id, CONTENT_PATHS if changed else [f"/{TARBALL}", f"/{SIDECAR}"])

def push_cli():
    bucket = os.environ.get("MRLYCDN_BUCKET")
    distribution_id = os.environ.get("MRLYCDN_ID")
    missing = [key for key, value in (("MRLYCDN_BUCKET", bucket), ("MRLYCDN_ID", distribution_id)) if not value]
    if missing:
        print(f"deploy (add to .env: {', '.join(missing)})")
        sys.exit(1)
    with open(os.path.join(ROOT_DIR, "Cargo.toml"), "rb") as f:
        ver = tomllib.load(f)["workspace"]["package"]["version"]
    out = os.path.join(RELEASE_DIR, ver)
    names = [f"{CLI_BIN}-{target}.tar.gz" for target in CLI_TARGETS]
    absent = [name for name in names if not os.path.isfile(os.path.join(out, name))]
    if absent:
        print(f"deploy (no data/release/{ver}: run utils/release.py cli first)")
        sys.exit(1)
    for name in names:
        put_file(os.path.join(out, name), f"cli/{ver}/{name}", bucket, content_type=GZIP_TYPE, cache_control=IMMUTABLE_CACHE)
        print(f"Uploaded: cli/{ver}/{name}")
    s3_client.put_object(Bucket=bucket, Key="cli/latest", Body=ver.encode(), ContentType="text/plain", CacheControl=NO_CACHE)
    print(f"Uploaded: cli/latest ({ver})")
    invalidate_cloudfront(distribution_id, ["/cli/latest"])

# SPLIT

CONTENT_KEYS = ("manifest.json", "tree.json")
NET_CONTENT_KEYS = ("site.json",)
BOT_CONTENT_KEYS = ("notes.json",)
CONTENT_PATHS = ["/manifest.json", "/tree.json", "/site.json", "/raw/*", f"/{TARBALL}", f"/{SIDECAR}"]

def plan_shell():
    plan = plan_dist(GIT_DIST_DIR)
    for key in CONTENT_KEYS:
        plan.pop(key, None)
    return plan

def plan_content():
    plan = {}
    for key in CONTENT_KEYS:
        path = os.path.join(GIT_DIST_DIR, key)
        plan[key] = (path, content_type(path), cache_control(key))
    for key in NET_CONTENT_KEYS:
        path = os.path.join(NET_DIST_DIR, key)
        plan[key] = (path, content_type(path), cache_control(key))
    return {**plan, **plan_raw()}

# SYNC

PROTECTED_RE = re.compile(r"^mrlyprod(-[^/]+)?\.tar\.gz(\.sha256)?$")

def protected(key):
    return bool(PROTECTED_RE.match(key))

def file_md5(local_path):
    digest = hashlib.md5()
    with open(local_path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()

def plan_dist(dist_dir):
    plan = {}
    for root, _, files in os.walk(dist_dir):
        for name in files:
            if name == ".DS_Store": continue
            local_path = os.path.join(root, name)
            key = os.path.relpath(local_path, dist_dir).replace(os.path.sep, "/")
            plan[key] = (local_path, content_type(local_path, key), cache_control(key))
    return plan

def plan_raw():
    result = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT_DIR, capture_output=True, text=True)
    if result.returncode != 0:
        print("deploy (git ls-files failed)")
        sys.exit(1)
    plan = {}
    for path in result.stdout.split("\0"):
        if not path: continue
        local_path = os.path.join(ROOT_DIR, path)
        if not os.path.isfile(local_path):
            print(f"deploy (tracked but missing: {path})")
            sys.exit(1)
        key = RAW_PREFIX + path
        plan[key] = (local_path, content_type(local_path, key), cache_control(key))
    return plan

def plan_cdn():
    plan = {}
    for prefix in CDN_PREFIXES:
        source = os.path.join(CDN_DIR, prefix)
        if not os.path.isdir(source): continue
        for name in sorted(os.listdir(source)):
            if name == ".DS_Store": continue
            local_path = os.path.join(source, name)
            plan[f"{prefix}/{name}"] = (local_path, content_type(local_path), IMMUTABLE_CACHE)
    return plan

def reconcile(bucket, plan, remote):
    digests = {}
    uploads = []
    for key, (local_path, _, _) in sorted(plan.items()):
        if local_path not in digests: digests[local_path] = file_md5(local_path)
        if remote.get(key) != digests[local_path]: uploads.append(key)
    strays = sorted(key for key in remote if key not in plan and not protected(key))
    def entry(key):
        name = key.rsplit("/", 1)[-1]
        return name.endswith(".html") or name in NO_CACHE_NAMES
    def upload(key):
        local_path, ctype, cache = plan[key]
        put_file(local_path, key, bucket, content_type=ctype, cache_control=cache)
    def upload_all(keys):
        failed = []
        with ThreadPoolExecutor(max_workers=32) as pool:
            futures = {pool.submit(upload, key): key for key in keys}
            for future in as_completed(futures):
                key = futures[future]
                try:
                    future.result()
                    print(f"Uploaded: {key}")
                except Exception as error:
                    failed.append((key, error))
        for key, error in sorted(failed):
            print(f"Failed: {key} ({error})")
        if failed:
            print(f"deploy (sync failed: {len(failed)} of {len(keys)} uploads)")
            sys.exit(1)
    upload_all([key for key in uploads if not entry(key)])
    upload_all([key for key in uploads if entry(key)])
    delete_keys(strays, bucket)
    for key in strays:
        print(f"Deleted: {key}")
    print(f"Sync complete ({len(uploads)} uploaded, {len(strays)} deleted, {len(plan) - len(uploads)} unchanged).")
    return bool(uploads or strays)

def sync(bucket, dist_dir, extra=None):
    plan = plan_dist(dist_dir)
    if not plan:
        print(f"deploy (nothing in '{dist_dir}': skipping sync)")
        return False
    plan = {**plan, **(extra or {})}
    print(f"Syncing s3://{bucket} from '{dist_dir}'...")
    return reconcile(bucket, plan, bucket_etags(bucket))

def sync_cdn(bucket):
    plan = {**plan_cdn(), **plan_content()}
    remote = {}
    for prefix in (*CDN_PREFIXES, RAW_PREFIX.rstrip("/")):
        remote.update(bucket_etags(bucket, f"{prefix}/"))
    remote.update({key: etag for key, etag in bucket_etags(bucket).items() if "/" not in key})
    print(f"Syncing s3://{bucket} (content store)...")
    return reconcile(bucket, plan, remote)

# PLAN

def show_git_plan():
    dist = plan_shell()
    raw = plan_content()
    plan = {**dist, **raw}
    kinds = {}
    for key, (_, ctype, cache) in sorted(plan.items()):
        prefix = RAW_PREFIX if key.startswith(RAW_PREFIX) else "dist/"
        kinds.setdefault((prefix, ctype, cache), 0)
        kinds[(prefix, ctype, cache)] += 1
    print(f"plan: {len(plan)} keys ({len(dist)} dist, {len(raw)} raw)")
    for (prefix, ctype, cache), count in sorted(kinds.items()):
        print(f"  {count:>5}  {prefix:<5}  {ctype:<26}  {cache}")

# TERMINAL

def help():
    commands = [
        ("net build", "math wasm + the React landing site (shell, assets, sitemap) into data/net/dist"),
        ("net push", "build net, sync changed keys to S3, invalidate CloudFront"),
        ("git build", "the React projection (shell, manifest, sitemap, tree) into data/git/dist"),
        ("git plan", "print the shell keys git push syncs and the content keys cdn push syncs"),
        ("git push", "build git, sync the shell to S3, invalidate CloudFront"),
        ("web build", "wasm release + the React face into data/web/dist"),
        ("web push", "build web, sync changed keys to S3, invalidate CloudFront"),
        ("bot build", "the React notebook (shell, assets) into data/bot/dist"),
        ("bot push", "build bot, sync the shell to S3, invalidate CloudFront; notes.json untouched"),
        ("cdn push", "the content store: manifest, tree, site, raw/, tarball, fonts, licenses; cli/ untouched"),
        ("cli push", "the built cli tarballs from data/release/<version>/ to the cdn, point cli/latest at the version"),
    ]
    width = max(len(name) for name, _ in commands)
    print("mrlydeploy")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["net"] | ["net", "build"]: build_net()
        case ["net", "push"]: push_net()
        case ["git"] | ["git", "build"]: build_git()
        case ["git", "plan"]: show_git_plan()
        case ["git", "push"]: push_git()
        case ["web"] | ["web", "build"]: build_web()
        case ["web", "push"]: push_web()
        case ["bot"] | ["bot", "build"]: build_bot()
        case ["bot", "push"]: push_bot()
        case ["cdn"] | ["cdn", "push"]: push_cdn()
        case ["cli"] | ["cli", "push"]: push_cli()
        case _: help()

# AWS

import common
from boto3.s3.transfer import TransferConfig
from botocore.exceptions import ClientError

s3_client = common.s3_client()
cf_client = common.cf_client()

SINGLE_PART = TransferConfig(multipart_threshold=5 * 1024**3)
BATCH = 1000

def put_file(local_path, key, bucket, content_type=None, cache_control=None):
    extra = {}
    if content_type: extra["ContentType"] = content_type
    if cache_control: extra["CacheControl"] = cache_control
    s3_client.upload_file(local_path, bucket, key, ExtraArgs=extra or None, Config=SINGLE_PART)

def bucket_etags(bucket, prefix=""):
    etags = {}
    paginator = s3_client.get_paginator("list_objects_v2")
    for page in paginator.paginate(Bucket=bucket, Prefix=prefix):
        for obj in page.get("Contents", []):
            etags[obj["Key"]] = obj["ETag"].strip('"')
    return etags

def delete_keys(keys, bucket):
    errors = []
    for start in range(0, len(keys), BATCH):
        objects = [{"Key": key} for key in keys[start:start + BATCH]]
        response = s3_client.delete_objects(Bucket=bucket, Delete={"Objects": objects})
        errors.extend(response.get("Errors", []))
    if errors:
        for error in errors:
            print(f"Failed to delete: {error['Key']} ({error.get('Code')})")
        print(f"deploy (delete failed: {len(errors)} of {len(keys)} keys)")
        sys.exit(1)

def invalidate_if_changed(distribution_id, changed, paths=["/*"]):
    if not changed:
        print("Nothing changed: skipping invalidation.")
        return
    invalidate_cloudfront(distribution_id, paths)

def invalidate_cloudfront(distribution_id, paths=["/*"]):
    print(f"Invalidating CloudFront: {distribution_id} {paths}...")
    try:
        response = cf_client.create_invalidation(
            DistributionId=distribution_id,
            InvalidationBatch={
                "Paths": {"Quantity": len(paths), "Items": paths},
                "CallerReference": str(uuid.uuid4()),
            },
        )
        print(f"Invalidation created: {response['Invalidation']['Id']}")
    except ClientError as e:
        print(f"Failed to invalidate: {e}")
        sys.exit(1)

if __name__ == "__main__":
    terminal()
