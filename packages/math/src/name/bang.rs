use crate::bang::Code;
use crate::bang::catalog::{antis, classics};
use crate::bang::factory::{corners_to_code, residue_corners, total_codes};
use crate::{three, two};
use mrly_core::Tensor;
use mrly_core::error::{Result, value_error};
use mrly_core::kind;
use mrly_core::name::Named;
use serde::{Deserialize, Serialize};

kind!("bang");

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lattice {
    #[default]
    Square,
    Hex,
}

impl Lattice {
    pub fn is_square(&self) -> bool {
        matches!(self, Lattice::Square)
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bang {
    pub kind: Kind,
    pub dim: usize,
    #[serde(default, skip_serializing_if = "Lattice::is_square")]
    pub lattice: Lattice,
    #[serde(default = "two", skip_serializing_if = "is_two")]
    pub base: usize,
    pub code: u128,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twist: Option<Vec<usize>>,
}

impl Bang {
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

pub fn of(code: Code, dim: usize, base: usize) -> Option<String> {
    let code = code.to_string();
    words(dim, base)
        .into_iter()
        .find(|(listed, _)| *listed == code)
        .map(|(_, word)| word)
}
