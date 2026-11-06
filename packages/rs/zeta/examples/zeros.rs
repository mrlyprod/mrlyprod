// The first ten zeros of the Riemann zeta function on the critical line, s = 1/2 + i*t.
// Each is a height t where the Hardy Z function crosses zero.
// cargo run -p mrly-zeta --example zeros
use mrly_zeta::Line;

fn main() {
    let line = Line::new();
    for (i, t) in line.zeros(10).iter().enumerate() {
        println!("{:>2}  {:.6}", i + 1, t);
    }
}
