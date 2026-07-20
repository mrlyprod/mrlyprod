use crate::core::errors::{value_error, Result};
use crate::core::ramp::Colorizer;
use crate::core::tensor::Tensor;
use crate::math::two::{self, Cell2d};

pub fn heatmap_range(
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

pub fn heatmap(grids: &[Cell2d], scale: usize) -> Result<Vec<Vec<u8>>> {
    heatmap_range(grids, 0, grids.len(), &Colorizer::heat(), scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::atoms;
    use crate::math::life::{animate, Boundary, Config};
    fn run() -> crate::math::life::Life {
        let mut m = atoms::carpet_2d(3);
        m.set(&[1, 1], 0);
        let config = Config {
            boundary: Boundary::Constant,
            max_generations: 8,
            ..Config::new(Cell2d::new(m), vec![3], vec![2, 3])
        };
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        animate(&Cell2d::new(t), &config).unwrap()
    }
    #[test]
    fn heatmap_frames_are_pngs() {
        let life = run();
        let pngs = heatmap(&life.grids, 4).unwrap();
        assert_eq!(pngs.len(), life.count);
        assert_eq!(&pngs[0][1..4], b"PNG");
    }
    #[test]
    fn range_renders_a_slice() {
        let life = run();
        let pngs = heatmap_range(&life.grids, 0, 1, &Colorizer::heat(), 4).unwrap();
        assert_eq!(pngs.len(), 1);
    }
    #[test]
    fn bad_range_errors() {
        let life = run();
        assert!(heatmap_range(&life.grids, 2, 1, &Colorizer::heat(), 4).is_err());
    }
}
