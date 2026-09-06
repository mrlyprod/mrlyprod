import os
import shutil
import tempfile
import urllib.request
import zipfile

from common import REGION, aws, gate, say, verb

# LAYER

NAME = "bun"
BUN = "1.3.14"
ARCH = "aarch64"
ARCHITECTURE = "arm64"
RUNTIMES = ["provided.al2", "provided.al2023"]
DESCRIPTION = f"Bun {BUN} {ARCH}"
TAG = f"bun-v{BUN}"
ASSET = f"bun-linux-{ARCH}.zip"
RELEASE = f"https://github.com/oven-sh/bun/releases/download/{TAG}/{ASSET}"
SOURCE = f"https://raw.githubusercontent.com/oven-sh/bun/{TAG}/packages/bun-lambda"
CARRIED = [("bootstrap", 0o755), ("runtime.ts", 0o644)]
AGENT = "bun-lambda"
EXEC_MODE = 0o755
REGULAR = 0o100000

# BUILD

def fetch(url, path):
    request = urllib.request.Request(url, headers={"User-Agent": AGENT})
    with urllib.request.urlopen(request) as response, open(path, "wb") as handle:
        shutil.copyfileobj(response, handle)
    return path

def unpack(archive, work):
    path = os.path.join(work, NAME)
    with zipfile.ZipFile(archive) as source:
        found = [n for n in source.namelist() if n.endswith(f"/{NAME}")]
        if not found:
            raise SystemExit(f"refuse: no {NAME} executable inside {ASSET}")
        with source.open(found[0]) as binary, open(path, "wb") as handle:
            shutil.copyfileobj(binary, handle)
    os.chmod(path, EXEC_MODE)
    return path

def store(target, path, name, mode):
    info = zipfile.ZipInfo(name)
    info.compress_type = zipfile.ZIP_DEFLATED
    info.external_attr = (REGULAR | mode) << 16
    with open(path, "rb") as handle, target.open(info, "w") as entry:
        shutil.copyfileobj(handle, entry)

def assemble(work):
    say(f"download {RELEASE}")
    binary = unpack(fetch(RELEASE, os.path.join(work, ASSET)), work)
    for name, _ in CARRIED:
        say(f"download {SOURCE}/{name}")
        fetch(f"{SOURCE}/{name}", os.path.join(work, name))
    layer = os.path.join(work, f"{NAME}-lambda-layer.zip")
    with zipfile.ZipFile(layer, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as target:
        store(target, binary, NAME, EXEC_MODE)
        for name, mode in CARRIED:
            store(target, os.path.join(work, name), name, mode)
    return layer

# VERBS

def publish():
    if not gate("publish", [
        f"download {ASSET} from the {TAG} release",
        f"download bootstrap and runtime.ts from packages/bun-lambda at {TAG}",
        f"zip {NAME} 0755, bootstrap 0755, runtime.ts 0644 at the zip root",
        f"publish layer {NAME} to {REGION} for {ARCHITECTURE}, {' '.join(RUNTIMES)}",
        "no add-layer-version-permission, so the layer stays private",
    ]): return
    with tempfile.TemporaryDirectory() as work:
        layer = assemble(work)
        say(f"{os.path.basename(layer)} {os.path.getsize(layer) / 1_000_000:.1f} MB")
        data = aws(
            "lambda", "publish-layer-version",
            "--layer-name", NAME,
            "--description", DESCRIPTION,
            "--zip-file", f"fileb://{layer}",
            "--compatible-runtimes", *RUNTIMES,
            "--compatible-architectures", ARCHITECTURE,
            region=REGION,
        )
    say(data["LayerVersionArn"])

def status():
    data = aws("lambda", "list-layer-versions", "--layer-name", NAME, region=REGION)
    versions = (data or {}).get("LayerVersions") or []
    if not versions:
        say(f"no {NAME} layer in {REGION}")
        return
    for item in versions:
        arches = " ".join(item.get("CompatibleArchitectures") or [])
        runtimes = " ".join(item.get("CompatibleRuntimes") or [])
        say(f"{item['Version']:<3} {item.get('Description', ''):<20} {arches:<6} {runtimes}")
        say(f"    {item['LayerVersionArn']}  {item['CreatedDate']}")

# MAIN

VERBS = {"publish": publish, "status": status}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
