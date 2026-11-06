// The first Bernoulli numbers as exact fractions.
// Every odd one after B1 is zero.
// cargo run -p mrly-num --example bernoulli_numbers

use mrly_num::series::bernoulli;

fn main() {
    for (i, (num, den)) in bernoulli(13).unwrap().iter().enumerate() {
        println!("B{i} = {num}/{den}");
    }
}
