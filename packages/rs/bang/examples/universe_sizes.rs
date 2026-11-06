// All designs on the corners of a d-dimensional cube: 2^(2^d) codes.
// Flips and axis swaps make many of them the same shape. This counts the distinct shapes.
// cargo run -p mrly-bang --example universe_sizes

use mrly_bang::bang;

fn main() {
    println!("dim codes shapes");
    for dimension in 1..=4 {
        let universe = bang(dimension).unwrap();
        println!(
            "{dimension:>3} {:>5} {:>6}",
            universe.total,
            universe.distinct()
        );
    }
}
