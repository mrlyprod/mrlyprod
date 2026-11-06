// Before compression each row can be rewritten as differences from its neighbours: a filter.
// Smooth rows turn into small numbers, which compress well. The encoder tries all five filters on
// every row and keeps the one that gives the smallest numbers. Printed: how many rows chose each filter.
// cargo run -p mrly-png --example filters
use mrly_png::{decompress, encode_level, header};

fn picture(paint: impl Fn(usize, usize) -> [u8; 3]) -> Vec<[u8; 4]> {
    let opaque = |[r, g, b]: [u8; 3]| [r, g, b, 255];
    (0..64 * 64)
        .map(|i| opaque(paint(i % 64, i / 64)))
        .collect()
}

// A stand-in for noise: a byte that jumps around from one (a, b) to the next.
fn jump(a: usize, b: usize) -> usize {
    let mut mixed = (a as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (b as u64) << 32;
    mixed ^= mixed >> 29;
    mixed = mixed.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    (mixed >> 40) as usize % 256
}

// Each row of pixels is stored after one byte that names its filter. Gather the IDAT chunks,
// inflate them, and read that first byte of every row.
fn filters(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let (width, _, depth, kind, _) = header(bytes)?;
    let channels = match kind {
        0 | 3 => 1,
        4 => 2,
        2 => 3,
        _ => 4,
    };
    let row = 1 + (width * channels * depth as usize).div_ceil(8);
    let (mut at, mut packed) = (8, Vec::new());
    while at < bytes.len() {
        let size = u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
        let end = at + 8 + size as usize;
        if &bytes[at + 4..at + 8] == b"IDAT" {
            packed.extend_from_slice(&bytes[at + 8..end]);
        }
        at = end + 4;
    }
    let rows = decompress(&packed, usize::MAX)?;
    Ok(rows.iter().step_by(row).copied().collect())
}

fn main() -> Result<(), String> {
    let byte = |value: usize| value as u8;
    let cases = [
        // Each row is a smooth ramp, but the rows start at unrelated values.
        (
            "smooth along rows",
            picture(|x, y| [x * 3 + jump(y, 1), x + jump(y, 2), x * 2 + jump(y, 3)].map(byte)),
        ),
        // The same turned on its side: columns are smooth, neighbours in a row are unrelated.
        (
            "smooth down columns",
            picture(|x, y| [y * 3 + jump(x, 1), y + jump(x, 2), y * 2 + jump(x, 3)].map(byte)),
        ),
        (
            "smooth both ways",
            picture(|x, y| [x * 2 + y, x + y * 3, 255 - x - y].map(byte)),
        ),
        (
            "noise",
            picture(|x, y| [jump(x, y), jump(y, x + 64), jump(x + 64, y + 64)].map(byte)),
        ),
        (
            "two flat colours",
            picture(|x, y| [(x / 8 + y / 8) % 2 * 255, 0, 0].map(byte)),
        ),
    ];
    // none keeps the row as it is. left subtracts the pixel to the left, up the pixel above,
    // mean the average of both, paeth a guess made from left, above and the corner between them.
    println!("picture               none  left    up  mean paeth  bytes");
    for (name, colors) in cases {
        // Level 6 chooses row by row. Levels 7 and up also try a single filter for the whole picture.
        let bytes = encode_level(&colors, 64, 64, 6)?;
        let rows = filters(&bytes)?;
        let count = |kind: u8| rows.iter().filter(|&&row| row == kind).count();
        let counts = [count(0), count(1), count(2), count(3), count(4)].map(|n| format!("{n:>5}"));
        println!("{name:<20} {} {:>6}", counts.join(" "), bytes.len());
    }
    // The last picture has two colours: it is stored at one bit a pixel, and such rows are never filtered.
    Ok(())
}
