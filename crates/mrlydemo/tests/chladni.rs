use mrlycore::json::parse;
use mrlycore::Tensor;
use mrlydemo::chladni::*;
use mrlydemo::life::life_noise;
use mrlymath::life::{design_mask, next_grid, Boundary};
use mrlymath::two::Cell2d;

fn agree(
    code: u128,
    side: usize,
    level: usize,
    birth: (f64, f64),
    survive: (f64, f64),
    born: &[usize],
    kept: &[usize],
) -> usize {
    let size = 32;
    let mask = design_mask(2, code, side, level).unwrap();
    let mut fast = life_noise(size, size, 0.5, 11);
    let mut slow = Cell2d::new(Tensor::of(fast.clone(), vec![size, size]));
    let name = code.to_string();
    for step in 0..8 {
        fast = chladni_next(
            &fast, size, &name, side, level, birth.0, birth.1, survive.0, survive.1,
        )
        .unwrap();
        slow = next_grid(&slow, born, kept, &mask, Boundary::Wrap).unwrap();
        assert_eq!(fast.as_slice(), slow.types().bytes(), "step {step}");
    }
    fast.iter().filter(|&&t| t != 0).count()
}

#[test]
fn the_fft_step_is_the_crate_step_on_the_moore_mask() {
    let live = agree(7, 3, 1, (0.375, 0.375), (0.25, 0.375), &[3], &[2, 3]);
    assert!(live > 0);
}

#[test]
fn the_fft_step_is_the_crate_step_on_the_level_two_carpet() {
    let born: Vec<usize> = (18..=24).collect();
    let kept: Vec<usize> = (18..=30).collect();
    let live = agree(7, 3, 2, (0.28, 0.38), (0.28, 0.48), &born, &kept);
    assert!(live > 0);
}

#[test]
fn the_kernel_picture_centres_the_mask() {
    let picture = chladni_kernel("7", 3, 2, 32).unwrap();
    assert_eq!((picture.width, picture.height), (32, 32));
    assert_eq!(
        picture.types.iter().map(|&t| usize::from(t)).sum::<usize>(),
        64
    );
    assert_eq!(picture.types[16 * 32 + 16], 0);
    assert_eq!(picture.types[12 * 32 + 12], 1);
    assert_eq!(picture.types[11 * 32 + 12], 0);
    assert!(chladni_kernel("7", 3, 4, 64).is_err());
    assert!(chladni_kernel("7", 3, 1, 12).is_err());
}

#[test]
fn the_spectrum_and_profile_read_a_plain_stripe() {
    let size = 32;
    let types: Vec<u8> = (0..size * size)
        .map(|i| u8::from((i % size) % 4 < 2))
        .collect();
    let spectrum = chladni_spectrum(&types, size).unwrap();
    assert_eq!(spectrum.len(), size * size);
    assert_eq!(spectrum[16 * 32 + 16], 1.0);
    let read = parse(&chladni_profile(&types, size).unwrap()).unwrap();
    assert_eq!(read["peak_ring"], 8);
    assert_eq!(read["wavelength"], 4.0);
    assert_eq!(read["profile"].as_array().unwrap().len(), 17);
    assert!(chladni_spectrum(&types, 31).is_err());
    assert!(chladni_profile(&types[1..], size).is_err());
}

fn pinned(density: f64) -> (usize, u64) {
    let grid = chladni_run("7", 3, 3, 0.28, 0.375, 0.28, 0.48, 128, 32, density, 7).unwrap();
    let live = grid.types.iter().filter(|&&t| t != 0).count();
    let read = parse(&chladni_profile(&grid.types, 128).unwrap()).unwrap();
    println!(
        "chladni_run 7 3 3 0.28 0.375 0.28 0.48 128 32 {density} 7 live {live} peak_ring {} wavelength {}",
        read["peak_ring"], read["wavelength"]
    );
    (live, read["peak_ring"].as_u64().unwrap())
}

#[test]
fn the_pinned_soups_and_their_profiles() {
    assert_eq!(pinned(0.5), (0, 1));
    assert_eq!(pinned(0.45), (5259, 3));
}
