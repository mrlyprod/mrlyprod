use super::{Skin, Visual};

pub fn skin() -> Skin {
    Skin::new(vec![Visual::pen(0), Visual::pen(1)])
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
