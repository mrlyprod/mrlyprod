import os
import shutil
from config import ROOT

def _wipe_dir(name, is_file=False):
    print(f"Wiping {name}...")
    for dirpath, dirnames, filenames in os.walk(ROOT):
        if is_file:
            if name in filenames:
                path = os.path.join(dirpath, name)
                os.remove(path)
                print(f"  {os.path.relpath(path, ROOT)}")
        else:
            if name in dirnames:
                path = os.path.join(dirpath, name)
                shutil.rmtree(path)
                print(f"  {os.path.relpath(path, ROOT)}")
                dirnames.remove(name)
    print()

def spaghetti():
    _wipe_dir("dist")
    _wipe_dir("target")
    _wipe_dir(".venv")
    _wipe_dir("node_modules")
    _wipe_dir("__pycache__")
    _wipe_dir(".DS_Store", is_file=True)

if __name__ == "__main__":
    spaghetti()
