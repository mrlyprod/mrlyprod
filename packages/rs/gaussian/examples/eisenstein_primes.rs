// Eisenstein primes a + b*w (w a cube root of unity) in a hexagonal window, drawn as an SVG.
// The lattice has six units, so the picture has six-fold symmetry. Blue is split, red is inert (2 mod 3 on an axis), green is ramified (norm 3).
// cargo run -p mrly-gaussian --example eisenstein_primes
use mrly_gaussian::{Class, Ring, Window};

fn main() {
    let ring = Ring::Eisenstein;
    let radius = 12;
    let window = Window::new(ring, radius);
    let step = 11.0;
    let span = (2 * radius + 2) as f64 * step;
    let mid = span / 2.0;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {span:.0} {span:.0}\" width=\"{span:.0}\" height=\"{span:.0}\">"
    );
    println!("<rect width=\"{span:.0}\" height=\"{span:.0}\" fill=\"#fff\"/>");
    for (class, fill) in [
        (Class::Split, "#1f6feb"),
        (Class::Inert, "#d1242f"),
        (Class::Ramified, "#2da44e"),
    ] {
        println!("<g fill=\"{fill}\">");
        for (a, b) in window.points() {
            if window.class(a, b) == class {
                // place() maps (a, b) to the hexagonal plane; flip y for the screen.
                let (px, py) = ring.place(a, b);
                let (x, y) = (mid + px * step, mid - py * step);
                println!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3.5\"/>");
            }
        }
        println!("</g>");
    }
    println!("</svg>");
}
