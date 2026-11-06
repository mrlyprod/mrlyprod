// The Apollonian gasket: start with one big circle and two inside it, fill every gap with the circle that fits.
// Curvature is 1 over the radius, so the big outer circle counts as negative.
// cargo run -p mrly-apollonian --example gasket
use mrly_apollonian::{frame, grow};

fn main() {
    let packing = grow("-1,2,2,3", 150).unwrap();
    let [x0, y0, x1, y1] = frame(&packing);
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{:.3} {:.3} {:.3} {:.3}\">",
        x0,
        y0,
        x1 - x0,
        y1 - y0
    );
    println!("<g fill=\"none\" stroke=\"#000\" stroke-width=\"0.002\">");
    let drawn = packing.root.iter().chain(&packing.circles);
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
