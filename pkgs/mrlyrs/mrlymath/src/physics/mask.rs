use crate::two::{self, Cell2d};
use mrlycore::atoms;
use mrlycore::errors::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mask {
    cell: Cell2d,
}

impl Mask {
    pub fn build(
        design: &str,
        number: usize,
        level: usize,
        padding: usize,
        subpixel: usize,
        invert: bool,
    ) -> Result<Mask> {
        let base = base_design(design, number, level)?;
        let up = subpixel.max(1);

        let upscaled = base.types().kron(&atoms::ones_2d(up));
        let mut cell = Cell2d::new(upscaled);

        if invert {
            cell = cell.invert();
        }
        if padding > 0 {
            cell = cell.pad(padding, 0);
        }

        Ok(Mask { cell })
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.cell.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.cell.height()
    }

    #[inline]
    pub fn solid(&self, x: f32, y: f32) -> bool {
        if x < 0.0 || y < 0.0 {
            return true;
        }
        let ix = x.floor() as usize;
        let iy = y.floor() as usize;
        if ix >= self.width() || iy >= self.height() {
            return true;
        }
        self.cell.types().get(&[iy, ix]) == 1
    }

    #[inline]
    pub fn cell(&self) -> &Cell2d {
        &self.cell
    }

    #[cfg(test)]
    pub fn open(w: usize, h: usize) -> Mask {
        use mrlycore::tensor::Tensor;
        Mask {
            cell: Cell2d::new(Tensor::new(vec![h, w])),
        }
    }
}

fn base_design(design: &str, number: usize, level: usize) -> Result<Cell2d> {
    match design {
        "carpet" => two::carpet(number, level),
        "net" => two::net(number, level),
        "htree" | "tree" => two::htree(number, level),
        "vtree" => two::vtree(number, level),
        "void" => two::void(number, level),
        _ => two::carpet(number, level),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_match_upscale_and_pad() {
        let base = two::carpet(3, 1).unwrap();
        assert_eq!(base.width(), 3);
        let m = Mask::build("carpet", 3, 1, 4, 2, false).unwrap();
        assert_eq!(m.width(), 14);
        assert_eq!(m.height(), 14);
    }

    #[test]
    fn out_of_bounds_is_wall() {
        let m = Mask::build("carpet", 3, 1, 0, 1, false).unwrap();
        assert!(m.solid(-1.0, 0.0));
        assert!(m.solid(0.0, -1.0));
        assert!(m.solid(m.width() as f32, 0.0));
        assert!(m.solid(0.0, m.height() as f32));
    }

    #[test]
    fn padding_is_open_void_even_when_inverted() {
        let m = Mask::build("carpet", 3, 1, 3, 1, true).unwrap();
        assert!(!m.solid(0.0, 0.0));
        assert!(!m.solid((m.width() - 1) as f32, (m.height() - 1) as f32));
    }

    #[test]
    fn invert_flips_interior() {
        let plain = Mask::build("carpet", 3, 1, 0, 1, false).unwrap();
        let inv = Mask::build("carpet", 3, 1, 0, 1, true).unwrap();
        assert_eq!(plain.width(), inv.width());
        let mut differ = 0;
        for y in 0..plain.height() {
            for x in 0..plain.width() {
                if plain.solid(x as f32, y as f32) != inv.solid(x as f32, y as f32) {
                    differ += 1;
                }
            }
        }
        assert_eq!(differ, plain.width() * plain.height());
    }

    #[test]
    fn upscale_is_nearest_neighbor_blocks() {
        let m = Mask::build("carpet", 3, 1, 0, 2, false).unwrap();
        for by in 0..3 {
            for bx in 0..3 {
                let v = m.solid((bx * 2) as f32, (by * 2) as f32);
                assert_eq!(v, m.solid((bx * 2 + 1) as f32, (by * 2) as f32));
                assert_eq!(v, m.solid((bx * 2) as f32, (by * 2 + 1) as f32));
                assert_eq!(v, m.solid((bx * 2 + 1) as f32, (by * 2 + 1) as f32));
            }
        }
    }
}
