pub mod glyphs;
pub mod letters;
mod models;
pub mod names;
pub mod raster;
pub mod serializer;
pub mod shape;

use std::collections::BTreeMap;

pub use letters::{all, digits, extras, lowers, specials, uppers};
pub use models::Glyph;
pub use names::name_of;
pub use raster::raster;
pub use serializer::{to_json, to_lists, to_strings};
pub use shape::{descends, trim, DESCENDERS};

pub fn glyph(c: char) -> Option<Glyph> {
    all().into_iter().find(|g| g.char == c)
}

pub fn supported() -> Vec<char> {
    all().iter().map(|g| g.char).collect()
}

pub fn map() -> BTreeMap<char, Vec<String>> {
    all().into_iter().map(|g| (g.char, g.rows)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(uppers().len(), 26);
        assert_eq!(lowers().len(), 26);
        assert_eq!(digits().len(), 10);
        assert_eq!(extras().len(), 42);
        assert_eq!(specials().len(), 4);
    }
    #[test]
    fn descenders_flagged() {
        assert!(descends('$'));
        assert!(descends('('));
        assert!(!descends('A'));
    }
    #[test]
    fn json_is_multiline_and_named() {
        let json = to_json(&all());
        assert!(json.contains("\"name\": \"LATIN CAPITAL LETTER A\""));
        assert!(json.contains("\"rows\": [\n"));
    }
}
