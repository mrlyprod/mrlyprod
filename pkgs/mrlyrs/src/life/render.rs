use crate::core::cell::Mode;
use crate::core::colors::{Color, BLACK, WHITE};
use crate::core::error::{value_error, Result};
use crate::core::ramp::Colorizer;
use crate::core::tensor::Tensor;
use crate::math::two::{self, Cell2d};
use std::collections::HashMap;

fn default_palette() -> HashMap<u8, Vec<Color>> {
    HashMap::from([(0, vec![WHITE]), (1, vec![BLACK])])
}

/// Renders grids to black-on-white PNG bytes at a pixel scale.
pub fn frames(grids: &[Cell2d], scale: usize) -> Result<Vec<Vec<u8>>> {
    let palette = default_palette();
    let mut out = Vec::with_capacity(grids.len());
    for grid in grids {
        let painted = grid.clone().paint(&palette, Mode::Type);
        out.push(two::png(&painted, scale)?);
    }
    Ok(out)
}

/// Renders one grid to black-on-white PNG bytes at a pixel scale.
pub fn frame(grid: &Cell2d, scale: usize) -> Result<Vec<u8>> {
    let painted = grid.clone().paint(&default_palette(), Mode::Type);
    two::png(&painted, scale)
}

/// Renders grids into one looping black-on-white gif, the delay in hundredths of a second.
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
    let size = span[0].types().size();
    let mut total = vec![0usize; size];
    for grid in span {
        for (i, &v) in grid.types().bytes().iter().enumerate() {
            total[i] += v as usize;
        }
    }
    let max = (*total.iter().max().unwrap_or(&0)).max(1);
    let mut cumulative = vec![0usize; size];
    let mut out = Vec::with_capacity(span.len());
    for grid in span {
        for (i, &v) in grid.types().bytes().iter().enumerate() {
            cumulative[i] += v as usize;
        }
        let colors = colorizer.colors(&cumulative, max);
        let mut cell = Cell2d::new(Tensor::new(shape.clone()));
        cell.cell.colors = Some(colors);
        out.push(two::png(&cell, scale)?);
    }
    Ok(out)
}

/// Renders a whole run's cumulative-visit heatmap frames with the heat ramp.
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
            ..Config::new(moore(), vec![3], vec![2, 3])
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
        assert_eq!(gif[gif.len() - 1], 0x3b);
        let loose: usize = frames(&life.grids, 4)
            .unwrap()
            .iter()
            .map(|f| f.len())
            .sum();
        assert!(gif.len() < loose);
        assert!(movie(&[], 4, 20).is_err());
        let smaller = Cell2d::new(Tensor::new(vec![3, 3]));
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
    fn a_backwards_heatmap_range_errors() {
        let life = run();
        assert!(heatmap_range(&life.grids, 2, 1, &Colorizer::heat(), 4).is_err());
    }
}
