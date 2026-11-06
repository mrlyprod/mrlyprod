// A turntable: the camera circles the level 1 sponge in steps of 45 degrees of yaw, 20 degrees above it.
// The sponge is painted in three slabs so the turn shows. The light rides with the camera, so the top
// is always brightest and the face on the right always darkest.
// cargo run -p mrly-space --example turntable
use mrly_color::{BLUE, ORANGE, PINK};
use mrly_image::{Image, svg};
use mrly_math::Mode;
use mrly_space::{View, render};
use mrly_three::{carpet, paint};
use std::collections::HashMap;

fn main() {
    let inks = HashMap::from([(1, vec![PINK, ORANGE, BLUE])]);
    let sponge = paint(carpet(3, 1).unwrap(), Some(&inks), Some(Mode::Depth), None).unwrap();
    let frames: Vec<Image> = (0..8)
        .map(|k| {
            let view = View {
                width: 24,
                height: 24,
                yaw: 45.0 * k as f64,
                samples: 2,
                ..View::default()
            };
            render(&sponge, &view).unwrap()
        })
        .collect();
    println!("{}", svg(&frames, 2, 4));
}
