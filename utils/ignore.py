import fnmatch
import os
from config import ROOT

ALWAYS = {".git"}

def patterns():
    path = os.path.join(ROOT, ".gitignore")
    lines = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#"):
                lines.append(line.rstrip("/"))
    return lines

PATTERNS = patterns()

def is_ignored(rel):
    name = os.path.basename(rel)
    if name in ALWAYS:
        return True
    for p in PATTERNS:
        if p.startswith("/"):
            if rel == p[1:]:
                return True
        elif fnmatch.fnmatch(name, p):
            return True
    return False
