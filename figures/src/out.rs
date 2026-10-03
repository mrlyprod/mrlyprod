use crate::board::Board;
use crate::ink;
use mrlyrs::core::error::{value_error, Result};
use png::{AdaptiveFilterType, BitDepth, ColorType, Compression, Encoder, FilterType};
use std::path::PathBuf;

// OUTPUT

/// Returns the workspace root, one level above this crate.
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Writes the board to files/figures/<name>-<theme>.png, dark or light, and announces the one line it printed.
pub fn save(name: &str, board: &Board) -> Result<PathBuf> {
    let name = format!("{name}-{}", ink::name());
    let folder = root().join("files").join("figures");
    std::fs::create_dir_all(&folder)
        .map_err(|e| mrlyrs::Error::Value(format!("cannot make {folder:?}: {e}")))?;
    let path = folder.join(format!("{name}.png"));
    let bytes = png(&board.pixels, board.width, board.height)?;
    if bytes.is_empty() {
        return value_error("the png came back empty.");
    }
    std::fs::write(&path, bytes)
        .map_err(|e| mrlyrs::Error::Value(format!("cannot write {path:?}: {e}")))?;
    println!("figure {name} {}x{}", board.width, board.height);
    Ok(path)
}

// PNG

fn refused(error: png::EncodingError) -> mrlyrs::Error {
    mrlyrs::Error::Value(error.to_string())
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

fn png(pixels: &[[u8; 4]], width: usize, height: usize) -> Result<Vec<u8>> {
    if Some(pixels.len()) != width.checked_mul(height) {
        return value_error("pixels length must equal width * height.");
    }
    let (Ok(wide), Ok(tall)) = (u32::try_from(width), u32::try_from(height)) else {
        return value_error("png side must fit in 32 bits.");
    };
    let mut bytes = Vec::with_capacity(pixels.len() + 128);
    {
        let mut encoder = Encoder::new(&mut bytes, wide, tall);
        encoder.set_compression(Compression::Best);
        encoder.set_filter(FilterType::Paeth);
        encoder.set_adaptive_filter(AdaptiveFilterType::Adaptive);
        let data = match table(pixels) {
            Some(table) => {
                let depth = depth(table.len());
                encoder.set_color(ColorType::Indexed);
                encoder.set_depth(depth);
                encoder.set_palette(plte(&table));
                let veils = trns(&table);
                if !veils.is_empty() {
                    encoder.set_trns(veils);
                }
                indices(pixels, &table, width, height, depth)
            }
            None => {
                encoder.set_color(ColorType::Rgba);
                encoder.set_depth(BitDepth::Eight);
                pixels.concat()
            }
        };
        let mut writer = encoder.write_header().map_err(refused)?;
        writer.write_image_data(&data).map_err(refused)?;
        writer.finish().map_err(refused)?;
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use png::Decoder;

    fn form(bytes: &[u8]) -> (ColorType, BitDepth) {
        let reader = Decoder::new(bytes).read_info().unwrap();
        let info = reader.info();
        (info.color_type, info.bit_depth)
    }
    #[test]
    fn png_packs_few_colors_into_a_palette() {
        let mut pixels = Vec::with_capacity(64 * 64);
        for y in 0..64usize {
            for x in 0..64usize {
                pixels.push(if (x / 8 + y / 8) % 2 == 0 {
                    [17, 34, 51, 255]
                } else {
                    [238, 221, 204, 64]
                });
            }
        }
        let bytes = png(&pixels, 64, 64).unwrap();
        assert_eq!(form(&bytes), (ColorType::Indexed, BitDepth::One));
        assert!(bytes.len() < 1024, "{} bytes", bytes.len());
    }
    #[test]
    fn png_keeps_rgba_past_the_palette() {
        let pixels: Vec<[u8; 4]> = (0..4096u32)
            .map(|i| [(i >> 4) as u8, (i & 15) as u8, (i % 251) as u8, 255])
            .collect();
        let bytes = png(&pixels, 64, 64).unwrap();
        assert_eq!(form(&bytes), (ColorType::Rgba, BitDepth::Eight));
    }
    #[test]
    fn refuses_png() {
        assert!(png(&[[0, 0, 0, 255]], 2, 2).is_err());
        assert!(png(&[], 1 << 40, 0).is_err());
    }
}
