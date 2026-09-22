//! The alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke.
//!
//! - `bitmaps` holds the raw on/off rows; `glyph` builds them into glyphs and trims them.
//! - `pens` holds every glyph's hand-penned strokes; `paths` reads them and drafts new ones.
//! - `names` gives a character its Unicode name.
//! - `raster` lays a text out as one 0/1 grid; `animate` writes it cell by cell and loops the cycle.
//!
//! The font is the uppers, their corner-rounded lowers, the digits, the punctuation and arrows, and four specials; most glyphs are five rows of five, the ten that dip below the baseline are seven.
//!
//! The doors: [`glyph`](crate::font::glyph()), [`supported`](crate::font::supported()), [`map`](crate::font::map()), [`raster`](crate::font::raster()), [`path`](crate::font::path()), [`floor`](crate::font::floor()), [`animate`](crate::font::animate()), [`merge`](crate::font::merge()), [`cycle`](crate::font::cycle()) and [`name_of`](crate::font::name_of()).
//!
//! `cargo run -p mrlyrs --example pen` prints the pen tables; `-- X` drafts one glyph and its stroke floor.

/// The stroke-order writing animations of text.
pub mod animate;
/// The raw bitmap tables of the font.
pub mod bitmaps;
/// The glyph, its trimming and descenders, and the builders for uppers, lowers, digits, extras and specials.
pub mod glyph;
/// The Unicode names of the font's characters.
pub mod names;
/// The stroke orders that write each character.
pub mod paths;
/// The hand-penned stroke tables, one per glyph.
pub mod pens;
/// The 0/1 grid a text renders to.
pub mod raster;

use std::collections::BTreeMap;
use std::sync::OnceLock;

pub use animate::{animate, cycle, merge, Anim, FPS, HOLD};
pub use glyph::{all, trim, Glyph};
pub use names::name_of;
pub use paths::{draft, floor, path, strokes};
pub use raster::raster;

fn book() -> &'static BTreeMap<char, Glyph> {
    static BOOK: OnceLock<BTreeMap<char, Glyph>> = OnceLock::new();
    BOOK.get_or_init(|| all().into_iter().map(|g| (g.char, g)).collect())
}

fn order() -> &'static Vec<char> {
    static ORDER: OnceLock<Vec<char>> = OnceLock::new();
    ORDER.get_or_init(|| all().iter().map(|g| g.char).collect())
}

/// Returns an owned copy of the character's glyph, or None outside the font.
///
/// ```
/// let a = mrlyrs::font::glyph('a').unwrap();
/// assert_eq!(a.rows[0], "01110");
/// assert_eq!(mrlyrs::font::glyph('\u{6f22}'), None);
/// ```
pub fn glyph(c: char) -> Option<Glyph> {
    book().get(&c).cloned()
}

/// Returns every character in the font, in font order.
///
/// ```
/// let font = mrlyrs::font::supported();
/// assert_eq!((font.len(), font[0]), (108, 'A'));
/// ```
pub fn supported() -> Vec<char> {
    order().clone()
}

/// Returns the whole font as a map from character to bitmap rows.
pub fn map() -> BTreeMap<char, Vec<String>> {
    all().into_iter().map(|g| (g.char, g.rows)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const WORDMARK: &str = "MRLYPROD";

    #[test]
    fn the_cached_glyph_is_the_built_glyph() {
        for want in all() {
            assert_eq!(
                glyph(want.char).as_ref(),
                Some(&want),
                "{} drifted",
                want.char
            );
        }
        assert_eq!(glyph('\u{6f22}'), None);
    }
    #[test]
    fn mrlyprod_union_is_x() {
        let mut union = vec![vec!['0'; 5]; 5];
        for c in WORDMARK.chars() {
            let g = glyph(c).unwrap();
            for (y, row) in g.rows.iter().enumerate() {
                for (x, ch) in row.chars().enumerate() {
                    if ch == '1' {
                        union[y][x] = '1';
                    }
                }
            }
        }
        let folded: Vec<String> = union
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
        assert_eq!(folded, glyph('X').unwrap().rows);
    }
    #[test]
    fn lowercase_rounds_corners() {
        let a = glyph('a').unwrap();
        assert_eq!(a.rows, vec!["01110", "10001", "11111", "10001", "00000"]);
        assert_eq!(glyph('A').unwrap().rows[0], "11111");
    }
    #[test]
    fn count_matches_layout() {
        assert_eq!(supported().len(), 108);
        assert_eq!(glyph::uppers().len(), 26);
        assert_eq!(glyph::lowers().len(), 26);
        assert_eq!(glyph::digits().len(), 10);
        assert_eq!(glyph::extras().len(), 42);
        assert_eq!(glyph::specials().len(), 4);
    }
    #[test]
    fn descenders_flagged() {
        assert!(glyph::descends('$'));
        assert!(glyph::descends('('));
        assert!(!glyph::descends('A'));
    }
    #[test]
    fn serde_round_trips() {
        let mark = glyph('A').unwrap();
        assert_eq!(
            mark,
            serde_json::from_str(&serde_json::to_string(&mark).unwrap()).unwrap()
        );
        let write = animate("MRLY", 1);
        assert_eq!(
            write,
            serde_json::from_str(&serde_json::to_string(&write).unwrap()).unwrap()
        );
    }
}
