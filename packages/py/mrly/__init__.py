import ctypes
import importlib.abc
import importlib.machinery
import json
import os
import platform
import sys
import sysconfig
import types

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
DATA_DIR = os.path.join(REPO, "data", "packages", "rs", "bridge")
SUFFIX = ".dylib" if sys.platform == "darwin" else ".so"
UNITS = {"core", "font", "life", "math", "num", "tile"}
KINDS = [
    np.dtype(kind)
    for kind in (
        np.uint8,
        np.uint16,
        np.uint32,
        np.int8,
        np.int16,
        np.int32,
        np.float32,
        np.float64,
        np.uint64,
        np.int64,
    )
]
TENSOR, CELL, IMAGE = ord("t"), ord("c"), ord("i")

if platform.python_implementation() != "CPython" or sysconfig.get_config_var(
    "Py_GIL_DISABLED"
):
    raise ImportError("mrly runs on CPython with the GIL; this Python is not supported")


class MrlyError(Exception):
    pass


class Thing(int):
    def __del__(self):
        if not self.__dict__.get("freed"):
            self.drop(int(self))

    def free(self):
        if not self.__dict__.get("freed"):
            self.freed = True
            self.drop(int(self))

    def __repr__(self):
        return f"<{type(self).__name__}>"


def shaped(array, shape):
    return array.reshape(shape) if array is not None else None


def form(kind, value):
    if kind == TENSOR:
        return shaped(value["data"], value["shape"])
    if kind == IMAGE:
        height, width = value["shape"]
        return shaped(value["colors"], (height, width, 4))
    shape = value["shape"]
    colors = value.get("colors")
    return {
        "types": shaped(value["types"], shape),
        "colors": shaped(colors, (*shape, 4)) if colors is not None else None,
        "tags": shaped(value.get("tags"), shape),
    }


def back(result, *args):
    backs, value = result
    for at, one in backs:
        target = args[at]
        if isinstance(target, dict):
            target.update(one)
        else:
            target[...] = one
    return value


def load(name):
    lib = ctypes.PyDLL(os.path.join(DATA_DIR, "py", f"libmrly_{name}{SUFFIX}"))
    with open(os.path.join(DATA_DIR, "api", f"{name}.json")) as f:
        api = json.load(f)
    lib.mrly_release.argtypes = [ctypes.c_uint32]
    lib.mrly_drop.argtypes = [ctypes.c_uint32]
    release = lib.mrly_release
    drop = lib.mrly_drop
    classes = {}

    class Base(Thing):
        pass

    class Keep:
        __slots__ = ("id",)

        def __init__(self, id):
            self.id = id

        def __del__(self):
            release(self.id)

    def array(kind, at, n, keep):
        dtype = KINDS[kind]
        buffer = (ctypes.c_char * (n * dtype.itemsize)).from_address(at)
        buffer.keep = keep
        return np.frombuffer(buffer, dtype=dtype)

    def wrap(id, of):
        return int.__new__(classes[of], id)

    lib.mrly_py_init.argtypes = [ctypes.py_object] * 7
    lib.mrly_py_init.restype = None
    lib.mrly_py_init(MrlyError, array, wrap, Keep, form, back, Base)
    lib.mrly_py_door.restype = ctypes.py_object
    lib.mrly_py_door.argtypes = [ctypes.c_size_t, ctypes.c_size_t]
    lib.mrly_py_direct.restype = ctypes.py_object
    lib.mrly_py_direct.argtypes = [ctypes.c_size_t, ctypes.c_size_t, ctypes.c_size_t]
    lib.mrly_py_method.restype = ctypes.py_object
    lib.mrly_py_method.argtypes = [ctypes.py_object]

    direct = api["direct"]
    names = api["doors"]
    index = {one: at for at, one in enumerate(names)}
    consts = set(api["consts"])

    def fast(symbol, n, ret):
        at = ctypes.cast(getattr(lib, symbol), ctypes.c_void_p).value
        return lib.mrly_py_direct(at, n, ret)

    def door(one):
        info = direct.get(one)
        if info:
            ret = 2 if info["bool"] else 1 if info["int"] else 0
            return fast(f"mrly_d{info['at']}", info["n"], ret)
        return lib.mrly_py_door(index[one], 0)

    def make(one, info):
        members = {"drop": staticmethod(drop)}
        for field, settable in info["fields"]:
            get = door(f"{one}.{field}$get")
            put = door(f"{one}.{field}$set") if settable else None
            members[field] = property(
                lambda this, get=get: get(this),
                (lambda this, value, put=put: put(this, value)) if put else None,
            )
        for method in info["methods"]:
            members[method] = lib.mrly_py_method(door(f"{one}.{method}"))
        for fixed in info["statics"]:
            if fixed != "new":
                members[fixed] = staticmethod(door(f"{one}.{fixed}"))
        if info["from"]:
            members["from_"] = staticmethod(door(f"{one}.from"))
        if info["json"]:
            members["to_json"] = lib.mrly_py_method(door(f"{one}.toJSON"))
        made = type(one.rsplit(".", 1)[-1], (Base,), members)
        if f"{one}.new" in index:
            maker = door(f"{one}.new")
            made.__new__ = lambda cls, *args, maker=maker: maker(*args)
            made.__init__ = lambda this, *args: None
        return made

    for one, info in api["classes"].items():
        classes[one] = make(one, info)
    classes["Rng"] = make(
        "Rng",
        {
            "fields": [],
            "methods": [
                "range",
                "boolean",
                "chance",
                "sample_indices",
                "order",
                "__state",
                "__restore",
            ],
            "statics": [],
            "from": False,
            "json": False,
        },
    )
    rng = classes["Rng"]
    rng.unit = lib.mrly_py_method(fast("mrly_rng_unit", 1, 0))
    rng.below = lib.mrly_py_method(fast("mrly_rng_below", 2, 1))
    rng.choice = lambda this, items: items[this.below(len(items))]

    def shuffle(this, items):
        moved = [items[at] for at in this.order(len(items))]
        items[:] = moved

    rng.shuffle = shuffle

    root = types.ModuleType(f"mrly.{name}")
    root.__path__ = []
    root.Rng = rng

    def module(parts):
        at = root
        full = f"mrly.{name}"
        for part in parts:
            full += f".{part}"
            child = getattr(at, part, None)
            if child is None:
                child = types.ModuleType(full)
                child.__path__ = []
                setattr(at, part, child)
                sys.modules[full] = child
            at = child
        return at

    def place(parts, value):
        at = module(parts[:-1])
        setattr(at, parts[-1], value)

    prefix = f"{name}."
    short = lambda one: one.removeprefix(prefix)
    for one, made in classes.items():
        if one != "Rng":
            place(short(one).split("."), made)
    for one in names:
        if "$" in one or one.startswith("Rng."):
            continue
        owner = one.rsplit(".", 1)[0]
        if owner in classes:
            continue
        parts = short(one).split(".")
        if one in consts:
            place(parts, door(one)())
        else:
            place(parts, door(one))
    return root


class Finder(importlib.abc.MetaPathFinder, importlib.abc.Loader):
    def find_spec(self, fullname, path=None, target=None):
        parts = fullname.split(".")
        if parts[0] == "mrly" and len(parts) == 2 and parts[1] in UNITS:
            return importlib.machinery.ModuleSpec(fullname, self)
        return None

    def create_module(self, spec):
        return load(spec.name.split(".")[1])

    def exec_module(self, module):
        pass


sys.meta_path.insert(0, Finder())
