// The encoder looks at the colours of a picture and stores it in the smallest form that loses nothing.
// Few colours go into a table (a palette) at 1, 2, 4 or 8 bits a pixel. Grays drop the colour channels.
// cargo run -p mrly-png --example forms
use mrly_png::{encode, header};

// A 32 x 32 picture whose pixel at (x, y) is paint(x, y).
fn picture(paint: impl Fn(usize, usize) -> [u8; 4]) -> Vec<[u8; 4]> {
    (0..32 * 32).map(|i| paint(i % 32, i / 32)).collect()
}

fn gray(value: usize) -> [u8; 4] {
    [value as u8, value as u8, value as u8, 255]
}

// n different colours laid out in 4 x 4 blocks.
fn blocks(n: usize) -> Vec<[u8; 4]> {
    picture(|x, y| {
        let pick = (x / 4 + y / 4 * 8) % n;
        [(pick * 37) as u8, (pick * 91) as u8, (pick / 2) as u8, 255]
    })
}

fn main() -> Result<(), String> {
    let violet = [103, 104, 250, 255];
    let cases = [
        (
            "black and white",
            picture(|x, y| gray((x / 4 + y / 4) % 2 * 255)),
        ),
        ("4 grays", picture(|x, _| gray(x / 8 * 85))),
        ("5 grays", picture(|x, _| gray(x / 7 * 60))),
        ("256 grays", picture(|x, y| gray(x * 8 + y / 4))),
        ("2 colours", blocks(2)),
        ("3 colours", blocks(3)),
        ("16 colours", blocks(16)),
        ("64 colours", blocks(64)),
        (
            "violet on clear",
            picture(|x, y| if (x + y) % 8 < 4 { [0; 4] } else { violet }),
        ),
        (
            "gray, soft alpha",
            picture(|x, y| [200, 200, 200, (x * 8 + y) as u8]),
        ),
        (
            "1024 colours",
            picture(|x, y| [(x * 8) as u8, (y * 8) as u8, 99, 255]),
        ),
        (
            "1024 colours, soft alpha",
            picture(|x, y| [(x * 8) as u8, (y * 8) as u8, 99, (x * y / 4) as u8]),
        ),
    ];
    println!(
        "{:<25} {:<11} {:>12} {:>6}",
        "picture", "stored as", "bits a pixel", "bytes"
    );
    for (name, colors) in cases {
        let bytes = encode(&colors, 32, 32)?;
        let (_, _, depth, kind, _) = header(&bytes)?;
        // The file names its form with a number: 0, 2, 3, 4 or 6.
        let (form, channels) = match kind {
            0 => ("gray", 1),
            2 => ("rgb", 3),
            3 => ("palette", 1),
            4 => ("gray+alpha", 2),
            _ => ("rgba", 4),
        };
        println!(
            "{name:<25} {form:<11} {:>12} {:>6}",
            depth * channels,
            bytes.len()
        );
    }
    Ok(())
}
