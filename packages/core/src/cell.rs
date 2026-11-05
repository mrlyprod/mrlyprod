use super::colors::{ALPHA, BLACK, BLUE, Color, GREEN, RED, WHITE};
use super::error::{Result, shape_error, value_error};
use super::rng::Rng;
use super::tensor::{Dtype, Tensor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Type,
    Tag,
    Index,
    Enumerate,
    Random,
    Row,
    Column,
    Depth,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        self.colors
            .as_ref()
            .and_then(|colors| colors.get(flat))
            .copied()
            .unwrap_or([0, 0, 0, 0])
    }
    pub fn rgba(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.size() * 4);
        for flat in 0..self.size() {
            let color = self
                .colors
                .as_ref()
                .and_then(|colors| colors.get(flat))
                .copied()
                .unwrap_or([0, 0, 0, 255]);
            out.extend_from_slice(&color);
        }
        out
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
    pub fn rotate(self, k: usize, axes: (usize, usize)) -> Result<Cell> {
        let map = rot90_map(&self.types.shape, k, axes)?;
        let mut shape = self.types.shape.clone();
        if k % 2 == 1 {
            shape.swap(axes.0, axes.1);
        }
        remap(&self, &map, &shape)
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
    pub fn tile(self, reps: &[usize]) -> Result<Cell> {
        let shape: Vec<usize> = self
            .types
            .shape
            .iter()
            .zip(reps)
            .map(|(n, r)| n * r)
            .collect();
        remap(&self, &tile_map(&self.types.shape, reps)?, &shape)
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
    pub fn paint(
        mut self,
        mapping: &HashMap<u8, Vec<Color>>,
        mode: Mode,
        rng: Option<&mut Rng>,
    ) -> Result<Cell> {
        let mut stream = rng;
        if mode == Mode::Random && stream.is_none() {
            return value_error("a random paint wants a random stream.");
        }
        let size = self.size();
        let mut colors = self
            .colors
            .take()
            .filter(|colors| colors.len() == size)
            .unwrap_or_else(|| vec![[0, 0, 0, 0]; size]);
        let mut keys: Vec<u8> = mapping.keys().copied().collect();
        keys.sort_unstable();
        for key in keys {
            let rgba: Vec<[u8; 4]> = mapping[&key].iter().map(|c| [c.r, c.g, c.b, c.a]).collect();
            if rgba.is_empty() {
                continue;
            }
            let mut enumerated = 0;
            for (flat, slot) in colors.iter_mut().enumerate() {
                if self.types.at(flat) != key as i64 {
                    continue;
                }
                let pick = match mode {
                    Mode::Type => 0,
                    Mode::Enumerate => {
                        let i = enumerated;
                        enumerated += 1;
                        i % rgba.len()
                    }
                    Mode::Random => match stream.as_deref_mut() {
                        Some(rng) => rng.below(rgba.len()),
                        None => 0,
                    },
                    Mode::Index => flat % rgba.len(),
                    Mode::Tag => match &self.tags {
                        Some(tags) if tags.size() == size => tags.at(flat) as usize % rgba.len(),
                        _ => 0,
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
                *slot = rgba[pick];
            }
        }
        self.colors = Some(colors);
        Ok(self)
    }
}

fn axis_index(t: &Tensor, flat: usize, axis: usize) -> usize {
    let mut stride = 1;
    for a in (axis + 1)..t.shape.len() {
        stride *= t.shape[a];
    }
    (flat / stride) % t.shape[axis]
}

pub fn tile_map(shape: &[usize], reps: &[usize]) -> Result<Vec<usize>> {
    if reps.len() != shape.len() {
        return shape_error(format!(
            "tile wants one count per axis, got {} for rank {}.",
            reps.len(),
            shape.len()
        ));
    }
    let tiled: Vec<usize> = shape.iter().zip(reps).map(|(n, r)| n * r).collect();
    let size = tiled.iter().product();
    let mut map = Vec::with_capacity(size);
    for flat in 0..size {
        let mut rem = flat;
        let mut source = 0;
        for (axis, &n) in shape.iter().enumerate() {
            let stride: usize = tiled[(axis + 1)..].iter().product();
            let i = rem / stride;
            rem %= stride;
            source = source * n + i % n;
        }
        map.push(source);
    }
    Ok(map)
}

pub fn remap(cell: &Cell, map: &[usize], shape: &[usize]) -> Result<Cell> {
    let size: usize = shape.iter().product();
    if map.len() != size {
        return shape_error(format!(
            "remap holds {} sources, shape {shape:?} wants {size}.",
            map.len()
        ));
    }
    if map.iter().any(|&src| src >= cell.size()) {
        return shape_error("remap source index is past the cell.");
    }
    Ok(Cell {
        types: gather(&cell.types, map, shape),
        colors: cell
            .colors
            .as_ref()
            .filter(|colors| colors.len() == cell.size())
            .map(|colors| map.iter().map(|&src| colors[src]).collect()),
        tags: cell
            .tags
            .as_ref()
            .filter(|tags| tags.size() == cell.size())
            .map(|tags| gather(tags, map, shape)),
    })
}

fn gather(source: &Tensor, map: &[usize], shape: &[usize]) -> Tensor {
    let mut out = Tensor::typed(shape.to_vec(), source.dtype());
    for (flat, &src) in map.iter().enumerate() {
        out.put(flat, source.at(src));
    }
    out
}

pub fn moore(dimension: usize) -> Tensor {
    let mut mask = Tensor::full(vec![3; dimension], 1);
    let mut center = 0;
    for _ in 0..dimension {
        center = center * 3 + 1;
    }
    mask.put(center, 0);
    mask
}

pub fn rot90_map(shape: &[usize], k: usize, axes: (usize, usize)) -> Result<Vec<usize>> {
    let (a, b) = axes;
    if a >= shape.len() || b >= shape.len() {
        return value_error(format!("axes {axes:?} are past the rank {}.", shape.len()));
    }
    if a == b {
        return value_error(format!("rot90 needs two distinct axes, got {axes:?}."));
    }
    let mut data: Vec<usize> = (0..shape.iter().product()).collect();
    let mut shape = shape.to_vec();
    for _ in 0..k % 4 {
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
    Ok(data)
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
    if reps.len() != inner.len() {
        return shape_error(format!(
            "merge wants one count per axis, got {} for rank {}.",
            reps.len(),
            inner.len()
        ));
    }
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
        let source = &cells[block].types;
        out.put(flat, source.at(source.index(&local)));
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
    let picked: Result<Vec<Cell>> = (0..mask.size())
        .map(|flat| {
            let i = mask.at(flat);
            match usize::try_from(i).ok().and_then(|i| cells.get(i)) {
                Some(cell) => Ok(cell.clone()),
                None => value_error(format!("mosaic index {i} out of range.")),
            }
        })
        .collect();
    merge(&picked?, &mask.shape)
}
