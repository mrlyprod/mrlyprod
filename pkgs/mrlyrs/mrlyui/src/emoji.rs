use crate::frame::over;
use mrlycore::{json, unpng};
use mrlymath::two::Cell2d;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

const ATLAS: &[u8] = include_bytes!("../../../../files/emoji/atlas.png");
const MANIFEST: &str = include_str!("../../../../files/emoji/atlas.json");
const CATALOG: &str = include_str!("../../../../files/emoji/catalog.txt");

pub fn bake(tile: &mut Cell2d, value: &str, k: usize) {
    let Some(sprite) = sprite(value, k) else {
        return;
    };
    if let Some(colors) = tile.cell.colors.as_mut() {
        for (i, px) in sprite.iter().enumerate() {
            over(&mut colors[i], *px);
        }
    }
}

pub fn known(value: &str) -> bool {
    book().map.contains_key(&key(value))
}

pub fn has(c: char) -> bool {
    !c.is_ascii() && singles().contains(&c)
}

pub fn tile(c: char, k: usize) -> Option<&'static [[u8; 4]]> {
    if !has(c) {
        return None;
    }
    sprite(&c.to_string(), k)
}

pub fn bands() -> &'static [Vec<char>] {
    static BANDS: OnceLock<Vec<Vec<char>>> = OnceLock::new();
    BANDS.get_or_init(|| {
        CATALOG
            .split("\n\n")
            .map(|band| band.split_whitespace().filter_map(one).collect())
            .filter(|band: &Vec<char>| !band.is_empty())
            .collect()
    })
}

fn one(value: &str) -> Option<char> {
    let name = key(value);
    let mut chars = name.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

fn singles() -> &'static HashSet<char> {
    static SINGLES: OnceLock<HashSet<char>> = OnceLock::new();
    SINGLES.get_or_init(|| book().map.keys().filter_map(|name| one(name)).collect())
}

fn key(value: &str) -> String {
    value.chars().filter(|&c| c != '\u{fe0f}').collect()
}

struct Book {
    cell: usize,
    cols: usize,
    map: HashMap<String, usize>,
}

fn book() -> &'static Book {
    static BOOK: OnceLock<Book> = OnceLock::new();
    BOOK.get_or_init(|| {
        let manifest = json::parse(MANIFEST).unwrap_or_else(|_| json::Json::Null);
        let cell = manifest["cell"].as_i64().unwrap_or(0) as usize;
        let cols = manifest["cols"].as_i64().unwrap_or(0) as usize;
        let mut map = HashMap::new();
        if let Some(entries) = manifest["map"].as_object() {
            for (name, index) in entries.iter() {
                if let Some(index) = index.as_i64() {
                    map.insert(name.clone(), index as usize);
                }
            }
        }
        Book { cell, cols, map }
    })
}

fn sheet() -> Option<&'static (usize, Vec<[u8; 4]>)> {
    static SHEET: OnceLock<Option<(usize, Vec<[u8; 4]>)>> = OnceLock::new();
    SHEET
        .get_or_init(|| unpng(ATLAS).ok().map(|(w, _, pixels)| (w, pixels)))
        .as_ref()
}

fn sprite(value: &str, k: usize) -> Option<&'static [[u8; 4]]> {
    static CACHE: OnceLock<Mutex<HashMap<(String, usize), &'static [[u8; 4]]>>> = OnceLock::new();
    if k == 0 {
        return None;
    }
    let name = key(value);
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cache = cache.lock().ok()?;
    if let Some(&sprite) = cache.get(&(name.clone(), k)) {
        return Some(sprite);
    }
    let book = book();
    let &index = book.map.get(&name)?;
    let (width, pixels) = sheet()?;
    let width = *width;
    let ox = (index % book.cols) * book.cell;
    let oy = (index / book.cols) * book.cell;
    let mut scaled = Vec::with_capacity(k * k);
    for ty in 0..k {
        let y0 = ty * book.cell / k;
        let y1 = ((ty + 1) * book.cell / k).max(y0 + 1);
        for tx in 0..k {
            let x0 = tx * book.cell / k;
            let x1 = ((tx + 1) * book.cell / k).max(x0 + 1);
            let mut sums = [0u64; 4];
            for y in y0..y1 {
                for x in x0..x1 {
                    let px = pixels[(oy + y) * width + ox + x];
                    let alpha = px[3] as u64;
                    sums[0] += px[0] as u64 * alpha;
                    sums[1] += px[1] as u64 * alpha;
                    sums[2] += px[2] as u64 * alpha;
                    sums[3] += alpha;
                }
            }
            let area = ((y1 - y0) * (x1 - x0)) as u64;
            scaled.push(if sums[3] == 0 {
                [0, 0, 0, 0]
            } else {
                [
                    (sums[0] / sums[3]) as u8,
                    (sums[1] / sums[3]) as u8,
                    (sums[2] / sums[3]) as u8,
                    (sums[3] / area) as u8,
                ]
            });
        }
    }
    let sprite: &'static [[u8; 4]] = Box::leak(scaled.into_boxed_slice());
    cache.insert((name, k), sprite);
    Some(sprite)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::solid_tile;

    const SKINNED: [&str; 22] = [
        "💣", "⛳", "🌱", "🌿", "🍀", "🌸", "🌼", "🌻", "🍁", "🍄", "🌴", "🌵", "🌟", "🍎", "🍋",
        "🍇", "🍓", "🍑", "🥝", "🍒", "🥥", "🍊",
    ];

    #[test]
    fn every_skin_emoji_is_in_the_atlas() {
        for value in SKINNED {
            assert!(known(value), "{value} missing from the atlas");
        }
    }

    #[test]
    fn bake_paints_visible_pixels() {
        for k in [8, 12, 16, 32, 48] {
            let mut tile = solid_tile(k, [0, 0, 0, 0]);
            bake(&mut tile, "💣", k);
            let colors = tile.cell.colors.as_ref().unwrap();
            let visible = colors.iter().filter(|c| c[3] > 0).count();
            assert!(visible > k * k / 4, "k={k} painted {visible}");
        }
    }

    #[test]
    fn bake_skips_unknown_glyphs() {
        let mut tile = solid_tile(8, [0, 0, 0, 0]);
        bake(&mut tile, "♔", 8);
        assert!(tile.cell.colors.as_ref().unwrap().iter().all(|c| c[3] == 0));
        assert!(!known("♔"));
    }

    #[test]
    fn variation_selectors_do_not_hide_glyphs() {
        assert!(known("☠\u{fe0f}"));
        assert!(known("☠"));
    }

    #[test]
    fn cache_returns_the_same_slice() {
        let first = sprite("🍎", 16).unwrap();
        let second = sprite("🍎", 16).unwrap();
        assert!(std::ptr::eq(first, second));
    }
}
