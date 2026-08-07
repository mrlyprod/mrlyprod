use super::{Skin, Visual};

pub fn skin() -> Skin {
    Skin::new(vec![
        Visual::none(),
        Visual::pen(0).design(),
        Visual::pen(1).design(),
        Visual::pen(2).design(),
    ])
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
