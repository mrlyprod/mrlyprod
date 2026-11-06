// snap moves every pixel to the nearest color of a small palette.
// cargo run -p mrly-color --example snap
use mrly_color::{BLUE, Color, INDIGO, PINK, PURPLE, RED, gradient, snap};

fn main() {
    // A smooth run from blue to red, as raw RGBA pixels.
    let smooth = gradient(&[BLUE, RED], 40).unwrap();
    let pixels: Vec<[u8; 4]> = smooth.iter().map(|c| [c.r, c.g, c.b, c.a]).collect();

    // Snap to five colors. Nearest is measured by squared RGBA distance.
    let palette = [BLUE, INDIGO, PURPLE, PINK, RED];
    let snapped = snap(&pixels, &palette);

    let cell = 12;
    let width = 16 + pixels.len() * cell;
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} 76\">");
    println!("<rect width=\"{width}\" height=\"76\" fill=\"#808080\"/>");
    for (row, line) in [&pixels, &snapped].into_iter().enumerate() {
        for (i, px) in line.iter().enumerate() {
            let c = Color::rgba(px[0], px[1], px[2], px[3]);
            println!(
                "<rect x=\"{}\" y=\"{}\" width=\"{cell}\" height=\"{cell}\" fill=\"{}\"/>",
                8 + i * cell,
                8 + row * 36,
                c.to_hex()
            );
        }
    }
    println!("</svg>");
}
