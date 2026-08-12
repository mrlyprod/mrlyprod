use mrlyskin::{Skin, Visual};

/// Builds the two-pen skin: the first pen for empty cells, the second for ink.
pub fn skin() -> Skin {
    Skin::new(vec![Visual::pen(0), Visual::pen(1)])
}

/// Lists the app's only skin, under the name tiles.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
