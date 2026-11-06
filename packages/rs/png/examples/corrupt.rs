// Broken input comes back as an error that says what is wrong. It never panics.
// Printed: a good file damaged in different ways, and what decode answers each time.
// cargo run -p mrly-png --example corrupt
use mrly_png::{crc32, decode, encode};

fn show(label: &str, bytes: &[u8]) {
    match decode(bytes) {
        Ok((width, height, _)) => println!("{label:<24} ok, {width} x {height}"),
        Err(error) => println!("{label:<24} {error}"),
    }
}

fn main() -> Result<(), String> {
    let colors: Vec<[u8; 4]> = (0..64u8).map(|i| [i * 4, 255 - i, i, 255]).collect();
    let good = encode(&colors, 8, 8)?;
    show("untouched", &good);
    show("no bytes at all", &[]);
    show("not a png", b"GIF89a, not a png at all");
    show("cut in half", &good[..good.len() / 2]);
    show("last 4 bytes missing", &good[..good.len() - 4]);

    // Every chunk carries a checksum, so one flipped bit in the pixel data is caught.
    let mut flipped = good.clone();
    flipped[50] ^= 1;
    show("one bit flipped", &flipped);

    // A header that asks for 100000 x 100000 pixels, with its checksum repaired to match.
    // The size is checked against a cap before any memory is set aside for the pixels.
    let mut huge = good.clone();
    huge[16..20].copy_from_slice(&100_000u32.to_be_bytes());
    huge[20..24].copy_from_slice(&100_000u32.to_be_bytes());
    let sum = crc32(&huge[12..29]);
    huge[29..33].copy_from_slice(&sum.to_be_bytes());
    show("claims 100000 x 100000", &huge);

    // A width of 9 instead of 8: the header is valid, but the rows no longer add up.
    let mut wide = good.clone();
    wide[19] = 9;
    let sum = crc32(&wide[12..29]);
    wide[29..33].copy_from_slice(&sum.to_be_bytes());
    show("width changed to 9", &wide);
    Ok(())
}
