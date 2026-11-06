mod envelope;
mod mix;
mod note;
mod timbre;
mod wave;

pub type Result<T> = std::result::Result<T, String>;

pub const RATE: usize = 44100;
pub const FADE: f64 = 1.0 / 64.0;
pub const CYCLE: usize = 1024;
pub const MOST: usize = 3600 * RATE;

pub use envelope::Envelope;
pub use mix::{mix, normalize, peak};
pub use note::{NOTES, hertz, midi, name};
pub use timbre::{Timbre, cycle, tone};
pub use wave::Wave;

pub fn samples(seconds: f64) -> usize {
    (seconds * RATE as f64) as usize
}
