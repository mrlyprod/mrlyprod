use super::field::Field;
use super::layer::{layer, Layer};
use super::{Lattice, Spec};
use crate::core::errors::Result;
use crate::core::fft::magnitude_spectrum;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Peak {
    pub dy: i64,
    pub dx: i64,
    pub angle: f64,
    pub magnitude: f64,
}

#[derive(Clone, Debug)]
pub struct Metrics {
    pub mean: f64,
    pub diagonal: f64,
    pub anti_diagonal: f64,
    pub zero_fraction: f64,
    pub peaks: Vec<Peak>,
}

pub fn metrics(field: &Field, num_peaks: usize) -> Metrics {
    let n = field.size;
    let mean = field.mean();
    let mut diag = 0.0f64;
    let mut anti = 0.0f64;
    let mut zeros = 0usize;
    for i in 0..n {
        diag += field.data[i * n + i] as f64;
        anti += field.data[i * n + (n - 1 - i)] as f64;
    }
    for &v in &field.data {
        if v == 0.0 {
            zeros += 1;
        }
    }
    let diag = if n > 0 { diag / n as f64 } else { 0.0 };
    let anti = if n > 0 { anti / n as f64 } else { 0.0 };
    let zero_fraction = if !field.data.is_empty() {
        zeros as f64 / field.data.len() as f64
    } else {
        0.0
    };
    let mut peaks = Vec::new();
    if n.is_power_of_two() && n >= 4 {
        let centered: Vec<f64> = field.data.iter().map(|&v| v as f64 - mean).collect();
        let spec = magnitude_spectrum(&centered, n);
        let c = (n / 2) as i64;
        let mut mag = spec.clone();
        for dy in -2..=2i64 {
            for dx in -2..=2i64 {
                let y = c + dy;
                let x = c + dx;
                if y >= 0 && y < n as i64 && x >= 0 && x < n as i64 {
                    mag[y as usize * n + x as usize] = 0.0;
                }
            }
        }
        let mut order: Vec<usize> = (0..mag.len()).collect();
        order.sort_by(|&a, &b| mag[b].partial_cmp(&mag[a]).unwrap());
        for &flat in order.iter().take(num_peaks) {
            let py = (flat / n) as i64;
            let px = (flat % n) as i64;
            let (dy, dx) = (py - c, px - c);
            let mut angle = (dy as f64).atan2(dx as f64).to_degrees().rem_euclid(180.0);
            if angle == 180.0 {
                angle = 0.0;
            }
            peaks.push(Peak {
                dy,
                dx,
                angle: (angle * 10.0).round() / 10.0,
                magnitude: spec[flat],
            });
        }
    }
    Metrics {
        mean,
        diagonal: diag,
        anti_diagonal: anti,
        zero_fraction,
        peaks,
    }
}

#[derive(Clone, Debug)]
pub struct GcdCorrelation {
    pub buckets: Vec<(usize, usize, f64)>,
    pub pearson: f64,
}

pub fn corr_by_gcd(spec: Spec, numbers: &[usize], size: usize) -> Result<GcdCorrelation> {
    let mut layers: Vec<Vec<f32>> = Vec::with_capacity(numbers.len());
    for &n in numbers {
        let params = Layer {
            spec,
            number: n,
            level: 1,
            lattice: Lattice::Square,
            size,
            slices: Vec::new(),
        };
        let m = layer(&params)?;
        layers.push(m.iter().map(|&b| b as u8 as f32).collect());
    }
    let mut bins: std::collections::BTreeMap<usize, Vec<f64>> = std::collections::BTreeMap::new();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            let r = pearson(&layers[i], &layers[j]);
            let (a, b) = (numbers[i], numbers[j]);
            let g = gcd(a, b);
            let key = if g == 1 {
                1
            } else if g < 5 {
                2
            } else {
                5
            };
            bins.entry(key).or_default().push(r);
            xs.push((g * g) as f64 / (a * b) as f64);
            ys.push(r);
        }
    }
    let buckets = bins
        .into_iter()
        .map(|(k, v)| {
            let mean = v.iter().sum::<f64>() / v.len() as f64;
            (k, v.len(), mean)
        })
        .collect();
    let pear = pearson_f64(&xs, &ys);
    Ok(GcdCorrelation {
        buckets,
        pearson: pear,
    })
}

fn pearson(a: &[f32], b: &[f32]) -> f64 {
    let a: Vec<f64> = a.iter().map(|&v| v as f64).collect();
    let b: Vec<f64> = b.iter().map(|&v| v as f64).collect();
    pearson_f64(&a, &b)
}

fn pearson_f64(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let ma = a.iter().sum::<f64>() / n;
    let mb = b.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut da = 0.0;
    let mut db = 0.0;
    for (&x, &y) in a.iter().zip(b.iter()) {
        let (dx, dy) = (x - ma, y - mb);
        num += dx * dy;
        da += dx * dx;
        db += dy * dy;
    }
    let den = (da * db).sqrt();
    if den == 0.0 {
        0.0
    } else {
        num / den
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metrics_basic_fields() {
        use crate::math::moire::{stack, Combine};
        let f = stack(
            Spec::new(7, 2, 2),
            &[1, 3, 5, 7],
            Combine::Sum,
            1,
            Lattice::Square,
            64,
            &[],
        )
        .unwrap();
        let m = metrics(&f, 5);
        assert!(m.mean > 0.0);
        assert_eq!(m.peaks.len(), 5);
        assert!(m.zero_fraction >= 0.0 && m.zero_fraction <= 1.0);
    }
    #[test]
    fn gcd_correlation_is_positive_for_a_fractal() {
        let numbers: Vec<usize> = (2..24).collect();
        let g = corr_by_gcd(Spec::new(7, 2, 2), &numbers, 64).unwrap();
        assert!(!g.buckets.is_empty());
        assert!(g.pearson > 0.0, "pearson {}", g.pearson);
    }
}
