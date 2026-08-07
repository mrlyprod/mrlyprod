use super::{Skin, Visual};

pub const KINDS: usize = 8;

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

pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
