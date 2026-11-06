use mrly_color::ramp::Colorizer;
use mrly_color::{BLACK, Color, WHITE};
use mrly_error::{Error, Result, shape_error, value_error};
use mrly_image::Image;
use mrly_math::cell::base::Mode;
use mrly_tensor::Tensor;
use mrly_two as two;
use mrly_two::Cell2d;
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

pub fn gif(grids: &[Cell2d], scale: usize, delay: u16) -> Result<Vec<u8>> {
    let Some(first) = grids.first() else {
        return value_error("a gif needs at least one grid.");
    };
    let (width, height) = (first.width(), first.height());
    let mut frames = Vec::with_capacity(grids.len());
    for grid in grids {
        if (grid.width(), grid.height()) != (width, height) {
            return shape_error("every grid must share one shape.");
        }
        let types = grid.types();
        let alive: Vec<u8> = (0..types.size())
            .map(|at| (types.at(at) != 0) as u8)
            .collect();
        frames.push(mrly_gif::scale(&alive, width, scale).map_err(Error::Value)?);
    }
    let palette = [BLACK, WHITE].map(|color| [color.r, color.g, color.b, 255]);
    mrly_gif::encode(&frames, width * scale, height * scale, &palette, &[delay])
        .map_err(Error::Value)
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
        let colors = mrly_color::ramp::colors(colorizer, &cumulative, max);
        let mut cell = Cell2d::new(Tensor::new(shape.clone()))?;
        cell.cell.colors = Some(colors);
        out.push(two::image(&cell, scale, None, 1, two::Shape::Square)?);
    }
    Ok(out)
}

pub fn heatmap(grids: &[Cell2d], scale: usize) -> Result<Vec<Image>> {
    heatmap_range(grids, 0, grids.len(), &Colorizer::heat(), scale)
}
