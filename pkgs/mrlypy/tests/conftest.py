import json
import pathlib

import pytest

FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "mrlyrs" / "fixtures"


@pytest.fixture
def row(request):
    module = request.module.__name__.removeprefix("test_")
    rows = json.loads((FIXTURES / f"{module}.json").read_text())

    def pick(name):
        return next(r for r in rows if r["fn"] == name)

    return pick
