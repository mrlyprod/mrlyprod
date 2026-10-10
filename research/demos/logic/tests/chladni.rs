use demos::chladni::*;
use mrlyrs::core::error::parse;

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
    assert_eq!(pinned(0.45), (3274, 1));
}
