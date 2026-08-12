import os
import sys
import tomllib
from config import ROOT, crates

MIT = os.path.join(ROOT, "files", "MIT.txt")
WORKSPACE = {"workspace": True}

def manifest(path):
    with open(path, "rb") as f:
        return tomllib.load(f)

def main():
    violations = []
    stamp = open(MIT).read()
    workspace = manifest(os.path.join(ROOT, "Cargo.toml"))
    if workspace.get("workspace", {}).get("package", {}).get("license") != "MIT":
        violations.append("Cargo.toml: the workspace license is not MIT")
    found = crates()
    packages = []
    for crate in (path for kind, path in found if kind != "bind"):
        package = manifest(os.path.join(crate, "Cargo.toml")).get("package", {})
        rel = os.path.relpath(crate, ROOT)
        field = package.get("license")
        if field is not None and field != "MIT" and field != WORKSPACE:
            violations.append(f"{rel}: license is not MIT")
        if package.get("publish") is False:
            continue
        packages.append(crate)
        if field != "MIT" and field != WORKSPACE:
            violations.append(f"{rel}: a package at the door carries no license field")
        if not str(package.get("description", "")).strip():
            violations.append(f"{rel}: a package at the door carries no description")
        if package.get("readme") != "README.md":
            violations.append(f"{rel}: a package at the door names no readme")
    for crate in (path for kind, path in found if kind == "bind"):
        packages.append(crate)
        rel = os.path.relpath(crate, ROOT)
        package = manifest(os.path.join(crate, "Cargo.toml")).get("package", {})
        if package.get("license") != WORKSPACE:
            violations.append(f"{rel}: license is not the workspace MIT")
        if not str(package.get("description", "")).strip():
            violations.append(f"{rel}: a package at the door carries no description")
        pyproject = os.path.join(crate, "pyproject.toml")
        if os.path.isfile(pyproject):
            project = manifest(pyproject).get("project", {})
            if project.get("license") != "MIT":
                violations.append(f"{os.path.relpath(pyproject, ROOT)}: license is not MIT")
    for crate in packages:
        rel = os.path.relpath(crate, ROOT)
        if not os.path.isfile(os.path.join(crate, "README.md")):
            violations.append(f"{rel}: a package at the door carries no README.md")
        path = os.path.join(crate, "LICENSE")
        if not os.path.isfile(path) or open(path).read() != stamp:
            with open(path, "w") as f:
                f.write(stamp)
            print(f"stamped {os.path.relpath(path, ROOT)}")
    if violations:
        for v in violations:
            print(v)
        sys.exit(1)
    print(f"licenses ({len(packages)} packages, MIT)")

if __name__ == "__main__":
    main()
