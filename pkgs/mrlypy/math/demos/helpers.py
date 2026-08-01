import os

DATA_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "data")

os.makedirs(DATA_DIR, exist_ok=True)

GLYPHS = {0: "  ", 1: "##", 2: "::", 3: "/\\", 4: "<<", 5: ">>"}

EMOJI = {0: "⬜️", 1: "⬛️", 2: "🟦", 3: "🔺", 4: "◀️", 5: "▶️"}

def path(name):
    return os.path.join(DATA_DIR, name)

def save_bytes(name, data):
    fp = path(name)
    with open(fp, "wb") as f:
        f.write(data)
    print(f"Saved: {fp}")
    return fp

def save_text(name, data):
    fp = path(name)
    with open(fp, "w") as f:
        f.write(data)
    print(f"Saved: {fp}")
    return fp

def render_text(cell, glyphs=None):
    glyphs = glyphs or GLYPHS
    return "\n".join("".join(glyphs.get(v, "??") for v in row) for row in cell.to_lists())

def show_text(cell, glyphs=None):
    out = render_text(cell, glyphs)
    print(out)
    return out
