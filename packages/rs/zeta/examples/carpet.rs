// A word of odd sides builds a carpet: split every filled square into s by s and punch out the centre.
// The word [3, 3] is the Sierpinski carpet at level 2. Here: its picture and its counts.
// cargo run -p mrly-zeta --example carpet
use mrly_zeta::sieve::{cells, holes, raster, ratio, side};

fn main() {
    let word = [3, 3];
    let (n, sites) = raster(&word).unwrap();
    for row in sites.chunks(n) {
        let line: String = row
            .iter()
            .map(|&b| if b == 1 { '#' } else { '.' })
            .collect();
        println!("{line}");
    }
    println!();
    println!("side   {}", side(&word).unwrap());
    println!("filled {}", cells(&word, 2).unwrap());
    println!("holes  {}", holes(&word, 2).unwrap());
    println!(
        "filled / total {:.4}  (8/9)^2 = {:.4}",
        ratio(&word, 2).unwrap(),
        (8.0f64 / 9.0).powi(2)
    );
}
