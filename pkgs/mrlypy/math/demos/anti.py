import mrlymath as mp
from helpers import save_bytes
from palette import BLACK, WHITE

SCALE = 8

NUMBER = 3

LEVEL = 3

def main():
    palette = {0: [WHITE], 1: [BLACK]}
    designs = {
        "carpet": mp.two.carpet(NUMBER, LEVEL),
        "net": mp.two.net(NUMBER, LEVEL),
        "htree": mp.two.htree(NUMBER, LEVEL),
        "void": mp.two.void(NUMBER, LEVEL),
    }
    for name, cell in designs.items():
        save_bytes(f"{name}.png", cell.paint(palette).png(SCALE))
    antis = {
        "point": mp.two.carpet(NUMBER).invert().fractal(LEVEL),
        "dust": mp.two.net(NUMBER).invert().fractal(LEVEL),
        "line": mp.two.htree(NUMBER).invert().fractal(LEVEL),
        "star": mp.two.void(NUMBER).invert().fractal(LEVEL),
    }
    for name, cell in antis.items():
        save_bytes(f"anti_{name}.png", cell.paint(palette).png(SCALE))

if __name__ == "__main__":
    main()
