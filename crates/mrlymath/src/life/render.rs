use super::models::Life;
use crate::two::{self, Cell2d};
use mrlycore::colors::{Color, BLACK, WHITE};
use mrlycore::enums::Mode;
use mrlycore::errors::{value_error, Result};
use std::collections::HashMap;

fn default_palette() -> HashMap<u8, Vec<Color>> {
    HashMap::from([(0, vec![WHITE]), (1, vec![BLACK])])
}

/// Renders every generation of a run to PNG bytes.
pub fn frames(life: &Life, scale: usize) -> Result<Vec<Vec<u8>>> {
    frames_of(&life.grids, scale)
}

/// Renders grids to black-on-white PNG bytes at a pixel scale.
pub fn frames_of(grids: &[Cell2d], scale: usize) -> Result<Vec<Vec<u8>>> {
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
    mrlycore::codec::gif(&views, &palette, width, height, scale, delay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{animate, moore, Boundary, Config};
    use mrlycore::tensor::Tensor;
    fn triple() -> Cell2d {
        let mut t = Tensor::new(vec![3, 3]);
        t.set(&[1, 0], 1);
        t.set(&[1, 1], 1);
        t.set(&[1, 2], 1);
        Cell2d::new(t)
    }
    #[test]
    fn frames_are_pngs() {
        let config = Config {
            boundary: Boundary::Constant,
            max_generations: 8,
            ..Config::new(moore(), vec![3], vec![2, 3])
        };
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        let life = animate(&Cell2d::new(t), &config).unwrap();
        let pngs = frames(&life, 4).unwrap();
        assert_eq!(pngs.len(), life.count);
        for png in &pngs {
            assert_eq!(&png[1..4], b"PNG");
        }
    }
    #[test]
    fn a_run_becomes_one_looping_gif() {
        let config = Config {
            boundary: Boundary::Constant,
            max_generations: 8,
            ..Config::new(moore(), vec![3], vec![2, 3])
        };
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        let life = animate(&Cell2d::new(t), &config).unwrap();
        let gif = movie(&life.grids, 4, 20).unwrap();
        assert_eq!(&gif[0..6], b"GIF89a");
        assert_eq!(&gif[6..10], &[20, 0, 20, 0]);
        assert_eq!(gif[gif.len() - 1], 0x3b);
        assert!(gif.len() < frames(&life, 4).unwrap().iter().map(|f| f.len()).sum());
        assert!(movie(&[], 4, 20).is_err());
        assert!(movie(&[triple(), life.grids[0].clone()], 4, 20).is_err());
    }
}
