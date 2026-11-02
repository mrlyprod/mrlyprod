//! The stored rows every binding replays, against what the crate returns today.

use mrly_core::Image;
use mrly_core::name::{Named, sha};
use mrly_core::rng::Rng;
use mrly_math::two::{self, Cell2d};
use mrly_tile as tile;
use serde_json::{Value, json};
use std::path::Path;

const SEED: u64 = 2026;

const CARPET_NAME: &str = r#"{"kind":"tile","code":7,"side":3,"level":2}"#;

fn row(path: &str, input: Value, out: Value, note: &str) -> Value {
    json!({"fn": path, "in": input, "out": out, "note": note})
}

fn cell(flat: &Cell2d) -> Value {
    serde_json::from_str(&two::to_json(flat)).unwrap()
}

fn pixels(image: &Image) -> Value {
    json!({
        "shape": [image.height, image.width],
        "sha256": sha::hex(&image.colors.concat()),
    })
}

// ROWS

fn rows() -> Vec<Value> {
    let name = tile::name::Tile::from_json(CARPET_NAME).unwrap();
    let recipe = name.recipe().unwrap();
    let folded = tile::name::Tile::of(&recipe).unwrap();
    let built = tile::build::build_2d(&recipe).unwrap();
    let mut drawing = Rng::new(SEED);
    let drawn = tile::draw::create(
        &tile::draw::ConfigNd::<2>::default(),
        |_, rng| rng.below(4),
        &mut drawing,
    )
    .unwrap();
    let mut varying = Rng::new(SEED);
    let made = tile::variation::create(&tile::variation::Config::default(), &mut varying).unwrap();
    vec![
        row(
            "tile::name::Tile::recipe",
            json!({"name": CARPET_NAME}),
            json!({
                "recipe": serde_json::to_value(&recipe).unwrap(),
                "json": folded.to_json(),
                "id": folded.to_id(),
            }),
            "the name read, unfolded to a recipe, folded back and hashed",
        ),
        row(
            "tile::draw::create",
            json!({"config": "ConfigNd::<2>::default()", "rotation": "rng.below(4)", "seed": SEED}),
            json!(tile::name::Tile::of(&drawn).unwrap().to_json()),
            "the drawn recipe as the canonical json of its name",
        ),
        row(
            "tile::build::build_2d",
            json!({"tile": {"fn": "tile::name::Tile::recipe", "in": {"name": CARPET_NAME}}}),
            cell(&built),
            "the classic carpet recipe built into its 9 by 9 cell",
        ),
        row(
            "tile::variation::create",
            json!({"config": "variation::Config::default()", "seed": SEED}),
            json!({
                "key": made.key,
                "seed": made.seed.to_string(),
                "edition": made.edition.name(),
                "tile": tile::name::Tile::of(&made.tile).unwrap().to_json(),
            }),
            "seed is a u64 drawn from the stream, so it crosses as a decimal string",
        ),
        row(
            "tile::background",
            json!({"seed": 1, "width": 2, "height": 2}),
            pixels(&tile::background(1, 2, 2).unwrap()),
            "the shape and the sha256 of the rgba bytes of a whole seeded artwork",
        ),
    ]
}

// REPLAY

#[test]
fn every_row_replays() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures.json");
    let fresh = rows();
    if std::env::var_os("MRLY_FIXTURES").is_some() {
        let mut text = serde_json::to_string_pretty(&fresh).unwrap();
        text.push('\n');
        std::fs::write(&file, text).unwrap();
    }
    let saved: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(saved.len(), fresh.len(), "{} rows stored", saved.len());
    for (was, now) in saved.iter().zip(&fresh) {
        assert_eq!(was, now, "{} drifted; MRLY_FIXTURES=1 rewrites", was["fn"]);
    }
}
