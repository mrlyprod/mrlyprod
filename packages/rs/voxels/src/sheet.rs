use mrly_color::Color;
use mrly_error::{Error, Result};
use mrly_image::Image;
use mrly_space::{View, render};
use mrly_three::Cell3d;

const WALL: Color = Color::rgb(30, 31, 36);
const GAP: usize = 16;
const HERO: (f64, f64) = (30.0, 30.0);
const ANGLES: [(f64, f64); 6] = [
    HERO,
    (120.0, 30.0),
    (210.0, 30.0),
    (300.0, 30.0),
    (30.0, 60.0),
    (0.0, 90.0),
];

pub fn shot(cell: &Cell3d, side: usize, yaw: f64, pitch: f64) -> Result<Image> {
    let view = View {
        width: side,
        height: side,
        yaw,
        pitch,
        background: WALL,
        ..View::default()
    };
    render(cell, &view)
}

pub fn png(cell: &Cell3d, side: usize) -> Result<Vec<u8>> {
    let image = shot(cell, side, HERO.0, HERO.1)?;
    mrly_png::encode(&image.colors, image.width, image.height).map_err(Error::Value)
}

fn sheet(shots: &[Image], across: usize) -> Result<Vec<u8>> {
    let across = across.max(1);
    let side = shots.first().map_or(0, |shot| shot.width);
    let down = shots.len().div_ceil(across);
    let (width, height) = (GAP + across * (side + GAP), GAP + down * (side + GAP));
    let mut colors = vec![[WALL.r, WALL.g, WALL.b, 255]; width * height];
    for (at, shot) in shots.iter().enumerate() {
        let (left, top) = (
            GAP + at % across * (side + GAP),
            GAP + at / across * (side + GAP),
        );
        for (y, row) in shot.colors.chunks(shot.width.max(1)).enumerate() {
            let start = (top + y) * width + left;
            colors[start..start + row.len()].copy_from_slice(row);
        }
    }
    mrly_png::encode(&colors, width, height).map_err(Error::Value)
}

pub fn view(cell: &Cell3d, side: usize) -> Result<Vec<u8>> {
    let shots: Result<Vec<Image>> = ANGLES
        .iter()
        .map(|&(yaw, pitch)| shot(cell, side, yaw, pitch))
        .collect();
    sheet(&shots?, 3)
}

pub fn overview(cells: &[Cell3d], side: usize) -> Result<Vec<u8>> {
    let shots: Result<Vec<Image>> = cells
        .iter()
        .map(|cell| shot(cell, side, HERO.0, HERO.1))
        .collect();
    sheet(&shots?, cells.len())
}

pub fn turntable(cell: &Cell3d, side: usize, steps: usize, delay: usize) -> Result<Vec<u8>> {
    let mut frames = Vec::with_capacity(steps);
    for step in 0..steps {
        let yaw = HERO.0 + 360.0 * step as f64 / steps.max(1) as f64;
        frames.push(shot(cell, side, yaw, HERO.1)?.colors);
    }
    let ticks = (delay.saturating_add(5) / 10).clamp(2, 65535) as u16;
    mrly_gif::encode_rgba(&frames, side, side, &[ticks], false).map_err(Error::Value)
}
