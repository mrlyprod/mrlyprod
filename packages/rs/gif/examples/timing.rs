// Delays are in hundredths of a second. A frame that repeats the one before is merged into it, so it costs no bytes.
// cargo run -p mrly-gif --example timing
use mrly_gif::{decode, encode};

fn main() -> Result<(), String> {
    let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
    let dark = vec![0u8; 16];
    let light = vec![1u8; 16];
    // Six frames, three of them repeats: dark dark dark light light dark.
    let frames = vec![
        dark.clone(),
        dark.clone(),
        dark.clone(),
        light.clone(),
        light,
        dark,
    ];
    let bytes = encode(&frames, 4, 4, &palette, &[4])?;
    let (_, _, shown, delays) = decode(&bytes)?;
    println!(
        "{} frames of 4/100 s in, {} out, delays {delays:?}",
        frames.len(),
        shown.len()
    );

    // One delay for each frame works too. Browsers show delays of 0 or 1 as 10.
    let bytes = encode(&frames[2..5], 4, 4, &palette, &[2, 30, 50])?;
    println!("own delays: {:?}", decode(&bytes)?.3);

    // A single frame is a still picture: no loop block, no delay, and the older GIF87a header.
    let still = encode(&[frames[0].clone()], 4, 4, &palette, &[100])?;
    println!(
        "still: {} bytes, {}, delays {:?}",
        still.len(),
        String::from_utf8_lossy(&still[..6]),
        decode(&still)?.3
    );
    Ok(())
}
