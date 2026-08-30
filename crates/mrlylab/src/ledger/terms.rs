use super::{Axis, Closed, Key, Measure};
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use mrlymath::bang::{code_to_corners, factory};
use mrlymath::formulas;
use mrlymath::{six, three, two};

fn binomial(n: usize, k: usize) -> i128 {
    (0..k).fold(1i128, |acc, i| acc * (n - i) as i128 / (i as i128 + 1))
}

fn grid(number: usize, dimension: usize, level: u32) -> Option<u128> {
    (number as u128).checked_pow((dimension as u32).checked_mul(level)?)
}

fn tile(key: &Key, number: usize) -> Result<Tensor> {
    factory::create(key.code, number, key.dimension, key.base, 1)
}

fn complex(key: &Key, number: usize, level: u32) -> Result<i128> {
    let Key {
        code,
        dimension,
        base,
        measure,
        ..
    } = *key;
    let value = match dimension {
        2 => {
            let tally = two::census(&two::create(code, number, level as usize, 0, base)?)?;
            match measure {
                Measure::Vertices => tally.vertices as i128,
                Measure::Edges => tally.edges as i128,
                Measure::Euler => tally.euler as i128,
                _ => return value_error("the plane carries no faces."),
            }
        }
        _ => {
            let tally = three::census(&three::create(code, number, level as usize, base)?)?;
            match measure {
                Measure::Vertices => tally.vertices as i128,
                Measure::Edges => tally.edges as i128,
                Measure::Faces => tally.faces as i128,
                _ => tally.euler as i128,
            }
        }
    };
    Ok(value)
}

fn term(key: &Key, number: usize, level: u32, cells: u128) -> Result<Option<i128>> {
    let Key {
        code,
        dimension,
        base,
        measure,
        ..
    } = *key;
    let fill = || -> Result<Option<u128>> {
        Ok(formulas::fill(code, number, dimension, 1, base)?.checked_pow(level))
    };
    let within = |cost: Option<u128>| cost.is_some_and(|cost| cost <= cells);
    let value = match measure {
        Measure::Fills => fill()?.and_then(|fill| i128::try_from(fill).ok()),
        Measure::Voids => grid(number, dimension, level)
            .zip(fill()?)
            .map(|(grid, fill)| (grid - fill) as i128),
        Measure::Surface => {
            let corners = code_to_corners(code, dimension, base)?;
            formulas::Exposure::from_corners(&corners, number, dimension, base)
                .at(level)
                .map(|faces| faces as i128)
        }
        Measure::Peak | Measure::Heights => {
            let span = (number as u128)
                .checked_pow(level)
                .map(|side| dimension as u128 * (side - 1) + 1);
            if !within(span) {
                return Ok(None);
            }
            let counts = formulas::profile_of_tile(&tile(key, number)?, level)?;
            Some(if measure == Measure::Peak {
                counts.iter().max().copied().unwrap_or(0) as i128
            } else {
                counts.iter().filter(|&&count| count > 0).count() as i128
            })
        }
        Measure::Vertices | Measure::Edges | Measure::Faces | Measure::Euler => {
            if !within(grid(number, dimension, level)) {
                return Ok(None);
            }
            Some(complex(key, number, level)?)
        }
        Measure::Triangles => {
            let cost = (number as u128)
                .checked_pow(2 * level)
                .and_then(|side| side.checked_mul(16));
            if !within(cost) {
                return Ok(None);
            }
            Some(formulas::cut_fills(code, number, level)? as i128)
        }
        Measure::Holes | Measure::Pieces => {
            if !within(grid(number, dimension, level)) {
                return Ok(None);
            }
            let slice = six::cut(&three::create(code, number, level as usize, base)?)?;
            Some(if measure == Measure::Holes {
                six::holes(&slice)? as i128
            } else {
                six::components(&slice)? as i128
            })
        }
    };
    Ok(value)
}

/// Reads the first terms of a design sequence within a cell budget, and whether the budget or a u128 cut them short.
///
/// ```
/// use mrlylab::ledger::{terms, Axis, Key, Measure, BUDGET};
/// let carpet = Key::new(7, 2, 2, Measure::Fills, Axis::Level);
/// assert_eq!(terms(&carpet, 3, BUDGET).unwrap(), (vec![8, 64, 512], false));
/// let sponge = Key::new(23, 3, 2, Measure::Surface, Axis::Level);
/// assert_eq!(terms(&sponge, 3, BUDGET).unwrap(), (vec![72, 1056, 18048], false));
/// ```
pub fn terms(key: &Key, count: usize, cells: u128) -> Result<(Vec<i128>, bool)> {
    code_to_corners(key.code, key.dimension, key.base)?;
    if !key.measure.applies(key.dimension, key.base) {
        return value_error(format!(
            "{} does not read dimension {} base {}.",
            key.measure.slug(),
            key.dimension,
            key.base
        ));
    }
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let (number, level) = key.axis.place(index, key.number());
        match term(key, number, level, cells)? {
            Some(value) => out.push(value),
            None => return Ok((out, true)),
        }
    }
    Ok((out, false))
}

/// Expands the odd-side fill `sum over the corners of k^(zeros) (k - 1)^(ones)` into coefficients by rising power of `k`.
///
/// ```
/// let carpet = mrlymath::bang::code_to_corners(7, 2, 2).unwrap();
/// assert_eq!(mrlylab::ledger::fill_polynomial(&carpet, 2), [0, -2, 3]);
/// ```
pub fn fill_polynomial(corners: &[Vec<u8>], dimension: usize) -> Vec<i128> {
    let mut out = vec![0i128; dimension + 1];
    for corner in corners {
        let ones = corner.iter().filter(|&&r| r != 0).count();
        for j in 0..=ones {
            let sign = if (ones - j).is_multiple_of(2) { 1 } else { -1 };
            out[dimension - ones + j] += sign * binomial(ones, j);
        }
    }
    out
}

fn odd_power(dimension: usize) -> Vec<i128> {
    (0..=dimension)
        .map(|j| {
            let sign = if (dimension - j).is_multiple_of(2) {
                1
            } else {
                -1
            };
            sign * binomial(dimension, j) * (1i128 << j)
        })
        .collect()
}

/// Returns the closed form of a design sequence when the ledger knows one.
///
/// Level fills are a power and level voids a difference of powers; base-2 side fills and voids
/// are polynomials in `k`; the level surface obeys the exposure recurrence.
pub fn closed(key: &Key) -> Result<Option<Closed>> {
    let Key {
        code,
        dimension,
        base,
        measure,
        axis,
    } = *key;
    let corners = code_to_corners(code, dimension, base)?;
    let fill = || formulas::fill(code, key.number(), dimension, 1, base);
    let form = match (measure, axis) {
        (Measure::Fills, Axis::Level) => Closed::Power(fill()?),
        (Measure::Voids, Axis::Level) => {
            Closed::Difference((key.number() as u128).pow(dimension as u32), fill()?)
        }
        (Measure::Fills, Axis::Side) if base == 2 => {
            Closed::Polynomial(fill_polynomial(&corners, dimension))
        }
        (Measure::Voids, Axis::Side) if base == 2 => Closed::Polynomial(
            odd_power(dimension)
                .iter()
                .zip(fill_polynomial(&corners, dimension))
                .map(|(all, filled)| all - filled)
                .collect(),
        ),
        (Measure::Surface, Axis::Level) => Closed::Recurrence(
            formulas::Exposure::from_corners(&corners, key.number(), dimension, base).recurrence(),
        ),
        _ => return Ok(None),
    };
    Ok(Some(form))
}

#[cfg(test)]
mod tests {
    use super::super::BUDGET;
    use super::*;

    fn read(code: u128, dimension: usize, measure: Measure, axis: Axis, count: usize) -> Vec<i128> {
        terms(&Key::new(code, dimension, 2, measure, axis), count, BUDGET)
            .unwrap()
            .0
    }

    #[test]
    fn the_closed_measures_read_the_classics() {
        assert_eq!(read(7, 2, Measure::Voids, Axis::Level, 3), [1, 17, 217]);
        assert_eq!(
            read(7, 2, Measure::Surface, Axis::Level, 4),
            [16, 80, 496, 3536]
        );
        assert_eq!(
            read(23, 3, Measure::Fills, Axis::Side, 4),
            [20, 81, 208, 425]
        );
        assert_eq!(
            read(23, 3, Measure::Voids, Axis::Side, 4),
            [7, 44, 135, 304]
        );
        assert_eq!(read(1, 2, Measure::Fills, Axis::Side, 3), [4, 9, 16]);
        assert_eq!(read(9, 2, Measure::Fills, Axis::Side, 3), [5, 13, 25]);
        assert_eq!(read(11, 2, Measure::Fills, Axis::Side, 3), [7, 19, 37]);
        assert_eq!(read(15, 2, Measure::Fills, Axis::Side, 3), [9, 25, 49]);
        assert_eq!(read(129, 3, Measure::Fills, Axis::Side, 3), [9, 35, 91]);
        assert_eq!(read(255, 3, Measure::Fills, Axis::Side, 3), [27, 125, 343]);
    }

    #[test]
    fn the_grid_measures_agree_with_the_censuses() {
        assert_eq!(read(7, 2, Measure::Euler, Axis::Level, 3), [0, -8, -72]);
        assert_eq!(read(23, 3, Measure::Euler, Axis::Level, 2), [-4, -80]);
        assert_eq!(read(23, 3, Measure::Faces, Axis::Level, 1), [96]);
        assert_eq!(read(23, 3, Measure::Vertices, Axis::Level, 1), [64]);
        assert_eq!(read(23, 3, Measure::Edges, Axis::Level, 1), [144]);
        assert_eq!(read(7, 2, Measure::Vertices, Axis::Side, 2), [16, 36]);
        assert_eq!(read(23, 3, Measure::Triangles, Axis::Level, 2), [42, 306]);
        assert_eq!(read(23, 3, Measure::Pieces, Axis::Side, 2), [1, 7]);
        assert_eq!(read(255, 3, Measure::Holes, Axis::Level, 2), [0, 0]);
        assert_eq!(read(23, 3, Measure::Peak, Axis::Level, 2), [6, 42]);
        assert_eq!(read(23, 3, Measure::Heights, Axis::Level, 2), [7, 25]);
    }

    #[test]
    fn the_budget_caps_honestly() {
        let key = Key::new(23, 3, 2, Measure::Euler, Axis::Level);
        assert_eq!(terms(&key, 8, 1000).unwrap(), (vec![-4, -80], true));
        assert_eq!(terms(&key, 8, 26).unwrap(), (vec![], true));
        let deep = Key::new(23, 3, 2, Measure::Fills, Axis::Level);
        let (fills, capped) = terms(&deep, 40, BUDGET).unwrap();
        assert!(capped && fills.len() == 29);
        let base3 = Key::new(100, 2, 3, Measure::Surface, Axis::Side);
        assert_eq!(terms(&base3, 2, BUDGET).unwrap().0.len(), 2);
        assert!(terms(&Key::new(7, 2, 2, Measure::Faces, Axis::Level), 1, BUDGET).is_err());
        assert!(terms(&Key::new(16, 2, 2, Measure::Fills, Axis::Level), 1, BUDGET).is_err());
    }

    #[test]
    fn the_closed_forms_match_the_terms() {
        for code in 0..16u128 {
            for measure in [Measure::Fills, Measure::Voids] {
                let key = Key::new(code, 2, 2, measure, Axis::Side);
                let Some(Closed::Polynomial(poly)) = closed(&key).unwrap() else {
                    panic!("no polynomial");
                };
                let (terms, _) = terms(&key, 5, BUDGET).unwrap();
                for (index, &term) in terms.iter().enumerate() {
                    let k = index as i128 + 2;
                    let value: i128 = poly
                        .iter()
                        .enumerate()
                        .map(|(p, &c)| c * k.pow(p as u32))
                        .sum();
                    assert_eq!(value, term, "code={code} k={k}");
                }
            }
        }
        assert_eq!(
            closed(&Key::new(23, 3, 2, Measure::Fills, Axis::Level)).unwrap(),
            Some(Closed::Power(20))
        );
        assert_eq!(
            closed(&Key::new(7, 2, 2, Measure::Voids, Axis::Level)).unwrap(),
            Some(Closed::Difference(9, 8))
        );
        assert_eq!(
            closed(&Key::new(7, 2, 3, Measure::Fills, Axis::Side)).unwrap(),
            None
        );
        assert_eq!(
            closed(&Key::new(7, 2, 2, Measure::Euler, Axis::Level)).unwrap(),
            None
        );
    }
}
