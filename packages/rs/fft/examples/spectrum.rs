// The log spectrum of a small disc shows rings around the center.
// cargo run -p mrly-fft --example spectrum
use mrly_fft::log_spectrum;

fn main() {
    let size = 16;
    // A disc of radius 3.5 in the middle of a 16x16 field.
    let field: Vec<f64> = (0..size * size)
        .map(|i| {
            let (r, c) = ((i / size) as f64 - 7.5, (i % size) as f64 - 7.5);
            f64::from(u8::from(r.hypot(c) <= 3.5))
        })
        .collect();

    // Zero frequency sits at the center. Brighter means stronger.
    let spectrum = log_spectrum(&field, size).unwrap();
    let top = spectrum.iter().copied().fold(0.0, f64::max);

    let cell = 12;
    let side = size * cell;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {side} {side}\" shape-rendering=\"crispEdges\">"
    );
    println!("<rect width=\"{side}\" height=\"{side}\" fill=\"#000\"/>");
    for (i, value) in spectrum.iter().enumerate() {
        let level = (value / top * 15.0).round() as usize;
        if level > 0 {
            let hex = format!("{level:x}");
            println!(
                "<rect x=\"{}\" y=\"{}\" width=\"{cell}\" height=\"{cell}\" fill=\"#{hex}{hex}{hex}\"/>",
                i % size * cell,
                i / size * cell
            );
        }
    }
    println!("</svg>");
}
