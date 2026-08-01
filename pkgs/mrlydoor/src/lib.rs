use mrlycore::errors::{value_error, Result};
use mrlycore::{json, unpng, Json};
use std::collections::HashMap;
use ttf_parser::{Face, RasterImageFormat};

pub mod raster;

pub const CELL: usize = 32;

pub fn key(grapheme: &str) -> String {
    grapheme.chars().filter(|&c| c != '\u{fe0f}').collect()
}

pub fn catalog(text: &str) -> Vec<String> {
    text.split_whitespace().map(str::to_string).collect()
}

pub fn emoji_atlas(font: &[u8], entries: &[String]) -> Result<(Vec<u8>, Json)> {
    let Ok(face) = Face::parse(font, 0) else {
        return value_error("unreadable font.");
    };
    if entries.is_empty() {
        return value_error("empty catalog.");
    }
    let mut sprites = Vec::with_capacity(entries.len());
    let mut map = json!({});
    for (index, entry) in entries.iter().enumerate() {
        let name = key(entry);
        let mut chars = name.chars();
        let (Some(single), None) = (chars.next(), chars.next()) else {
            return value_error(format!("not a single glyph: {entry}."));
        };
        if map.get(name.as_str()).is_some() {
            return value_error(format!("duplicate entry: {entry}."));
        }
        let Some(glyph) = face.glyph_index(single) else {
            return value_error(format!("no glyph for {entry}."));
        };
        let Some(image) = face.glyph_raster_image(glyph, u16::MAX) else {
            return value_error(format!("no bitmap for {entry}."));
        };
        if image.format != RasterImageFormat::PNG {
            return value_error(format!("not a png bitmap: {entry}."));
        }
        let (width, height, pixels) = unpng(image.data)?;
        sprites.push(shrink(&pixels, width, height));
        map[name.as_str()] = json!(index);
    }
    pack(sprites, map)
}

pub fn symbol_atlas(
    material: &[u8],
    codepoints: &str,
    symbols2: &[u8],
    entries: &[String],
) -> Result<(Vec<u8>, Json)> {
    let Ok(material) = Face::parse(material, 0) else {
        return value_error("unreadable symbols font.");
    };
    let Ok(symbols2) = Face::parse(symbols2, 0) else {
        return value_error("unreadable symbols2 font.");
    };
    if entries.is_empty() {
        return value_error("empty catalog.");
    }
    let mut points = HashMap::new();
    for line in codepoints.lines() {
        if let Some((name, code)) = line.split_once(' ') {
            let Ok(code) = u32::from_str_radix(code.trim(), 16) else {
                return value_error(format!("bad codepoint line: {line}."));
            };
            let Some(single) = char::from_u32(code) else {
                return value_error(format!("bad codepoint line: {line}."));
            };
            points.insert(name.to_string(), single);
        }
    }
    let mut sprites = Vec::with_capacity(entries.len());
    let mut map = json!({});
    for (index, entry) in entries.iter().enumerate() {
        if map.get(entry.as_str()).is_some() {
            return value_error(format!("duplicate entry: {entry}."));
        }
        let (face, single) = match points.get(entry.as_str()) {
            Some(&single) => (&material, single),
            None => {
                let mut chars = entry.chars();
                let (Some(single), None) = (chars.next(), chars.next()) else {
                    return value_error(format!("not an icon name or symbol: {entry}."));
                };
                (&symbols2, single)
            }
        };
        let Some(glyph) = face.glyph_index(single) else {
            return value_error(format!("no glyph for {entry}."));
        };
        let mut outline = raster::Outline::new();
        if face.outline_glyph(glyph, &mut outline).is_none() || outline.is_empty() {
            return value_error(format!("no outline for {entry}."));
        }
        let cover = raster::coverage(&outline, face.units_per_em() as f64, CELL);
        let cell = cover.iter().map(|&c| [c, c, c, c]).collect();
        sprites.push((CELL, CELL, cell));
        map[entry.as_str()] = json!(index);
    }
    pack(sprites, map)
}

fn pack(sprites: Vec<(usize, usize, Vec<[u8; 4]>)>, map: Json) -> Result<(Vec<u8>, Json)> {
    let cols = (1usize..).find(|c| c * c >= sprites.len()).unwrap_or(1);
    let rows = sprites.len().div_ceil(cols);
    let width = cols * CELL;
    let height = rows * CELL;
    let mut canvas = vec![[0u8; 4]; width * height];
    for (index, (w, h, sprite)) in sprites.iter().enumerate() {
        let ox = (index % cols) * CELL + (CELL - w) / 2;
        let oy = (index / cols) * CELL + (CELL - h) / 2;
        for y in 0..*h {
            for x in 0..*w {
                canvas[(oy + y) * width + ox + x] = sprite[y * w + x];
            }
        }
    }
    let png = mrlycore::png(&canvas, width, height, 1)?;
    let manifest = json!({
        "cell": CELL,
        "cols": cols,
        "rows": rows,
        "count": sprites.len(),
        "version": 1,
        "map": map,
    });
    Ok((png, manifest))
}

fn shrink(pixels: &[[u8; 4]], width: usize, height: usize) -> (usize, usize, Vec<[u8; 4]>) {
    let side = width.max(height).max(1);
    let tw = (width * CELL).div_ceil(side).clamp(1, CELL);
    let th = (height * CELL).div_ceil(side).clamp(1, CELL);
    let mut out = Vec::with_capacity(tw * th);
    for ty in 0..th {
        let y0 = ty * height / th;
        let y1 = ((ty + 1) * height / th).max(y0 + 1);
        for tx in 0..tw {
            let x0 = tx * width / tw;
            let x1 = ((tx + 1) * width / tw).max(x0 + 1);
            let mut sums = [0u64; 4];
            for y in y0..y1 {
                for x in x0..x1 {
                    let px = pixels[y * width + x];
                    let alpha = px[3] as u64;
                    sums[0] += px[0] as u64 * alpha;
                    sums[1] += px[1] as u64 * alpha;
                    sums[2] += px[2] as u64 * alpha;
                    sums[3] += alpha;
                }
            }
            let area = ((y1 - y0) * (x1 - x0)) as u64;
            let alpha = sums[3] / area;
            out.push(if sums[3] == 0 {
                [0, 0, 0, 0]
            } else {
                [
                    (sums[0] / sums[3]) as u8,
                    (sums[1] / sums[3]) as u8,
                    (sums[2] / sums[3]) as u8,
                    alpha as u8,
                ]
            });
        }
    }
    (tw, th, out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

    fn font() -> Vec<u8> {
        fs::read(format!("{ROOT}/files/vendor/emoji.ttf")).unwrap()
    }

    #[test]
    fn key_strips_variation_selectors() {
        assert_eq!(key("\u{2620}\u{fe0f}"), "\u{2620}");
        assert_eq!(key("💣"), "💣");
    }

    #[test]
    fn small_atlas_has_visible_cells() {
        let entries = catalog("💣 ⛳ 🍎");
        let (png, manifest) = emoji_atlas(&font(), &entries).unwrap();
        assert_eq!(manifest["cell"], json!(CELL));
        assert_eq!(manifest["cols"], json!(2));
        assert_eq!(manifest["rows"], json!(2));
        assert_eq!(manifest["map"]["💣"], json!(0));
        assert_eq!(manifest["map"]["🍎"], json!(2));
        let (w, h, pixels) = unpng(&png).unwrap();
        assert_eq!((w, h), (2 * CELL, 2 * CELL));
        let visible = pixels.iter().filter(|p| p[3] > 0).count();
        assert!(visible > 3 * CELL * CELL / 4);
        let empty = &pixels[(2 * CELL - 1) * w + CELL..];
        assert!(empty.iter().all(|p| p[3] == 0));
    }

    #[test]
    fn atlas_rejects_bad_catalogs() {
        let font = font();
        assert!(emoji_atlas(&font, &[]).is_err());
        assert!(emoji_atlas(&font, &catalog("💣 💣")).is_err());
        assert!(emoji_atlas(&font, &catalog("ab")).is_err());
        assert!(emoji_atlas(&font, &catalog("♔")).is_err());
    }

    #[test]
    fn small_symbol_atlas_serves_both_fonts() {
        let material = fs::read(format!("{ROOT}/files/vendor/symbols.ttf")).unwrap();
        let codepoints =
            fs::read_to_string(format!("{ROOT}/files/vendor/symbols.codepoints")).unwrap();
        let symbols2 = fs::read(format!("{ROOT}/files/vendor/symbols2.ttf")).unwrap();
        let entries = catalog("close ♔");
        let (png, manifest) = symbol_atlas(&material, &codepoints, &symbols2, &entries).unwrap();
        assert_eq!(manifest["map"]["close"], json!(0));
        assert_eq!(manifest["map"]["♔"], json!(1));
        let (w, h, pixels) = unpng(&png).unwrap();
        assert_eq!((w, h), (2 * CELL, CELL));
        assert!(pixels.iter().filter(|p| p[3] > 0).count() > CELL * CELL / 8);
        assert!(pixels
            .iter()
            .all(|p| p[0] == p[3] && p[1] == p[3] && p[2] == p[3]));
        let unknown = symbol_atlas(&material, &codepoints, &symbols2, &catalog("nonsuch_zz"));
        assert!(unknown.is_err());
    }

    #[test]
    fn shipped_symbol_atlas_matches_the_catalog() {
        let material = fs::read(format!("{ROOT}/files/vendor/symbols.ttf")).unwrap();
        let codepoints =
            fs::read_to_string(format!("{ROOT}/files/vendor/symbols.codepoints")).unwrap();
        let symbols2 = fs::read(format!("{ROOT}/files/vendor/symbols2.ttf")).unwrap();
        let entries =
            catalog(&fs::read_to_string(format!("{ROOT}/files/symbols/catalog.txt")).unwrap());
        let (png, manifest) = symbol_atlas(&material, &codepoints, &symbols2, &entries).unwrap();
        for piece in "♔♕♖♗♘♙♚♛♜♝♞♟".chars() {
            assert!(manifest["map"].get(piece.to_string().as_str()).is_some());
        }
        let shipped_png = fs::read(format!("{ROOT}/files/symbols/atlas.png")).unwrap();
        let shipped_manifest =
            fs::read_to_string(format!("{ROOT}/files/symbols/atlas.json")).unwrap();
        assert_eq!(png, shipped_png);
        assert_eq!(manifest.pretty(), shipped_manifest.trim_end());
    }

    #[test]
    fn shipped_atlas_matches_the_catalog() {
        let entries =
            catalog(&fs::read_to_string(format!("{ROOT}/files/emoji/catalog.txt")).unwrap());
        let (png, manifest) = emoji_atlas(&font(), &entries).unwrap();
        let shipped_png = fs::read(format!("{ROOT}/files/emoji/atlas.png")).unwrap();
        let shipped_manifest =
            fs::read_to_string(format!("{ROOT}/files/emoji/atlas.json")).unwrap();
        assert_eq!(png, shipped_png);
        assert_eq!(manifest.pretty(), shipped_manifest.trim_end());
    }
}
