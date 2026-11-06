// The renderer uses only integers, adds, multiplies, divides and square roots, which every platform rounds
// the same way, and its own sine and cosine instead of the system's. So a view always gives the same pixels.
// Each line is the SHA-256 of one picture's bytes: if this file changes on a new machine, something drifted.
// Angles that differ by whole turns land on the same pixels.
// cargo run -p mrly-space --example fingerprints
use mrly_name::sha::short;
use mrly_space::{View, render};
use mrly_three::carpet;

// name, yaw, pitch, roll, fov, samples
const VIEWS: [(&str, f64, f64, f64, f64, usize); 7] = [
    ("default", 30.0, 20.0, 0.0, 0.0, 4),
    ("yaw + 360", 390.0, 20.0, 0.0, 0.0, 4),
    ("yaw - 720", -690.0, 20.0, 0.0, 0.0, 4),
    ("odd angles", 17.3, -41.9, 7.7, 0.0, 4),
    ("perspective", 30.0, 20.0, 0.0, 50.0, 4),
    ("one sample", 30.0, 20.0, 0.0, 0.0, 1),
    ("8 by 8 samples", 30.0, 20.0, 0.0, 0.0, 8),
];

fn main() {
    let sponge = carpet(3, 2).unwrap();
    for (name, yaw, pitch, roll, fov, samples) in VIEWS {
        let view = View {
            width: 64,
            height: 64,
            yaw,
            pitch,
            roll,
            fov,
            samples,
            ..View::default()
        };
        let image = render(&sponge, &view).unwrap();
        println!("{name:<15} {}", short(image.colors.as_flattened()));
    }
}
