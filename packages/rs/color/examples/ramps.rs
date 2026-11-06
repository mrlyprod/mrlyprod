// The three built-in Colorizers, from count 0 up to the maximum.
// cargo run -p mrly-color --example ramps
use mrly_color::Colorizer;
use mrly_color::ramp::color;

fn main() {
    let max = 64;
    let cell = 8;
    let (width, height) = (16 + (max + 1) * cell, 8 + 3 * 36);
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\">");
    println!("<rect width=\"{width}\" height=\"{height}\" fill=\"#808080\"/>");
    // Value 0 is the background. Values 1..=max walk the ramp.
    for (row, ramp) in [Colorizer::heat(), Colorizer::fire(), Colorizer::diverge()]
        .iter()
        .enumerate()
    {
        for value in 0..=max {
            println!(
                "<rect x=\"{}\" y=\"{}\" width=\"{cell}\" height=\"28\" fill=\"{}\"/>",
                8 + value * cell,
                8 + row * 36,
                color(ramp, value, max).to_hex()
            );
        }
    }
    println!("</svg>");
}
