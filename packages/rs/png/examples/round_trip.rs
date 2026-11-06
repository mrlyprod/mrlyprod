// Encode a small picture as PNG bytes, decode it again, and check nothing changed.
// PNG is lossless: every pixel comes back exactly, the see-through ones included.
// cargo run -p mrly-png --example round_trip
use mrly_png::{decode, encode};

fn main() -> Result<(), String> {
    // A 16 x 16 picture: a red disc with a soft edge on a transparent background.
    let (width, height) = (16, 16);
    let mut colors = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let (dx, dy) = (x as f64 - 7.5, y as f64 - 7.5);
            let inside = (7.5 - (dx * dx + dy * dy).sqrt()).clamp(0.0, 1.0);
            colors.push([255, 61, 64, (inside * 255.0) as u8]);
        }
    }

    let bytes = encode(&colors, width, height)?;
    let (w, h, back) = decode(&bytes)?;

    println!("picture      {width} x {height}");
    println!("raw rgba     {} bytes", colors.len() * 4);
    println!("png          {} bytes", bytes.len());
    println!("decoded      {w} x {h}");
    println!("same pixels  {}", back == colors);

    // Every PNG file starts with the same eight bytes.
    println!("signature    {:?}", &bytes[..8]);
    Ok(())
}
