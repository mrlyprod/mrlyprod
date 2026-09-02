#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Notes, waves and the synth's constants.
pub mod audio;
/// The composer: voices walking chord pools into frames of midi notes.
pub mod music;

mod wav;

pub use audio::{Note, Timbre, Wave};
pub use music::{compose, mix, track, ChordType, Movement, Voice};
pub use wav::wav;
