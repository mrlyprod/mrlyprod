// A seeded random tile: create_2d draws a group, designs and sizes from the config, then build_2d makes the pattern.
// The same seed always gives the same tile. Seed 3, sides 3 to 9, odd sides only.
// cargo run -p mrly-tile --example random
use mrly_name::Named;
use mrly_rand::Rng;
use mrly_tile::build::{Config2d, build_2d, create_2d};
use mrly_tile::name::Tile as Name;
use mrly_two::text;
use std::collections::HashMap;

fn main() {
    let mut rng = Rng::new(3);
    let recipe = create_2d(&Config2d::default(), &mut rng).unwrap();
    println!("{}", Name::of(&recipe).unwrap().to_json());
    println!(
        "{} {}x{}, rotations {:?}",
        recipe.group.name(),
        recipe.width,
        recipe.height,
        recipe.rotations
    );
    println!();
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    for row in text(&build_2d(&recipe).unwrap(), Some(&glyphs)) {
        println!("{row}");
    }
}
