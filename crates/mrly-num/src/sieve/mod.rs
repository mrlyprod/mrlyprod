use mrly_core::error::{Result, overflow_error, value_error};
use std::f64::consts::FRAC_PI_4;

mod limit;
mod row;

pub use limit::{limit, parity_product, solid_limit};
pub use row::{Row, row_fill, row_law, row_profile, row_settle};

/// The limit of the plane Wallis sieve's surviving area, pi over four.
pub const PLANE_LIMIT: f64 = FRAC_PI_4;

/// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
///
/// ```
/// assert_eq!(mrly_num::sieve::odd_word(4), vec![3, 5, 7, 9]);
/// ```
pub fn odd_word(levels: usize) -> Vec<u64> {
    (1..=levels as u64).map(|k| 2 * k + 1).collect()
}

/// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
///
/// ```
/// assert_eq!(mrly_num::sieve::flat_word(3, 3), vec![3, 3, 3]);
/// ```
pub fn flat_word(side: u64, levels: usize) -> Vec<u64> {
    vec![side; levels]
}

fn letter(side: u64) -> Result<u64> {
    if side < 3 || side.is_multiple_of(2) {
        return value_error(format!("a letter is an odd side from three, not {side}."));
    }
    Ok(side)
}

/// Returns the side of the word, the product of its letters' sides.
///
/// ```
/// assert_eq!(mrly_num::sieve::side(&mrly_num::sieve::odd_word(4)).unwrap(), 945);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the product overruns a u128.
pub fn side(word: &[u64]) -> Result<u128> {
    let mut run = 1u128;
    for &s in word {
        run = match run.checked_mul(u128::from(letter(s)?)) {
            Some(value) => value,
            None => return overflow_error("the word's side overruns a u128."),
        };
    }
    Ok(run)
}

/// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.
///
/// ```
/// assert_eq!(mrly_num::sieve::cells(&mrly_num::sieve::odd_word(3), 2).unwrap(), 9216);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the product overruns a u128.
pub fn cells(word: &[u64], dimension: u32) -> Result<u128> {
    let mut run = 1u128;
    for &s in word {
        let fill = u128::from(letter(s)?).pow(dimension) - 1;
        run = match run.checked_mul(fill) {
            Some(value) => value,
            None => return overflow_error("the word's cells overrun a u128."),
        };
    }
    Ok(run)
}

/// Returns the punctures the word makes, one per surviving cell at every level.
///
/// ```
/// assert_eq!(mrly_num::sieve::holes(&mrly_num::sieve::odd_word(3), 2).unwrap(), 1 + 8 + 192);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the count overruns a u128.
pub fn holes(word: &[u64], dimension: u32) -> Result<u128> {
    let mut total = 0u128;
    for place in 0..word.len() {
        total += cells(&word[..place], dimension)?;
    }
    Ok(total)
}

/// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
///
/// ```
/// let flat = mrly_num::sieve::ratio(&mrly_num::sieve::flat_word(3, 2), 2).unwrap();
/// assert!((flat - 64.0 / 81.0).abs() < 1e-15);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn ratio(word: &[u64], dimension: u32) -> Result<f64> {
    let mut run = 1.0f64;
    for &s in word {
        run *= 1.0 - 1.0 / (letter(s)? as f64).powi(dimension as i32);
    }
    Ok(run)
}

/// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
///
/// Changing the letter is not enough: the alternating word 3, 5, 3, 5 changes at every step and freezes at log 192 / log 15, its ratio falling to nothing like a fixed-ratio schedule. The hypothesis that buys a positive area is strictly increasing odd letters, under which sum s_k^(-d) converges.
///
/// ```
/// let carpet = mrly_num::sieve::exponent(&mrly_num::sieve::flat_word(3, 5), 2).unwrap();
/// assert!((carpet - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn exponent(word: &[u64], dimension: u32) -> Result<f64> {
    let mut up = 0.0f64;
    let mut down = 0.0f64;
    for &s in word {
        let s = letter(s)? as f64;
        up += (s.powi(dimension as i32) - 1.0).ln();
        down += s.ln();
    }
    if down == 0.0 {
        return Ok(dimension as f64);
    }
    Ok(up / down)
}

/// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
///
/// ```
/// let (side, cells) = mrly_num::sieve::raster(&mrly_num::sieve::odd_word(2)).unwrap();
/// assert_eq!(side, 15);
/// assert_eq!(cells.iter().filter(|&&b| b == 1).count(), 192);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn raster(word: &[u64]) -> Result<(usize, Vec<u8>)> {
    let mut side = 1usize;
    let mut sites = vec![1u8];
    for &s in word {
        let s = letter(s)? as usize;
        let half = s / 2;
        let wide = side * s;
        let mut next = vec![0u8; wide * wide];
        for row in 0..side {
            for col in 0..side {
                if sites[row * side + col] == 0 {
                    continue;
                }
                for i in 0..s {
                    for j in 0..s {
                        if i == half && j == half {
                            continue;
                        }
                        next[(row * s + i) * wide + col * s + j] = 1;
                    }
                }
            }
        }
        side = wide;
        sites = next;
    }
    Ok((side, sites))
}

/// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
///
/// ```
/// let holes = mrly_num::sieve::punctures(&mrly_num::sieve::odd_word(2), 2).unwrap();
/// assert_eq!(holes.len() / 3, 9);
/// assert_eq!(&holes[..3], &[5, 5, 5]);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the word's side overruns a u128.
pub fn punctures(word: &[u64], dimension: u32) -> Result<Vec<u64>> {
    let axes = dimension as usize;
    let mut scale = side(word)? as u64;
    let mut alive: Vec<u64> = vec![0; axes];
    let mut out: Vec<u64> = Vec::new();
    let mut digits = vec![0u64; axes];
    for (place, &s) in word.iter().enumerate() {
        let s = letter(s)?;
        let half = s / 2;
        let last = place + 1 == word.len();
        scale /= s;
        let count = s.pow(dimension);
        let mut next: Vec<u64> = Vec::new();
        for origin in alive.chunks(axes) {
            for axis in origin {
                out.push((axis * s + half) * scale);
            }
            out.push(scale);
            if last {
                continue;
            }
            for index in 0..count {
                let mut rest = index;
                let mut centre = true;
                for axis in (0..axes).rev() {
                    digits[axis] = rest % s;
                    centre &= digits[axis] == half;
                    rest /= s;
                }
                if centre {
                    continue;
                }
                for axis in 0..axes {
                    next.push(origin[axis] * s + digits[axis]);
                }
            }
        }
        if !last {
            alive = next;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::wallis_quarter_pi;

    fn brute(word: &[u64]) -> usize {
        raster(word).unwrap().1.iter().filter(|&&b| b == 1).count()
    }

    #[test]
    fn the_odd_word_walks_the_sides_of_the_classical_sieve() {
        let word = odd_word(4);
        let sides: Vec<u128> = (1..=4).map(|n| side(&word[..n]).unwrap()).collect();
        assert_eq!(sides, vec![3, 15, 105, 945]);
        let counts: Vec<u128> = (1..=4).map(|n| cells(&word[..n], 2).unwrap()).collect();
        assert_eq!(counts, vec![8, 192, 9216, 737_280]);
    }

    #[test]
    fn the_raster_counts_what_the_product_promises() {
        let word = odd_word(3);
        for n in 1..=3 {
            assert_eq!(
                brute(&word[..n]) as u128,
                cells(&word[..n], 2).unwrap(),
                "level {n}"
            );
        }
        for n in 1..=4 {
            let flat = flat_word(3, n);
            assert_eq!(
                brute(&flat) as u128,
                cells(&flat, 2).unwrap(),
                "carpet level {n}"
            );
        }
        let five = flat_word(5, 2);
        assert_eq!(brute(&five) as u128, cells(&five, 2).unwrap());
    }

    #[test]
    fn the_raster_is_the_ratio_times_the_area() {
        for n in 1..=3 {
            let word = odd_word(n);
            let (wide, sites) = raster(&word).unwrap();
            let lit = sites.iter().filter(|&&b| b == 1).count() as f64;
            assert!(
                (lit / (wide * wide) as f64 - ratio(&word, 2).unwrap()).abs() < 1e-12,
                "level {n}"
            );
        }
    }

    #[test]
    fn the_punctures_fill_the_gap_the_survivors_leave() {
        for dimension in 2..=3u32 {
            for n in 1..=3 {
                let word = odd_word(n);
                let axes = dimension as usize;
                let list = punctures(&word, dimension).unwrap();
                assert_eq!(
                    list.len() / (axes + 1),
                    holes(&word, dimension).unwrap() as usize
                );
                let volume: u128 = list
                    .chunks(axes + 1)
                    .map(|hole| u128::from(hole[axes]).pow(dimension))
                    .sum();
                let whole = side(&word).unwrap().pow(dimension);
                assert_eq!(
                    volume + cells(&word, dimension).unwrap(),
                    whole,
                    "{dimension}d level {n}"
                );
            }
        }
    }

    #[test]
    fn the_plane_ratio_is_the_wallis_product_the_series_walks() {
        for n in 1..=40 {
            assert!(
                (ratio(&odd_word(n), 2).unwrap() - wallis_quarter_pi(n)).abs() < 1e-15,
                "level {n}"
            );
        }
        assert!((ratio(&odd_word(200_000), 2).unwrap() - PLANE_LIMIT).abs() < 1e-5);
    }

    #[test]
    fn the_carpet_schedule_freezes_the_ratio_and_the_exponent() {
        for n in 1..=8 {
            let word = flat_word(3, n);
            assert!((ratio(&word, 2).unwrap() - (8.0f64 / 9.0).powi(n as i32)).abs() < 1e-15);
            assert!((exponent(&word, 2).unwrap() - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
        }
        let long = odd_word(4_000);
        let read = exponent(&long, 2).unwrap();
        assert!(read > 1.999_5 && read < 2.0);
    }

    #[test]
    fn the_alternating_word_changes_at_every_step_and_freezes() {
        let pinned = 192f64.ln() / 15f64.ln();
        for pairs in 1..=4 {
            let word: Vec<u64> = [3u64, 5].iter().cycle().take(2 * pairs).copied().collect();
            assert!(
                (exponent(&word, 2).unwrap() - pinned).abs() < 1e-12,
                "{pairs}"
            );
        }
        assert_eq!(format!("{pinned:.6}"), "1.941432");
        assert!(ratio(&[3, 5, 3, 5], 2).unwrap() < ratio(&odd_word(4), 2).unwrap());
    }

    #[test]
    fn the_plane_ratio_at_two_million_factors_rounds_to_nine_digits() {
        let read = ratio(&odd_word(2_000_000), 2).unwrap();
        assert_eq!(format!("{read:.9}"), "0.785398262");
        assert!(read > PLANE_LIMIT);
    }

    #[test]
    fn refuses_a_letter_that_is_not_an_odd_side_from_three() {
        for bad in [0u64, 1, 2, 4, 6] {
            assert!(side(&[bad]).is_err(), "{bad}");
            assert!(cells(&[bad], 2).is_err(), "{bad}");
            assert!(holes(&[bad, 3], 2).is_err(), "{bad}");
            assert!(ratio(&[bad], 2).is_err(), "{bad}");
            assert!(exponent(&[bad], 2).is_err(), "{bad}");
            assert!(raster(&[bad]).is_err(), "{bad}");
            assert!(punctures(&[bad], 2).is_err(), "{bad}");
            assert!(limit(&[bad, bad + 2], 2).is_err(), "{bad}");
        }
        assert!(side(&[3]).is_ok());
    }
}
