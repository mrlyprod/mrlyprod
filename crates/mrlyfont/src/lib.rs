#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The stroke-order writing animations of text.
pub mod animate;
/// The raw bitmap tables of the font.
pub mod glyphs;
/// The glyph builders for uppers, lowers, digits, extras and specials.
pub mod letters;
mod models;
/// The Unicode names of the font's characters.
pub mod names;
/// The stroke orders that write each character.
pub mod paths;
/// The hand-penned stroke tables, one per glyph.
pub mod pens;
/// The 0/1 grid a text renders to.
pub mod raster;
/// The descenders and the trimming of bitmaps.
pub mod shape;

use std::collections::BTreeMap;
use std::sync::OnceLock;

pub use animate::{animate, cycle, merge, Anim, FPS, HOLD};
pub use letters::all;
pub use models::Glyph;
pub use names::name_of;
pub use paths::{draft, floor, path, strokes};
pub use raster::raster;
pub use shape::trim;

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
/// let a = mrlyfont::glyph('a').unwrap();
/// assert_eq!(a.rows[0], "01110");
/// assert_eq!(mrlyfont::glyph('\u{6f22}'), None);
/// ```
pub fn glyph(c: char) -> Option<Glyph> {
    book().get(&c).cloned()
}

/// Returns every character in the font, in font order.
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
    fn the_book_never_reorders_the_font() {
        let straight: Vec<char> = all().iter().map(|g| g.char).collect();
        assert_eq!(
            supported(),
            straight,
            "the font app seeds its order from supported()"
        );
        assert_eq!(supported().first(), Some(&'A'), "uppers still lead");
    }

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
    fn trim_collapses_blank() {
        let space = glyph(' ').unwrap();
        assert_eq!(trim(&space.rows), vec!["0"; 5]);
    }
    #[test]
    fn trim_drops_edge_columns() {
        let rows = vec!["00100".to_string(), "00100".to_string()];
        assert_eq!(trim(&rows), vec!["1".to_string(), "1".to_string()]);
    }
    #[test]
    fn count_matches_layout() {
        assert_eq!(supported().len(), 108);
        assert_eq!(letters::uppers().len(), 26);
        assert_eq!(letters::lowers().len(), 26);
        assert_eq!(letters::digits().len(), 10);
        assert_eq!(letters::extras().len(), 42);
        assert_eq!(letters::specials().len(), 4);
    }
    #[test]
    fn descenders_flagged() {
        assert!(shape::descends('$'));
        assert!(shape::descends('('));
        assert!(!shape::descends('A'));
    }
}
