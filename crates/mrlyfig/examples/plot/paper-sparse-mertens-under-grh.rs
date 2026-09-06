use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Ramp};

const BASE: i64 = 7;
const DROP: i64 = 3;
const LEVEL: u32 = 3;

fn kept() -> Vec<i64> {
    (0..BASE).filter(|d| *d != DROP).collect()
}

fn symbol(kept: &[i64], t: f64) -> f64 {
    let (mut re, mut im) = (0.0, 0.0);
    for d in kept {
        let angle = std::f64::consts::TAU * (*d as f64) * t;
        re += angle.cos();
        im += angle.sin();
    }
    re.hypot(im)
}

fn transform(kept: &[i64], a: i64, span: i64) -> f64 {
    let mut mass = 1.0;
    let mut scale = 1i64;
    for _ in 0..LEVEL {
        let t = (a * scale).rem_euclid(span) as f64 / span as f64;
        mass *= symbol(kept, t);
        scale *= BASE;
    }
    mass
}

fn proved() -> f64 {
    let q = BASE as f64;
    let n = ((BASE - 2) as f64 / 2.0).ceil();
    let harmonic = n.ln() + 0.577_215_664_901_532_9 + 1.0 / (2.0 * n);
    let pi = std::f64::consts::PI;
    let phi = (4.0 / pi) * q + (2.0 * q / pi) * harmonic + (1.0 - 2.0 / pi) * (q - 2.0) + 0.727;
    1.0 + phi / q
}

fn main() -> Result<()> {
    let kept = kept();
    let span = BASE.pow(LEVEL);
    let peak = (kept.len() as f64).powi(LEVEL as i32);
    let half = (span - 1) / 2;
    let mass: Vec<f64> = (-half..=half).map(|a| transform(&kept, a, span)).collect();

    assert_eq!(mass.len(), span as usize);
    assert_eq!(kept.len() as i64, BASE - 1);
    assert!((mass[half as usize] - peak).abs() < 1e-9);
    let energy: f64 = mass.iter().map(|v| v * v).sum();
    assert!((energy - span as f64 * peak).abs() < 1e-6 * span as f64 * peak);
    let total: f64 = mass.iter().sum();
    assert!(total < (BASE as f64 * proved()).powi(LEVEL as i32));
    assert!(total >= span as f64);

    let mut board = Board::square();
    let frame = board.frame(0.07);
    let ramp = Ramp::tone(ink::BLUE, ink::GOLD);
    let slot = frame.w / mass.len() as f64;
    let pad = slot * 0.18;
    let axis = frame.y + frame.h / 2.0;
    for (i, value) in mass.iter().enumerate() {
        let reach = frame.h / 2.0 * (value / peak).powf(1.0 / LEVEL as f64);
        board.rect(
            frame.x + i as f64 * slot + pad,
            axis - reach,
            slot - 2.0 * pad,
            2.0 * reach,
            ramp.at(2.0 * reach / frame.h),
        );
    }
    save("paper-sparse-mertens-under-grh", &board)?;
    Ok(())
}
