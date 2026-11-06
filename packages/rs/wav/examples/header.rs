// The 44 bytes in front of every plain WAV file, field by field.
// A WAV is a RIFF container: named chunks, each with a 4-byte size, little-endian throughout.
// cargo run -p mrly-wav --example header
use mrly_wav::encode;

fn main() -> Result<(), String> {
    // Three mono samples at 8000 Hz: 6 bytes of sound behind a 44-byte header.
    // A sample runs from -1 to 1; at 16 bits one step is 1/32768.
    let step = 1.0 / 32768.0;
    let bytes = encode(&[0.0, step, -step], 1, 8000, 16)?;
    let text = |at: usize| String::from_utf8_lossy(&bytes[at..at + 4]).to_string();
    let u32_at =
        |at: usize| u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
    let u16_at = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
    println!("file is {} bytes", bytes.len());
    println!(" 0  {:<6} container", text(0));
    println!(" 4  {:<6} bytes after this field", u32_at(4));
    println!(" 8  {:<6} kind", text(8));
    println!("12  {:<6} chunk: the format", text(12));
    println!("16  {:<6} chunk size", u32_at(16));
    println!(
        "20  {:<6} tag: 1 is whole-number pcm, 3 is float",
        u16_at(20)
    );
    println!("22  {:<6} channels", u16_at(22));
    println!("24  {:<6} samples a second", u32_at(24));
    println!("28  {:<6} bytes a second", u32_at(28));
    println!(
        "32  {:<6} bytes a frame (one sample for every channel)",
        u16_at(32)
    );
    println!("34  {:<6} bits a sample", u16_at(34));
    println!("36  {:<6} chunk: the sound", text(36));
    println!("40  {:<6} chunk size", u32_at(40));
    println!(
        "44  {:?} the samples 0, 1, -1 as 16-bit pairs",
        &bytes[44..]
    );
    Ok(())
}
