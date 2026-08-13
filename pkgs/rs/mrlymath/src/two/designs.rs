use super::models::Cell2d;
use crate::bang::factory;
use crate::bang::universe::Code;
use mrlycore::atoms;
use mrlycore::errors::{value_error, Result};
use mrlycore::state;
use mrlycore::tensor::Tensor;
use mrlycore::tile::Design;

pub use crate::bang::factory::levels_code;

fn build(pattern: Tensor, level: usize, rotation: usize) -> Result<Cell2d> {
    let mut cell = crate::dim::grow::<2>(pattern, level)?;
    if rotation != 0 {
        cell = cell.rotate(rotation);
    }
    Ok(cell)
}

/// Builds the design a universe code names, deepened to the level and rotated by quarter-turns.
pub fn create(
    code: Code,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<Cell2d> {
    build(factory::create(code, number, 2, base, 1)?, level, rotation)
}

/// Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns.
pub fn from_corners(
    corners: &[Vec<u8>],
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<Cell2d> {
    build(
        factory::create_from_corners(corners, number, 2, base, 1)?,
        level,
        rotation,
    )
}

/// Builds the level-set design, filling every residue corner whose digits sum to a named level.
///
/// ```
/// let n = 5;
/// assert_eq!(
///     mrlymath::two::level_set(n, &[0, 1], 1, 0, 2).unwrap(),
///     mrlymath::two::carpet(n, 1).unwrap()
/// );
/// ```
pub fn level_set(
    number: usize,
    levels: &[usize],
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<Cell2d> {
    create(levels_code(2, base, levels), number, level, rotation, base)
}

/// Builds the design the name picks, deepened to the level and rotated by quarter-turns.
pub fn named(design: Design, number: usize, level: usize, rotation: usize) -> Result<Cell2d> {
    let pattern = match design {
        Design::Carpet => atoms::carpet_2d(number),
        Design::Net => atoms::net_2d(number),
        Design::Htree => atoms::htree_2d(number),
        Design::Vtree => atoms::vtree_2d(number),
        Design::Void => atoms::void_2d(number),
        other => return value_error(format!("design {} is not 2d.", other.name())),
    };
    build(pattern, level, rotation)
}

/// Builds an all-empty cell of the given size and level.
pub fn zeros(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::zeros_2d(number), level, 0)
}

/// Builds an all-filled cell of the given size and level.
pub fn ones(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::ones_2d(number), level, 0)
}

/// Draws a noise cell whose sites fill with the given probability, deepened to the level.
pub fn noise(number: usize, level: usize, density: f64) -> Result<Cell2d> {
    build(atoms::noise_2d(number, density), level, 0)
}

/// Draws a random universe code and builds its design at the given size and level.
pub fn random(number: usize, level: usize, base: usize) -> Result<Cell2d> {
    let total = factory::total_codes(2, base);
    let code = state::randint(0, (total - 1) as i64) as Code;
    create(code, number, level, 0, base)
}

/// Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level.
pub fn carpet(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::carpet_2d(number), level, 0)
}

/// Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level.
pub fn net(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::net_2d(number), level, 0)
}

/// Builds the htree fractal, its seed striped along even rows, deepened to the level.
pub fn htree(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::htree_2d(number), level, 0)
}

/// Builds the vtree fractal, its seed striped along even columns, deepened to the level.
pub fn vtree(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::vtree_2d(number), level, 0)
}

/// Builds the void fractal, its seed a checkerboard on even parity, deepened to the level.
pub fn void(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::void_2d(number), level, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn carpet_fractal_growth() {
        let c = carpet(3, 3).unwrap();
        assert_eq!(c.width(), 27);
        assert_eq!(c.types().sum(), carpet(3, 1).unwrap().types().sum().pow(3));
    }
    #[test]
    fn create_matches_carpet() {
        let by_name = carpet(5, 2).unwrap();
        let by_code = create(7, 5, 2, 0, 2).unwrap();
        assert_eq!(by_name, by_code);
    }
    #[test]
    fn random_is_seeded() {
        let _g = mrlycore::state::guard();
        mrlycore::state::seed(99);
        let a = random(4, 1, 2).unwrap();
        mrlycore::state::seed(99);
        let b = random(4, 1, 2).unwrap();
        assert_eq!(a, b);
    }
    #[test]
    fn level_sets_reproduce_the_symmetric_designs() {
        for n in 1..8 {
            assert_eq!(
                level_set(n, &[0, 1], 1, 0, 2).unwrap(),
                carpet(n, 1).unwrap()
            );
            assert_eq!(level_set(n, &[1, 2], 1, 0, 2).unwrap(), net(n, 1).unwrap());
            assert_eq!(level_set(n, &[0, 2], 1, 0, 2).unwrap(), void(n, 1).unwrap());
        }
        assert_eq!(level_set(4, &[], 1, 0, 2).unwrap(), zeros(4, 1).unwrap());
        assert_eq!(
            level_set(4, &[0, 1, 2], 1, 0, 2).unwrap(),
            ones(4, 1).unwrap()
        );
    }
    #[test]
    fn level_sets_carry_level_and_rotation() {
        let deep = level_set(3, &[0, 1], 2, 0, 2).unwrap();
        assert_eq!(deep, carpet(3, 2).unwrap());
        let turned = level_set(5, &[1, 2], 1, 1, 2).unwrap();
        assert_eq!(turned, net(5, 1).unwrap().rotate(1));
    }
    #[test]
    fn named_builders_take_a_rotation() {
        for (design, plain) in [
            (Design::Carpet, carpet(5, 1).unwrap()),
            (Design::Net, net(5, 1).unwrap()),
            (Design::Htree, htree(5, 1).unwrap()),
            (Design::Vtree, vtree(5, 1).unwrap()),
            (Design::Void, void(5, 1).unwrap()),
        ] {
            assert_eq!(named(design, 5, 1, 0).unwrap(), plain);
            for k in 1..4 {
                assert_eq!(named(design, 5, 1, k).unwrap(), plain.clone().rotate(k));
            }
        }
        assert_eq!(named(Design::Vtree, 5, 1, 1).unwrap(), htree(5, 1).unwrap());
        assert!(named(Design::Ztree, 5, 1, 0).is_err());
    }
}
