use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::sumset::Pair;

const LEVEL: u32 = 6;
const SAMPLES: usize = 3072;

// THE MEASURE

fn cantor(mut y: f64) -> f64 {
    let mut out = 0.0;
    let mut w = 1.0;
    for _ in 0..40 {
        if y < 0.0 {
            return out;
        }
        if y >= 0.5 {
            return out + w;
        }
        y *= 3.0;
        w /= 2.0;
        if y >= 0.5 {
            out += w;
            y -= 1.0;
        }
    }
    out + w / 2.0
}

fn energy(tau: f64, level: u32) -> f64 {
    let t = tau * 3f64.powi(level as i32);
    let digits = (t.log(4.0).ceil() as i32).max(0) + 7;
    let mut atoms = vec![0.0f64];
    for l in 1..=digits {
        let step = t * 4f64.powi(-l);
        let more: Vec<f64> = atoms.iter().map(|c| c + step).collect();
        atoms.extend(more);
    }
    let tail = t * 4f64.powi(-digits) / 6.0;
    let weight = 1.0 / atoms.len() as f64;
    let mut p = vec![0.0f64; (t / 3.0).ceil() as usize + 3];
    for c in atoms {
        let c = c + tail;
        let n = c.floor();
        let g = cantor(n + 1.0 - c);
        p[n as usize] += weight * g;
        p[n as usize + 1] += weight * (1.0 - g);
    }
    for j in 0..level {
        let s = 3usize.pow(j);
        let mut q = vec![0.0f64; p.len() + s];
        for (i, v) in p.iter().enumerate() {
            q[i] += v / 2.0;
            q[i + s] += v / 2.0;
        }
        p = q;
    }
    3f64.powi(level as i32) * p.iter().map(|v| v * v).sum::<f64>()
}

fn chain(k: u32) -> Pair {
    let p = 3u64.pow(k);
    let mut m = 0;
    while 4u64.pow(m) < p {
        m += 1;
    }
    Pair { three: k, four: m }
}

// THE CHECKS

fn lattice(pair: Pair) -> f64 {
    3f64.powi(pair.three as i32) * pair.energy() as f64 / 4f64.powi((pair.three + pair.four) as i32)
}

fn main() -> Result<()> {
    let taus: Vec<f64> = (0..=SAMPLES)
        .map(|i| 4f64.powf(i as f64 / SAMPLES as f64))
        .collect();
    let curves: Vec<Vec<f64>> = (0..=LEVEL)
        .map(|k| taus.iter().map(|&t| energy(t, k)).collect())
        .collect();
    assert_eq!(curves[0][0], 1.0);
    assert_eq!(curves[0][SAMPLES], 0.5);
    for k in 0..LEVEL as usize {
        assert!((0..=SAMPLES).all(|i| curves[k + 1][i] >= curves[k][i] - 1e-12));
    }
    for hs in &curves {
        let area: f64 = (0..SAMPLES)
            .map(|i| (taus[i + 1] - taus[i]) * (hs[i] + hs[i + 1]) / 2.0)
            .sum();
        assert!(area > 2.0 && area < 4.0);
    }

    let orbit: Vec<(f64, f64)> = (0..=LEVEL)
        .map(|k| {
            if k == 0 {
                return (1.0, energy(1.0, 0));
            }
            let pair = chain(k);
            let tau = pair.scale();
            assert!((1.0..4.0).contains(&tau));
            let h = energy(tau, k);
            assert!((h - lattice(pair)).abs() < 1e-12 * h);
            (tau, h)
        })
        .collect();
    for k in 0..LEVEL as usize {
        let step = orbit[k + 1].1 / orbit[k].1;
        assert!((9.0 / 16.0..=4.5).contains(&step));
    }
    let least = chain(6);
    assert_eq!(least.four, 5);
    assert!((least.ratio(least.energy()) - 858849.0 / 524288.0).abs() < 1e-15);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let lo = 0.5 * 0.92;
    let hi = curves[LEVEL as usize]
        .iter()
        .copied()
        .fold(0.0f64, f64::max)
        * 1.03;
    let at = |tau: f64, h: f64| {
        (
            frame.x + frame.w * tau.log(4.0),
            frame.y + frame.h * (hi - h) / (hi - lo),
        )
    };
    let foot = frame.y + frame.h;
    for &(tau, h) in &orbit {
        let (x, y) = at(tau, h);
        board.segment((x, foot), (x, y), 1.4, ink::line());
    }
    board.segment((frame.x, foot), (frame.x + frame.w, foot), 1.4, ink::line());
    for (k, hs) in curves.iter().enumerate() {
        let pts: Vec<(f64, f64)> = taus.iter().zip(hs).map(|(&t, &h)| at(t, h)).collect();
        let t = k as f64 / LEVEL as f64;
        let color = ink::mix(ink::dim(), ink::blue(), t * t);
        board.polyline(&pts, 1.3 + 1.3 * t, color);
    }
    for &(tau, h) in &orbit {
        let (x, y) = at(tau, h);
        board.disc(x, y, 9.0, ink::ground());
        board.disc(x, y, 6.5, ink::orange());
    }
    save("paper-erdos-125-upper-density", &board)?;
    Ok(())
}
