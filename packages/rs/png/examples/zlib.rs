// PNG compresses its pixel rows with zlib. The same pair of functions works on any bytes.
// Printed: a repetitive text squeezed at a few levels, read back, and two ways it can fail.
// cargo run -p mrly-png --example zlib
use mrly_png::{adler32, compress, decompress};

fn main() -> Result<(), String> {
    let text = "the quick brown fox jumps over the lazy dog. ".repeat(40);
    let bytes = text.as_bytes();
    println!("text       {} bytes", bytes.len());
    for level in [0, 1, 6, 9] {
        println!("level {level}    {} bytes", compress(bytes, level).len());
    }

    // decompress wants a limit: the most bytes it may hand back. Past it, it stops with an error.
    let packed = compress(bytes, 9);
    let back = decompress(&packed, bytes.len())?;
    println!("round trip {}", back == bytes);
    println!("limit 100  {}", decompress(&packed, 100).unwrap_err());

    // The last four bytes are a checksum of the data. Change one and the stream is refused.
    let mut wrong = packed.clone();
    let last = wrong.len() - 1;
    wrong[last] ^= 1;
    println!(
        "bad sum    {}",
        decompress(&wrong, bytes.len()).unwrap_err()
    );

    // zlib wraps a deflate stream: two header bytes in front, the Adler-32 sum of the data behind.
    // 78 says deflate with a 32 KiB window. The second byte hints at the level, and is chosen so
    // that the pair, read as one number, divides by 31.
    let n = packed.len();
    let stored = u32::from_be_bytes([packed[n - 4], packed[n - 3], packed[n - 2], packed[n - 1]]);
    println!("header     {:02x} {:02x}", packed[0], packed[1]);
    println!(
        "checksum   {stored:08x}, adler32 of the text {:08x}",
        adler32(bytes)
    );
    Ok(())
}
