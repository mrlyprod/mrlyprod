use crate::entry::Entry;
use crate::grid::Grid;
use mrly_error::{Error, Result};
use mrly_json::{Json, json};
use mrly_math::cell::models::Cell2d;
use mrly_tensor::Tensor;

pub fn cells(grid: &Grid) -> Result<Vec<Cell2d>> {
    let mut out = Vec::with_capacity(grid.frames.len());
    for (at, frame) in grid.frames.iter().enumerate() {
        let mut cell = Cell2d::new(Tensor::of(frame.clone(), vec![grid.height, grid.width])?)?;
        cell.cell.colors = Some(grid.colors(at));
        out.push(cell);
    }
    Ok(out)
}

pub fn json(entry: &Entry, grid: &Grid, rows: &str, blocks: &str) -> String {
    let mut palette = vec![Json::Null];
    for &(_, [r, g, b]) in &grid.palette {
        palette.push(json!(format!("{r:02x}{g:02x}{b:02x}")));
    }
    let frames: Vec<Vec<&[u8]>> = grid
        .frames
        .iter()
        .map(|frame| frame.chunks(grid.width.max(1)).collect())
        .collect();
    let mut data = json!({
        "v": 1,
        "title": entry.title,
        "description": entry.description,
        "tags": entry.tags,
        "maker": entry.maker,
        "date": entry.date,
        "origin": entry.origin,
        "width": grid.width,
        rows: grid.height,
        "palette": palette,
        blocks: frames,
    });
    if entry.delay > 0 {
        data["delay"] = json!(entry.delay);
    }
    data.to_string()
}

fn inks(grid: &Grid) -> Vec<[u8; 4]> {
    let mut out = vec![[0, 0, 0, 0]];
    out.extend(grid.palette.iter().map(|&(_, [r, g, b])| [r, g, b, 255]));
    out
}

fn enlarged(grid: &Grid, scale: usize) -> Result<Vec<Vec<u8>>> {
    grid.frames
        .iter()
        .map(|frame| mrly_gif::scale(frame, grid.width, scale).map_err(Error::Value))
        .collect()
}

pub fn png(grid: &Grid, scale: usize) -> Result<Vec<u8>> {
    let shots = enlarged(grid, scale)?;
    let (width, height) = (grid.width * scale, grid.height * scale);
    let mut strip = vec![0u8; width * shots.len() * height];
    for (at, shot) in shots.iter().enumerate() {
        for (y, row) in shot.chunks(width.max(1)).enumerate() {
            let start = y * width * shots.len() + at * width;
            strip[start..start + width].copy_from_slice(row);
        }
    }
    mrly_png::encode_indexed(&strip, width * shots.len(), height, &inks(grid)).map_err(Error::Value)
}

pub fn gif(grid: &Grid, delay: usize, scale: usize) -> Result<Vec<u8>> {
    let shots = enlarged(grid, scale)?;
    let ticks = (delay.saturating_add(5) / 10).clamp(2, 65535) as u16;
    mrly_gif::encode(
        &shots,
        grid.width * scale,
        grid.height * scale,
        &inks(grid),
        &[ticks],
    )
    .map_err(Error::Value)
}
