use super::Cell3d;
use mrly_error::{Result, value_error};
use mrly_math::atoms;
use mrly_math::catalog::Design;
use mrly_math::code::Code;
use mrly_math::factory;
use mrly_rand::Rng;
use mrly_tensor::Tensor;

pub use mrly_math::factory::levels_code;

fn build(pattern: Tensor, level: usize) -> Result<Cell3d> {
    mrly_math::cell::grow::<3>(pattern, level)
}

pub fn create(code: Code, number: usize, level: usize, base: usize) -> Result<Cell3d> {
    build(factory::create(code, number, 3, base, 1)?, level)
}

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

pub fn zeros(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::zeros_3d(number), level)
}

pub fn ones(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ones_3d(number), level)
}

pub fn noise(number: usize, level: usize, density: f64, rng: &mut Rng) -> Result<Cell3d> {
    build(atoms::noise_3d(number, density, rng), level)
}

pub fn carpet(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::carpet_3d(number), level)
}

pub fn net(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::net_3d(number), level)
}

pub fn xtree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::xtree_3d(number), level)
}

pub fn ytree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ytree_3d(number), level)
}

pub fn ztree(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::ztree_3d(number), level)
}

pub fn void(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::void_3d(number), level)
}

pub fn point(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::point_3d(number), level)
}

pub fn dust(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::dust_3d(number), level)
}

pub fn xline(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::xline_3d(number), level)
}

pub fn yline(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::yline_3d(number), level)
}

pub fn zline(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::zline_3d(number), level)
}

pub fn star(number: usize, level: usize) -> Result<Cell3d> {
    build(atoms::star_3d(number), level)
}

pub fn level_set(number: usize, levels: &[usize], level: usize, base: usize) -> Result<Cell3d> {
    create(levels_code(3, base, levels), number, level, base)
}

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
