import os
import re
import urllib.request
from config import ROOT

UA = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0 Safari/537.36"
CSS2 = "https://fonts.googleapis.com/css2"
OFL = "https://raw.githubusercontent.com/google/fonts/main/ofl/{}/OFL.txt"
APACHE = "https://raw.githubusercontent.com/google/material-design-icons/master/LICENSE"
VENDOR = os.path.join(ROOT, "files", "vendor")
FAMILIES = {
    "mono": "JetBrains Mono",
    "sans": "Inter",
    "serif": "Lora",
    "display": "Silkscreen",
}

def fetch(url):
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req) as res:
        return res.read()

def save(name, data):
    with open(os.path.join(VENDOR, name), "wb") as f:
        f.write(data)
    print(f"vendor/{name} ({len(data)} bytes)")

def union():
    src = open(os.path.join(ROOT, "apps", "web", "src", "icons.ts")).read()
    body = src.split("{", 1)[1].split("}", 1)[0]
    return sorted(set(re.findall(r'"([a-z0-9_]+)"', body)))

def faces(css):
    return re.findall(r"@font-face\s*\{[^}]+\}", css)

def url_of(block):
    return re.search(r"url\((\S+?)\)", block).group(1)

def icons():
    axes = "Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0"
    css = fetch(f"{CSS2}?family={axes}&icon_names={','.join(union())}").decode()
    block = faces(css)[0]
    save("icons.woff2", fetch(url_of(block)))
    block = block.replace(url_of(block), "icons.woff2")
    block = re.sub(r"font-family: '[^']+';", "font-family: 'mrly-icons';", block)
    save("icons.css", (block + "\n").encode())

def emoji():
    css = fetch(f"{CSS2}?family=Noto+Color+Emoji").decode()
    out = []
    for i, block in enumerate(faces(css)):
        name = f"emoji.{i}.woff2"
        save(name, fetch(url_of(block)))
        block = block.replace(url_of(block), name)
        block = block.replace("font-family: 'Noto Color Emoji';", "font-family: 'noto';")
        out.append(block)
    save("emoji.css", ("\n".join(out) + "\n").encode())

def fonts():
    out = []
    for role, family in FAMILIES.items():
        css = fetch(f"{CSS2}?family={family.replace(' ', '+')}:wght@400").decode()
        latin = [b for c, b in re.findall(r"/\* ([a-z-]+) \*/\s*(@font-face\s*\{[^}]+\})", css) if c == "latin"]
        block = latin[-1] if latin else faces(css)[-1]
        name = f"{role}.woff2"
        save(name, fetch(url_of(block)))
        block = block.replace(url_of(block), name)
        block = re.sub(r"font-family: '[^']+';", f"font-family: 'mrly-{role}';", block)
        out.append(block)
    save("fonts.css", ("\n".join(out) + "\n").encode())

def licenses():
    save("LICENSE-icons.txt", fetch(APACHE))
    save("LICENSE-emoji.txt", fetch(OFL.format("notocoloremoji")))
    for role, family in FAMILIES.items():
        save(f"LICENSE-{role}.txt", fetch(OFL.format(family.lower().replace(" ", ""))))

def main():
    os.makedirs(VENDOR, exist_ok=True)
    icons()
    emoji()
    fonts()
    licenses()

if __name__ == "__main__":
    main()
