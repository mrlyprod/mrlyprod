// One light, four places. light is the direction towards the lamp as the camera sees it:
// x to the right, y up, z towards you. Each face's shade follows how squarely it faces that direction.
// From the upper left (the default), from the right, from straight behind the camera, and from below.
// cargo run -p mrly-space --example light
use mrly_image::{Image, svg};
use mrly_space::{View, draw, solid};

fn main() {
    let icosa = solid("icosa").unwrap();
    let lights = [
        [-1.0, 4.0, 2.0],
        [4.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, -4.0, 1.0],
    ];
    let images: Vec<Image> = lights
        .iter()
        .map(|&light| {
            let view = View {
                width: 32,
                height: 32,
                light,
                samples: 2,
                ..View::default()
            };
            draw(&icosa, &view).unwrap()
        })
        .collect();
    println!("{}", svg(&images, 4, 4));
}
