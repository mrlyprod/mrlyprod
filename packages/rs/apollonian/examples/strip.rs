// The strip packing: circles fill the gaps between two parallel lines and two seed circles.
// The pattern repeats every unit to the left and right; this shows one period.
// cargo run -p mrly-apollonian --example strip
use mrly_apollonian::{frame, grow};

fn main() {
    let packing = grow("strip", 300).unwrap();
    let [x0, y0, x1, y1] = frame(&packing);
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">",
        x0,
        y0,
        x1 - x0,
        y1 - y0
    );
    println!("<g fill=\"none\" stroke=\"#000\" stroke-width=\"0.002\">");
    println!("<path d=\"M0 0H1M0 1H1\"/>");
    let drawn = packing
        .root
        .iter()
        .filter(|c| !c.is_line())
        .chain(&packing.circles);
    for c in drawn {
        let (x, y) = c.centre().unwrap();
        println!(
            "<circle cx=\"{:.3}\" cy=\"{:.3}\" r=\"{:.3}\"/>",
            x,
            y,
            c.radius().unwrap()
        );
    }
    println!("</g>");
    println!("</svg>");
}
