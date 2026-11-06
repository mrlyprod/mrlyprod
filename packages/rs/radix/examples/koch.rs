// The Koch curve from four digits in base 3 on the Eisenstein lattice.
// Each digit carries a twist, a unit that turns the copy of the lower word. The words come out in order along the curve, so joining them draws it.
// cargo run -p mrly-radix --example koch
use mrly_gaussian::Ring;
use mrly_radix::koch;

fn main() {
    let radix = koch().unwrap();
    let level = 4;
    let (width, height) = (360.0, 110.0);
    let pad = 8.0;
    let words = radix.words(level);
    let reach = 3f64.powi(level as i32);
    let scale = (width - 2.0 * pad) / reach;
    let points: Vec<String> = words
        .iter()
        .map(|&(a, b)| {
            let (x, y) = Ring::Eisenstein.place(a, b);
            format!("{:.1},{:.1}", pad + x * scale, height - pad - y * scale)
        })
        .collect();
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\">"
    );
    println!("<rect width=\"{width}\" height=\"{height}\" fill=\"#fff\"/>");
    println!(
        "<polyline points=\"{}\" fill=\"none\" stroke=\"#1f6feb\" stroke-width=\"1\"/>",
        points.join(" ")
    );
    println!("</svg>");
}
