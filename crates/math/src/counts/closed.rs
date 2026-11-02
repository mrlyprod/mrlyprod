use serde::{Deserialize, Serialize};

/// A closed form of a design sequence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Closed {
    /// `fill^level`.
    Power(u128),
    /// `cells^level - fill^level`.
    Difference(u128, u128),
    /// A polynomial in `k` by rising power, at side `2k - 1`.
    Polynomial(Vec<i128>),
    /// `a(level) = c[0] a(level-1) + c[1] a(level-2) + ...`.
    Recurrence(Vec<i128>),
}

fn signed(text: &mut String, coefficient: i128, first: bool) {
    if first {
        if coefficient < 0 {
            text.push('-');
        }
    } else {
        text.push_str(if coefficient < 0 { " - " } else { " + " });
    }
}

impl Closed {
    /// Spells the closed form.
    ///
    /// ```
    /// use mrly_math::counts::Closed;
    /// assert_eq!(Closed::Polynomial(vec![0, -2, 3]).text(), "3k^2 - 2k");
    /// assert_eq!(Closed::Recurrence(vec![28, -160]).text(), "a(level) = 28 a(level-1) - 160 a(level-2)");
    /// ```
    pub fn text(&self) -> String {
        match self {
            Closed::Power(f) => format!("{f}^level"),
            Closed::Difference(g, f) => format!("{g}^level - {f}^level"),
            Closed::Polynomial(coefficients) => {
                let mut text = String::new();
                for (power, &c) in coefficients.iter().enumerate().rev() {
                    if c == 0 {
                        continue;
                    }
                    let first = text.is_empty();
                    signed(&mut text, c, first);
                    if c.abs() != 1 || power == 0 {
                        text.push_str(&c.abs().to_string());
                    }
                    match power {
                        0 => {}
                        1 => text.push('k'),
                        _ => text.push_str(&format!("k^{power}")),
                    }
                }
                if text.is_empty() {
                    text.push('0');
                }
                text
            }
            Closed::Recurrence(coefficients) => {
                let mut text = String::new();
                for (back, &c) in coefficients.iter().enumerate() {
                    if c == 0 {
                        continue;
                    }
                    let first = text.is_empty();
                    signed(&mut text, c, first);
                    if c.abs() != 1 {
                        text.push_str(&c.abs().to_string());
                        text.push(' ');
                    }
                    text.push_str(&format!("a(level-{})", back + 1));
                }
                if text.is_empty() {
                    text.push('0');
                }
                format!("a(level) = {text}")
            }
        }
    }
}

fn binomial(n: usize, k: usize) -> i128 {
    (0..k).fold(1i128, |acc, i| acc * (n - i) as i128 / (i as i128 + 1))
}

/// Expands the odd-side fill at side `2k - 1`, `sum over the corners of k^(zeros) (k - 1)^(ones)`, into coefficients by rising power of `k`.
///
/// ```
/// let carpet = mrly_math::bang::code_to_corners(mrly_math::bang::Code::from(7u64), 2, 2).unwrap();
/// assert_eq!(mrly_math::counts::fill_polynomial(&carpet, 2), [0, -2, 3]);
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

/// Expands the whole grid at side `2k - 1`, `(2k - 1)^dimension`, into coefficients by rising power of `k`.
///
/// ```
/// assert_eq!(mrly_math::counts::odd_power(2), [1, -4, 4]);
/// ```
pub fn odd_power(dimension: usize) -> Vec<i128> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bang::{Code, code_to_corners};

    #[test]
    fn the_closed_forms_spell_themselves() {
        assert_eq!(Closed::Power(8).text(), "8^level");
        assert_eq!(Closed::Difference(9, 8).text(), "9^level - 8^level");
        assert_eq!(Closed::Polynomial(vec![1, -2, 2]).text(), "2k^2 - 2k + 1");
        assert_eq!(Closed::Polynomial(vec![0, 0, 1]).text(), "k^2");
        assert_eq!(Closed::Polynomial(vec![-1, 0, -1]).text(), "-k^2 - 1");
        assert_eq!(Closed::Polynomial(vec![0]).text(), "0");
        assert_eq!(
            Closed::Recurrence(vec![11, -24]).text(),
            "a(level) = 11 a(level-1) - 24 a(level-2)"
        );
        assert_eq!(Closed::Recurrence(vec![1]).text(), "a(level) = a(level-1)");
        assert_eq!(Closed::Recurrence(vec![0]).text(), "a(level) = 0");
    }

    #[test]
    fn the_odd_side_polynomials_match_the_fill_counts() {
        for code in 0..16u64 {
            let corners = code_to_corners(Code::from(code), 2, 2).unwrap();
            let fills = fill_polynomial(&corners, 2);
            let grid = odd_power(2);
            for k in 2..7i128 {
                let side = (2 * k - 1) as usize;
                let at = |poly: &[i128]| -> i128 {
                    poly.iter()
                        .enumerate()
                        .map(|(p, &c)| c * k.pow(p as u32))
                        .sum()
                };
                let fill = crate::counts::fill(Code::from(code), side, 2, 1, 2).unwrap() as i128;
                assert_eq!(at(&fills), fill, "code={code} k={k}");
                assert_eq!(at(&grid), (side * side) as i128, "k={k}");
            }
        }
    }
}
