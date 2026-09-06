use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::two;
use mrlynum::spiral::{snail, Growth};

const TOP: u64 = 300;
const BASE: u64 = 2;
const CODE: u128 = 7;
const MARGIN: f64 = 0.07;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.area(MARGIN);
    let shell = snail(BASE, TOP, Growth::Every);
    assert_eq!(shell.tiles.len(), 300);
    assert_eq!(shell.primes, 62);
    assert_eq!(shell.levels, vec![1, 2, 4, 8, 16, 32, 64, 128, 45]);
    assert_eq!(shell.area, 5_345_865);
    let wide = (shell.high.0 - shell.low.0) as f64;
    let tall = (shell.high.1 - shell.low.1) as f64;
    assert_eq!((wide, tall), (4608.0, 4352.0));
    let unit = (frame.w / wide).min(frame.h / tall);
    let left = frame.x + (frame.w - wide * unit) / 2.0;
    let foot = frame.y + (frame.h + tall * unit) / 2.0;
    let at = |x: f64, y: f64| {
        (
            left + (x - shell.low.0 as f64) * unit,
            foot - (y - shell.low.1 as f64) * unit,
        )
    };
    let path: Vec<(f64, f64)> = shell
        .tiles
        .iter()
        .map(|tile| {
            let half = tile.side as f64 / 2.0;
            at(tile.x as f64 + half, tile.y as f64 + half)
        })
        .collect();
    board.polyline(&path, 1.6, ink::LINE);
    let mut art = Vec::new();
    for level in 0..shell.levels.len() {
        art.push(match level {
            0 => None,
            _ => Some(two::create(CODE, BASE as usize, level, 0, 2)?),
        });
    }
    let mut drawn = 0usize;
    for tile in shell.tiles.iter().filter(|tile| tile.level > 0) {
        let cell = art[tile.level as usize].as_ref().expect("a grown level");
        let types = cell.types();
        let side = cell.width();
        let tone = if tile.prime { ink::BLUE } else { ink::DIM };
        for row in 0..side {
            for col in 0..side {
                if types.get(&[row, col]) == 0 {
                    continue;
                }
                let (x, y) = at(
                    tile.x as f64 + col as f64,
                    tile.y as f64 + (side - row) as f64,
                );
                board.rect(x, y, unit, unit, tone);
                drawn += 1;
            }
        }
    }
    assert_eq!(drawn, 631_167);
    save("demo-snail", &board)?;
    Ok(())
}
