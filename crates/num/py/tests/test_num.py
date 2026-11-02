import json
import pathlib

from mrly.num import factor, prime, series

FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "tests" / "fixtures.json"
ROWS = json.loads(FIXTURES.read_text())


def row(fn):
    return next(r for r in ROWS if r["fn"] == fn)


def test_factorial():
    r = row("num::factor::factorial")
    assert factor.factorial(r["in"]["number"]) == int(r["out"])


def test_gcd():
    r = row("num::factor::gcd")
    assert [str(factor.gcd(int(a), int(b))) for a, b in r["in"]["pairs"]] == r["out"]


def test_divisors():
    r = row("num::factor::divisors")
    assert factor.divisors(r["in"]["number"]) == r["out"]


def test_mobius_sieve():
    r = row("num::factor::mobius_sieve")
    assert factor.mobius_sieve(r["in"]["limit"]) == r["out"]


def test_is_prime():
    r = row("num::prime::is_prime")
    kept = [n for n in range(r["in"]["from"], r["in"]["to"] + 1) if prime.is_prime(n)]
    assert kept == r["out"]


def test_zeta():
    r = row("num::series::zeta")
    assert round(series.zeta(r["in"]["s"], r["in"]["terms"]), 12) == r["out"]
