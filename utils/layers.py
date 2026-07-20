import os
import re
import sys

SRC = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "pkgs",
    "mrlyrs",
    "src",
)

ALLOWED = {
    "core": {"core"},
    "math": {"core", "math", "io"},
    "physics": {"core", "math", "physics"},
    "crypto": {"core", "math", "io", "crypto"},
    "font": {"core", "font"},
    "music": {"core", "music"},
    "os": {"core", "os"},
    "io": {"core", "os", "io"},
    "sys": {"core", "os", "sys"},
    "apps": {"core", "math", "physics", "crypto", "font", "music", "os", "ui", "apps"},
    "net": {"core", "math", "os", "apps", "net"},
    "ui": {"core", "math", "font", "crypto", "ui"},
}

PATH = re.compile(r"\$?crate::((?:[A-Za-z_][A-Za-z0-9_]*::)*)(\{[^}]*\}|[A-Za-z_][A-Za-z0-9_]*)")

# COLLECT

def files():
    out = []
    for root, _, names in os.walk(SRC):
        for name in names:
            if name.endswith(".rs"):
                out.append(os.path.join(root, name))
    return sorted(out)

def refs(path):
    with open(path) as f:
        for lineno, line in enumerate(f, 1):
            if line.strip() == "#[cfg(test)]":
                return
            for m in PATH.finditer(line):
                segments = [s for s in m.group(1).split("::") if s] + [m.group(2)]
                yield lineno, m.group(0), segments

# RULES

def own(path):
    rel = os.path.relpath(path, SRC)
    parts = rel.split(os.sep)
    return rel, (parts[0] if len(parts) > 1 else None), parts

def family_depth(segments, start):
    tail = segments[start:]
    depth = 0
    for s in tail:
        if s.startswith("{") or s[0].isupper():
            break
        depth += 1
    return depth

def check(path, results):
    rel, layer, parts = own(path)
    if layer is None or layer not in ALLOWED:
        return
    for lineno, text, segments in refs(path):
        target = segments[0]
        if target not in ALLOWED:
            continue
        if target not in ALLOWED[layer]:
            results["violations"].append(f"{rel}:{lineno}: {text} ({layer} -> {target})")
            continue
        results["count"] += 1
        if target in ("math", "crypto") and len(segments) > 2:
            fam = segments[1]
            if not (layer == target and len(parts) > 2 and parts[1] == fam):
                if family_depth(segments, 2) > 1:
                    results["violations"].append(f"{rel}:{lineno}: {text} ({layer} -> math/{fam} internals)")
        if target == "apps" and layer == "apps" and len(segments) > 1:
            name = segments[1]
            if not name.startswith("{") and not name[0].isupper() and len(parts) > 2 and parts[1] != name:
                results["violations"].append(f"{rel}:{lineno}: {text} (apps/{parts[1]} -> apps/{name})")

# TERMINAL

def main():
    results = {"count": 0, "violations": []}
    scanned = files()
    for path in scanned:
        check(path, results)
    if results["violations"]:
        for v in results["violations"]:
            print(v)
        sys.exit(1)
    print(f"layers (clean, {len(scanned)} files, {results['count']} imports)")

if __name__ == "__main__":
    main()
