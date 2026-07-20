use super::models::Cell2d;
use crate::core::atoms;
use crate::core::errors::Result;
use crate::core::state;
use crate::core::tensor::Tensor;
use crate::math::bang::factory;
use crate::math::bang::universe::Code;

fn build(pattern: Tensor, level: usize, rotation: usize) -> Result<Cell2d> {
    let mut cell = crate::math::dim::grow::<2>(pattern, level)?;
    if rotation != 0 {
        cell = cell.rotate(rotation);
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

pub fn zeros(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::zeros_2d(number), level, 0)
}

pub fn ones(number: usize, level: usize) -> Result<Cell2d> {
    build(atoms::ones_2d(number), level, 0)
}

pub fn noise(number: usize, level: usize, density: f64) -> Result<Cell2d> {
    build(atoms::noise_2d(number, density), level, 0)
}

pub fn random(number: usize, level: usize, base: usize) -> Result<Cell2d> {
    let total = factory::total_codes(2, base);
    let code = state::randint(0, (total - 1) as i64) as Code;
    create(code, number, level, 0, base)
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
        let _g = crate::core::state::guard();
        crate::core::state::seed(99);
        let a = random(4, 1, 2).unwrap();
        crate::core::state::seed(99);
        let b = random(4, 1, 2).unwrap();
        assert_eq!(a, b);
    }
}
