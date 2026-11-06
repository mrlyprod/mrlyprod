mod lzw;
mod map;
mod plan;
mod quant;
mod read;
mod write;

pub type Result<T> = std::result::Result<T, String>;

pub type Movie = (usize, usize, Vec<Vec<[u8; 4]>>, Vec<u16>);

pub const LIMIT: usize = 1 << 26;

pub(crate) const KEEP: u8 = 1;
pub(crate) const BACKGROUND: u8 = 2;
pub(crate) const PREVIOUS: u8 = 3;

pub use plan::{encode, encode_rgba};
pub use read::decode;

pub fn scale(indices: &[u8], width: usize, factor: usize) -> Result<Vec<u8>> {
    if width == 0 || !indices.len().is_multiple_of(width) {
        return Err(format!(
            "{} indices do not fill rows of {width}.",
            indices.len()
        ));
    }
    if factor == 0 {
        return Err("a scale of 0 leaves no pixels.".into());
    }
    let size = factor
        .checked_mul(factor)
        .and_then(|block| block.checked_mul(indices.len()))
        .filter(|&size| size <= LIMIT)
        .ok_or_else(|| {
            format!(
                "{} indices scaled by {factor} pass the limit of {LIMIT} pixels.",
                indices.len()
            )
        })?;
    let mut out = Vec::with_capacity(size);
    for row in indices.chunks(width) {
        let start = out.len();
        for &index in row {
            out.extend(std::iter::repeat_n(index, factor));
        }
        for _ in 1..factor {
            out.extend_from_within(start..start + width * factor);
        }
    }
    Ok(out)
}
