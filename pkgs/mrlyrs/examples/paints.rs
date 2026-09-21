use mrlyrs::core::paint::{self, Config as PaintConfig, Edition, Paint};
use mrlyrs::core::state::seed;
use mrlyrs::core::{json, Image, Json};
use mrlyrs::gen::build::{build_2d, create_2d, Config2d};
use mrlyrs::gen::recipe::Tile;

const SEED: u64 = 7;

fn tiles() -> Config2d {
    Config2d {
        min_size: 3,
        max_size: 9,
        anti: Some(false),
        ..Config2d::default()
    }
}

fn row(edition: Edition, tile: &Tile, paint: &Paint, image: &Image) -> Json {
    json!({
        "edition": edition.name(),
        "scheme": paint.scheme.name(),
        "target": paint.target.name(),
        "primary": paint.primary.name(),
        "secondary": paint.secondary.iter().map(|ink| ink.name()).collect::<Vec<_>>(),
        "shades": paint.shades.clone(),
        "size": format!("{}x{}", tile.width, tile.height),
        "group": tile.group.name(),
        "image": image,
    })
}

fn main() {
    let config = tiles();
    let mut rows = Vec::new();
    for (i, edition) in Edition::all().into_iter().enumerate() {
        seed(SEED + i as u64);
        let tile = create_2d(&config).expect("no tile fits the size constraints");
        let mut cell = build_2d(&tile).expect("the tile would not build");
        let recipe = PaintConfig {
            editions: Some(vec![edition]),
            ..PaintConfig::default()
        };
        let paint = paint::paint(&mut cell.cell, &recipe, None).expect("the paint would not apply");
        let colors = cell.cell.colors.clone().expect("the paint left no colors");
        let image = Image::from_pixels(cell.width(), cell.height(), &colors);
        rows.push(row(edition, &tile, &paint, &image));
    }
    println!(
        "{}",
        json!({
            "seed": SEED,
            "paints": rows,
        })
    );
}
