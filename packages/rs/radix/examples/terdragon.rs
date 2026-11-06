// The terdragon curve: three digits in base 2 + w on the Eisenstein lattice, with twists of 0, 120 and 0 degrees.
// The words come out in order along the curve. Some corners are visited twice: 729 words land on 307 distinct points.
// cargo run -p mrly-radix --example terdragon
use mrly_gaussian::Ring;
use mrly_radix::terdragon;

fn main() {
    let radix = terdragon().unwrap();
    let level = 6;
    let words = radix.words(level);
    let placed: Vec<(f64, f64)> = words
        .iter()
        .map(|&(a, b)| Ring::Eisenstein.place(a, b))
        .collect();
    let xs = placed.iter().map(|p| p.0);
    let ys = placed.iter().map(|p| p.1);
    let (x0, x1) = (
        xs.clone().fold(f64::MAX, f64::min),
        xs.fold(f64::MIN, f64::max),
    );
    let (y0, y1) = (
        ys.clone().fold(f64::MAX, f64::min),
        ys.fold(f64::MIN, f64::max),
    );
    let (width, pad) = (360.0, 8.0);
    let scale = (width - 2.0 * pad) / (x1 - x0);
    let height = (y1 - y0) * scale + 2.0 * pad;
    let points: Vec<String> = placed
        .iter()
        .map(|&(x, y)| {
            format!(
                "{:.1},{:.1}",
                pad + (x - x0) * scale,
                height - pad - (y - y0) * scale
            )
        })
        .collect();
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height:.0}\" width=\"{width}\" height=\"{height:.0}\">"
    );
    println!("<rect width=\"{width}\" height=\"{height:.0}\" fill=\"#fff\"/>");
    println!(
        "<polyline points=\"{}\" fill=\"none\" stroke=\"#1f6feb\" stroke-width=\"1\"/>",
        points.join(" ")
    );
    println!("</svg>");
}
