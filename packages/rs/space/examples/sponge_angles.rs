// The level 2 Menger sponge from three angles: straight on, from above and to the right, and corner-on.
// Straight on, the tunnels line up and you see through them; turned, their walls hide each other.
// The frame fits the sponge's bounding sphere, so no angle ever clips it.
// cargo run -p mrly-space --example sponge_angles
use mrly_image::svg;
use mrly_space::{View, render};
use mrly_three::carpet;

fn main() {
    let sponge = carpet(3, 2).unwrap();
    let mut images = Vec::new();
    for (yaw, pitch) in [(0.0, 0.0), (30.0, 20.0), (45.0, 35.26)] {
        let view = View {
            width: 40,
            height: 40,
            yaw,
            pitch,
            samples: 2,
            ..View::default()
        };
        images.push(render(&sponge, &view).unwrap());
    }
    println!("{}", svg(&images, 4, 4));
}
