pub mod cell;
pub mod colors;
pub mod error;
pub mod image;
pub mod name;
mod named;
pub mod paint;
pub mod ramp;
pub mod resample;
pub mod rng;
pub mod tensor;

#[doc(hidden)]
pub mod __private {
    pub use serde;
}

pub use cell::{Cell, Mode};
pub use colors::Color;
pub use error::{Error, Result};
pub use image::Image;
pub use ramp::Colorizer;
pub use resample::{Filter, HEX_RATIO, block, hex_fit, hex_size, resample};
pub use rng::Rng;
pub use serde_json::{Value as Json, json};
pub use tensor::{Dtype, Tensor};

pub type Map = serde_json::Map<String, Json>;
