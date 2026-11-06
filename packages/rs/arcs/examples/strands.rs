// Truchet quarter arcs on a 9 by 9 grid. Every cell joins the midpoints of its edges in pairs with two quarter circles.
// Which pair depends on the cell's bit in the design (base 3, code 495: a 3 by 3 tile with its middle left out, repeated twice).
// The arcs link into curves. curves() returns each as a list of arcs; each curve gets its own colour. Thick curves are closed loops (3 here), thin ones run off the edge (18).
// cargo run -p mrly-arcs --example strands
use mrly_arcs::{curves, draw};
use mrly_math::code::Code;

fn main() {
    let (code, base, level) = (Code(495), 3, 2);
    let tiles = draw(code, base, level).unwrap();
    let list = curves(code, base, level).unwrap();
    let n = tiles.side;
    let s = 24.0;
    let r = s / 2.0;
    let size = n as f64 * s;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {size} {size}\" width=\"{size}\" height=\"{size}\">"
    );
    println!("<rect width=\"{size}\" height=\"{size}\" fill=\"#fff\"/>");
    for (k, curve) in list.iter().enumerate() {
        let width = if curve.closed { 3.5 } else { 1.5 };
        let hue = k * 47 % 360;
        let mut d = String::new();
        for &arc in &curve.arcs {
            // Arc 2i and 2i+1 are the two arcs of cell i. A set bit joins around corners (x, y) and (x + 1, y + 1), a clear bit around (x + 1, y) and (x, y + 1).
            let (x, y) = ((arc / 2) % n, (arc / 2) / n);
            let filled = tiles.cells[arc / 2] & 1 == 1;
            let (cx, cy) = match (arc % 2 == 1, filled) {
                (true, true) => (x + 1, y + 1),
                (true, false) => (x, y + 1),
                (false, true) => (x, y),
                (false, false) => (x + 1, y),
            };
            let (cx, cy) = (cx as f64 * s, cy as f64 * s);
            let from = ((x as f64 + 0.5) * s, cy);
            let to = (cx, (y as f64 + 0.5) * s);
            let cross = (from.0 - cx) * (to.1 - cy) - (from.1 - cy) * (to.0 - cx);
            let sweep = u8::from(cross > 0.0);
            d += &format!(
                "M{} {}A{r} {r} 0 0 {sweep} {} {}",
                from.0, from.1, to.0, to.1
            );
        }
        println!(
            "<path d=\"{d}\" fill=\"none\" stroke=\"hsl({hue},70%,45%)\" stroke-width=\"{width}\" stroke-linecap=\"round\"/>"
        );
    }
    println!("</svg>");
}
