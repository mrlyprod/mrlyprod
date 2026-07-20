import glob
import os
import shutil
import subprocess
import sys
from config import ROOT, run

RS = os.path.join(ROOT, "pkgs", "mrlyrs")
JS = os.path.join(ROOT, "pkgs", "mrlyjs")
PY = os.path.join(ROOT, "pkgs", "mrlypy")
OUT = os.path.join(ROOT, "data", "parity")

def _node():
    found = shutil.which("node")
    if found:
        return found
    hits = sorted(glob.glob(os.path.expanduser("~/.nvm/versions/node/*/bin/node")))
    return hits[-1] if hits else None

def _rust():
    trace = run(["cargo", "run", "--quiet", "--example", "parity"], cwd=RS)
    if trace.returncode != 0:
        return None, trace.stderr.strip()
    return trace.stdout, None

def _wasm():
    node = _node()
    if not node:
        return None, "node not found"
    build = run(["wasm-pack", "build", JS, "--target", "nodejs"])
    if build.returncode != 0:
        return None, build.stderr.strip()
    trace = run([node, os.path.join(JS, "parity.mjs")])
    if trace.returncode != 0:
        return None, trace.stderr.strip()
    return trace.stdout, None

def _maturin():
    beside = os.path.join(os.path.dirname(sys.executable), "maturin")
    if os.path.exists(beside):
        return beside
    return shutil.which("maturin")

def _python():
    maturin = _maturin()
    if not maturin:
        return None, "maturin not found (run via uv)"
    env = dict(os.environ, VIRTUAL_ENV=sys.prefix)
    env["PATH"] = os.path.dirname(sys.executable) + os.pathsep + env.get("PATH", "")
    build = subprocess.run([maturin, "develop", "--release"], cwd=PY, env=env, capture_output=True, text=True)
    if build.returncode != 0:
        return None, build.stderr.strip()
    trace = run([sys.executable, os.path.join(PY, "parity.py")])
    if trace.returncode != 0:
        return None, trace.stderr.strip()
    return trace.stdout, None

def _first_diff(a, b):
    al, bl = a.splitlines(), b.splitlines()
    for i in range(max(len(al), len(bl))):
        x = al[i] if i < len(al) else "<missing>"
        y = bl[i] if i < len(bl) else "<missing>"
        if x != y:
            return i, x, y
    return None

def parity():
    os.makedirs(OUT, exist_ok=True)
    lanes = [("rust", _rust), ("js", _wasm), ("py", _python)]
    traces = {}
    for name, build in lanes:
        print(f"parity: {name}...", end=" ", flush=True)
        text, err = build()
        if err:
            print(f"FAIL ({err})")
            return 1
        with open(os.path.join(OUT, f"{name}.txt"), "w") as f:
            f.write(text)
        traces[name] = text
        print(f"ok ({len(text.splitlines())} lines)")
    base = traces["rust"]
    for name in ("js", "py"):
        diff = _first_diff(base, traces[name])
        if diff:
            i, x, y = diff
            print(f"\nparity: rust != {name} at line {i}")
            print(f"  rust: {x}")
            print(f"  {name:>4}: {y}")
            return 1
    print(f"\nparity: rust == js == py  ({len(base.splitlines())} lines, {len(base)} bytes)  PASS")
    return 0

if __name__ == "__main__":
    sys.exit(parity())
