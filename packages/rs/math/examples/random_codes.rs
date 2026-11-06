// Draw random codes from a fixed seed, each with at least 2 filled corners.
// The same seed always gives the same list.
// cargo run -p mrly-math --example random_codes

use mrly_math::factory::random_code;
use mrly_rand::Rng;

fn main() {
    let mut rng = Rng::new(7);
    for _ in 0..8 {
        let code = random_code(2, 2, &mut rng, 2).unwrap();
        println!("code {:>2} = {:04b}", code.get(), code.get());
    }
}
