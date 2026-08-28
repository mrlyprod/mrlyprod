use super::field::Field;
use super::stack::stack;
use super::{Combine, Lattice, Spec};
use mrlycore::errors::{value_error, Result};
use mrlymath::bang::corners_to_code;

/// One named moire recipe: the design, the scales it stacks and the lattice it samples.
#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    /// The name the recipe answers to.
    pub name: &'static str,
    /// The design sampled at every scale.
    pub spec: Spec,
    /// The side numbers stacked.
    pub numbers: Vec<usize>,
    /// The way the layers merge.
    pub combine: Combine,
    /// The fractal depth of each layer.
    pub level: usize,
    /// The lattice the layers are sampled on.
    pub lattice: Lattice,
}

fn parity() -> Spec {
    Spec::new(corners_to_code(&[vec![0, 0]], 2, 2), 2, 2)
}

fn pierced() -> Spec {
    let corners: Vec<Vec<u8>> = (0..3u8)
        .flat_map(|a| (0..3u8).map(move |b| vec![a, b]))
        .filter(|corner| corner != &vec![1, 1])
        .collect();
    Spec::new(corners_to_code(&corners, 2, 3), 3, 2)
}

fn odds(limit: usize) -> Vec<usize> {
    (1..=limit.max(1)).step_by(2).collect()
}

impl Preset {
    /// The parity heatmap: odd scales of the low corner summed on the square lattice.
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
    /// The parity weave: the same odd scales folded to their parity instead of summed.
    pub fn weave(limit: usize) -> Preset {
        Preset {
            name: "weave",
            combine: Combine::Xor,
            ..Preset::heatmap(limit)
        }
    }
    /// The hive: the parity heatmap sampled on the hexagonal lattice.
    pub fn hive(limit: usize) -> Preset {
        Preset {
            name: "hive",
            lattice: Lattice::Hex,
            ..Preset::heatmap(limit)
        }
    }
    /// The carpet stack: every base-three corner but the centre, summed over odd scales.
    pub fn carpet(limit: usize) -> Preset {
        Preset {
            name: "carpet",
            spec: pierced(),
            ..Preset::heatmap(limit)
        }
    }
    /// Samples the preset into a square field of the given side.
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

/// Returns every preset stacked up to the given scale.
pub fn all(limit: usize) -> Vec<Preset> {
    vec![
        Preset::heatmap(limit),
        Preset::weave(limit),
        Preset::hive(limit),
        Preset::carpet(limit),
    ]
}

/// Returns the preset the name picks, or an error naming the ones there are.
///
/// ```
/// let preset = mrlylab::moire::presets::named("heatmap", 9).unwrap();
/// assert_eq!(preset.numbers, vec![1, 3, 5, 7, 9]);
/// assert!(mrlylab::moire::presets::named("soup", 9).is_err());
/// ```
pub fn named(name: &str, limit: usize) -> Result<Preset> {
    match all(limit).into_iter().find(|p| p.name == name) {
        Some(preset) => Ok(preset),
        None => value_error(format!(
            "no preset {name:?}, only heatmap, weave, hive and carpet."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_heatmap_sums_the_parity_of_every_odd_scale() {
        let (size, limit) = (24usize, 9usize);
        let field = Preset::heatmap(limit).field(size).unwrap();
        let mut want = vec![0f32; size * size];
        for n in (1..=limit).step_by(2) {
            for row in 0..size {
                let v = (row as f64 + 0.5) / size as f64;
                for col in 0..size {
                    let u = (col as f64 + 0.5) / size as f64;
                    let even = |value: f64| (value.floor() as i64).rem_euclid(2) == 0;
                    let lit = even(n as f64 * u) && even(n as f64 * v);
                    want[row * size + col] += u8::from(lit) as f32;
                }
            }
        }
        assert_eq!(field.data, want);
        assert_eq!(field.size, size);
    }

    #[test]
    fn the_weave_folds_the_same_layers_to_parity() {
        let field = Preset::weave(9).field(32).unwrap();
        assert!(field.data.iter().all(|&v| v == 0.0 || v == 1.0));
        let summed = Preset::heatmap(9).field(32).unwrap();
        for (woven, count) in field.data.iter().zip(summed.data.iter()) {
            assert_eq!(*woven, (*count as u32 % 2) as f32);
        }
    }

    #[test]
    fn the_carpet_stack_keeps_eight_corners_of_nine() {
        let preset = Preset::carpet(7);
        assert_eq!(preset.spec.base, 3);
        let table = super::super::sample::membership(preset.spec.code, 3, 2).unwrap();
        assert_eq!(table.iter().filter(|&&lit| lit).count(), 8);
        assert!(!table[super::super::sample::pack(&[1, 1], 3)]);
        let field = preset.field(32).unwrap();
        assert!(field.max() <= preset.numbers.len() as f32);
        assert!(field.mean() > 0.0);
    }

    #[test]
    fn the_hive_leans_the_lattice() {
        let hive = Preset::hive(9).field(48).unwrap();
        let flat = Preset::heatmap(9).field(48).unwrap();
        assert_eq!(hive.data.len(), flat.data.len());
        assert_ne!(hive.data, flat.data);
    }

    #[test]
    fn every_preset_names_itself_and_renders() {
        let presets = all(5);
        let mut names: Vec<&str> = presets.iter().map(|p| p.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), presets.len());
        for preset in &presets {
            assert_eq!(named(preset.name, 5).unwrap(), *preset);
            let field = preset.field(16).unwrap();
            assert_eq!(field.data.len(), 256);
        }
        assert!(named("heatmap", 0).unwrap().numbers == vec![1]);
    }
}
