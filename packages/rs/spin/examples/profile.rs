// A radial profile is the average of a picture around each circle centred on it.
// The picture is a filled disc of radius 20 on a 64 by 64 raster, so the average is 1 inside radius 20 and 0 outside.
// Adding up profile times circumference (mass) gives back the picture's pixel total (1264, close to pi * 20^2 = 1257).
// cargo run -p mrly-spin --example profile
use mrly_spin::{mass, mass_within, profile, ring};

fn main() {
    let size = 64;
    let c = size as f64 / 2.0;
    let mut data = vec![0.0f32; size * size];
    for row in 0..size {
        for col in 0..size {
            let (dx, dy) = (col as f64 + 0.5 - c, row as f64 + 0.5 - c);
            if dx.hypot(dy) < 20.0 {
                data[row * size + col] = 1.0;
            }
        }
    }
    for r in [0.0, 5.0, 10.0, 15.0, 19.5, 20.5, 25.0, 40.0] {
        let v = ring(&data, size, r).unwrap();
        println!("average at radius {r:>4}: {v:.3}");
    }
    let line = profile(&data, size, 400).unwrap();
    println!("pixel total          {}", data.iter().sum::<f32>());
    println!("mass of the profile  {:.1}", mass(&line, size));
    println!(
        "mass within radius 20 {:.1}",
        mass_within(&line, size, 20.0)
    );
}
