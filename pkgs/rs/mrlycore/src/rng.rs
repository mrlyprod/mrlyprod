use crate::chacha::ChaCha8;

/// A seeded, seekable ChaCha8 random stream.
#[derive(Clone)]
pub struct Rng {
    inner: ChaCha8,
}

impl Rng {
    /// Builds the stream from a seed.
    pub fn new(seed: u64) -> Rng {
        Rng {
            inner: ChaCha8::from_u64(seed),
        }
    }
    /// Draws an integer between lo and hi inclusive, or lo when hi is not above lo.
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        if hi <= lo {
            lo
        } else {
            self.inner.range_i64(lo, hi)
        }
    }
    /// Draws an integer below n, or zero when n is zero.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            self.inner.below_u64(n as u64) as usize
        }
    }
    /// Draws a float at or above zero and below one.
    pub fn unit(&mut self) -> f64 {
        self.inner.unit()
    }
    /// Draws a fair coin flip.
    pub fn boolean(&mut self) -> bool {
        self.inner.boolean()
    }
    /// Returns true with probability p.
    pub fn chance(&mut self, p: f64) -> bool {
        self.unit() < p
    }
    /// Draws one element of the slice.
    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
    /// Returns the stream's current word position.
    pub fn pos(&self) -> u128 {
        self.inner.word_pos()
    }
    /// Moves the stream to a word position so later draws replay from there.
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
