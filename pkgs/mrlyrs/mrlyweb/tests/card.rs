use mrlyweb::card::card_png;
use mrlyweb::registry::catalogue;

#[test]
fn golden_clock_card_is_pinned() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let clock = catalogue()
        .into_iter()
        .find(|a| a.route() == "clock")
        .unwrap()
        .manifest();
    let png = card_png(&clock.route, &clock.title).unwrap();
    assert!(!png.is_empty());
    let mut h = DefaultHasher::new();
    png.hash(&mut h);
    assert_eq!(png.len(), 28652);
    assert_eq!(h.finish(), 13264194717396979273);
}
