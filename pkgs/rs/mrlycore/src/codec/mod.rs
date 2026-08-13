mod base64;
mod deflate;
mod gif;
mod png;
mod wav;

pub use base64::base64;
pub use deflate::{deflate, inflate};
pub use gif::gif;
pub use png::{png, unpng};
pub use wav::wav;
