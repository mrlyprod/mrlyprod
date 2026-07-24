use super::universe::Code;
use crate::rules;
use mrlycore::errors::{value_error, Result};
use mrlycore::Tensor;
use std::collections::HashSet;

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

pub fn total_codes(dimension: usize, base: usize) -> Code {
    let cells = base.pow(dimension as u32);
    assert!(cells < 128, "too many cells for a u128 code");
    1 << cells
}

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

pub fn name(code: Code, dimension: usize, base: usize) -> String {
    format!("mrly_d{dimension}_b{base}_{code}")
}

pub fn parse_name(spec: &str) -> Result<(Code, Option<usize>, Option<usize>)> {
    let text = spec.trim().to_lowercase();
    if let Some(rest) = text.strip_prefix("mrly_d") {
        if let Some((d_part, tail)) = rest.split_once("_b") {
            if let Some((b_part, code_part)) = tail.split_once('_') {
                if let (Ok(d), Ok(b), Ok(code)) =
                    (d_part.parse(), b_part.parse(), code_part.parse())
                {
                    return Ok((code, Some(d), Some(b)));
                }
            }
        }
    }
    let bare = if let Some(rest) = text.strip_prefix("mrly") {
        rest.trim_start_matches('_')
    } else {
        &text
    };
    match bare.parse() {
        Ok(code) => Ok((code, None, None)),
        Err(_) => value_error(format!("cannot parse design spec {spec:?}.")),
    }
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

pub fn create_named(spec: &str, number: usize, level: usize) -> Result<Tensor> {
    let (code, dimension, base) = parse_name(spec)?;
    match (dimension, base) {
        (Some(d), Some(b)) => create(code, number, d, b, level),
        _ => {
            value_error("dimension is required for a bare code (use a mrly_d{D}_b{B}_{code} name).")
        }
    }
}

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
    fn levels_code(dimension: usize, levels: &[usize]) -> Code {
        let cells = residue_corners(dimension, 2);
        let filled: Vec<Vec<u8>> = cells
            .into_iter()
            .filter(|c| levels.contains(&c.iter().map(|&b| b as usize).sum()))
            .collect();
        corners_to_code(&filled, dimension, 2)
    }
    #[test]
    fn menger_carpet_code() {
        assert_eq!(levels_code(3, &[0, 1]), 23);
        assert_eq!(name(23, 3, 2), "mrly_d3_b2_23");
        let truth = create(23, 3, 3, 2, 1).unwrap();
        assert_eq!(create_named("mrly_d3_b2_23", 3, 1).unwrap(), truth);
        assert_eq!(truth.sum(), 20);
        assert_eq!(truth.shape, vec![3, 3, 3]);
    }
    #[test]
    fn menger_matches_python_exactly() {
        let truth = create(23, 3, 3, 2, 1).unwrap();
        let python = vec![
            1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
        ];
        assert_eq!(truth.bytes(), python);
    }
    #[test]
    fn parse_names() {
        assert_eq!(parse_name("mrly_023").unwrap(), (23, None, None));
        assert_eq!(parse_name("mrly23").unwrap(), (23, None, None));
        assert_eq!(parse_name("mrly_d3_b2_23").unwrap(), (23, Some(3), Some(2)));
        assert_eq!(parse_name("mrly_d2_b3_0").unwrap(), (0, Some(2), Some(3)));
        assert_eq!(parse_name("23").unwrap(), (23, None, None));
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
        let code = levels_code(3, &[0, 1]);
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
