use crate::core::error::{value_error, Result};

/// A seeded xoshiro256++ random stream.
#[derive(Clone, Debug)]
pub struct Rng {
    s: [u64; 4],
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

impl Rng {
    /// Builds the stream from a seed.
    ///
    /// ```
    /// use mrlyrs::core::Rng;
    /// let mut rng = Rng::new(42);
    /// assert_eq!(rng.below(6), Rng::new(42).below(6));
    /// ```
    pub fn new(seed: u64) -> Rng {
        let mut state = seed;
        let mut s = [0u64; 4];
        for word in s.iter_mut() {
            *word = splitmix(&mut state);
        }
        if s == [0; 4] {
            s[0] |= 1;
        }
        Rng { s }
    }
    fn next_u64(&mut self) -> u64 {
        let [s0, s1, s2, s3] = self.s;
        let out = s0.wrapping_add(s3).rotate_left(23).wrapping_add(s0);
        let t = s1 << 17;
        let s2 = s2 ^ s0;
        let s3 = s3 ^ s1;
        let s1 = s1 ^ s2;
        let s0 = s0 ^ s3;
        self.s = [s0, s1, s2 ^ t, s3.rotate_left(45)];
        out
    }
    fn below_u64(&mut self, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut m = u128::from(self.next_u64()) * u128::from(n);
        let mut low = m as u64;
        if low < n {
            let zone = n.wrapping_neg() % n;
            while low < zone {
                m = u128::from(self.next_u64()) * u128::from(n);
                low = m as u64;
            }
        }
        (m >> 64) as u64
    }
    /// Draws a float at or above zero and below one.
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    /// Draws an integer below n, or zero when n is zero.
    pub fn below(&mut self, n: usize) -> usize {
        self.below_u64(n as u64) as usize
    }
    /// Draws an integer between lo and hi inclusive, or lo when hi is not above lo.
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        if hi <= lo {
            lo
        } else {
            lo.wrapping_add(self.below_u64(hi.abs_diff(lo).wrapping_add(1)) as i64)
        }
    }
    /// Draws a fair coin flip.
    pub fn boolean(&mut self) -> bool {
        self.next_u64() >> 63 == 1
    }
    /// Returns true with probability p.
    pub fn chance(&mut self, p: f64) -> bool {
        self.unit() < p
    }
    /// Draws one element of the slice.
    ///
    /// # Errors
    ///
    /// Errs when the slice is empty.
    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> Result<&'a T> {
        if items.is_empty() {
            return value_error("a choice wants at least one item.");
        }
        Ok(&items[self.below(items.len())])
    }
    /// Shuffles the slice in place.
    pub fn shuffle<T>(&mut self, seq: &mut [T]) {
        for i in (1..seq.len()).rev() {
            seq.swap(i, self.below(i + 1));
        }
    }
    /// Draws amount distinct indices below length, or every index when amount is larger.
    pub fn sample_indices(&mut self, length: usize, amount: usize) -> Vec<usize> {
        let amount = amount.min(length);
        let mut pool: Vec<usize> = (0..length).collect();
        for i in 0..amount {
            let j = i + self.below(length - i);
            pool.swap(i, j);
        }
        pool.truncate(amount);
        pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_seed_replays_one_stream() {
        let draw = |mut rng: Rng| (0..10).map(|_| rng.range(0, 100)).collect::<Vec<i64>>();
        assert_eq!(draw(Rng::new(42)), draw(Rng::new(42)));
        assert_ne!(draw(Rng::new(42)), draw(Rng::new(43)));
    }
    #[test]
    fn below_zero_is_zero() {
        let mut rng = Rng::new(1);
        assert_eq!(rng.below(0), 0);
        assert!((0..100).all(|_| rng.below(3) < 3));
    }
    #[test]
    fn range_is_inclusive_and_collapses_when_hi_is_not_above_lo() {
        let mut rng = Rng::new(7);
        let drawn: Vec<i64> = (0..200).map(|_| rng.range(1, 3)).collect();
        assert!(drawn.iter().all(|v| (1..=3).contains(v)));
        assert!([1, 2, 3].iter().all(|v| drawn.contains(v)));
        assert_eq!(rng.range(5, 5), 5);
        assert_eq!(rng.range(5, 2), 5);
        assert!(rng.range(0, i64::MAX) >= 0);
    }
    #[test]
    fn unit_lies_in_the_half_open_interval() {
        let mut rng = Rng::new(3);
        assert!((0..1000).all(|_| {
            let u = rng.unit();
            (0.0..1.0).contains(&u)
        }));
    }
    #[test]
    fn shuffle_is_a_permutation() {
        let mut rng = Rng::new(9);
        let mut items: Vec<usize> = (0..20).collect();
        rng.shuffle(&mut items);
        let mut sorted = items.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..20).collect::<Vec<usize>>());
        assert_ne!(items, sorted);
    }
    #[test]
    fn refuses_a_choice_with_nothing_to_draw() {
        let mut rng = Rng::new(2);
        let empty: [usize; 0] = [];
        assert!(rng.choice(&empty).is_err());
        assert_eq!(*rng.choice(&[7]).unwrap(), 7);
    }
    #[test]
    fn sample_indices_are_distinct_and_in_range() {
        let mut rng = Rng::new(4);
        let picked = rng.sample_indices(10, 4);
        assert_eq!(picked.len(), 4);
        assert!(picked.iter().all(|&i| i < 10));
        let mut sorted = picked.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 4);
        assert_eq!(rng.sample_indices(3, 9).len(), 3);
        assert!(rng.sample_indices(0, 2).is_empty());
    }
}
