use std::collections::BTreeSet;

use mrlynum::factor::gcd;
use mrlynum::lattice::{farey, new_nodes, nodes, totients};

const QS: [usize; 7] = [125, 250, 500, 1000, 2000, 4000, 8000];

const SMALL: [usize; 4] = [10, 30, 60, 125];

fn literal_stack(q: usize) -> BTreeSet<(usize, usize)> {
    let mut lit = BTreeSet::new();
    for n in 1..=q {
        for k in 1..=n {
            let g = gcd(k, n);
            lit.insert((k / g, n / g));
        }
    }
    lit
}

fn window_pairs(q: usize) -> BTreeSet<(usize, usize)> {
    nodes(q)
        .into_iter()
        .filter(|node| node.num > 0)
        .map(|node| (node.num as usize, node.den as usize))
        .collect()
}

fn walk_pairs(q: usize) -> BTreeSet<(usize, usize)> {
    farey(q)
        .into_iter()
        .filter(|node| node.num > 0)
        .map(|node| (node.num as usize, node.den as usize))
        .collect()
}

fn drawn_by(q: usize, pair: (usize, usize)) -> usize {
    let (a, b) = pair;
    (1..=q).filter(|n| (a * n) % b == 0).count()
}

fn brightness_ok(q: usize) -> bool {
    nodes(q).into_iter().filter(|node| node.num > 0).all(|node| {
        let b = node.den as usize;
        node.brightness == (q / b) as u64 && drawn_by(q, (node.num as usize, b)) == q / b
    })
}

fn totient_sum(q: usize, phi: &[u64]) -> u64 {
    phi[1..=q].iter().sum()
}

fn meter_walk(q: usize, m: u64) -> (f64, f64, u64) {
    let mut s1 = 0.0f64;
    let mut s2 = 0.0f64;
    let mut j = 0u64;
    for node in farey(q) {
        if node.num == 0 {
            continue;
        }
        j += 1;
        let delta = node.num as f64 / node.den as f64 - j as f64 / m as f64;
        s1 += delta.abs();
        s2 += delta * delta;
    }
    (s1, s2, j)
}

fn meter_window(q: usize, m: u64) -> (f64, f64, u64) {
    let mut s1 = 0.0f64;
    let mut s2 = 0.0f64;
    let mut j = 0u64;
    for node in nodes(q) {
        if node.num == 0 {
            continue;
        }
        j += 1;
        let delta = node.num as f64 / node.den as f64 - j as f64 / m as f64;
        s1 += delta.abs();
        s2 += delta * delta;
    }
    (s1, s2, j)
}

fn verdict(ok: bool) -> &'static str {
    if ok {
        "PASS"
    } else {
        "FAIL"
    }
}

fn main() {
    let top = *QS.last().unwrap();
    let phi = totients(top);

    println!("LIT NODES ARE THE FAREY NODES");
    println!("      Q   lit set   window   mediant walk   sum phi(k)   floor(Q/b)   agree");
    for q in SMALL {
        let lit = literal_stack(q);
        let win = window_pairs(q);
        let walk = walk_pairs(q);
        let control = totient_sum(q, &phi);
        let bright = brightness_ok(q);
        let ok = lit == win && win == walk && lit.len() as u64 == control && bright;
        println!(
            "  {:>5}   {:>7}   {:>6}   {:>12}   {:>10}   {:>10}   {}",
            q,
            lit.len(),
            win.len(),
            walk.len(),
            control,
            verdict(bright),
            verdict(ok)
        );
    }

    let fresh = (2..=60).all(|n| new_nodes(n) == phi[n]);
    println!(
        "  new nodes at scale n equals phi(n), n = 2..60 : {}",
        verdict(fresh)
    );

    println!();
    println!("THE DISCREPANCY METER");
    println!("      Q       nodes   sum phi(k)     S2*Q   S1/sqrt(Q)   exp S2   exp S1");
    let mut prev: Option<(usize, f64, f64)> = None;
    for q in QS {
        let m = totient_sum(q, &phi);
        let (s1, s2, seen) = meter_walk(q, m);
        let (mut e2, mut e1) = ("-".to_string(), "-".to_string());
        if let Some((pq, p1, p2)) = prev {
            let span = (q as f64 / pq as f64).ln();
            e2 = format!("{:+.3}", (s2 / p2).ln() / span);
            e1 = format!("{:+.3}", (s1 / p1).ln() / span);
        }
        prev = Some((q, s1, s2));
        println!(
            "  {:>5}  {:>10}  {:>11}   {:.4}       {:.4}   {:>6}   {:>6}",
            q,
            seen,
            m,
            s2 * q as f64,
            s1 / (q as f64).sqrt(),
            e2,
            e1
        );
    }

    println!();
    println!("CROSS-CHECK  the sorted window route against the mediant walk");
    println!("      Q       nodes     S2*Q   S1/sqrt(Q)   agrees");
    for q in [125usize, 250, 500, 1000] {
        let m = totient_sum(q, &phi);
        let (w1, w2, wn) = meter_window(q, m);
        let (r1, r2, rn) = meter_walk(q, m);
        let ok = wn == rn && wn == m && (w1 - r1).abs() < 1e-9 && (w2 - r2).abs() < 1e-12;
        println!(
            "  {:>5}  {:>10}   {:.4}       {:.4}   {}",
            q,
            wn,
            w2 * q as f64,
            w1 / (q as f64).sqrt(),
            verdict(ok)
        );
    }
}
