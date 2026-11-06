// Euclid's algorithm works in the Gaussian integers too: divide, keep the remainder, repeat.
// Each step picks the nearest quotient, so the remainder is always smaller than the divisor.
// cargo run -p mrly-gaussian --example gcd
use mrly_gaussian::Ring;

fn show((a, b): (i64, i64)) -> String {
    format!("{a:+}{b:+}i")
}

fn main() {
    let ring = Ring::Gaussian;
    let (mut z, mut w) = ((11, 3), (1, 8));
    println!("gcd({}, {})", show(z), show(w));
    println!("norms {} and {}", ring.norm(z.0, z.1), ring.norm(w.0, w.1));
    while w != (0, 0) {
        let (q, r) = ring.div_rem(z, w);
        println!(
            "{} = {} * {} + {}   norm of remainder {}",
            show(z),
            show(q),
            show(w),
            show(r),
            ring.norm(r.0, r.1)
        );
        z = w;
        w = r;
    }
    let g = ring.canon(z.0, z.1);
    println!("last non-zero remainder {}", show(z));
    println!("gcd, rotated to the first quadrant: {}", show(g));
    println!(
        "same from gaussian_gcd: {}",
        show(ring.gaussian_gcd((11, 3), (1, 8)))
    );
}
