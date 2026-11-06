// to_png turns an image into the bytes of a PNG file. from_png reads such bytes back.
// Printed: a small heart written and read again, then what happens with bytes that are no PNG.
// cargo run -p mrly-image --example png
use mrly_image::{Image, from_png, to_png};

const HEART: [&str; 7] = [
    ".##.##.", "#######", "#######", "#######", ".#####.", "..###..", "...#...",
];

fn main() {
    // Red where the drawing has a #, fully see-through elsewhere.
    let paint = |c: char| {
        if c == '#' {
            [255, 61, 64, 255]
        } else {
            [0, 0, 0, 0]
        }
    };
    let colors = HEART
        .iter()
        .flat_map(|row| row.chars())
        .map(paint)
        .collect();
    let heart = Image::new(7, 7, colors).unwrap();

    let bytes = to_png(&heart).unwrap();
    let back = from_png(&bytes).unwrap();
    println!(
        "image       {} x {}, {} bytes as rgba",
        heart.width,
        heart.height,
        heart.colors.len() * 4
    );
    println!("png         {} bytes", bytes.len());
    println!("starts      {:?}", String::from_utf8_lossy(&bytes[1..4]));
    println!("read back   {} x {}", back.width, back.height);
    println!("same image  {}", back == heart);

    // Nothing is lost on the way: the clear pixels keep their zero alpha.
    println!("corner      {:?}", back.colors[0]);
    println!("middle      {:?}", back.colors[3 * 7 + 3]);

    // Bytes that are not a PNG come back as an error, not a crash.
    println!("garbage     {}", from_png(b"hello").unwrap_err());
}
