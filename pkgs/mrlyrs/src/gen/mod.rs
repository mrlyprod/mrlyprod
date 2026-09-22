/// The recipe to cell builders, one per dimension, and the random draws that feed them.
pub mod build;
/// The random tile drawing the dimensions share, on the seeded stream.
pub mod draw;
/// The tile name: a full tile recipe folded to its one canonical object.
pub mod name;
/// The tile recipe: its families, groups, parities, catalog and size lists.
pub mod recipe;
/// The seeded artwork run from tile recipe to rendered files.
pub mod variation;

pub use recipe::{Group, Parity, Tile};

use crate::core::error::{value_error, Result};
use crate::core::rng::Rng;

/// Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe
/// constraints and paint, repeated `width` across and `height` down at one pixel per cell.
///
/// The seed opens one stream, `variation::create` draws its own seed from it, and the paint and
/// render follow on the same stream, so one seed always paints the same background.
pub fn background(seed: u64, width: usize, height: usize) -> Result<Vec<u8>> {
    if width == 0 || height == 0 {
        return value_error("a background wants a width and a height above zero.");
    }
    let mut rng = Rng::new(seed);
    let config = variation::Config {
        files: vec![(width, height)],
        ..variation::Config::default()
    };
    let drawn = variation::create(&config, &mut rng)?;
    let built = variation::generate(drawn, &config, &mut rng)?;
    let rendered = variation::render(built, 1, &mut rng)?;
    match rendered.files.into_iter().next() {
        Some(file) => Ok(file.png),
        None => value_error("the background rendered no file."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn background_is_a_png_of_the_seed() {
        let a = background(7, 2, 3).unwrap();
        let b = background(7, 2, 3).unwrap();
        assert_eq!(&a[1..4], b"PNG");
        assert_eq!(a, b);
        assert_ne!(a, background(8, 2, 3).unwrap());
    }
    #[test]
    fn refuses_a_background_with_no_pixels() {
        assert!(background(7, 0, 3).is_err());
        assert!(background(7, 2, 0).is_err());
    }
}
