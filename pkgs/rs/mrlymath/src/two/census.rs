use super::models::Cell2d;
use crate::dim::census;

/// One reading of a cell: its fills, voids and perimeter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Census {
    /// The count of filled sites.
    pub fills: usize,
    /// The count of empty sites.
    pub voids: usize,
    /// The count of filled faces open to emptiness or the border.
    pub perimeter: u128,
}

/// Counts the filled sites of the cell.
pub fn fills(cell: &Cell2d) -> usize {
    census::fills(cell)
}

/// Counts the empty sites of the cell.
pub fn voids(cell: &Cell2d) -> usize {
    census::voids(cell)
}

/// Counts the faces of filled sites open to emptiness or the border.
pub fn perimeter(cell: &Cell2d) -> u128 {
    census::exposure(cell)
}

/// Takes the cell's full census in one reading.
///
/// ```
/// let cell = mrlymath::two::carpet(3, 1).unwrap();
/// let census = mrlymath::two::census::census(&cell);
/// assert_eq!((census.fills, census.voids, census.perimeter), (8, 1, 16));
/// ```
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
    use crate::formulas;
    use crate::two::designs;
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
