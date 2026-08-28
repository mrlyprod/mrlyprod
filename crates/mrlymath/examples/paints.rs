use mrlycore::paint::{self, Config as PaintConfig, Edition, Paint};
use mrlycore::state::seed;
use mrlycore::tile::Tile;
use mrlycore::{json, Image, Json};
use mrlymath::two::tile as tile2d;

const SEED: u64 = 7;

fn tiles() -> tile2d::Config {
    tile2d::Config {
        min_size: 3,
        max_size: 9,
        anti: Some(false),
        ..tile2d::Config::default()
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
        let tile = tile2d::create(&config).expect("no tile fits the size constraints");
        let mut cell = tile2d::build(&tile).expect("the tile would not build");
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
