use crate::core::cell::Mode;
use crate::core::colors::{Color, BLACK, WHITE};
use crate::core::error::{shape_error, value_error, Result};
use crate::core::image::Image;
use crate::core::ramp::Colorizer;
use crate::core::tensor::Tensor;
use crate::math::two::{self, Cell2d};
use std::collections::HashMap;

fn default_palette() -> HashMap<u8, Vec<Color>> {
    HashMap::from([(0, vec![BLACK]), (1, vec![WHITE])])
}

/// Renders grids to white-on-black images at a pixel scale.
///
/// ```
/// use mrlyrs::core::tensor::Tensor;
/// use mrlyrs::life::frames;
/// use mrlyrs::math::two::Cell2d;
/// let grid = Cell2d::new(Tensor::of(vec![1, 0, 0, 1], vec![2, 2])?)?;
/// assert_eq!(frames(&[grid.clone(), grid], 4)?.len(), 2);
/// # Ok::<(), mrlyrs::Error>(())
/// ```
///
/// # Errors
///
/// Errs on a scale below one.
pub fn frames(grids: &[Cell2d], scale: usize) -> Result<Vec<Image>> {
    let palette = default_palette();
    let mut out = Vec::with_capacity(grids.len());
    for grid in grids {
        let painted = grid.clone().paint(&palette, Mode::Type, None)?;
        out.push(two::image(&painted, scale, None, 1, two::Shape::Square)?);
    }
    Ok(out)
}

/// Renders one grid to a white-on-black image at a pixel scale.
///
/// # Errors
///
/// Errs on a scale below one.
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
        let colors = crate::core::ramp::colors(colorizer, &cumulative, max);
        let mut cell = Cell2d::new(Tensor::new(shape.clone()))?;
        cell.cell.colors = Some(colors);
        out.push(two::image(&cell, scale, None, 1, two::Shape::Square)?);
    }
    Ok(out)
}

/// Renders a whole run's cumulative-visit heatmap frames with the heat ramp.
///
/// ```
/// use mrlyrs::core::tensor::Tensor;
/// use mrlyrs::life::heatmap;
/// use mrlyrs::math::two::Cell2d;
/// let grid = Cell2d::new(Tensor::of(vec![1, 0, 0, 1], vec![2, 2])?)?;
/// assert_eq!(heatmap(&[grid.clone(), grid], 4)?.len(), 2);
/// # Ok::<(), mrlyrs::Error>(())
/// ```
///
/// # Errors
///
/// Errs when the grids differ in shape, or on a scale below one.
pub fn heatmap(grids: &[Cell2d], scale: usize) -> Result<Vec<Image>> {
    heatmap_range(grids, 0, grids.len(), &Colorizer::heat(), scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{animate, blinker, moore, Boundary, Config, Life};
    fn run() -> Life {
        let config = Config {
            boundary: Boundary::Constant,
            max_generations: 8,
            ..Config::new(moore().unwrap(), vec![3].into(), vec![2, 3].into())
        };
        animate(&blinker(), &config).unwrap()
    }
    #[test]
    fn frames_paint_the_live_cells_white_on_black() {
        let life = run();
        let sheets = frames(&life.grids, 4).unwrap();
        assert_eq!(sheets.len(), life.count);
        let white = [WHITE.r, WHITE.g, WHITE.b, WHITE.a];
        let black = [BLACK.r, BLACK.g, BLACK.b, BLACK.a];
        for (sheet, grid) in sheets.iter().zip(&life.grids) {
            assert_eq!(
                (sheet.width, sheet.height),
                (grid.width() * 4, grid.height() * 4)
            );
            let lit = sheet.colors.iter().filter(|&&p| p == white).count();
            let dark = sheet.colors.iter().filter(|&&p| p == black).count();
            assert_eq!(lit, grid.types().sum() as usize * 16);
            assert_eq!(lit + dark, sheet.colors.len());
        }
        assert_eq!(frame(&life.grids[0], 4).unwrap(), sheets[0]);
    }
    #[test]
    fn a_heatmap_covers_every_generation() {
        let life = run();
        assert_eq!(heatmap(&life.grids, 4).unwrap().len(), life.count);
    }
    #[test]
    fn a_heatmap_range_renders_its_slice() {
        let life = run();
        let sheets = heatmap_range(&life.grids, 0, 1, &Colorizer::heat(), 4).unwrap();
        assert_eq!(sheets.len(), 1);
    }
    #[test]
    fn refuses_heatmap() {
        let life = run();
        let heat = Colorizer::heat();
        assert!(heatmap_range(&life.grids, 2, 1, &heat, 4).is_err());
        assert!(heatmap_range(&life.grids, 0, life.count + 1, &heat, 4).is_err());
        let odd = Cell2d::new(Tensor::new(vec![3, 3])).unwrap();
        assert!(heatmap(&[life.grids[0].clone(), odd], 4).is_err());
        assert!(heatmap(&life.grids, 0).is_err());
    }
}
