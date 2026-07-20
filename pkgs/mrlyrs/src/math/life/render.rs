use super::models::Life;
use crate::core::colors::{Color, BLACK, WHITE};
use crate::core::enums::Mode;
use crate::core::errors::Result;
use crate::math::two::{self, Cell2d};
use std::collections::HashMap;

fn default_palette() -> HashMap<u8, Vec<Color>> {
    HashMap::from([(0, vec![WHITE]), (1, vec![BLACK])])
}

pub fn frames(life: &Life, scale: usize) -> Result<Vec<Vec<u8>>> {
    frames_of(&life.grids, scale)
}

pub fn frames_of(grids: &[Cell2d], scale: usize) -> Result<Vec<Vec<u8>>> {
    let palette = default_palette();
    let mut out = Vec::with_capacity(grids.len());
    for grid in grids {
        let painted = grid.clone().paint(&palette, Mode::Type);
        out.push(two::png(&painted, scale)?);
    }
    Ok(out)
}

pub fn frame(grid: &Cell2d, scale: usize) -> Result<Vec<u8>> {
    let painted = grid.clone().paint(&default_palette(), Mode::Type);
    two::png(&painted, scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::atoms;
    use crate::math::life::{animate, Boundary, Config};
    #[test]
    fn frames_are_pngs() {
        let mut m = atoms::carpet_2d(3);
        m.set(&[1, 1], 0);
        let config = Config {
            boundary: Boundary::Constant,
            max_generations: 8,
            ..Config::new(Cell2d::new(m), vec![3], vec![2, 3])
        };
        let mut t = crate::core::tensor::Tensor::new(vec![5, 5]);
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
}
