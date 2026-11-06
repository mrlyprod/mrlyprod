use mrly_error::{Result, value_error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    pub fn below(&mut self, n: usize) -> usize {
        self.below_u64(n as u64) as usize
    }
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        if hi <= lo {
            lo
        } else {
            lo.wrapping_add(self.below_u64(hi.abs_diff(lo).wrapping_add(1)) as i64)
        }
    }
    pub fn boolean(&mut self) -> bool {
        self.next_u64() >> 63 == 1
    }
    pub fn chance(&mut self, p: f64) -> bool {
        self.unit() < p
    }
    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> Result<&'a T> {
        if items.is_empty() {
            return value_error("a choice wants at least one item.");
        }
        Ok(&items[self.below(items.len())])
    }
    pub fn shuffle<T>(&mut self, seq: &mut [T]) {
        for i in (1..seq.len()).rev() {
            seq.swap(i, self.below(i + 1));
        }
    }
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
