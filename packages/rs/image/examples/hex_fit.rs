// Squash an image by sqrt(3)/2 along one side, the size a hexagon grid needs.
// cargo run -p mrly-image --example hex_fit
use mrly_image::sample::{Filter, HEX_RATIO, hex_fit, hex_size};

fn main() {
    println!("ratio {HEX_RATIO:.6}");
    // vertical false shrinks the height; true shrinks the width.
    println!("100x100, rows   -> {:?}", hex_size(100, 100, false));
    println!("100x100, columns -> {:?}", hex_size(100, 100, true));
    println!("1x1 never hits zero -> {:?}", hex_size(1, 1, false));

    // hex_fit resamples the pixels to that size and returns them with it.
    let pixels: Vec<[u8; 4]> = (0..16).map(|i| [i * 16, 0, 0, 255]).collect();
    let (w, h, out) = hex_fit(&pixels, 4, 4, false, Filter::Box).unwrap();
    println!("4x4 fitted -> {w}x{h}, {} pixels", out.len());
}
