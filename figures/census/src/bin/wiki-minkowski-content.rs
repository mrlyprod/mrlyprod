use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::{json, Json};

const NAME: &str = "wiki-minkowski-content";
const ROWS: usize = 7;
const WIDEST: f64 = 0.1;
const THREAD: usize = 5;
const FROM: f64 = 2.0;
const TO: f64 = 6.0;
const SAMPLES: usize = 4000;
const LOW: f64 = 2.494975716;

fn intervals(level: usize) -> Vec<(f64, f64)> {
    let side = 3f64.powi(-(level as i32));
    (0..1usize << level)
        .map(|i| {
            let a: f64 = (0..level)
                .filter(|&l| (i >> (level - 1 - l)) & 1 == 1)
                .map(|l| 2.0 * 3f64.powi(-(l as i32 + 1)))
                .sum();
            (a, a + side)
        })
        .collect()
}

fn tube(eps: f64) -> Vec<(f64, f64)> {
    let level = (0..40)
        .find(|&k| 3f64.powi(-(k as i32 + 1)) <= 2.0 * eps)
        .unwrap_or(40);
    let mut out: Vec<(f64, f64)> = Vec::new();
    for (a, b) in intervals(level) {
        let (a, b) = (a - eps, b + eps);
        match out.last_mut() {
            Some(last) if a <= last.1 => last.1 = last.1.max(b),
            _ => out.push((a, b)),
        }
    }
    out
}

fn volume(eps: f64) -> f64 {
    2.0 * eps
        + (1..80)
            .map(|k| 2f64.powi(k - 1) * 3f64.powi(-k).min(2.0 * eps))
            .sum::<f64>()
}

fn reading(eps: f64, dimension: f64) -> f64 {
    eps.powf(dimension - 1.0) * volume(eps)
}

fn pairs(spans: &[(f64, f64)]) -> Json {
    json!(spans.iter().map(|&(a, b)| [a, b]).collect::<Vec<_>>())
}

fn main() -> Result<()> {
    let dimension = 2f64.ln() / 3f64.ln();
    let thread = intervals(THREAD);
    let mut tubes = Vec::new();
    for row in 0..ROWS {
        let eps = WIDEST * 3f64.powf(-(row as f64) / 2.0);
        let tube = tube(eps);
        let length: f64 = tube.iter().map(|(a, b)| b - a).sum();
        assert!((length - volume(eps)).abs() < 1e-12);
        tubes.push(pairs(&tube));
    }

    let xs: Vec<f64> = (0..=SAMPLES)
        .map(|i| FROM + (TO - FROM) * i as f64 / SAMPLES as f64)
        .collect();
    let ys: Vec<f64> = xs
        .iter()
        .map(|&x| reading(3f64.powf(-x), dimension))
        .collect();
    let high = 2f64.powf(2.0 - dimension);
    let lo = ys.iter().copied().fold(f64::MAX, f64::min);
    let hi = ys.iter().copied().fold(f64::MIN, f64::max);
    assert!((lo - LOW).abs() < 1e-5);
    assert!(hi <= high + 1e-9);
    assert!((reading(1.0 / 18.0, dimension) - high).abs() < 1e-9);
    for k in 2..=6 {
        assert!((reading(3f64.powi(-k), dimension) - 2.5).abs() < 1e-9);
    }
    for &x in xs.iter().step_by(97) {
        let eps = 3f64.powf(-x);
        assert!((reading(eps / 3.0, dimension) - reading(eps, dimension)).abs() < 1e-9);
    }

    save(
        NAME,
        &json!({"tubes": tubes, "thread": pairs(&thread), "ys": ys, "low": LOW, "high": high}),
    )
}
