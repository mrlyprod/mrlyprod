use super::colors::{Color, ALPHA, BLACK, BLUE, GREEN, RED, WHITE};
use super::enums::Mode;
use super::errors::{value_error, Result};
use super::state;
use super::tensor::{Dtype, Tensor};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub types: Tensor,
    pub colors: Option<Vec<[u8; 4]>>,
    pub tags: Option<Tensor>,
}

pub fn mapping() -> HashMap<u8, Vec<Color>> {
    HashMap::from([
        (0, vec![WHITE]),
        (1, vec![BLACK]),
        (2, vec![ALPHA]),
        (3, vec![RED]),
        (4, vec![GREEN]),
        (5, vec![BLUE]),
    ])
}

impl Cell {
    pub fn new(types: Tensor) -> Cell {
        Cell {
            types,
            colors: None,
            tags: None,
        }
    }
    pub fn shape(&self) -> &[usize] {
        &self.types.shape
    }
    pub fn size(&self) -> usize {
        self.types.size()
    }
    pub fn color_at(&self, flat: usize) -> [u8; 4] {
        match &self.colors {
            Some(colors) => colors[flat],
            None => [0, 0, 0, 0],
        }
    }
    pub fn invert(mut self) -> Cell {
        self.types = self.types.invert();
        self
    }
    pub fn anti(self) -> Cell {
        self.invert()
    }
    pub fn pad(mut self, count: usize, value: u8) -> Cell {
        self.types = self.types.pad(count, value);
        self.colors = None;
        self.tags = self.tags.map(|t| t.pad(count, value));
        self
    }
    pub fn rotate(mut self, k: usize, axes: (usize, usize)) -> Cell {
        if let Some(colors) = &self.colors {
            let map = rot90_map(&self.types.shape, k, axes);
            self.colors = Some(map.iter().map(|&src| colors[src]).collect());
        }
        self.types = self.types.rot90(k, axes);
        self.tags = self.tags.map(|t| t.rot90(k, axes));
        self
    }
    pub fn fractal(mut self, level: usize) -> Result<Cell> {
        if level < 1 {
            return value_error("Fractal level must be at least 1.");
        }
        self.types = self.types.fractal(level);
        self.colors = None;
        self.tags = None;
        Ok(self)
    }
    pub fn tile(mut self, reps: &[usize]) -> Cell {
        self.types = self.types.tile(reps);
        self.colors = None;
        self.tags = self.tags.map(|t| t.tile(reps));
        self
    }
    pub fn layers(mut self, dtype: Dtype) -> Cell {
        self.tags = Some(self.types.layers(dtype));
        self
    }
    pub fn neighbors(
        mut self,
        mask: &Tensor,
        target: u8,
        wrap: bool,
        dtype: Dtype,
    ) -> Result<Cell> {
        self.tags = Some(self.types.neighbors(mask, target, wrap, dtype)?);
        Ok(self)
    }
    pub fn binarize(mut self, threshold: u8) -> Cell {
        self.types = self.types.binarize(threshold);
        self.colors = None;
        self
    }
    pub fn binarize_otsu(mut self) -> Cell {
        self.types = self.types.binarize_otsu();
        self.colors = None;
        self
    }
    pub fn blur(mut self, mask: &Tensor, wrap: bool) -> Result<Cell> {
        self.types = self.types.blur(mask, wrap)?;
        self.colors = None;
        Ok(self)
    }
    pub fn perforate(mut self, mask: &Tensor, value: u8) -> Result<Cell> {
        self.types = self.types.perforate(mask, value)?;
        self.colors = None;
        Ok(self)
    }
    pub fn combine(&self, other: &Cell) -> Cell {
        Cell::new(self.types.kron(&other.types))
    }
    pub fn paint(mut self, mapping: &HashMap<u8, Vec<Color>>, mode: Mode) -> Cell {
        let size = self.size();
        let mut colors = self
            .colors
            .take()
            .unwrap_or_else(|| vec![[0, 0, 0, 0]; size]);
        for (&key, palette) in mapping {
            let rgba: Vec<[u8; 4]> = palette.iter().map(|c| [c.r, c.g, c.b, c.a]).collect();
            if rgba.is_empty() {
                continue;
            }
            let mut enumerated = 0;
            for (flat, &t) in self.types.bytes().iter().enumerate() {
                if t != key {
                    continue;
                }
                let pick = match mode {
                    Mode::Type => 0,
                    Mode::Random => state::randint(0, rgba.len() as i64 - 1) as usize,
                    Mode::Enumerate => {
                        let i = enumerated;
                        enumerated += 1;
                        i % rgba.len()
                    }
                    Mode::Index => flat % rgba.len(),
                    Mode::Tag => match &self.tags {
                        Some(tags) => tags.at(flat) as usize % rgba.len(),
                        None => 0,
                    },
                    Mode::Row | Mode::Column | Mode::Depth => {
                        let axis = match mode {
                            Mode::Row => 0,
                            Mode::Column => 1,
                            _ => 2,
                        };
                        if axis < self.types.shape.len() {
                            axis_index(&self.types, flat, axis) % rgba.len()
                        } else {
                            0
                        }
                    }
                };
                colors[flat] = rgba[pick];
            }
        }
        self.colors = Some(colors);
        self
    }
}

fn axis_index(t: &Tensor, flat: usize, axis: usize) -> usize {
    let mut stride = 1;
    for a in (axis + 1)..t.shape.len() {
        stride *= t.shape[a];
    }
    (flat / stride) % t.shape[axis]
}

pub fn rot90_map(shape: &[usize], k: usize, axes: (usize, usize)) -> Vec<usize> {
    let mut data: Vec<usize> = (0..shape.iter().product()).collect();
    let mut shape = shape.to_vec();
    for _ in 0..k % 4 {
        let (a, b) = axes;
        let mut next_shape = shape.clone();
        next_shape.swap(a, b);
        let mut next = vec![0; data.len()];
        for (flat, item) in next.iter_mut().enumerate() {
            let mut rem = flat;
            let mut multi = Vec::with_capacity(next_shape.len());
            for axis in 0..next_shape.len() {
                let stride: usize = next_shape[(axis + 1)..].iter().product();
                multi.push(rem / stride);
                rem %= stride;
            }
            multi[a] = next_shape[a] - 1 - multi[a];
            multi.swap(a, b);
            let mut source = 0;
            for axis in 0..shape.len() {
                source = source * shape[axis] + multi[axis];
            }
            *item = data[source];
        }
        data = next;
        shape = next_shape;
    }
    data
}

pub fn merge(cells: &[Cell], reps: &[usize]) -> Result<Cell> {
    if cells.is_empty() {
        return value_error("Cannot merge an empty list of cells.");
    }
    let count: usize = reps.iter().product();
    if cells.len() != count {
        return value_error(format!("Expected {count} cells, got {}", cells.len()));
    }
    let inner = cells[0].types.shape.clone();
    for cell in cells {
        if cell.types.shape != inner {
            return value_error("All cells in a merge operation must have the same dimensions.");
        }
    }
    let shape: Vec<usize> = inner.iter().zip(reps).map(|(n, r)| n * r).collect();
    let mut out = Tensor::new(shape.clone());
    let dims = shape.len();
    for flat in 0..out.size() {
        let mut rem = flat;
        let mut block = 0;
        let mut local = Vec::with_capacity(dims);
        let mut block_multi = Vec::with_capacity(dims);
        for (axis, &inner_n) in inner.iter().enumerate() {
            let stride: usize = shape[(axis + 1)..].iter().product();
            let i = rem / stride;
            rem %= stride;
            block_multi.push(i / inner_n);
            local.push(i % inner_n);
        }
        for (axis, &b) in block_multi.iter().enumerate() {
            block = block * reps[axis] + b;
        }
        out.bytes_mut()[flat] = cells[block].types.get(&local);
    }
    Ok(Cell::new(out))
}

pub fn magic(cells: &[Cell]) -> Result<Cell> {
    if cells.len() < 2 {
        return value_error("Magic composition requires at least two cells.");
    }
    let mut out = cells[0].combine(&cells[1]);
    for cell in &cells[2..] {
        out = out.combine(cell);
    }
    Ok(out)
}

pub fn mosaic(mask: &Tensor, cells: &[Cell]) -> Result<Cell> {
    let picked: Result<Vec<Cell>> = mask
        .bytes()
        .iter()
        .map(|&i| match cells.get(i as usize) {
            Some(cell) => Ok(cell.clone()),
            None => value_error(format!("mosaic index {i} out of range.")),
        })
        .collect();
    merge(&picked?, &mask.shape)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atoms;
    #[test]
    fn rot90_map_matches_tensor() {
        let t = Tensor::of((0..24).map(|v| v as u8).collect(), vec![2, 3, 4]);
        for axes in [(0, 1), (0, 2), (1, 2)] {
            for k in 0..5 {
                let rotated = t.rot90(k, axes);
                let map = rot90_map(&t.shape, k, axes);
                let mapped: Vec<u8> = map.iter().map(|&s| t.bytes()[s]).collect();
                assert_eq!(mapped, rotated.bytes());
            }
        }
    }
    #[test]
    fn merge_two_by_two() {
        let a = Cell::new(atoms::ones_2d(2));
        let b = Cell::new(atoms::zeros_2d(2));
        let m = merge(&[a.clone(), b.clone(), b, a], &[2, 2]).unwrap();
        assert_eq!(m.types.shape, vec![4, 4]);
        assert_eq!(m.types.sum(), 8);
        assert_eq!(m.types.get(&[0, 0]), 1);
        assert_eq!(m.types.get(&[0, 2]), 0);
        assert_eq!(m.types.get(&[2, 0]), 0);
        assert_eq!(m.types.get(&[3, 3]), 1);
    }
    #[test]
    fn mosaic_picks_cells() {
        let a = Cell::new(atoms::ones_2d(2));
        let b = Cell::new(atoms::zeros_2d(2));
        let mask = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]);
        let m = mosaic(&mask, &[a, b]).unwrap();
        assert_eq!(m.types.sum(), 8);
        assert_eq!(m.types.get(&[0, 0]), 1);
        assert_eq!(m.types.get(&[0, 2]), 0);
    }
    #[test]
    fn paint_type_mode() {
        let cell = Cell::new(atoms::carpet_2d(3)).paint(&mapping(), Mode::Type);
        let colors = cell.colors.as_ref().unwrap();
        assert_eq!(colors[0], [0, 0, 0, 255]);
        assert_eq!(colors[4], [255, 255, 255, 255]);
        let dark = cell
            .types
            .bytes()
            .iter()
            .zip(colors)
            .filter(|(&t, _)| t == 1)
            .count();
        assert_eq!(dark, 8);
    }
    #[test]
    fn magic_is_kron_chain() {
        let a = Cell::new(atoms::carpet_2d(2));
        let b = Cell::new(atoms::ones_2d(3));
        let m = magic(&[a.clone(), b]).unwrap();
        assert_eq!(m.types.shape, vec![6, 6]);
        assert_eq!(m.types.sum(), a.types.sum() * 9);
    }
    #[test]
    fn binarize_clears_colors_and_thresholds() {
        let cell = Cell::new(atoms::carpet_2d(3))
            .paint(&mapping(), Mode::Type)
            .binarize(1);
        assert!(cell.colors.is_none());
        assert_eq!(cell.types.bytes(), atoms::carpet_2d(3).bytes());
    }
    #[test]
    fn blur_and_perforate_wrappers_delegate_to_tensor() {
        let cell = Cell::new(atoms::carpet_2d(3));
        let mask = Tensor::full(vec![3, 3], 1);
        let blurred = cell.clone().blur(&mask, true).unwrap();
        assert_eq!(blurred.types.shape, cell.types.shape);
        let perforated = cell.clone().perforate(&Tensor::new(vec![3, 3]), 9).unwrap();
        assert_eq!(perforated.types, cell.types);
    }
}
