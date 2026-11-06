// A Chladni plate vibrating in the mode cos(3 pi x) cos(5 pi y) + cos(5 pi x) cos(3 pi y), drawn as its nodal lines: the curves where the plate stays still.
// plate() gives the height of the plate on a grid; lines() cuts that grid along zero and returns segments (x0, y0, x1, y1) in the unit square.
// The sign argument picks the symmetric (+1) or antisymmetric (-1) mix of the two terms.
// cargo run -p mrly-chladni --example nodal_lines
use mrly_chladni::{lines, plate};

fn main() {
    let size = 48;
    let field = plate(3, 5, 1, size).unwrap();
    let segments = lines(&field, size).unwrap();
    let side = 240.0;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {side} {side}\" width=\"{side}\" height=\"{side}\">"
    );
    println!("<rect width=\"{side}\" height=\"{side}\" fill=\"#fff\" stroke=\"#999\"/>");
    let mut d = String::new();
    for s in segments.chunks(4) {
        d += &format!(
            "M{:.1} {:.1}L{:.1} {:.1}",
            s[0] * side,
            s[1] * side,
            s[2] * side,
            s[3] * side
        );
    }
    println!(
        "<path d=\"{d}\" fill=\"none\" stroke=\"#1f2a44\" stroke-width=\"1.5\" stroke-linecap=\"round\"/>"
    );
    println!("</svg>");
}
