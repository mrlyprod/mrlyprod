use super::models::Cell3d;
use crate::dim::census;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Census {
    pub fills: usize,
    pub voids: usize,
    pub surface: u128,
}

pub fn fills(cell: &Cell3d) -> usize {
    census::fills(cell)
}

pub fn voids(cell: &Cell3d) -> usize {
    census::voids(cell)
}

pub fn volume(cell: &Cell3d) -> usize {
    fills(cell)
}

pub fn surface(cell: &Cell3d) -> u128 {
    census::exposure(cell)
}

pub fn census(cell: &Cell3d) -> Census {
    Census {
        fills: fills(cell),
        voids: voids(cell),
        surface: surface(cell),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas;
    use crate::three::designs;
    #[test]
    fn census_matches_formulas() {
        for code in [23u128, 129, 17, 232] {
            for level in 1..3u32 {
                let cell = designs::create(code, 3, level as usize, 2).unwrap();
                assert_eq!(
                    fills(&cell) as u128,
                    formulas::fill(code, 3, 3, level, 2).unwrap()
                );
                assert_eq!(
                    surface(&cell),
                    formulas::surface(code, 3, level, 2).unwrap(),
                    "code={code} l={level}"
                );
            }
        }
    }
    #[test]
    fn menger_census() {
        let c = designs::carpet(3, 1).unwrap();
        let result = census(&c);
        assert_eq!(result.fills, 20);
        assert_eq!(result.voids, 7);
        assert_eq!(result.surface, 72);
    }
}
