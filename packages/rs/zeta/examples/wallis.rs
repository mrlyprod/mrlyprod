// A carpet whose sides grow, 3 then 5 then 7 and so on, keeps a share of its squares that settles on pi/4.
// The share is the product of (1 - 1/s^2) over the sides, which is Wallis's product for pi/4.
// cargo run -p mrly-zeta --example wallis
use mrly_zeta::sieve::{PLANE_LIMIT, odd_word, ratio};

fn main() {
    println!("levels  share filled");
    for levels in [1, 2, 4, 8, 16, 64, 256, 1024] {
        println!("{levels:>6}  {:.6}", ratio(&odd_word(levels), 2).unwrap());
    }
    println!();
    println!("pi/4    {PLANE_LIMIT:.6}");
}
