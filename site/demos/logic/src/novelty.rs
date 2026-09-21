use crate::Fault;
use mrlyrs::num::lattice::totients;
use mrlyrs::num::zeta::{
    novelty_main, novelty_wave, sharp_novelty, smoothed_novelty, Complex, Line,
};
use wasm_bindgen::prelude::*;

const LOW: f64 = 8.0;
const HIGHEST: f64 = 21.0;
const PER_OCTAVE: usize = 32;
const ZEROS: usize = 138;

/// The novelty meter: the totients sieved once, the smoothed and the sharp novelty error read on a log grid of y, and the first zeros with the coefficients of their waves.
#[wasm_bindgen]
pub struct Novelty {
    heights: Vec<f64>,
    smooth: Vec<f64>,
    rough: Vec<f64>,
    gammas: Vec<f64>,
    coef: Vec<Complex>,
    sieve: usize,
}

#[wasm_bindgen]
impl Novelty {
    /// Builds the meter on y = 2^-j for j from eight to the height at the given samples per octave, the totients sieved to two to the height plus one, with the given count of zeros, at most 138.
    #[wasm_bindgen(constructor)]
    pub fn new(high: f64, per_octave: usize, zeros: usize) -> Result<Novelty, Fault> {
        if !(LOW + 1.0..=HIGHEST).contains(&high) {
            return Err(Fault::new(format!(
                "the height runs from {} to {HIGHEST}.",
                LOW + 1.0
            )));
        }
        if !(1..=PER_OCTAVE).contains(&per_octave) {
            return Err(Fault::new(format!(
                "an octave takes 1 to {PER_OCTAVE} samples."
            )));
        }
        if zeros > ZEROS {
            return Err(Fault::new(format!("the meter holds {ZEROS} zeros.")));
        }
        let sieve = 2.0f64.powf(high + 1.0).ceil() as usize;
        let phi = totients(sieve);
        let mut prefix = vec![0u64; sieve + 1];
        for n in 1..=sieve {
            prefix[n] = prefix[n - 1] + phi[n];
        }
        let main = novelty_main();
        let samples = ((high - LOW) * per_octave as f64).round() as usize + 1;
        let heights: Vec<f64> = (0..samples)
            .map(|k| LOW + k as f64 / per_octave as f64)
            .collect();
        let smooth = heights
            .iter()
            .map(|&j| {
                let y = 2.0f64.powf(-j);
                smoothed_novelty(&phi, y, main) / y.powf(1.5)
            })
            .collect();
        let rough = heights
            .iter()
            .map(|&j| {
                let y = 2.0f64.powf(-j);
                sharp_novelty(&prefix, y) / y
            })
            .collect();
        let line = Line::new();
        let gammas = line.zeros(zeros);
        let coef = line.novelty_coefficients(&gammas);
        Ok(Novelty {
            heights,
            smooth,
            rough,
            gammas,
            coef,
            sieve,
        })
    }
    fn first(&self, count: usize) -> Result<usize, Fault> {
        if count > self.gammas.len() {
            return Err(Fault::new(format!(
                "the meter holds {} zeros.",
                self.gammas.len()
            )));
        }
        Ok(count)
    }
    /// Returns j at every sample, the log base two of one over y.
    pub fn heights(&self) -> Vec<f64> {
        self.heights.clone()
    }
    /// Returns the error at every sample: the smoothed error over y to the three halves, or the sharp window's error over y.
    pub fn dots(&self, sharp: bool) -> Vec<f64> {
        if sharp {
            self.rough.clone()
        } else {
            self.smooth.clone()
        }
    }
    /// Returns the ordinate of every zero the meter holds.
    pub fn gammas(&self) -> Vec<f64> {
        self.gammas.clone()
    }
    /// Returns the modulus of every zero's wave coefficient.
    pub fn amplitudes(&self) -> Vec<f64> {
        self.coef.iter().map(|c| c.abs()).collect()
    }
    /// Sums the waves of the first zeros at the given heights j, on the scale of the dots: over y to the three halves, or over y when sharp, where the waves shrink by the root of y.
    pub fn wave(&self, count: usize, sharp: bool, heights: &[f64]) -> Result<Vec<f64>, Fault> {
        let count = self.first(count)?;
        let log2 = 2.0f64.ln();
        Ok(heights
            .iter()
            .map(|&j| {
                let sum = novelty_wave(&self.gammas[..count], &self.coef[..count], -j * log2);
                if sharp {
                    sum * 2.0f64.powf(-0.5 * j)
                } else {
                    sum
                }
            })
            .collect())
    }
    /// Returns the largest gap between the smoothed dots and the wave of the first zeros, over the largest dot.
    pub fn miss(&self, count: usize) -> Result<f64, Fault> {
        let wave = self.wave(count, false, &self.heights)?;
        let peak = self.smooth.iter().fold(0.0f64, |a, v| a.max(v.abs()));
        let gap = self
            .smooth
            .iter()
            .zip(&wave)
            .map(|(d, w)| (d - w).abs())
            .fold(0.0f64, f64::max);
        Ok(gap / peak)
    }
    /// Returns the reach of the totient sieve.
    pub fn sieve(&self) -> usize {
        self.sieve
    }
}
