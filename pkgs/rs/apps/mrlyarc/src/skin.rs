use mrlyskin::{Skin, Visual};

/// The pen count of the arc palette.
pub const PENS: usize = 10;

/// Builds the tiles skin: one pen per color.
pub fn skin() -> Skin {
    Skin::new((0..PENS).map(Visual::pen).collect())
}

/// Lists every named skin the app owns.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
