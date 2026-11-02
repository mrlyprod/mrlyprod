//! Every public function fed a bad input returns an error instead of panicking.

use mrly_core::name::Named;
use mrly_core::rng::Rng;
use mrly_math::six::Projection;
use mrly_tile::name::Tile as Name;
use mrly_tile::recipe::{Design, Group, Source, Tile as Recipe};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::str::FromStr;

fn turn(_: Source, rng: &mut Rng) -> usize {
    rng.below(4)
}

const BAD: &[(&str, fn())] = &[
    ("recipe::Tile::new, a recipe with no slots", || {
        let _ = Recipe::new(Group::General).check();
    }),
    ("recipe::Tile::size, sizes that disagree", || {
        let _ = Recipe::new(Group::General).size(5, 5).check();
    }),
    ("draw::random_tile, a size below the floor", || {
        let _ = mrly_tile::draw::random_tile::<2>(1, turn, &mut Rng::new(1));
    }),
    ("build::build_2d, a recipe with no slots", || {
        let _ = mrly_tile::build::build_2d(&Recipe::new(Group::General));
    }),
    ("build::build_2d, a cubic design in the plane", || {
        let mut cubic = Recipe::new(Group::General);
        cubic.sources = vec![Source::Classic(Design::Xtree)];
        cubic.numbers = vec![3];
        cubic.levels = vec![1];
        cubic.rotations = vec![0];
        cubic.resize();
        let _ = mrly_tile::build::build_2d(&cubic);
    }),
    ("build::build_3d, a recipe with no slots", || {
        let _ = mrly_tile::build::build_3d(&Recipe::new(Group::General));
    }),
    ("build::build_6d, a recipe with no slots", || {
        let bare = mrly_tile::build::HexTile {
            projection: Projection::Iso,
            tile: Recipe::new(Group::General),
        };
        let _ = mrly_tile::build::build_6d(&bare);
    }),
    ("name::Tile::from_json, an empty text", || {
        let _ = Name::from_json("");
    }),
    ("variation::create, a config that draws no tile", || {
        let _ = mrly_tile::variation::create(
            &mrly_tile::variation::Config::default(),
            &mut Rng::new(5),
        );
    }),
    ("build::build_3d, a flat design in the cube", || {
        let mut flat = Recipe::new(Group::General);
        flat.sources = vec![Source::Classic(Design::Htree)];
        flat.numbers = vec![3];
        flat.levels = vec![1];
        flat.rotations = vec![0];
        flat.resize();
        let _ = mrly_tile::build::build_3d(&flat);
    }),
    ("recipe::Group::from_str, an empty text", || {
        let _ = Group::from_str("");
    }),
    ("recipe::Parity::from_str, an empty text", || {
        let _ = mrly_tile::recipe::Parity::from_str("");
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
