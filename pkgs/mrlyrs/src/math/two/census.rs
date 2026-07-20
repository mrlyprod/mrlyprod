use super::models::Cell2d;
use crate::math::dim::census;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Census {
    pub fills: usize,
    pub voids: usize,
    pub perimeter: u128,
}

pub fn fills(cell: &Cell2d) -> usize {
    census::fills(cell)
}

pub fn voids(cell: &Cell2d) -> usize {
    census::voids(cell)
}

pub fn perimeter(cell: &Cell2d) -> u128 {
    census::exposure(cell)
}

pub fn census(cell: &Cell2d) -> Census {
    Census {
        fills: fills(cell),
        voids: voids(cell),
        perimeter: perimeter(cell),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::formulas;
    use crate::math::two::designs;
    #[test]
    fn census_matches_formulas() {
        for code in [7u128, 14, 9, 5] {
            for level in 1..4u32 {
                let cell = designs::create(code, 3, level as usize, 0, 2).unwrap();
                assert_eq!(
                    fills(&cell) as u128,
                    formulas::fill(code, 3, 2, level, 2).unwrap(),
                    "code={code} l={level}"
                );
                assert_eq!(
                    voids(&cell) as u128,
                    formulas::void(code, 3, 2, level, 2).unwrap()
                );
            }
        }
    }
    #[test]
    fn carpet_perimeter() {
        let c = designs::carpet(3, 1).unwrap();
        assert_eq!(
            census(&c),
            Census {
                fills: 8,
                voids: 1,
                perimeter: 16
            }
        );
    }
}
