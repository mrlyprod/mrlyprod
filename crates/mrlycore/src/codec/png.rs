use crate::errors::{value_error, MrlyError, Result};
use crate::resample::block;
use png::{
    AdaptiveFilterType, BitDepth, ColorType, Compression, Decoder, Encoder, FilterType,
    Transformations,
};

impl From<png::EncodingError> for MrlyError {
    fn from(error: png::EncodingError) -> MrlyError {
        MrlyError::Value(error.to_string())
    }
}

impl From<png::DecodingError> for MrlyError {
    fn from(error: png::DecodingError) -> MrlyError {
        MrlyError::Value(error.to_string())
    }
}

/// Encodes rgba colors as a png, drawing each source pixel as a scale by scale block.
///
/// Images of 256 colors or fewer are written as a palette png at the smallest bit depth that
/// fits; everything else stays 8-bit rgba. Both forms read back through [`unpng`].
pub fn png(colors: &[[u8; 4]], width: usize, height: usize, scale: usize) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    if colors.len() != width * height {
        return value_error("colors length must equal width * height.");
    }
    let pixels = block(colors, width, height, scale);
    let (width, height) = (width * scale, height * scale);
    let mut bytes = Vec::with_capacity(pixels.len() + 128);
    {
        let mut encoder = Encoder::new(&mut bytes, width as u32, height as u32);
        encoder.set_compression(Compression::Best);
        encoder.set_filter(FilterType::Paeth);
        encoder.set_adaptive_filter(AdaptiveFilterType::Adaptive);
        let data = match table(&pixels) {
            Some(table) => {
                let depth = depth(table.len());
                encoder.set_color(ColorType::Indexed);
                encoder.set_depth(depth);
                encoder.set_palette(plte(&table));
                let veils = trns(&table);
                if !veils.is_empty() {
                    encoder.set_trns(veils);
                }
                indices(&pixels, &table, width, height, depth)
            }
            None => {
                encoder.set_color(ColorType::Rgba);
                encoder.set_depth(BitDepth::Eight);
                pixels.concat()
            }
        };
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&data)?;
        writer.finish()?;
    }
    Ok(bytes)
}

fn table(pixels: &[[u8; 4]]) -> Option<Vec<[u8; 4]>> {
    let mut keys: Vec<u32> = Vec::with_capacity(256);
    for pixel in pixels {
        let key = u32::from_be_bytes(*pixel);
        if let Err(slot) = keys.binary_search(&key) {
            if keys.len() == 256 {
                return None;
            }
            keys.insert(slot, key);
        }
    }
    let mut table: Vec<[u8; 4]> = keys.into_iter().map(u32::to_be_bytes).collect();
    table.sort_by_key(|color| color[3]);
    Some(table)
}

fn depth(count: usize) -> BitDepth {
    match count {
        0..=2 => BitDepth::One,
        3..=4 => BitDepth::Two,
        5..=16 => BitDepth::Four,
        _ => BitDepth::Eight,
    }
}

fn plte(table: &[[u8; 4]]) -> Vec<u8> {
    table
        .iter()
        .flat_map(|color| [color[0], color[1], color[2]])
        .collect()
}

fn trns(table: &[[u8; 4]]) -> Vec<u8> {
    let opaque = table
        .iter()
        .position(|color| color[3] == 255)
        .unwrap_or(table.len());
    table[..opaque].iter().map(|color| color[3]).collect()
}

fn indices(
    pixels: &[[u8; 4]],
    table: &[[u8; 4]],
    width: usize,
    height: usize,
    depth: BitDepth,
) -> Vec<u8> {
    let mut lookup: Vec<(u32, u8)> = table
        .iter()
        .enumerate()
        .map(|(slot, color)| (u32::from_be_bytes(*color), slot as u8))
        .collect();
    lookup.sort_unstable();
    let bits = depth as usize;
    let per_byte = 8 / bits;
    let stride = width.div_ceil(per_byte);
    let mut data = vec![0u8; stride * height];
    if width == 0 {
        return data;
    }
    for (y, row) in pixels.chunks_exact(width).enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let key = u32::from_be_bytes(*pixel);
            let slot = lookup.binary_search_by_key(&key, |entry| entry.0).unwrap();
            let shift = 8 - bits * (x % per_byte + 1);
            data[y * stride + x / per_byte] |= lookup[slot].1 << shift;
        }
    }
    data
}

/// Decodes a png to its width, height, and rgba colors, or an error for a broken file.
///
/// Grayscale, rgb, palette and 16-bit files all come back as 8-bit rgba.
pub fn unpng(bytes: &[u8]) -> Result<(usize, usize, Vec<[u8; 4]>)> {
    let mut decoder = Decoder::new(bytes);
    decoder.set_transformations(Transformations::normalize_to_color8());
    let mut reader = decoder.read_info()?;
    let mut data = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut data)?;
    let data = &data[..info.buffer_size()];
    let colors = match info.color_type {
        ColorType::Grayscale => data.iter().map(|&g| [g, g, g, 255]).collect(),
        ColorType::GrayscaleAlpha => data
            .chunks_exact(2)
            .map(|p| [p[0], p[0], p[0], p[1]])
            .collect(),
        ColorType::Rgb => data
            .chunks_exact(3)
            .map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        ColorType::Rgba => data
            .chunks_exact(4)
            .map(|p| [p[0], p[1], p[2], p[3]])
            .collect(),
        ColorType::Indexed => return value_error("png palette did not expand."),
    };
    Ok((info.width as usize, info.height as usize, colors))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn raw(
        color: ColorType,
        depth: BitDepth,
        width: u32,
        height: u32,
        data: &[u8],
        palette: &[u8],
        trns: &[u8],
    ) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut encoder = Encoder::new(&mut bytes, width, height);
        encoder.set_color(color);
        encoder.set_depth(depth);
        if !palette.is_empty() {
            encoder.set_palette(palette.to_vec());
        }
        if !trns.is_empty() {
            encoder.set_trns(trns.to_vec());
        }
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(data).unwrap();
        writer.finish().unwrap();
        bytes
    }
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
        assert!(png(&[], 0, 0, 1).is_err());
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
    fn unpng_expands_gray_and_rgb() {
        let gray = raw(
            ColorType::Grayscale,
            BitDepth::Eight,
            3,
            1,
            &[0, 128, 255],
            &[],
            &[],
        );
        let (w, h, out) = unpng(&gray).unwrap();
        assert_eq!((w, h), (3, 1));
        assert_eq!(
            out,
            [[0, 0, 0, 255], [128, 128, 128, 255], [255, 255, 255, 255]]
        );
        let bits = raw(
            ColorType::Grayscale,
            BitDepth::One,
            3,
            1,
            &[0b1010_0000],
            &[],
            &[],
        );
        let (_, _, out) = unpng(&bits).unwrap();
        assert_eq!(
            out,
            [[255, 255, 255, 255], [0, 0, 0, 255], [255, 255, 255, 255]]
        );
        let veiled = raw(
            ColorType::GrayscaleAlpha,
            BitDepth::Eight,
            1,
            1,
            &[7, 9],
            &[],
            &[],
        );
        assert_eq!(unpng(&veiled).unwrap().2, [[7, 7, 7, 9]]);
        let rgb = raw(
            ColorType::Rgb,
            BitDepth::Eight,
            1,
            2,
            &[1, 2, 3, 4, 5, 6],
            &[],
            &[],
        );
        let (w, h, out) = unpng(&rgb).unwrap();
        assert_eq!((w, h), (1, 2));
        assert_eq!(out, [[1, 2, 3, 255], [4, 5, 6, 255]]);
    }
    #[test]
    fn unpng_expands_palette_and_transparency() {
        let palette = [10, 20, 30, 40, 50, 60, 70, 80, 90];
        let paletted = raw(
            ColorType::Indexed,
            BitDepth::Two,
            4,
            1,
            &[0b0001_1001],
            &palette,
            &[0],
        );
        let (w, h, out) = unpng(&paletted).unwrap();
        assert_eq!((w, h), (4, 1));
        assert_eq!(
            out,
            [
                [10, 20, 30, 0],
                [40, 50, 60, 255],
                [70, 80, 90, 255],
                [40, 50, 60, 255]
            ]
        );
        let opaque = raw(
            ColorType::Indexed,
            BitDepth::Eight,
            2,
            1,
            &[2, 0],
            &palette,
            &[],
        );
        assert_eq!(
            unpng(&opaque).unwrap().2,
            [[70, 80, 90, 255], [10, 20, 30, 255]]
        );
    }
    #[test]
    fn unpng_strips_sixteen_bit_samples() {
        let deep = [0x12, 0x34, 0xff, 0xff, 0x80, 0x00, 0x00, 0xff];
        let bytes = raw(ColorType::Rgba, BitDepth::Sixteen, 1, 1, &deep, &[], &[]);
        assert_eq!(unpng(&bytes).unwrap().2, [[0x12, 0xff, 0x80, 0x00]]);
    }
    #[test]
    fn unpng_rejects_garbage() {
        assert!(unpng(&[]).is_err());
        assert!(unpng(b"not a png at all").is_err());
        let mut bytes = png(&[[1, 2, 3, 4]], 1, 1, 1).unwrap();
        bytes[25] = 16;
        assert!(unpng(&bytes).is_err());
        bytes.truncate(40);
        assert!(unpng(&bytes).is_err());
    }

    fn form(bytes: &[u8]) -> (ColorType, BitDepth) {
        let reader = Decoder::new(bytes).read_info().unwrap();
        let info = reader.info();
        (info.color_type, info.bit_depth)
    }
    #[test]
    fn png_packs_few_colors_into_a_palette() {
        let mut colors = Vec::with_capacity(64 * 64);
        for y in 0..64usize {
            for x in 0..64usize {
                colors.push(if (x / 8 + y / 8) % 2 == 0 {
                    [17, 34, 51, 255]
                } else {
                    [238, 221, 204, 64]
                });
            }
        }
        let bytes = png(&colors, 64, 64, 1).unwrap();
        assert_eq!(form(&bytes), (ColorType::Indexed, BitDepth::One));
        assert!(bytes.len() < 1024, "{} bytes", bytes.len());
        let (w, h, out) = unpng(&bytes).unwrap();
        assert_eq!((w, h), (64, 64));
        assert_eq!(out, colors);
    }
    #[test]
    fn png_keeps_rgba_past_the_palette() {
        let colors: Vec<[u8; 4]> = (0..4096u32)
            .map(|i| [(i >> 4) as u8, (i & 15) as u8, (i % 251) as u8, 255])
            .collect();
        let bytes = png(&colors, 64, 64, 1).unwrap();
        assert_eq!(form(&bytes), (ColorType::Rgba, BitDepth::Eight));
        assert_eq!(unpng(&bytes).unwrap().2, colors);
    }
}
