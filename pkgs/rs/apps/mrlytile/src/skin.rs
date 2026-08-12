use mrlyskin::{Skin, Visual};

/// The number of pens one tile drawing may spend.
pub const PENS: usize = 16;

/// Builds the tile skin: one visual per pen.
pub fn skin() -> Skin {
    Skin::new((0..PENS).map(Visual::pen).collect())
}

/// Lists every skin the app offers, paired with its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
