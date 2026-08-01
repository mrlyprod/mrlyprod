import mrlymath as mp
from helpers import EMOJI, render_text

def main():
    mp.seed(42)
    for number in [3, 5]:
        for level in [1, 2]:
            for density in [0.25, 0.5, 0.75]:
                print(f"number={number} level={level} density={density}")
                print(render_text(mp.two.noise(number, level, density), EMOJI))
                print()

if __name__ == "__main__":
    main()
