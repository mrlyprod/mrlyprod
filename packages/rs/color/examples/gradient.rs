// gradient blends through any number of colors in a fixed number of steps.
// cargo run -p mrly-color --example gradient
use mrly_color::{BLUE, GREEN, RED, WHITE, YELLOW, gradient};

fn strip(colors: &[mrly_color::Color], steps: usize, y: usize) {
    let cell = 14;
    for (i, c) in gradient(colors, steps).unwrap().iter().enumerate() {
        println!(
            "<rect x=\"{}\" y=\"{y}\" width=\"{cell}\" height=\"28\" fill=\"{}\"/>",
            8 + i * cell,
            c.to_hex()
        );
    }
}

fn main() {
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 464 88\">");
    println!("<rect width=\"464\" height=\"88\" fill=\"#ffffff\"/>");
    // Stops are spread evenly. Three stops give two segments.
    strip(&[BLUE, WHITE, RED], 32, 8);
    strip(&[RED, YELLOW, GREEN], 32, 48);
    println!("</svg>");
}
