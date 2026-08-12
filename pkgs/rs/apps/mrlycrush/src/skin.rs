use mrlyskin::{Skin, Visual};

/// The most tile kinds a board may ask for.
pub const KINDS: usize = 8;

/// Builds the tile skin: an empty cell, one design per kind, then their crushable twins.
pub fn skin() -> Skin {
    let mut visuals = vec![Visual::none()];
    for kind in 0..KINDS {
        visuals.push(Visual::pen(kind).design());
    }
    for _ in 0..KINDS {
        visuals.push(Visual::pen(KINDS).design());
    }
    Skin::new(visuals)
}

/// Lists the one skin under the name the app draws with.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
