// Encode, then decode, then compare. Indexed frames come back exactly, and so do RGBA frames with at most 256 colors.
// A palette color is red, green, blue and alpha. A GIF pixel is either solid or clear, and one palette color may be clear.
// decode() returns the size, every frame as it looks on screen, and each frame's delay in hundredths of a second.
// cargo run -p mrly-gif --example round_trip
use mrly_gif::{decode, encode, encode_rgba};

fn main() -> Result<(), String> {
    // Three colors, and 4 frames of a 12 by 12 board where a 3 by 3 block slides along the diagonal.
    let palette = [[16, 24, 64, 255], [240, 200, 40, 255], [200, 60, 60, 255]];
    let (width, height) = (12, 12);
    let frames: Vec<Vec<u8>> = (0..4)
        .map(|step| {
            (0..width * height)
                .map(|i| {
                    let (x, y) = (i % width, i / width);
                    let inside = (step * 3..step * 3 + 3).contains(&x)
                        && (step * 3..step * 3 + 3).contains(&y);
                    if inside {
                        1
                    } else {
                        ((x + y) % 5 == 0) as u8 * 2
                    }
                })
                .collect()
        })
        .collect();
    let bytes = encode(&frames, width, height, &palette, &[10])?;
    let (w, h, shown, delays) = decode(&bytes)?;
    let same = shown.iter().zip(&frames).all(|(colors, indices)| {
        colors
            .iter()
            .zip(indices)
            .all(|(c, &i)| *c == palette[i as usize])
    });
    println!(
        "indexed: {w} by {h}, {} frames, delays {delays:?}, {} bytes",
        shown.len(),
        bytes.len()
    );
    println!("  every pixel back: {same}");

    // The same frames with the first palette color made clear: pixel art on no background.
    // Alpha under 128 is clear, and a clear pixel decodes as [0, 0, 0, 0].
    let mut cut = palette;
    cut[0] = [0, 0, 0, 0];
    let bytes = encode(&frames, width, height, &cut, &[10])?;
    let (_, _, shown, _) = decode(&bytes)?;
    let same = shown.iter().zip(&frames).all(|(colors, indices)| {
        colors
            .iter()
            .zip(indices)
            .all(|(c, &i)| *c == cut[i as usize])
    });
    println!(
        "indexed with a clear color: {} bytes, every pixel back: {same}",
        bytes.len()
    );

    // RGBA frames with a hole work the same way.
    let mut rgba: Vec<[u8; 4]> = frames[0].iter().map(|&i| palette[i as usize]).collect();
    rgba[0] = [0, 0, 0, 0];
    let bytes = encode_rgba(&[rgba.clone()], width, height, &[0], false)?;
    let (_, _, shown, _) = decode(&bytes)?;
    println!(
        "rgba with a hole: {} bytes, every pixel back: {}",
        bytes.len(),
        shown[0] == rgba
    );
    Ok(())
}
