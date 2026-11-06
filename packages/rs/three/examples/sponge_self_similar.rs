// Volume of the thickened sponge (all points within radius r), scaled by r^(d - 3) where d is the dimension.
// The scaled value settles toward profile(r), which repeats exactly every time r shrinks by 3.
// cargo run -p mrly-three --example sponge_self_similar

use mrly_three::sponge::{profile, reading};

fn main() {
    println!("{:>10} {:>10} {:>10}", "radius", "scaled", "profile");
    let mut radius = 0.1;
    for _ in 0..8 {
        println!(
            "{radius:>10.6} {:>10.6} {:>10.6}",
            reading(radius).unwrap(),
            profile(radius).unwrap()
        );
        radius /= 3.0;
    }
}
