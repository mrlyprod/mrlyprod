import os
import posixpath
import re
import sys

TAG = re.compile(r"<(script|style)\b[^>]*>.*?</\1>|<[^>]+>", re.S | re.I)
SHEET = re.compile(r"<link\b[^>]*>", re.I)
MODULE = re.compile(r'<script\b[^>]*type="module"[^>]*>', re.I)
PICTURE = re.compile(r"<picture\b[^>]*>(.*?)</picture>", re.S | re.I)
IMG = re.compile(r"<img\b[^>]*>", re.I)
SOURCE = re.compile(r"<source\b[^>]*>", re.I)
ATTR = re.compile(r'([a-z-]+)="([^"]*)"', re.I)
IMPORT = re.compile(r'(?:^|[\s;}])(?:import|export)[^;\n]*?from\s*["\']([^"\']+)["\']|(?:^|[\s;}])import\s*["\']([^"\']+)["\']')
FACE = re.compile(r"@font-face\s*\{([^}]*)\}", re.S | re.I)
RULE = re.compile(r"([^{}]*)\{([^{}]*)\}", re.S)
QUOTED = re.compile(r'"([^"]+)"')
URL = re.compile(r"url\(\s*['\"]?([^'\")]+)")
RANGE = re.compile(r"U\+([0-9a-f?]+)(?:-([0-9a-f]+))?", re.I)
ICON = ("icon", "apple-touch-icon", "manifest")

def attrs(tag):
    return {k.lower(): v for k, v in ATTR.findall(tag)}

def resolve(base, url):
    if url.startswith("/"):
        return url.lstrip("/")
    return posixpath.normpath(posixpath.join(posixpath.dirname(base), url))

def size(dist, path):
    file = os.path.join(dist, path)
    return os.path.getsize(file) if os.path.exists(file) else 0

def closure(dist, entry):
    seen, stack = [], [entry]
    while stack:
        path = stack.pop()
        if path in seen or not os.path.exists(os.path.join(dist, path)):
            continue
        seen.append(path)
        body = open(os.path.join(dist, path), encoding="utf8", errors="ignore").read()
        for one, two in IMPORT.findall(body):
            url = one or two
            if url.startswith(".") or url.startswith("/"):
                stack.append(resolve(path, url))
    return seen

def words(html):
    text = TAG.sub(" ", html)
    for code, char in [("&amp;", "&"), ("&lt;", "<"), ("&gt;", ">"), ("&quot;", '"'), ("&#39;", "'"), ("&nbsp;", " ")]:
        text = text.replace(code, char)
    return {ord(c) for c in text if ord(c) > 0xFF}

def ranges(spec):
    out = []
    for lo, hi in RANGE.findall(spec):
        if "?" in lo:
            out.append((int(lo.replace("?", "0"), 16), int(lo.replace("?", "f"), 16)))
        else:
            out.append((int(lo, 16), int(hi, 16) if hi else int(lo, 16)))
    return out

def families(css):
    out = set()
    for selector, body in RULE.findall(FACE.sub(" ", css)):
        if "[data-font" in selector:
            continue
        out.update(QUOTED.findall(body))
    return out

def fonts(dist, sheets, text):
    css = "".join(open(os.path.join(dist, path), encoding="utf8", errors="ignore").read() for path in sheets if os.path.exists(os.path.join(dist, path)))
    named = families(css)
    out = []
    for path in sheets:
        body = open(os.path.join(dist, path), encoding="utf8", errors="ignore").read() if os.path.exists(os.path.join(dist, path)) else ""
        for block in FACE.findall(body):
            name = QUOTED.search(block)
            src = URL.search(block)
            if not name or not src or name.group(1) not in named:
                continue
            span = re.search(r"unicode-range\s*:([^;]+);", block, re.I)
            if span and not any(lo <= c <= hi for lo, hi in ranges(span.group(1)) for c in text):
                continue
            out.append(resolve(path, src.group(1)))
    return out

def pick(html, dark):
    eager, lazy = [], []
    rest = html
    for block in PICTURE.finditer(html):
        rest = rest.replace(block.group(0), " ")
        img = IMG.search(block.group(1))
        a = attrs(img.group(0)) if img else {}
        shown = a.get("src", "")
        for tag in SOURCE.findall(block.group(1)):
            s = attrs(tag)
            if dark and "prefers-color-scheme: dark" in s.get("media", ""):
                shown = s.get("srcset", "").split(",")[0].strip().split(" ")[0]
        if shown:
            (lazy if a.get("loading") == "lazy" else eager).append(shown)
    for tag in IMG.findall(rest):
        a = attrs(tag)
        src = a.get("src", "")
        if src and not src.startswith("data:"):
            (lazy if a.get("loading") == "lazy" else eager).append(src)
    return eager, lazy

def weigh(dist, route, dark):
    page = posixpath.join(route.strip("/"), "index.html").lstrip("/")
    html = open(os.path.join(dist, page), encoding="utf8").read()
    sheets = []
    for tag in SHEET.findall(html):
        a = attrs(tag)
        if a.get("rel") == "stylesheet" and a.get("href") and a.get("rel") not in ICON:
            sheets.append(resolve(page, a["href"]))
    scripts = []
    for tag in MODULE.findall(html):
        src = attrs(tag).get("src")
        if src:
            scripts += closure(dist, resolve(page, src))
    eager, lazy = pick(html, dark)
    faces = fonts(dist, sheets, words(html))
    kinds = [("html", [page]), ("css", sheets), ("js", scripts), ("img", [resolve(page, url) for url in eager]), ("lazy", [resolve(page, url) for url in lazy]), ("font", faces)]
    return [(kind, sorted(set(paths)), sum(size(dist, path) for path in sorted(set(paths)))) for kind, paths in kinds]

def main():
    if len(sys.argv) < 3:
        print("weight.py <dist> <route> [--files]")
        return 1
    dist, route = sys.argv[1], sys.argv[2]
    for dark in (False, True):
        rows = weigh(dist, route, dark)
        print(f"{route} {'dark' if dark else 'light'}  {dist}")
        for kind, paths, total in rows:
            print(f"  {kind:5} {total / 1024:9.1f} KB  {len(paths)}")
            if "--files" in sys.argv:
                for path in paths:
                    print(f"        {size(dist, path) / 1024:9.1f} KB  {path}")
        print(f"  {'total':5} {sum(row[2] for row in rows) / 1024:9.1f} KB")
    return 0

if __name__ == "__main__":
    sys.exit(main())
