// A PNG file is a signature followed by chunks: a four-letter name, a body and a checksum each.
// Printed: the chunks of a small picture with three colours, one of them see-through.
// cargo run -p mrly-png --example chunks
use mrly_png::encode;

fn main() -> Result<(), String> {
    // 24 x 24: a clear background, a blue square, an orange dot in the middle.
    let (clear, blue, orange) = ([0, 0, 0, 0], [0, 140, 255, 255], [255, 143, 44, 255]);
    let mut colors = vec![clear; 24 * 24];
    for y in 4..20 {
        for x in 4..20 {
            let middle = (10..14).contains(&x) && (10..14).contains(&y);
            colors[y * 24 + x] = if middle { orange } else { blue };
        }
    }
    let bytes = encode(&colors, 24, 24)?;

    let meaning = |name: &[u8]| match name {
        b"IHDR" => "size, bit depth, colour type",
        b"PLTE" => "the colour table, three bytes a colour",
        b"tRNS" => "alpha for the first table entries",
        b"IDAT" => "the pixel rows, compressed",
        b"IEND" => "end of file",
        _ => "",
    };
    // After the 8-byte signature, each chunk is a 4-byte length, a 4-letter name,
    // the body, and a 4-byte checksum. Step from one to the next.
    println!("name  bytes  holds");
    let (mut at, mut count, mut bodies) = (8, 0, 0);
    while at < bytes.len() {
        let size = u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
        let name = &bytes[at + 4..at + 8];
        let label = String::from_utf8_lossy(name);
        println!("{label}  {size:>5}  {}", meaning(name));
        (at, count, bodies) = (at + 12 + size as usize, count + 1, bodies + size);
    }
    println!();
    println!(
        "signature 8 + bodies {bodies} + 12 x {count} chunks = {} bytes",
        bytes.len()
    );
    Ok(())
}
