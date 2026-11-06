// scale turns every pixel into a block. A PNG barely grows with it, because flat blocks repeat
// and the compressor only notes the repetition. Printed: one picture at growing scales.
// cargo run -p mrly-image --example png_scale
use mrly_image::{Image, to_png};

fn main() {
    // An 8 x 8 checkerboard of four colours.
    let palette = [
        [255, 61, 64, 255],
        [0, 140, 255, 255],
        [255, 209, 0, 255],
        [50, 204, 88, 255],
    ];
    let colors = (0..64).map(|i| palette[(i % 8 + i / 8 * 3) % 4]).collect();
    let small = Image::new(8, 8, colors).unwrap();

    println!("scale     size   pixels  rgba bytes  png bytes");
    for scale in [1, 2, 4, 8, 16, 32] {
        let image = small.scale(scale).unwrap();
        let bytes = to_png(&image).unwrap();
        let pixels = image.colors.len();
        println!(
            "{scale:>5} {:>4}x{:<4} {pixels:>7} {:>11} {:>10}",
            image.width,
            image.height,
            pixels * 4,
            bytes.len()
        );
    }
    // 1024 times the pixels, not even twice the bytes.
}
