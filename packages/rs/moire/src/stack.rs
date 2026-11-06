use super::field::Field;
use super::layer::{Layer, layer};
use super::{Combine, Lattice, Spec};
use mrly_error::Result;

pub(crate) fn merge(acc: &mut [f32], mask: &[bool], combine: Combine, first: bool) {
    match combine {
        Combine::Sum => {
            for (a, &b) in acc.iter_mut().zip(mask.iter()) {
                *a += b as u8 as f32;
            }
        }
        Combine::And => {
            for (a, &b) in acc.iter_mut().zip(mask.iter()) {
                *a = if first {
                    b as u8 as f32
                } else {
                    *a * b as u8 as f32
                };
            }
        }
        Combine::Xor => {
            for (a, &b) in acc.iter_mut().zip(mask.iter()) {
                *a = ((*a != 0.0) ^ b) as u8 as f32;
            }
        }
    }
}

pub fn stack(
    spec: Spec,
    numbers: &[usize],
    combine: Combine,
    level: usize,
    lattice: Lattice,
    size: usize,
    slices: &[f64],
) -> Result<Field> {
    let mut acc = vec![0.0f32; size * size];
    let mut first = true;
    for &n in numbers {
        let params = Layer {
            spec,
            number: n,
            level,
            lattice,
            size,
            slices: slices.to_vec(),
        };
        merge(&mut acc, &layer(&params)?, combine, first);
        first = false;
    }
    Field::from_data(acc, size)
}

pub fn stack_codes(
    specs: &[Spec],
    number: usize,
    level: usize,
    lattice: Lattice,
    size: usize,
    slices: &[f64],
) -> Result<Field> {
    let mut acc = vec![0.0f32; size * size];
    for &spec in specs {
        let params = Layer {
            spec,
            number,
            level,
            lattice,
            size,
            slices: slices.to_vec(),
        };
        let m = layer(&params)?;
        for (a, &b) in acc.iter_mut().zip(m.iter()) {
            *a += b as u8 as f32;
        }
    }
    Field::from_data(acc, size)
}
