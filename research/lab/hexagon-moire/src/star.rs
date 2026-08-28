use crate::lattice::{Family, Rule, FAMILIES};
use crate::sums::{mean, odds};
use mrlymath::six::{geometry::cut, FILL, GRID};
use num_rational::Ratio;

pub struct Slice {
    pub rows: usize,
    pub cols: usize,
    pub types: Vec<u8>,
}

pub fn slice(family: Family, number: usize) -> Slice {
    let hex = cut(&family.cube(number)).expect("a cut cube");
    let (rows, cols) = (hex.height(), hex.width());
    let types = (0..rows * cols)
        .map(|at| hex.cell.types().at(at) as u8)
        .collect();
    Slice { rows, cols, types }
}

impl Slice {
    fn ink(&self) -> Ratio<i64> {
        let filled = self.types.iter().filter(|cell| **cell == FILL).count();
        let inside = self.types.iter().filter(|cell| **cell != GRID).count();
        Ratio::new(filled as i64, inside as i64)
    }

    fn coordinates(&self, number: usize) -> Vec<[f64; 3]> {
        let (n, cols) = (number as i64, self.cols as i64);
        let mut out = vec![[0.0; 3]; self.rows * self.cols];
        for row in 0..self.rows as i64 {
            let z = 2 * row;
            let target = 6 * n - 2 - z;
            let low = 0.max(target - (4 * n - 1));
            let reach = (4 * n - 1).min(target) - low + 1;
            let offset = (cols - reach) / 2;
            for step in 0..reach {
                let x = low + step;
                let scale = 4.0 * n as f64;
                out[(row * cols + offset + step) as usize] = [
                    x as f64 / scale,
                    (target - x) as f64 / scale,
                    z as f64 / scale,
                ];
            }
        }
        out
    }
}

pub fn law(family: Family, number: usize) -> Ratio<i64> {
    let n = number as i64;
    let chi = if (3 * n - 1) / 2 % 2 == 0 { 1 } else { -1 };
    let r = |p: i64, q: i64| Ratio::new(p, q);
    let carpet = r(1, 2) + r(chi, 8) + r(1, 2 * n) - r(chi, 8 * n * n);
    match family {
        Family::Carpet => carpet,
        Family::Net => r(1, 1) - carpet,
        Family::Tree => r(1, 4) + (r(1, 3) - r(chi, 12)) / r(n, 1) + r(1 - chi, 6 * n * n),
        Family::Void => r(1, 4) - r(chi, 4 * n) + r(1, 2 * n * n),
    }
}

pub fn ink_laws(limit: usize) {
    println!("cut ink of every odd n <= {limit} against the closed forms, exact rationals");
    for family in FAMILIES {
        let mut matched = 0;
        let mut centre = 0;
        let mut layers = 0;
        for number in odds(limit) {
            let hex = slice(family, number);
            matched += usize::from(hex.ink() == law(family, number));
            centre += usize::from(hex.types[number * hex.cols + 2 * number - 1] == FILL);
            layers += 1;
        }
        println!(
            "  {}: {matched}/{layers} layers match, centre cell ink in {centre}/{layers}",
            family.name()
        );
    }
}

const HEIGHT: usize = 1200;
const WIDTH: usize = 2399;
const DEEPEST: usize = 111;

fn sample(count: usize, extent: usize) -> Vec<usize> {
    (0..count)
        .map(|slot| ((slot as f64 + 0.5) / count as f64 * extent as f64).floor() as usize)
        .collect()
}

fn raster<T: Copy>(hex: &Slice, cells: &[T]) -> Vec<T> {
    let lines = sample(HEIGHT, hex.rows);
    let slots = sample(WIDTH, hex.cols);
    let mut out = Vec::with_capacity(HEIGHT * WIDTH);
    for line in &lines {
        for slot in &slots {
            out.push(cells[line * hex.cols + slot]);
        }
    }
    out
}

fn masked(values: &[f64], keep: &[bool]) -> f64 {
    let kept: Vec<f64> = values
        .iter()
        .zip(keep)
        .filter(|(_, inside)| **inside)
        .map(|(value, _)| *value)
        .collect();
    mean(&kept)
}

pub fn ghost() {
    println!("carpet cut layers rendered on a {HEIGHT} by {WIDTH} raster, odd n <= {DEEPEST}");
    let layers: Vec<(Vec<bool>, Vec<bool>)> = odds(DEEPEST)
        .map(|number| {
            let hex = slice(Family::Carpet, number);
            let fill = raster(
                &hex,
                &hex.types.iter().map(|c| *c == FILL).collect::<Vec<bool>>(),
            );
            let inside = raster(
                &hex,
                &hex.types.iter().map(|c| *c != GRID).collect::<Vec<bool>>(),
            );
            (fill, inside)
        })
        .collect();
    let mut hexagon = vec![true; HEIGHT * WIDTH];
    for (_, inside) in &layers {
        for (keep, ok) in hexagon.iter_mut().zip(inside) {
            *keep &= ok;
        }
    }
    let deepest = slice(Family::Carpet, DEEPEST);
    let gaps = raster(&deepest, &deepest.coordinates(DEEPEST));
    let near = |cell: usize, width: f64| {
        let [x, y, z] = gaps[cell];
        (x - y).abs() < width || (y - z).abs() < width || (z - x).abs() < width
    };
    let star: Vec<bool> = (0..HEIGHT * WIDTH)
        .map(|cell| hexagon[cell] && near(cell, 0.004))
        .collect();
    let background: Vec<bool> = (0..HEIGHT * WIDTH)
        .map(|cell| hexagon[cell] && !near(cell, 0.02))
        .collect();
    println!(
        "  pixels: hexagon {}  star {}  background {}",
        hexagon.iter().filter(|k| **k).count(),
        star.iter().filter(|k| **k).count(),
        background.iter().filter(|k| **k).count()
    );
    let mut total = vec![0.0f64; HEIGHT * WIDTH];
    for (count, (fill, _)) in layers.iter().enumerate() {
        for (slot, value) in total.iter_mut().zip(fill) {
            *slot += f64::from(*value);
        }
        let stacked = count + 1;
        if [5, 28, 56].contains(&stacked) {
            let field: Vec<f64> = total.iter().map(|value| value / stacked as f64).collect();
            let (star_ink, background_ink) = (masked(&field, &star), masked(&field, &background));
            println!(
                "  layers {stacked:2}: star {star_ink:.5}  background {background_ink:.5}  star minus background {:+.5}",
                star_ink - background_ink
            );
        }
    }
}

pub fn fading_lattice(rule: &Rule) {
    println!("carpet lattice frame, band |x-y| <= 0.01 of the cube side, per-layer excess over the exact ink law");
    let counts = [14usize, 28, 56, 100, 200, 400];
    let mut excesses = Vec::new();
    for number in odds(2 * counts[counts.len() - 1] - 1) {
        let n = number as i64;
        let size = 4 * n;
        let step = 1.0 / size as f64;
        let reach = 2.max((0.01 * size as f64).ceil() as i64 + 2);
        let (mut total, mut inked) = (0.0f64, 0.0f64);
        for index in 0..2 * n {
            let z = 2 * index;
            let target = 6 * n - 2 - z;
            for offset in -reach..=reach {
                let x = target.div_euclid(2) + offset;
                let y = target - x;
                if !(0..size).contains(&x) || !(0..size).contains(&y) {
                    continue;
                }
                let d = (x - y) as f64 * step;
                let weight = ((d + step).min(0.01) - (d - step).max(-0.01)).max(0.0);
                total += weight;
                inked += weight * f64::from(rule.filled(x, y, z));
            }
        }
        let background = law(Family::Carpet, number);
        let background = *background.numer() as f64 / *background.denom() as f64;
        excesses.push(if total > 0.0 {
            inked / total - background
        } else {
            0.0
        });
    }
    let mut scaled = Vec::new();
    for count in counts {
        let excess = mean(&excesses[..count]);
        println!(
            "  L = {count:3}: excess {excess:+.6}  excess * L {:+.4}",
            excess * count as f64
        );
        scaled.push(excess * count as f64);
    }
    println!(
        "  slope of excess * L against ln L: {:+.4} (100 to 200)  {:+.4} (200 to 400)",
        (scaled[4] - scaled[3]) / 2f64.ln(),
        (scaled[5] - scaled[4]) / 2f64.ln()
    );
}
