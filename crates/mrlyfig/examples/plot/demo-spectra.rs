use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlymath::{six, three};
use mrlynum::graph::{census, largest_component};
use mrlynum::spectrum::{laplacian_spectrum, spectral_fit, spectral_points};

const WINDOW: f64 = 0.10;

fn main() -> Result<()> {
    let cell = six::cut(&three::create(23, 3, 2, 2)?)?;
    let whole = six::graph::slice_core_graph(&cell)?;
    let pieces = census(&whole).components;
    let network = largest_component(&whole);
    let values = laplacian_spectrum(&network, true)?;
    let points = spectral_points(&values);
    let (intercept, slope, fitted) = spectral_fit(&values, WINDOW).expect("the low window fits");
    assert_eq!((network.nodes.len(), network.branches.len(), pieces), (306, 378, 1));
    assert_eq!((points.len(), fitted), (286, 30));
    assert!((2.0 * slope - 1.253284).abs() < 1e-5);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let xs: Vec<f64> = points.iter().map(|p| p.0.ln()).collect();
    let ys: Vec<f64> = points.iter().map(|p| p.1.ln()).collect();
    let (left, right) = (xs[0], *xs.last().unwrap());
    let (foot, roof) = (ys[0], 0.0);
    let at = |x: f64, y: f64| {
        (
            frame.x + frame.w * (x - left) / (right - left),
            frame.y + frame.h * (1.0 - (y - foot) / (roof - foot)),
        )
    };
    let ray = |x: f64| intercept + slope * x;
    let reach = |from: f64, to: f64| {
        let low = ((foot - intercept) / slope).max(from);
        let high = ((roof - intercept) / slope).min(to);
        (at(low, ray(low)), at(high, ray(high)))
    };

    let (edge, _) = at(xs[fitted - 1], roof);
    board.rect(frame.x, frame.y, edge - frame.x, frame.h, ink::panel());
    plot::axis(&mut board, frame, ink::line());
    let mut stair = vec![at(xs[0], ys[0])];
    for index in 1..points.len() {
        stair.push(at(xs[index], ys[index - 1]));
        stair.push(at(xs[index], ys[index]));
    }
    board.polyline(&stair, 4.0, ink::blue());
    let (a, b) = reach(left, right);
    board.segment(a, b, 2.5, ink::fade(ink::yellow(), 0.6));
    let (a, b) = reach(left, xs[fitted - 1]);
    board.segment(a, b, 6.0, ink::yellow());
    save("demo-spectra", &board)?;
    Ok(())
}
