// A box blur spreads one bright pixel over its neighbors.
// cargo run -p mrly-image --example blur
use mrly_image::blur;

fn main() {
    let (w, h) = (5, 5);
    let mut pixels = vec![[0, 0, 0, 255]; w * h];
    pixels[2 * w + 2] = [225, 225, 225, 255];

    // radius r averages the (2r+1) x (2r+1) window around each pixel.
    // radius 1 should turn the bright pixel into a 3x3 block of 225 / 9 = 25.
    // radius 2 should fill the whole 5x5 with 225 / 25 = 9.
    for radius in [1, 2] {
        println!("radius {radius}");
        let out = blur(&pixels, w, h, radius);
        for row in out.chunks(w) {
            let cells: Vec<String> = row.iter().map(|px| format!("{:>3}", px[0])).collect();
            println!("  {}", cells.join(" "));
        }
    }
}
