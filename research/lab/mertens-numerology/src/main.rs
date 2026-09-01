use std::f64::consts::PI;

// SETUP

const GAMMA: f64 = 0.577_215_664_901_532_9;
const GUARD: f64 = 1e-12;
const BASES: [u64; 10] = [
    1_000,
    2_000,
    3_000,
    3_689,
    3_690,
    5_000,
    10_000,
    100_000,
    1_000_000,
    1_000_000_000,
];
const SWEEP_LO: u64 = 3_690;
const SWEEP_HI: u64 = 100_000;
const CORO: [u64; 3] = [10_000, 100_000, 1_000_000];

// KERNEL CONSTANTS

fn harmonic_bound(n: f64) -> f64 {
    n.ln() + GAMMA + 0.5 / n
}

fn phi(q: f64) -> f64 {
    let n = ((q - 2.0) / 2.0).ceil();
    (4.0 / PI) * q + (2.0 * q / PI) * harmonic_bound(n) + (1.0 - 2.0 / PI) * (q - 2.0) + 0.727
}

fn pb(q: f64, m: f64) -> f64 {
    m.sqrt() + phi(q) / q
}

fn alpha(q: f64, m: f64) -> f64 {
    (q - m).ln() / q.ln()
}

fn c_exp(q: f64, m: f64) -> f64 {
    pb(q, m).ln() / q.ln()
}

fn delta(q: f64, m: f64) -> f64 {
    let a = alpha(q, m);
    (a - 0.75 - c_exp(q, m)) / a
}

fn closes(q: f64, m: f64) -> bool {
    0.75 + c_exp(q, m) < alpha(q, m)
}

fn gap(q: f64, m: f64) -> f64 {
    (q - m) * q.powf(-0.75) - pb(q, m)
}

// SAFE ROUNDING

fn ceil_units(x: f64, digits: u32) -> i64 {
    let s = 10f64.powi(digits as i32);
    ((x + GUARD) * s).ceil() as i64
}

fn floor_units(x: f64, digits: u32) -> i64 {
    let s = 10f64.powi(digits as i32);
    ((x - GUARD) * s).floor() as i64
}

fn fixed(units: i64, digits: u32) -> String {
    let s = 10i64.pow(digits);
    let sign = if units < 0 { "-" } else { "" };
    let a = units.abs();
    format!(
        "{}{}.{:0width$}",
        sign,
        a / s,
        a % s,
        width = digits as usize
    )
}

fn label(q: u64) -> String {
    let mut p = q;
    let mut e = 0u32;
    while p % 10 == 0 {
        p /= 10;
        e += 1;
    }
    if p == 1 && e >= 4 {
        format!("10^{e}")
    } else {
        q.to_string()
    }
}

// TABLE ROWS

struct Row {
    q: u64,
    m: u64,
    alpha: i64,
    c: i64,
    delta: i64,
    closes: bool,
}

fn row(q: u64, m: u64) -> Row {
    let (qf, mf) = (q as f64, m as f64);
    Row {
        q,
        m,
        alpha: floor_units(alpha(qf, mf), 6),
        c: ceil_units(c_exp(qf, mf), 5),
        delta: floor_units(delta(qf, mf), 5),
        closes: closes(qf, mf),
    }
}

fn render(r: &Row) -> String {
    format!(
        "| {} | {} | {} | {} | {} |",
        label(r.q),
        fixed(r.alpha, 6),
        fixed(r.c, 5),
        fixed(r.delta, 5),
        if r.closes { "yes" } else { "no" }
    )
}

fn render_coro(r: &Row) -> String {
    format!(
        "| {} | {} | {} | {} | {} |",
        label(r.q),
        r.m,
        fixed(r.alpha, 6),
        fixed(r.c, 5),
        fixed(r.delta, 5)
    )
}

// CHECKS

fn gap_sweep(lo: u64, hi: u64, m: u64) -> (u64, f64, f64) {
    let mf = m as f64;
    let mut prev = gap(lo as f64, mf);
    let mut worst = f64::INFINITY;
    let mut arg = lo;
    let mut min_gap = prev;
    for q in (lo + 1)..=hi {
        let g = gap(q as f64, mf);
        let step = g - prev;
        if step < worst {
            worst = step;
            arg = q - 1;
        }
        if g < min_gap {
            min_gap = g;
        }
        prev = g;
    }
    (arg, worst, min_gap)
}

fn max_excluded(q: u64) -> u64 {
    let qf = q as f64;
    let mut m = 0u64;
    while gap(qf, (m + 1) as f64) > 0.0 {
        m += 1;
    }
    m
}

// MAIN

fn main() {
    println!("mertens-numerology");
    println!(
        "guard {GUARD:e}; alpha_q truncated down at 6 digits, c_q rounded up at 5, delta_q rounded down at 5"
    );
    println!();
    println!("TABLE, one excluded digit");
    println!("| `q` | `alpha_q` | `c_q` (proved, up) | `delta_q` (down) | closes |");
    println!("|---|---|---|---|---|");
    for q in BASES {
        let r = row(q, 1);
        assert!(c_exp(q as f64, 1.0) >= 0.0, "l^1 floor broken at q = {q}");
        assert_eq!(r.closes, gap(q as f64, 1.0) > 0.0, "tests disagree at q = {q}");
        println!("{}", render(&r));
    }
    println!();
    println!("WALL, one excluded digit");
    assert!(!closes(3689.0, 1.0), "3689 must not close");
    assert!(closes(3690.0, 1.0), "3690 must close");
    println!(
        "- closes fails at `q = 3689`, holds at `q = 3690`; gap `<= {}` and `>= {}`",
        fixed(ceil_units(gap(3689.0, 1.0), 6), 6),
        fixed(floor_units(gap(3690.0, 1.0), 6), 6)
    );
    let (arg, worst, min_gap) = gap_sweep(SWEEP_LO, SWEEP_HI, 1);
    assert!(worst > 0.0, "gap not stepwise increasing");
    assert!(min_gap > 0.0, "gap not positive on the sweep");
    println!(
        "- gap `(q-1) q^(-3/4) - PB_q(1)` steps up at all {} steps of `{SWEEP_LO}..{SWEEP_HI}`; smallest step `>= {}` at `q = {arg}`",
        SWEEP_HI - SWEEP_LO,
        fixed(floor_units(worst, 8), 8)
    );
    println!();
    println!("M-COROLLARY, largest `m` with `PB_q(m) < (q-m) q^(-3/4)`");
    println!("| `q` | max `m` | `alpha_q` | `c_q` (proved, up) | `delta_q` (down) |");
    println!("|---|---|---|---|---|");
    for q in CORO {
        let m = max_excluded(q);
        let r = row(q, m);
        assert!(r.closes, "corollary maximum must close at q = {q}");
        assert!(!closes(q as f64, (m + 1) as f64), "maximum not maximal at q = {q}");
        println!("{}", render_coro(&r));
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    fn kernel_sum(q: u64, t: f64) -> f64 {
        let qf = q as f64;
        let num = (PI * t).sin().abs();
        let mut s = 0.0;
        for r in 0..q {
            let x = (t + r as f64) / qf;
            let d = if x <= 0.5 { x } else { 1.0 - x };
            let den = (PI * d).sin();
            s += if den < 1e-12 { qf } else { num / den };
        }
        s
    }

    #[test]
    fn table_rows_pinned() {
        let got: Vec<String> = BASES.iter().map(|&q| render(&row(q, 1))).collect();
        let want = [
            "| 1000 | 0.999855 | 0.28087 | -0.03102 | no |",
            "| 2000 | 0.999934 | 0.26335 | -0.01342 | no |",
            "| 3000 | 0.999958 | 0.25430 | -0.00434 | no |",
            "| 3689 | 0.999966 | 0.24997 | -0.00001 | no |",
            "| 3690 | 0.999967 | 0.24997 | 0.00000 | yes |",
            "| 5000 | 0.999976 | 0.24393 | 0.00605 | yes |",
            "| 10^4 | 0.999989 | 0.23141 | 0.01858 | yes |",
            "| 10^5 | 0.999999 | 0.19906 | 0.05094 | yes |",
            "| 10^6 | 0.999999 | 0.17589 | 0.07411 | yes |",
            "| 10^9 | 0.999999 | 0.13305 | 0.11695 | yes |",
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn wall_sign_flip() {
        assert!(!closes(3689.0, 1.0));
        assert!(closes(3690.0, 1.0));
        assert!(gap(3689.0, 1.0) < 0.0);
        assert!(gap(3690.0, 1.0) > 0.0);
        for q in 3..3689u64 {
            assert!(!closes(q as f64, 1.0), "unexpected close below the wall at {q}");
        }
    }

    #[test]
    fn gap_steps_up_and_stays_positive() {
        let (arg, worst, min_gap) = gap_sweep(SWEEP_LO, SWEEP_HI, 1);
        assert!(worst > 0.0);
        assert!(min_gap > 0.0);
        assert_eq!(arg, SWEEP_HI - 2);
        assert_eq!(fixed(floor_units(worst, 8), 8), "0.00003172");
        for q in SWEEP_LO..5_000 {
            assert!(gap((q + 1) as f64, 1.0) > gap(q as f64, 1.0), "step down at {q}");
        }
    }

    #[test]
    fn closes_matches_gap_sign() {
        for q in 3..20_000u64 {
            assert_eq!(closes(q as f64, 1.0), gap(q as f64, 1.0) > 0.0, "at q = {q}");
        }
    }

    #[test]
    fn corollary_maxima() {
        assert_eq!(max_excluded(10_000), 6);
        assert_eq!(max_excluded(100_000), 78);
        assert_eq!(max_excluded(1_000_000), 451);
        let got: Vec<String> = CORO
            .iter()
            .map(|&q| render_coro(&row(q, max_excluded(q))))
            .collect();
        let want = [
            "| 10^4 | 6 | 0.999934 | 0.24865 | 0.00129 |",
            "| 10^5 | 78 | 0.999932 | 0.24972 | 0.00022 |",
            "| 10^6 | 451 | 0.999967 | 0.24994 | 0.00003 |",
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn rounding_is_directional() {
        for q in BASES {
            let (qf, mf) = (q as f64, 1.0);
            let r = row(q, 1);
            assert!(fixed(r.c, 5).parse::<f64>().unwrap() >= c_exp(qf, mf) - GUARD);
            assert!(fixed(r.delta, 5).parse::<f64>().unwrap() <= delta(qf, mf) + GUARD);
            assert!(fixed(r.alpha, 6).parse::<f64>().unwrap() <= alpha(qf, mf) + GUARD);
            assert!(c_exp(qf, mf) - fixed(r.c, 5).parse::<f64>().unwrap() > -1e-5);
        }
    }

    #[test]
    fn l1_floor_holds() {
        for q in 3..5_000u64 {
            assert!(c_exp(q as f64, 1.0) >= 0.0, "negative c_q at {q}");
            assert!(pb(q as f64, 1.0) >= 1.0, "PB below the l^1 floor at {q}");
        }
    }

    #[test]
    fn harmonic_bound_dominates() {
        let mut h = 0.0;
        for n in 1..=2_000u64 {
            h += 1.0 / n as f64;
            assert!(harmonic_bound(n as f64) >= h, "harmonic bound fails at {n}");
        }
    }

    #[test]
    fn kernel_bound_dominates_sampled_sup() {
        for q in [50u64, 101, 200] {
            let bound = phi(q as f64);
            let mut worst: f64 = 0.0;
            for i in 0..4_001u64 {
                let t = i as f64 / 4_001.0;
                worst = worst.max(kernel_sum(q, t));
            }
            assert!(worst <= bound, "kernel bound fails at q = {q}: {worst} > {bound}");
            assert!(worst > 0.8 * bound, "kernel bound absurdly loose at q = {q}");
        }
    }

    #[test]
    fn asymptotic_shape() {
        let q = 1e12;
        let c = c_exp(q, 1.0);
        let model = (q.ln().ln() + (2.0 / PI).ln()) / q.ln();
        assert!((c - model).abs() < 0.01);
        assert!(delta(q, 1.0) > 0.13 && delta(q, 1.0) < 0.25);
    }
}
