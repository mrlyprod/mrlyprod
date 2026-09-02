use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};

const RE_LO: f64 = -1.0;
const RE_HI: f64 = 1.0;
const IM_REACH: f64 = 40.0;

fn cexp(z: (f64, f64)) -> (f64, f64) {
    let e = z.0.exp();
    (e * z.1.cos(), e * z.1.sin())
}

fn cdiv(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    let d = b.0 * b.0 + b.1 * b.1;
    ((a.0 * b.0 + a.1 * b.1) / d, (a.1 * b.0 - a.0 * b.1) / d)
}

fn power(base: f64, s: (f64, f64)) -> (f64, f64) {
    let l = base.ln();
    cexp((-s.0 * l, -s.1 * l))
}

fn residual(s: (f64, f64)) -> (f64, f64) {
    let a = power(3.0, s);
    let b = power(5.0, s);
    (a.0 + b.0 - 1.0, a.1 + b.1)
}

fn slope(s: (f64, f64)) -> (f64, f64) {
    let a = power(3.0, s);
    let b = power(5.0, s);
    let (l3, l5) = (3f64.ln(), 5f64.ln());
    (-l3 * a.0 - l5 * b.0, -l3 * a.1 - l5 * b.1)
}

fn newton(seed: (f64, f64)) -> Option<(f64, f64)> {
    let mut s = seed;
    for _ in 0..80 {
        if s.0.abs() > 6.0 || s.1.abs() > 200.0 {
            return None;
        }
        let d = slope(s);
        if d.0.hypot(d.1) < 1e-14 {
            return None;
        }
        let step = cdiv(residual(s), d);
        s = (s.0 - step.0, s.1 - step.1);
    }
    let r = residual(s);
    if r.0.hypot(r.1) > 1e-10 {
        return None;
    }
    if s.0 < RE_LO || s.0 > RE_HI || s.1.abs() > IM_REACH {
        return None;
    }
    Some(s)
}

fn control_poles() -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for i in 0..=80 {
        for j in 0..=320 {
            let seed = (
                RE_LO + (RE_HI - RE_LO) * i as f64 / 80.0,
                -IM_REACH + 2.0 * IM_REACH * j as f64 / 320.0,
            );
            if let Some(root) = newton(seed) {
                if !out
                    .iter()
                    .any(|p| (p.0 - root.0).hypot(p.1 - root.1) < 1e-6)
                {
                    out.push(root);
                }
            }
        }
    }
    out
}

fn main() -> Result<()> {
    let real = 2f64.ln() / 3f64.ln();
    let omega = 2.0 * std::f64::consts::PI / 3f64.ln();
    let lattice: Vec<(f64, f64)> = (-6..=6).map(|m| (real, m as f64 * omega)).collect();
    let control = control_poles();
    assert_eq!(lattice.len(), 13);
    assert_eq!(control.len(), 21);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    plot::axis(&mut board, frame, ink::LINE);
    let at = |re: f64, im: f64| {
        (
            frame.x + frame.w * (re - RE_LO) / (RE_HI - RE_LO),
            frame.y + frame.h * (1.0 - (im + IM_REACH) / (2.0 * IM_REACH)),
        )
    };
    board.segment(at(0.0, -IM_REACH), at(0.0, IM_REACH), 1.6, ink::LINE);
    board.segment(at(RE_LO, 0.0), at(RE_HI, 0.0), 1.6, ink::LINE);
    board.segment(
        at(real, -IM_REACH),
        at(real, IM_REACH),
        1.8,
        ink::fade(ink::BLUE, 0.35),
    );
    for pole in &control {
        let (x, y) = at(pole.0, pole.1);
        board.disc(x, y, 7.0, ink::ORANGE);
    }
    for pole in &lattice {
        let (x, y) = at(pole.0, pole.1);
        board.disc(x, y, 9.0, ink::BLUE);
    }
    save("research-dimensions", &board)?;
    Ok(())
}
