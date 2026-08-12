use mrlyskin::{Skin, Visual};

/// Builds the tile skin: bare cells, then a pen each for head, body and food.
pub fn skin() -> Skin {
    Skin::new(vec![
        Visual::none(),
        Visual::pen(0).design(),
        Visual::pen(1).design(),
        Visual::pen(2).design(),
    ])
}

/// Lists every skin the app offers, paired with its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("tiles", skin())]
}
