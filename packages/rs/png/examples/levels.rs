// The level trades time for size: 1 is the fastest, 9 searches hardest. 0 does not compress at all.
// Printed: the file size at every level for a flat fractal and for a noisy fade.
// cargo run -p mrly-png --example levels
use mrly_png::{BEST, DEFAULT, encode_level};

// A Sierpinski carpet: a square with its middle ninth cut out, again and again.
fn carpet(side: usize) -> Vec<[u8; 4]> {
    let hole = |mut x: usize, mut y: usize| {
        while x > 0 || y > 0 {
            if x % 3 == 1 && y % 3 == 1 {
                return true;
            }
            (x, y) = (x / 3, y / 3);
        }
        false
    };
    let paint = |i: usize| {
        if hole(i % side, i / side) {
            [255; 4]
        } else {
            [103, 104, 250, 255]
        }
    };
    (0..side * side).map(paint).collect()
}

// A fade with a little grain on top, closer to a photograph.
fn fade(side: usize) -> Vec<[u8; 4]> {
    let grain = |i: usize| ((i as u64).wrapping_mul(2654435761) >> 13) as usize % 5;
    let paint = |i: usize| {
        let (x, y) = (i % side, i / side);
        [
            (x * 2 + grain(i)) as u8,
            (y * 2 + grain(i + 7)) as u8,
            (x + y) as u8,
            255,
        ]
    };
    (0..side * side).map(paint).collect()
}

fn main() -> Result<(), String> {
    let (flat, noisy) = (carpet(243), fade(96));
    println!("raw rgba  {:>6} {:>6}", flat.len() * 4, noisy.len() * 4);
    println!();
    // encode() uses DEFAULT. BEST is a much slower search for the last few percent.
    println!("level     carpet   fade");
    for level in 0..=BEST {
        let a = encode_level(&flat, 243, 243, level)?.len();
        let b = encode_level(&noisy, 96, 96, level)?.len();
        let note = match level {
            DEFAULT => "  DEFAULT, what encode() uses",
            BEST => "  BEST",
            _ => "",
        };
        println!("{level:<8}  {a:>6} {b:>6}{note}");
    }
    // The carpet has two colours, so even level 0 packs eight pixels into a byte.
    Ok(())
}
