use super::{Motif, Skin, Visual};

pub fn skin() -> Skin {
    Skin::new(vec![
        Visual::none(),
        Visual::pen(0).design(),
        Visual::pen(1),
        Visual::pen(2),
        Visual::pen(3),
        Visual {
            motif: Some(Motif::Name("net".to_string())),
            ..Visual::pen(4)
        },
    ])
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
