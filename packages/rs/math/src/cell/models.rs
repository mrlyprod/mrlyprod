use crate::cell::base::Cell;
use crate::cell::base::Mode;
use mrly_color::Color;
use mrly_error::{Result, shape_error, value_error};
use mrly_rand::Rng;
use mrly_tensor::{Dtype, Tensor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

pub type Cell2d = CellNd<2>;
pub type Cell3d = CellNd<3>;

pub fn dtype_for(peak: i64) -> Dtype {
    if peak <= Dtype::U8.max() {
        Dtype::U8
    } else if peak <= Dtype::U16.max() {
        Dtype::U16
    } else {
        Dtype::U32
    }
}

pub fn counting_dtype(mask: &Tensor) -> Dtype {
    dtype_for(mask.sum() as i64)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellNd<const N: usize> {
    pub cell: Cell,
}

#[derive(Serialize)]
struct PlainRef<'a> {
    cell: &'a Cell,
}

#[derive(Deserialize)]
struct Plain {
    cell: Cell,
}

impl<const N: usize> Serialize for CellNd<N> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        if mrly_wire::on() {
            self.cell.serialize(s)
        } else {
            PlainRef { cell: &self.cell }.serialize(s)
        }
    }
}

impl<'de, const N: usize> Deserialize<'de> for CellNd<N> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        use serde::de::Error as _;
        let cell = if mrly_wire::on() {
            Cell::deserialize(d)?
        } else {
            Plain::deserialize(d)?.cell
        };
        if cell.types.shape.len() != N {
            return Err(D::Error::custom(format!(
                "a {N}d cell wants a shape of {N} lengths."
            )));
        }
        Ok(CellNd { cell })
    }
}

impl<const N: usize> CellNd<N> {
    pub fn new(types: Tensor) -> Result<CellNd<N>> {
        if types.shape.len() != N {
            return shape_error(format!(
                "a {N}d cell needs a {N}d tensor, got {}d.",
                types.shape.len()
            ));
        }
        Ok(CellNd {
            cell: Cell::new(types),
        })
    }
    pub fn width(&self) -> usize {
        let shape = &self.cell.types.shape;
        if N == 3 {
            shape[0]
        } else {
            shape[shape.len() - 1]
        }
    }
    pub fn height(&self) -> usize {
        let shape = &self.cell.types.shape;
        shape[shape.len() - 2]
    }
    pub fn types(&self) -> &Tensor {
        &self.cell.types
    }
    pub fn invert(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.invert(),
        }
    }
    pub fn anti(self) -> CellNd<N> {
        self.invert()
    }
    pub fn pad(self, count: usize, value: u8) -> CellNd<N> {
        CellNd {
            cell: self.cell.pad(count, value),
        }
    }
    pub fn fractal(self, level: usize) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.fractal(level)?,
        })
    }
    pub fn layers(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.layers(Dtype::U8),
        }
    }
    pub fn neighbors(self, mask: &Tensor, target: u8, wrap: bool) -> Result<CellNd<N>> {
        let dtype = counting_dtype(mask);
        Ok(CellNd {
            cell: self.cell.neighbors(mask, target, wrap, dtype)?,
        })
    }
    pub fn binarize(self, threshold: u8) -> CellNd<N> {
        CellNd {
            cell: self.cell.binarize(threshold),
        }
    }
    pub fn binarize_otsu(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.binarize_otsu(),
        }
    }
    pub fn blur(self, mask: &Tensor, wrap: bool) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.blur(mask, wrap)?,
        })
    }
    pub fn perforate(self, mask: &Tensor, value: u8) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.perforate(mask, value)?,
        })
    }
    pub fn combine(&self, other: &CellNd<N>) -> CellNd<N> {
        CellNd {
            cell: self.cell.combine(&other.cell),
        }
    }
    pub fn paint(
        self,
        mapping: &HashMap<u8, Vec<Color>>,
        mode: Mode,
        rng: Option<&mut Rng>,
    ) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.paint(mapping, mode, rng)?,
        })
    }
}

impl CellNd<2> {
    pub fn rotate(self, k: usize) -> Result<Cell2d> {
        Ok(CellNd {
            cell: self.cell.rotate(k, (0, 1))?,
        })
    }
    pub fn tile(self, width: usize, height: usize) -> Result<Cell2d> {
        Ok(CellNd {
            cell: self.cell.tile(&[height, width])?,
        })
    }
}

impl CellNd<3> {
    pub fn depth(&self) -> usize {
        self.cell.types.shape[2]
    }
    pub fn rotate(self, k: usize, axes: (usize, usize)) -> Result<Cell3d> {
        Ok(CellNd {
            cell: self.cell.rotate(k, axes)?,
        })
    }
    pub fn orient(self, index: usize) -> Result<Cell3d> {
        let table = orientations();
        match table.get(index) {
            Some(&(a, b, c)) => self.rotate(a, (1, 2))?.rotate(b, (0, 2))?.rotate(c, (0, 1)),
            None => value_error(format!("orientation index {index} out of range (0..23).")),
        }
    }
    pub fn tile(self, width: usize, height: usize, depth: usize) -> Result<Cell3d> {
        Ok(CellNd {
            cell: self.cell.tile(&[width, height, depth])?,
        })
    }
}

pub fn orientations() -> Vec<(usize, usize, usize)> {
    static TABLE: OnceLock<Vec<(usize, usize, usize)>> = OnceLock::new();
    TABLE
        .get_or_init(|| {
            let mut probe = Tensor::new(vec![3, 3, 3]);
            for flat in 0..probe.size() {
                probe.put(flat, flat as i64);
            }
            let mut seen: Vec<Vec<u8>> = Vec::new();
            let mut table = Vec::new();
            for a in 0..4 {
                for b in 0..4 {
                    for c in 0..4 {
                        let turned = probe
                            .rot90(a, (1, 2))
                            .and_then(|t| t.rot90(b, (0, 2)))
                            .and_then(|t| t.rot90(c, (0, 1)));
                        let Ok(image) = turned.and_then(|t| t.bytes().map(<[u8]>::to_vec)) else {
                            continue;
                        };
                        if !seen.contains(&image) {
                            seen.push(image);
                            table.push((a, b, c));
                        }
                    }
                }
            }
            table
        })
        .clone()
}
