use std::f64::consts::FRAC_PI_4;

/// The limit of the plane Wallis sieve's surviving area, pi over four.
pub const PLANE_LIMIT: f64 = FRAC_PI_4;

/// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
///
/// ```
/// assert_eq!(mrlynum::sieve::odd_word(4), vec![3, 5, 7, 9]);
/// ```
pub fn odd_word(levels: usize) -> Vec<u64> {
    (1..=levels as u64).map(|k| 2 * k + 1).collect()
}

/// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
///
/// ```
/// assert_eq!(mrlynum::sieve::flat_word(3, 3), vec![3, 3, 3]);
/// ```
pub fn flat_word(side: u64, levels: usize) -> Vec<u64> {
    vec![side; levels]
}

fn letter(side: u64) -> u64 {
    assert!(
        side >= 3 && side % 2 == 1,
        "a letter is an odd side from three"
    );
    side
}

/// Returns the side of the word, the product of its letters' sides, and panics when that overruns a u128.
///
/// ```
/// assert_eq!(mrlynum::sieve::side(&mrlynum::sieve::odd_word(4)), 945);
/// ```
pub fn side(word: &[u64]) -> u128 {
    word.iter().fold(1u128, |run, &s| {
        run.checked_mul(u128::from(letter(s)))
            .expect("the word's side overruns a u128")
    })
}

/// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter, and panics when that overruns a u128.
///
/// ```
/// assert_eq!(mrlynum::sieve::cells(&mrlynum::sieve::odd_word(3), 2), 9216);
/// ```
pub fn cells(word: &[u64], dimension: u32) -> u128 {
    word.iter().fold(1u128, |run, &s| {
        let fill = u128::from(letter(s)).pow(dimension) - 1;
        run.checked_mul(fill)
            .expect("the word's cells overrun a u128")
    })
}

/// Returns the punctures the word makes, one per surviving cell at every level, and panics when that overruns a u128.
///
/// ```
/// assert_eq!(mrlynum::sieve::holes(&mrlynum::sieve::odd_word(3), 2), 1 + 8 + 192);
/// ```
pub fn holes(word: &[u64], dimension: u32) -> u128 {
    let mut total = 0u128;
    for place in 0..word.len() {
        total += cells(&word[..place], dimension);
    }
    total
}

/// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
///
/// ```
/// let flat = mrlynum::sieve::ratio(&mrlynum::sieve::flat_word(3, 2), 2);
/// assert!((flat - 64.0 / 81.0).abs() < 1e-15);
/// ```
pub fn ratio(word: &[u64], dimension: u32) -> f64 {
    word.iter().fold(1.0f64, |run, &s| {
        run * (1.0 - 1.0 / (letter(s) as f64).powi(dimension as i32))
    })
}

/// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
///
/// Changing the letter is not enough: the alternating word 3, 5, 3, 5 changes at every step and freezes at log 192 / log 15, its ratio falling to nothing like a fixed-ratio schedule. The hypothesis that buys a positive area is strictly increasing odd letters, under which sum s_k^(-d) converges.
///
/// ```
/// let carpet = mrlynum::sieve::exponent(&mrlynum::sieve::flat_word(3, 5), 2);
/// assert!((carpet - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
/// ```
pub fn exponent(word: &[u64], dimension: u32) -> f64 {
    let mut up = 0.0f64;
    let mut down = 0.0f64;
    for &s in word {
        let s = letter(s) as f64;
        up += (s.powi(dimension as i32) - 1.0).ln();
        down += s.ln();
    }
    if down == 0.0 {
        return dimension as f64;
    }
    up / down
}

/// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
///
/// ```
/// let (side, cells) = mrlynum::sieve::raster(&mrlynum::sieve::odd_word(2));
/// assert_eq!(side, 15);
/// assert_eq!(cells.iter().filter(|&&b| b == 1).count(), 192);
/// ```
pub fn raster(word: &[u64]) -> (usize, Vec<u8>) {
    let mut side = 1usize;
    let mut sites = vec![1u8];
    for &s in word {
        let s = letter(s) as usize;
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
    (side, sites)
}

/// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
///
/// ```
/// let holes = mrlynum::sieve::punctures(&mrlynum::sieve::odd_word(2), 2);
/// assert_eq!(holes.len() / 3, 9);
/// assert_eq!(&holes[..3], &[5, 5, 5]);
/// ```
pub fn punctures(word: &[u64], dimension: u32) -> Vec<u64> {
    let axes = dimension as usize;
    let mut scale = side(word) as u64;
    let mut alive: Vec<u64> = vec![0; axes];
    let mut out: Vec<u64> = Vec::new();
    let mut digits = vec![0u64; axes];
    for (place, &s) in word.iter().enumerate() {
        let s = letter(s);
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
    out
}

// THE SOLID LIMIT

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
/// assert!((mrlynum::sieve::solid_limit() - 0.948_815_485_719_679_6).abs() < 4e-16);
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
/// assert_eq!(mrlynum::sieve::limit(&mrlynum::sieve::odd_word(3), 2), Some(std::f64::consts::FRAC_PI_4));
/// assert_eq!(mrlynum::sieve::limit(&mrlynum::sieve::flat_word(3, 3), 2), Some(0.0));
/// assert!((mrlynum::sieve::limit(&[5, 7, 9], 2).unwrap() - 0.883_572_933_822_129_3).abs() < 1e-15);
/// assert_eq!(mrlynum::sieve::limit(&[3, 7, 11], 2), None);
/// ```
pub fn limit(word: &[u64], dimension: u32) -> Option<f64> {
    if word.len() < 2 {
        return None;
    }
    let first = letter(word[0]);
    if word.iter().all(|&s| letter(s) == first) {
        return Some(0.0);
    }
    if !word
        .iter()
        .enumerate()
        .all(|(k, &s)| s == first + 2 * k as u64)
    {
        return None;
    }
    let whole = match dimension {
        2 => PLANE_LIMIT,
        3 => solid_limit(),
        _ => return None,
    };
    let head = odd_word(((first - 3) / 2) as usize);
    Some(whole / ratio(&head, dimension))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::wallis;
    use std::f64::consts::PI;

    fn truncated(start: u64, step: u64, stop: u64, scale: f64) -> f64 {
        let count = (stop - start) / step + 1;
        let mut sum = 0.0f64;
        for place in (0..count).rev() {
            let n = (start + place * step) as f64;
            sum += (-scale / n.powi(3)).ln_1p();
        }
        sum.exp()
    }

    fn brute(word: &[u64]) -> usize {
        raster(word).1.iter().filter(|&&b| b == 1).count()
    }

    #[test]
    fn the_odd_word_walks_the_sides_of_the_classical_sieve() {
        let word = odd_word(4);
        let sides: Vec<u128> = (1..=4).map(|n| side(&word[..n])).collect();
        assert_eq!(sides, vec![3, 15, 105, 945]);
        let counts: Vec<u128> = (1..=4).map(|n| cells(&word[..n], 2)).collect();
        assert_eq!(counts, vec![8, 192, 9216, 737_280]);
    }

    #[test]
    fn the_raster_counts_what_the_product_promises() {
        let word = odd_word(3);
        for n in 1..=3 {
            assert_eq!(brute(&word[..n]) as u128, cells(&word[..n], 2), "level {n}");
        }
        for n in 1..=4 {
            let flat = flat_word(3, n);
            assert_eq!(brute(&flat) as u128, cells(&flat, 2), "carpet level {n}");
        }
        let five = flat_word(5, 2);
        assert_eq!(brute(&five) as u128, cells(&five, 2));
    }

    #[test]
    fn the_raster_is_the_ratio_times_the_area() {
        for n in 1..=3 {
            let word = odd_word(n);
            let (wide, sites) = raster(&word);
            let lit = sites.iter().filter(|&&b| b == 1).count() as f64;
            assert!(
                (lit / (wide * wide) as f64 - ratio(&word, 2)).abs() < 1e-12,
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
                let list = punctures(&word, dimension);
                assert_eq!(list.len() / (axes + 1), holes(&word, dimension) as usize);
                let volume: u128 = list
                    .chunks(axes + 1)
                    .map(|hole| u128::from(hole[axes]).pow(dimension))
                    .sum();
                let whole = side(&word).pow(dimension);
                assert_eq!(
                    volume + cells(&word, dimension),
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
                (ratio(&odd_word(n), 2) - wallis(n)).abs() < 1e-15,
                "level {n}"
            );
        }
        assert!((ratio(&odd_word(200_000), 2) - PLANE_LIMIT).abs() < 1e-5);
    }

    #[test]
    fn the_carpet_schedule_freezes_the_ratio_and_the_exponent() {
        for n in 1..=8 {
            let word = flat_word(3, n);
            assert!((ratio(&word, 2) - (8.0f64 / 9.0).powi(n as i32)).abs() < 1e-15);
            assert!((exponent(&word, 2) - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
        }
        let long = odd_word(4_000);
        assert!(exponent(&long, 2) > 1.999_5 && exponent(&long, 2) < 2.0);
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
    fn the_alternating_word_changes_at_every_step_and_freezes() {
        let pinned = 192f64.ln() / 15f64.ln();
        for pairs in 1..=4 {
            let word: Vec<u64> = [3u64, 5].iter().cycle().take(2 * pairs).copied().collect();
            assert!((exponent(&word, 2) - pinned).abs() < 1e-12, "{pairs}");
        }
        assert_eq!(format!("{pinned:.6}"), "1.941432");
        assert!(ratio(&[3, 5, 3, 5], 2) < ratio(&odd_word(4), 2));
    }

    #[test]
    fn the_limit_names_a_schedule_or_says_it_cannot() {
        let plane = limit(&[5, 7, 9], 2).unwrap();
        assert!((plane - FRAC_PI_4 / (8.0 / 9.0)).abs() < 1e-15, "{plane}");
        assert_eq!(format!("{plane:.6}"), "0.883573");
        let solid = limit(&[5, 7, 9], 3).unwrap();
        assert!(
            (solid - solid_limit() / (26.0 / 27.0)).abs() < 1e-15,
            "{solid}"
        );
        assert_eq!(limit(&odd_word(4), 2), Some(PLANE_LIMIT));
        assert_eq!(limit(&flat_word(7, 3), 2), Some(0.0));
        assert_eq!(limit(&[3, 7, 11], 2), None);
        assert_eq!(limit(&[5], 2), None);
        assert_eq!(limit(&[3, 5, 3, 5], 2), None);
        assert_eq!(limit(&odd_word(4), 4), None);
    }

    #[test]
    fn the_plane_ratio_at_two_million_factors_rounds_to_nine_digits() {
        let read = ratio(&odd_word(2_000_000), 2);
        assert_eq!(format!("{read:.9}"), "0.785398262");
        assert!(read > PLANE_LIMIT);
    }

    #[test]
    #[should_panic(expected = "a letter is an odd side from three")]
    fn an_even_letter_has_no_centre_to_punch() {
        let _ = cells(&[4], 2);
    }
}
