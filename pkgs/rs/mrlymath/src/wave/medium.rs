use super::{C_FAST, C_SLOW};
use crate::two::Cell2d;

/// A two-speed corridor: a design slab between a source pad and a receiver tail.
#[derive(Clone, Debug, PartialEq)]
pub struct Medium {
    /// The speed at every site, row-major.
    pub speed: Vec<f64>,
    /// The row count.
    pub height: usize,
    /// The column count.
    pub width: usize,
    /// The fast columns before the slab.
    pub pad: usize,
    /// The slab width in columns.
    pub slab: usize,
}

impl Medium {
    /// Builds an all-fast corridor of the given shape.
    pub fn free(height: usize, width: usize) -> Medium {
        Medium {
            speed: vec![C_FAST; height * width],
            height,
            width,
            pad: 0,
            slab: 0,
        }
    }
}

/// Stamps a mask into a corridor: sites of type one slow the slab, the pad and tail stay fast.
///
/// The mask blows up by the largest whole factor keeping its height at most slab_px.
pub fn medium(mask: &Cell2d, slab_px: usize, pad: usize, tail: usize) -> Medium {
    let side = mask.height();
    let factor = (slab_px / side).max(1);
    let height = mask.height() * factor;
    let slab = mask.width() * factor;
    let width = slab + pad + tail;
    let mut speed = vec![C_FAST; height * width];
    for r in 0..height {
        for c in 0..slab {
            if mask.types().get(&[r / factor, c / factor]) == 1 {
                speed[r * width + pad + c] = C_SLOW;
            }
        }
    }
    Medium {
        speed,
        height,
        width,
        pad,
        slab,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two;

    #[test]
    fn carpet_stamps_slow_sites_between_fast_ends() {
        let mask = two::carpet(3, 1).unwrap();
        let m = medium(&mask, 3, 4, 5);
        assert_eq!((m.height, m.width, m.pad, m.slab), (3, 12, 4, 3));
        assert_eq!(m.speed[4], C_SLOW);
        assert_eq!(m.speed[m.width + 4 + 1], C_FAST);
        assert_eq!(m.speed[0], C_FAST);
        assert_eq!(m.speed[m.width - 1], C_FAST);
    }

    #[test]
    fn the_mask_blows_up_to_the_slab_size() {
        let mask = two::carpet(3, 1).unwrap();
        let m = medium(&mask, 8, 2, 2);
        assert_eq!((m.height, m.slab), (6, 6));
        assert_eq!(m.speed[2], C_SLOW);
        assert_eq!(m.speed[2 * m.width + 2 + 2], C_FAST);
    }

    #[test]
    fn a_free_corridor_is_all_fast() {
        let m = Medium::free(4, 7);
        assert_eq!(m.speed.len(), 28);
        assert!(m.speed.iter().all(|&s| s == C_FAST));
    }
}
