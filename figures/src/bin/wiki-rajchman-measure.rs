use figures::{ink, plot, save, Board, Frame};
use mrlyrs::core::error::Result;
use std::f64::consts::PI;

const TOP: usize = 243;
const DEPTH: u32 = 40;
const WIDTH: f64 = 0.1;
const STEPS: usize = 4096;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let gap = frame.h * 0.08;
    let half = (frame.h - gap) / 2.0;
    let upper = Frame::new(frame.x, frame.y, frame.w, half);
    let lower = Frame::new(frame.x, frame.y + half + gap, frame.w, half);

    let cantor: Vec<f64> = (1..=TOP).map(|n| cantor(n as f64)).collect();
    let bump: Vec<f64> = (1..=TOP).map(|n| bump(n as f64)).collect();

    let powers: Vec<usize> = (0..)
        .map(|k| 3usize.pow(k))
        .take_while(|&p| p <= TOP)
        .collect();
    let ceiling = cantor[0];
    for &p in &powers {
        assert!((cantor[p - 1] - ceiling).abs() < 1e-9);
    }
    assert_eq!(powers.len(), 6);
    assert!(cantor.iter().all(|&v| v <= ceiling + 1e-9));
    assert!(bump[0] > 0.9);
    assert!(bump[40..].iter().all(|&v| v < 0.01));

    comb(&mut board, upper, &cantor, ceiling, &powers);
    comb(&mut board, lower, &bump, ceiling.max(bump[0]), &[]);
    plot::axis(&mut board, upper, ink::line());
    plot::axis(&mut board, lower, ink::line());
    save("wiki-rajchman-measure", &board)?;
    Ok(())
}

fn comb(board: &mut Board, frame: Frame, values: &[f64], peak: f64, lit: &[usize]) {
    let slot = frame.w / values.len() as f64;
    let pad = slot * 0.2;
    for (i, &v) in values.iter().enumerate() {
        let h = frame.h * v / peak;
        let color = if lit.contains(&(i + 1)) {
            ink::yellow()
        } else {
            ink::blue()
        };
        board.rect(
            frame.x + i as f64 * slot + pad,
            frame.y + frame.h - h,
            slot - 2.0 * pad,
            h,
            color,
        );
    }
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
