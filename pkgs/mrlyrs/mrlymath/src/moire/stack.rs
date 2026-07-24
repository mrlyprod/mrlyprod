use super::field::Field;
use super::layer::{layer, Layer};
use super::{Combine, Lattice, Spec};
use mrlycore::errors::Result;

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
        let m = layer(&params)?;
        match combine {
            Combine::Sum => {
                for (a, &b) in acc.iter_mut().zip(m.iter()) {
                    *a += b as u8 as f32;
                }
            }
            Combine::And => {
                if first {
                    for (a, &b) in acc.iter_mut().zip(m.iter()) {
                        *a = b as u8 as f32;
                    }
                } else {
                    for (a, &b) in acc.iter_mut().zip(m.iter()) {
                        *a *= b as u8 as f32;
                    }
                }
            }
            Combine::Xor => {
                for (a, &b) in acc.iter_mut().zip(m.iter()) {
                    let cur = *a != 0.0;
                    *a = (cur ^ b) as u8 as f32;
                }
            }
        }
        first = false;
    }
    Ok(Field::from_data(acc, size))
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
    Ok(Field::from_data(acc, size))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn spec(code: u128) -> Spec {
        Spec::new(code, 2, 2)
    }
    #[test]
    fn sum_stack_counts_layers() {
        let numbers = [1, 3, 5];
        let f = stack(spec(7), &numbers, Combine::Sum, 1, Lattice::Square, 64, &[]).unwrap();
        assert!(f.max() <= numbers.len() as f32);
        assert!(f.min() >= 0.0);
    }
    #[test]
    fn complement_identity() {
        let numbers: Vec<usize> = (1..52).step_by(2).collect();
        let size = 48;
        for code in [1u128, 2, 3, 6, 7] {
            let h = stack(
                spec(code),
                &numbers,
                Combine::Sum,
                1,
                Lattice::Square,
                size,
                &[],
            )
            .unwrap();
            let hc = stack(
                spec(15 - code),
                &numbers,
                Combine::Sum,
                1,
                Lattice::Square,
                size,
                &[],
            )
            .unwrap();
            let n = numbers.len() as f32;
            let max_err = h
                .data
                .iter()
                .zip(hc.data.iter())
                .map(|(&a, &b)| (a + b - n).abs())
                .fold(0.0f32, f32::max);
            assert!(max_err < 1e-3, "code {code} max_err {max_err}");
        }
    }
    #[test]
    fn sponge_slice0_equals_carpet_stack() {
        use crate::bang::corners_to_code;
        let carpet_corners: Vec<Vec<u8>> = (0..3)
            .flat_map(|a| (0..3).map(move |b| vec![a as u8, b as u8]))
            .filter(|c| !(c[0] == 1 && c[1] == 1))
            .collect();
        let carpet_code = corners_to_code(&carpet_corners, 2, 3);
        let sponge_corners: Vec<Vec<u8>> = (0..3)
            .flat_map(|a| {
                (0..3).flat_map(move |b| (0..3).map(move |c| vec![a as u8, b as u8, c as u8]))
            })
            .filter(|c| {
                let centers = (c[0] == 1) as u8 + (c[1] == 1) as u8 + (c[2] == 1) as u8;
                centers <= 1
            })
            .collect();
        let sponge_code = corners_to_code(&sponge_corners, 3, 3);
        let numbers: Vec<usize> = (1..20).step_by(2).collect();
        let size = 48;
        let carpet = stack(
            Spec::new(carpet_code, 3, 2),
            &numbers,
            Combine::Sum,
            1,
            Lattice::Square,
            size,
            &[],
        )
        .unwrap();
        let sponge = stack(
            Spec::new(sponge_code, 3, 3),
            &numbers,
            Combine::Sum,
            1,
            Lattice::Square,
            size,
            &[0.0],
        )
        .unwrap();
        assert_eq!(carpet.data, sponge.data);
    }
}
