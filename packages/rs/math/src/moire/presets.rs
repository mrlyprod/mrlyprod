use super::field::Field;
use super::stack::stack;
use super::{Combine, Lattice, Spec};
use crate::bang::corners_to_code;
use mrly_core::error::{Result, value_error};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Preset {
    pub name: &'static str,
    pub spec: Spec,
    pub numbers: Vec<usize>,
    pub combine: Combine,
    pub level: usize,
    pub lattice: Lattice,
}

fn parity() -> Spec {
    Spec::new(corners_to_code(&[vec![0, 0]], 2, 2).get(), 2, 2)
}

fn pierced() -> Spec {
    let corners: Vec<Vec<u8>> = (0..3u8)
        .flat_map(|a| (0..3u8).map(move |b| vec![a, b]))
        .filter(|corner| corner != &vec![1, 1])
        .collect();
    Spec::new(corners_to_code(&corners, 2, 3).get(), 3, 2)
}

fn odds(limit: usize) -> Vec<usize> {
    (1..=limit.max(1)).step_by(2).collect()
}

impl Preset {
    pub fn heatmap(limit: usize) -> Preset {
        Preset {
            name: "heatmap",
            spec: parity(),
            numbers: odds(limit),
            combine: Combine::Sum,
            level: 1,
            lattice: Lattice::Square,
        }
    }
    pub fn weave(limit: usize) -> Preset {
        Preset {
            name: "weave",
            combine: Combine::Xor,
            ..Preset::heatmap(limit)
        }
    }
    pub fn hive(limit: usize) -> Preset {
        Preset {
            name: "hive",
            lattice: Lattice::Hex,
            ..Preset::heatmap(limit)
        }
    }
    pub fn carpet(limit: usize) -> Preset {
        Preset {
            name: "carpet",
            spec: pierced(),
            ..Preset::heatmap(limit)
        }
    }
    pub fn field(&self, size: usize) -> Result<Field> {
        stack(
            self.spec,
            &self.numbers,
            self.combine,
            self.level,
            self.lattice,
            size,
            &[],
        )
    }
}

pub fn all(limit: usize) -> Vec<Preset> {
    vec![
        Preset::heatmap(limit),
        Preset::weave(limit),
        Preset::hive(limit),
        Preset::carpet(limit),
    ]
}

pub fn named(name: &str, limit: usize) -> Result<Preset> {
    match all(limit).into_iter().find(|p| p.name == name) {
        Some(preset) => Ok(preset),
        None => value_error(format!(
            "no preset {name:?}, only heatmap, weave, hive and carpet."
        )),
    }
}
