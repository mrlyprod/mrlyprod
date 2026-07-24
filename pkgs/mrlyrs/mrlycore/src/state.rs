use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::sync::{Mutex, OnceLock};

fn rng() -> &'static Mutex<ChaCha8Rng> {
    static RNG: OnceLock<Mutex<ChaCha8Rng>> = OnceLock::new();
    RNG.get_or_init(|| Mutex::new(ChaCha8Rng::from_entropy()))
}

pub fn seed(s: u64) {
    *rng().lock().unwrap() = ChaCha8Rng::seed_from_u64(s);
}

pub fn random() -> f64 {
    rng().lock().unwrap().gen()
}

pub fn boolean() -> bool {
    rng().lock().unwrap().gen()
}

pub fn randint(a: i64, b: i64) -> i64 {
    rng().lock().unwrap().gen_range(a..=b)
}

pub fn choice<T: Clone>(seq: &[T]) -> T {
    let i = rng().lock().unwrap().gen_range(0..seq.len());
    seq[i].clone()
}

pub fn shuffle<T>(seq: &mut [T]) {
    seq.shuffle(&mut *rng().lock().unwrap());
}

pub fn sample<T: Clone>(seq: &[T], k: usize) -> Vec<T> {
    let mut guard = rng().lock().unwrap();
    seq.choose_multiple(&mut *guard, k).cloned().collect()
}

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
