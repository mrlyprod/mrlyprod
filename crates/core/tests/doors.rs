//! Every public function fed a bad input returns an error instead of panicking.

use mrly_core::Image;
use mrly_core::cell::{Cell, magic, merge, mosaic, remap};
use mrly_core::colors::Color;
use mrly_core::ramp::Colorizer;
use mrly_core::resample::{Filter, resample};
use mrly_core::rng::Rng;
use mrly_core::tensor::Tensor;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::str::FromStr;

fn unit_cell() -> Cell {
    Cell::new(Tensor::full(vec![2, 2], 1))
}

fn dot() -> Image {
    Image {
        width: 1,
        height: 1,
        colors: vec![[0, 0, 0, 255]],
    }
}

fn torn() -> Image {
    Image {
        width: 2,
        height: 2,
        colors: vec![[0, 0, 0, 255]],
    }
}

const BAD: &[(&str, fn())] = &[
    ("cell::remap, a map shorter than the cell", || {
        let _ = remap(&unit_cell(), &[0, 1, 2], &[2, 2]);
    }),
    ("cell::merge, reps that do not hold the cells", || {
        let _ = merge(&[unit_cell(), unit_cell()], &[2]);
    }),
    ("cell::mosaic, a mask past the cell list", || {
        let mask = Tensor::of(vec![0, 2, 0, 0], vec![2, 2]).unwrap();
        let _ = mosaic(&mask, std::slice::from_ref(&unit_cell()));
    }),
    ("cell::magic, one layer", || {
        let _ = magic(&[unit_cell()]);
    }),
    ("colors::from_hex, an empty text", || {
        let _ = Color::from_hex("");
    }),
    ("colors::to_hex, a transparent color", || {
        let _ = Color::rgba(0, 0, 0, 0).to_hex();
    }),
    (
        "image::Image::new, a size that is not the pixel count",
        || {
            let _ = Image::new(2, 2, vec![[0, 0, 0, 255]]);
        },
    ),
    ("image::Image::new, a size past usize", || {
        let _ = Image::new(usize::MAX, 2, Vec::new());
    }),
    ("image::Image::scale, no pixels", || {
        let _ = Image::new(0, 0, Vec::new()).map(|image| image.scale(3));
    }),
    ("image::Image::scale, a scale past the width", || {
        let _ = dot().scale(1 << 40);
    }),
    (
        "image::Image::scale, colors that do not fill the size",
        || {
            let _ = torn().scale(2);
        },
    ),
    (
        "image::Image::resample, colors that do not fill the size",
        || {
            let _ = torn().resample(4, 4, Filter::Nearest);
        },
    ),
    ("ramp::color, a zero maximum", || {
        let _ = mrly_core::ramp::color(&Colorizer::heat(), usize::MAX, 0);
    }),
    ("resample::resample, a target past the width", || {
        let _ = resample(&[[0u8, 0, 0, 255]; 4], 2, 2, usize::MAX, 2, Filter::Nearest);
    }),
    ("rng::below, a zero bound", || {
        let _ = Rng::new(2).below(0);
    }),
    ("rng::range, a high below the low", || {
        let _ = Rng::new(2).range(9, 1);
    }),
    ("rng::sample_indices, more than there are", || {
        let _ = Rng::new(2).sample_indices(2, 9);
    }),
    ("paint::Edition::from_str, an empty text", || {
        let _ = mrly_core::paint::Edition::from_str("");
    }),
    ("paint::Ink::from_str, an empty text", || {
        let _ = mrly_core::paint::Ink::from_str("");
    }),
    ("paint::Scheme::from_str, an empty text", || {
        let _ = mrly_core::paint::Scheme::from_str("");
    }),
    ("paint::Target::from_str, an empty text", || {
        let _ = mrly_core::paint::Target::from_str("");
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
