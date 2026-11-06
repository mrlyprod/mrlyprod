// The same picture as pixels, for places that cannot show svg.
// image gives an mrly_image::Image, which is a width, a height and one RGBA value per pixel.
// The sha-256 of those pixels is the proof that every machine draws the same thing.
// cargo run -p mrly-avatar --example image
use mrly_avatar::image;
use mrly_name::sha;

fn main() {
    for size in [9, 36, 64] {
        let picture = image("mrly", size).unwrap();
        let bytes: Vec<u8> = picture.colors.iter().flatten().copied().collect();
        println!(
            "size {size:>2}  {}x{}  sha256 {}",
            picture.width,
            picture.height,
            &sha::hex(&bytes)[..16]
        );
    }

    // At size 9 each pixel is one cell, margin included. The ink shows as # and the ground as a dot.
    let small = image("mrly", 9).unwrap();
    let ground = small.colors[0];
    println!();
    for row in small.colors.chunks(small.width) {
        let line: String = row
            .iter()
            .map(|&px| if px == ground { '.' } else { '#' })
            .collect();
        println!("{line}");
    }

    // Asking twice must give the same bytes.
    let again = image("mrly", 64).unwrap();
    println!();
    println!("repeatable {}", again == image("mrly", 64).unwrap());
}
