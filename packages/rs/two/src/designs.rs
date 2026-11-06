use super::Cell2d;
use mrly_error::{Result, value_error};
use mrly_math::atoms;
use mrly_math::catalog::Design;
use mrly_math::code::Code;
use mrly_math::factory;
use mrly_rand::Rng;
use mrly_tensor::Tensor;

pub use mrly_math::factory::levels_code;

fn build(pattern: Tensor, level: usize, rotation: usize) -> Result<Cell2d> {
    let mut cell = mrly_math::cell::grow::<2>(pattern, level)?;
    if rotation != 0 {
        cell = cell.rotate(rotation)?;
    }
    Ok(cell)
}

pub fn create(
    code: Code,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<Cell2d> {
    build(factory::create(code, number, 2, base, 1)?, level, rotation)
}

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

pub fn level_set(
    number: usize,
    levels: &[usize],
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<Cell2d> {
    create(levels_code(2, base, levels), number, level, rotation, base)
}

pub fn named(design: Design, number: usize, level: usize, rotation: usize) -> Result<Cell2d> {
    let pattern = match design {
        Design::Carpet => atoms::carpet_2d(number),
        Design::Net => atoms::net_2d(number),
        Design::Htree => atoms::htree_2d(number),
        Design::Vtree => atoms::vtree_2d(number),
        Design::Void => atoms::void_2d(number),
        Design::Point => atoms::point_2d(number),
        Design::Dust => atoms::dust_2d(number),
        Design::Hline => atoms::hline_2d(number),
        Design::Vline => atoms::vline_2d(number),
        Design::Star => atoms::star_2d(number),
        other => return value_error(format!("design {} is not 2d.", other.name())),
    };
    build(pattern, level, rotation)
}

pub fn zeros(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::zeros_2d(number), level, 0)
}

pub fn ones(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::ones_2d(number), level, 0)
}

pub fn noise(number: usize, level: usize, density: f64, rng: &mut Rng) -> Result<Cell2d> {
    build(atoms::noise_2d(number, density, rng), level, 0)
}

pub fn carpet(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::carpet_2d(number), level, 0)
}

pub fn net(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::net_2d(number), level, 0)
}

pub fn htree(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::htree_2d(number), level, 0)
}

pub fn vtree(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::vtree_2d(number), level, 0)
}

pub fn void(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::void_2d(number), level, 0)
}

pub fn point(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::point_2d(number), level, 0)
}

pub fn dust(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::dust_2d(number), level, 0)
}

pub fn hline(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::hline_2d(number), level, 0)
}

pub fn vline(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::vline_2d(number), level, 0)
}

pub fn star(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::star_2d(number), level, 0)
}
