use mrly_core::cell::Mode;
use mrly_core::colors::{BLACK, Color, WHITE};
use mrly_core::error::{Result, shape_error, value_error};
use mrly_core::image::Image;
use mrly_core::ramp::Colorizer;
use mrly_core::tensor::Tensor;
use mrly_math::two::{self, Cell2d};
use std::collections::HashMap;

fn default_palette() -> HashMap<u8, Vec<Color>> {
    HashMap::from([(0, vec![BLACK]), (1, vec![WHITE])])
}

pub fn frames(grids: &[Cell2d], scale: usize) -> Result<Vec<Image>> {
    let palette = default_palette();
    let mut out = Vec::with_capacity(grids.len());
    for grid in grids {
        let painted = grid.clone().paint(&palette, Mode::Type, None)?;
        out.push(two::image(&painted, scale, None, 1, two::Shape::Square)?);
    }
    Ok(out)
}

pub fn frame(grid: &Cell2d, scale: usize) -> Result<Image> {
    let painted = grid.clone().paint(&default_palette(), Mode::Type, None)?;
    two::image(&painted, scale, None, 1, two::Shape::Square)
}

fn heatmap_range(
    grids: &[Cell2d],
    start: usize,
    end: usize,
    colorizer: &Colorizer,
    scale: usize,
) -> Result<Vec<Image>> {
    if grids.is_empty() {
        return Ok(Vec::new());
    }
    if start >= end || end > grids.len() {
        return value_error("heatmap range must satisfy 0 <= start < end <= frame count.");
    }
    let span = &grids[start..end];
    let shape = span[0].types().shape.clone();
    if span.iter().any(|grid| grid.types().shape != shape) {
        return shape_error("every grid must share one shape.");
    }
    let size = span[0].types().size();
    let mut total = vec![0usize; size];
    for grid in span {
        for (i, slot) in total.iter_mut().enumerate() {
            *slot += grid.types().at(i) as usize;
        }
    }
    let max = (*total.iter().max().unwrap_or(&0)).max(1);
    let mut cumulative = vec![0usize; size];
    let mut out = Vec::with_capacity(span.len());
    for grid in span {
        for (i, slot) in cumulative.iter_mut().enumerate() {
            *slot += grid.types().at(i) as usize;
        }
        let colors = mrly_core::ramp::colors(colorizer, &cumulative, max);
        let mut cell = Cell2d::new(Tensor::new(shape.clone()))?;
        cell.cell.colors = Some(colors);
        out.push(two::image(&cell, scale, None, 1, two::Shape::Square)?);
    }
    Ok(out)
}

pub fn heatmap(grids: &[Cell2d], scale: usize) -> Result<Vec<Image>> {
    heatmap_range(grids, 0, grids.len(), &Colorizer::heat(), scale)
}
