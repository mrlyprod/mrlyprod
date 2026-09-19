use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board, Color, Frame};
use std::f64::consts::PI;

const VERTICES: usize = 32;
const CEILING: f64 = 4.0;

fn path() -> Vec<f64> {
    (0..VERTICES)
        .map(|k| 2.0 - 2.0 * (PI * k as f64 / VERTICES as f64).cos())
        .collect()
}

fn cycle() -> Vec<f64> {
    let mut out: Vec<f64> = (0..VERTICES)
        .map(|k| 2.0 - 2.0 * (2.0 * PI * k as f64 / VERTICES as f64).cos())
        .collect();
    out.sort_by(|a, b| a.partial_cmp(b).expect("a finite spectrum"));
    out
}

fn distinct(values: &[f64]) -> usize {
    let mut count = 1;
    for pair in values.windows(2) {
        if pair[1] - pair[0] > 1e-9 {
            count += 1;
        }
    }
    count
}

fn steps(board: &mut Board, frame: Frame, values: &[f64], thick: f64, paint: Color) {
    let slot = frame.w / values.len() as f64;
    let mut pts = Vec::with_capacity(values.len() * 2);
    for (index, value) in values.iter().enumerate() {
        let y = frame.y + frame.h * (1.0 - value / CEILING);
        pts.push((frame.x + index as f64 * slot, y));
        pts.push((frame.x + (index + 1) as f64 * slot, y));
    }
    board.polyline(&pts, thick, paint);
    for (index, value) in values.iter().enumerate() {
        let y = frame.y + frame.h * (1.0 - value / CEILING);
        board.disc(frame.x + (index as f64 + 0.5) * slot, y, thick * 0.9, paint);
    }
}

fn main() -> Result<()> {
    let line = path();
    let ring = cycle();
    assert_eq!((line.len(), ring.len()), (VERTICES, VERTICES));
    assert_eq!((distinct(&line), distinct(&ring)), (32, 17));
    assert!(line[0].abs() < 1e-12 && ring[0].abs() < 1e-12);
    assert!((ring[VERTICES - 1] - CEILING).abs() < 1e-12);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let stage = frame.inset(frame.w * 0.04);
    plot::baseline(&mut board, stage, ink::line());
    for k in 1..4 {
        let y = stage.y + stage.h * (1.0 - k as f64 / CEILING);
        board.segment(
            (stage.x, y),
            (stage.x + stage.w, y),
            1.5,
            ink::fade(ink::dim(), 0.7),
        );
    }
    steps(&mut board, stage, &ring, 7.0, ink::orange());
    steps(&mut board, stage, &line, 7.0, ink::blue());
    save("wiki-graph-laplacian", &board)?;
    Ok(())
}
