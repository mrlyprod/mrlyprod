// Broken files are refused with a reason, never a crash. Each line damages one good file and prints what decode() says.
// A header can claim a canvas of 65535 by 65535; decode() refuses before asking for gigabytes.
// GIF has no checksum, so a flipped bit can also decode cleanly to the wrong pixels.
// cargo run -p mrly-gif --example corrupt
use mrly_gif::{LIMIT, decode, encode};

fn main() -> Result<(), String> {
    let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
    let frames: Vec<Vec<u8>> = (0..3)
        .map(|f| (0..64).map(|i| ((i + f) % 3 == 0) as u8).collect())
        .collect();
    let good = encode(&frames, 8, 8, &palette, &[5])?;
    println!("a good file: {} bytes, 3 frames", good.len());
    println!();

    let mut huge = good.clone();
    huge[6..10].copy_from_slice(&[0xFF; 4]);
    let flip = |at: usize, bit: u8| {
        let mut bytes = good.clone();
        bytes[at] ^= 1 << bit;
        bytes
    };
    let mut stray = good.clone();
    let last = stray.len() - 1;
    stray[last] = 0x99;
    let cases: Vec<(&str, Vec<u8>)> = vec![
        ("empty", Vec::new()),
        ("not a gif", b"\x89PNG\r\n\x1a\n".to_vec()),
        ("cut in the header", good[..9].to_vec()),
        ("cut in the colors", good[..16].to_vec()),
        ("cut in a frame", good[..good.len() - 12].to_vec()),
        ("no trailer", good[..good.len() - 1].to_vec()),
        ("huge canvas", huge),
        ("unknown block", stray),
        ("bad code size", flip(56, 3)),
        ("bad lzw code", flip(60, 1)),
        ("wrong pixels", flip(60, 0)),
    ];
    let (_, _, right, _) = decode(&good)?;
    for (name, bytes) in cases {
        match decode(&bytes) {
            Ok((w, h, frames, _)) => {
                let wrong = frames
                    .iter()
                    .flatten()
                    .zip(right.iter().flatten())
                    .filter(|(a, b)| a != b)
                    .count();
                println!(
                    "{name:<18} ok: {} frames of {w} by {h}, {wrong} pixels differ",
                    frames.len()
                );
            }
            Err(e) => println!("{name:<18} {e}"),
        }
    }
    println!();
    println!(
        "LIMIT is {LIMIT} pixels: the most decode() holds, counting the canvas and every frame."
    );
    Ok(())
}
