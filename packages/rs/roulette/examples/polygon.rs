// A wheel rolls around the outside of a pentagon. Along each side it rolls like on a line; at each corner it pivots through the exterior angle.
// The pentagon is the grey outline. Two pencils on the wheel (at 0.5 and 0.9 of its radius) draw the curves.
// cargo run -p mrly-roulette --example polygon
use mrly_roulette::{Kind, Pencil, frame, trace, track};

fn main() {
    let path = track("polyout", 14, 4, 5, 1).unwrap();
    let list: Vec<Pencil> = [0.5, 0.9]
        .iter()
        .map(|&reach| Pencil {
            x: reach,
            y: 0.0,
            seat: (0, 0),
            kind: Kind::Fill,
        })
        .collect();
    let samples = 250;
    let points = trace(&path, &list, samples).unwrap();
    let [x0, y0, x1, y1] = frame(&path, &list);
    let side = 360.0;
    let scale = side / (x1 - x0).max(y1 - y0);
    let at = |x: f64, y: f64| format!("{:.1},{:.1}", (x - x0) * scale, side - (y - y0) * scale);
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {side} {side}\" width=\"{side}\" height=\"{side}\">"
    );
    println!("<rect width=\"{side}\" height=\"{side}\" fill=\"#fff\"/>");
    let outline: Vec<String> = path.outline.iter().map(|p| at(p.0, p.1)).collect();
    println!(
        "<polygon points=\"{}\" fill=\"#eee\" stroke=\"#999\"/>",
        outline.join(" ")
    );
    for (k, curve) in points.chunks(2 * samples).enumerate() {
        let d: Vec<String> = curve
            .chunks(2)
            .map(|p| at(p[0] as f64, p[1] as f64))
            .collect();
        println!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"hsl({},70%,45%)\" stroke-width=\"1.5\"/>",
            d.join(" "),
            [210, 20][k]
        );
    }
    println!("</svg>");
}
