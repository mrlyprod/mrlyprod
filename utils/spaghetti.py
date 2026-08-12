import os
import shutil
from config import ROOT

DIRS = {"dist", "target", ".venv", "node_modules", "__pycache__"}
FILES = {".DS_Store"}

def spaghetti():
    for dirpath, dirnames, filenames in os.walk(ROOT):
        for name in list(dirnames):
            if name == ".git":
                dirnames.remove(name)
            elif name in DIRS:
                path = os.path.join(dirpath, name)
                shutil.rmtree(path)
                print(os.path.relpath(path, ROOT))
                dirnames.remove(name)
        for name in filenames:
            if name in FILES:
                path = os.path.join(dirpath, name)
                os.remove(path)
                print(os.path.relpath(path, ROOT))

if __name__ == "__main__":
    spaghetti()
