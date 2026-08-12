import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def crates():
    rs = os.path.join(ROOT, "pkgs", "rs")
    out = []
    for entry in sorted(os.listdir(rs)):
        path = os.path.join(rs, entry)
        if entry != "apps" and os.path.isfile(os.path.join(path, "Cargo.toml")):
            out.append(("lib", path))
    apps = os.path.join(rs, "apps")
    for entry in sorted(os.listdir(apps)):
        path = os.path.join(apps, entry)
        if os.path.isfile(os.path.join(path, "Cargo.toml")):
            out.append(("app", path))
    for lang in ("py", "js"):
        path = os.path.join(ROOT, "pkgs", lang, "web")
        if os.path.isfile(os.path.join(path, "Cargo.toml")):
            out.append(("bind", path))
    return out

if __name__ == "__main__":
    print(ROOT)
