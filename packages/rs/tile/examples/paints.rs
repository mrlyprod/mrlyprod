// One tile painted seven ways, one per edition: the same 0s and 1s, different rules for choosing colours.
// Simple colours by type, Index and Rows and Columns by position, Layers and Neighbors by tag, Random by chance.
// cargo run -p mrly-tile --example paints
use mrly_rand::Rng;
use mrly_tile::build::build_2d;
use mrly_tile::paint::{self, Config, Edition};
use mrly_tile::recipe::{Design, Group, Source, Tile};
use std::collections::BTreeMap;

const SEED: u64 = 7;

fn main() {
    let mut recipe = Tile::new(Group::Fractal);
    recipe.sources = vec![Source::Classic(Design::Carpet)];
    recipe.numbers = vec![3];
    recipe.levels = vec![2];
    recipe.rotations = vec![0];
    recipe.resize();
    let tile = build_2d(&recipe).unwrap();
    let (side, gap) = (recipe.width, 2);
    let editions = Edition::all();
    let wide = editions.len() * (side + gap) - gap;
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {wide} {side}\">");
    for (i, edition) in editions.into_iter().enumerate() {
        let mut painted = tile.clone();
        let mut rng = Rng::new(SEED + i as u64);
        let config = Config {
            editions: Some(vec![edition]),
            ..Config::default()
        };
        paint::paint(&mut painted.cell, &config, None, &mut rng).unwrap();
        let colors = painted.cell.colors.as_ref().unwrap();
        let left = i * (side + gap);
        let mut bars: BTreeMap<[u8; 3], String> = BTreeMap::new();
        for y in 0..side {
            let mut x = 0;
            while x < side {
                let c = colors[y * side + x];
                let start = x;
                while x < side && colors[y * side + x] == c {
                    x += 1;
                }
                let w = x - start;
                bars.entry([c[0], c[1], c[2]])
                    .or_default()
                    .push_str(&format!("M{} {y}h{w}v1h-{w}z", left + start));
            }
        }
        for (c, d) in bars {
            println!(
                "<path fill=\"#{:02x}{:02x}{:02x}\" d=\"{d}\"/>",
                c[0], c[1], c[2]
            );
        }
    }
    println!("</svg>");
}
