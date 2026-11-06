// A tile has a short name, as JSON, that holds the whole recipe: the design codes, the side and the group.
// The name turns back into the recipe. Here one tile of each group: its name, and the group and size it comes back as.
// cargo run -p mrly-tile --example name
use mrly_name::Named;
use mrly_tile::name::Tile as Name;
use mrly_tile::recipe::{Design, Group, Source, Tile};

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
    let tiles = [
        tile(Group::General, &[Carpet], 5, 1, 0),
        tile(Group::Fractal, &[Net], 3, 2, 0),
        tile(Group::Magic, &[Carpet, Star], 3, 1, 0),
        tile(Group::Special, &[Carpet], 3, 1, 3),
        tile(Group::Mosaic, &[Carpet, Star, Void], 3, 1, 3),
    ];
    for tile in &tiles {
        let name = Name::of(tile).unwrap();
        let back = name.recipe().unwrap();
        println!("{}", name.to_json());
        println!(
            "  back as {} {}x{}",
            back.group.name(),
            back.width,
            back.height
        );
    }
}
