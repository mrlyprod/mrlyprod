use super::models::Cell3d;
use crate::bang::factory;
use crate::bang::universe::Code;
use mrlycore::atoms;
use mrlycore::errors::Result;
use mrlycore::state;
use mrlycore::tensor::Tensor;

fn build(pattern: Tensor, level: usize) -> Result<Cell3d> {
    crate::dim::grow::<3>(pattern, level)
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

pub fn noise(number: usize, level: usize, density: f64) -> Result<Cell3d> {
    build(atoms::noise_3d(number, density), level)
}

pub fn random(number: usize, level: usize, base: usize) -> Result<Cell3d> {
    let total = factory::total_codes(3, base);
    let code = state::randint(0, (total - 1) as i64) as Code;
    create(code, number, level, base)
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
    fn trees_are_orientations_of_each_other() {
        let x = xtree(3, 1).unwrap();
        let z = ztree(3, 1).unwrap();
        let images: Vec<Vec<u8>> = (0..24)
            .map(|i| x.clone().orient(i).unwrap().types().bytes().to_vec())
            .collect();
        assert!(images.contains(&z.types().bytes().to_vec()));
    }
}
