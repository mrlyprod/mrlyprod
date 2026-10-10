use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use mrlyrs::math::six::star::Star;

const NAME: &str = "demo-star";
const CODE: u128 = 23;
const LAYERS: usize = 28;
const HALF: i64 = 6;
const CELLS: usize = 400;
const SUPER: usize = 5;
const REACH: f64 = 0.712;
const TAIL: f64 = 0.02;
const STEPS: f64 = 65535.0;
const DIGITS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

struct Look {
    ink: f64,
    arm: bool,
}

#[derive(Clone, Copy, Default)]
struct Cell {
    value: f64,
    arms: usize,
    hits: usize,
}

// READ

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

fn window(cells: &[Cell]) -> (f64, f64) {
    let mut whole: Vec<f64> = cells
        .iter()
        .filter(|cell| cell.hits == SUPER * SUPER)
        .map(|cell| cell.value)
        .collect();
    whole.sort_by(|a, b| a.partial_cmp(b).expect("a field of finite means"));
    let step = ((whole.len() - 1) as f64 * TAIL) as usize;
    (whole[step], whole[whole.len() - 1 - step])
}

// PACK

fn base64(bytes: &[u8]) -> String {
    let mut text = String::new();
    for chunk in bytes.chunks(3) {
        let word = chunk.iter().enumerate().fold(0u32, |word, (i, &byte)| {
            word | u32::from(byte) << (16 - 8 * i)
        });
        for k in 0..=chunk.len() {
            text.push(char::from(DIGITS[(word >> (18 - 6 * k) & 63) as usize]));
        }
    }
    text
}

fn runs(codes: &[usize]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for &code in codes {
        if out.len() >= 2 && out[out.len() - 2] == code {
            let last = out.len() - 1;
            out[last] += 1;
        } else {
            out.push(code);
            out.push(1);
        }
    }
    out
}

fn main() -> Result<()> {
    let star = Star::new(CODE)?;
    assert_eq!(star.arm(55, 0)?.reduced(), (28, 55));
    assert_eq!(star.hexagon(3)?.reduced(), (7, 9));
    let fine = (CELLS * SUPER) as f64;
    let mut cells = vec![Cell::default(); CELLS * CELLS];
    for line in 0..CELLS {
        for slot in 0..CELLS {
            let (mut total, mut arms, mut hits) = (0.0, 0, 0);
            for down in 0..SUPER {
                for right in 0..SUPER {
                    let u = (2.0 * ((slot * SUPER + right) as f64 + 0.5) / fine - 1.0) * REACH;
                    let v = (1.0 - 2.0 * ((line * SUPER + down) as f64 + 0.5) / fine) * REACH;
                    if let Some(look) = read(&star, u, v) {
                        total += look.ink;
                        arms += usize::from(look.arm);
                        hits += 1;
                    }
                }
            }
            if hits > 0 {
                cells[line * CELLS + slot] = Cell {
                    value: total / hits as f64,
                    arms,
                    hits,
                };
            }
        }
    }
    let drawn = cells.iter().filter(|cell| cell.hits > 0).count();
    assert!(drawn * 2 > cells.len());
    let starred = cells
        .iter()
        .filter(|cell| cell.arms * 2 > cell.hits)
        .count();
    assert!(starred * 5 < drawn && starred * 200 > drawn);
    let (low, high) = window(&cells);
    assert!(high - low > 0.01 && low < 0.5 && high > 0.5);

    let mut tone = Vec::with_capacity(2 * drawn);
    for cell in cells.iter().filter(|cell| cell.hits > 0) {
        let t = ((cell.value - low) / (high - low)).clamp(0.0, 1.0);
        tone.extend(((t * STEPS).round() as u16).to_le_bytes());
    }
    let codes: Vec<usize> = cells
        .iter()
        .map(|cell| cell.hits * (SUPER * SUPER + 1) + cell.arms)
        .collect();
    save(
        NAME,
        &json!({
            "size": CELLS,
            "seats": SUPER * SUPER,
            "tone": base64(&tone),
            "mask": runs(&codes),
        }),
    )
}
