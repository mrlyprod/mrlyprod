import mrlymath as mp
import numpy as np
from helpers import path
from palette import RED, GREEN, BLUE, gradient
from PIL import Image

SIZE = (200, 200)
OUTPUT = (1000, 1000)
MAX_ITER = 100

def main():
    grid = np.array(mp.fractal.mandelbrot(*SIZE, MAX_ITER), dtype=np.int16)
    lut = np.array(gradient([RED, GREEN, BLUE], MAX_ITER), dtype=np.uint8)
    pixels = lut[np.clip(grid, 0, MAX_ITER - 1)]
    image = Image.fromarray(pixels, "RGB").resize(OUTPUT, Image.Resampling.NEAREST)
    fp = path("mandelbrot.png")
    image.save(fp)
    print(f"Saved: {fp}")

if __name__ == "__main__":
    main()
