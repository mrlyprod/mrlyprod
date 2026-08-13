import os
import sys
import tomllib
from config import ROOT, crates

CORE = "mrlycore"
OS = "mrlyos"
MATH = "mrlymath"
FONT = "mrlyfont"
SKIN = "mrlyskin"
GAME = "mrlygame"
WEB = "mrlyweb"
CLI = "mrlycli"
MOJI = "mrlymoji"
RUN = "mrlyrun"
SOLO = "mrlysolo"
TORCH = "mrlytorch"
DATA = "mrlydata"

LAW = {
    CORE: set(),
    OS: {CORE},
    MATH: {CORE},
    FONT: {CORE},
    SKIN: {CORE, MATH, FONT},
    GAME: {CORE, MATH},
    MOJI: set(),
    RUN: set(),
    SOLO: set(),
    TORCH: set(),
    DATA: {CORE, OS, MATH, FONT, SKIN, WEB},
}
APP_MAY = {CORE, OS, MATH, FONT, SKIN}
BIND_MAY = {CORE, OS, FONT, WEB}

TABLES = ("dependencies", "dev-dependencies", "build-dependencies")

def manifest(path):
    with open(path, "rb") as f:
        data = tomllib.load(f)
    tables = [data.get(t, {}) for t in TABLES]
    for target in data.get("target", {}).values():
        tables += [target.get(t, {}) for t in TABLES]
    deps = {dep for table in tables for dep in table if dep.startswith("mrly")}
    return data.get("package", {}).get("name"), deps

def main():
    manifests = [(kind, os.path.join(path, "Cargo.toml")) for kind, path in crates()]
    found = [(kind, path) + manifest(path) for kind, path in manifests]
    apps = {name for kind, _, name, _ in found if kind == "app"}
    violations = []
    edges = 0
    for kind, path, name, deps in found:
        rel = os.path.relpath(path, ROOT)
        edges += len(deps)
        if kind == "app":
            stray = deps - APP_MAY
            note = "an app rests on core, os, math, font, skin and nothing else"
        elif kind == "bind":
            stray = deps - BIND_MAY
            note = "a binding rests on core, os, font, web"
        elif name == WEB:
            stray = deps & {CLI}
            note = "the web carries every app, never the cli"
            for app in sorted(apps - deps):
                violations.append(f"{rel}: {name} forgets {app} (the web carries every app)")
        elif name == CLI:
            stray = deps & apps
            note = "the cli reaches an app only through the web"
        elif name in LAW:
            stray = deps - LAW[name]
            note = f"{name} rests on {', '.join(sorted(LAW[name])) or 'nothing'}"
        else:
            violations.append(f"{rel}: {name} has no place in the law")
            continue
        for dep in sorted(stray):
            violations.append(f"{rel}: {name} -> {dep} ({note})")
    if violations:
        for v in violations:
            print(v)
        sys.exit(1)
    print(f"layers (clean, {len(found)} crates, {edges} edges)")

if __name__ == "__main__":
    main()
