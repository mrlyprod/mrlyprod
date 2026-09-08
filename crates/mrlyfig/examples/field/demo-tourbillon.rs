use mrlycore::errors::Result;
use mrlyfig::board::Board;
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlynum::tourbillon;

const TOP: usize = 55;
const RASTER: usize = 1024;
const LAYERS: usize = 28;
const CENTRE: f64 = 14.0 / 28.0;
const SEED: u32 = 1;
const MARGIN: f64 = 0.08;
const CURVE: f64 = 1.45;

fn shrink(source: &[f32], size: usize, side: usize) -> Vec<f64> {
    let step = size as f64 / side as f64;
    let span = |k: usize| (k as f64 * step, (k + 1) as f64 * step);
    let mut out = Vec::with_capacity(side * side);
    for row in 0..side {
        let (top, bottom) = span(row);
        for column in 0..side {
            let (left, right) = span(column);
            let mut total = 0.0;
            for y in top as usize..(bottom.ceil() as usize).min(size) {
                let height = bottom.min(y as f64 + 1.0) - top.max(y as f64);
                for x in left as usize..(right.ceil() as usize).min(size) {
                    let width = right.min(x as f64 + 1.0) - left.max(x as f64);
                    let value = source[y * size + x];
                    if !value.is_nan() {
                        total += height * width * f64::from(value);
                    }
                }
            }
            out.push(total / (step * step));
        }
    }
    out
}

fn main() -> Result<()> {
    let list = tourbillon::layers(TOP, "golden", 0.0, "odd", "plain", SEED)?;
    assert_eq!(list.len(), LAYERS);
    assert_eq!(list[0].scale, 1);
    assert_eq!(list[LAYERS - 1].scale, TOP);

    let spun = tourbillon::field(
        TOP, RASTER, "golden", 0.0, "odd", "plain", "cells", "mean", SEED,
    )?;
    assert_eq!(spun.len(), RASTER * RASTER);
    let read = tourbillon::stats(
        &spun, RASTER, TOP, "golden", 0.0, "odd", "plain", "mean", SEED,
    )?;
    assert_eq!(read.layers, LAYERS);
    assert_eq!(read.centre, CENTRE);

    let mut board = Board::square();
    let frame = board.frame(MARGIN);
    let side = frame.w.round() as usize;
    let disc: Vec<f64> = shrink(&spun, RASTER, side)
        .iter()
        .map(|value| (value / read.centre).clamp(0.0, 1.0).powf(CURVE))
        .collect();
    assert_eq!(disc.len(), side * side);

    let ramp = Ramp::new(vec![ink::ground(), ink::indigo(), ink::pink()]);
    field::draw_range(&mut board, frame, side, side, &disc, (0.0, 1.0), &ramp);
    save("demo-tourbillon", &board)?;
    Ok(())
}
