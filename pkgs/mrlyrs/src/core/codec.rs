use super::errors::{value_error, Result};
use super::MrlyError;

pub fn png(colors: &[[u8; 4]], width: usize, height: usize, scale: usize) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    if colors.len() != width * height {
        return value_error("colors length must equal width * height.");
    }
    let out_w = width * scale;
    let out_h = height * scale;
    let mut pixels = Vec::with_capacity(out_w * out_h * 4);
    for y in 0..out_h {
        for x in 0..out_w {
            pixels.extend_from_slice(&colors[(y / scale) * width + (x / scale)]);
        }
    }
    encode(&pixels, out_w, out_h)
}

fn encode(pixels: &[u8], width: usize, height: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| MrlyError::Value(e.to_string()))?;
        writer
            .write_image_data(pixels)
            .map_err(|e| MrlyError::Value(e.to_string()))?;
    }
    Ok(bytes)
}

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        let n = (b0 as u32) << 16 | (b1 as u32) << 8 | b2 as u32;
        out.push(BASE64_ALPHABET[(n >> 18 & 0x3f) as usize] as char);
        out.push(BASE64_ALPHABET[(n >> 12 & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 {
            BASE64_ALPHABET[(n >> 6 & 0x3f) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            BASE64_ALPHABET[(n & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn png_signature_and_scaled_size() {
        let colors = vec![
            [255, 0, 0, 255],
            [0, 255, 0, 255],
            [0, 0, 255, 255],
            [255, 255, 0, 255],
        ];
        let bytes = png(&colors, 2, 2, 4).unwrap();
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        assert!(bytes.len() > 100);
    }
    #[test]
    fn png_rejects_bad_inputs() {
        let colors = vec![[0, 0, 0, 255]];
        assert!(png(&colors, 1, 1, 0).is_err());
        assert!(png(&colors, 2, 2, 1).is_err());
    }
    #[test]
    fn base64_matches_rfc_4648_vectors() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
    #[test]
    fn base64_handles_binary() {
        assert_eq!(base64(&[0, 1, 2, 253, 254, 255]), "AAEC/f7/");
    }
}
