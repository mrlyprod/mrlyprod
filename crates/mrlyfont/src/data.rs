use mrlycore::data::{shuffle, Well};
use mrlycore::{json, Json};

/// Returns the wells this crate pours: the glyph enumeration alone.
pub fn wells() -> Vec<Box<dyn Well>> {
    vec![Box::new(Glyphs)]
}

struct Glyphs;

impl Well for Glyphs {
    fn name(&self) -> &str {
        "glyphs"
    }
    fn about(&self) -> &str {
        "Every supported glyph with its Unicode name, bitmap rows and descender flag."
    }
    fn pour(&self, seed: u64, count: usize) -> Vec<Json> {
        let mut rows: Vec<Json> = crate::all()
            .iter()
            .map(|glyph| {
                json!({
                    "char": glyph.char.to_string(),
                    "name": crate::name_of(glyph.char),
                    "rows": glyph.rows.clone(),
                    "descends": crate::descends(glyph.char),
                })
            })
            .collect();
        shuffle(&mut rows, seed);
        rows.truncate(count);
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_full_pour_covers_the_font() {
        let rows = Glyphs.pour(7, 999);
        assert_eq!(rows.len(), crate::supported().len());
        let mut chars: Vec<String> = rows
            .iter()
            .map(|row| row["char"].as_str().unwrap().to_string())
            .collect();
        chars.sort();
        chars.dedup();
        assert_eq!(chars.len(), rows.len());
    }

    #[test]
    fn pours_replay_and_prefix() {
        let a = Glyphs.pour(3, 5);
        let b = Glyphs.pour(3, 5);
        assert_eq!(a, b);
        assert_eq!(a[..], Glyphs.pour(3, 9)[..5]);
    }

    #[test]
    fn the_seed_deals_the_order() {
        assert_ne!(Glyphs.pour(1, 108), Glyphs.pour(2, 108));
    }

    #[test]
    fn rows_carry_the_bitmap_shape() {
        for row in Glyphs.pour(0, 4) {
            let rows = row["rows"].as_array().unwrap();
            assert!(rows.len() >= 5);
            assert!(row["name"].as_str().is_some());
            assert!(row["descends"].as_bool().is_some());
        }
    }
}
