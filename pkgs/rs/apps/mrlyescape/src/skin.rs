use mrlyskin::{Motif, Skin, Visual};

/// Builds the maze's tile skin, one visual per cell id from bare floor to player.
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

/// Collects the one tile skin under its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
