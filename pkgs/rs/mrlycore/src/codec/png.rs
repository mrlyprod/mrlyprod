use super::deflate::{inflate, zlib};
use crate::errors::{value_error, Result};
use crate::resample::block;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

const ADLER_MODULO: u32 = 65521;

const CRC32_TABLE: [u32; 256] = crc32_table();

/// Encodes rgba colors as a png, drawing each source pixel as a scale by scale block.
pub fn png(colors: &[[u8; 4]], width: usize, height: usize, scale: usize) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    if colors.len() != width * height {
        return value_error("colors length must equal width * height.");
    }
    let pixels = block(colors, width, height, scale).concat();
    encode(&pixels, width * scale, height * scale)
}

fn encode(pixels: &[u8], width: usize, height: usize) -> Result<Vec<u8>> {
    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&(width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    let mut bytes = Vec::with_capacity(pixels.len() + height + 128);
    bytes.extend_from_slice(&PNG_SIGNATURE);
    chunk(&mut bytes, b"IHDR", &header);
    chunk(&mut bytes, b"IDAT", &zlib(&filter(pixels, width, height)));
    chunk(&mut bytes, b"IEND", &[]);
    Ok(bytes)
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], payload: &[u8]) {
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(payload);
    out.extend_from_slice(&crc32(&[kind, payload]).to_be_bytes());
}

fn filter(pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
    let stride = width * 4;
    let mut out = Vec::with_capacity(height * (stride + 1));
    for row in 0..height {
        out.push(0);
        out.extend_from_slice(&pixels[row * stride..(row + 1) * stride]);
    }
    out
}

const fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut index = 0;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 {
                0xedb8_8320 ^ (value >> 1)
            } else {
                value >> 1
            };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

fn crc32(parts: &[&[u8]]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for part in parts {
        for &byte in *part {
            crc = CRC32_TABLE[((crc ^ byte as u32) & 0xff) as usize] ^ (crc >> 8);
        }
    }
    crc ^ 0xffff_ffff
}

pub(super) fn adler32(bytes: &[u8]) -> u32 {
    let mut low = 1u32;
    let mut high = 0u32;
    for &byte in bytes {
        low = (low + byte as u32) % ADLER_MODULO;
        high = (high + low) % ADLER_MODULO;
    }
    high << 16 | low
}

/// Decodes a png to its width, height, and rgba colors, or an error for broken or exotic files.
pub fn unpng(bytes: &[u8]) -> Result<(usize, usize, Vec<[u8; 4]>)> {
    if bytes.len() < 8 || bytes[..8] != PNG_SIGNATURE {
        return value_error("not a png.");
    }
    let mut at = 8;
    let mut width = 0;
    let mut height = 0;
    let mut color = 0u8;
    let mut depth = 0usize;
    let mut headed = false;
    let mut palette: Vec<[u8; 4]> = Vec::new();
    let mut idat = Vec::new();
    while at + 12 <= bytes.len() {
        let len = u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
        let len = len as usize;
        if at + 12 + len > bytes.len() {
            return value_error("truncated png chunk.");
        }
        let kind = &bytes[at + 4..at + 8];
        let body = &bytes[at + 8..at + 8 + len];
        match kind {
            b"IHDR" => {
                if len != 13 || body[10] != 0 || body[11] != 0 || body[12] != 0 {
                    return value_error("unsupported png header.");
                }
                width = u32::from_be_bytes([body[0], body[1], body[2], body[3]]) as usize;
                height = u32::from_be_bytes([body[4], body[5], body[6], body[7]]) as usize;
                depth = body[8] as usize;
                color = body[9];
                headed = true;
            }
            b"PLTE" => palette = body.chunks(3).map(|c| [c[0], c[1], c[2], 255]).collect(),
            b"tRNS" => {
                for (i, &alpha) in body.iter().enumerate() {
                    if i < palette.len() {
                        palette[i][3] = alpha;
                    }
                }
            }
            b"IDAT" => idat.extend_from_slice(body),
            b"IEND" => break,
            _ => {}
        }
        at += 12 + len;
    }
    if !headed || width == 0 || height == 0 {
        return value_error("missing png header.");
    }
    let channels = match color {
        0 => 1,
        2 => 3,
        3 => 1,
        4 => 2,
        6 => 4,
        _ => return value_error("unsupported png color type."),
    };
    let packed = matches!(color, 0 | 3) && matches!(depth, 1 | 2 | 4);
    if depth != 8 && !packed {
        return value_error("unsupported png bit depth.");
    }
    let raw = inflate(&idat)?;
    let line = (width * channels * depth).div_ceil(8);
    if raw.len() != height * (line + 1) {
        return value_error("bad png data length.");
    }
    let bpp = (channels * depth).div_ceil(8);
    let data = unfilter(&raw, line, height, bpp)?;
    let sample = |y: usize, i: usize| {
        if depth == 8 {
            data[y * line + i]
        } else {
            let byte = data[y * line + i * depth / 8];
            byte >> (8 - depth - i * depth % 8) & ((1 << depth) - 1)
        }
    };
    let full = ((1u32 << depth) - 1) as u8;
    let mut colors = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let px = |c: usize| sample(y, x * channels + c);
            colors.push(match color {
                0 => {
                    let g = (px(0) as u32 * 255 / full as u32) as u8;
                    [g, g, g, 255]
                }
                2 => [px(0), px(1), px(2), 255],
                3 => match palette.get(px(0) as usize) {
                    Some(&entry) => entry,
                    None => return value_error("palette index out of range."),
                },
                4 => [px(0), px(0), px(0), px(1)],
                _ => [px(0), px(1), px(2), px(3)],
            });
        }
    }
    Ok((width, height, colors))
}

fn unfilter(raw: &[u8], stride: usize, height: usize, bpp: usize) -> Result<Vec<u8>> {
    let mut out = vec![0u8; stride * height];
    for row in 0..height {
        let kind = raw[row * (stride + 1)];
        let line = &raw[row * (stride + 1) + 1..(row + 1) * (stride + 1)];
        for x in 0..stride {
            let left = if x >= bpp {
                out[row * stride + x - bpp]
            } else {
                0
            };
            let up = if row > 0 {
                out[(row - 1) * stride + x]
            } else {
                0
            };
            let corner = if row > 0 && x >= bpp {
                out[(row - 1) * stride + x - bpp]
            } else {
                0
            };
            let guess = match kind {
                0 => 0,
                1 => left,
                2 => up,
                3 => ((left as u32 + up as u32) / 2) as u8,
                4 => paeth(left, up, corner),
                _ => return value_error("bad png filter."),
            };
            out[row * stride + x] = line[x].wrapping_add(guess);
        }
    }
    Ok(out)
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = a as i32 + b as i32 - c as i32;
    let pa = (p - a as i32).abs();
    let pb = (p - b as i32).abs();
    let pc = (p - c as i32).abs();
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
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
        assert_eq!(&bytes[16..20], &8u32.to_be_bytes());
        assert_eq!(&bytes[20..24], &8u32.to_be_bytes());
    }
    #[test]
    fn png_rejects_bad_inputs() {
        let colors = vec![[0, 0, 0, 255]];
        assert!(png(&colors, 1, 1, 0).is_err());
        assert!(png(&colors, 2, 2, 1).is_err());
    }
    #[test]
    fn png_has_expected_chunk_layout() {
        let bytes = png(&[[1, 2, 3, 4]], 1, 1, 1).unwrap();
        assert_eq!(&bytes[0..8], &PNG_SIGNATURE);
        assert_eq!(&bytes[8..12], &13u32.to_be_bytes());
        assert_eq!(&bytes[12..16], b"IHDR");
        assert_eq!(&bytes[16..20], &1u32.to_be_bytes());
        assert_eq!(&bytes[20..24], &1u32.to_be_bytes());
        assert_eq!(&bytes[24..29], &[8, 6, 0, 0, 0]);
        assert_eq!(
            &bytes[29..33],
            &crc32(&[b"IHDR", &bytes[16..29]]).to_be_bytes()
        );
        let idat_len = u32::from_be_bytes([bytes[33], bytes[34], bytes[35], bytes[36]]) as usize;
        assert_eq!(&bytes[37..41], b"IDAT");
        let payload = &bytes[41..41 + idat_len];
        assert_eq!(payload, &zlib(&[0, 1, 2, 3, 4])[..]);
        assert_eq!(inflate(payload).unwrap(), [0, 1, 2, 3, 4]);
        let crc_at = 41 + idat_len;
        assert_eq!(
            &bytes[crc_at..crc_at + 4],
            &crc32(&[b"IDAT", payload]).to_be_bytes()
        );
        let iend = crc_at + 4;
        assert_eq!(&bytes[iend..iend + 4], &0u32.to_be_bytes());
        assert_eq!(&bytes[iend + 4..iend + 8], b"IEND");
        assert_eq!(&bytes[iend + 8..iend + 12], &0xae42_6082u32.to_be_bytes());
        assert_eq!(bytes.len(), iend + 12);
    }
    #[test]
    fn crc32_matches_known_vectors() {
        assert_eq!(crc32(&[b"IEND"]), 0xae42_6082);
        assert_eq!(crc32(&[b""]), 0x0000_0000);
        assert_eq!(crc32(&[b"123456789"]), 0xcbf4_3926);
        assert_eq!(crc32(&[b"1234", b"56789"]), crc32(&[b"123456789"]));
    }
    #[test]
    fn adler32_matches_known_vectors() {
        assert_eq!(adler32(b"Wikipedia"), 0x11e6_0398);
        assert_eq!(adler32(b""), 0x0000_0001);
    }
    #[test]
    fn unpng_round_trips_the_encoder() {
        let colors = vec![
            [255, 0, 0, 255],
            [0, 255, 0, 128],
            [0, 0, 255, 0],
            [7, 8, 9, 10],
            [250, 251, 252, 253],
            [1, 1, 1, 255],
        ];
        let bytes = png(&colors, 3, 2, 1).unwrap();
        let (w, h, out) = unpng(&bytes).unwrap();
        assert_eq!((w, h), (3, 2));
        assert_eq!(out, colors);
    }
    #[test]
    fn unpng_round_trips_scaled_output() {
        let colors = vec![[9, 9, 9, 255], [0, 0, 0, 0]];
        let bytes = png(&colors, 2, 1, 3).unwrap();
        let (w, h, out) = unpng(&bytes).unwrap();
        assert_eq!((w, h), (6, 3));
        assert_eq!(out[0], [9, 9, 9, 255]);
        assert_eq!(out[5], [0, 0, 0, 0]);
        assert_eq!(out.len(), 18);
    }
    #[test]
    fn unpng_rejects_garbage() {
        assert!(unpng(&[]).is_err());
        assert!(unpng(b"not a png at all").is_err());
        let mut bytes = png(&[[1, 2, 3, 4]], 1, 1, 1).unwrap();
        bytes[25] = 16;
        assert!(unpng(&bytes).is_err());
    }
}
