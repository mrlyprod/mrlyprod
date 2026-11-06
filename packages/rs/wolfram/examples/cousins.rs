// Rules that are the same rule in disguise: mirror left and right, swap 0 and 1, or both.
// Wolfram's class has up to 4 rules. The NPN class also allows the 48 symmetries of the cube, so it is wider.
// cargo run -p mrly-wolfram --example cousins
use mrly_wolfram::{npn_class, wolfram_class};

fn main() {
    for rule in [30u8, 90, 204] {
        println!("rule {rule}");
        println!("  mirror and complement  {:?}", wolfram_class(rule));
        println!("  with cube symmetries   {:?}", npn_class(rule));
    }
}
