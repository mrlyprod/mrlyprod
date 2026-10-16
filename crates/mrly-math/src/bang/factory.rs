use super::code::Code;
use crate::name::{Bang, Named};
use crate::rules;
use mrly_core::Tensor;
use mrly_core::error::{Result, overflow_error, value_error};
use mrly_core::rng::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// One ordered layer of a magic composition: a coded design at its own side number.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MagicLayer {
    /// The layer's coded design.
    pub design: Bang,
    /// The layer's side number.
    pub number: usize,
}

impl MagicLayer {
    /// Pins a design to the side number it renders at.
    pub fn new(design: Bang, number: usize) -> MagicLayer {
        MagicLayer { design, number }
    }
}

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
///
/// # Errors
///
/// Errors when the corner count reaches the hundred and twenty-eight bits a u128 code holds.
pub fn total_codes(dimension: usize, base: usize) -> Result<Code> {
    let cells = base.pow(dimension as u32);
    if cells >= 128 {
        return overflow_error(format!(
            "dimension {dimension} base {base} has {cells} corners, past the 127 a u128 code holds."
        ));
    }
    Ok(Code(1 << cells))
}

/// Unpacks a code into its filled residue corners.
///
/// # Errors
///
/// Errors when the code names a corner the dimension and base do not hold.
pub fn code_to_corners(code: Code, dimension: usize, base: usize) -> Result<Vec<Vec<u8>>> {
    let cells = residue_corners(dimension, base);
    let code = code.get();
    if cells.len() >= 128 {
        return overflow_error(format!(
            "dimension {dimension} base {base} has {} corners, past the 127 a u128 code holds.",
            cells.len()
        ));
    }
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
/// assert_eq!(mrly_math::bang::factory::levels_code(3, 2, &[0, 1]).get(), 23);
/// assert_eq!(mrly_math::bang::factory::levels_code(2, 2, &[0, 1]).get(), 7);
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
    Code(
        cells
            .iter()
            .enumerate()
            .filter(|(_, c)| wanted.contains(c))
            .map(|(i, _)| 1 << i)
            .sum(),
    )
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
    let tile = rules::render(filled, number, dimension, base)?;
    Ok(tile.fractal(level))
}

/// Renders a coded design to a tensor at its side number, dimension, base and fractal level.
///
/// ```
/// use mrly_math::bang::Code;
/// let menger = mrly_math::bang::factory::create(Code::from(23u64), 3, 3, 2, 1).unwrap();
/// assert_eq!(menger.shape, vec![3, 3, 3]);
/// assert_eq!(menger.sum(), 20);
/// ```
///
/// # Errors
///
/// Errors when the code is out of range for the dimension and base.
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

/// Renders a design from its canonical JSON name.
///
/// # Errors
///
/// Errors for JSON that is not a bang name.
pub fn create_named(spec: &str, number: usize, level: usize) -> Result<Tensor> {
    let bang = Bang::from_json(spec)?;
    create(Code::from(bang.code), number, bang.dim, bang.base, level)
}

/// Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost.
///
/// A run of one repeated layer is the ordinary self-similar fractal level.
///
/// ```
/// use mrly_math::bang::{magic, MagicLayer};
/// use mrly_math::name::Bang;
/// let carpet = MagicLayer::new(Bang::new(7, 2, 2), 3);
/// let net = MagicLayer::new(Bang::new(14, 2, 2), 7);
/// let void = MagicLayer::new(Bang::new(9, 2, 2), 5);
/// assert_eq!(magic(&[carpet, net, void]).unwrap().shape, vec![105, 105]);
/// ```
///
/// # Errors
///
/// Errors below two layers, across dimensions, or on a code out of range.
pub fn magic(layers: &[MagicLayer]) -> Result<Tensor> {
    if layers.len() < 2 {
        return value_error("magic needs at least two layers.");
    }
    let dimension = layers[0].design.dim;
    if layers.iter().any(|l| l.design.dim != dimension) {
        return value_error("magic layers must share one dimension.");
    }
    let mut out: Option<Tensor> = None;
    for layer in layers {
        let next = create(
            Code::from(layer.design.code),
            layer.number,
            dimension,
            layer.design.base,
            1,
        )?;
        out = Some(match out {
            Some(tile) => tile.kron(&next),
            None => next,
        });
    }
    Ok(out.expect("two or more layers leave a tile"))
}

/// Composes JSON-named layers in order.
///
/// # Errors
///
/// Errors for an object that is not a bang name, or layers that do not compose.
pub fn magic_named(layers: &[(&str, usize)]) -> Result<Tensor> {
    let parsed: Result<Vec<MagicLayer>> = layers
        .iter()
        .map(|(spec, number)| Ok(MagicLayer::new(Bang::from_json(spec)?, *number)))
        .collect();
    magic(&parsed?)
}

/// Renders a design straight from its filled residue corners.
///
/// # Errors
///
/// Errors when a corner does not hold one residue per axis.
pub fn create_from_corners(
    filled: &[Vec<u8>],
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor> {
    render(filled, number, dimension, base, level)
}

/// Returns the deepest level, never below one, whose grid of number to the dimension per level stays within the cell budget.
///
/// ```
/// assert_eq!(mrly_math::bang::level_cap(3, 2, 60000), 5);
/// ```
pub fn level_cap(number: usize, dimension: usize, budget: usize) -> usize {
    if number < 2 || dimension == 0 {
        return 1;
    }
    let mut level = 1;
    while dimension
        .checked_mul(level + 1)
        .and_then(|power| u32::try_from(power).ok())
        .and_then(|power| (number as u128).checked_pow(power))
        .is_some_and(|cells| cells <= budget as u128)
    {
        level += 1;
    }
    level
}

/// Rolls a code uniformly from the nonempty codes of a dimension and base that fill at least min_fills corners.
///
/// # Errors
///
/// Errors when the corners overflow a code, or min_fills is more than the corners.
pub fn random_code(dimension: usize, base: usize, rng: &mut Rng, min_fills: usize) -> Result<Code> {
    total_codes(dimension, base)?;
    let corners = base.pow(dimension as u32);
    let least = min_fills.max(1);
    if least > corners {
        return value_error(format!(
            "dimension {dimension} base {base} has {corners} corners, fewer than {least} fills."
        ));
    }
    let mut row = vec![0u128; corners + 1];
    row[0] = 1;
    for n in 1..=corners {
        for k in (1..=n).rev() {
            row[k] += row[k - 1];
        }
    }
    let total: u128 = row[least..].iter().sum();
    let width = 128 - (total - 1).leading_zeros();
    let mut draw = loop {
        let draw = (0..width).fold(0u128, |draw, i| draw | u128::from(rng.boolean()) << i);
        if draw < total {
            break draw;
        }
    };
    let mut fills = least;
    while draw >= row[fills] {
        draw -= row[fills];
        fills += 1;
    }
    Ok(Code(
        rng.sample_indices(corners, fills)
            .into_iter()
            .fold(0, |code, i| code | 1 << i),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menger_carpet_code() {
        assert_eq!(levels_code(3, 2, &[0, 1]), Code(23));
        let truth = create(Code(23), 3, 3, 2, 1).unwrap();
        assert_eq!(
            create_named(r#"{"kind":"bang","dim":3,"code":23}"#, 3, 1).unwrap(),
            truth
        );
        assert_eq!(truth.sum(), 20);
        assert_eq!(truth.shape, vec![3, 3, 3]);
    }
    #[test]
    fn menger_holds_its_pinned_bytes() {
        let truth = create(Code(23), 3, 3, 2, 1).unwrap();
        let pinned = vec![
            1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
        ];
        assert_eq!(truth.bytes().unwrap(), pinned);
    }
    #[test]
    fn create_named_takes_the_canonical_name_only() {
        assert!(create_named(r#"{"kind":"bang","dim":2,"base":3,"code":100}"#, 3, 1).is_ok());
        for bad in ["bang dim 3, code 23", "bang_dim=3_code=23", "{}", "23"] {
            assert!(create_named(bad, 3, 1).is_err(), "{bad}");
        }
    }
    #[test]
    fn code_corner_round_trip() {
        for d in 2..=3 {
            for code in [
                Code(0),
                Code(1),
                Code(7),
                Code(total_codes(d, 2).unwrap().get() - 1),
            ] {
                let filled = code_to_corners(code, d, 2).unwrap();
                assert_eq!(corners_to_code(&filled, d, 2), code);
            }
        }
    }
    #[test]
    fn all_3d_codes_render() {
        for bits in 0..256 {
            let code = Code(bits);
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
    #[test]
    fn magic_recovers_the_self_similar_level() {
        let carpet = MagicLayer::new(Bang::new(7, 2, 2), 3);
        assert_eq!(
            magic(&[carpet.clone(), carpet.clone(), carpet]).unwrap(),
            create(Code(7), 3, 2, 2, 3).unwrap()
        );
    }
    #[test]
    fn magic_composes_mixed_designs_in_order() {
        let carpet = MagicLayer::new(Bang::new(7, 2, 2), 3);
        let net = MagicLayer::new(Bang::new(14, 2, 2), 7);
        let void = MagicLayer::new(Bang::new(9, 2, 2), 5);
        let got = magic(&[carpet, net, void]).unwrap();
        let expected = create(Code(7), 3, 2, 2, 1)
            .unwrap()
            .kron(&create(Code(14), 7, 2, 2, 1).unwrap())
            .kron(&create(Code(9), 5, 2, 2, 1).unwrap());
        assert_eq!(got, expected);
        assert_eq!(got.shape, vec![105, 105]);
        assert_eq!(got.sum(), 8 * 33 * 13);
    }
    #[test]
    fn magic_accepts_mixed_bases_and_canonical_names() {
        let got = magic_named(&[
            (r#"{"kind":"bang","dim":2,"code":7}"#, 3),
            (r#"{"kind":"bang","dim":2,"base":3,"code":98}"#, 3),
        ])
        .unwrap();
        let expected = magic(&[
            MagicLayer::new(Bang::new(7, 2, 2), 3),
            MagicLayer::new(Bang::new(98, 2, 3), 3),
        ])
        .unwrap();
        assert_eq!(got, expected);
        assert_eq!(got.shape, vec![9, 9]);
    }
    #[test]
    fn refuses_a_code_past_its_range_and_a_word_of_too_few_or_mismatched_layers() {
        let plane = MagicLayer::new(Bang::new(7, 2, 2), 3);
        let cube = MagicLayer::new(Bang::new(23, 3, 2), 3);
        assert!(code_to_corners(Code(16), 2, 2).is_err());
        assert!(code_to_corners(Code(100), 2, 2).is_err());
        assert!(code_to_corners(Code(1), 7, 2).is_err());
        assert!(total_codes(7, 2).is_err());
        assert!(magic(&[]).is_err());
        assert!(magic(std::slice::from_ref(&plane)).is_err());
        assert!(magic(&[plane, cube]).is_err());
    }
    #[test]
    fn the_level_cap_keeps_the_grown_cells_under_the_budget() {
        let cap = level_cap;
        assert_eq!([cap(3, 2, 60000), cap(3, 3, 60000)], [5, 3]);
        let sheet = |number, copies: usize| cap(number, 2, 262144 / copies);
        assert_eq!(
            [sheet(3, 25), sheet(3, 1), sheet(7, 144), sheet(5, 16)],
            [4, 5, 1, 3]
        );
        let six = |number, copies: usize| cap(number, 2, 32000 / (6 * copies));
        assert_eq!([six(3, 1), six(5, 1), six(7, 1), six(9, 1)], [3, 2, 2, 1]);
        assert_eq!(
            [six(3, 7), six(5, 7), six(7, 7), six(3, 19), six(3, 61)],
            [3, 2, 1, 2, 2]
        );
        assert_eq!([cap(2, 2, 65536), cap(3, 2, 65536)], [8, 5]);
        for number in [2usize, 3, 4, 5] {
            let level = cap(number, 3, 60000) as u32;
            assert!(number.pow(3 * level) <= 60000 && number.pow(3 * (level + 1)) > 60000);
        }
        assert_eq!([cap(1, 2, 10), cap(9, 3, 10)], [1, 1]);
    }
    #[test]
    fn a_rolled_code_fills_its_least_corners_and_one_seed_rolls_one_code() {
        for seed in 0..50 {
            let code = random_code(2, 3, &mut Rng::new(seed), 3).unwrap().get();
            assert!((1..512).contains(&code) && code.count_ones() >= 3, "{code}");
        }
        assert_eq!(
            random_code(2, 3, &mut Rng::new(9), 3).unwrap(),
            random_code(2, 3, &mut Rng::new(9), 3).unwrap()
        );
        assert!((1..16).contains(&random_code(2, 2, &mut Rng::new(1), 0).unwrap().get()));
        assert_eq!(
            random_code(3, 3, &mut Rng::new(1), 27).unwrap().get(),
            (1 << 27) - 1
        );
        assert!(random_code(2, 2, &mut Rng::new(1), 5).is_err());
        assert!(random_code(7, 2, &mut Rng::new(1), 1).is_err());
    }
}
