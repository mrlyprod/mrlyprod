use rand::{Rng as RandRng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Clone)]
pub struct Rng {
    inner: ChaCha8Rng,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng {
            inner: ChaCha8Rng::seed_from_u64(seed),
        }
    }
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        if hi <= lo {
            lo
        } else {
            self.inner.gen_range(lo..=hi)
        }
    }
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            self.inner.gen_range(0..n as u64) as usize
        }
    }
    pub fn unit(&mut self) -> f64 {
        self.inner.gen::<f64>()
    }
    pub fn boolean(&mut self) -> bool {
        self.inner.gen::<bool>()
    }
    pub fn chance(&mut self, p: f64) -> bool {
        self.unit() < p
    }
    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
    pub fn pos(&self) -> u128 {
        self.inner.get_word_pos()
    }
    pub fn seek(&mut self, pos: u128) {
        self.inner.set_word_pos(pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn seek_resumes_the_stream() {
        let mut a = Rng::new(7);
        a.below(100);
        a.unit();
        let pos = a.pos();
        let next: Vec<usize> = (0..5).map(|_| a.below(1000)).collect();
        let mut b = Rng::new(7);
        b.seek(pos);
        let again: Vec<usize> = (0..5).map(|_| b.below(1000)).collect();
        assert_eq!(next, again);
    }
}
