// A tile recipe belongs to one of five groups. Each builds a square pattern of 1s (filled) and 0s (void).
// General: one design. Fractal: a design inside itself. Magic: several designs multiplied together.
// Special: a design combined with a tree. Mosaic: three designs laid out by a tree mask.
// cargo run -p mrly-tile --example groups
use mrly_tile::build::build_2d;
use mrly_tile::recipe::{Design, Group, Source, Tile};
use mrly_two::text;
use std::collections::HashMap;

fn tile(group: Group, designs: &[Design], number: usize, level: usize, factor: usize) -> Tile {
    let mut tile = Tile::new(group);
    tile.sources = designs.iter().map(|&d| Source::Classic(d)).collect();
    tile.numbers = vec![number; designs.len()];
    tile.levels = vec![level; designs.len()];
    tile.rotations = vec![0; designs.len()];
    tile.factor = factor;
    tile.resize();
    tile
}

fn main() {
    use Design::{Carpet, Net, Star, Void};
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    let tiles = [
        tile(Group::General, &[Carpet], 5, 1, 0),
        tile(Group::Fractal, &[Net], 3, 2, 0),
        tile(Group::Magic, &[Carpet, Star], 3, 1, 0),
        tile(Group::Special, &[Carpet], 3, 1, 3),
        tile(Group::Mosaic, &[Carpet, Star, Void], 3, 1, 3),
    ];
    for tile in &tiles {
        tile.check().unwrap();
        println!("{} {}x{}", tile.group.name(), tile.width, tile.height);
        for row in text(&build_2d(tile).unwrap(), Some(&glyphs)) {
            println!("  {row}");
        }
        println!();
    }
}
