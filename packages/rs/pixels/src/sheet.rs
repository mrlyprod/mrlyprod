use crate::grid::Grid;
use mrly_error::{Error, Result};

const WALL: [u8; 4] = [30, 31, 36, 255];
const TILES: [[u8; 4]; 2] = [[112, 112, 118, 255], [126, 126, 132, 255]];
const TILE: usize = 8;
const GAP: usize = 16;

struct Sheet {
    width: usize,
    height: usize,
    colors: Vec<[u8; 4]>,
}

impl Sheet {
    fn new(width: usize, height: usize) -> Sheet {
        Sheet {
            width,
            height,
            colors: vec![WALL; width * height],
        }
    }
    fn stamp(
        &mut self,
        grid: &Grid,
        frame: usize,
        left: usize,
        top: usize,
        scale: usize,
        gray: bool,
    ) {
        let colors = grid.colors(frame);
        for y in 0..grid.height * scale {
            for x in 0..grid.width * scale {
                let [r, g, b, a] = colors[y / scale * grid.width + x / scale];
                let shown = if a == 0 {
                    TILES[(x / TILE + y / TILE) % 2]
                } else if gray {
                    let luma =
                        (299 * u32::from(r) + 587 * u32::from(g) + 114 * u32::from(b)) / 1000;
                    [luma as u8, luma as u8, luma as u8, 255]
                } else {
                    [r, g, b, 255]
                };
                self.colors[(top + y) * self.width + left + x] = shown;
            }
        }
    }
    fn png(&self) -> Result<Vec<u8>> {
        mrly_png::encode(&self.colors, self.width, self.height).map_err(Error::Value)
    }
}

fn fit(grid: &Grid, side: usize) -> usize {
    (side / grid.width.max(grid.height).max(1)).max(1)
}

pub fn view(grid: &Grid, side: usize) -> Result<Vec<u8>> {
    let scale = fit(grid, side);
    let (width, height) = (grid.width * scale, grid.height * scale);
    let count = grid.frames.len();
    let mut sheet = Sheet::new(GAP + count * (width + GAP), height + 2 * GAP);
    for at in 0..count {
        sheet.stamp(grid, at, GAP + at * (width + GAP), GAP, scale, false);
    }
    sheet.png()
}

pub fn quarters(grid: &Grid, side: usize) -> Result<Vec<Vec<u8>>> {
    let (width, height) = (grid.width.div_ceil(2), grid.height.div_ceil(2));
    let mut out = Vec::with_capacity(4);
    for (left, top) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        let (left, top) = (left * (grid.width - width), top * (grid.height - height));
        let frames = grid
            .frames
            .iter()
            .map(|frame| {
                frame
                    .chunks(grid.width.max(1))
                    .skip(top)
                    .take(height)
                    .flat_map(|row| row[left..left + width].iter().copied())
                    .collect()
            })
            .collect();
        let part = Grid {
            width,
            height,
            palette: grid.palette.clone(),
            frames,
        };
        out.push(view(&part, side)?);
    }
    Ok(out)
}

pub fn overview(grids: &[Grid], side: usize) -> Result<Vec<u8>> {
    let tall = grids.iter().map(|grid| grid.height).max().unwrap_or(0);
    let wide = grids
        .iter()
        .map(|grid| (grid.width * fit(grid, side)).max(side))
        .max()
        .unwrap_or(side);
    let high = grids
        .iter()
        .map(|grid| grid.height * fit(grid, side))
        .max()
        .unwrap_or(side);
    let mut sheet = Sheet::new(GAP + grids.len() * (wide + GAP), 4 * GAP + 2 * high + tall);
    for (at, grid) in grids.iter().enumerate() {
        let (left, scale) = (GAP + at * (wide + GAP), fit(grid, side));
        sheet.stamp(grid, 0, left, GAP, scale, false);
        sheet.stamp(grid, 0, left, 2 * GAP + high, scale, true);
        sheet.stamp(grid, 0, left, 3 * GAP + 2 * high, 1, false);
    }
    sheet.png()
}
