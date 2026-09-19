use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlynum::gauss::{Class, Ring, Window};

const REACH: u64 = 40;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.09);
    let window = Window::new(Ring::Gaussian, REACH);
    let census = window.census();
    assert_eq!(census.points, 6561);
    assert_eq!(census.ramified, 4);
    assert_eq!(census.split + census.inert + census.ramified, census.primes);
    let step = frame.w / (2 * REACH) as f64;
    let (cx, cy) = frame.center();
    let place = |a: i64, b: i64| (cx + a as f64 * step, cy - b as f64 * step);
    let mut split = Vec::new();
    let mut inert = Vec::new();
    let mut ramified = Vec::new();
    for (a, b) in window.points() {
        match window.class(a, b) {
            Class::Split => split.push(place(a, b)),
            Class::Inert => inert.push(place(a, b)),
            Class::Ramified => ramified.push(place(a, b)),
            _ => {}
        }
    }
    assert_eq!(split.len(), census.split);
    assert_eq!(inert.len(), census.inert);
    assert_eq!(ramified.len(), census.ramified);
    let dot = step * 0.36;
    plot::dots(&mut board, &split, dot, ink::blue());
    plot::dots(&mut board, &inert, dot, ink::orange());
    plot::dots(&mut board, &ramified, dot, ink::yellow());
    save("demo-gaussian", &board)?;
    Ok(())
}
