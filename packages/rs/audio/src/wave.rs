use crate::CYCLE;
use mrly_name::named_enum;
use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_1_PI, TAU};

named_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Wave {
        #[default]
        Sine => "sine",
        Triangle => "triangle",
        Square => "square",
        Sawtooth => "sawtooth",
    }
}

impl Wave {
    pub fn sample(&self, phase: f64) -> f64 {
        let t = phase - phase.floor();
        match self {
            Wave::Sine => sine(t),
            Wave::Triangle => 1.0 - 4.0 * (t - 0.25 - (t + 0.25).floor()).abs(),
            Wave::Square if t == 0.0 || t == 0.5 => 0.0,
            Wave::Square if t < 0.5 => 1.0,
            Wave::Square => -1.0,
            Wave::Sawtooth => 2.0 * (t - (t + 0.5).floor()),
        }
    }
    pub fn recipe(&self, count: usize) -> Vec<(f64, f64)> {
        let count = count.min(CYCLE);
        match self {
            Wave::Sine => vec![(1.0, 1.0)],
            Wave::Triangle => odd(count)
                .enumerate()
                .map(|(i, n)| (n, alternate(i) * 8.0 * FRAC_1_PI * FRAC_1_PI / (n * n)))
                .collect(),
            Wave::Square => odd(count).map(|n| (n, 4.0 * FRAC_1_PI / n)).collect(),
            Wave::Sawtooth => (0..count)
                .map(|i| {
                    let n = (i + 1) as f64;
                    (n, alternate(i) * 2.0 * FRAC_1_PI / n)
                })
                .collect(),
        }
    }
}

fn alternate(i: usize) -> f64 {
    if i.is_multiple_of(2) { 1.0 } else { -1.0 }
}

fn odd(count: usize) -> impl Iterator<Item = f64> {
    (0..count).map(|i| (2 * i + 1) as f64)
}

fn sine(t: f64) -> f64 {
    let mut t = if t > 0.5 { t - 1.0 } else { t };
    if t > 0.25 {
        t = 0.5 - t;
    } else if t < -0.25 {
        t = -0.5 - t;
    }
    let x = TAU * t;
    let x2 = x * x;
    x * TAYLOR.iter().rev().fold(0.0, |sum, c| c + x2 * sum)
}

const TAYLOR: [f64; 8] = [
    1.0,
    -1.0 / 6.0,
    1.0 / 120.0,
    -1.0 / 5040.0,
    1.0 / 362_880.0,
    -1.0 / 39_916_800.0,
    1.0 / 6_227_020_800.0,
    -1.0 / 1_307_674_368_000.0,
];
