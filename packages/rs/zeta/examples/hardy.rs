// The Hardy Z function is real on the critical line and has the same zeros as zeta.
// Plotted from t = 0 to 50: the curve crosses the axis at each zero, marked with a dot.
// cargo run -p mrly-zeta --example hardy
use mrly_zeta::Line;

fn main() {
    let line = Line::new();
    let (width, height, top) = (50.0, 6.0, 50);
    let y = |z: f64| height / 2.0 - z;
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\">");
    println!(
        "<path d=\"M0 {0}H{width}\" stroke=\"#999\" stroke-width=\"0.05\"/>",
        height / 2.0
    );
    let points: Vec<String> = (0..=top * 8)
        .map(|k| {
            let t = k as f64 / 8.0;
            format!("{t} {:.2}", y(line.z(t)))
        })
        .collect();
    println!(
        "<polyline points=\"{}\" fill=\"none\" stroke=\"#000\" stroke-width=\"0.08\"/>",
        points.join(" ")
    );
    for t in line.zeros(10) {
        println!(
            "<circle cx=\"{t:.3}\" cy=\"{}\" r=\"0.18\" fill=\"#c00\"/>",
            height / 2.0
        );
    }
    println!("</svg>");
}
