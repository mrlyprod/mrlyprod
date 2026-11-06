import hashlib
import os
import shutil
import urllib.request
import zipfile

from common import FUNCTIONS, LAYERS, gate, pick, say, verb
from lib import DATA_DIR, HERE, REPO, aws, maybe, save_env, shell

ARCH = "arm64"
FILE_MODE = 0o644 << 16
BUN = "1.4.4"
BUN_TAG = f"bun-v{BUN}"
BUN_ASSET = "bun-linux-aarch64.zip"
BUN_RELEASE = f"https://github.com/oven-sh/bun/releases/download/{BUN_TAG}/{BUN_ASSET}"
BUN_SOURCE = (
    f"https://raw.githubusercontent.com/oven-sh/bun/{BUN_TAG}/packages/bun-lambda"
)
BUN_CARRIED = [("bootstrap", 0o755), ("runtime.ts", 0o644)]
NODE_FILES = ("package.json",)
NODE_DROP = (".bin", ".cache")
NODE_SHA = "node/bun.lock.sha256"
BUN_LAYER_MB = 100
LAYER_BUDGET_MB = 250
LAYERS_DIR = os.path.join(DATA_DIR, "layers")


def stage_dir(key):
    return os.path.join(LAYERS_DIR, key)


def zip_path(key):
    return os.path.join(LAYERS_DIR, f"{key}.zip")


def store(archive, path, arc, mode):
    info = zipfile.ZipInfo(arc, (1980, 1, 1, 0, 0, 0))
    info.external_attr = (0o100000 | mode) << 16
    info.compress_type = zipfile.ZIP_DEFLATED
    with open(path, "rb") as handle:
        archive.writestr(info, handle.read())


def fetch(url, path):
    request = urllib.request.Request(url, headers={"User-Agent": "bun-lambda"})
    with urllib.request.urlopen(request) as response, open(path, "wb") as handle:
        shutil.copyfileobj(response, handle)
    return path


def build_bun(key):
    root = stage_dir(key)
    shutil.rmtree(root, ignore_errors=True)
    os.makedirs(root, exist_ok=True)
    say(f"download {BUN_RELEASE}")
    asset = fetch(BUN_RELEASE, os.path.join(root, BUN_ASSET))
    binary = os.path.join(root, "bun")
    with zipfile.ZipFile(asset) as source:
        found = [n for n in source.namelist() if n.endswith("/bun")]
        if not found:
            raise SystemExit(f"refuse: no bun executable inside {BUN_ASSET}")
        with source.open(found[0]) as inner, open(binary, "wb") as handle:
            shutil.copyfileobj(inner, handle)
    for name, _ in BUN_CARRIED:
        say(f"download {BUN_SOURCE}/{name}")
        fetch(f"{BUN_SOURCE}/{name}", os.path.join(root, name))
    with zipfile.ZipFile(
        zip_path(key), "w", zipfile.ZIP_DEFLATED, compresslevel=9
    ) as archive:
        store(archive, binary, "bun", 0o755)
        for name, mode in BUN_CARRIED:
            store(archive, os.path.join(root, name), name, mode)


def lock_sha(site):
    with open(os.path.join(REPO, "bun.lock"), "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def zipped_sha(path):
    with zipfile.ZipFile(path) as archive:
        return archive.read(NODE_SHA).decode().strip()


def build_node(key):
    site = LAYERS[key]["site"]
    root = stage_dir(key)
    node = os.path.join(root, "node")
    shutil.rmtree(root, ignore_errors=True)
    os.makedirs(node, exist_ok=True)
    for name in NODE_FILES:
        shutil.copyfile(os.path.join(site, name), os.path.join(node, name))
    shell(
        "bun",
        "install",
        "--os",
        "linux",
        "--cpu",
        "arm64",
        cwd=node,
    )
    modules = os.path.join(node, "node_modules")
    for name in NODE_DROP:
        shutil.rmtree(os.path.join(modules, name), ignore_errors=True)
    with open(os.path.join(root, NODE_SHA), "w") as handle:
        handle.write(lock_sha(site) + "\n")
    unzipped = 0
    skipped = 0
    with zipfile.ZipFile(zip_path(key), "w", zipfile.ZIP_DEFLATED) as archive:
        store(archive, os.path.join(root, NODE_SHA), NODE_SHA, 0o644)
        for folder, dirs, files in sorted(os.walk(modules)):
            skipped += sum(
                1 for name in dirs if os.path.islink(os.path.join(folder, name))
            )
            for file in sorted(files):
                full = os.path.join(folder, file)
                if os.path.islink(full):
                    skipped += 1
                    continue
                unzipped += os.path.getsize(full)
                store(archive, full, os.path.relpath(full, root), 0o644)
    total = unzipped / 1e6 + BUN_LAYER_MB
    say(
        f"unzipped {unzipped / 1e6:.1f} MB, {skipped} symlinks skipped, {total:.0f} MB with the bun layer of the {LAYER_BUDGET_MB} MB budget"
    )
    if total > LAYER_BUDGET_MB:
        say(
            f"WARN over the {LAYER_BUDGET_MB} MB unzipped budget: the function will not deploy"
        )


BUILDERS = {"bun": build_bun, "node": build_node}


def build_steps(key, kind):
    if kind == "bun":
        return [
            f"download bun {BUN} aarch64 and the bun-lambda bootstrap, zip bun 0755, bootstrap 0755, runtime.ts 0644"
        ]
    site = os.path.relpath(LAYERS[key]["site"], HERE)
    return [
        f"copy {site}/package.json and bun.lock into {stage_dir(key)}/node",
        "bun install --frozen-lockfile --os linux --cpu arm64 there, drop node_modules/.bin and .cache",
        f"zip node/node_modules and {NODE_SHA} at 0644, symlinks skipped",
        f"warn past {LAYER_BUDGET_MB} MB unzipped with the {BUN_LAYER_MB} MB bun layer",
    ]


def build():
    key = pick(LAYERS, "layer")
    spec = LAYERS[key]
    if not gate(
        f"layers build {key}",
        build_steps(key, spec["kind"]) + [f"write {zip_path(key)}"],
    ):
        return
    os.makedirs(LAYERS_DIR, exist_ok=True)
    BUILDERS[spec["kind"]](key)
    say(f"built {zip_path(key)} ({os.path.getsize(zip_path(key)) / 1e6:.1f} MB)")


def publish():
    key = pick(LAYERS, "layer")
    spec = LAYERS[key]
    path = zip_path(key)
    if not os.path.exists(path):
        raise SystemExit(f"refuse: {path} is missing, run build first")
    described = []
    if spec["kind"] == "node":
        sha = zipped_sha(path)
        if sha != lock_sha(spec["site"]):
            raise SystemExit(
                f"refuse: {path} holds lockfile {sha[:12]}, the checkout holds {lock_sha(spec['site'])[:12]}, run build first"
            )
        described = ["--description", sha]
    if not gate(
        f"layers publish {key}",
        [
            f"publish-layer-version {spec['name']} from {path}",
            f"runtimes {' '.join(spec['runtimes'])}, architecture {ARCH}, no cross-account permission",
            *(
                [f"description {described[1][:12]}, the sha256 of bun.lock"]
                if described
                else []
            ),
            f"save {spec['env']} to the secrets",
        ],
    ):
        return
    data = aws(
        "lambda",
        "publish-layer-version",
        "--layer-name",
        spec["name"],
        "--zip-file",
        f"fileb://{path}",
        "--compatible-runtimes",
        *spec["runtimes"],
        "--compatible-architectures",
        ARCH,
        *described,
    )
    say(f"published version {data['Version']}")
    save_env(spec["env"], data["LayerVersionArn"], repair=True)


def show():
    for key, spec in LAYERS.items():
        data = maybe("lambda", "list-layer-versions", "--layer-name", spec["name"])
        found = (data or {}).get("LayerVersions") or []
        if not found:
            say(f"{key:<20} {spec['name']:<24} none yet")
            continue
        latest = found[0]
        held = os.environ.get(spec["env"], "")
        state = (
            "matches the secrets"
            if held == latest["LayerVersionArn"]
            else (
                f"{spec['env']} holds another version"
                if held
                else f"{spec['env']} missing"
            )
        )
        if spec["kind"] == "node":
            state += (
                ", fresh"
                if latest.get("Description", "") == lock_sha(spec["site"])
                else ", stale against bun.lock"
            )
        say(
            f"{key:<20} {spec['name']:<24} v{latest['Version']} {' '.join(latest.get('CompatibleRuntimes') or [])}  {state}"
        )


def attached(spec, arn):
    out = []
    for name, fn in FUNCTIONS.items():
        if spec["env"] not in fn["layers"]:
            continue
        data = maybe(
            "lambda", "get-function-configuration", "--function-name", fn["name"]
        )
        held = [one["Arn"] for one in (data or {}).get("Layers") or []]
        out.append((name, arn in held))
    return out


def check():
    key = pick(LAYERS, "layer")
    spec = LAYERS[key]
    if spec["kind"] != "node":
        raise SystemExit(f"refuse: {key} is not a node layer")
    data = maybe("lambda", "list-layer-versions", "--layer-name", spec["name"])
    found = (data or {}).get("LayerVersions") or []
    if not found:
        say(f"{key} missing: no version published")
        raise SystemExit(1)
    latest = found[0]
    arn = latest["LayerVersionArn"]
    if latest.get("Description", "") != lock_sha(spec["site"]):
        say(f"{key} stale: v{latest['Version']} was built from another bun.lock")
        raise SystemExit(1)
    if os.environ.get(spec["env"], "") != arn:
        say(f"{key} stale: secret {spec['env']} is not v{latest['Version']}")
        raise SystemExit(1)
    behind = [name for name, held in attached(spec, arn) if not held]
    if behind:
        say(f"{key} stale: v{latest['Version']} is not attached to {' '.join(behind)}")
        raise SystemExit(1)
    say(f"{key} fresh: v{latest['Version']}")


VERBS = {
    "list": (
        show,
        "",
        "the latest version of each layer and whether the secrets hold it; a node layer also reads fresh or stale against bun.lock",
    ),
    "check": (
        check,
        "<node layer>",
        "read-only; exit 1 unless the latest version was built from the checkout's bun.lock, .env holds its arn and every function that lists it has it",
    ),
    "build": (
        build,
        "<layer>",
        "bun: bun 1.4.4 and the bun-lambda bootstrap; node: the checkout's node_modules from bun.lock; zips land in data/layers/ and warn past 250 MB unzipped",
    ),
    "publish": (
        publish,
        "<layer>",
        "publish the zip as a layer version and save its arn to .env; a node layer carries its bun.lock sha256 and refuses a zip built from another",
    ),
}

ABOUT = "Lambda layers from LAYERS in common.py. A mutating verb prints a PLAN; --yes applies it. deploy.py runs check and, when stale, build, publish and fn.py deploy."

if __name__ == "__main__":
    verb(VERBS, ABOUT)()
