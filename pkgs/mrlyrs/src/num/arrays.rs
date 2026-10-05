use super::design::elements;
use crate::core::error::{value_error, Result};

/// The most sensors [`fractal`] builds, `2^20`.
pub const MOST: u64 = 1 << 20;

// THE FRACTAL ARRAY

/// Returns the fractal array of the generator at the base and level, ascending: every `sum_(i < level) g_i base^i` with each digit `g_i` in the generator.
///
/// The generator is the digit set of a dim 1 digit design, its least digit `0`; at base `2a + 1`, `a` its largest digit, this is the fractal array of Cohen and Eldar, and every base above `a` gives `L^level` distinct sensors, `L` the size of the generator.
///
/// ```
/// assert_eq!(mrlyrs::num::arrays::fractal(&[0, 1], 3, 2).unwrap(), vec![0, 1, 3, 4]);
/// ```
///
/// # Errors
///
/// Errs when the generator is empty, misses `0`, repeats a digit or holds one at or above the base, or when the array passes [`MOST`] sensors.
pub fn fractal(generator: &[u64], base: u64, level: u32) -> Result<Vec<u64>> {
    let mut digits = generator.to_vec();
    digits.sort_unstable();
    digits.dedup();
    if digits.len() != generator.len() || digits.first() != Some(&0) {
        return value_error("a generator lists distinct digits, 0 among them.");
    }
    if digits.iter().any(|&d| d >= base) {
        return value_error(format!("every digit sits below the base {base}."));
    }
    let size = (digits.len() as u64).checked_pow(level);
    if size.is_none_or(|n| n > MOST) || base.checked_pow(level).is_none() {
        return value_error(format!(
            "level {level} passes the {MOST} sensors an array holds."
        ));
    }
    Ok(std::iter::once(0)
        .chain(elements(base, &digits, level as usize))
        .collect())
}

/// Returns the essential count and the sensor count that Theorem A of the arrays note proves for the fractal array of the generator at base `2a + 1`, `(u^level, L^level)`, `u` the count of [`paired`] digits; `None` off the theorem: a level below 2, a generator without `0`, or a generator whose coarray has a hole.
///
/// ```
/// assert_eq!(mrlyrs::num::arrays::law(&[0, 1, 2, 4], 4), Some((81, 256)));
/// ```
pub fn law(generator: &[u64], level: u32) -> Option<(u64, u64)> {
    if level < 2 || !generator.contains(&0) || !holes(generator).is_empty() {
        return None;
    }
    let u = paired(generator).len() as u64;
    Some((
        u.checked_pow(level)?,
        (generator.len() as u64).checked_pow(level)?,
    ))
}

// THE COARRAY

/// Returns the coarray weights of the sensors, `w(t)` for every lag `t` from `-span` to `span` at index `t + span`, `span` the largest sensor less the least: the count of ordered pairs `(p, q)` of sensors with `p - q = t`.
///
/// ```
/// let w = mrlyrs::num::arrays::weights(&[0, 1, 4, 6]);
/// assert_eq!(w, vec![1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 1]);
/// ```
pub fn weights(sensors: &[u64]) -> Vec<u64> {
    let Some(span) = spread(sensors) else {
        return Vec::new();
    };
    let mut out = vec![0; 2 * span as usize + 1];
    for &p in sensors {
        for &q in sensors {
            out[(p + span - q) as usize] += 1;
        }
    }
    out
}

/// Returns the holes of the coarray, ascending: the lags between `-span` and `span` that no pair of sensors makes.
///
/// ```
/// assert_eq!(mrlyrs::num::arrays::holes(&[0, 1, 4]), vec![-2, 2]);
/// ```
pub fn holes(sensors: &[u64]) -> Vec<i64> {
    let span = spread(sensors).unwrap_or(0) as i64;
    weights(sensors)
        .iter()
        .enumerate()
        .filter(|&(_, &w)| w == 0)
        .map(|(i, _)| i as i64 - span)
        .collect()
}

/// Returns the paired sensors, ascending: each `g` with a partner `h` whose lag `g - h` has weight `1`, the set `U` of condition C1 of Cohen and Eldar; a paired sensor is essential.
///
/// ```
/// assert_eq!(mrlyrs::num::arrays::paired(&[0, 1, 2, 4]), vec![0, 1, 4]);
/// ```
pub fn paired(sensors: &[u64]) -> Vec<u64> {
    let Some(span) = spread(sensors) else {
        return Vec::new();
    };
    let w = weights(sensors);
    let mut out: Vec<u64> = sensors
        .iter()
        .copied()
        .filter(|&g| {
            sensors
                .iter()
                .any(|&h| h != g && w[(g + span - h) as usize] == 1)
        })
        .collect();
    out.sort_unstable();
    out
}

/// Returns whether each sensor is essential, in the order given: whether removing it deletes a lag from the coarray, which happens exactly when some lag `t != 0` has all its pairs through it, the one pair of a lag of weight `1` or the two pairs `(s, s - t)` and `(s + t, s)` of a lag of weight `2`.
///
/// ```
/// let flags = mrlyrs::num::arrays::essential(&[0, 1, 2]);
/// assert_eq!(flags, vec![true, true, true]);
/// ```
pub fn essential(sensors: &[u64]) -> Vec<bool> {
    let Some(reader) = Reader::new(sensors) else {
        return Vec::new();
    };
    sensors
        .iter()
        .map(|&s| reader.kills(s).next().is_some())
        .collect()
}

/// Returns the lags that removing the sensor deletes from the coarray, ascending: those every pair of which holds it; empty when the sensor is inessential or absent.
///
/// ```
/// assert_eq!(mrlyrs::num::arrays::lost(&[0, 1, 2], 1), vec![-1, 1]);
/// ```
pub fn lost(sensors: &[u64], sensor: u64) -> Vec<i64> {
    let Some(reader) = Reader::new(sensors) else {
        return Vec::new();
    };
    if !reader.holds(sensor as i64) {
        return Vec::new();
    }
    let mut out: Vec<i64> = reader.kills(sensor).flat_map(|t| [t, -t]).collect();
    out.sort_unstable();
    out.dedup();
    out
}

fn spread(sensors: &[u64]) -> Option<u64> {
    Some(sensors.iter().max()? - sensors.iter().min()?)
}

struct Reader<'a> {
    sensors: &'a [u64],
    least: i64,
    span: i64,
    weights: Vec<u64>,
    member: Vec<bool>,
}

impl<'a> Reader<'a> {
    fn new(sensors: &'a [u64]) -> Option<Reader<'a>> {
        let span = spread(sensors)?;
        let least = *sensors.iter().min()?;
        let mut member = vec![false; span as usize + 1];
        for &s in sensors {
            member[(s - least) as usize] = true;
        }
        Some(Reader {
            sensors,
            least: least as i64,
            span: span as i64,
            weights: weights(sensors),
            member,
        })
    }

    fn holds(&self, x: i64) -> bool {
        (0..=self.span).contains(&(x - self.least)) && self.member[(x - self.least) as usize]
    }

    fn kills(&self, sensor: u64) -> impl Iterator<Item = i64> + '_ {
        let s = sensor as i64;
        self.sensors.iter().filter_map(move |&q| {
            let t = s - q as i64;
            match self.weights[(t + self.span) as usize] {
                1 if t != 0 => Some(t),
                2 if t != 0 && self.holds(s + t) => Some(t),
                _ => None,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn coarray(s: &[u64]) -> BTreeSet<i64> {
        s.iter()
            .flat_map(|&p| s.iter().map(move |&q| p as i64 - q as i64))
            .collect()
    }

    fn brute(s: &[u64]) -> Vec<bool> {
        let whole = coarray(s);
        s.iter()
            .map(|&x| {
                let rest: Vec<u64> = s.iter().copied().filter(|&y| y != x).collect();
                coarray(&rest) != whole
            })
            .collect()
    }

    fn count(generator: &[u64], base: u64, level: u32) -> usize {
        let f = fractal(generator, base, level).unwrap();
        essential(&f).iter().filter(|&&e| e).count()
    }

    fn row(generator: &[u64], base: u64, levels: u32) -> Vec<usize> {
        (1..=levels).map(|r| count(generator, base, r)).collect()
    }

    #[test]
    fn the_fast_test_is_the_definition() {
        for (g, b, r) in [
            (&[0, 1, 2][..], 5, 2),
            (&[0, 1, 2][..], 4, 3),
            (&[0, 1, 2, 4][..], 9, 2),
            (&[0, 1, 4, 6][..], 8, 3),
            (&[0, 1, 2, 3][..], 7, 2),
            (&[0, 1, 2, 3, 7][..], 11, 2),
            (&[0, 3, 5][..], 6, 3),
        ] {
            let f = fractal(g, b, r).unwrap();
            assert_eq!(essential(&f), brute(&f));
            for &s in &f {
                let rest: Vec<u64> = f.iter().copied().filter(|&y| y != s).collect();
                let gone: Vec<i64> = coarray(&f).difference(&coarray(&rest)).copied().collect();
                assert_eq!(lost(&f, s), gone);
            }
        }
    }

    #[test]
    fn theorem_a_counts_u_to_the_level() {
        let g = [0, 1, 2, 4];
        assert_eq!(paired(&g), vec![0, 1, 4]);
        assert_eq!(count(&g, 9, 4), 81);
        assert_eq!(law(&g, 4), Some((81, 256)));
        assert_eq!((count(&[0, 1, 2], 5, 2), count(&[0, 1, 2], 5, 3)), (4, 8));
        assert_eq!(count(&[0, 1, 2, 3], 7, 2), 4);
        assert_eq!(count(&[0, 1, 4, 6], 13, 3), 64);
        assert_eq!((count(&g, 9, 5), count(&g, 9, 6)), (243, 729));
        assert_eq!(law(&[0, 1, 2], 1), None);
        assert_eq!(law(&[0, 1, 4], 2), None);
        assert!(holes(&fractal(&g, 9, 3).unwrap()).is_empty());
    }

    #[test]
    fn cohen_and_eldar_table_one_reads_as_printed() {
        let s = [0, 1, 2, 4, 7, 10, 13, 16, 18, 19, 20];
        let g = [0, 1, 3, 5, 11, 13, 17, 18, 19, 20];
        assert_eq!(
            weights(&s)[20..],
            [11, 4, 4, 6, 2, 2, 5, 2, 2, 4, 2, 2, 3, 2, 2, 2, 3, 2, 3, 2, 1]
        );
        assert_eq!(
            weights(&g)[20..],
            [10, 4, 5, 2, 2, 2, 3, 2, 3, 1, 2, 1, 2, 2, 2, 2, 2, 3, 2, 2, 1]
        );
        assert_eq!(paired(&s), vec![0, 20]);
        assert_eq!(paired(&g), vec![0, 11, 20]);
        assert_eq!(row(&s, 41, 3), [3, 4, 8]);
        assert_eq!(row(&g, 41, 3), [3, 9, 27]);
    }

    #[test]
    fn the_compressed_base_reads_as_printed() {
        assert_eq!(row(&[0, 1, 2], 4, 6), [3, 2, 2, 2, 2, 2]);
        assert_eq!(row(&[0, 1, 4, 6], 7, 5), [4, 7, 6, 6, 6]);
        assert_eq!(row(&[0, 1, 4, 6], 8, 5), [4, 11, 25, 53, 109]);
        assert_eq!(row(&[0, 1, 2, 3, 7], 11, 4), [5, 14, 29, 61]);
        assert!(holes(&fractal(&[0, 1, 4, 6], 8, 4).unwrap()).is_empty());
    }

    #[test]
    fn a_bad_generator_errs() {
        assert!(fractal(&[1, 2], 5, 2).is_err());
        assert!(fractal(&[0, 1, 1], 5, 2).is_err());
        assert!(fractal(&[0, 5], 5, 2).is_err());
        assert!(fractal(&[0, 1], 3, 21).is_err());
    }
}
