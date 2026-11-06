// Four measures of how well a hash spreads its inputs, for the sponge and for SHA-256.
// avalanche: flip one input bit and count the output bits that change. Ideal 0.5.
// balance: the most lopsided output bit over many inputs, in standard deviations. Ideal under 3 or so.
// entropy: bits per byte of output, at most 8. Few samples cap it a little under 8 for any hash.
// collisions: inputs that land on the same first 16 output bits, against the number chance predicts.
// cargo run -p mrly-avatar --example quality
use mrly_avatar::digest;
use mrly_avatar::metrics::{avalanche, balance, collisions, entropy, expected};
use mrly_name::sha::sha256;

const TRIALS: usize = 120;
const KEYS: usize = 1500;
const WIDTH: usize = 16;

type Hasher<'a> = &'a dyn Fn(&[u8]) -> Vec<u8>;

fn main() {
    let sponge = |bytes: &[u8]| digest(bytes);
    let sha = |bytes: &[u8]| sha256(bytes).to_vec();
    let hashers: [(&str, Hasher); 2] = [("sponge", &sponge), ("sha-256", &sha)];
    println!("{:<8} avalanche  balance  entropy  collisions", "");
    for (name, hasher) in hashers {
        println!(
            "{name:<8} {:>9.4}  {:>7.2}  {:>7.4}  {:>10}",
            avalanche(hasher, TRIALS),
            balance(hasher, TRIALS),
            entropy(hasher, TRIALS),
            collisions(hasher, KEYS, WIDTH)
        );
    }
    println!();
    println!(
        "{KEYS} keys, first {WIDTH} bits: chance predicts {:.1} collisions",
        expected(KEYS, WIDTH)
    );
}
