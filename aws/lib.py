import json
import os
import subprocess
import tempfile


def where(key):
    value = os.environ.get(key)
    if not value:
        raise SystemExit(f"refuse: {key} is not set; zshrc exports it")
    return value


HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)

NET = os.path.join(REPO, "sites", "net")
ENGINE = os.path.join(REPO, "os")
UI = os.path.join(REPO, "ui")
ORG = os.path.join(REPO, "sites", "org")
COM = os.path.join(REPO, "sites", "com")
DATA_DIR = os.path.join(REPO, "data", os.path.relpath(HERE, REPO))
ENV_FILE = where("MRLY_ENV")

SHELL_ENV = dict(os.environ)


def env(key, fallback=""):
    return os.environ.get(key) or fallback


def need(key):
    value = env(key)
    if not value:
        raise SystemExit(
            f"refuse: {key} is not set; add it to the file MRLY_ENV names and run with uv run --env-file"
        )
    return value


def save_env(key, value, repair=False):
    held = env(key)
    if held == value:
        print(f"skip {key}, already set")
        return
    if held and not repair:
        print(f"skip {key}, already holds a value")
        return
    lines = []
    if os.path.exists(ENV_FILE):
        with open(ENV_FILE) as handle:
            lines = handle.read().splitlines()
    lines = [line for line in lines if not line.startswith(f"{key}=")]
    lines.append(f"{key}={value}")
    with open(ENV_FILE, "w") as handle:
        handle.write("\n".join(lines) + "\n")
    os.chmod(ENV_FILE, 0o600)
    os.environ[key] = value
    print(f"{key} {'repaired' if held else 'written'} in the file MRLY_ENV names")


def shell(*args, cwd=None, environ=None):
    cmd = [str(a) for a in args]
    done = subprocess.run(
        cmd, capture_output=True, text=True, cwd=cwd, env=environ, check=False
    )
    if done.returncode != 0:
        raise RuntimeError(" ".join(cmd) + "\n" + done.stderr.strip())
    return done.stdout


def stream(*args, cwd=None, environ=None):
    cmd = [str(a) for a in args]
    if subprocess.run(cmd, cwd=cwd, env=environ, check=False).returncode != 0:
        raise RuntimeError(" ".join(cmd))


def tmpfile(text, suffix):
    handle, path = tempfile.mkstemp(suffix=suffix)
    os.write(handle, text.encode())
    os.close(handle)
    return path


def head(repo):
    done = subprocess.run(
        ["git", "-C", repo, "rev-parse", "HEAD"],
        capture_output=True,
        text=True,
        check=False,
    )
    return done.stdout.strip() if done.returncode == 0 else ""


def slug(repo):
    return "/".join(os.path.normpath(repo).split(os.sep)[-2:])


REGION = env("AWS_DEFAULT_REGION", "us-east-2")
EAST = "us-east-1"


def aws(*args, region=None, text=False):
    cmd = ["aws", *[str(a) for a in args], "--region", region or REGION]
    if not text:
        cmd += ["--output", "json"]
    done = subprocess.run(cmd, capture_output=True, text=True, check=False)
    if done.returncode != 0 and (done.returncode != 1 or done.stderr.strip()):
        raise RuntimeError(" ".join(cmd) + "\n" + done.stderr.strip())
    if text:
        return done.stdout
    out = done.stdout.strip()
    return json.loads(out) if out else None


def maybe(*args, region=None, text=False):
    try:
        return aws(*args, region=region, text=text)
    except RuntimeError:
        return None
