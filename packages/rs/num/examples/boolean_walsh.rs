// Boolean functions of 3 inputs, stored as an 8-bit truth table.
// The Walsh spectrum shows how close a function is to a straight XOR.
// Nonlinearity is the distance to the nearest such function; higher is harder to approximate.
// cargo run -p mrly-num --example boolean_walsh

use mrly_num::boolean::{is_balanced, nonlinearity, sac, walsh_spectrum};

fn main() {
    let functions: [(&str, u128); 3] = [
        ("xor of 3 bits", 0b10010110),
        ("majority", 0b11101000),
        ("and of 3 bits", 0b10000000),
    ];
    for (name, code) in functions {
        println!(
            "{name:<14} walsh={:?} nonlinearity={} balanced={} sac={:.3}",
            walsh_spectrum(code, 3),
            nonlinearity(code, 3),
            is_balanced(code, 3),
            sac(code, 3)
        );
    }
}
