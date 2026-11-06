// Resize a tiny gray image three ways and compare the numbers.
// cargo run -p mrly-image --example resample
use mrly_image::Image;
use mrly_image::sample::Filter;

fn show(label: &str, image: &Image) {
    println!("{label} {}x{}", image.width, image.height);
    for row in image.colors.chunks(image.width) {
        let cells: Vec<String> = row.iter().map(|px| format!("{:>3}", px[0])).collect();
        println!("  {}", cells.join(" "));
    }
}

fn main() {
    // A 4x4 gray image with a uniform bottom row.
    let gray = |v: u8| [v, v, v, 255];
    let values = [
        0, 0, 200, 200, 0, 0, 200, 200, 0, 0, 200, 200, 100, 100, 100, 100,
    ];
    let image = Image::new(4, 4, values.iter().map(|&v| gray(v)).collect()).unwrap();
    show("source", &image);

    // Down to 2x2: nearest picks one pixel, box averages the pixels it covers.
    show("nearest", &image.resample(2, 2, Filter::Nearest).unwrap());
    show("box", &image.resample(2, 2, Filter::Box).unwrap());
    // Linear blends the four nearest pixels, so it also smooths when growing.
    show("linear", &image.resample(2, 2, Filter::Linear).unwrap());
    show("linear up", &image.resample(6, 3, Filter::Linear).unwrap());
    show(
        "nearest up",
        &image.resample(6, 3, Filter::Nearest).unwrap(),
    );
}
