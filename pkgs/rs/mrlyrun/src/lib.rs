#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Waits this many milliseconds before an attempt, doubling from the beat and resting at eight seconds.
///
/// ```
/// assert_eq!(mrlyrun::backoff(0), 0);
/// assert_eq!(mrlyrun::backoff(3), 500);
/// assert_eq!(mrlyrun::backoff(9), 8000);
/// ```
pub fn backoff(attempt: u32) -> u64 {
    match attempt {
        0 => 0,
        n => 125 << (n - 1).min(6),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn doubles_then_rests() {
        let waits: Vec<u64> = (0..9).map(super::backoff).collect();
        assert_eq!(waits, [0, 125, 250, 500, 1000, 2000, 4000, 8000, 8000]);
    }
}
