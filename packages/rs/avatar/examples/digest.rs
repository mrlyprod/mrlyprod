// The hash that feeds every picture, beside SHA-256.
// The sponge digest is 256 bits like SHA-256, but it is built from a cellular automaton.
// It is a picture hash: good at spreading names apart, and not a security tool.
// cargo run -p mrly-avatar --example digest
use mrly_avatar::digest;
use mrly_name::sha;

fn hex(bytes: &[u8]) -> String {
    digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    for input in ["", "a", "b", "mrly", "mrlz"] {
        println!(
            "{:<7} {}  sponge",
            format!("{input:?}"),
            hex(input.as_bytes())
        );
        println!("{:<7} {}  sha-256", "", sha::hex(input.as_bytes()));
    }

    // mrly and mrlz differ by one bit, yet their digests share nothing visible.
    let (a, b) = (hex(b"mrly"), hex(b"mrlz"));
    let same = a.chars().zip(b.chars()).filter(|(x, y)| x == y).count();
    println!();
    println!("mrly vs mrlz: {same} of 64 hex digits agree, 4 expected by chance");
}
