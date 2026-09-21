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

/// Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe
/// constraints and paint, repeated `width` across and `height` down at one pixel per cell.
///
/// The seed primes the global stream and `variation::create` draws its own seed from it, so one
/// seed always paints the same background.
pub fn background(seed: u64, width: usize, height: usize) -> Result<Vec<u8>> {
    crate::core::state::seed(seed);
    let config = variation::Config {
        files: vec![(width, height)],
        ..variation::Config::default()
    };
    let drawn = variation::create(&config)?;
    let built = variation::generate(drawn, &config)?;
    let rendered = variation::render(built, 1)?;
    match rendered.files.into_iter().next() {
        Some(file) => Ok(file.png),
        None => value_error("the background rendered no file."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::guard;
    #[test]
    fn background_is_a_png_of_the_seed() {
        let _guard = guard();
        let a = background(7, 2, 3).unwrap();
        let b = background(7, 2, 3).unwrap();
        assert_eq!(&a[1..4], b"PNG");
        assert_eq!(a, b);
        assert_ne!(a, background(8, 2, 3).unwrap());
    }
}
