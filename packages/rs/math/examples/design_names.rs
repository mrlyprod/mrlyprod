// Every 2D pattern on a 2x2 grid of corners is a number from 0 to 15, called a code.
// Ten of those codes have a name. This lists them.
// cargo run -p mrly-math --example design_names

use mrly_math::name::words;

fn main() {
    for (code, name) in words(2, 2) {
        println!("code {code:>2} is {name}");
    }
}
