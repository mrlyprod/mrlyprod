use mrlycore::errors::Result;
use mrlycore::MrlyError;
use mrlyfig::out::root;
use mrlyfig::{ink, save, Board};
use std::path::PathBuf;

const NAME: &str = "paper-first-base-below-a-quarter";
const LOW: usize = 10;
const HIGH: usize = 40;
const SUB: usize = 4;
const CAP: f64 = 6.0e6;
const QUARTER: f64 = 0.25;
const RUNGS: usize = 395;

// LADDER

fn weights(q: usize, a0: usize, nd: u32, m: usize) -> Vec<f64> {
    let w = q.pow(nd);
    let inv = 1.0 / (q as f64 - 1.0);
    let c = 2.0 * a0 as f64 - q as f64 + 1.0;
    let mass: usize = (0..q).filter(|a| *a != a0).sum();
    let lip = 2.0 * std::f64::consts::PI * mass as f64 * inv;
    let slack = lip / (2.0 * w as f64 * m as f64) + 1e-12;
    let step = 1.0 / (w as f64 * m as f64);
    let mut g = vec![0.0; w];
    for cell in 0..w.div_ceil(2) {
        let mut top = 0.0f64;
        for r in 0..m {
            let t = ((cell * m + r) as f64 + 0.5) * step;
            let d = (std::f64::consts::PI * q as f64 * t).sin() / (std::f64::consts::PI * t).sin();
            let e = d * d - 2.0 * d * (std::f64::consts::PI * c * t).cos() + 1.0;
            let v = if e > 0.0 { e.sqrt() * inv } else { 0.0 };
            top = top.max(v);
        }
        let u = (top + slack).min(1.0);
        g[cell] = u;
        g[w - 1 - cell] = u;
    }
    g
}

fn sweep(g: &[f64], q: usize, nd: u32, y: &[f64]) -> Vec<f64> {
    let s = q.pow(nd - 1);
    let p = q.pow(nd - 2);
    (0..s)
        .map(|v| {
            let b = (v % p) * q;
            (0..q).map(|c| g[v * q + c] * y[b + c]).sum()
        })
        .collect()
}

fn power(g: &[f64], q: usize, nd: u32, seed: Option<Vec<f64>>, steps: usize) -> Vec<f64> {
    let s = q.pow(nd - 1);
    let mut y = seed.unwrap_or_else(|| vec![1.0; s]);
    for _ in 0..steps {
        let z = sweep(g, q, nd, &y);
        let top = z.iter().copied().fold(0.0f64, f64::max);
        if top <= 0.0 {
            return y;
        }
        y = z.into_iter().map(|x| x / top).collect();
    }
    y
}

fn bound(g: &[f64], q: usize, nd: u32, y: &[f64]) -> f64 {
    let z = sweep(g, q, nd, y);
    let mu = z
        .iter()
        .zip(y.iter())
        .map(|(a, b)| a / b)
        .fold(0.0f64, f64::max);
    mu.ln() / (q as f64).ln()
}

fn lift(y: &[f64], q: usize) -> Vec<f64> {
    (0..y.len() * q).map(|v| y[v / q]).collect()
}

fn exponent(q: usize, a0: usize) -> f64 {
    let mut nd = 3u32;
    let start = weights(q, a0, nd, SUB);
    let mut y = power(&start, q, nd, None, 300);
    let mut e = bound(&start, q, nd, &y);
    while e >= QUARTER && nd < 5 && (q as f64).powi(nd as i32 + 1) <= CAP {
        let g = weights(q, a0, nd + 1, SUB);
        let z = power(&g, q, nd + 1, Some(lift(&y, q)), 40);
        let step = bound(&g, q, nd + 1, &z);
        let gain = e - step;
        nd += 1;
        y = z;
        e = step;
        if e >= QUARTER && gain <= e - QUARTER {
            break;
        }
    }
    e
}

fn ladder() -> Vec<(usize, f64)> {
    let mut rungs: Vec<(usize, f64)> = Vec::new();
    for q in LOW..=HIGH {
        for a0 in 0..q.div_ceil(2) {
            rungs.push((q, exponent(q, a0)));
        }
    }
    rungs
}

// DATA

fn path() -> PathBuf {
    root()
        .join("files")
        .join("figures")
        .join("data")
        .join(format!("{NAME}.json"))
}

fn write_data(rungs: &[(usize, f64)]) -> Result<PathBuf> {
    let file = path();
    let folder = file.parent().unwrap().to_path_buf();
    std::fs::create_dir_all(&folder)
        .map_err(|e| MrlyError::Value(format!("cannot make {folder:?}: {e}")))?;
    let bases: Vec<String> = rungs.iter().map(|rung| rung.0.to_string()).collect();
    let exponents: Vec<String> = rungs.iter().map(|rung| format!("{}", rung.1)).collect();
    let text = format!(
        "{{\"base\":[{}],\"exponent\":[{}]}}",
        bases.join(","),
        exponents.join(",")
    );
    std::fs::write(&file, text)
        .map_err(|e| MrlyError::Value(format!("cannot write {file:?}: {e}")))?;
    Ok(file)
}

fn field(text: &str, name: &str) -> Vec<f64> {
    let head = format!("\"{name}\":[");
    let Some(start) = text.find(&head) else {
        return Vec::new();
    };
    let body = &text[start + head.len()..];
    let end = body.find(']').unwrap_or(0);
    body[..end]
        .split(',')
        .filter_map(|token| token.parse().ok())
        .collect()
}

fn read_data() -> Result<Vec<(usize, f64)>> {
    let file = path();
    let raw = std::fs::read_to_string(&file)
        .map_err(|e| MrlyError::Value(format!("cannot read {file:?}: {e}; run -- compute")))?;
    let text: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    let bases = field(&text, "base");
    let exponents = field(&text, "exponent");
    Ok(bases
        .iter()
        .zip(exponents.iter())
        .map(|(base, exponent)| (*base as usize, *exponent))
        .collect())
}

// PRESS

fn compute() -> Result<()> {
    let rungs = ladder();
    let below = |q: usize| rungs.iter().filter(|r| r.0 == q && r.1 < QUARTER).count();
    let sets = |q: usize| q.div_ceil(2);
    assert_eq!(rungs.len(), RUNGS, "the ladder has 395 rungs");
    assert!((LOW..21).all(|q| below(q) == 0), "no base under 21 clears");
    assert_eq!(below(21), 1, "base 21 clears at exactly one digit");
    assert!(
        (34..=HIGH).all(|q| below(q) == sets(q)),
        "every base from 34 clears at every digit"
    );
    let gold = rungs.iter().filter(|r| r.1 < QUARTER).count();
    assert_eq!(gold, 163, "163 rungs sit under the quarter");
    let file = write_data(&rungs)?;
    println!(
        "{NAME} {} rungs {gold} under the quarter -> {file:?}",
        RUNGS
    );
    Ok(())
}

fn draw() -> Result<()> {
    let rungs = read_data()?;
    assert_eq!(rungs.len(), RUNGS);
    let lo = rungs.iter().map(|r| r.1).fold(f64::MAX, f64::min);
    let hi = rungs.iter().map(|r| r.1).fold(f64::MIN, f64::max);
    let pad = (hi - lo) * 0.06;
    let (foot, head) = (lo - pad, hi + pad);
    assert!(
        foot < QUARTER && QUARTER < head,
        "the quarter line is inside"
    );

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let span = (HIGH - LOW + 1) as f64;
    let slot = frame.w / span;
    let dash = slot * 0.72;
    let place = |e: f64| frame.y + frame.h * (head - e) / (head - foot);

    let line = place(QUARTER);
    board.rect(
        frame.x,
        line - 1.0,
        frame.w,
        2.0,
        ink::fade(ink::blue(), 0.9),
    );

    for (q, e) in &rungs {
        let x = frame.x + ((q - LOW) as f64 + 0.5) * slot - dash / 2.0;
        let y = place(*e);
        let (color, thick) = if *e < QUARTER {
            (ink::yellow(), 3.8)
        } else {
            (ink::fade(ink::dim(), 0.7), 2.6)
        };
        board.rect(x, y - thick / 2.0, dash, thick, color);
    }
    save(NAME, &board)?;
    Ok(())
}

fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() == Some("compute") {
        return compute();
    }
    draw()
}
