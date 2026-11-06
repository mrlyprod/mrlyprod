// Two checksums guard a PNG file: CRC-32 after every chunk, Adler-32 after the compressed pixels.
// Printed: both sums for well-known test strings, and the CRC found inside a real file.
// cargo run -p mrly-png --example checksums
use mrly_png::{adler32, crc32, encode};

fn main() -> Result<(), String> {
    // Published check values: cbf43926 for the CRC-32 of "123456789", 11e60398 for the Adler-32 of "Wikipedia".
    println!("crc32   \"123456789\"  {:08x}", crc32(b"123456789"));
    println!("adler32 \"Wikipedia\"  {:08x}", adler32(b"Wikipedia"));

    // One changed letter gives a very different CRC, which is why it catches damage.
    println!("crc32   \"123456780\"  {:08x}", crc32(b"123456780"));
    println!("crc32   \"\"           {:08x}", crc32(b""));
    println!("adler32 \"\"           {:08x}", adler32(b""));

    // In a file the CRC covers the chunk name and body. The first chunk, IHDR, sits at bytes 12 to 29
    // and its checksum is stored right after, big end first.
    let bytes = encode(&[[9, 9, 9, 255]], 1, 1)?;
    let stored = u32::from_be_bytes([bytes[29], bytes[30], bytes[31], bytes[32]]);
    println!(
        "IHDR stored {stored:08x}, computed {:08x}",
        crc32(&bytes[12..29])
    );
    Ok(())
}
