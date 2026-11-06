// lightness slides a color toward black (0) or white (100); 50 leaves it alone.
// cargo run -p mrly-color --example lightness
use mrly_color::{BLUE, GREEN, ORANGE, PINK, PURPLE};

fn main() {
    let cell = 30;
    let hues = [BLUE, GREEN, ORANGE, PINK, PURPLE];
    let (width, height) = (16 + 11 * cell, 16 + hues.len() * cell);
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\">");
    println!("<rect width=\"{width}\" height=\"{height}\" fill=\"#808080\"/>");
    for (row, hue) in hues.iter().enumerate() {
        for step in 0..=10u8 {
            let shaded = hue.lightness(step * 10).unwrap();
            println!(
                "<rect x=\"{}\" y=\"{}\" width=\"{cell}\" height=\"{cell}\" fill=\"{}\"/>",
                8 + step as usize * cell,
                8 + row * cell,
                shaded.to_hex()
            );
        }
    }
    println!("</svg>");
}
