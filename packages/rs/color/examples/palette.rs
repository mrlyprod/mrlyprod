// The fifteen palette colors, each with its light, base and dark shade.
// cargo run -p mrly-color --example palette
use mrly_color::{DARK, NAMES, PALETTE, shades};

fn main() {
    let (cell, gap, left, top) = (44, 6, 16, 16);
    let width = left * 2 + PALETTE.len() * (cell + gap) - gap;
    let height = top * 2 + 3 * (cell + gap) + 12;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" font-family=\"sans-serif\" font-size=\"9\">"
    );
    println!(
        "<rect width=\"{width}\" height=\"{height}\" fill=\"{}\"/>",
        DARK.bg.to_hex()
    );
    for (i, (&hue, name)) in PALETTE.iter().zip(NAMES).enumerate() {
        let x = left + i * (cell + gap);
        // shades gives [lighter, the color itself, darker].
        for (row, shade) in shades(hue).into_iter().enumerate() {
            let y = top + row * (cell + gap);
            println!(
                "<rect x=\"{x}\" y=\"{y}\" width=\"{cell}\" height=\"{cell}\" rx=\"4\" fill=\"{}\" stroke=\"{}\"/>",
                shade.to_hex(),
                DARK.line.to_hex()
            );
        }
        let y = top + 3 * (cell + gap) + 4;
        println!(
            "<text x=\"{}\" y=\"{y}\" text-anchor=\"middle\" fill=\"{}\">{name}</text>",
            x + cell / 2,
            DARK.dim.to_hex()
        );
    }
    println!("</svg>");
}
