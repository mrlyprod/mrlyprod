use crate::sums::odds;
use mrlynum::series::{beta, chi4, dirichlet, zeta};
use std::f64::consts::PI;

const TERMS: usize = 10_000_000;

fn sums(limit: usize) -> [f64; 4] {
    let mut out = [0.0; 4];
    for n in odds(limit) {
        let x = n as f64;
        let sign = f64::from(chi4(n));
        out[0] += sign;
        out[1] += sign / x;
        out[2] += sign / (x * x);
        out[3] += 1.0 / (x * x);
    }
    out
}

pub fn run() {
    let catalan = beta(2.0, TERMS);
    let eisenstein = dirichlet(2.0, &[0, 1, -1], TERMS);
    let zeta3 = zeta(3.0, 2_000_000);
    println!("constants from their own series");
    println!("  G = {catalan:.10}  L(2, chi_-3) = {eisenstein:.10}  zeta(3) = {zeta3:.10}");
    println!(
        "  carpet split  pi/4 + pi^2/32     = {:.10}",
        PI / 4.0 + PI * PI / 32.0
    );
    println!(
        "  carpet mean   G/8                = {:.10}   G/8 - 1/8 = {:.10}",
        catalan / 8.0,
        catalan / 8.0 - 0.125
    );
    println!(
        "  void mean     (pi + pi^2)/16     = {:.10}",
        (PI + PI * PI) / 16.0
    );
    println!(
        "  flat stack    pi^2 ln2/(7 zeta3) = {:.10}",
        PI * PI * 2f64.ln() / (7.0 * zeta3)
    );
    println!(
        "  tree mean     pi/48+pi^2/48+G/6  = {:.10}",
        PI / 48.0 + PI * PI / 48.0 + catalan / 6.0
    );
    println!("partial character sums over odd n <= N, M layers, from the exact ink laws");
    for limit in [53usize, 55] {
        let [s0, s1, s2, s3] = sums(limit);
        println!(
            "  N = {limit}: M(I1 - I3 + 1/4) = {:.10}  M(mean ink - 1/2 - eps/2) = {:.10}  void M(mean - 1/4) = {:.10}",
            s1 + s3 / 4.0,
            -s0 / 8.0 + s2 / 8.0,
            s1 / 4.0 + s3 / 2.0
        );
    }
}
