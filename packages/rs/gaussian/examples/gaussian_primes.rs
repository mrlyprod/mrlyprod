// Gaussian primes a + bi in a window of the complex integers, drawn as an SVG.
// Blue is split (the norm is prime), red is inert (a rational prime 3 mod 4 on an axis), green is ramified (1 + i).
// cargo run -p mrly-gaussian --example gaussian_primes
use mrly_gaussian::{Class, Ring, Window};

fn main() {
    let radius = 14;
    let window = Window::new(Ring::Gaussian, radius);
    let step = 10.0;
    let span = (2 * radius + 1) as f64 * step;
    let mid = span / 2.0;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {span} {span}\" width=\"{span}\" height=\"{span}\">"
    );
    println!("<rect width=\"{span}\" height=\"{span}\" fill=\"#fff\"/>");
    println!("<path d=\"M0 {mid}H{span}M{mid} 0V{span}\" stroke=\"#bbb\" stroke-width=\"0.5\"/>");
    for (class, fill) in [
        (Class::Split, "#1f6feb"),
        (Class::Inert, "#d1242f"),
        (Class::Ramified, "#2da44e"),
    ] {
        println!("<g fill=\"{fill}\">");
        for (a, b) in window.points() {
            if window.class(a, b) == class {
                // Screen y grows downward, so b is flipped.
                let (x, y) = (mid + a as f64 * step, mid - b as f64 * step);
                println!("<circle cx=\"{x}\" cy=\"{y}\" r=\"3.5\"/>");
            }
        }
        println!("</g>");
    }
    println!("</svg>");
}
