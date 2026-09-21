#![doc = include_str!("README.md")]

/// The cell grid: type bytes with optional per-cell colors and tags.
pub mod cell;
/// The ChaCha8 keystream, a source of random words.
pub mod chacha;
/// The png and gif codecs, rented from the png and gif crates.
pub mod codec;
/// The rgba color, its themes and the fifteen-color palette utils/colors.py stamps.
pub mod colors;
/// The one error type, its Result and the json parser over the rented value.
pub mod error;
/// The paletted image and its rows.
pub mod image;
/// The natural logarithm, written from a series.
pub mod logs;
mod named;
/// The editions that distribute a palette over a cell.
pub mod paint;
/// The colorizers that turn counter values into colors.
pub mod ramp;
/// The pixel resamplers and the hex squash.
pub mod resample;
/// The seeded random stream.
pub mod rng;
/// The global random state: seed once, every draw replays.
pub mod state;
/// The tensor and its dtypes.
pub mod tensor;

pub use cell::{Cell, Mode};
pub use codec::{gif, png, unpng, PNG_MAGIC};
pub use colors::Color;
pub use error::{MrlyError, Result};
pub use image::Image;
pub(crate) use named::named_enum;
pub use ramp::Colorizer;
pub use resample::{hex_fit, hex_size, resample, Filter};
pub use rng::Rng;
pub use serde_json::{json, Value as Json};
pub use tensor::{Dtype, Tensor};

/// An object's entries, kept in insertion order.
pub type Map = serde_json::Map<String, Json>;
