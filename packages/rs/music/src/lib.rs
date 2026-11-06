mod compose;
mod render;
mod scale;
mod voice;

pub type Result<T> = std::result::Result<T, String>;

pub const ROOT: i64 = 43;
pub const BLUES: &str = "CCCCFFCCGFCG";
pub const BAR: usize = 4;
pub const BEAT: f64 = 0.25;

pub use compose::{compose, frames, random, walk};
pub use render::{render, wav};
pub use scale::{ChordType, Scale, chord, chords, pool};
pub use voice::{Movement, Piece, Voice};
