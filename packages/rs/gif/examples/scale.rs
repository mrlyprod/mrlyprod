// Pixel art is drawn small and shown big. scale() turns each pixel into a square block, with no blur.
// A 5 by 5 heart that beats, exported at 1, 2, 4 and 8 times its size.
// cargo run -p mrly-gif --example scale
use mrly_gif::{encode, scale};

fn grid(rows: [&str; 5]) -> Vec<u8> {
    rows.iter()
        .flat_map(|row| row.bytes().map(|b| (b == b'#') as u8))
        .collect()
}

fn main() -> Result<(), String> {
    let big = grid([".#.#.", "#####", "#####", ".###.", "..#.."]);
    let small = grid([".....", ".#.#.", ".###.", "..#..", "....."]);
    let palette = [[255, 255, 255, 255], [220, 30, 60, 255]];

    // Twice the size: every pixel becomes a 2 by 2 block.
    for row in scale(&big, 5, 2)?.chunks(10) {
        let line: String = row
            .iter()
            .map(|&i| if i == 1 { '#' } else { '.' })
            .collect();
        println!("{line}");
    }
    println!();

    // The palette and the delays stay the same at every size; only the grids grow.
    for factor in [1, 2, 4, 8] {
        let frames = [scale(&big, 5, factor)?, scale(&small, 5, factor)?];
        let side = 5 * factor;
        let bytes = encode(&frames, side, side, &palette, &[40, 20])?;
        println!("{factor}x: {side} by {side}, {} bytes", bytes.len());
    }
    println!("0x: {}", scale(&big, 5, 0).unwrap_err());
    Ok(())
}
