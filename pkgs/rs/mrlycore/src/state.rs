use crate::chacha::ChaCha8;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::{Mutex, OnceLock};

fn entropy() -> u64 {
    RandomState::new().build_hasher().finish()
}

fn rng() -> &'static Mutex<ChaCha8> {
    static RNG: OnceLock<Mutex<ChaCha8>> = OnceLock::new();
    RNG.get_or_init(|| Mutex::new(ChaCha8::from_u64(entropy())))
}

/// Reseeds the global stream so every draw after it replays.
pub fn seed(s: u64) {
    *rng().lock().unwrap() = ChaCha8::from_u64(s);
}

/// Draws a float at or above zero and below one from the global stream.
pub fn random() -> f64 {
    rng().lock().unwrap().unit()
}

/// Draws a fair coin flip from the global stream.
pub fn boolean() -> bool {
    rng().lock().unwrap().boolean()
}

/// Draws an integer between a and b inclusive from the global stream.
pub fn randint(a: i64, b: i64) -> i64 {
    rng().lock().unwrap().range_i64(a, b)
}

/// Draws one element of the slice from the global stream.
pub fn choice<T: Clone>(seq: &[T]) -> T {
    let i = rng().lock().unwrap().below_u64(seq.len() as u64) as usize;
    seq[i].clone()
}

/// Shuffles the slice in place with the global stream.
pub fn shuffle<T>(seq: &mut [T]) {
    rng().lock().unwrap().shuffle(seq);
}

/// Draws k distinct elements of the slice from the global stream.
pub fn sample<T: Clone>(seq: &[T], k: usize) -> Vec<T> {
    let k = k.min(seq.len());
    let indices = rng().lock().unwrap().sample_indices(seq.len(), k);
    indices.into_iter().map(|i| seq[i].clone()).collect()
}

/// Locks the global stream so one seeded test runs at a time.
#[cfg(any(test, feature = "testkit"))]
pub fn guard() -> std::sync::MutexGuard<'static, ()> {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn seeded_is_reproducible() {
        let _g = guard();
        seed(42);
        let a: Vec<i64> = (0..10).map(|_| randint(0, 100)).collect();
        seed(42);
        let b: Vec<i64> = (0..10).map(|_| randint(0, 100)).collect();
        assert_eq!(a, b);
    }
    #[test]
    fn randint_is_inclusive() {
        let _g = guard();
        seed(7);
        for _ in 0..100 {
            let v = randint(1, 3);
            assert!((1..=3).contains(&v));
        }
    }
}
