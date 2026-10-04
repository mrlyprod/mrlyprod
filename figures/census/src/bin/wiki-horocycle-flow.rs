use std::collections::BTreeSet;

use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use mrlyrs::num::factor::gcd;

const NAME: &str = "wiki-horocycle-flow";
const HEIGHT: f64 = 0.01;
const SAMPLES: usize = 20000;

fn cusps() -> Vec<(i64, i64)> {
    let floor = 3f64.sqrt() / 2.0;
    let mut out = Vec::new();
    let mut c = 1i64;
    while 1.0 / ((c * c) as f64 * HEIGHT) >= floor {
        let radius = 1.0 / (2.0 * (c * c) as f64 * HEIGHT);
        let reach = ((radius + 0.5) * c as f64).floor() as i64;
        for a in -reach..=reach {
            if gcd(a.unsigned_abs() as u128, c as u128) == 1 {
                out.push((a, c));
            }
        }
        c += 1;
    }
    out
}

fn fold(cusps: &[(i64, i64)]) -> usize {
    let known: BTreeSet<(i64, i64)> = cusps.iter().copied().collect();
    let mut hit = BTreeSet::new();
    for k in 0..SAMPLES {
        let (mut x, mut y) = ((k as f64 + 0.5) / SAMPLES as f64, HEIGHT);
        let mut m = [[1i64, 0], [0, 1]];
        loop {
            let n = x.round();
            x -= n;
            m = [
                [m[0][0] - n as i64 * m[1][0], m[0][1] - n as i64 * m[1][1]],
                m[1],
            ];
            let norm = x * x + y * y;
            if norm >= 1.0 {
                break;
            }
            (x, y) = (-x / norm, y / norm);
            m = [[-m[1][0], -m[1][1]], m[0]];
        }
        let (mut a, mut c) = (m[0][0], m[1][0]);
        if c < 0 {
            (a, c) = (-a, -c);
        }
        let centre = a as f64 / c as f64;
        let radius = 1.0 / (2.0 * (c * c) as f64 * HEIGHT);
        let off = ((x - centre).powi(2) + (y - radius).powi(2)).sqrt() - radius;
        assert!(off.abs() < 1e-6 * radius);
        assert!(known.contains(&(a, c)));
        hit.insert((a, c));
    }
    hit.len()
}

fn main() -> Result<()> {
    let cusps = cusps();
    let hits = fold(&cusps);
    assert_eq!((cusps.len(), hits), (243, 239));
    let pairs: Vec<[i64; 2]> = cusps.iter().map(|&(a, c)| [a, c]).collect();
    save(NAME, &json!({"cusps": pairs}))
}
