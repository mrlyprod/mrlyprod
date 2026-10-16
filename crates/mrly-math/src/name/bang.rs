use super::Named;
use crate::bang::Code;
use crate::bang::catalog::{antis, classics};
use crate::bang::factory::{corners_to_code, residue_corners, total_codes};
use crate::kind;
use crate::{three, two};
use mrly_core::Tensor;
use mrly_core::error::{Result, value_error};
use serde::{Deserialize, Serialize};

kind!("bang");

/// The lattice the cells sit on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lattice {
    /// The square lattice, the default the name elides.
    #[default]
    Square,
    /// The hexagonal lattice.
    Hex,
}

impl Lattice {
    /// Returns whether this is the square lattice.
    pub fn is_square(&self) -> bool {
        matches!(self, Lattice::Square)
    }
    /// Returns the number of unit directions a twist may pick from.
    pub fn units(self) -> usize {
        match self {
            Lattice::Square => 4,
            Lattice::Hex => 6,
        }
    }
}

fn two() -> usize {
    2
}

fn is_two(base: &usize) -> bool {
    *base == 2
}

/// A design code pinned to its dimension, lattice and base, with one unit index per filled digit when it twists.
///
/// ```
/// use mrly_math::name::{Bang, Named};
/// let carpet = Bang::new(7, 2, 2);
/// assert_eq!(carpet.to_json(), r#"{"kind":"bang","dim":2,"code":7}"#);
/// assert_eq!(Bang::from_json(&carpet.to_json()).unwrap(), carpet);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bang {
    /// The kind word.
    pub kind: Kind,
    /// The number of axes.
    pub dim: usize,
    /// The lattice, square unless said.
    #[serde(default, skip_serializing_if = "Lattice::is_square")]
    pub lattice: Lattice,
    /// The digits per axis, 2 unless said.
    #[serde(default = "two", skip_serializing_if = "is_two")]
    pub base: usize,
    /// The design as a number.
    pub code: u128,
    /// One unit index per filled digit, absent when nothing turns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twist: Option<Vec<usize>>,
}

impl Bang {
    /// Pins a code to its dimension and base on the square lattice.
    pub fn new(code: u128, dim: usize, base: usize) -> Bang {
        Bang {
            kind: Kind,
            dim,
            lattice: Lattice::Square,
            base,
            code,
            twist: None,
        }
    }
    /// Returns the number of digits the code addresses.
    ///
    /// # Errors
    ///
    /// Errors below dimension one, below base two, or past the u128 code space.
    pub fn cells(&self) -> Result<u32> {
        if self.dim < 1 {
            return value_error("dim must be at least 1.");
        }
        if self.base < 2 {
            return value_error("base must be at least 2.");
        }
        match u32::try_from(self.base)
            .ok()
            .zip(u32::try_from(self.dim).ok())
            .and_then(|(b, d)| b.checked_pow(d))
        {
            Some(cells) if cells < 128 => Ok(cells),
            _ => value_error(format!(
                "dim {} base {} exceeds the u128 code space.",
                self.dim, self.base
            )),
        }
    }
}

impl Named for Bang {
    const KIND: &'static str = "bang";
    const LISTS: &'static [&'static str] = &["twist"];
    const BARE: &'static [&'static str] = &["lattice"];
    fn checked(mut self) -> Result<Bang> {
        let cells = self.cells()?;
        if self.code >> cells != 0 {
            return value_error(format!(
                "code {} out of range for dim {} base {} (0..{}).",
                self.code,
                self.dim,
                self.base,
                (1u128 << cells) - 1
            ));
        }
        if let Some(twist) = &self.twist {
            if twist.iter().all(|&unit| unit == 0) {
                self.twist = None;
            } else if twist.len() != self.code.count_ones() as usize {
                return value_error(format!(
                    "twist holds {} units for {} filled digits.",
                    twist.len(),
                    self.code.count_ones()
                ));
            } else if let Some(unit) = twist.iter().find(|&&unit| unit >= self.lattice.units()) {
                return value_error(format!(
                    "twist unit {unit} is not below the {} units of the lattice.",
                    self.lattice.units()
                ));
            }
        }
        Ok(self)
    }
}

/// Lists every classic then anti design of the plane or the cube as its decimal code and its word, empty for any other dimension.
///
/// ```
/// let words = mrly_math::name::words(2, 2);
/// assert_eq!(words[0], ("7".to_string(), "carpet".to_string()));
/// ```
pub fn words(dim: usize, base: usize) -> Vec<(String, String)> {
    if total_codes(dim, base).is_err() {
        return Vec::new();
    }
    let drawn = |types: &Tensor| {
        let filled: Vec<Vec<u8>> = residue_corners(dim, base)
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| types.at(i) != 0)
            .map(|(_, corner)| corner)
            .collect();
        corners_to_code(&filled, dim, base).to_string()
    };
    classics(dim)
        .into_iter()
        .chain(antis(dim))
        .filter_map(|design| {
            let code = match dim {
                2 => drawn(two::named(design, base, 1, 0).ok()?.types()),
                3 => drawn(three::named(design, base, 1).ok()?.types()),
                _ => return None,
            };
            Some((code, design.name().to_lowercase()))
        })
        .collect()
}

/// Returns the word of the first named design a code draws in the plane or the cube, or None for a code no named design draws.
///
/// ```
/// use mrly_math::bang::Code;
/// assert_eq!(mrly_math::name::of(Code::from(7u64), 2, 2).as_deref(), Some("carpet"));
/// ```
pub fn of(code: Code, dim: usize, base: usize) -> Option<String> {
    let code = code.to_string();
    words(dim, base)
        .into_iter()
        .find(|(listed, _)| *listed == code)
        .map(|(_, word)| word)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrly_core::rng::Rng;

    const KOCH: &str =
        r#"{"kind":"bang","dim":2,"lattice":"hex","base":3,"code":39,"twist":[0,1,5,0]}"#;

    fn koch() -> Bang {
        Bang {
            lattice: Lattice::Hex,
            twist: Some(vec![0, 1, 5, 0]),
            ..Bang::new(39, 2, 3)
        }
    }

    #[test]
    fn defaults_elide() {
        assert_eq!(
            Bang::new(7, 2, 2).to_json(),
            r#"{"kind":"bang","dim":2,"code":7}"#
        );
        assert_eq!(
            Bang::new(23, 3, 2).to_json(),
            r#"{"kind":"bang","dim":3,"code":23}"#
        );
        assert_eq!(
            Bang::new(0, 2, 3).to_json(),
            r#"{"kind":"bang","dim":2,"base":3,"code":0}"#
        );
        assert_eq!(koch().to_json(), KOCH);
    }
    #[test]
    fn canonical_names_parse() {
        assert_eq!(
            Bang::from_json(r#"{"kind":"bang","dim":2,"code":7}"#).unwrap(),
            Bang::new(7, 2, 2)
        );
        assert_eq!(
            Bang::from_json(r#"{"kind":"bang","dim":2,"base":3,"code":511}"#).unwrap(),
            Bang::new(511, 2, 3)
        );
        assert_eq!(Bang::from_json(KOCH).unwrap(), koch());
    }
    #[test]
    fn a_spelt_default_folds_to_the_canonical_string() {
        let spelt =
            r#"{"code":7, "base":2, "lattice":"square", "dim":2, "kind":"bang", "twist":[0,0,0]}"#;
        assert_eq!(Bang::from_json(spelt).unwrap(), Bang::new(7, 2, 2));
        assert_eq!(
            Bang::from_json(spelt).unwrap().to_json(),
            r#"{"kind":"bang","dim":2,"code":7}"#
        );
    }
    #[test]
    fn refuses_every_text_that_is_not_a_bang() {
        for bad in [
            r#"{"kind":"rule","dim":2,"code":7}"#,
            r#"{"dim":2,"code":7}"#,
            r#"{"kind":"bang","code":7}"#,
            r#"{"kind":"bang","dim":2}"#,
            r#"{"kind":"bang","dim":2,"code":16}"#,
            r#"{"kind":"bang","dim":0,"code":1}"#,
            r#"{"kind":"bang","dim":2,"base":1,"code":1}"#,
            r#"{"kind":"bang","dim":7,"code":1}"#,
            r#"{"kind":"bang","dim":2,"code":-1}"#,
            r#"{"kind":"bang","dim":2,"code":"7"}"#,
            r#"{"kind":"bang","dim":2,"code":7,"level":3}"#,
            r#"{"kind":"bang","dim":2,"code":7,"lattice":"cubic"}"#,
            r#"{"kind":"bang","dim":2,"code":7,"twist":[1,2]}"#,
            r#"{"kind":"bang","dim":2,"code":7,"twist":[1,2,4]}"#,
            r#"{"kind":"bang","dim":2,"lattice":"hex","code":7,"twist":[1,2,6]}"#,
            "bang dim 2, code 7",
            "bang_dim=2_code=7",
            "7",
        ] {
            assert!(Bang::from_json(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn a_code_past_u64_survives() {
        let wide = Bang::new(1u128 << 99, 1, 100);
        let text = wide.to_json();
        assert_eq!(
            text,
            format!(
                r#"{{"kind":"bang","dim":1,"base":100,"code":{}}}"#,
                1u128 << 99
            )
        );
        assert_eq!(Bang::from_json(&text).unwrap(), wide);
        assert_eq!(Bang::from_file(&wide.to_file().unwrap()).unwrap(), wide);
    }
    #[test]
    fn the_koch_row_holds_through_every_view() {
        let koch = koch();
        assert_eq!(
            koch.to_url().unwrap(),
            "/bang?dim=2&lattice=hex&base=3&code=39&twist=0,1,5,0"
        );
        assert_eq!(
            koch.to_file().unwrap(),
            "bang_dim=2_lattice=hex_base=3_code=39_twist=[0,1,5,0]"
        );
        assert_eq!(
            koch.to_prose().unwrap(),
            "bang dim 2, hex, base 3, code 39, twist [0 1 5 0]"
        );
        assert_eq!(Bang::from_url(&koch.to_url().unwrap()).unwrap(), koch);
        assert_eq!(Bang::from_file(&koch.to_file().unwrap()).unwrap(), koch);
        assert_eq!(koch.to_id().len(), 8);
        assert_ne!(koch.to_id(), Bang::new(39, 2, 3).to_id());
        assert_eq!(Bang::new(7, 2, 2).to_prose().unwrap(), "bang dim 2, code 7");
        assert_eq!(Bang::new(7, 2, 2).to_file().unwrap(), "bang_dim=2_code=7");
    }
    #[test]
    fn seeded_values_round_trip() {
        let mut rng = Rng::new(11);
        for _ in 0..500 {
            let base = *rng.choice(&[2usize, 3]).unwrap();
            let top = if base == 2 { 4 } else { 3 };
            let dim = rng.range(1, top) as usize;
            let cells = (base as u32).pow(dim as u32);
            let code = rng.below(1usize << cells) as u128;
            let bang = Bang::new(code, dim, base);
            let text = bang.to_json();
            assert_eq!(Bang::from_json(&text).unwrap(), bang);
            assert_eq!(Bang::from_json(&text).unwrap().to_json(), text);
            assert_eq!(Bang::from_url(&bang.to_url().unwrap()).unwrap(), bang);
            assert_eq!(Bang::from_file(&bang.to_file().unwrap()).unwrap(), bang);
        }
    }
    #[test]
    fn every_named_design_grows_back_from_its_listed_code_in_both_bases() {
        for dim in [2, 3] {
            for base in [2, 3] {
                let list = words(dim, base);
                assert_eq!(list.len(), if dim == 2 { 10 } else { 12 });
                for ((code, word), design) in
                    list.iter().zip(classics(dim).into_iter().chain(antis(dim)))
                {
                    let code: Code = code.parse().unwrap();
                    let (seed, grown) = match dim {
                        2 => (
                            two::named(design, base, 1, 0).unwrap().types().clone(),
                            two::create(code, base, 1, 0, base).unwrap().types().clone(),
                        ),
                        _ => (
                            three::named(design, base, 1).unwrap().types().clone(),
                            three::create(code, base, 1, base).unwrap().types().clone(),
                        ),
                    };
                    assert_eq!(*word, design.name().to_lowercase());
                    assert_eq!(grown, seed, "{dim} {base} {word}");
                }
            }
        }
        assert!(words(4, 2).is_empty());
    }
    #[test]
    fn a_code_names_the_design_it_draws() {
        let word = |code: u64, dim, base| of(Code::from(code), dim, base);
        assert_eq!(word(7, 2, 2).as_deref(), Some("carpet"));
        assert_eq!(word(23, 3, 2).as_deref(), Some("carpet"));
        assert_eq!(word(11, 2, 2), None);
    }
}
