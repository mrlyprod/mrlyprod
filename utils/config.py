import os
import random
import subprocess

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

BREW_PATHS = ["/opt/homebrew/bin", "/usr/local/bin"]

def run(cmd, cwd=None):
    try:
        return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    except FileNotFoundError:
        return subprocess.CompletedProcess(cmd, returncode=1, stdout="", stderr=f"{cmd[0]} not found")

def ensure_path():
    for p in BREW_PATHS:
        path = os.environ.get("PATH", "")
        if p not in path:
            os.environ["PATH"] = p + ":" + path

def hex_key(k=8):
    return "".join(random.choices("0123456789abcdef", k=k))

if __name__ == "__main__":
    print(ROOT)
