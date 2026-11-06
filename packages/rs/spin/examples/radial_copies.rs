// Spin copies of a picture about its centre and blend them. The source is one disc off to the right.
// Six copies, each a sixth of a turn further round, are added with Blend::Sum: 1 is one disc, 2 is where two discs overlap.
// cargo run -p mrly-spin --example radial_copies
use mrly_spin::{Blend, radial};

fn main() {
    let size = 64;
    let c = size as f64 / 2.0;
    let mut data = vec![0.0f32; size * size];
    for row in 0..size {
        for col in 0..size {
            let dx = col as f64 + 0.5 - (c + 0.5 * c);
            let dy = row as f64 + 0.5 - c;
            if dx.hypot(dy) < 0.3 * c {
                data[row * size + col] = 1.0;
            }
        }
    }
    let out = 48;
    let field = radial(&data, size, out, 6, 1.0 / 6.0, Blend::Sum, 2).unwrap();
    println!("max {}", field.iter().cloned().fold(0.0, f32::max));
    for row in field.chunks(out) {
        let line: String = row
            .iter()
            .map(|&v| match v {
                v if v < 0.25 => "  ",
                v if v < 1.25 => "++",
                _ => "##",
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
