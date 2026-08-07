use super::{Skin, Visual};

pub const PENS: usize = 16;

pub fn skin() -> Skin {
    Skin::new((0..PENS).map(Visual::pen).collect())
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
