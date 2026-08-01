import os
import random
import subprocess

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def run(cmd, cwd=None):
    try:
        return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    except FileNotFoundError:
        return subprocess.CompletedProcess(cmd, returncode=1, stdout="", stderr=f"{cmd[0]} not found")

def hex_key(k=8):
    return "".join(random.choices("0123456789abcdef", k=k))

if __name__ == "__main__":
    print(ROOT)
