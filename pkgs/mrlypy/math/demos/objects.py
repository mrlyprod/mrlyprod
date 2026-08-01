import mrlymath as mp
from helpers import save_text

def main():
    objects = {
        "sponge": mp.three.carpet(3, 2),
        "net": mp.three.net(3, 2),
        "xtree": mp.three.xtree(3, 2),
        "ztree": mp.three.ztree(3, 2),
    }
    for name, cell in objects.items():
        print(f"{name}: {cell.width}x{cell.height}x{cell.depth}, "
              f"volume={cell.volume()}, surface={cell.surface()}")
        save_text(f"{name}.obj", cell.obj())

if __name__ == "__main__":
    main()
