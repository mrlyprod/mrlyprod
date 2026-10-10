use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use std::f64::consts::PI;

const NAME: &str = "wiki-rajchman-measure";
const TOP: usize = 243;
const DEPTH: u32 = 40;
const WIDTH: f64 = 0.1;
const STEPS: usize = 4096;

fn trim(v: f64) -> f64 {
    (v * 1e9).round() / 1e9 + 0.0
}

fn cantor(t: f64) -> f64 {
    (1..=DEPTH)
        .map(|k| (2.0 * PI * t / 3f64.powi(k as i32)).cos().abs())
        .product()
}

fn bump(n: f64) -> f64 {
    let mut re = 0.0;
    let mut im = 0.0;
    let mut mass = 0.0;
    for i in 0..STEPS {
        let u = (i as f64 + 0.5) / STEPS as f64;
        let w = (-1.0 / (u * (1.0 - u))).exp();
        let x = 0.5 + WIDTH * (u - 0.5);
        re += w * (2.0 * PI * n * x).cos();
        im += w * (2.0 * PI * n * x).sin();
        mass += w;
    }
    re.hypot(im) / mass
}

fn main() -> Result<()> {
    let cantors: Vec<f64> = (1..=TOP).map(|n| cantor(n as f64)).collect();
    let bumps: Vec<f64> = (1..=TOP).map(|n| bump(n as f64)).collect();
    let powers: Vec<usize> = (0..)
        .map(|k| 3usize.pow(k))
        .take_while(|&p| p <= TOP)
        .collect();
    let ceiling = cantors[0];
    for &p in &powers {
        assert!((cantors[p - 1] - ceiling).abs() < 1e-9);
    }
    assert_eq!(powers.len(), 6);
    assert!(cantors.iter().all(|&v| v <= ceiling + 1e-9));
    assert!(bumps[0] > 0.9);
    assert!(bumps[40..].iter().all(|&v| v < 0.01));
    let cantors: Vec<f64> = cantors.into_iter().map(trim).collect();
    let bumps: Vec<f64> = bumps.into_iter().map(trim).collect();
    save(
        NAME,
        &json!({"cantor": cantors, "bump": bumps, "powers": powers}),
    )
}
