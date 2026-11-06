use crate::{CYCLE, MOST, RATE, Result, Wave, samples};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Timbre {
    pub shape: Wave,
    pub harmonics: usize,
}

impl Default for Timbre {
    fn default() -> Timbre {
        Timbre::new(Wave::Sine, 1)
    }
}

impl Timbre {
    pub fn new(shape: Wave, harmonics: usize) -> Timbre {
        Timbre { shape, harmonics }
    }
    pub fn partials(&self, hz: f64) -> Vec<(f64, f64)> {
        self.shape
            .recipe(self.harmonics)
            .into_iter()
            .filter(|&(multiple, _)| 2.0 * (hz * multiple).abs() < RATE as f64)
            .collect()
    }
}

pub fn tone(hz: f64, timbre: &Timbre, seconds: f64) -> Result<Vec<f32>> {
    let count = samples(seconds);
    if count > MOST {
        return Err(format!("a tone holds at most {MOST} samples."));
    }
    let mut out = vec![0.0f64; count];
    for (multiple, weight) in timbre.partials(hz) {
        let step = hz * multiple / RATE as f64;
        for (i, s) in out.iter_mut().enumerate() {
            *s += weight * Wave::Sine.sample(i as f64 * step);
        }
    }
    Ok(unit(out))
}

pub fn cycle(timbre: &Timbre, hz: f64) -> Vec<f32> {
    let mut out = vec![0.0f64; CYCLE];
    for (multiple, weight) in timbre.partials(hz) {
        for (i, s) in out.iter_mut().enumerate() {
            *s += weight * Wave::Sine.sample(multiple * i as f64 / CYCLE as f64);
        }
    }
    unit(out)
}

fn unit(sum: Vec<f64>) -> Vec<f32> {
    let top = sum.iter().fold(0.0f64, |m, s| m.max(s.abs()));
    let k = if top > 0.0 { 1.0 / top } else { 0.0 };
    sum.into_iter().map(|s| (s * k) as f32).collect()
}
