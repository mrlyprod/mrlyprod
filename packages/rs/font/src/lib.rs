pub mod animate;
pub mod bitmaps;
pub mod glyph;
pub mod names;
pub mod paths;
pub mod pens;
pub mod raster;

use std::collections::BTreeMap;
use std::sync::OnceLock;

pub use animate::{Anim, FPS, HOLD, animate, cycle, merge};
pub use glyph::{Glyph, all, digits, extras, lower, lowers, specials, trim, uppers};
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

pub fn glyph(c: char) -> Option<Glyph> {
    book().get(&c).cloned()
}

pub fn supported() -> Vec<char> {
    order().clone()
}

pub fn map() -> BTreeMap<char, Vec<String>> {
    all().into_iter().map(|g| (g.char, g.rows)).collect()
}
