use mrly_core::error::{Result, value_error};

/// The highest level [`gasket`] counts.
pub const GASKET_LEVELS: u32 = 32;

fn odd_coprime(mut a: u64, mut b: u64) -> bool {
    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    a == 1
}

/// Counts the coprime points of the Sierpinski gasket at a level: the pairs `0 <= i, j < 2^level` with `i AND j = 0` and `gcd(i, j) = 1`, the sequence [A396934](https://oeis.org/A396934).
///
/// Walks the rows `m = i + j = i OR j`: only odd rows hold a coprime pair, `gcd(i, j) = gcd(i, m)`, the odd submasks `i` of `m` are kept when coprime to `m`, and the swap `(i, j) -> (j, i)` doubles them.
///
/// ```
/// let terms: Vec<u64> = (0..=6).map(|level| mrly_math::coprime::gasket(level).unwrap()).collect();
/// assert_eq!(terms, [0, 2, 4, 12, 34, 122, 362]);
/// ```
///
/// # Errors
///
/// Fails past [`GASKET_LEVELS`].
pub fn gasket(level: u32) -> Result<u64> {
    if level > GASKET_LEVELS {
        return value_error(format!(
            "level {level} is past the {GASKET_LEVELS} the gasket count reaches."
        ));
    }
    let mut count = 0u64;
    let mut m = 1u64;
    while m < 1u64 << level {
        let rest = m - 1;
        let mut sub = rest;
        loop {
            if odd_coprime(sub + 1, m) {
                count += 1;
            }
            if sub == 0 {
                break;
            }
            sub = (sub - 1) & rest;
        }
        m += 2;
    }
    Ok(2 * count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "a billion gcds to level twenty, 45 s; run it in release"]
    fn the_gasket_reads_a396934_to_level_20() {
        let a396934: [u64; 21] = [
            0, 2, 4, 12, 34, 122, 362, 1130, 3406, 10506, 31550, 95260, 285838, 860244, 2581102,
            7746082, 23227630, 69761602, 209332584, 628052220, 1884174908,
        ];
        let terms: Vec<u64> = (0..=20).map(|level| gasket(level).unwrap()).collect();
        assert_eq!(terms, a396934);
        assert!(gasket(GASKET_LEVELS + 1).is_err());
    }
}
