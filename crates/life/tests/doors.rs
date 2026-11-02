//! Every public function fed a bad input returns an error instead of panicking.

use mrly_core::tensor::Tensor;
use mrly_life::models::Config;
use mrly_life::source::{Counts, Source as Seq};
use mrly_math::cell::models::Cell2d;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::str::FromStr;

fn empty_2d() -> Cell2d {
    Cell2d::new(Tensor::new(vec![0, 0])).unwrap()
}

const BAD: &[(&str, fn())] = &[
    ("source::Counts::values, a code past the universe", || {
        let _ = Counts::drawn(Seq::CodeFills(1 << 20), false, false).values(8);
    }),
    ("metrics::entropy, an empty grid", || {
        let _ = mrly_life::metrics::entropy(&empty_2d());
    }),
    (
        "models::Config::counts, a drawn source past the universe",
        || {
            let mask = Cell2d::new(Tensor::full(vec![3, 3], 1)).unwrap();
            let drawn = Counts::drawn(Seq::CodeFills(1 << 20), false, false);
            let _ = Config::new(mask, drawn, vec![2, 3].into()).counts();
        },
    ),
    ("Fate::from_str, an empty text", || {
        let _ = mrly_life::Fate::from_str("");
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
