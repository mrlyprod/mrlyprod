use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlymath::two;
use std::f64::consts::TAU;

const CODE: u128 = 495;
const SIDE: usize = 3;
const LEVEL: usize = 5;
const SPAN: usize = 243;
const CELL: f64 = 3.0;
const FLOOR: f64 = 0.09;
const REACH: f64 = 0.60;

fn digits() -> Result<Vec<(usize, usize)>> {
    let tile = two::designs::create(CODE, SIDE, 1, 0, SIDE)?;
    let types = tile.types();
    Ok((0..SIDE)
        .flat_map(|a| (0..SIDE).map(move |b| (a, b)))
        .filter(|&(a, b)| types.get(&[a, b]) != 0)
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
    let reads: Vec<f64> = ratios
        .iter()
        .map(|v| ((v.powf(1.0 / LEVEL as f64) - FLOOR) / (REACH - FLOOR)).clamp(0.0, 1.0))
        .collect();

    let ramp = Ramp::new(vec![ink::ground(), ink::blue(), ink::yellow()]);
    let mut board = Board::square();
    let plate = CELL * SPAN as f64;
    let edge = ((board.width as f64 - plate) / 2.0).floor();
    let frame = Frame::new(edge, edge, plate, plate);
    field::draw_range(&mut board, frame, SPAN, SPAN, &reads, (0.0, 1.0), &ramp);
    save("demo-modes", &board)?;
    Ok(())
}
