use mrlyskin::{Skin, Visual};

/// The number of pens the tiles skin carries.
pub const PENS: usize = 16;

/// Builds the tiles skin: one visual per pen, the void pen first and the fill pen next.
pub fn skin() -> Skin {
    Skin::new((0..PENS).map(Visual::pen).collect())
}

/// Lists the app's only skin, under the name tiles.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
