use super::colors::{Color, ALPHA, BLACK, BLUE, GREEN, RED, WHITE};
use super::error::{shape_error, value_error, Result};
use super::tensor::{Dtype, Tensor};
use std::collections::HashMap;

/// The ways paint picks a color within a type's palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// The first palette color, always.
    Type,
    /// The color each cell's tag indexes.
    Tag,
    /// The color the flat position indexes.
    Index,
    /// The colors cycled in encounter order.
    Enumerate,
    /// The color the row index picks.
    Row,
    /// The color the column index picks.
    Column,
    /// The color the depth index picks.
    Depth,
}

/// A grid of type bytes with optional per-cell colors and tags.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The type of every cell.
    pub types: Tensor,
    /// The painted rgba of every cell, once painted.
    pub colors: Option<Vec<[u8; 4]>>,
    /// The tag layer over the cells, once built.
    pub tags: Option<Tensor>,
}

/// Returns the default mapping of the first six types to white, black, alpha, red, green, and blue.
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
    /// Wraps a tensor of types in a bare cell, colorless and tagless.
    pub fn new(types: Tensor) -> Cell {
        Cell {
            types,
            colors: None,
            tags: None,
        }
    }
    /// Returns the shape of the type tensor.
    pub fn shape(&self) -> &[usize] {
        &self.types.shape
    }
    /// Returns the number of cells.
    pub fn size(&self) -> usize {
        self.types.size()
    }
    /// Returns the painted color at a flat index, or transparent while unpainted or past the end.
    pub fn color_at(&self, flat: usize) -> [u8; 4] {
        self.colors
            .as_ref()
            .and_then(|colors| colors.get(flat))
            .copied()
            .unwrap_or([0, 0, 0, 0])
    }
    /// Flips every type to one minus itself.
    pub fn invert(mut self) -> Cell {
        self.types = self.types.invert();
        self
    }
    /// Flips every type to one minus itself, same as invert.
    pub fn anti(self) -> Cell {
        self.invert()
    }
    /// Wraps the cell in count layers of value on every side, dropping colors.
    pub fn pad(mut self, count: usize, value: u8) -> Cell {
        self.types = self.types.pad(count, value);
        self.colors = None;
        self.tags = self.tags.map(|t| t.pad(count, value));
        self
    }
    /// Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along, or an error for axes off the cell.
    pub fn rotate(self, k: usize, axes: (usize, usize)) -> Result<Cell> {
        let map = rot90_map(&self.types.shape, k, axes)?;
        let mut shape = self.types.shape.clone();
        if k % 2 == 1 {
            shape.swap(axes.0, axes.1);
        }
        remap(&self, &map, &shape)
    }
    /// Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags.
    pub fn fractal(mut self, level: usize) -> Result<Cell> {
        if level < 1 {
            return value_error("Fractal level must be at least 1.");
        }
        self.types = self.types.fractal(level);
        self.colors = None;
        self.tags = None;
        Ok(self)
    }
    /// Repeats the cell reps times along each axis, carrying colors and tags along, or an error without one count per axis.
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
    /// Tags every cell with its concentric shell distance from the center.
    pub fn layers(mut self, dtype: Dtype) -> Cell {
        self.tags = Some(self.types.layers(dtype));
        self
    }
    /// Tags every cell with its count of target-valued neighbors under the mask.
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
    /// Maps every type to one at or above the threshold and zero below, dropping colors.
    pub fn binarize(mut self, threshold: u8) -> Cell {
        self.types = self.types.binarize(threshold);
        self.colors = None;
        self
    }
    /// Binarizes the types at Otsu's threshold, dropping colors.
    pub fn binarize_otsu(mut self) -> Cell {
        self.types = self.types.binarize_otsu();
        self.colors = None;
        self
    }
    /// Replaces every type with the rounded mean of its masked neighborhood, dropping colors.
    pub fn blur(mut self, mask: &Tensor, wrap: bool) -> Result<Cell> {
        self.types = self.types.blur(mask, wrap)?;
        self.colors = None;
        Ok(self)
    }
    /// Stamps value wherever the tiled mask is on, dropping colors.
    pub fn perforate(mut self, mask: &Tensor, value: u8) -> Result<Cell> {
        self.types = self.types.perforate(mask, value)?;
        self.colors = None;
        Ok(self)
    }
    /// Builds the Kronecker product of the two cells' types.
    pub fn combine(&self, other: &Cell) -> Cell {
        Cell::new(self.types.kron(&other.types))
    }
    /// Colors every mapped cell, picking within each type's palette by the mode.
    pub fn paint(mut self, mapping: &HashMap<u8, Vec<Color>>, mode: Mode) -> Cell {
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

/// Builds the flat source index of every destination cell after tiling reps copies per axis, or an error without one count per axis.
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

/// Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map, or an error when the map does not fit.
///
/// The map holds one source index per destination cell, so it must be as long as the shape's size and point inside the cell.
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

/// Builds the 3-wide Moore mask of the dimension, every site on but the center.
///
/// ```
/// let mask = mrlyrs::core::cell::moore(2);
/// assert_eq!(mask.shape, vec![3, 3]);
/// assert_eq!(mask.sum(), 8);
/// ```
pub fn moore(dimension: usize) -> Tensor {
    let mut mask = Tensor::full(vec![3; dimension], 1);
    let mut center = 0;
    for _ in 0..dimension {
        center = center * 3 + 1;
    }
    mask.put(center, 0);
    mask
}

/// Builds the flat source index of every destination cell after k quarter turns in the plane of the axes, or an error when the axes are not two distinct axes of the shape.
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

/// Stitches same-shaped cells into one grid of reps blocks per axis, or an error when counts or shapes disagree.
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

/// Folds at least two cells into one by chained Kronecker products.
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

/// Lays the cell each mask entry indexes into that entry's place and merges the lot.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::atoms;
    #[test]
    fn rot90_map_matches_tensor() {
        let t = Tensor::of((0..24).map(|v| v as u8).collect(), vec![2, 3, 4]).unwrap();
        for axes in [(0, 1), (0, 2), (1, 2)] {
            for k in 0..5 {
                let rotated = t.rot90(k, axes).unwrap();
                let map = rot90_map(&t.shape, k, axes).unwrap();
                let bytes = t.bytes().unwrap();
                let mapped: Vec<u8> = map.iter().map(|&s| bytes[s]).collect();
                assert_eq!(mapped, rotated.bytes().unwrap());
            }
        }
    }
    #[test]
    fn remap_carries_types_colors_and_tags() {
        let painted = Cell::new(atoms::carpet_2d(3))
            .layers(Dtype::U8)
            .paint(&mapping(), Mode::Type);
        let map = rot90_map(&painted.types.shape, 1, (0, 1)).unwrap();
        let turned = remap(&painted, &map, &[3, 3]).unwrap();
        assert_eq!(turned.types, painted.types.rot90(1, (0, 1)).unwrap());
        assert_eq!(
            turned.tags.as_ref().unwrap(),
            &painted.tags.as_ref().unwrap().rot90(1, (0, 1)).unwrap()
        );
        let colors = turned.colors.as_ref().unwrap();
        let source = painted.colors.as_ref().unwrap();
        for (flat, &src) in map.iter().enumerate() {
            assert_eq!(colors[flat], source[src], "at {flat}");
        }
    }
    #[test]
    fn remap_keeps_a_wide_tag_layer_wide() {
        let mut grid = Cell::new(atoms::ones_2d(2));
        grid.tags = Some(Tensor::filled(vec![2, 2], 300, Dtype::U16));
        let tiled = grid.tile(&[2, 2]).unwrap();
        let tags = tiled.tags.unwrap();
        assert_eq!(tags.dtype(), Dtype::U16);
        assert_eq!(tags.at(15), 300);
    }
    #[test]
    fn moore_masks_every_site_but_the_center() {
        let flat = moore(2);
        assert_eq!(flat.shape, vec![3, 3]);
        assert_eq!(flat.get(&[1, 1]).unwrap(), 0);
        assert_eq!(flat.sum(), 8);
        let cube = moore(3);
        assert_eq!(cube.shape, vec![3, 3, 3]);
        assert_eq!(cube.get(&[1, 1, 1]).unwrap(), 0);
        assert_eq!(cube.sum(), 26);
    }
    #[test]
    fn merge_two_by_two() {
        let a = Cell::new(atoms::ones_2d(2));
        let b = Cell::new(atoms::zeros_2d(2));
        let m = merge(&[a.clone(), b.clone(), b, a], &[2, 2]).unwrap();
        assert_eq!(m.types.shape, vec![4, 4]);
        assert_eq!(m.types.sum(), 8);
        assert_eq!(m.types.get(&[0, 0]).unwrap(), 1);
        assert_eq!(m.types.get(&[0, 2]).unwrap(), 0);
        assert_eq!(m.types.get(&[2, 0]).unwrap(), 0);
        assert_eq!(m.types.get(&[3, 3]).unwrap(), 1);
    }
    #[test]
    fn mosaic_picks_cells() {
        let a = Cell::new(atoms::ones_2d(2));
        let b = Cell::new(atoms::zeros_2d(2));
        let mask = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]).unwrap();
        let m = mosaic(&mask, &[a, b]).unwrap();
        assert_eq!(m.types.sum(), 8);
        assert_eq!(m.types.get(&[0, 0]).unwrap(), 1);
        assert_eq!(m.types.get(&[0, 2]).unwrap(), 0);
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
            .unwrap()
            .iter()
            .zip(colors)
            .filter(|(&t, _)| t == 1)
            .count();
        assert_eq!(dark, 8);
    }
    #[test]
    fn tile_carries_colors_and_tags() {
        let painted = Cell::new(atoms::carpet_2d(3))
            .layers(Dtype::U8)
            .paint(&mapping(), Mode::Type);
        let tiled = painted.clone().tile(&[2, 3]).unwrap();
        assert_eq!(tiled.types.shape, vec![6, 9]);
        let colors = tiled.colors.as_ref().unwrap();
        let source = painted.colors.as_ref().unwrap();
        assert_eq!(colors.len(), 54);
        for y in 0..6 {
            for x in 0..9 {
                assert_eq!(colors[y * 9 + x], source[(y % 3) * 3 + x % 3]);
            }
        }
        assert_eq!(tiled.tags.as_ref().unwrap().shape, vec![6, 9]);
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
        assert_eq!(
            cell.types.bytes().unwrap(),
            atoms::carpet_2d(3).bytes().unwrap()
        );
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
    #[test]
    fn refuses_rotate_and_tile() {
        let cell = Cell::new(atoms::carpet_2d(3));
        assert!(cell.clone().rotate(1, (0, 2)).is_err());
        assert!(cell.clone().rotate(1, (1, 1)).is_err());
        assert!(rot90_map(&[3, 3], 1, (0, 2)).is_err());
        assert!(rot90_map(&[3, 3], 1, (0, 0)).is_err());
        assert!(cell.clone().tile(&[2]).is_err());
        assert!(tile_map(&[3, 3], &[2, 2, 2]).is_err());
    }
    #[test]
    fn refuses_remap_merge_and_mosaic() {
        let cell = Cell::new(atoms::ones_2d(2));
        assert!(remap(&cell, &[0, 1, 2], &[2, 2]).is_err());
        assert!(remap(&cell, &[0, 1, 2, 4], &[2, 2]).is_err());
        assert!(merge(&[], &[1]).is_err());
        assert!(merge(&[cell.clone(), cell.clone()], &[2]).is_err());
        assert!(merge(&[cell.clone(), cell.clone()], &[2, 1, 1]).is_err());
        let odd = Cell::new(atoms::ones_2d(3));
        assert!(merge(&[cell.clone(), odd], &[2, 1]).is_err());
        let mask = Tensor::of(vec![0, 2, 0, 0], vec![2, 2]).unwrap();
        assert!(mosaic(&mask, std::slice::from_ref(&cell)).is_err());
        assert!(magic(&[cell]).is_err());
    }
}
