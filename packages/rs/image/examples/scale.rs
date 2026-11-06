// scale turns every pixel into a square block, keeping edges sharp.
// The small heart is the source; the large one is scale(4).
// cargo run -p mrly-image --example scale
use mrly_image::{Image, svg};

const HEART: [&str; 7] = [
    ".##.##.", "#######", "#######", "#######", ".#####.", "..###..", "...#...",
];

fn main() {
    let colors: Vec<[u8; 4]> = HEART
        .iter()
        .flat_map(|row| row.chars())
        .map(|c| {
            if c == '#' {
                [255, 61, 64, 255]
            } else {
                [0, 0, 0, 0]
            }
        })
        .collect();
    let small = Image::new(7, 7, colors).unwrap();
    let big = small.scale(4).unwrap();

    println!("{}", svg(&[small, big], 3, 8));
}
