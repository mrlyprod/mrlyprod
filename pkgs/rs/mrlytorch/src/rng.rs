use crate::math;

const GOLDEN: u64 = 0x9E3779B97F4A7C15;

const MIX_A: u64 = 0xBF58476D1CE4E5B9;

const MIX_B: u64 = 0x94D049BB133111EB;

const UNIT_24: f32 = (1u64 << 24) as f32;

const UNIT_53: f64 = (1u64 << 53) as f64;

/// A seeded splitmix64 stream whose draws replay exactly on every run.
///
/// ```
/// use mrlytorch::rng::Rng;
/// let mut a = Rng::new(mrlytorch::seed(1, 0));
/// let mut b = Rng::new(mrlytorch::seed(1, 0));
/// assert_eq!(a.normal(), b.normal());
/// assert_eq!(a.unit(), b.unit());
/// ```
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Builds the stream from a seed.
    pub fn new(seed: u64) -> Rng {
        Rng { state: seed }
    }

    /// Draws the next raw word of the stream.
    pub fn next_word(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(MIX_A);
        z = (z ^ (z >> 27)).wrapping_mul(MIX_B);
        z ^ (z >> 31)
    }

    /// Draws a float at or above zero and below one.
    pub fn unit(&mut self) -> f32 {
        (self.next_word() >> 40) as f32 / UNIT_24
    }

    /// Draws a float at or above lo and below hi, or lo when hi is not above lo.
    pub fn uniform(&mut self, lo: f32, hi: f32) -> f32 {
        if hi <= lo {
            lo
        } else {
            lo + (hi - lo) * self.unit()
        }
    }

    /// Draws a standard normal through the Box-Muller transform.
    pub fn normal(&mut self) -> f32 {
        let u1 = 1.0 - (self.next_word() >> 11) as f64 / UNIT_53;
        let u2 = (self.next_word() >> 11) as f64 / UNIT_53;
        let radius = (-2.0 * math::ln(u1)).sqrt();
        (radius * math::cos(core::f64::consts::TAU * u2)) as f32
    }

    /// Draws an integer below n, or zero when n is zero.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_word() % n as u64) as usize
        }
    }

    /// Fills a slice with uniform draws between lo and hi.
    pub fn fill_uniform(&mut self, buf: &mut [f32], lo: f32, hi: f32) {
        for v in buf {
            *v = self.uniform(lo, hi);
        }
    }

    /// Fills a slice with normal draws scaled by sd around mean.
    pub fn fill_normal(&mut self, buf: &mut [f32], mean: f32, sd: f32) {
        for v in buf {
            *v = mean + sd * self.normal();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_seed_replays_every_draw() {
        let mut a = Rng::new(crate::seed(7, 3));
        let mut b = Rng::new(crate::seed(7, 3));
        let first: Vec<u64> = (0..8).map(|_| a.next_word()).collect();
        let again: Vec<u64> = (0..8).map(|_| b.next_word()).collect();
        assert_eq!(first, again);
        assert_eq!(a.normal(), b.normal());
        assert_eq!(a.uniform(-2.0, 2.0), b.uniform(-2.0, 2.0));
        assert_eq!(a.below(1000), b.below(1000));
    }

    #[test]
    fn different_seeds_split_the_streams() {
        let mut a = Rng::new(crate::seed(7, 3));
        let mut b = Rng::new(crate::seed(7, 4));
        let first: Vec<u64> = (0..8).map(|_| a.next_word()).collect();
        let other: Vec<u64> = (0..8).map(|_| b.next_word()).collect();
        assert_ne!(first, other);
    }

    #[test]
    fn unit_stays_in_the_half_open_interval() {
        let mut rng = Rng::new(crate::seed(1, 1));
        for _ in 0..10000 {
            let v = rng.unit();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn normal_moments_look_standard() {
        let mut rng = Rng::new(crate::seed(2, 2));
        let draws: Vec<f64> = (0..20000).map(|_| rng.normal() as f64).collect();
        let mean = draws.iter().sum::<f64>() / draws.len() as f64;
        let var = draws.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / draws.len() as f64;
        assert!(mean.abs() < 0.05, "mean {mean}");
        assert!((var - 1.0).abs() < 0.05, "var {var}");
    }

    #[test]
    fn fills_replay_and_respect_bounds() {
        let mut a = Rng::new(crate::seed(3, 0));
        let mut b = Rng::new(crate::seed(3, 0));
        let mut one = [0.0f32; 32];
        let mut two = [0.0f32; 32];
        a.fill_uniform(&mut one, -1.0, 1.0);
        b.fill_uniform(&mut two, -1.0, 1.0);
        assert_eq!(one, two);
        assert!(one.iter().all(|v| (-1.0..1.0).contains(v)));
        a.fill_normal(&mut one, 0.0, 0.5);
        b.fill_normal(&mut two, 0.0, 0.5);
        assert_eq!(one, two);
    }
}
