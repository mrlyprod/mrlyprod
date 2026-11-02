//! Every public function fed a bad input returns an error instead of panicking.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::str::FromStr;

const BAD: &[(&str, fn())] = &[
    ("factor::factorial, a four byte maximum", || {
        let _ = mrly_num::factor::factorial(u32::MAX as usize);
    }),
    ("factor::gcd, the widest pair", || {
        let _ = mrly_num::factor::gcd(u128::MAX, u128::MAX);
    }),
    ("prime::is_prime, the widest number", || {
        let _ = mrly_num::prime::is_prime(usize::MAX);
    }),
    ("chladni::plate, a sign of two", || {
        let _ = mrly_num::chladni::plate(1, 2, 2, 8);
    }),
    ("chladni::plate, a size of zero", || {
        let _ = mrly_num::chladni::plate(1, 2, 1, 0);
    }),
    ("chladni::plate, a size past the cap", || {
        let _ = mrly_num::chladni::plate(1, 2, 1, usize::MAX);
    }),
    ("chladni::plate, a wave number past the cap", || {
        let _ = mrly_num::chladni::plate(usize::MAX, 2, 1, 8);
    }),
    ("chladni::ladder, a cap past the cap", || {
        let _ = mrly_num::chladni::ladder(usize::MAX);
    }),
    ("chladni::shake, a side whose square overflows", || {
        let _ = mrly_num::chladni::shake(&[0.0; 4], usize::MAX, &[0.5, 0.5], 0, 1, 1);
    }),
    (
        "chladni::shake, a field that is not the side squared",
        || {
            let _ = mrly_num::chladni::shake(&[0.0; 5], 2, &[0.5, 0.5], 0, 1, 1);
        },
    ),
    ("chladni::shake, a field holding an infinity", || {
        let _ = mrly_num::chladni::shake(&[f64::INFINITY; 4], 2, &[0.5, 0.5], 0, 1, 1);
    }),
    ("chladni::shake, an odd count of grains", || {
        let _ = mrly_num::chladni::shake(&[0.0; 4], 2, &[0.5], 0, 1, 1);
    }),
    ("chladni::shake, steps past the cap", || {
        let _ = mrly_num::chladni::shake(&[0.0; 4], 2, &[0.5, 0.5], u64::MAX, usize::MAX, 1);
    }),
    ("chladni::shake, grains far off the plate", || {
        let _ = mrly_num::chladni::shake(
            &[1.0, -1.0, f64::NAN, 0.5],
            2,
            &[1e30, -7.0, f32::INFINITY, 0.5],
            u64::MAX,
            2,
            1,
        );
    }),
    ("chladni::lines, a side of zero", || {
        let _ = mrly_num::chladni::lines(&[], 0);
    }),
    ("chladni::lines, a side whose square overflows", || {
        let _ = mrly_num::chladni::lines(&[0.0; 4], usize::MAX);
    }),
    ("chladni::lines, a field of holes and zeros", || {
        let _ = mrly_num::chladni::lines(&[f64::NAN, 0.0, -0.0, f64::NAN], 2);
    }),
    ("spiral::Lattice::from_str, an empty text", || {
        let _ = mrly_num::spiral::Lattice::from_str("");
    }),
    ("spiral::Mark::from_str, an empty text", || {
        let _ = mrly_num::spiral::Mark::from_str("");
    }),
    ("spiral::Growth::from_str, an empty text", || {
        let _ = mrly_num::spiral::Growth::from_str("");
    }),
    ("morse::Lift::from_str, an empty text", || {
        let _ = mrly_num::morse::Lift::from_str("");
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
