// The draw methods of Rng, all from seed 1.
// A seed fixes the whole stream, so these lines never change.
// cargo run -p mrly-rand --example draws
use mrly_rand::Rng;

fn main() {
    let mut rng = Rng::new(1);
    println!("unit     {:.6}", rng.unit());
    println!("below 10 {}", rng.below(10));
    println!("range    {}", rng.range(-5, 5));
    println!("boolean  {}", rng.boolean());
    println!("chance   {}", rng.chance(0.25));

    // A clone carries the same state, so it repeats what comes next.
    let mut fork = rng.clone();
    println!("next     {} {}", rng.below(1000), fork.below(1000));

    // A new seed starts a different stream.
    println!("seed 2   {:.6}", Rng::new(2).unit());
}
