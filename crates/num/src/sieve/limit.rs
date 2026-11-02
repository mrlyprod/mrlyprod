use super::{PLANE_LIMIT, letter, odd_word, ratio};
use mrly_core::error::{Result, value_error};

const SPLIT: u64 = 1001;

fn odd_tail(start: u64, power: i32) -> f64 {
    let n = start as f64;
    let s = f64::from(power);
    let p = n.powi(-power);
    p * n / (2.0 * (s - 1.0)) + p / 2.0 + s * p / (6.0 * n)
        - s * (s + 1.0) * (s + 2.0) * p / (90.0 * n * n * n)
}

/// Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.
///
/// Write m = 2k + 1 and factor m^3 - 1 = (m - 1)(m - w)(m - w^2) at w = exp(2 pi i / 3), the three cube roots of one; dividing by m^3 = 8 (k + 1/2)^3 turns the k-th factor into k (k + (1 - w)/2) (k + (1 - w^2)/2) / (k + 1/2)^3, so the product is a ratio of three rising shifts over one repeated three times.
///
/// The three shifts a1 = 0, a2 = (1 - w)/2 and a3 = (1 - w^2)/2 sum to (2 - w - w^2)/2 = 3/2, exactly three times the shift b = 1/2 below, and prod_{k=1..N} (k + a) = Gamma(N + 1 + a) / Gamma(1 + a) with Gamma(N + 1 + a) / Gamma(N + 1 + b) ~ N^(a - b), so the N-dependent factor tends to N^(a1 + a2 + a3 - 3b) = 1 and the limit is Gamma(1 + b)^3 / (Gamma(1 + a1) Gamma(1 + a2) Gamma(1 + a3)).
///
/// That is Gamma(3/2)^3 / (Gamma(1) Gamma((3 - w)/2) Gamma((3 - w^2)/2)) = pi^(3/2) / (8 |Gamma(7/4 - i sqrt(3)/4)|^2), the two gamma values being conjugates; the same Weierstrass product gives the identity prod_{k >= 1} (1 - a^3/k^3) = 1 / (Gamma(1 - a) Gamma(1 - a w) Gamma(1 - a w^2)) the ratio form of this limit leans on, at a = 1/2 against prod_{n >= 2} (1 - n^-3) = cosh(pi sqrt(3)/2) / (3 pi).
///
/// The value is read from the logarithm rather than from that gamma ratio, which cancels four digits away: log P = sum over odd n from 3 to 999 of log(1 - n^-3), taken from its smallest term, minus sum_{j = 1..3} (1/j) sum_{n odd >= 1001} n^(-3j), each inner sum by Euler-Maclaurin through the fourth Bernoulli term. Every piece carries one sign, so nothing cancels and the exponential lands within a few ulps.
///
/// ```
/// assert!((mrly_num::sieve::solid_limit() - 0.948_815_485_719_679_6).abs() < 4e-16);
/// ```
pub fn solid_limit() -> f64 {
    let mut log = 0.0f64;
    let mut n = SPLIT - 2;
    while n >= 3 {
        log += (-(n as f64).powi(-3)).ln_1p();
        n -= 2;
    }
    for j in 1..=3i32 {
        log -= odd_tail(SPLIT, 3 * j) / f64::from(j);
    }
    log.exp()
}

/// Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.
///
/// A run of consecutive odd letters from a, of two letters or more, names the schedule a, a + 2, a + 4 and on: its letters are strictly increasing, sum s_k^(-d) converges, and the limit is positive, pi over four in the plane and the closed form above in the cube divided by the head the run skips. Two or more copies of one letter name the fixed-ratio schedule, whose limit is zero; that is what repetition costs, and an alternating word such as 3, 5, 3, 5 changes at every step and still loses the whole measure. Every other word, and every word of one letter, names no schedule and reads None.
///
/// ```
/// assert_eq!(mrly_num::sieve::limit(&mrly_num::sieve::odd_word(3), 2).unwrap(), Some(std::f64::consts::FRAC_PI_4));
/// assert_eq!(mrly_num::sieve::limit(&mrly_num::sieve::flat_word(3, 3), 2).unwrap(), Some(0.0));
/// assert!((mrly_num::sieve::limit(&[5, 7, 9], 2).unwrap().unwrap() - 0.883_572_933_822_129_3).abs() < 1e-15);
/// assert_eq!(mrly_num::sieve::limit(&[3, 7, 11], 2).unwrap(), None);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn limit(word: &[u64], dimension: u32) -> Result<Option<f64>> {
    if word.len() < 2 {
        return Ok(None);
    }
    let first = letter(word[0])?;
    let mut flat = true;
    for &s in word {
        flat &= letter(s)? == first;
    }
    if flat {
        return Ok(Some(0.0));
    }
    if !word
        .iter()
        .enumerate()
        .all(|(k, &s)| s == first + 2 * k as u64)
    {
        return Ok(None);
    }
    let whole = match dimension {
        2 => PLANE_LIMIT,
        3 => solid_limit(),
        _ => return Ok(None),
    };
    let head = odd_word(((first - 3) / 2) as usize);
    Ok(Some(whole / ratio(&head, dimension)?))
}

/// Returns the Wallis sieve product of a parity design, `prod_(N odd >= 3) (1 - N^-dim)` for the corners with an odd count of odd coordinates and `prod (1 + N^-dim)` for the even count, at `dim >= 2`.
///
/// At odd `N` the sum of `(-1)^(x_1 + ... + x_dim)` over the box is 1, so the two designs fill `(N^dim -+ 1)/2` and the row word's constant is this product. The logarithm is summed over odd `N` from 3 to 999, smallest term first, and the tail past 1001 is the series `sum_j (-+1)^j / j sum N^(-dim j)`, each inner sum by Euler-Maclaurin through the fourth Bernoulli term, as for the solid limit.
///
/// ```
/// let plane = mrly_num::sieve::parity_product(2, true).unwrap();
/// assert!((plane - std::f64::consts::FRAC_PI_4).abs() < 1e-15);
/// ```
///
/// # Errors
///
/// Errs below two dimensions, where the product runs off to 0 or to infinity.
pub fn parity_product(dimension: u32, odd: bool) -> Result<f64> {
    if dimension < 2 {
        return value_error(format!(
            "a parity product converges from dim 2, not dim {dimension}."
        ));
    }
    let power = dimension as i32;
    let sign = if odd { -1.0 } else { 1.0 };
    let mut log = 0.0f64;
    let mut n = SPLIT - 2;
    while n >= 3 {
        log += (sign * (n as f64).powi(-power)).ln_1p();
        n -= 2;
    }
    for j in 1..=4i32 {
        let term = odd_tail(SPLIT, power * j) / f64::from(j);
        log += if odd || j % 2 == 0 { -term } else { term };
    }
    Ok(log.exp())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sieve::flat_word;
    use std::f64::consts::{FRAC_PI_4, PI};

    fn truncated(start: u64, step: u64, stop: u64, scale: f64) -> f64 {
        let count = (stop - start) / step + 1;
        let mut sum = 0.0f64;
        for place in (0..count).rev() {
            let n = (start + place * step) as f64;
            sum += (-scale / n.powi(3)).ln_1p();
        }
        sum.exp()
    }

    #[test]
    fn the_solid_limit_holds_the_true_constant_to_four_ulps() {
        assert!((solid_limit() - 0.948_815_485_719_679_6).abs() < 4e-16);
    }

    #[test]
    fn the_solid_limit_matches_its_truncated_product_to_one_part_in_1e14() {
        let product = truncated(3, 2, 8_000_001, 1.0);
        assert!((solid_limit() - product).abs() < 1e-14, "{product}");
    }

    #[test]
    fn the_solid_limit_splits_into_the_cosh_and_the_even_product() {
        let all = truncated(2, 1, 2_000_000, 1.0);
        let cosh = (PI * 3f64.sqrt() / 2.0).cosh() / (3.0 * PI);
        assert!((all - cosh).abs() < 1e-12, "{all} {cosh}");
        let even = truncated(1, 1, 1_000_000, 0.125);
        assert!((solid_limit() - cosh / even).abs() < 1e-12);
    }

    #[test]
    fn the_limit_names_a_schedule_or_says_it_cannot() {
        let plane = limit(&[5, 7, 9], 2).unwrap().unwrap();
        assert!((plane - FRAC_PI_4 / (8.0 / 9.0)).abs() < 1e-15, "{plane}");
        assert_eq!(format!("{plane:.6}"), "0.883573");
        let solid = limit(&[5, 7, 9], 3).unwrap().unwrap();
        assert!(
            (solid - solid_limit() / (26.0 / 27.0)).abs() < 1e-15,
            "{solid}"
        );
        assert_eq!(limit(&odd_word(4), 2).unwrap(), Some(PLANE_LIMIT));
        assert_eq!(limit(&flat_word(7, 3), 2).unwrap(), Some(0.0));
        assert_eq!(limit(&[3, 7, 11], 2).unwrap(), None);
        assert_eq!(limit(&[5], 2).unwrap(), None);
        assert_eq!(limit(&[3, 5, 3, 5], 2).unwrap(), None);
        assert_eq!(limit(&odd_word(4), 4).unwrap(), None);
    }
}
