import mrlymath as mp
from helpers import save_bytes

GRID = [
    [0, 1, 1],
    [1, 0, 1],
    [1, 1, 0],
]

def main():
    for level in [2, 3]:
        cell = mp.two.from_lists(GRID).fractal(level)
        save_bytes(f"fromarray_{level}.png", cell.png(10))

if __name__ == "__main__":
    main()
