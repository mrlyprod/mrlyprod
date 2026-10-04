use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use std::f64::consts::TAU;

const NAME: &str = "wiki-mahler-measure";
const LEHMER: [f64; 11] = [1.0, 1.0, 0.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0];
const SALEM: f64 = 1.176_280_818_259_917_5;
const SAMPLES: usize = 4096;
const FINE: usize = 1 << 16;

type Z = (f64, f64);

fn mul(a: Z, b: Z) -> Z {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn div(a: Z, b: Z) -> Z {
    let d = b.0 * b.0 + b.1 * b.1;
    ((a.0 * b.0 + a.1 * b.1) / d, (a.1 * b.0 - a.0 * b.1) / d)
}

fn norm(z: Z) -> f64 {
    z.0.hypot(z.1)
}

fn eval(z: Z) -> Z {
    LEHMER.iter().rev().fold((0.0, 0.0), |acc, c| {
        let m = mul(acc, z);
        (m.0 + c, m.1)
    })
}

fn height(t: f64) -> f64 {
    norm(eval((t.cos(), t.sin()))).ln()
}

fn roots() -> Vec<Z> {
    let degree = LEHMER.len() - 1;
    let seed = (0.4, 0.9);
    let mut zs: Vec<Z> = Vec::with_capacity(degree);
    let mut z = (1.0, 0.0);
    for _ in 0..degree {
        zs.push(z);
        z = mul(z, seed);
    }
    for _ in 0..500 {
        for k in 0..degree {
            let mut den = (1.0, 0.0);
            for j in 0..degree {
                if j != k {
                    den = mul(den, (zs[k].0 - zs[j].0, zs[k].1 - zs[j].1));
                }
            }
            let step = div(eval(zs[k]), den);
            zs[k] = (zs[k].0 - step.0, zs[k].1 - step.1);
        }
    }
    zs
}

fn pairs(zs: &[Z]) -> Vec<[f64; 2]> {
    zs.iter().map(|z| [z.0, z.1]).collect()
}

fn main() -> Result<()> {
    let zs = roots();
    let on: Vec<Z> = zs
        .iter()
        .copied()
        .filter(|z| (norm(*z) - 1.0).abs() < 1e-9)
        .collect();
    let off: Vec<Z> = zs
        .iter()
        .copied()
        .filter(|z| (norm(*z) - 1.0).abs() >= 1e-9)
        .collect();
    assert_eq!(on.len(), 8);
    assert_eq!(off.len(), 2);
    let outside: f64 = off.iter().map(|z| norm(*z).max(1.0)).product();
    assert!((outside - SALEM).abs() < 1e-12);
    assert!(off.iter().all(|z| z.1.abs() < 1e-12 && z.0 > 0.0));
    let mean = (0..FINE)
        .map(|i| height((i as f64 + 0.5) / FINE as f64 * TAU))
        .sum::<f64>()
        / FINE as f64;
    assert!((mean - SALEM.ln()).abs() < 1e-4);

    let heights: Vec<f64> = (0..SAMPLES)
        .map(|i| height(i as f64 / SAMPLES as f64 * TAU))
        .collect();
    let on_heights: Vec<f64> = on
        .iter()
        .map(|z| height(z.1.atan2(z.0).rem_euclid(TAU)))
        .collect();
    save(
        NAME,
        &json!({
            "heights": heights,
            "on": pairs(&on),
            "on_heights": on_heights,
            "off": pairs(&off),
        }),
    )
}
