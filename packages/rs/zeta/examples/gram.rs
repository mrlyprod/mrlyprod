// Gram points g_n are where the phase theta(t) of zeta equals n*pi.
// Gram's law says exactly one zero lies between g_n and g_(n+1). It holds here, and fails first near n = 126.
// cargo run -p mrly-zeta --example gram
use mrly_zeta::Line;

fn main() {
    let line = Line::new();
    let zeros = line.zeros(10);
    println!("   n  gram point  zeros in (g_n, g_n+1)");
    for n in -1..9 {
        let (a, b) = (line.gram(n), line.gram(n + 1));
        let inside: Vec<String> = zeros
            .iter()
            .filter(|&&t| t > a && t < b)
            .map(|t| format!("{t:.4}"))
            .collect();
        println!("{n:>4}  {a:>10.4}  {}", inside.join(" "));
    }
}
