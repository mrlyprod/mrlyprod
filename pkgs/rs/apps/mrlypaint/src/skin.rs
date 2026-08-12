use mrlyskin::{Skin, Visual};

/// The number of pens the tiles skin expects.
pub const PENS: usize = 16;

/// Builds the tiles skin with one pen visual per pen.
pub fn skin() -> Skin {
    Skin::new((0..PENS).map(Visual::pen).collect())
}

/// Returns the app's skins paired with their names.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
