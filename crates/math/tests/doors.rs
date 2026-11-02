//! Every public function fed a bad input returns an error instead of panicking.

use mrly_core::name::Named;
use mrly_core::tensor::Tensor;
use mrly_math::bang::Code;
use mrly_math::bang::catalog::Design;
use mrly_math::bang::factory::MagicLayer;
use mrly_math::cell::models::{Cell2d, Cell3d};
use mrly_math::graph::models::{Branch, Network};
use mrly_math::name::{Bang, Word};
use mrly_math::shape::{Frac, Shape};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::str::FromStr;

fn empty_2d() -> Cell2d {
    Cell2d::new(Tensor::new(vec![0, 0])).unwrap()
}

fn empty_3d() -> Cell3d {
    Cell3d::new(Tensor::new(vec![0, 0, 0])).unwrap()
}

fn stray_network() -> Network {
    let mut net = Network::new(1);
    net.add_node(vec![0.0]).unwrap();
    net.branches.push(Branch {
        parent: 0,
        child: 9,
        radius: 1.0,
    });
    net
}

fn lonely_network() -> Network {
    let mut net = Network::new(1);
    net.add_node(vec![0.0]).unwrap();
    net
}

fn wild_shape() -> Shape {
    Shape::Ball {
        center: vec![Frac {
            num: 1,
            den: i64::MAX,
        }],
        radius: Frac::new(1, 2).unwrap(),
    }
}

const BAD: &[(&str, fn())] = &[
    ("atoms::carpet_2d, a zero side", || {
        let _ = mrly_math::atoms::carpet_2d(0);
    }),
    ("nodal::figures, a code past its range", || {
        let _ = mrly_math::nodal::figures(u64::MAX, 3, 3, 8);
    }),
    ("nodal::figures, a level past every cap", || {
        let _ = mrly_math::nodal::figures(7, 3, usize::MAX, 8);
    }),
    ("nodal::figures, a number of zero", || {
        let _ = mrly_math::nodal::figures(7, 0, 3, 8);
    }),
    ("nodal::figures, a count past the cells", || {
        let _ = mrly_math::nodal::figures(7, 3, 2, usize::MAX);
    }),
    ("nodal::figures, a count of zero", || {
        let _ = mrly_math::nodal::figures(7, 3, 2, 0);
    }),
    ("nodal::holds, a code past its range", || {
        let _ = mrly_math::nodal::holds(u64::MAX, 3, 3);
    }),
    (
        "nodal::holds, a number of one at a level past every cap",
        || {
            let _ = mrly_math::nodal::holds(7, 1, usize::MAX);
        },
    ),
    ("nodal::holds, a number past the side cap", || {
        let _ = mrly_math::nodal::holds(7, usize::MAX, 1);
    }),
    (
        "spectrum::laplacian_spectrum, a lone node normalised",
        || {
            let _ = mrly_math::spectrum::laplacian_spectrum(&lonely_network(), true);
        },
    ),
    ("graph::census::census, a branch past the nodes", || {
        let _ = mrly_math::graph::census::census(&stray_network());
    }),
    ("two::carpet, a zero side", || {
        let _ = mrly_math::two::carpet(0, 1);
    }),
    ("two::carpet, a zero level", || {
        let _ = mrly_math::two::carpet(3, 0);
    }),
    ("two::to_json, an empty cell", || {
        let _ = mrly_math::two::to_json(&empty_2d());
    }),
    ("two::census, an empty cell", || {
        let _ = mrly_math::two::census::census(&empty_2d());
    }),
    ("three::census::surface, an empty cell", || {
        let _ = mrly_math::three::census::surface(&empty_3d());
    }),
    (
        "bang::baseq::group_order, a dimension past the factorial",
        || {
            let _ = mrly_math::bang::baseq::group_order(2, 35);
        },
    ),
    (
        "bang::baseq::predicted_group_order, a dimension past the factorial",
        || {
            let _ = mrly_math::bang::baseq::predicted_group_order(2, 35);
        },
    ),
    ("bang::baseq::group_order, a dimension past a u128", || {
        let _ = mrly_math::bang::baseq::group_order(2, 200);
    }),
    (
        "bang::baseq::predicted_group_order, a dimension past a u128",
        || {
            let _ = mrly_math::bang::baseq::predicted_group_order(3, 200);
        },
    ),
    ("press::Press::new, a universe past its corners", || {
        let _ = mrly_math::press::Press::new(7, 2);
    }),
    ("name::Bang::from_json, an empty text", || {
        let _ = mrly_math::name::Bang::from_json("");
    }),
    ("name::Bang::from_json, a code past its range", || {
        let _ = mrly_math::name::Bang::from_json(r#"{"kind":"bang","dim":2,"code":16}"#);
    }),
    ("name::Sequence::from_json, an empty text", || {
        let _ = mrly_math::name::Sequence::from_json("");
    }),
    ("name::Word::from_json, an empty text", || {
        let _ = Word::from_json("");
    }),
    ("name::Bang::from_json, a text without a kind", || {
        let _ = mrly_math::name::Bang::from_json("{}");
    }),
    ("name::Bang::from_json, a broken json", || {
        let _ = mrly_math::name::Bang::from_json(r#"{"kind":"bang""#);
    }),
    ("spectrum::laplacian, a lone node normalised", || {
        let _ = mrly_math::spectrum::laplacian(&lonely_network(), true);
    }),
    ("graph::census::tips, a branch past the nodes", || {
        let _ = mrly_math::graph::census::tips(&stray_network());
    }),
    ("graph::census::components, a branch past the nodes", || {
        let _ = mrly_math::graph::census::components(&stray_network());
    }),
    ("bang::word::components, layers of two dimensions", || {
        let plane = MagicLayer::new(Bang::new(7, 2, 2), 3);
        let cube = MagicLayer::new(Bang::new(23, 3, 2), 3);
        let _ = mrly_math::bang::word::components(&[plane, cube]);
    }),
    ("bang::word::fill, no layers", || {
        let _ = mrly_math::bang::word::fill(&[]);
    }),
    ("shape::fit, no extents", || {
        let _ = mrly_math::shape::fit(&wild_shape(), &[]);
    }),
    ("shape::fit, a shape of another dimension", || {
        let _ = mrly_math::shape::fit(&wild_shape(), &[2, 3]);
    }),
    ("shape::fit, an extent past an i64", || {
        let ball = mrly_math::shape::named("ball", 2, Frac::new(1, 2).unwrap()).unwrap();
        let _ = mrly_math::shape::fit(&ball, &[usize::MAX, 1]);
    }),
    ("shape::reach, the widest den", || {
        let _ = mrly_math::shape::reach("ball", 2, u32::MAX);
    }),
    ("six::geometry::disc, a cell that is not a hexagon", || {
        let cell = mrly_math::six::cut_design(Code::from(23u64), 3, 1, 2).unwrap();
        let flat = mrly_math::six::Cell6d::new(empty_2d(), cell.projection, cell.orientation, 0);
        let _ = mrly_math::six::disc(&flat, 1);
    }),
    ("arcs::curves, a code past the base", || {
        let _ = mrly_math::arcs::curves(Code::from(16u64), 2, 1);
    }),
    ("arcs::curves, a level past the machine word", || {
        let _ = mrly_math::arcs::curves(Code::from(9u64), 2, 200);
    }),
    ("bang::Code::from_str, an empty text", || {
        let _ = Code::from_str("");
    }),
    ("bang::catalog::Design::from_str, an empty text", || {
        let _ = Design::from_str("");
    }),
    ("bang::word::Schedule::from_str, an empty text", || {
        let _ = mrly_math::bang::word::Schedule::from_str("");
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
