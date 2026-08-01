import importlib
import sys

DEMOS = [
    "fromarray",
    "mandelbrot",
    "julia",
    "heatmap",
    "anti",
    "noise",
    "objects",
]

def main():
    failed = []
    for name in DEMOS:
        print(f"\n{'=' * 60}\n# {name}\n{'=' * 60}")
        try:
            mod = importlib.import_module(name)
            mod.main()
        except Exception as e:
            print(f"!! {name} FAILED: {type(e).__name__}: {e}")
            failed.append(name)
    print(f"\n{'=' * 60}")
    if failed:
        print(f"FAILED: {', '.join(failed)}")
        sys.exit(1)
    print(f"ALL {len(DEMOS)} DEMOS RAN OK")

if __name__ == "__main__":
    main()
