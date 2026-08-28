use super::models::Cell3d;
use crate::bang::factory;
use crate::bang::universe::Code;
use mrlycore::atoms;
use mrlycore::errors::{value_error, Result};
use mrlycore::state;
use mrlycore::tensor::Tensor;
use mrlycore::tile::Design;

pub use crate::bang::factory::levels_code;

fn build(pattern: Tensor, level: usize) -> Result<Cell3d> {
    crate::dim::grow::<3>(pattern, level)
}

/// Builds the cube the universe code names, deepened to the given fractal level.
pub fn create(code: Code, number: usize, level: usize, base: usize) -> Result<Cell3d> {
    build(factory::create(code, number, 3, base, 1)?, level)
}

/// Builds a cube from its corner patterns, deepened to the given fractal level.
pub fn from_corners(
    corners: &[Vec<u8>],
    number: usize,
    level: usize,
    base: usize,
) -> Result<Cell3d> {
    build(
        factory::create_from_corners(corners, number, 3, base, 1)?,
        level,
    )
}

/// Builds the all-void cube at the given size and level.
pub fn zeros(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::zeros_3d(number), level)
}

/// Builds the solid cube at the given size and level.
pub fn ones(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ones_3d(number), level)
}

/// Builds a random cube of the given density at the given size and level.
pub fn noise(number: usize, level: usize, density: f64) -> Result<Cell3d> {
    build(atoms::noise_3d(number, density), level)
}

/// Draws a random universe code and builds its cube.
pub fn random(number: usize, level: usize, base: usize) -> Result<Cell3d> {
    let total = factory::total_codes(3, base);
    let code = state::randint(0, (total - 1) as i64) as Code;
    create(code, number, level, base)
}

/// Builds the Menger sponge, filled where at most one coordinate is odd, at the given level.
///
/// ```
/// let sponge = mrlymath::three::carpet(3, 1).unwrap();
/// assert_eq!(sponge.types().sum(), 20);
/// ```
pub fn carpet(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::carpet_3d(number), level)
}

/// Builds the net cube, filled where at least two coordinates are odd, at the given level.
pub fn net(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::net_3d(number), level)
}

/// Builds the cube of beams along the x axis at the given size and level.
pub fn xtree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::xtree_3d(number), level)
}

/// Builds the cube of beams along the y axis at the given size and level.
pub fn ytree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ytree_3d(number), level)
}

/// Builds the cube of beams along the z axis at the given size and level.
pub fn ztree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ztree_3d(number), level)
}

/// Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.
pub fn void(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::void_3d(number), level)
}

// LEVEL SET

/// Builds the cube filled wherever the residue sum lands in the levels, at the given level.
///
/// Carpet, net and void are the three presets of this one engine: the levels are all a
/// symmetric cube design ever names.
///
/// ```
/// let sponge = mrlymath::three::level_set(3, &[0, 1], 1, 2).unwrap();
/// assert_eq!(sponge, mrlymath::three::carpet(3, 1).unwrap());
/// ```
pub fn level_set(number: usize, levels: &[usize], level: usize, base: usize) -> Result<Cell3d> {
    create(levels_code(3, base, levels), number, level, base)
}

// NAMED

/// Builds the cube the name picks, deepened to the given fractal level.
pub fn named(design: Design, number: usize, level: usize) -> Result<Cell3d> {
    let pattern = match design {
        Design::Carpet => atoms::carpet_3d(number),
        Design::Net => atoms::net_3d(number),
        Design::Xtree => atoms::xtree_3d(number),
        Design::Ytree => atoms::ytree_3d(number),
        Design::Ztree => atoms::ztree_3d(number),
        Design::Void => atoms::void_3d(number),
        other => return value_error(format!("design {} is not 3d.", other.name())),
    };
    build(pattern, level)
}

/// Draws one of the six named cube designs and builds it at the given size and level.
pub fn random_classic(number: usize, level: usize) -> Result<Cell3d> {
    let classics = mrlycore::tile::classics(3);
    let pick = state::randint(0, classics.len() as i64 - 1) as usize;
    named(classics[pick], number, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn carpet_is_menger() {
        let c = carpet(3, 1).unwrap();
        assert_eq!(c.types().sum(), 20);
        assert_eq!(carpet(3, 2).unwrap().types().sum(), 400);
        assert_eq!(create(23, 3, 1, 2).unwrap(), c);
    }
    #[test]
    fn the_level_sets_name_the_symmetric_three() {
        for (levels, preset) in [
            (vec![0, 1], carpet(3, 2).unwrap()),
            (vec![2, 3], net(3, 2).unwrap()),
            (vec![0, 3], void(3, 2).unwrap()),
        ] {
            assert_eq!(level_set(3, &levels, 2, 2).unwrap(), preset);
        }
        assert_eq!(levels_code(3, 2, &[0, 1]), 23);
        assert_eq!(level_set(3, &[], 1, 2).unwrap().types().sum(), 0);
        assert_eq!(
            level_set(3, &[0, 1, 2, 3], 1, 2).unwrap(),
            ones(3, 1).unwrap()
        );
    }
    #[test]
    fn level_sets_take_a_wider_base() {
        let corners: Vec<Vec<u8>> = factory::residue_corners(3, 3)
            .into_iter()
            .filter(|corner| corner.iter().map(|&b| b as usize).sum::<usize>() <= 1)
            .collect();
        let by_hand = from_corners(&corners, 3, 1, 3).unwrap();
        assert_eq!(level_set(3, &[0, 1], 1, 3).unwrap(), by_hand);
    }
    #[test]
    fn the_named_builders_answer_to_the_classics() {
        for (design, plain) in [
            (Design::Carpet, carpet(3, 1).unwrap()),
            (Design::Net, net(3, 1).unwrap()),
            (Design::Xtree, xtree(3, 1).unwrap()),
            (Design::Ytree, ytree(3, 1).unwrap()),
            (Design::Ztree, ztree(3, 1).unwrap()),
            (Design::Void, void(3, 1).unwrap()),
        ] {
            assert_eq!(named(design, 3, 1).unwrap(), plain);
        }
        assert!(named(Design::Htree, 3, 1).is_err());
    }
    #[test]
    fn random_classic_draws_one_of_the_named_six() {
        let _g = state::guard();
        state::seed(7);
        let a = random_classic(3, 1).unwrap();
        state::seed(7);
        assert_eq!(random_classic(3, 1).unwrap(), a);
        let classics: Vec<Cell3d> = mrlycore::tile::classics(3)
            .into_iter()
            .map(|design| named(design, 3, 1).unwrap())
            .collect();
        assert!(classics.contains(&a));
    }
    #[test]
    fn trees_are_orientations_of_each_other() {
        let x = xtree(3, 1).unwrap();
        let z = ztree(3, 1).unwrap();
        let images: Vec<Vec<u8>> = (0..24)
            .map(|i| x.clone().orient(i).unwrap().types().bytes().to_vec())
            .collect();
        assert!(images.contains(&z.types().bytes().to_vec()));
    }
}
