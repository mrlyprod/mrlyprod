use super::medium::{medium, Medium};
use super::stepper::run;
use super::F_SRC;
use crate::fft;
use crate::two::Cell2d;
use mrlycore::errors::{value_error, Result};

/// The read band's lower edge in cycles per step.
pub const BAND_LO: f64 = 0.015;
/// The read band's upper edge in cycles per step.
pub const BAND_HI: f64 = 0.12;
const SMOOTH: usize = 5;
const FLOOR: f64 = 0.02;
const SRC_COL: usize = 12;
const REC_GAP: usize = 10;

/// A transmission spectrum: the read-band frequencies and their smoothed ratios.
#[derive(Clone, Debug, PartialEq)]
pub struct Spectrum {
    /// The band frequencies in cycles per step.
    pub freqs: Vec<f64>,
    /// The smoothed transmission at each frequency.
    pub values: Vec<f64>,
}

/// Measures a mask's transmission: its corridor run over a free-space run of the same shape.
///
/// Both runs record the receiver column, transform in time, and divide magnitude
/// spectra where the reference is loud enough; the read band then smooths over
/// five bins. Steps must be a power of two for the transform.
pub fn transmission(
    mask: &Cell2d,
    slab_px: usize,
    pad: usize,
    tail: usize,
    steps: usize,
) -> Result<Spectrum> {
    if !steps.is_power_of_two() || steps < 4 {
        return value_error(format!(
            "steps {steps} must be a power of two of at least 4."
        ));
    }
    let m = medium(mask, slab_px, pad, tail);
    if m.height < 3 {
        return value_error(format!("corridor height {} must be at least 3.", m.height));
    }
    if m.width <= SRC_COL + REC_GAP {
        return value_error(format!(
            "corridor width {} must exceed {}.",
            m.width,
            SRC_COL + REC_GAP
        ));
    }
    let rec_col = m.width - REC_GAP;
    let sig = run(&m, steps, SRC_COL, rec_col, F_SRC);
    let free = Medium::free(m.height, m.width);
    let ref_sig = run(&free, steps, SRC_COL, rec_col, F_SRC);
    let s = magnitudes(&sig);
    let r = magnitudes(&ref_sig);
    let peak = r.iter().cloned().fold(0.0f64, f64::max);
    let mut freqs = Vec::new();
    let mut values = Vec::new();
    for k in 0..s.len() {
        let f = k as f64 / steps as f64;
        if f > BAND_LO && f < BAND_HI {
            freqs.push(f);
            values.push(if r[k] > FLOOR * peak {
                s[k] / r[k]
            } else {
                f64::NAN
            });
        }
    }
    let values = smooth(&values, SMOOTH);
    Ok(Spectrum { freqs, values })
}

fn magnitudes(signal: &[f64]) -> Vec<f64> {
    let mut re = signal.to_vec();
    let mut im = vec![0.0; signal.len()];
    fft::fft(&mut re, &mut im, false);
    (0..=signal.len() / 2)
        .map(|k| (re[k] * re[k] + im[k] * im[k]).sqrt())
        .collect()
}

fn bridge(values: &[f64]) -> Vec<f64> {
    let good: Vec<usize> = (0..values.len()).filter(|&i| !values[i].is_nan()).collect();
    if good.is_empty() || good.len() == values.len() {
        return values.to_vec();
    }
    let mut out = values.to_vec();
    for (i, slot) in out.iter_mut().enumerate() {
        if !slot.is_nan() {
            continue;
        }
        *slot = match (
            good.iter().rev().find(|&&g| g < i),
            good.iter().find(|&&g| g > i),
        ) {
            (Some(&a), Some(&b)) => {
                let t = (i - a) as f64 / (b - a) as f64;
                values[a] + (values[b] - values[a]) * t
            }
            (None, Some(&b)) => values[b],
            (Some(&a), None) => values[a],
            (None, None) => unreachable!(),
        };
    }
    out
}

fn smooth(values: &[f64], window: usize) -> Vec<f64> {
    let filled = bridge(values);
    let half = window / 2;
    let n = filled.len();
    let mut out = vec![0.0; n];
    for (i, slot) in out.iter_mut().enumerate() {
        let lo = i.saturating_sub(half);
        let hi = (i + half).min(n.saturating_sub(1));
        *slot = filled[lo..=hi].iter().sum::<f64>() / window as f64;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two;

    #[test]
    fn free_space_transmits_one_across_the_band() {
        let mask = two::zeros(3, 1).unwrap();
        let spec = transmission(&mask, 6, 16, 16, 256).unwrap();
        assert_eq!(spec.freqs.len(), 27);
        for &v in &spec.values[2..25] {
            assert!((v - 1.0).abs() < 1e-9, "in-band value {v}");
        }
    }

    #[test]
    fn two_transmissions_agree_bit_for_bit() {
        let mask = two::carpet(3, 1).unwrap();
        let a = transmission(&mask, 6, 16, 16, 256).unwrap();
        let b = transmission(&mask, 6, 16, 16, 256).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn a_tiny_carpet_run_stays_pinned() {
        let mask = two::carpet(3, 1).unwrap();
        let spec = transmission(&mask, 6, 16, 16, 256).unwrap();
        assert_eq!(spec.freqs.len(), 27);
        assert!((spec.freqs[0] - 0.015625).abs() < 1e-12);
        assert!((spec.values[0] - 0.892539818537432).abs() < 1e-9);
        assert!((spec.values[13] - 0.171818151693466).abs() < 1e-9);
        assert!((spec.values[26] - 0.501141456134949).abs() < 1e-9);
        let sum: f64 = spec.values.iter().sum();
        assert!(
            (sum - 15.963663754856993).abs() < 1e-9,
            "pinned sum drifted to {sum}"
        );
    }

    #[test]
    fn bad_step_counts_are_refused() {
        let mask = two::zeros(3, 1).unwrap();
        assert!(transmission(&mask, 6, 16, 16, 200).is_err());
        assert!(transmission(&mask, 6, 4, 4, 256).is_err());
    }

    #[test]
    fn smoothing_bridges_gaps_and_averages() {
        let curve = [1.0, f64::NAN, 3.0, 3.0, 3.0];
        let smoothed = smooth(&curve, 5);
        assert!((smoothed[2] - (1.0 + 2.0 + 3.0 + 3.0 + 3.0) / 5.0).abs() < 1e-12);
        assert!((smoothed[0] - (1.0 + 2.0 + 3.0) / 5.0).abs() < 1e-12);
    }
}
