use mrlyfig::{ink, plot, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::gauss::{Class, Ring, Window};

const REACH: u64 = 20;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.09);
    let window = Window::new(Ring::Gaussian, REACH);
    let census = window.census();
    assert_eq!(census.points, 1681);
    assert_eq!(census.ramified, 4);
    assert_eq!(census.split + census.inert + census.ramified, census.primes);
    let step = frame.w / (2 * REACH) as f64;
    let (cx, cy) = frame.center();
    let place = |a: i64, b: i64| (cx + a as f64 * step, cy - b as f64 * step);
    let mut broken = Vec::new();
    let mut whole = Vec::new();
    for (a, b) in window.points() {
        match window.class(a, b) {
            Class::Split | Class::Ramified => broken.push(place(a, b)),
            Class::Inert => whole.push(place(a, b)),
            _ => {}
        }
    }
    assert_eq!(broken.len(), census.split + census.ramified);
    assert_eq!(whole.len(), census.inert);
    let dot = step * 0.34;
    plot::dots(&mut board, &broken, dot, ink::blue());
    plot::dots(&mut board, &whole, dot, ink::orange());
    save("wiki-gaussian-integers", &board)?;
    Ok(())
}
