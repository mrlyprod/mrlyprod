//! The substrate: the road from a grid of bytes to pixels, with nothing mrly on it.
//!
//! - `tensor` makes and tallies the byte grids; `cell` dresses one in colors and tags.
//! - `colors` holds the rgba color, the two themes and the house palette; `paint` spreads a palette over a cell.
//! - `ramp` turns counter values into colors; `resample` rescales pixels and squashes them for hex.
//! - `image` holds the paletted pixels; `codec` writes them as a png or a gif and reads a png back.
//! - `rng` deals seeded chance: one xoshiro256++ stream, passed by hand, never global.
//! - `error` holds the one error, its Result and the json parser; `named` names an enum.
//!
//! The json value and map are serde_json's, kept under `preserve_order` so an object comes back in the order it was written and both bridges print the same text.
//!
//! The doors: [`Tensor::of`](crate::core::tensor::Tensor::of), [`Tensor::rot90`](crate::core::tensor::Tensor::rot90), [`Color::from_hex`](crate::core::colors::Color::from_hex), [`Color::to_hex`](crate::core::colors::Color::to_hex), [`png`](crate::core::png()), [`unpng`](crate::core::unpng()), [`gif`](crate::core::gif()) and [`Colorizer::color`](crate::core::ramp::Colorizer::color).

/// The cell grid: type bytes with optional per-cell colors and tags.
pub mod cell;
/// The png and gif codecs, rented from the png and gif crates.
pub mod codec;
/// The rgba color, its themes and the fifteen-color palette utils/colors.py stamps.
pub mod colors;
/// The one error type, its Result and the json parser over the rented value.
pub mod error;
/// The paletted image and its rows.
pub mod image;
mod named;
/// The editions that distribute a palette over a cell.
pub mod paint;
/// The colorizers that turn counter values into colors.
pub mod ramp;
/// The pixel resamplers and the hex squash.
pub mod resample;
/// The seeded random stream.
pub mod rng;
/// The tensor and its dtypes.
pub mod tensor;

pub use cell::{Cell, Mode};
pub use codec::{gif, png, unpng, PNG_MAGIC};
pub use colors::Color;
pub use error::{Error, Result};
pub use image::Image;
pub(crate) use named::named_enum;
pub use ramp::Colorizer;
pub use resample::{hex_fit, hex_size, resample, Filter};
pub use rng::Rng;
pub use serde_json::{json, Value as Json};
pub use tensor::{Dtype, Tensor};

/// An object's entries, kept in insertion order.
pub type Map = serde_json::Map<String, Json>;

#[cfg(test)]
mod tests {
    use super::cell::Mode;
    use super::colors::{Color, DARK, RED, WHITE};
    use super::paint::Config;
    use super::ramp::Colorizer;
    use super::resample::Filter;
    use super::rng::Rng;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    fn same<T: Serialize + DeserializeOwned>(value: &T) -> bool {
        let text = serde_json::to_string(value).unwrap();
        let back: T = serde_json::from_str(&text).unwrap();
        serde_json::to_string(&back).unwrap() == text
    }

    #[test]
    fn serde_round_trips() {
        assert!(same(&DARK));
        assert!(same(&Mode::Tag));
        assert!(same(&Filter::Box));
        assert!(same(&Colorizer::Bins {
            background: Color::rgb(0, 0, 0),
            ramp: vec![WHITE, RED],
        }));
        assert!(same(&Config::default()));
        assert!(same(&Rng::new(7)));
    }
}
