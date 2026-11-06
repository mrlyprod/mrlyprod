// Fill count of the carpet tile as a polynomial in k, the number of even positions on a side.
// The side is n = 2k - 1, so k = 2, 3, 4 gives sides 3, 5, 7. The polynomial and a direct count agree.
// cargo run -p mrly-sequences --example fill_polynomial_in_k

use mrly_math::Code;
use mrly_math::factory::code_to_corners;
use mrly_sequences::counts::{Closed, fill, fill_polynomial};

fn main() {
    let carpet = Code::from(7u64);
    let corners = code_to_corners(carpet, 2, 2).unwrap();
    let poly = fill_polynomial(&corners, 2);
    println!("fills = {}", Closed::Polynomial(poly.clone()).text());
    for k in 2..=6i128 {
        let n = (2 * k - 1) as usize;
        let by_poly: i128 = poly
            .iter()
            .enumerate()
            .map(|(power, c)| c * k.pow(power as u32))
            .sum();
        println!(
            "k={k} side {n:>2}: polynomial {by_poly:>3}, counted {}",
            fill(carpet, n, 2, 1, 2).unwrap()
        );
    }
}
