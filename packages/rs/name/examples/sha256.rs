// SHA-256 checked against published values, plus the short id form.
// cargo run -p mrly-name --example sha256
use mrly_name::sha;

fn main() {
    // The first two are the standard test vectors.
    println!("empty  {}", sha::hex(b""));
    println!("abc    {}", sha::hex(b"abc"));

    // 56 bytes forces the padding into a second block.
    println!(
        "long   {}",
        sha::hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
    );

    // short is the first 8 hex digits.
    println!("short  {}", sha::short(b"abc"));

    // sha256 gives the raw 32 bytes.
    println!("bytes  {}", sha::sha256(b"abc").len());
}
