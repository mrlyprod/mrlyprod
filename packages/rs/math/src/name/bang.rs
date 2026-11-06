use crate::atoms;
use crate::catalog::Design;
use crate::catalog::{antis, classics};
use crate::code::Code;
use crate::factory::{corners_to_code, residue_corners, total_codes};
use mrly_error::{Result, value_error};
use mrly_name::Named;
use mrly_name::kind;
use mrly_tensor::Tensor;
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

fn flat(design: Design, number: usize) -> Option<Tensor> {
    Some(match design {
        Design::Carpet => atoms::carpet_2d(number),
        Design::Net => atoms::net_2d(number),
        Design::Htree => atoms::htree_2d(number),
        Design::Vtree => atoms::vtree_2d(number),
        Design::Void => atoms::void_2d(number),
        Design::Point => atoms::point_2d(number),
        Design::Dust => atoms::dust_2d(number),
        Design::Hline => atoms::hline_2d(number),
        Design::Vline => atoms::vline_2d(number),
        Design::Star => atoms::star_2d(number),
        _ => return None,
    })
}

fn solid(design: Design, number: usize) -> Option<Tensor> {
    Some(match design {
        Design::Carpet => atoms::carpet_3d(number),
        Design::Net => atoms::net_3d(number),
        Design::Xtree => atoms::xtree_3d(number),
        Design::Ytree => atoms::ytree_3d(number),
        Design::Ztree => atoms::ztree_3d(number),
        Design::Void => atoms::void_3d(number),
        Design::Point => atoms::point_3d(number),
        Design::Dust => atoms::dust_3d(number),
        Design::Xline => atoms::xline_3d(number),
        Design::Yline => atoms::yline_3d(number),
        Design::Zline => atoms::zline_3d(number),
        Design::Star => atoms::star_3d(number),
        _ => return None,
    })
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
                2 => drawn(&flat(design, base)?),
                3 => drawn(&solid(design, base)?),
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
