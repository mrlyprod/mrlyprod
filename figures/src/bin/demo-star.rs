use mrlycore::errors::Result;
use mrlyfig::board::Board;
use mrlyfig::ink::Ramp;
use mrlyfig::{ink, save};
use mrlymath::six::star::Star;

const CODE: u128 = 23;
const LAYERS: usize = 28;
const HALF: i64 = 6;
const CELLS: usize = 400;
const SUPER: usize = 5;
const REACH: f64 = 0.712;
const TAIL: f64 = 0.02;

struct Look {
    ink: f64,
    arm: bool,
}

fn place(u: f64, v: f64, n: f64) -> (i64, i64) {
    let side = 4.0 * n;
    let raise = v * 6f64.sqrt() / 3.0;
    let across = 0.5 + (raise + u * 2f64.sqrt()) / 2.0;
    let up = 0.5 - raise;
    if !(0.0..1.0).contains(&across) || !(0.0..1.0).contains(&up) {
        return (-1, -1);
    }
    (
        ((across * side).floor() as i64).min(side as i64 - 1),
        (2 * (up * 2.0 * n).floor() as i64).min(side as i64 - 2),
    )
}

fn read(star: &Star, u: f64, v: f64) -> Option<Look> {
    let deepest = 2 * LAYERS - 1;
    let n = deepest as i64;
    let (x, z) = place(u, v, deepest as f64);
    if x < 0 {
        return None;
    }
    let y = 6 * n - 2 - x - z;
    star.cell(deepest, x, z)?;
    let arm = (x - y).abs() <= HALF || (y - z).abs() <= HALF || (z - x).abs() <= HALF;
    let (mut total, mut reached) = (0.0, 0.0);
    for step in 0..LAYERS {
        let number = 2 * step + 1;
        let (x, z) = place(u, v, number as f64);
        if x < 0 {
            continue;
        }
        if let Some(fill) = star.cell(number, x, z) {
            total += f64::from(u8::from(fill));
            reached += 1.0;
        }
    }
    Some(Look {
        ink: total / reached,
        arm,
    })
}

fn window(values: &[(f64, f64, f64)]) -> (f64, f64) {
    let mut whole: Vec<f64> = values
        .iter()
        .filter(|(_, _, cover)| *cover > 0.99)
        .map(|(value, _, _)| *value)
        .collect();
    whole.sort_by(|a, b| a.partial_cmp(b).expect("a field of finite means"));
    let step = ((whole.len() - 1) as f64 * TAIL) as usize;
    (whole[step], whole[whole.len() - 1 - step])
}

fn main() -> Result<()> {
    let star = Star::new(CODE)?;
    assert_eq!(star.arm(55, 0)?.reduced(), (28, 55));
    assert_eq!(star.hexagon(3)?.reduced(), (7, 9));
    let fine = (CELLS * SUPER) as f64;
    let seats = (SUPER * SUPER) as f64;
    let mut values = vec![(0.0f64, 0.0f64, 0.0f64); CELLS * CELLS];
    for line in 0..CELLS {
        for slot in 0..CELLS {
            let (mut total, mut arms, mut hits) = (0.0, 0.0, 0.0);
            for down in 0..SUPER {
                for right in 0..SUPER {
                    let u = (2.0 * ((slot * SUPER + right) as f64 + 0.5) / fine - 1.0) * REACH;
                    let v = (1.0 - 2.0 * ((line * SUPER + down) as f64 + 0.5) / fine) * REACH;
                    if let Some(look) = read(&star, u, v) {
                        total += look.ink;
                        arms += f64::from(u8::from(look.arm));
                        hits += 1.0;
                    }
                }
            }
            values[line * CELLS + slot] = if hits > 0.0 {
                (total / hits, arms / hits, hits / seats)
            } else {
                (0.0, 0.0, 0.0)
            };
        }
    }
    let drawn = values.iter().filter(|(_, _, cover)| *cover > 0.0).count();
    assert!(drawn * 2 > values.len());
    let starred = values.iter().filter(|(_, arm, _)| *arm > 0.5).count();
    assert!(starred * 5 < drawn && starred * 200 > drawn);
    let (low, high) = window(&values);
    assert!(high - low > 0.01 && low < 0.5 && high > 0.5);

    let bright = Ramp::new(vec![ink::ground(), ink::blue(), ink::yellow()]);
    let quiet = Ramp::tone(ink::ground(), ink::mix(ink::line(), ink::dim(), 0.5));
    let mut board = Board::square();
    let frame = board.frame(0.05);
    let x0 = frame.x.ceil() as usize;
    let y0 = frame.y.ceil() as usize;
    let x1 = ((frame.x + frame.w).floor() as usize).min(board.width);
    let y1 = ((frame.y + frame.h).floor() as usize).min(board.height);
    for py in y0..y1 {
        for px in x0..x1 {
            let u = (px as f64 + 0.5 - frame.x) / frame.w;
            let v = (py as f64 + 0.5 - frame.y) / frame.h;
            let slot = ((u * CELLS as f64) as usize).min(CELLS - 1);
            let line = ((v * CELLS as f64) as usize).min(CELLS - 1);
            let (value, arm, cover) = values[line * CELLS + slot];
            if cover <= 0.0 {
                continue;
            }
            let t = (value - low) / (high - low);
            let shade = ink::mix(quiet.at(t), bright.at(t), arm);
            board.blend(px, py, shade, cover);
        }
    }
    save("demo-star", &board)?;
    Ok(())
}
