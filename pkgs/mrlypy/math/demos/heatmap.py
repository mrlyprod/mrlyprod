import mrlymath as mp
import numpy as np
from helpers import path
from PIL import Image

SIZE = (250, 250)
OUTPUT = (1000, 1000)
LEVEL = 1
NUMBERS = range(1, 50, 2)
FAMILIES = {
    "carpet": mp.two.carpet,
    "net": mp.two.net,
    "htree": mp.two.htree,
    "vtree": mp.two.vtree,
    "void": mp.two.void,
}

def resize(grid):
    if grid.shape == SIZE:
        return grid
    old_h, old_w = grid.shape
    new_y, new_x = np.indices(SIZE)
    return grid[new_y * old_h // SIZE[0], new_x * old_w // SIZE[1]]

def draw(heat, name):
    top = int(heat.max())
    lut = np.linspace(0, 255, top + 1, dtype=np.uint8)
    pixels = np.stack([lut[heat]] * 3, axis=2)
    image = Image.fromarray(pixels, "RGB").resize(OUTPUT, Image.Resampling.NEAREST)
    fp = path(f"heatmap_{name}.png")
    image.save(fp)
    print(f"Saved: {fp}")

def main():
    for name, func in FAMILIES.items():
        heat = np.zeros(SIZE, dtype=np.int16)
        for number in NUMBERS:
            heat += resize(np.array(func(number, LEVEL).to_lists(), dtype=np.int16))
        draw(heat, name)

if __name__ == "__main__":
    main()
