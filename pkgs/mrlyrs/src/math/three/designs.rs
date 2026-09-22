use super::Cell3d;
use crate::core::error::{value_error, Result};
use crate::core::rng::Rng;
use crate::core::tensor::Tensor;
use crate::math::atoms;
use crate::math::bang::catalog::Design;
use crate::math::bang::factory;
use crate::math::bang::Code;

pub use crate::math::bang::factory::levels_code;

fn build(pattern: Tensor, level: usize) -> Result<Cell3d> {
    crate::math::cell::grow::<3>(pattern, level)
}

/// Builds the cube the universe code names, deepened to the given fractal level.
///
/// # Errors
///
/// Errors when the code is out of range, or the level is below one.
pub fn create(code: Code, number: usize, level: usize, base: usize) -> Result<Cell3d> {
    build(factory::create(code, number, 3, base, 1)?, level)
}

/// Builds a cube from its corner patterns, deepened to the given fractal level.
///
/// # Errors
///
/// Errors when a corner does not fit the dimension and base, or the level is below one.
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
///
/// # Errors
///
/// Errors below level one.
pub fn zeros(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::zeros_3d(number), level)
}

/// Builds the solid cube at the given size and level.
///
/// # Errors
///
/// Errors below level one.
pub fn ones(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ones_3d(number), level)
}

/// Builds a cube whose every site turns on with probability density, at the given level.
///
/// # Errors
///
/// Errors below level one.
pub fn noise(number: usize, level: usize, density: f64, rng: &mut Rng) -> Result<Cell3d> {
    build(atoms::noise_3d(number, density, rng), level)
}

/// Builds the Menger sponge, filled where at most one coordinate is odd, at the given level.
///
/// ```
/// let sponge = mrlyrs::math::three::carpet(3, 1).unwrap();
/// assert_eq!(sponge.types().sum(), 20);
/// ```
///
/// # Errors
///
/// Errors below level one.
pub fn carpet(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::carpet_3d(number), level)
}

/// Builds the net cube, filled where at least two coordinates are odd, at the given level.
///
/// # Errors
///
/// Errors below level one.
pub fn net(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::net_3d(number), level)
}

/// Builds the cube of beams along the x axis at the given size and level.
///
/// # Errors
///
/// Errors below level one.
pub fn xtree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::xtree_3d(number), level)
}

/// Builds the cube of beams along the y axis at the given size and level.
///
/// # Errors
///
/// Errors below level one.
pub fn ytree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ytree_3d(number), level)
}

/// Builds the cube of beams along the z axis at the given size and level.
///
/// # Errors
///
/// Errors below level one.
pub fn ztree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ztree_3d(number), level)
}

/// Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.
///
/// # Errors
///
/// Errors below level one.
pub fn void(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::void_3d(number), level)
}

/// Builds the point cube, filled where every coordinate is odd, at the given level.
///
/// # Errors
///
/// Errors below level one.
pub fn point(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::point_3d(number), level)
}

/// Builds the dust cube, filled where every coordinate is even, at the given level.
///
/// # Errors
///
/// Errors below level one.
pub fn dust(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::dust_3d(number), level)
}

/// Builds the cube of rods along the x axis at the given size and level.
///
/// # Errors
///
/// Errors below level one.
pub fn xline(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::xline_3d(number), level)
}

/// Builds the cube of rods along the y axis at the given size and level.
///
/// # Errors
///
/// Errors below level one.
pub fn yline(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::yline_3d(number), level)
}

/// Builds the cube of rods along the z axis at the given size and level.
///
/// # Errors
///
/// Errors below level one.
pub fn zline(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::zline_3d(number), level)
}

/// Builds the star cube, filled where exactly one coordinate is odd, at the given level.
///
/// # Errors
///
/// Errors below level one.
pub fn star(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::star_3d(number), level)
}

// LEVEL SET

/// Builds the cube filled wherever the residue sum lands in the levels, at the given level.
///
/// Carpet, net and void are the three presets of this one engine: the levels are all a
/// symmetric cube design ever names.
///
/// ```
/// let sponge = mrlyrs::math::three::level_set(3, &[0, 1], 1, 2).unwrap();
/// assert_eq!(sponge, mrlyrs::math::three::carpet(3, 1).unwrap());
/// ```
///
/// # Errors
///
/// Errors below level one.
pub fn level_set(number: usize, levels: &[usize], level: usize, base: usize) -> Result<Cell3d> {
    create(levels_code(3, base, levels), number, level, base)
}

// NAMED

/// Builds the cube the name picks, deepened to the given fractal level.
///
/// # Errors
///
/// Errors for a design that is not 3d, or a level below one.
pub fn named(design: Design, number: usize, level: usize) -> Result<Cell3d> {
    let pattern = match design {
        Design::Carpet => atoms::carpet_3d(number),
        Design::Net => atoms::net_3d(number),
        Design::Xtree => atoms::xtree_3d(number),
        Design::Ytree => atoms::ytree_3d(number),
        Design::Ztree => atoms::ztree_3d(number),
        Design::Void => atoms::void_3d(number),
        Design::Point => atoms::point_3d(number),
        Design::Dust => atoms::dust_3d(number),
        Design::Xline => atoms::xline_3d(number),
        Design::Yline => atoms::yline_3d(number),
        Design::Zline => atoms::zline_3d(number),
        Design::Star => atoms::star_3d(number),
        other => return value_error(format!("design {} is not 3d.", other.name())),
    };
    build(pattern, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn noise_fills_a_cube_at_the_density() {
        let mut rng = Rng::new(1);
        assert_eq!(
            noise(5, 1, 0.5, &mut rng).unwrap().types().shape,
            vec![5, 5, 5]
        );
        let big = noise(27, 1, 0.5, &mut rng).unwrap();
        let fraction = big.types().sum() as f64 / big.types().size() as f64;
        assert!((fraction - 0.5).abs() < 0.05, "{fraction}");
    }

    #[test]
    fn carpet_is_menger() {
        let c = carpet(3, 1).unwrap();
        assert_eq!(c.types().sum(), 20);
        assert_eq!(carpet(3, 2).unwrap().types().sum(), 400);
        assert_eq!(create(Code(23), 3, 1, 2).unwrap(), c);
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
            (Design::Point, point(3, 1).unwrap()),
            (Design::Dust, dust(3, 1).unwrap()),
            (Design::Xline, xline(3, 1).unwrap()),
            (Design::Yline, yline(3, 1).unwrap()),
            (Design::Zline, zline(3, 1).unwrap()),
            (Design::Star, star(3, 1).unwrap()),
        ] {
            assert_eq!(named(design, 3, 1).unwrap(), plain);
        }
        assert!(named(Design::Htree, 3, 1).is_err());
        assert!(named(Design::Hline, 3, 1).is_err());
    }
    #[test]
    fn trees_are_orientations_of_each_other() {
        let x = xtree(3, 1).unwrap();
        let z = ztree(3, 1).unwrap();
        let images: Vec<Vec<u8>> = (0..24)
            .map(|i| {
                x.clone()
                    .orient(i)
                    .unwrap()
                    .types()
                    .bytes()
                    .unwrap()
                    .to_vec()
            })
            .collect();
        assert!(images.contains(&z.types().bytes().unwrap().to_vec()));
    }
}
