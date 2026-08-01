use crate::frame::over;
use mrlycore::{json, unpng};
use mrlymath::two::Cell2d;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

const ATLAS: &[u8] = include_bytes!("../../../../files/symbols/atlas.png");
const MANIFEST: &str = include_str!("../../../../files/symbols/atlas.json");

pub fn bake(tile: &mut Cell2d, value: &str, k: usize, ink: [u8; 4]) {
    let Some(sprite) = sprite(value, k, ink) else {
        return;
    };
    if let Some(colors) = tile.cell.colors.as_mut() {
        for (i, px) in sprite.iter().enumerate() {
            over(&mut colors[i], *px);
        }
    }
}

pub fn known(value: &str) -> bool {
    book().map.contains_key(value)
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

pub(crate) fn sprite(value: &str, k: usize, ink: [u8; 4]) -> Option<&'static [[u8; 4]]> {
    type Key = (String, usize, [u8; 4]);
    static CACHE: OnceLock<Mutex<HashMap<Key, &'static [[u8; 4]]>>> = OnceLock::new();
    if k == 0 {
        return None;
    }
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cache = cache.lock().ok()?;
    if let Some(&sprite) = cache.get(&(value.to_string(), k, ink)) {
        return Some(sprite);
    }
    let book = book();
    let &index = book.map.get(value)?;
    let (width, _, pixels) = unpng(ATLAS).ok()?;
    let ox = (index % book.cols) * book.cell;
    let oy = (index / book.cols) * book.cell;
    let mut tinted = Vec::with_capacity(k * k);
    for ty in 0..k {
        let y0 = ty * book.cell / k;
        let y1 = ((ty + 1) * book.cell / k).max(y0 + 1);
        for tx in 0..k {
            let x0 = tx * book.cell / k;
            let x1 = ((tx + 1) * book.cell / k).max(x0 + 1);
            let mut sum = 0u64;
            for y in y0..y1 {
                for x in x0..x1 {
                    sum += pixels[(oy + y) * width + ox + x][3] as u64;
                }
            }
            let area = ((y1 - y0) * (x1 - x0)) as u64;
            let cover = sum / area;
            tinted.push([ink[0], ink[1], ink[2], (cover * ink[3] as u64 / 255) as u8]);
        }
    }
    let sprite: &'static [[u8; 4]] = Box::leak(tinted.into_boxed_slice());
    cache.insert((value.to_string(), k, ink), sprite);
    Some(sprite)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::solid_tile;

    #[test]
    fn catalog_serves_icons_and_chess() {
        assert!(known("close"));
        assert!(known("search"));
        for piece in "♔♕♖♗♘♙♚♛♜♝♞♟".chars() {
            assert!(known(&piece.to_string()), "{piece} missing");
        }
        assert!(!known("💣"));
        assert!(!known("nonsuch"));
    }

    #[test]
    fn bake_tints_with_the_ink() {
        let red = [200, 30, 30, 255];
        let blue = [30, 30, 200, 255];
        let mut tile = solid_tile(16, [0, 0, 0, 0]);
        bake(&mut tile, "♔", 16, red);
        let colors = tile.cell.colors.as_ref().unwrap();
        let visible = colors.iter().filter(|c| c[3] > 0).count();
        assert!(visible > 16, "painted {visible}");
        assert!(colors.iter().all(|c| c[3] == 0 || (c[0] >= c[2])));
        let a = sprite("♔", 16, red).unwrap();
        let b = sprite("♔", 16, blue).unwrap();
        assert!(!std::ptr::eq(a, b));
        assert_ne!(a, b);
        assert!(std::ptr::eq(a, sprite("♔", 16, red).unwrap()));
    }

    #[test]
    fn unknown_values_paint_nothing() {
        let mut tile = solid_tile(8, [0, 0, 0, 0]);
        bake(&mut tile, "nonsuch", 8, [255, 255, 255, 255]);
        assert!(tile.cell.colors.as_ref().unwrap().iter().all(|c| c[3] == 0));
    }
}
