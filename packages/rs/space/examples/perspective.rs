// Orthographic against perspective. Orthographic keeps parallel lines parallel, so the tunnels look flat.
// Perspective (fov 60) puts the eye close: near squares grow, far ones shrink, you see into the tunnels,
// and the side face that orthographic shows slips out of sight, because the eye is no longer beside it.
// cargo run -p mrly-space --example perspective
use mrly_image::svg;
use mrly_space::{View, render};
use mrly_three::carpet;

fn main() {
    let sponge = carpet(3, 2).unwrap();
    let view = View {
        width: 48,
        height: 48,
        yaw: 12.0,
        pitch: 9.0,
        samples: 2,
        ..View::default()
    };
    let flat = render(&sponge, &view).unwrap();
    let deep = render(&sponge, &View { fov: 60.0, ..view }).unwrap();
    println!("{}", svg(&[flat, deep], 4, 4));
}
