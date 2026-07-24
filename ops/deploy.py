import json
import mimetypes
import os
import re
import shutil
import subprocess
import sys
import uuid
from datetime import date

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIST_DIR = os.path.join(ROOT_DIR, "data", "web", "dist")
WASM_PKG_DIR = os.path.join(ROOT_DIR, "pkgs", "mrlyjs", "pkg")
WASM_FILE = "mrlyjs_bg.wasm"
PUBLIC_DIR = os.path.join(ROOT_DIR, "apps", "web", "public")

SITEMAP_BASE = "https://mrly.net"
CDN_PAGES_DIR = os.path.join(ROOT_DIR, "cdn", "pages")

# ENV

def load_env():
    path = os.path.join(ROOT_DIR, ".env")
    if not os.path.exists(path): return
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"): continue
            key, _, value = line.partition("=")
            os.environ.setdefault(key.strip(), value.strip())

load_env()

# BUILD

def run(cmd, cwd=ROOT_DIR, env=None):
    result = subprocess.run(cmd, cwd=cwd, env=env)
    if result.returncode != 0:
        print(f"deploy (step failed: {' '.join(cmd)})")
        sys.exit(1)

def build_wasm():
    print("Building wasm (release)...")
    run(["wasm-pack", "build", "pkgs/mrlyjs", "--target", "web", "--release"], env={**os.environ, "MRLY_RELEASE": "1"})

def build_bundle():
    print("Bundling web app...")
    if os.path.exists(DIST_DIR): shutil.rmtree(DIST_DIR)
    os.makedirs(DIST_DIR, exist_ok=True)
    run(["bun", "build", "apps/web/index.html", "--outdir", DIST_DIR, "--minify", "--external", "/*"])

def copy_wasm():
    src = os.path.join(WASM_PKG_DIR, WASM_FILE)
    shutil.copy2(src, os.path.join(DIST_DIR, WASM_FILE))

def copy_public():
    if not os.path.exists(PUBLIC_DIR): return
    for name in os.listdir(PUBLIC_DIR):
        if name == ".DS_Store": continue
        src = os.path.join(PUBLIC_DIR, name)
        dst = os.path.join(DIST_DIR, name)
        if os.path.isdir(src):
            shutil.copytree(src, dst, dirs_exist_ok=True)
        else:
            shutil.copy2(src, dst)

# ROUTES

def read_example(name):
    cmd = ["cargo", "run", "-p", "mrlynet", "--example", name]
    result = subprocess.run(cmd, cwd=ROOT_DIR, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"deploy (step failed: {' '.join(cmd)})")
        sys.exit(1)
    return json.loads(result.stdout)

def read_routes():
    return read_example("routes")

def read_pages():
    return read_example("pages")

def write_og():
    cmd = ["cargo", "run", "-p", "mrlynet", "--example", "og", "--", os.path.join(DIST_DIR, "og")]
    result = subprocess.run(cmd, cwd=ROOT_DIR)
    if result.returncode != 0:
        print(f"deploy (step failed: {' '.join(cmd)})")
        sys.exit(1)

def stamped_routes(routes):
    return [r for r in routes if not r["hidden"]]

def esc(value):
    return value.replace("&", "&amp;").replace('"', "&quot;").replace("<", "&lt;").replace(">", "&gt;")

def sub_attr(html, pattern, value):
    return re.sub(pattern, lambda m: m.group(1) + value + m.group(2), html, count=1)

def stamp_meta(base, title, canon, image):
    html = re.sub(r"<title>[^<]*</title>", lambda m: f"<title>{title}</title>", base, count=1)
    html = sub_attr(html, r'(<link[^>]*?rel="canonical"[^>]*?href=")[^"]*(")', canon)
    html = sub_attr(html, r'(<meta property="og:title"[^>]*?content=")[^"]*(")', title)
    html = sub_attr(html, r'(<meta property="og:url"[^>]*?content=")[^"]*(")', canon)
    html = sub_attr(html, r'(<meta property="og:image"[^>]*?content=")[^"]*(")', image)
    html = sub_attr(html, r'(<meta name="twitter:title"[^>]*?content=")[^"]*(")', title)
    html = sub_attr(html, r'(<meta name="twitter:image"[^>]*?content=")[^"]*(")', image)
    return html

def add_jsonld(html, data):
    script = f'<script type="application/ld+json">{json.dumps(data)}</script>'
    return html.replace("</head>", f"{script}</head>", 1)

def stamp_routes(routes):
    with open(os.path.join(DIST_DIR, "index.html")) as f:
        base = f.read()
    count = 0
    for r in stamped_routes(routes):
        route = r["route"]
        title = esc(f"{r['title']} - MrlyProd")
        canon = f"{SITEMAP_BASE}/{route}"
        image = f"{SITEMAP_BASE}/og/{route}.png"
        html = stamp_meta(base, title, canon, image)
        html = add_jsonld(html, {
            "@context": "https://schema.org",
            "@type": "SoftwareApplication",
            "name": r["title"],
            "url": canon,
            "image": image,
        })
        with open(os.path.join(DIST_DIR, f"{route}.html"), "w") as f:
            f.write(html)
        count += 1
    print(f"Stamped {count} route htmls")

def stamp_pages(pages):
    with open(os.path.join(DIST_DIR, "index.html")) as f:
        base = f.read()
    outdir = os.path.join(DIST_DIR, "pages")
    os.makedirs(outdir, exist_ok=True)
    for p in pages:
        slug = p["slug"]
        title = esc(f"{p['title']} - MrlyProd")
        canon = f"{SITEMAP_BASE}/pages/{slug}"
        image = f"{SITEMAP_BASE}/og/pages-{slug}.png"
        html = stamp_meta(base, title, canon, image)
        html = html.replace('<main id="mrly"></main>', f'<main id="mrly">{p["html"]}</main>', 1)
        html = add_jsonld(html, {
            "@context": "https://schema.org",
            "@type": "WebPage",
            "name": p["title"],
            "url": canon,
            "image": image,
        })
        with open(os.path.join(outdir, f"{slug}.html"), "w") as f:
            f.write(html)
    print(f"Stamped {len(pages)} page htmls")

# SITEMAP

def page_slugs():
    if not os.path.exists(CDN_PAGES_DIR): return []
    return sorted(name[:-3] for name in os.listdir(CDN_PAGES_DIR) if name.endswith(".md"))

def sitemap_url(path, today, home=False):
    url = f"  <url><loc>{SITEMAP_BASE}{path}</loc>"
    if home: url += "<priority>1.0</priority>"
    url += f"<lastmod>{today}</lastmod></url>"
    return url

def generate_sitemap(routes):
    today = date.today().isoformat()
    lines = ['<?xml version="1.0" encoding="UTF-8"?>']
    lines.append('<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">')
    lines.append(sitemap_url("/", today, home=True))
    for r in stamped_routes(routes):
        lines.append(sitemap_url(f"/{r['route']}", today))
    for slug in page_slugs():
        lines.append(sitemap_url(f"/pages/{slug}", today))
    lines.append("</urlset>")
    with open(os.path.join(DIST_DIR, "sitemap.xml"), "w") as f:
        f.write("\n".join(lines) + "\n")
    print(f"Generated sitemap.xml ({len(lines) - 3} routes, lastmod={today})")

# SUMMARY

def summarize():
    file_count = 0
    total_bytes = 0
    for root, _, files in os.walk(DIST_DIR):
        for name in files:
            file_count += 1
            total_bytes += os.path.getsize(os.path.join(root, name))
    wasm_size = os.path.getsize(os.path.join(DIST_DIR, WASM_FILE))
    print(f"dist: {file_count} files, {total_bytes / 1024:.1f} KB total, wasm {wasm_size / 1024:.1f} KB")

def build():
    build_wasm()
    build_bundle()
    copy_wasm()
    copy_public()
    routes = read_routes()
    write_og()
    stamp_routes(routes)
    stamp_pages(read_pages())
    generate_sitemap(routes)
    summarize()
    return routes

# CACHE

NO_CACHE_NAMES = {"robots.txt", "sitemap.xml", WASM_FILE}
HASHED_RE = re.compile(r"-[0-9a-z]{8}\.[^.]+$")

IMMUTABLE_CACHE = "public, max-age=31536000, immutable"
DEFAULT_CACHE = "max-age=86400"
NO_CACHE = "no-cache"

def cache_control(name):
    if name.endswith(".html") or name in NO_CACHE_NAMES: return NO_CACHE
    if HASHED_RE.search(name): return IMMUTABLE_CACHE
    return DEFAULT_CACHE

CONTENT_TYPES = {
    ".html": "text/html",
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
    ".wasm": "application/wasm",
}

def content_type(local_path):
    ext = os.path.splitext(local_path)[1]
    return CONTENT_TYPES.get(ext) or mimetypes.guess_type(local_path)[0] or "application/octet-stream"

# PUSH

def push():
    bucket = os.environ.get("MRLYNET_BUCKET")
    distribution_id = os.environ.get("MRLY_ID")
    missing = [key for key, value in (("MRLYNET_BUCKET", bucket), ("MRLY_ID", distribution_id)) if not value]
    if missing:
        print(f"deploy (add to .env: {', '.join(missing)})")
        return
    routes = build()
    sync_site(bucket, routes)
    invalidate_cloudfront(distribution_id)

def extensionless_keys(routes):
    keys = [r["route"] for r in stamped_routes(routes)]
    keys += [f"pages/{slug}" for slug in page_slugs()]
    return keys

def sync_site(bucket, routes):
    print(f"Wiping s3://{bucket}...")
    delete_folder("", bucket)
    print(f"Uploading s3://{bucket} from '{DIST_DIR}'...")
    upload_count = 0
    for root, _, files in os.walk(DIST_DIR):
        for name in files:
            if name == ".DS_Store": continue
            local_path = os.path.join(root, name)
            key = os.path.relpath(local_path, DIST_DIR).replace(os.path.sep, "/")
            put_file(local_path, key, bucket, content_type=content_type(local_path), cache_control=cache_control(name))
            upload_count += 1
            print(f"Uploaded: {key}")
    for key in extensionless_keys(routes):
        local_path = os.path.join(DIST_DIR, f"{key}.html")
        if not os.path.exists(local_path): continue
        put_file(local_path, key, bucket, content_type="text/html", cache_control=NO_CACHE)
        upload_count += 1
        print(f"Uploaded: {key}")
    print(f"Upload complete ({upload_count} files).")

# TERMINAL

def help():
    commands = [
        ("build", "wasm release build + bun bundle into data/web/dist"),
        ("push", "build, wipe and sync S3, invalidate CloudFront"),
    ]
    width = max(len(name) for name, _ in commands)
    print("mrlydeploy")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case [] | ["build"]: build()
        case ["push"]: push()
        case _: help()

# AWS

import boto3
from botocore.exceptions import ClientError

s3_client = boto3.client("s3")
cf_client = boto3.client("cloudfront")

def put_file(local_path, key, bucket, content_type=None, cache_control=None):
    extra = {}
    if content_type: extra["ContentType"] = content_type
    if cache_control: extra["CacheControl"] = cache_control
    s3_client.upload_file(local_path, bucket, key, ExtraArgs=extra or None)

def delete_folder(prefix, bucket):
    paginator = s3_client.get_paginator("list_objects_v2")
    for page in paginator.paginate(Bucket=bucket, Prefix=prefix):
        if "Contents" not in page: continue
        objects = [{"Key": obj["Key"]} for obj in page["Contents"]]
        s3_client.delete_objects(Bucket=bucket, Delete={"Objects": objects})

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

if __name__ == "__main__":
    terminal()
