// A GIF holds at most 256 colors, so encode_rgba() cuts a true-color picture down to a palette.
// It splits the colors into boxes by median cut, then moves each palette color to the mean of the pixels nearest it.
// cargo run -p mrly-gif --example quantize
use mrly_gif::{decode, encode_rgba};

fn error(a: &[[u8; 4]], b: &[[u8; 4]]) -> f64 {
    let total: u32 = a
        .iter()
        .zip(b)
        .map(|(p, q)| (0..3).map(|c| p[c].abs_diff(q[c]) as u32).sum::<u32>())
        .sum();
    total as f64 / (3 * a.len()) as f64
}

fn blur(colors: &[[u8; 4]], width: usize) -> Vec<[u8; 4]> {
    let height = colors.len() / width;
    let mut out = Vec::new();
    for by in (0..height).step_by(4) {
        for bx in (0..width).step_by(4) {
            let mut sum = [0u32; 3];
            for y in by..by + 4 {
                for x in bx..bx + 4 {
                    for c in 0..3 {
                        sum[c] += colors[y * width + x][c] as u32;
                    }
                }
            }
            out.push([
                (sum[0] / 16) as u8,
                (sum[1] / 16) as u8,
                (sum[2] / 16) as u8,
                255,
            ]);
        }
    }
    out
}

fn main() -> Result<(), String> {
    // 128 by 128: red grows to the right, blue grows downwards, green with both. 16384 colors.
    let width = 128;
    let frames: [Vec<[u8; 4]>; 1] = [(0..width * width)
        .map(|i| {
            let (x, y) = (i % width, i / width);
            [(x * 2) as u8, (x + y) as u8, (y * 2) as u8, 255]
        })
        .collect()];
    let gradient = &frames[0];
    let soft = blur(gradient, width);

    // Plain: each pixel takes its nearest palette color. Dithered: each pixel's error is spread
    // onto its neighbors (Floyd-Steinberg), so small patches keep their average color.
    println!("mean error a channel   colors   bytes   per pixel   per 4x4 patch");
    for (name, dither) in [("nearest color", false), ("dithered", true)] {
        let bytes = encode_rgba(&frames, width, width, &[0], dither)?;
        let (_, _, shown, _) = decode(&bytes)?;
        let mut colors = shown[0].clone();
        colors.sort_unstable();
        colors.dedup();
        println!(
            "{name:<20} {:>8} {:>7} {:>11.2} {:>15.2}",
            colors.len(),
            bytes.len(),
            error(gradient, &shown[0]),
            error(&soft, &blur(&shown[0], width))
        );
    }
    Ok(())
}
