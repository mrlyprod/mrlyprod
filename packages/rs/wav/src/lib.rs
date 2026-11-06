mod read;
mod write;

pub type Result<T> = std::result::Result<T, String>;

const PCM: u16 = 1;
const FLOAT: u16 = 3;
const EXTENSIBLE: u16 = 0xFFFE;

#[derive(Clone, Debug, PartialEq)]
pub struct Wav {
    pub rate: usize,
    pub channels: usize,
    pub bits: usize,
    pub float: bool,
    pub samples: Vec<f32>,
}

impl Wav {
    pub fn frames(&self) -> usize {
        self.samples.len() / self.channels.max(1)
    }
    pub fn seconds(&self) -> f64 {
        self.frames() as f64 / self.rate.max(1) as f64
    }
}

pub use read::decode;
pub use write::{dither, encode, encode_float};
