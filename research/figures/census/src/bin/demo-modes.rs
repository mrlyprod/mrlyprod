use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use mrlyrs::math::bang::Code;
use mrlyrs::math::two;
use std::f64::consts::TAU;

const NAME: &str = "demo-modes";
const CODE: u128 = 495;
const SIDE: usize = 3;
const LEVEL: usize = 5;
const SPAN: usize = 243;
const FLOOR: f64 = 0.09;
const REACH: f64 = 0.60;

fn digits() -> Result<Vec<(usize, usize)>> {
    let tile = two::designs::create(Code::from(CODE), SIDE, 1, 0, SIDE)?;
    let types = tile.types();
    Ok((0..SIDE)
        .flat_map(|a| (0..SIDE).map(move |b| (a, b)))
        .filter(|&(a, b)| types.get(&[a, b]).is_ok_and(|v| v != 0))
        .collect())
}

fn modulus(digits: &[(usize, usize)], t1: usize, t2: usize) -> f64 {
    let (mut re, mut im) = (1.0, 0.0);
    let mut scale = 1usize;
    for _ in 0..LEVEL {
        let (mut fr, mut fi) = (0.0, 0.0);
        for &(a, b) in digits {
            let angle = TAU * (((a * t1 + b * t2) % SPAN) * scale % SPAN) as f64 / SPAN as f64;
            fr += angle.cos();
            fi += angle.sin();
        }
        let next = (re * fr - im * fi, re * fi + im * fr);
        re = next.0;
        im = next.1;
        scale = scale * SIDE % SPAN;
    }
    re.hypot(im)
}

fn read(ratio: f64) -> f64 {
    ((ratio.powf(1.0 / LEVEL as f64) - FLOOR) / (REACH - FLOOR)).clamp(0.0, 1.0)
}

fn main() -> Result<()> {
    let digits = digits()?;
    assert_eq!(digits.len(), 8);
    let mass = (digits.len() as f64).powi(LEVEL as i32);
    assert_eq!(mass, 32768.0);
    let half = SPAN / 2;
    let mut ratios = Vec::with_capacity(SPAN * SPAN);
    for row in 0..SPAN {
        for col in 0..SPAN {
            let t1 = (row + SPAN - half) % SPAN;
            let t2 = (col + SPAN - half) % SPAN;
            ratios.push(modulus(&digits, t1, t2) / mass);
        }
    }
    assert_eq!(ratios.len(), 59049);
    assert!((ratios[half * SPAN + half] - 1.0).abs() < 1e-9);
    let mut sorted = ratios.clone();
    sorted.sort_by(|a, b| b.partial_cmp(a).expect("a field of finite ratios"));
    assert!((sorted[1] - 0.125).abs() < 1e-9);
    assert_eq!(ratios.iter().filter(|&&v| v >= 0.1).count(), 25);

    let fold = |row: usize, col: usize| {
        let (a, b) = (row.abs_diff(half), col.abs_diff(half));
        let (hi, lo) = (a.max(b), a.min(b));
        hi * (hi + 1) / 2 + lo
    };
    let mut reads = vec![0.0; (half + 1) * (half + 2) / 2];
    for hi in 0..=half {
        for lo in 0..=hi {
            reads[hi * (hi + 1) / 2 + lo] = read(modulus(&digits, hi, lo) / mass);
        }
    }
    for row in 0..SPAN {
        for col in 0..SPAN {
            let full = read(ratios[row * SPAN + col]);
            assert!((full - reads[fold(row, col)]).abs() < 1e-9);
        }
    }
    let reads: Vec<f64> = reads.iter().map(|v| (v * 1e9).round() / 1e9).collect();
    save(NAME, &json!({"reads": reads}))
}
