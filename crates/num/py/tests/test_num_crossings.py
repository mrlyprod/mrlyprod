import pytest
from mrly.num import factor, series


def test_a_rust_overflow_lands_as_an_overflow_error():
    assert series.bernoulli(3) == [(1, 1), (-1, 2), (1, 6)]
    with pytest.raises(OverflowError):
        series.bernoulli(33)


def test_a_code_crosses_as_a_plain_int():
    assert factor.gcd(1071, 462) == 21
    assert factor.gcd(2**127, 2**100) == 2**100
