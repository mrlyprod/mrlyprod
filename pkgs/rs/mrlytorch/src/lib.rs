#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Mixes a train and a step into one reproducible seed.
///
/// ```
/// assert_eq!(mrlytorch::seed(1, 0), mrlytorch::seed(1, 0));
/// assert_ne!(mrlytorch::seed(1, 0), mrlytorch::seed(1, 1));
/// ```
pub fn seed(train: u64, step: u64) -> u64 {
    let mut x = train ^ step.rotate_left(32) ^ 0x9E3779B97F4A7C15;
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_run_replays() {
        assert_eq!(super::seed(7, 125), super::seed(7, 125));
        assert_ne!(super::seed(7, 125), super::seed(8, 125));
        assert_ne!(super::seed(0, 0), 0);
    }
}
