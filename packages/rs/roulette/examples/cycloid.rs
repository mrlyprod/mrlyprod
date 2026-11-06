// A wheel rolls along a line and a pencil fixed to it draws a roulette. The pencil's distance from the hub decides the curve.
// Inside the rim (0.5) the curve is a smooth wave, on the rim (1.0) it is a cycloid with sharp cusps, outside the rim (1.5) it loops.
// cargo run -p mrly-roulette --example cycloid
use mrly_roulette::{Kind, Pencil, frame, trace, track};

fn main() {
    let wheel = 10;
    let path = track("line", 1, wheel, 0, 2).unwrap();
    let list: Vec<Pencil> = [0.5, 1.0, 1.5]
        .iter()
        .map(|&reach| Pencil {
            x: 0.0,
            y: -reach,
            seat: (0, 0),
            kind: Kind::Fill,
        })
        .collect();
    let samples = 160;
    let points = trace(&path, &list, samples).unwrap();
    let [x0, y0, x1, y1] = frame(&path, &list);
    let width = 400.0;
    let scale = width / (x1 - x0);
    let height = (y1 - y0) * scale;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height:.0}\" width=\"{width}\" height=\"{height:.0}\">"
    );
    println!("<rect width=\"{width}\" height=\"{height:.0}\" fill=\"#fff\"/>");
    println!(
        "<line x1=\"0\" y1=\"{0:.1}\" x2=\"{width}\" y2=\"{0:.1}\" stroke=\"#bbb\"/>",
        height + y0 * scale
    );
    for (k, curve) in points.chunks(2 * samples).enumerate() {
        let d: Vec<String> = curve
            .chunks(2)
            .map(|p| {
                format!(
                    "{:.1},{:.1}",
                    (p[0] as f64 - x0) * scale,
                    height - (p[1] as f64 - y0) * scale
                )
            })
            .collect();
        println!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"hsl({},70%,45%)\" stroke-width=\"1.5\"/>",
            d.join(" "),
            [210, 20, 130][k]
        );
    }
    println!("</svg>");
}
