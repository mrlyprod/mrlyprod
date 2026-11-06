mod deflate;
mod huffman;
mod inflate;
mod matcher;
mod read;
mod route;
mod sum;
mod tables;
mod write;
mod zlib;

pub type Result<T> = std::result::Result<T, String>;

pub use read::{decode, header};
pub use sum::{adler32, crc32};
pub use write::{BEST, DEFAULT, encode, encode_indexed, encode_level};
pub use zlib::{compress, decompress};
