//! Every public function fed a bad input returns an error instead of panicking.

use std::panic::{AssertUnwindSafe, catch_unwind};

const BAD: &[(&str, fn())] = &[
    ("raster::raster, a char the alphabet has not", || {
        let _ = mrly_font::raster::raster("\u{1f600}");
    }),
    ("paths::path, a char the alphabet has not", || {
        let _ = mrly_font::paths::path('\u{1f600}');
    }),
    ("paths::floor, no rows", || {
        let _ = mrly_font::paths::floor(&[]);
    }),
    ("paths::draft, no rows", || {
        let _ = mrly_font::paths::draft(&[]);
    }),
    ("animate::merge, an empty text", || {
        let _ = mrly_font::animate::merge("", 0);
    }),
    ("animate::cycle, an empty merge", || {
        let _ = mrly_font::animate::cycle(&mrly_font::animate::animate("a", 1), &[], 1);
    }),
    ("animate::cycle, an empty write", || {
        let _ = mrly_font::animate::cycle(&mrly_font::animate::animate("", 0), &[vec![0]], 1);
    }),
];

#[test]
fn no_door_panics_on_a_bad_input() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let panicked: Vec<&str> = BAD
        .iter()
        .filter(|(_, call)| catch_unwind(AssertUnwindSafe(call)).is_err())
        .map(|(name, _)| *name)
        .collect();
    std::panic::set_hook(hook);
    assert!(panicked.is_empty(), "{panicked:?}");
}
