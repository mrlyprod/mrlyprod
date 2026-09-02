use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::two::designs;
use mrlynum::classics::{fibonacci, gcd};
use std::collections::HashMap;

const LEVEL: usize = 5;

fn main() -> Result<()> {
    let cell = designs::from_corners(&[vec![0, 0], vec![1, 0], vec![0, 1]], 3, LEVEL, 0, 3)?;
    let side = cell.width();
    let types = cell.types();
    let mut points = Vec::new();
    for row in 0..side {
        for col in 0..side {
            if types.get(&[row, col]) != 0 {
                points.push((row, col));
            }
        }
    }
    assert_eq!(side, 3usize.pow(LEVEL as u32));
    assert_eq!(points.len(), 3usize.pow(LEVEL as u32));

    let mut rays: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for point in &points {
        if *point == (0, 0) {
            continue;
        }
        let step = gcd(point.0 as u128, point.1 as u128) as usize;
        rays.entry((point.0 / step, point.1 / step))
            .or_default()
            .push(*point);
    }
    let ladder = fibonacci(1000);
    let shift = rays[&(1, 3)].len();
    assert_eq!(shift, ladder[LEVEL] - 1);
    assert_eq!(rays[&(3, 1)].len(), shift);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let reach = points
        .iter()
        .map(|point| point.0.max(point.1))
        .max()
        .expect("the gasket holds a point") as f64;
    let step = frame.w / reach;
    let at = |point: &(usize, usize)| {
        (
            frame.x + point.1 as f64 * step,
            frame.y + frame.h - point.0 as f64 * step,
        )
    };
    let origin = at(&(0, 0));
    let mut order: Vec<_> = rays.iter().collect();
    order.sort_by_key(|(ray, on)| (on.len(), **ray));
    for (ray, on) in order {
        let far = on
            .iter()
            .max_by_key(|point| point.0 + point.1)
            .expect("a ray holds a point");
        let mass = on.len() as f64;
        let gold = *ray == (1, 3) || *ray == (3, 1);
        let color = if gold {
            ink::GOLD
        } else {
            ink::fade(ink::BLUE, (0.16 + 0.13 * (mass - 1.0)).min(1.0))
        };
        board.segment(origin, at(far), if gold { 3.0 } else { 1.6 }, color);
    }
    for point in &points {
        let (x, y) = at(point);
        board.disc(x, y, 2.8, ink::FG);
    }
    save("paper-gasket-ray-machine", &board)?;
    Ok(())
}
