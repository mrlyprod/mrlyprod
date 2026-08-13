use super::universe::Code;
use crate::name::Named;
use crate::rules;
use mrlycore::errors::{value_error, Result};
use mrlycore::Tensor;
use std::collections::HashSet;

/// Returns every base-q residue corner of a dimension in row-major order.
pub fn residue_corners(dimension: usize, base: usize) -> Vec<Vec<u8>> {
    let count = base.pow(dimension as u32);
    (0..count)
        .map(|i| {
            (0..dimension)
                .map(|j| ((i / base.pow((dimension - 1 - j) as u32)) % base) as u8)
                .collect()
        })
        .collect()
}

/// Returns the code count of a dimension and base, two to the number of corners.
pub fn total_codes(dimension: usize, base: usize) -> Code {
    let cells = base.pow(dimension as u32);
    assert!(cells < 128, "too many cells for a u128 code");
    1 << cells
}

/// Unpacks a code into its filled residue corners, or an error when the code is out of range.
pub fn code_to_corners(code: Code, dimension: usize, base: usize) -> Result<Vec<Vec<u8>>> {
    let cells = residue_corners(dimension, base);
    if code >= (1 << cells.len()) {
        return value_error(format!(
            "code {code} out of range for dimension {dimension} base {base} (0..{}).",
            (1u128 << cells.len()) - 1
        ));
    }
    Ok(cells
        .into_iter()
        .enumerate()
        .filter(|(i, _)| (code >> i) & 1 == 1)
        .map(|(_, c)| c)
        .collect())
}

/// Returns the code of the design filled wherever a corner's residue sum lands in the levels.
///
/// ```
/// assert_eq!(mrlymath::bang::factory::levels_code(3, 2, &[0, 1]), 23);
/// assert_eq!(mrlymath::bang::factory::levels_code(2, 2, &[0, 1]), 7);
/// ```
pub fn levels_code(dimension: usize, base: usize, levels: &[usize]) -> Code {
    let filled: Vec<Vec<u8>> = residue_corners(dimension, base)
        .into_iter()
        .filter(|corner| levels.contains(&corner.iter().map(|&b| b as usize).sum()))
        .collect();
    corners_to_code(&filled, dimension, base)
}

/// Packs filled residue corners back into their code.
pub fn corners_to_code(filled: &[Vec<u8>], dimension: usize, base: usize) -> Code {
    let cells = residue_corners(dimension, base);
    let wanted: HashSet<&Vec<u8>> = filled.iter().collect();
    cells
        .iter()
        .enumerate()
        .filter(|(_, c)| wanted.contains(c))
        .map(|(i, _)| 1 << i)
        .sum()
}

fn render(
    filled: &[Vec<u8>],
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor> {
    if level < 1 {
        return value_error("level must be at least 1.");
    }
    let wanted: HashSet<Vec<u8>> = filled.iter().cloned().collect();
    let tile = rules::render(|p| wanted.contains(p), number, dimension, base)?;
    Ok(tile.fractal(level))
}

/// Renders a coded design to a tensor at its side number, dimension, base and fractal level.
///
/// ```
/// let menger = mrlymath::bang::factory::create(23, 3, 3, 2, 1).unwrap();
/// assert_eq!(menger.shape, vec![3, 3, 3]);
/// assert_eq!(menger.sum(), 20);
/// ```
pub fn create(
    code: Code,
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor> {
    let filled = code_to_corners(code, dimension, base)?;
    render(&filled, number, dimension, base, level)
}

/// Renders a design from its canonical mrly name, or an error for any other spelling.
pub fn create_named(spec: &str, number: usize, level: usize) -> Result<Tensor> {
    let bang = crate::name::Bang::from_str(spec)?;
    create(bang.code, number, bang.dimension, bang.base, level)
}

/// Renders a design straight from its filled residue corners.
pub fn create_from_corners(
    filled: &[Vec<u8>],
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor> {
    render(filled, number, dimension, base, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menger_carpet_code() {
        assert_eq!(levels_code(3, 2, &[0, 1]), 23);
        let truth = create(23, 3, 3, 2, 1).unwrap();
        assert_eq!(create_named("mrly_bang_d3_23", 3, 1).unwrap(), truth);
        assert_eq!(truth.sum(), 20);
        assert_eq!(truth.shape, vec![3, 3, 3]);
    }
    #[test]
    fn menger_holds_its_pinned_bytes() {
        let truth = create(23, 3, 3, 2, 1).unwrap();
        let pinned = vec![
            1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
        ];
        assert_eq!(truth.bytes(), pinned);
    }
    #[test]
    fn create_named_takes_the_canonical_name_only() {
        assert!(create_named("mrly_bang_d2_q3_100", 3, 1).is_ok());
        for bad in ["mrly_d3_b2_23", "mrly_023", "mrly23", "23"] {
            assert!(create_named(bad, 3, 1).is_err(), "{bad}");
        }
    }
    #[test]
    fn code_corner_round_trip() {
        for d in 2..=3 {
            for code in [0, 1, 7, total_codes(d, 2) - 1] {
                let filled = code_to_corners(code, d, 2).unwrap();
                assert_eq!(corners_to_code(&filled, d, 2), code);
            }
        }
    }
    #[test]
    fn out_of_range_rejected() {
        assert!(code_to_corners(16, 2, 2).is_err());
        assert!(code_to_corners(100, 2, 2).is_err());
    }
    #[test]
    fn all_3d_codes_render() {
        for code in 0..256 {
            let arr = create(code, 3, 3, 2, 1).unwrap();
            assert_eq!(arr.shape, vec![3, 3, 3]);
            let filled = code_to_corners(code, 3, 2).unwrap();
            assert_eq!(
                arr.sum(),
                create_from_corners(&filled, 3, 3, 2, 1).unwrap().sum()
            );
        }
    }
    #[test]
    fn fractal_level() {
        let code = levels_code(3, 2, &[0, 1]);
        let base = create(code, 3, 3, 2, 1).unwrap();
        let lvl3 = create(code, 3, 3, 2, 3).unwrap();
        assert_eq!(lvl3.sum(), base.sum().pow(3));
        assert_eq!(lvl3.shape, vec![27, 27, 27]);
    }
    #[test]
    fn base3_has_more_corners() {
        assert_eq!(residue_corners(3, 2).len(), 8);
        assert_eq!(residue_corners(3, 3).len(), 27);
    }
}
