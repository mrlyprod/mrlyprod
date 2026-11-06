// Supersampling: each pixel is the average of a samples by samples grid of points inside it.
// One sample gives hard stair steps; more samples soften every edge in proportion to how much it covers.
// A cube 20 pixels wide with 1, 2, 4 and 8 samples on each side of the pixel.
// cargo run -p mrly-space --example samples
use mrly_image::{Image, svg};
use mrly_space::{View, draw, solid};

fn main() {
    let cube = solid("cube").unwrap();
    let images: Vec<Image> = [1, 2, 4, 8]
        .iter()
        .map(|&samples| {
            let view = View {
                width: 20,
                height: 20,
                yaw: 25.0,
                pitch: 30.0,
                samples,
                ..View::default()
            };
            draw(&cube, &view).unwrap()
        })
        .collect();
    println!("{}", svg(&images, 2, 4));
}
