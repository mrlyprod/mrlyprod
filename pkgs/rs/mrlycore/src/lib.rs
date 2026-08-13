#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Ready-made tensors: zeros, ones, noise and carpets in two or three dimensions.
pub mod atoms;
/// Notes, waves and the synth's constants.
pub mod audio;
/// The cell grid: type bytes with optional per-cell colors and tags.
pub mod cell;
/// The ChaCha8 keystream, a seekable source of random words.
pub mod chacha;
/// The encoders and decoders: png, unpng, base64.
pub mod codec;
/// The rgba color and its palettes.
pub mod colors;
/// The well: how a crate declares the datasets it can pour.
pub mod data;
/// The ways paint picks a color within a type's palette.
pub mod enums;
/// The one error type and its Result.
pub mod errors;
/// The paletted image and its rows.
pub mod image;
/// The road from type bytes through a colorizer to rgba pixels.
pub mod io;
/// The JSON value and its parser.
pub mod json;
/// The natural logarithm, written from a series.
pub mod logs;
/// The composer: voices walking chord pools into frames of midi notes.
pub mod music;
/// The editions that distribute a palette over a cell.
pub mod paint;
/// The colorizers that turn counter values into colors.
pub mod ramp;
/// The pixel resamplers and the hex squash.
pub mod resample;
/// The seeded, seekable random stream.
pub mod rng;
/// The global random state: seed once, every draw replays.
pub mod state;
/// The tensor and its dtypes.
pub mod tensor;
/// The tile families, groups and parities.
pub mod tile;
/// Civil time unfolded from epoch days.
pub mod time;
/// Table trig: one turn in a fixed count of samples.
pub mod trig;

pub use audio::{Note, Timbre, Wave};
pub use cell::Cell;
pub use codec::{base64, deflate, gif, inflate, png, unpng, wav};
pub use colors::Color;
pub use enums::Mode;
pub use errors::{MrlyError, Result};
pub use image::Image;
pub use json::{Json, Map};
pub use music::{compose, mix, track, ChordType, Movement, Voice};
pub use ramp::Colorizer;
pub use resample::{hex_fit, hex_size, resample, Filter};
pub use rng::Rng;
pub use tensor::{Dtype, Tensor};
pub use tile::{Base, Group, Parity, Tile};
