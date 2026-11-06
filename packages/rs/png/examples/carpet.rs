// Flat pictures with few colours are where PNG shines, and most of ours look like that.
// Printed: a Sierpinski carpet at growing sizes, the bytes it takes, and bits spent per pixel.
// cargo run -p mrly-png --example carpet
use mrly_png::{decode, encode};

// A pixel is a hole when x and y, written in base 3, have a 1 at the same place.
fn hole(mut x: usize, mut y: usize) -> bool {
    while x > 0 || y > 0 {
        if x % 3 == 1 && y % 3 == 1 {
            return true;
        }
        (x, y) = (x / 3, y / 3);
    }
    false
}

fn main() -> Result<(), String> {
    println!("side   pixels  raw rgba   png  bits a pixel");
    for level in 1..=6 {
        let side = 3usize.pow(level);
        let paint = |i: usize| {
            if hole(i % side, i / side) {
                [0; 4]
            } else {
                [255, 61, 64, 255]
            }
        };
        let colors: Vec<[u8; 4]> = (0..side * side).map(paint).collect();
        let bytes = encode(&colors, side, side)?;
        assert_eq!(decode(&bytes)?.2, colors);
        let bits = bytes.len() as f64 * 8.0 / colors.len() as f64;
        println!(
            "{side:>4} {:>8} {:>9} {:>5}  {bits:.4}",
            colors.len(),
            colors.len() * 4,
            bytes.len()
        );
    }
    // Each step has 9 times the pixels but only a few more bytes: the picture repeats itself,
    // and the compressor points back at what it has already written.
    Ok(())
}
