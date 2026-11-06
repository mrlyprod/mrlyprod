// A background is a repeating painted tile. background(seed, w, h) draws a tile and a paint from the seed and repeats the tile w by h times.
// Seed 3, two tiles across and one down. Pixels of one colour are gathered into one path.
// cargo run -p mrly-tile --example background
use mrly_tile::background;
use std::collections::BTreeMap;

fn main() {
    let image = background(3, 2, 1).unwrap();
    let (width, height) = (image.width, image.height);
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\">");
    let mut bars: BTreeMap<[u8; 3], String> = BTreeMap::new();
    for y in 0..height {
        let mut x = 0;
        while x < width {
            let c = image.colors[y * width + x];
            let start = x;
            while x < width && image.colors[y * width + x] == c {
                x += 1;
            }
            let w = x - start;
            bars.entry([c[0], c[1], c[2]])
                .or_default()
                .push_str(&format!("M{start} {y}h{w}v1h-{w}z"));
        }
    }
    for (c, d) in bars {
        println!(
            "<path fill=\"#{:02x}{:02x}{:02x}\" d=\"{d}\"/>",
            c[0], c[1], c[2]
        );
    }
    println!("</svg>");
}
