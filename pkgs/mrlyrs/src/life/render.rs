use crate::core::cell::Mode;
use crate::core::colors::{Color, BLACK, WHITE};
use crate::core::error::{shape_error, value_error, Result};
use crate::core::ramp::Colorizer;
use crate::core::tensor::Tensor;
use crate::math::two::{self, Cell2d};
use std::collections::HashMap;

fn default_palette() -> HashMap<u8, Vec<Color>> {
    HashMap::from([(0, vec![BLACK]), (1, vec![WHITE])])
}

/// Renders grids to white-on-black PNG bytes at a pixel scale.
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
/// Errs when a grid will not encode to PNG at the scale.
pub fn frames(grids: &[Cell2d], scale: usize) -> Result<Vec<Vec<u8>>> {
    let palette = default_palette();
    let mut out = Vec::with_capacity(grids.len());
    for grid in grids {
        let painted = grid.clone().paint(&palette, Mode::Type, None)?;
        out.push(two::png(&painted, scale, None, 1, two::Shape::Square)?);
    }
    Ok(out)
}

/// Renders one grid to white-on-black PNG bytes at a pixel scale.
///
/// # Errors
///
/// Errs when the grid will not encode to PNG at the scale.
pub fn frame(grid: &Cell2d, scale: usize) -> Result<Vec<u8>> {
    let painted = grid.clone().paint(&default_palette(), Mode::Type, None)?;
    two::png(&painted, scale, None, 1, two::Shape::Square)
}

/// Renders grids into one looping black-on-white gif, the delay in hundredths of a second.
///
/// # Errors
///
/// Errs when no grid is given, the grids differ in size, or the gif will not encode.
pub fn movie(grids: &[Cell2d], scale: usize, delay: usize) -> Result<Vec<u8>> {
    let Some(first) = grids.first() else {
        return value_error("a movie needs at least one grid.");
    };
    let (width, height) = (first.width(), first.height());
    let mut frames = Vec::with_capacity(grids.len());
    for grid in grids {
        if (grid.width(), grid.height()) != (width, height) {
            return value_error("every grid must share one size.");
        }
        let types = grid.types();
        frames.push(
            (0..types.size())
                .map(|i| u8::from(types.at(i) != 0))
                .collect::<Vec<u8>>(),
        );
    }
    let views: Vec<&[u8]> = frames.iter().map(|frame| frame.as_slice()).collect();
    let palette = [
        [WHITE.r, WHITE.g, WHITE.b, WHITE.a],
        [BLACK.r, BLACK.g, BLACK.b, BLACK.a],
    ];
    crate::core::codec::gif(&views, &palette, width, height, scale, delay)
}

/// Renders cumulative-visit heatmap frames over a slice of a run, or an error at a bad range.
fn heatmap_range(
    grids: &[Cell2d],
    start: usize,
    end: usize,
    colorizer: &Colorizer,
    scale: usize,
) -> Result<Vec<Vec<u8>>> {
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
        let colors = colorizer.colors(&cumulative, max);
        let mut cell = Cell2d::new(Tensor::new(shape.clone()))?;
        cell.cell.colors = Some(colors);
        out.push(two::png(&cell, scale, None, 1, two::Shape::Square)?);
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
/// Errs when the grids differ in shape, or a frame will not encode at the scale.
pub fn heatmap(grids: &[Cell2d], scale: usize) -> Result<Vec<Vec<u8>>> {
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
            ..Config::new(moore().unwrap(), vec![3], vec![2, 3])
        };
        animate(&blinker(), &config).unwrap()
    }
    #[test]
    fn frames_are_pngs() {
        let life = run();
        let pngs = frames(&life.grids, 4).unwrap();
        assert_eq!(pngs.len(), life.count);
        for png in &pngs {
            assert_eq!(&png[1..4], b"PNG");
        }
    }
    #[test]
    fn a_run_becomes_one_looping_gif() {
        let life = run();
        let gif = movie(&life.grids, 4, 20).unwrap();
        assert_eq!(&gif[6..10], &[20, 0, 20, 0]);
        let loose: usize = frames(&life.grids, 4)
            .unwrap()
            .iter()
            .map(|f| f.len())
            .sum();
        assert!(gif.len() < loose);
        assert!(movie(&[], 4, 20).is_err());
        let smaller = Cell2d::new(Tensor::new(vec![3, 3])).unwrap();
        assert!(movie(&[smaller, life.grids[0].clone()], 4, 20).is_err());
    }
    #[test]
    fn a_heatmap_covers_every_generation() {
        let life = run();
        assert_eq!(heatmap(&life.grids, 4).unwrap().len(), life.count);
    }
    #[test]
    fn a_heatmap_range_renders_its_slice() {
        let life = run();
        let pngs = heatmap_range(&life.grids, 0, 1, &Colorizer::heat(), 4).unwrap();
        assert_eq!(pngs.len(), 1);
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
