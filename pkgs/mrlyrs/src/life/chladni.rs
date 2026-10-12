use crate::core::error::{shape_error, value_error, Result};
use crate::core::rng::Rng;
use crate::core::tensor::Tensor;
use crate::math::two::Cell2d;
use crate::num::fft::{
    convolve_with, embed_kernel, log_spectrum, peak_ring, peak_wavelength, radial_profile,
    transform,
};
use serde::{Deserialize, Serialize};

/// The radial reading of a field's log spectrum: the ring means, the peak ring past the centre and its wavelength in cells.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    /// The mean of every ring of integer radius, ring 0 through size over two.
    pub profile: Vec<f64>,
    /// The ring past the centre where the profile peaks, a tie broken at the smaller ring.
    pub peak_ring: usize,
    /// The size over the peak ring, or zero when there is no ring past the centre.
    pub wavelength: f64,
}

fn checked_size(size: usize) -> Result<()> {
    if size == 0 || !size.is_power_of_two() {
        return value_error(format!("the field size {size} is not a power of two."));
    }
    Ok(())
}

fn checked_mask(mask: &Tensor, size: usize) -> Result<(usize, usize)> {
    checked_size(size)?;
    if mask.shape.len() != 2 || mask.shape[0] != mask.shape[1] {
        return shape_error(format!("the mask {:?} is not square.", mask.shape));
    }
    let span = mask.shape[0];
    if span > size {
        return value_error(format!(
            "the mask side {span} does not fit the field {size}."
        ));
    }
    let budget = mask.sum() as usize;
    if budget == 0 {
        return value_error("the mask is empty.");
    }
    Ok((span, budget))
}

fn side_of(cell: &Cell2d) -> Result<usize> {
    let shape = &cell.types().shape;
    if shape.len() != 2 || shape[0] != shape[1] {
        return shape_error(format!("the grid {shape:?} is not square."));
    }
    checked_size(shape[0])?;
    Ok(shape[0])
}

fn field_of(types: &[u8]) -> Vec<f64> {
    types
        .iter()
        .map(|&t| if t != 0 { 1.0 } else { 0.0 })
        .collect()
}

fn window(lo: f64, hi: f64, budget: usize) -> Vec<bool> {
    (0..=budget)
        .map(|k| {
            let share = k as f64 / budget as f64;
            lo <= share && share <= hi
        })
        .collect()
}

struct Engine {
    size: usize,
    budget: usize,
    kernel_re: Vec<f64>,
    kernel_im: Vec<f64>,
    born: Vec<bool>,
    kept: Vec<bool>,
}

fn engine(mask: &Tensor, birth: (f64, f64), survive: (f64, f64), size: usize) -> Result<Engine> {
    let (span, budget) = checked_mask(mask, size)?;
    let (kernel_re, kernel_im) = transform(&embed_kernel(mask.bytes()?, span, size)?, size)?;
    Ok(Engine {
        size,
        budget,
        kernel_re,
        kernel_im,
        born: window(birth.0, birth.1, budget),
        kept: window(survive.0, survive.1, budget),
    })
}

fn advance(engine: &Engine, types: &mut [u8]) -> Result<()> {
    let counts = convolve_with(
        &field_of(types),
        &engine.kernel_re,
        &engine.kernel_im,
        engine.size,
    )?;
    for (slot, &count) in types.iter_mut().zip(&counts) {
        let n = (count.round().max(0.0) as usize).min(engine.budget);
        let lives = if *slot != 0 {
            engine.kept[n]
        } else {
            engine.born[n]
        };
        *slot = u8::from(lives);
    }
    Ok(())
}

/// Draws a mask centred in a size-square picture, the mask centre at size over two.
///
/// # Errors
///
/// Errs when the size is no power of two, the mask is not square, past the size or empty.
pub fn kernel(mask: &Tensor, size: usize) -> Result<Cell2d> {
    let (span, _) = checked_mask(mask, size)?;
    let cells = mask.bytes()?;
    let start = size / 2 - span / 2;
    let mut types = vec![0u8; size * size];
    for r in 0..span {
        for c in 0..span {
            types[(start + r) * size + start + c] = cells[r * span + c];
        }
    }
    Cell2d::new(Tensor::of(types, vec![size, size])?)
}

/// Advances a square grid one generation on a wrapped torus: a dead cell is born and a live one kept when its mask count over the mask budget lies in the closed birth or survive window, the count read by FFT convolution.
///
/// # Errors
///
/// Errs when the grid is not square with a power-of-two side, or the mask is not square, past the side or empty.
pub fn next(
    cell: &Cell2d,
    mask: &Tensor,
    birth: (f64, f64),
    survive: (f64, f64),
) -> Result<Cell2d> {
    let size = side_of(cell)?;
    let engine = engine(mask, birth, survive, size)?;
    let mut types = cell.types().bytes()?.to_vec();
    advance(&engine, &mut types)?;
    Cell2d::new(Tensor::of(types, vec![size, size])?)
}

/// Runs a seeded soup of the given density for the given steps on a size-square torus under the closed birth and survive windows.
///
/// # Errors
///
/// Errs when the size is no power of two, or the mask is not square, past the size or empty.
pub fn run(
    mask: &Tensor,
    birth: (f64, f64),
    survive: (f64, f64),
    size: usize,
    steps: usize,
    density: f64,
    seed: u64,
) -> Result<Cell2d> {
    let engine = engine(mask, birth, survive, size)?;
    let mut rng = Rng::new(seed);
    let mut types: Vec<u8> = (0..size * size)
        .map(|_| u8::from(rng.chance(density)))
        .collect();
    for _ in 0..steps {
        advance(&engine, &mut types)?;
    }
    Cell2d::new(Tensor::of(types, vec![size, size])?)
}

/// Returns the centred log magnitude spectrum of a square 0/1 grid, scaled to at most one by its peak.
///
/// # Errors
///
/// Errs when the grid is not square with a power-of-two side.
pub fn spectrum(cell: &Cell2d) -> Result<Vec<f64>> {
    let size = side_of(cell)?;
    let spectrum = log_spectrum(&field_of(cell.types().bytes()?), size)?;
    let top = spectrum.iter().cloned().fold(0.0f64, f64::max);
    let scale = if top > 0.0 { 1.0 / top } else { 0.0 };
    Ok(spectrum.iter().map(|&v| v * scale).collect())
}

/// Reads the ring means of a square 0/1 grid's log spectrum, its peak ring past the centre and that ring's wavelength in cells.
///
/// # Errors
///
/// Errs when the grid is not square with a power-of-two side.
pub fn profile(cell: &Cell2d) -> Result<Profile> {
    let size = side_of(cell)?;
    let profile = radial_profile(&log_spectrum(&field_of(cell.types().bytes()?), size)?, size)?;
    Ok(Profile {
        peak_ring: peak_ring(&profile),
        wavelength: peak_wavelength(&profile, size),
        profile,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{design_mask, next_grid, Boundary};
    use crate::math::bang::Code;
    #[test]
    fn the_kernel_picture_centres_the_mask() {
        let picture = kernel(&design_mask(2, Code::from(7u128), 3, 2).unwrap(), 32).unwrap();
        let types = picture.types();
        assert_eq!(types.shape, vec![32, 32]);
        assert_eq!(types.sum(), 64);
        assert_eq!(types.get(&[16, 16]).unwrap(), 0);
        assert_eq!(types.get(&[12, 12]).unwrap(), 1);
        assert_eq!(types.get(&[11, 12]).unwrap(), 0);
        assert!(kernel(&design_mask(2, Code::from(7u128), 3, 4).unwrap(), 64).is_err());
        assert!(kernel(&design_mask(2, Code::from(7u128), 3, 1).unwrap(), 12).is_err());
    }
    #[test]
    fn the_pinned_soups_and_their_profiles() {
        let mask = design_mask(2, Code::from(7u128), 3, 3).unwrap();
        let pinned = |density: f64| {
            let grid = run(&mask, (0.28, 0.375), (0.28, 0.48), 128, 32, density, 7).unwrap();
            (grid.types().sum(), profile(&grid).unwrap().peak_ring)
        };
        assert_eq!(pinned(0.5), (0, 1));
        assert_eq!(pinned(0.45), (3274, 1));
    }
    #[test]
    fn the_fft_step_is_the_crate_step_on_a_design_mask() {
        let size = 32;
        for (code, side, level, birth, survive) in [
            (7u128, 3usize, 1usize, (0.375, 0.375), (0.25, 0.375)),
            (7, 3, 2, (0.28, 0.38), (0.28, 0.48)),
        ] {
            let mask = design_mask(2, Code::from(code), side, level).unwrap();
            let budget = mask.sum() as usize;
            let counts = |(lo, hi): (f64, f64)| -> Vec<usize> {
                (0..=budget)
                    .filter(|&k| {
                        let share = k as f64 / budget as f64;
                        lo <= share && share <= hi
                    })
                    .collect()
            };
            let (born, kept) = (counts(birth), counts(survive));
            let mut fast = run(&mask, birth, survive, size, 0, 0.5, 11).unwrap();
            let mut slow = fast.clone();
            for step in 0..8 {
                fast = next(&fast, &mask, birth, survive).unwrap();
                slow = next_grid(&slow, &born, &kept, &mask, Boundary::Wrap).unwrap();
                assert_eq!(fast, slow, "code {code} level {level} step {step}");
            }
            assert!(fast.types().sum() > 0);
        }
    }
}
