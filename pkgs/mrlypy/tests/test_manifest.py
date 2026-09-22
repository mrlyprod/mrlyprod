import importlib
import json
import keyword
import pathlib

import mrlypy

MANIFEST = pathlib.Path(__file__).resolve().parents[3] / "bridge" / "manifest.json"


def load():
    return json.loads(MANIFEST.read_text())


def name_of(word):
    return word + "_" if keyword.iskeyword(word) else word


def locate(fn, kinds):
    module = importlib.import_module("mrlypy." + fn["module"].replace("::", "."))
    exported = name_of(fn["path"].rsplit("::", 1)[-1])
    owner = fn["owner"]
    if owner and kinds[owner] in ("class", "plain", "enum"):
        return getattr(getattr(module, owner.rsplit("::", 1)[-1]), exported)
    return getattr(module, exported)


def test_every_ok_function_is_callable_under_its_rust_doc():
    manifest = load()
    kinds = {t["path"]: t["cross"]["kind"] for t in manifest["types"]}
    count = 0
    for fn in manifest["functions"]:
        if fn["cross"]["status"] != "ok":
            continue
        target = locate(fn, kinds)
        assert callable(target), fn["path"]
        doc = target.__doc__ or ""
        if fn["docs"]:
            if fn["dims"]:
                assert fn["docs"][0] in doc, fn["path"]
            else:
                assert doc.startswith(fn["docs"][0]), fn["path"]
        count += 1
    assert count == 1109
