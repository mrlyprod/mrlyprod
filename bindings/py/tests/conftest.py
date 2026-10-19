import hashlib
import json
import pathlib

import pytest

FIXTURES = pathlib.Path(__file__).resolve().parents[3] / "crates" / "mrly" / "fixtures"


@pytest.fixture
def row(request):
    module = pathlib.Path(request.module.__file__).stem.removeprefix("test_")
    rows = json.loads((FIXTURES / f"{module}.json").read_text())

    def pick(name):
        return next(r for r in rows if r["fn"] == name)

    return pick


@pytest.fixture
def pixels():
    def pin(image):
        return {"shape": list(image.shape[:2]), "sha256": hashlib.sha256(image.tobytes()).hexdigest()}

    return pin
