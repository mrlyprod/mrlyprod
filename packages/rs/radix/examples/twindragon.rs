// Write complex integers in base 1 + i with digits 0 and 1. Every number has exactly one such form, so the words tile the plane.
// All 2^10 words of length ten are drawn: the twin dragon.
// cargo run -p mrly-radix --example twindragon
use mrly_radix::twindragon;
use std::collections::HashSet;

fn main() {
    let radix = twindragon().unwrap();
    let words = radix.words(10);
    let seen: HashSet<(i64, i64)> = words.iter().copied().collect();
    println!("{} words, {} distinct points", words.len(), seen.len());
    let (xs, ys): (Vec<i64>, Vec<i64>) = seen.iter().copied().unzip();
    let (x0, x1) = (*xs.iter().min().unwrap(), *xs.iter().max().unwrap());
    let (y0, y1) = (*ys.iter().min().unwrap(), *ys.iter().max().unwrap());
    // Imaginary part grows upward, so rows run from the top down. Each point is two characters wide to look square.
    for y in (y0..=y1).rev() {
        let row: String = (x0..=x1)
            .map(|x| if seen.contains(&(x, y)) { "##" } else { "  " })
            .collect();
        println!("{}", row.trim_end());
    }
}
