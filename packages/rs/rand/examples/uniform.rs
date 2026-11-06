// 10000 draws of below(10) land evenly in ten buckets.
// cargo run -p mrly-rand --example uniform
use mrly_rand::Rng;

fn main() {
    let mut rng = Rng::new(42);
    let mut buckets = [0usize; 10];
    for _ in 0..10_000 {
        buckets[rng.below(10)] += 1;
    }
    // Each bucket should sit near 1000.
    for (digit, count) in buckets.iter().enumerate() {
        println!("{digit} {count:>5} {}", "#".repeat(count / 25));
    }
}
