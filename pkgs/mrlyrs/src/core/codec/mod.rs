//! A png is written paletted whenever 256 colors or fewer fit, rgba otherwise; a gif is always paletted and always loops.

mod gif;
mod png;

pub use gif::gif;
pub use png::{png, unpng, PNG_MAGIC};
