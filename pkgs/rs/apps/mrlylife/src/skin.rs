use mrlyskin::{Skin, Visual};

/// Builds the tile skin: pen 0 for dead cells, pen 1 for live ones.
pub fn skin() -> Skin {
    Skin::new(vec![Visual::pen(0), Visual::pen(1)])
}

/// Lists every skin the app offers, paired with its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
