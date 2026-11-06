// Pixel art often comes as a short palette and a grid of indices into it. encode_indexed writes
// exactly that: the palette in the order given, at the fewest bits a pixel that can hold it.
// Printed: a small sprite, how it is stored, and what goes wrong with a bad index.
// cargo run -p mrly-png --example indexed
use mrly_png::{decode, encode, encode_indexed, header};

const SPRITE: [&str; 8] = [
    "..#.....#..",
    "...#...#...",
    "..#######..",
    ".##o###o##.",
    "###########",
    "#.#######.#",
    "#.#.....#.#",
    "...##.##...",
];

fn main() -> Result<(), String> {
    // Three colours: index 0 is the body, 1 the eyes, 2 a clear background.
    let palette = [[60, 200, 90, 255], [255, 255, 255, 255], [0, 0, 0, 0]];
    let index = |c: char| "#o.".find(c).unwrap() as u8;
    let indices: Vec<u8> = SPRITE
        .iter()
        .flat_map(|row| row.chars())
        .map(index)
        .collect();
    let (width, height) = (SPRITE[0].len(), SPRITE.len());

    let bytes = encode_indexed(&indices, width, height, &palette)?;
    let (_, _, depth, kind, _) = header(&bytes)?;
    for row in SPRITE {
        println!("{row}");
    }
    println!();
    println!(
        "colour type {kind} (palette), {depth} bits a pixel, {} bytes",
        bytes.len()
    );

    // Decoding gives colours back, and each one is the palette entry its index named.
    let (_, _, colors) = decode(&bytes)?;
    let kept = colors
        .iter()
        .zip(&indices)
        .all(|(color, &i)| *color == palette[i as usize]);
    println!("every pixel is palette[index]  {kept}");

    // encode() finds the same three colours on its own and puts the clear one first. Alpha is
    // stored for every entry up to the last see-through one, so it needs 1 byte of it, not 3.
    println!(
        "encode() on those colours      {} bytes",
        encode(&colors, width, height)?.len()
    );

    println!(
        "index 3 with 3 colours         {}",
        encode_indexed(&[3], 1, 1, &palette).unwrap_err()
    );
    Ok(())
}
