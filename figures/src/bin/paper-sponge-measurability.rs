use figures::{ink, plot, save, Board, Frame, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::math::three::sponge::{distance, profile, tube, COVER};
use mrlyrs::math::three::{carpet, slice};
use std::f64::consts::PI;

const LEVEL: usize = 4;
const SIDE: usize = 81;
const EPS: f64 = 1.0 / 36.0;
const BANDS: [(f64, f64, f64); 3] = [
    (1.0 / 12.0, 2.122718, 2.122723),
    (1.0 / 8.0, 2.134668, 2.135742),
    (1.0 / 6.0, 2.135019, 2.136794),
];
const TUBES: [(f64, f64, f64); 2] = [
    (1.0 / 12.0, 0.180947086, 0.180947093),
    (1.0 / 8.0, 0.234186414, 0.234701259),
];

fn dist(x: f64, y: f64) -> f64 {
    distance([x, y, 0.5])
}

// THE MARKS

fn bands(board: &mut Board, frame: Frame) {
    plot::axis(board, frame, ink::line());
    let area = frame.inset(24.0);
    let (lo, hi) = (2.120, 2.140);
    let y_of = |v: f64| area.y + area.h * (hi - v) / (hi - lo);
    let width = area.w * 0.3;
    let (a, b) = (BANDS[0], BANDS[2]);
    for (k, band) in [a, b].iter().enumerate() {
        let x = area.x + area.w * (0.25 + 0.5 * k as f64) - width / 2.0;
        let (top, foot) = (y_of(band.2), y_of(band.1));
        let h = (foot - top).max(4.0);
        board.rect(x, top, width, h, ink::orange());
    }
    let (cx, gap_lo, gap_hi) = (area.x + area.w / 2.0, y_of(a.2) - 2.0, y_of(b.1) + 2.0);
    board.segment((cx, gap_lo), (cx, gap_hi), 2.0, ink::dim());
    board.segment((cx - 10.0, gap_lo), (cx + 10.0, gap_lo), 2.0, ink::dim());
    board.segment((cx - 10.0, gap_hi), (cx + 10.0, gap_hi), 2.0, ink::dim());
}

fn main() -> Result<()> {
    let sponge = carpet(3, LEVEL)?;
    assert_eq!(sponge.types().sum(), 160000);
    let dust = slice(&sponge, 2, (SIDE - 1) / 2)?;
    let cells = dust.types().bytes()?.to_vec();
    assert_eq!(cells.len(), SIDE * SIDE);
    assert_eq!(cells.iter().filter(|&&b| b != 0).count(), 256);

    let ident = (PI + 8.0) / 36.0 - 2f64.sqrt() / 27.0;
    assert!((tube(1.0 / 6.0).unwrap_or(0.0) - ident).abs() < 1e-9);
    for (delta, lo, hi) in TUBES {
        let t = tube(delta).unwrap_or(0.0);
        assert!(lo <= t && t <= hi);
    }
    let mut values = [0.0; 3];
    for (k, (eps, lo, hi)) in BANDS.iter().enumerate() {
        values[k] = profile(*eps).unwrap_or(0.0);
        assert!(*lo <= values[k] && values[k] <= *hi);
    }
    assert!(values[2] - values[0] >= 0.012296);
    assert!((dist(0.5, 0.5) - COVER).abs() < 1e-15);
    assert!((dist(1.0 / 6.0, 1.0 / 6.0) - 2f64.sqrt() / 18.0).abs() < 1e-15);

    let mut board = Board::square();
    let margin = (board.width as f64 * 0.08).round();
    let plate = 600.0;
    let sheet = Frame::new(margin, margin, plate, plate);
    board.rect(sheet.x, sheet.y, sheet.w, sheet.h, ink::panel());
    let n = plate as usize;
    let blue = ink::blue();
    for j in 0..n {
        for i in 0..n {
            let (x, y) = ((i as f64 + 0.5) / plate, (j as f64 + 0.5) / plate);
            let cover = (0.5 + (EPS - dist(x, y)) * plate).clamp(0.0, 1.0);
            if cover > 0.0 {
                board.blend(sheet.x as usize + i, sheet.y as usize + j, blue, cover);
            }
        }
    }
    let lattice = Grid::new(sheet, SIDE, SIDE, 0.0);
    for row in 0..SIDE {
        for col in 0..SIDE {
            if cells[row * SIDE + col] != 0 {
                lattice.fill(&mut board, col, row, ink::fg());
            }
        }
    }
    let panel = Frame::new(
        board.width as f64 - margin - 240.0,
        board.height as f64 - margin - 240.0,
        240.0,
        240.0,
    );
    bands(&mut board, panel);
    save("paper-sponge-measurability", &board)?;
    Ok(())
}
