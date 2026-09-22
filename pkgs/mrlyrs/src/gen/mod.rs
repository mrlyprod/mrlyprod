//! The generator: the pipeline from a recipe to a file, for datasets, the automator, backgrounds.
//!
//! - `recipe`: the tile recipe, its families, groups, parities, catalog and size lists.
//! - `draw`: one recipe drawn at random from a seeded stream under size constraints.
//! - `build`: the recipe to cell builders, one per dimension.
//! - `variation`: the seeded artwork, from recipe through paint to rendered files.
//! - `name`: the canonical name of a recipe, and the recipe back out of it.
//!
//! The doors: [`Tile::new`](crate::gen::recipe::Tile::new),
//! [`Tile::size`](crate::gen::recipe::Tile::size),
//! [`Tile::check`](crate::gen::recipe::Tile::check),
//! [`draw::create`](crate::gen::draw::create), [`random_design`](crate::gen::random_design),
//! [`random_rotation`](crate::gen::random_rotation), [`build_2d`](crate::gen::build::build_2d),
//! [`tree_mask`](crate::gen::tree_mask), [`variation::create`](crate::gen::variation::create),
//! [`hex_key`](crate::gen::hex_key), [`classic_code_nd`](crate::gen::classic_code_nd),
//! [`Tile::of`](crate::gen::name::Tile::of) and [`background`](crate::gen::background).

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

pub use build::tree_mask;
pub use draw::{random_design, random_rotation};
pub use name::{classic_code, classic_code_nd};
pub use recipe::{Group, Parity, Tile};
pub use variation::hex_key;

use crate::core::error::{value_error, Result};
use crate::core::rng::Rng;

/// Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe
/// constraints and paint, repeated `width` across and `height` down at one pixel per cell.
///
/// The seed opens one stream, `variation::create` draws its own seed from it, and the paint and
/// render follow on the same stream, so one seed always paints the same background.
///
/// ```
/// let png = mrlyrs::gen::background(1, 2, 2)?;
/// assert_eq!(png[..8], [137, 80, 78, 71, 13, 10, 26, 10]);
/// # Ok::<(), mrlyrs::Error>(())
/// ```
///
/// # Errors
///
/// Errs when the width or the height is zero, or when the artwork will not draw or render.
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
    #[test]
    fn serde_round_trips() {
        for parity in Parity::all() {
            let text = serde_json::to_string(&parity).unwrap();
            assert_eq!(parity, serde_json::from_str(&text).unwrap());
        }
    }
}
