import os
import re
import shutil
import sys
import urllib.request
import mrlyweb as mp
from PIL import Image
import font
from config import ROOT

FILES = os.path.join(ROOT, "files")
BRAND = os.path.join(FILES, "brand")
VENDOR = os.path.join(FILES, "vendor")
MRLYFONT = os.path.join(FILES, "mrlyfont")
CDN = os.path.join(ROOT, "data", "cdn")
CDN_URL = "https://cdn.mrly.net"
CELL = 200

# MANIFEST

FAMILIES = {
    "mono": "JetBrains Mono",
    "sans": "Inter",
    "serif": "Lora",
    "display": "Silkscreen",
}

def site_icons():
    return [
        "account_tree",
        "alternate_email",
        "brightness_auto",
        "chevron_right",
        "content_copy",
        "custom_typography",
        "dark_mode",
        "explore",
        "gavel",
        "groups",
        "home",
        "info",
        "light_mode",
        "mail",
        "markdown",
        "more_vert",
        "raw_on",
        "search",
        "settings",
        "shield",
        "toc",
        "volunteer_activism",
    ]

def web_icons():
    with open(os.path.join(ROOT, "apps", "web", "src", "icons.ts"), encoding="utf-8") as f:
        body = f.read().split("{", 1)[1].split("}", 1)[0]
    return sorted(set(re.findall(r'"([a-z0-9_]+)"', body)))

def ui_icons():
    with open(os.path.join(ROOT, "pkgs", "mrlyui", "src", "Glyphs.tsx"), encoding="utf-8") as f:
        body = f.read().split("SymbolName =", 1)[1].split("// SIZE", 1)[0]
    names = set(re.findall(r'"([a-z0-9_]+)"', body))
    return sorted(names | set(site_icons()) | set(web_icons()))

MANIFEST = {
    "net": {
        "public": "apps/net/public",
        "brand": ["favicon.ico", "mark.svg", "icons/mrly_192_192.png", "icons/mrly_512_512.png"],
        "copy": [],
        "css": {},
    },
    "git": {
        "public": "apps/git/public",
        "brand": ["favicon.ico", "mark.svg", "icons/mrly_192_192.png", "icons/mrly_512_512.png"],
        "copy": [],
        "css": {},
    },
    "web": {
        "public": "apps/web/public",
        "brand": ["favicon.ico", "mrlyprod.png", "mrlyprod.svg", "icons/mrly_192_192.png", "icons/mrly_512_512.png"],
        "copy": [],
        "css": {
            "fonts.css": ["mono", "sans", "serif", "display", "mrlyfont"],
            "icons.css": ["icons"],
            "emoji.css": ["emoji"],
        },
    },
    "jsx": {
        "public": "apps/jsx/public",
        "brand": ["favicon.ico", "mrlyprod.png", "mrlyprod.svg", "icons/mrly_192_192.png", "icons/mrly_512_512.png"],
        "copy": [
            ("mrlyfont/MrlyFont.json", "apps/jsx/src/lib/mrlyfont.json"),
        ],
        "css": {
            "fonts.css": ["mrlyfont"],
        },
    },
}

SUBSETS = {
    "site.woff2": site_icons,
    "icons.woff2": web_icons,
    "ui.woff2": ui_icons,
}

UI_FACES = ["mono", "sans", "serif", "display", "mrlyfont", "emoji", "ui", "seti"]

PACKAGES = {
    "pkgs/mrlycss/faces.css": ["mono", "mrlyfont", "site", "seti"],
    "pkgs/mrlyui/styles/faces.css": UI_FACES,
}

# FETCH

UA = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0 Safari/537.36"
CSS2 = "https://fonts.googleapis.com/css2"
OFL = "https://raw.githubusercontent.com/google/fonts/main/ofl/{}/OFL.txt"
APACHE = "https://raw.githubusercontent.com/google/material-design-icons/master/LICENSE"
NOTO_TTF = "https://raw.githubusercontent.com/googlefonts/noto-emoji/main/fonts/NotoColorEmoji.ttf"
MATERIAL = "https://raw.githubusercontent.com/google/material-design-icons/master/variablefont/MaterialSymbolsOutlined%5BFILL%2CGRAD%2Copsz%2Cwght%5D"
SYMBOLS2_TTF = "https://raw.githubusercontent.com/google/fonts/main/ofl/notosanssymbols2/NotoSansSymbols2-Regular.ttf"
AXES = "Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0"

def get(url):
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req) as res:
        return res.read()

def save(name, data):
    with open(os.path.join(VENDOR, name), "wb") as f:
        f.write(data)
    print(f"vendor/{name} ({len(data)} bytes)")

def blocks(css):
    return re.findall(r"@font-face\s*\{[^}]+\}", css)

def url_of(block):
    return re.search(r"url\((\S+?)\)", block).group(1)

def subset(name, names):
    css = get(f"{CSS2}?family={AXES}&icon_names={','.join(sorted(names()))}").decode()
    save(name, get(url_of(blocks(css)[0])))

def subsets():
    for name, names in SUBSETS.items():
        subset(name, names)

def emoji():
    css = get(f"{CSS2}?family=Noto+Color+Emoji").decode()
    out = []
    for i, block in enumerate(blocks(css)):
        name = f"emoji.{i}.woff2"
        save(name, get(url_of(block)))
        block = block.replace(url_of(block), name)
        block = block.replace("font-family: 'Noto Color Emoji';", "font-family: 'noto';")
        out.append(block)
    save("emoji.css", ("\n".join(out) + "\n").encode())
    save("emoji.ttf", get(NOTO_TTF))

def symbols():
    save("symbols.ttf", get(MATERIAL + ".ttf"))
    save("symbols.codepoints", get(MATERIAL + ".codepoints"))
    save("symbols2.ttf", get(SYMBOLS2_TTF))

def fonts():
    out = []
    for role, family in FAMILIES.items():
        css = get(f"{CSS2}?family={family.replace(' ', '+')}:wght@400").decode()
        latin = [b for c, b in re.findall(r"/\* ([a-z-]+) \*/\s*(@font-face\s*\{[^}]+\})", css) if c == "latin"]
        block = latin[-1] if latin else blocks(css)[-1]
        name = f"{role}.woff2"
        save(name, get(url_of(block)))
        block = block.replace(url_of(block), name)
        block = re.sub(r"font-family: '[^']+';", f"font-family: 'mrly-{role}';", block)
        out.append(block)
    save("fonts.css", ("\n".join(out) + "\n").encode())

def licenses():
    save("LICENSE-icons.txt", get(APACHE))
    save("LICENSE-emoji.txt", get(OFL.format("notocoloremoji")))
    save("LICENSE-symbols2.txt", get(OFL.format("notosanssymbols2")))
    for role, family in FAMILIES.items():
        save(f"LICENSE-{role}.txt", get(OFL.format(family.lower().replace(" ", ""))))

def fetch():
    os.makedirs(VENDOR, exist_ok=True)
    subsets()
    emoji()
    symbols()
    fonts()
    licenses()

# MARK

def mark():
    return mp.read(mp.boot(), "font/glyphs/X")

def runs(row):
    spans = []
    start = None
    for x, bit in enumerate(row + "0"):
        if bit == "1" and start is None:
            start = x
        if bit != "1" and start is not None:
            spans.append((start, x - start))
            start = None
    return spans

def mark_svg(rows):
    lines = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 5 5">']
    lines.append("<style>svg{color:#000}@media(prefers-color-scheme:dark){svg{color:#fff}}</style>")
    for y, row in enumerate(rows):
        for x, w in runs(row):
            lines.append(f'<rect x="{x}" y="{y}" width="{w}" height="1" fill="currentColor"/>')
    lines.append("</svg>")
    return "\n".join(lines) + "\n"

def grid_svg(rows):
    size = len(rows) * CELL
    lines = [f'<svg width="{size}" height="{size}" xmlns="http://www.w3.org/2000/svg">']
    for y, row in enumerate(rows):
        for x, bit in enumerate(row):
            fill = "#000000" if bit == "1" else "#ffffff"
            lines.append(
                f'<rect x="{x * CELL}" y="{y * CELL}" width="{CELL}" height="{CELL}" fill="{fill}" stroke="none"/>'
            )
    lines.append("</svg>")
    return "\n".join(lines) + "\n"

def tile(rows):
    im = Image.new("RGB", (len(rows[0]), len(rows)))
    for y, row in enumerate(rows):
        for x, bit in enumerate(row):
            im.putpixel((x, y), (0, 0, 0) if bit == "1" else (255, 255, 255))
    return im

def scaled(im, size):
    return im.resize((size, size), Image.NEAREST)

def save_text(name, text):
    path = os.path.join(BRAND, name)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    print(f"Saved: {path}")

def save_image(im, name, **kwargs):
    path = os.path.join(BRAND, name)
    im.save(path, **kwargs)
    print(f"Saved: {path}")

def render():
    os.makedirs(os.path.join(BRAND, "icons"), exist_ok=True)
    rows = mark()
    save_text("mark.svg", mark_svg(rows))
    save_text("mrlyprod.svg", grid_svg(rows))
    im = tile(rows)
    save_image(scaled(im, 1000), "mrlyprod.png")
    save_image(scaled(im, 192), os.path.join("icons", "mrly_192_192.png"))
    save_image(scaled(im, 512), os.path.join("icons", "mrly_512_512.png"))
    save_image(
        scaled(im, 32),
        "favicon.ico",
        sizes=[(16, 16), (32, 32)],
        append_images=[scaled(im, 16)],
    )

# CDN

def fnv(data):
    h = 0xCBF29CE484222325
    for b in data:
        h = ((h ^ b) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return f"{((h >> 32) ^ h) & 0xFFFFFFFF:08x}"

def hashed(name, data):
    stem, _, ext = name.rpartition(".")
    return f"{stem}-{fnv(data)}.{ext}"

def parsed(name):
    with open(os.path.join(VENDOR, name), encoding="utf-8") as f:
        css = f.read()
    out = []
    for block in blocks(css):
        found = re.search(r"unicode-range:\s*([^;]+);", block)
        out.append((url_of(block), found.group(1).strip() if found else None))
    return out

def payload():
    items = [(os.path.join(MRLYFONT, "MrlyFont.woff2"), "fonts", "MrlyFont.woff2")]
    for role in FAMILIES:
        items.append((os.path.join(VENDOR, f"{role}.woff2"), "fonts", f"{role}.woff2"))
    for name in SUBSETS:
        items.append((os.path.join(VENDOR, name), "fonts", name))
    items.append((os.path.join(VENDOR, "seti", "seti.woff2"), "fonts", "seti.woff2"))
    for i, (name, _) in enumerate(parsed("emoji.css")):
        items.append((os.path.join(VENDOR, name), "fonts", f"emoji{i}.woff2"))
    for name in sorted(n for n in os.listdir(VENDOR) if n.startswith("LICENSE-")):
        items.append((os.path.join(VENDOR, name), "licenses", name))
    items.append((os.path.join(VENDOR, "seti", "LICENSE-seti.txt"), "licenses", "LICENSE-seti.txt"))
    return items

def cdn():
    if os.path.exists(CDN):
        shutil.rmtree(CDN)
    urls = {}
    for src, prefix, name in payload():
        with open(src, "rb") as f:
            data = f.read()
        target = hashed(name, data)
        path = os.path.join(CDN, prefix, target)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as f:
            f.write(data)
        urls[os.path.basename(src)] = f"{CDN_URL}/{prefix}/{target}"
        print(f"Wrote: data/cdn/{prefix}/{target}")
    return urls

# CSS

def face(family, url, display, ranges):
    lines = [
        "@font-face {",
        f'  font-family: "{family}";',
        "  font-style: normal;",
        "  font-weight: 400;",
        f'  src: url("{url}") format("woff2");',
        f"  font-display: {display};",
    ]
    if ranges:
        lines.append(f"  unicode-range: {ranges};")
    lines.append("}")
    return "\n".join(lines)

def shared(urls):
    ranges = dict(parsed("fonts.css"))
    out = {role: [face(f"mrly-{role}", urls[f"{role}.woff2"], "swap", ranges[f"{role}.woff2"])] for role in FAMILIES}
    out["mrlyfont"] = [face("MrlyFont", urls["MrlyFont.woff2"], "swap", None)]
    out["emoji"] = [face("noto", urls[name], "swap", rng) for name, rng in parsed("emoji.css")]
    out["site"] = [face("mrly-icons", urls["site.woff2"], "block", None)]
    out["icons"] = [face("mrly-icons", urls["icons.woff2"], "block", None)]
    out["ui"] = [face("mrly-icons", urls["ui.woff2"], "block", None)]
    out["seti"] = [face("seti", urls["seti.woff2"], "block", None)]
    return out

def sheet(keys, faces):
    out = []
    for key in keys:
        out += faces[key]
    return "\n\n".join(out) + "\n"

# EMIT

def copy(src, dst):
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(src, dst)
    print(f"Synced: {os.path.relpath(dst, ROOT)}")

def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    print(f"Wrote: {os.path.relpath(path, ROOT)}")

def emit():
    faces = shared(cdn())
    for site in MANIFEST.values():
        public = os.path.join(ROOT, site["public"])
        for rel in site["brand"]:
            copy(os.path.join(BRAND, rel), os.path.join(public, rel))
        for src, dst in site["copy"]:
            copy(os.path.join(FILES, src), os.path.join(ROOT, dst))
        for name, keys in site["css"].items():
            write(os.path.join(public, name), sheet(keys, faces))
    for rel, keys in PACKAGES.items():
        write(os.path.join(ROOT, rel), sheet(keys, faces))

# TERMINAL

def main(refetch=False):
    if refetch:
        fetch()
    font.create()
    render()
    emit()

def icons():
    subsets()
    emit()

def bundle():
    target = os.path.join(ROOT, "pkgs", "mrlyui", "fonts")
    os.makedirs(target, exist_ok=True)
    urls = {}
    for src, prefix, name in payload():
        if prefix != "fonts":
            continue
        shutil.copyfile(src, os.path.join(target, name))
        urls[os.path.basename(src)] = f"../fonts/{name}"
    write(os.path.join(ROOT, "pkgs", "mrlyui", "styles", "faces-local.css"), sheet(UI_FACES, shared(urls)))

def terminal():
    match sys.argv[1:]:
        case ["fetch"]: main(refetch=True)
        case ["icons"]: icons()
        case ["bundle"]: bundle()
        case _: main()

if __name__ == "__main__":
    terminal()
