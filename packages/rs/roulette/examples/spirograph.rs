// A spirograph: a wheel of radius 3 rolls inside a ring of radius 8, and a pencil fixed to the wheel draws a curve.
// The pencils sit on the cells of a tile. This tile is a plus of five cells, so the picture has four-fold symmetry.
// The track closes after b laps, where ring / wheel = a / b in lowest terms; here a = 8 and b = 3, so every curve has 8 lobes.
// cargo run -p mrly-roulette --example spirograph
use mrly_roulette::{frame, pencils, trace, track};

fn main() {
    let (w, h) = (3, 3);
    let tile = [0, 1, 0, 1, 1, 1, 0, 1, 0];
    let list = pencils(&tile, w, h, "fill", 0.9, 0.0, 1).unwrap();
    let path = track("in", 8, 3, 0, 1).unwrap();
    let samples = 160;
    let points = trace(&path, &list, samples).unwrap();
    let [x0, y0, x1, y1] = frame(&path, &list);
    let side = 320.0;
    let scale = side / (x1 - x0).max(y1 - y0);
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {side} {side}\" width=\"{side}\" height=\"{side}\">"
    );
    println!("<rect width=\"{side}\" height=\"{side}\" fill=\"#fff\"/>");
    for (k, curve) in points.chunks(2 * samples).enumerate() {
        let d: Vec<String> = curve
            .chunks(2)
            .map(|p| {
                format!(
                    "{:.1},{:.1}",
                    (p[0] as f64 - x0) * scale,
                    side - (p[1] as f64 - y0) * scale
                )
            })
            .collect();
        println!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"hsl({},70%,45%)\" stroke-width=\"1\"/>",
            d.join(" "),
            k * 70
        );
    }
    println!("</svg>");
}
