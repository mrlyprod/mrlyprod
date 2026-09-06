use mrlycore::Tensor;
use mrlymath::bang::factory::create;
use mrlymath::shape::{census, named, Frac, Shape};

// ROOTS

fn ceil_sqrt(value: u64) -> u64 {
    if value == 0 {
        return 0;
    }
    let mut root = (value as f64).sqrt() as u64;
    while root.saturating_mul(root) < value {
        root += 1;
    }
    while root > 0 && (root - 1) * (root - 1) >= value {
        root -= 1;
    }
    root
}

fn bucket(quad: u64) -> u64 {
    ceil_sqrt(quad).div_ceil(2)
}

// DESIGNS

fn design(dimension: usize, level: usize) -> Tensor {
    match dimension {
        2 => create(7, 3, 2, 2, level).unwrap(),
        _ => create(23, 3, 3, 2, level).unwrap(),
    }
}

// SWEEP

struct Sweep {
    r_max: u64,
    seen: Vec<u64>,
    inside: Vec<u64>,
    touch: Vec<u64>,
    all_seen: Vec<u64>,
    all_inside: Vec<u64>,
    all_touch: Vec<u64>,
}

fn sweep(types: &Tensor, shift: i64, r_max: u64) -> Sweep {
    let dims = types.shape.clone();
    let rank = dims.len();
    let width = (r_max + 2) as usize;
    let mut out = Sweep {
        r_max,
        seen: vec![0; width],
        inside: vec![0; width],
        touch: vec![0; width],
        all_seen: vec![0; width],
        all_inside: vec![0; width],
        all_touch: vec![0; width],
    };
    let mut index = vec![0i64; rank];
    let bytes = types.bytes();
    for cell in bytes {
        let mut centre = 0u64;
        let mut far = 0u64;
        let mut near = 0u64;
        for coordinate in &index {
            let offset = (2 * coordinate + 1 - shift).unsigned_abs();
            centre += offset * offset;
            far += (offset + 1) * (offset + 1);
            let low = offset.saturating_sub(1);
            near += low * low;
        }
        let slots = [bucket(centre), bucket(far), bucket(near)];
        let filled = *cell != 0;
        for (which, slot) in slots.iter().enumerate() {
            if *slot > r_max {
                continue;
            }
            let at = *slot as usize;
            match which {
                0 => {
                    out.all_seen[at] += 1;
                    if filled {
                        out.seen[at] += 1;
                    }
                }
                1 => {
                    out.all_inside[at] += 1;
                    if filled {
                        out.inside[at] += 1;
                    }
                }
                _ => {
                    out.all_touch[at] += 1;
                    if filled {
                        out.touch[at] += 1;
                    }
                }
            }
        }
        for axis in (0..rank).rev() {
            index[axis] += 1;
            if (index[axis] as usize) < dims[axis] {
                break;
            }
            index[axis] = 0;
        }
    }
    for column in [
        &mut out.seen,
        &mut out.inside,
        &mut out.touch,
        &mut out.all_seen,
        &mut out.all_inside,
        &mut out.all_touch,
    ] {
        for r in 1..column.len() {
            column[r] += column[r - 1];
        }
    }
    out
}

impl Sweep {
    fn cut(&self, r: u64) -> u64 {
        self.touch[r as usize] - self.inside[r as usize]
    }
    fn all_cut(&self, r: u64) -> u64 {
        self.all_touch[r as usize] - self.all_inside[r as usize]
    }
}

// ORACLE

fn ball(dimension: usize, shift: i64, radius: Frac) -> Shape {
    if shift == 0 {
        Shape::Ball {
            center: vec![Frac::whole(0); dimension],
            radius,
        }
    } else {
        named("ball", dimension, radius).unwrap()
    }
}

fn oracle(label: &str, types: &Tensor, shift: i64, side: usize, table: &Sweep, radii: &[u64]) {
    for r in radii {
        let shape = ball(types.shape.len(), shift, Frac::new(*r as i64, side as i64));
        let tally = census(&shape, types);
        assert_eq!(tally.filled[2] as u64, table.inside[*r as usize]);
        assert_eq!(tally.filled[1] as u64, table.cut(*r));
        assert_eq!(tally.cells[2] as u64, table.all_inside[*r as usize]);
        assert_eq!(tally.cells[1] as u64, table.all_cut(*r));
        println!(
            "circle-crop oracle {label} r={r} census_in={} census_cut={} sweep_in={} sweep_cut={}",
            tally.filled[2],
            tally.filled[1],
            table.inside[*r as usize],
            table.cut(*r)
        );
    }
}

// TREND

fn trend(label: &str, name: &str, running: &[f64], from: u64, to: u64) {
    if to < from * 3 {
        return;
    }
    let mut least = f64::INFINITY;
    let mut most = f64::NEG_INFINITY;
    let mut total = 0.0f64;
    let mut count = 0u64;
    for r in from..=(to / 3) {
        let here = running[r as usize];
        let there = running[(r * 3) as usize];
        if here <= 0.0 || there <= 0.0 {
            continue;
        }
        let step = (there / here).ln() / 3.0f64.ln();
        least = least.min(step);
        most = most.max(step);
        total += step;
        count += 1;
    }
    if count == 0 {
        return;
    }
    let mut sum_x = 0.0f64;
    let mut sum_y = 0.0f64;
    let mut sum_xx = 0.0f64;
    let mut sum_xy = 0.0f64;
    let mut points = 0.0f64;
    for r in from..=to {
        if running[r as usize] <= 0.0 {
            continue;
        }
        let x = (r as f64).ln();
        let y = running[r as usize].ln();
        sum_x += x;
        sum_y += y;
        sum_xx += x * x;
        sum_xy += x * y;
        points += 1.0;
    }
    let fitted = (points * sum_xy - sum_x * sum_y) / (points * sum_xx - sum_x * sum_x);
    let ends =
        (running[to as usize] / running[from as usize]).ln() / (to as f64 / from as f64).ln();
    println!(
        "circle-crop trend {label} {name} r={from}..{} steps={count} min={:.6} mean={:.6} max={:.6} fitted={fitted:.6} ends={ends:.6}",
        to,
        least,
        total / count as f64,
        most
    );
}

// WINDOWS

fn windows(label: &str, series: &[(&str, Vec<f64>, u64)]) {
    for (name, values, limit) in series {
        let mut lows: Vec<(u64, f64, u64)> = Vec::new();
        let mut power = 1u64;
        while power <= *limit {
            let top = (power * 3 - 1).min(*limit);
            let mut best = 0.0f64;
            let mut spot = power;
            for r in power..=top {
                if values[r as usize] > best {
                    best = values[r as usize];
                    spot = r;
                }
            }
            lows.push((power, best, spot));
            power *= 3;
        }
        for step in 0..lows.len() {
            let (start, best, spot) = lows[step];
            let raw = if start > 1 && best > 0.0 {
                best.ln() / (start as f64).ln()
            } else {
                f64::NAN
            };
            let slope = if step + 1 < lows.len() && best > 0.0 && lows[step + 1].1 > 0.0 {
                (lows[step + 1].1 / best).ln() / 3.0f64.ln()
            } else {
                f64::NAN
            };
            println!(
                "circle-crop window {label} {name} k={step} r={start}..{} max={best:.3} at={spot} at_over_start={:.4} raw={raw:.6} slope={slope:.6}",
                (start * 3 - 1).min(*limit),
                spot as f64 / start as f64
            );
        }
    }
}

// CORNER

fn corner(tag: &str, dimension: usize, levels: &[usize], fill: u64) -> (u64, u64) {
    let mut prior: Option<Sweep> = None;
    let mut tally = (0u64, 0u64);
    for (step, level) in levels.iter().enumerate() {
        let types = design(dimension, *level);
        let side = types.shape[0];
        let r_max = (side - 1) as u64;
        let table = sweep(&types, 0, r_max);
        let label = format!("{tag} corner L={level}");
        for r in 1..=r_max {
            let at = r as usize;
            assert!(table.inside[at] <= table.seen[at]);
            assert!(table.seen[at] <= table.touch[at]);
            assert!(table.all_inside[at] <= table.all_seen[at]);
            assert!(table.all_seen[at] <= table.all_touch[at]);
            let crossing = match dimension {
                2 => (3 * r + 5) as f64,
                _ => std::f64::consts::PI * 3.0f64.sqrt() * (r * r + 1) as f64,
            };
            assert!(table.all_cut(r) as f64 <= crossing);
        }
        if let Some(past) = &prior {
            for r in 1..=past.r_max {
                let at = r as usize;
                assert_eq!(table.seen[at], past.seen[at]);
                assert_eq!(table.inside[at], past.inside[at]);
                assert_eq!(table.touch[at], past.touch[at]);
            }
            println!(
                "circle-crop levelfree {label} matches L={} on r=1..{}",
                levels[step - 1],
                past.r_max
            );
        }
        let width = match (dimension, side) {
            (_, 0..=243) => 5,
            (2, 244..=729) => 4,
            (2, 730..=2187) => 3,
            (2, 2188..=6561) => 2,
            (2, _) => 0,
            (_, _) => 0,
        };
        let sample: Vec<u64> = [13u64, 7, 5, 3, 2][..width]
            .iter()
            .map(|d| (r_max / d).max(1))
            .collect();
        oracle(&label, &types, 0, side, &table, &sample);
        if step + 1 == levels.len() {
            tally = report(&label, &table, fill);
        }
        prior = Some(table);
    }
    tally
}

fn report(label: &str, table: &Sweep, fill: u64) -> (u64, u64) {
    let r_max = table.r_max;
    let reach_max = r_max / 3;
    let width = (r_max + 2) as usize;
    let mut low = vec![0.0f64; width];
    let mut high = vec![0.0f64; width];
    let mut swing = vec![0.0f64; width];
    let mut delta = vec![0.0f64; width];
    let mut rows: Vec<String> = Vec::new();
    let mut banded = 0u64;
    for r in 1..=r_max {
        let at = r as usize;
        let mut depth = 0u32;
        let mut reach = r;
        while reach * 3 <= r_max {
            reach *= 3;
            depth += 1;
        }
        if depth > 0 {
            banded += 1;
        }
        let scale = (fill as f64).powi(depth as i32);
        let centre = table.seen[at] as f64 - table.seen[reach as usize] as f64 / scale;
        let band = table.cut(reach) as f64 / scale;
        swing[at] = centre;
        low[at] = (centre.abs() - band).max(0.0);
        high[at] = centre.abs() + band;
        assert!(low[at] <= table.cut(r) as f64);
        let mut line = format!(
            "circle-crop row {label} r={r} res={} N={} in={} cut={} Nfull={} Nfull_in={} Nfull_cut={}",
            u64::from(r.is_power_of_three()),
            table.seen[at],
            table.inside[at],
            table.cut(r),
            table.all_seen[at],
            table.all_inside[at],
            table.all_cut(r)
        );
        if r <= reach_max {
            let jump = table.seen[(r * 3) as usize] as i64 - fill as i64 * table.seen[at] as i64;
            assert!(jump.unsigned_abs() <= table.cut(r * 3) + fill * table.cut(r));
            delta[at] = jump.abs() as f64;
            line.push_str(&format!(" delta={jump}"));
        } else {
            line.push_str(" delta=na");
        }
        line.push_str(&format!(
            " E={centre:.6} Elow={:.6} Ehigh={:.6} depth={depth}",
            (low[at] * 1e6).floor() / 1e6,
            (high[at] * 1e6).ceil() / 1e6
        ));
        rows.push(line);
    }
    let mut running_delta = vec![0.0f64; width];
    let mut running_cut = vec![0.0f64; width];
    for r in 1..=r_max {
        let at = r as usize;
        running_delta[at] = running_delta[at - 1].max(delta[at]);
        running_cut[at] = running_cut[at - 1].max(table.cut(r) as f64);
        println!(
            "{} run_delta={:.0} run_cut={:.0}",
            rows[at - 1],
            running_delta[at],
            running_cut[at]
        );
    }
    trend(label, "delta", &running_delta, 27, reach_max);
    trend(label, "cut", &running_cut, 27, r_max);
    let mut power = 1u64;
    while power <= r_max {
        let at = power as usize;
        let over = if power >= 3 {
            table.seen[at] as f64 / table.seen[(power / 3) as usize] as f64
        } else {
            f64::NAN
        };
        let mut best = 0.0f64;
        let mut under = 0u64;
        let mut span = 0u64;
        let top = (power * 3 - 1).min(reach_max);
        if power <= reach_max {
            for r in power..=top {
                best = best.max(delta[r as usize]);
                span += 1;
                if delta[r as usize] <= delta[at] {
                    under += 1;
                }
            }
        }
        let rank = if span > 0 {
            under as f64 / span as f64
        } else {
            f64::NAN
        };
        let mut quiet = 0u64;
        let mut reach_span = 0u64;
        for r in power..=(power * 3 - 1).min(r_max) {
            reach_span += 1;
            if table.cut(r) <= table.cut(power) {
                quiet += 1;
            }
        }
        let cut_rank = quiet as f64 / reach_span as f64;
        let defect = if power <= reach_max {
            format!(
                "delta={:.0} window_max_delta={best:.0} delta_rank={rank:.4} E={:.6}",
                delta[at], swing[at]
            )
        } else {
            String::from(
                "delta=beyond_reach window_max_delta=beyond_reach delta_rank=beyond_reach E=beyond_reach",
            )
        };
        let mass = (fill as f64).powi(power.ilog(3) as i32);
        let mu_low = (table.inside[at] as f64 / mass * 1e9).floor() / 1e9;
        let mu_high = ((table.inside[at] + table.cut(power)) as f64 / mass * 1e9).ceil() / 1e9;
        println!(
            "circle-crop resonance {label} r={power} N={} ratio={over:.6} in={} cut={} {defect} cut_rank={cut_rank:.4} mu_low={mu_low:.9} mu_high={mu_high:.9}",
            table.seen[at],
            table.inside[at],
            table.cut(power)
        );
        power *= 3;
    }
    let cuts: Vec<f64> = (0..width)
        .map(|r| table.cut((r as u64).min(r_max)) as f64)
        .collect();
    windows(
        label,
        &[
            ("cut", cuts, r_max),
            ("delta", delta, reach_max),
            ("errorlow", low, reach_max),
            ("errorhigh", high, reach_max),
        ],
    );
    println!("circle-crop bands {label} rows={r_max} banded={banded}");
    (r_max, banded)
}

trait Triadic {
    fn is_power_of_three(&self) -> bool;
}

impl Triadic for u64 {
    fn is_power_of_three(&self) -> bool {
        let mut value = *self;
        if value == 0 {
            return false;
        }
        while value.is_multiple_of(3) {
            value /= 3;
        }
        value == 1
    }
}

// CENTRE

fn centre(tag: &str, dimension: usize, level: usize) -> u64 {
    let types = design(dimension, level);
    let side = types.shape[0];
    let r_max = ((side - 1) / 2) as u64;
    let table = sweep(&types, side as i64, r_max);
    let label = format!("{tag} centre L={level}");
    let cells = (side as u64).pow(dimension as u32);
    let filled = types.sum();
    println!("circle-crop density {label} side={side} cells={cells} fill={filled}");
    let sample: Vec<u64> = [11u64, 5, 3, 2]
        .iter()
        .map(|d| (r_max / d).max(1))
        .collect();
    let sample = if level >= 7 || (dimension == 3 && level >= 5) {
        sample[..2].to_vec()
    } else {
        sample
    };
    oracle(&label, &types, side as i64, side, &table, &sample);
    let mut first = 0u64;
    let mut relative = vec![0.0f64; (r_max + 2) as usize];
    for r in 1..=r_max {
        let at = r as usize;
        assert!(table.inside[at] <= table.seen[at]);
        assert!(table.seen[at] <= table.touch[at]);
        if first == 0 && table.seen[at] > 0 {
            first = r;
        }
        let exact =
            cells as i64 * table.seen[at] as i64 - filled as i64 * table.all_seen[at] as i64;
        let error = exact as f64 / cells as f64;
        let main = filled as f64 / cells as f64 * table.all_seen[at] as f64;
        relative[at] = if main > 0.0 {
            (error / main).abs()
        } else {
            0.0
        };
        println!(
            "circle-crop row {label} r={r} res={} N={} in={} cut={} Nfull={} err_num={exact} err={error:.6} rel={:.6}",
            u64::from(r.is_power_of_three()),
            table.seen[at],
            table.inside[at],
            table.cut(r),
            table.all_seen[at],
            relative[at]
        );
    }
    let hole = (side / 3 - 1) as u64 / 2;
    assert!(first > hole);
    if dimension == 2 {
        assert_eq!(first, hole + 1);
    }
    println!("circle-crop hole {label} block_inradius={hole} first_hit={first}");
    let cuts: Vec<f64> = (0..=(r_max + 1))
        .map(|r| table.cut(r.min(r_max)) as f64)
        .collect();
    windows(
        &label,
        &[("cut", cuts, r_max), ("relative", relative, r_max)],
    );
    r_max
}

// GAUSS

fn gauss(dimension: usize, level: usize) {
    let types = design(dimension, level);
    let side = types.shape[0];
    let r_max = (side - 1) as u64;
    let table = sweep(&types, 0, r_max);
    let pi = std::f64::consts::PI;
    let mut worst = 0.0f64;
    for r in 1..=r_max {
        let volume = if dimension == 2 {
            pi * (r as f64) * (r as f64) / 4.0
        } else {
            pi * (r as f64).powi(3) / 6.0
        };
        let gap = (table.all_seen[r as usize] as f64 - volume).abs()
            / (r as f64).powi(dimension as i32 - 1);
        worst = worst.max(gap);
    }
    println!(
        "circle-crop gauss D={dimension} L={level} r_max={r_max} max|Nfull-vol|/r^(D-1)={worst:.6}"
    );
}

// TRANSFORM

fn digits(dimension: usize) -> Vec<Vec<f64>> {
    let types = design(dimension, 1);
    let mut out: Vec<Vec<f64>> = Vec::new();
    let mut index = vec![0usize; dimension];
    for cell in types.bytes() {
        if *cell != 0 {
            out.push(index.iter().map(|value| *value as f64).collect());
        }
        for axis in (0..dimension).rev() {
            index[axis] += 1;
            if index[axis] < 3 {
                break;
            }
            index[axis] = 0;
        }
    }
    out
}

fn factor(set: &[Vec<f64>], point: &[f64]) -> (f64, f64) {
    let mut real = 0.0f64;
    let mut imaginary = 0.0f64;
    for digit in set {
        let dot: f64 = digit.iter().zip(point).map(|(a, b)| a * b).sum();
        let angle = -2.0 * std::f64::consts::PI * dot;
        real += angle.cos();
        imaginary += angle.sin();
    }
    let mass = set.len() as f64;
    (real / mass, imaginary / mass)
}

fn hat(set: &[Vec<f64>], point: &[f64], terms: u32) -> (f64, f64) {
    let mut real = 1.0f64;
    let mut imaginary = 0.0f64;
    for term in 1..=terms {
        let scale = 3.0f64.powi(term as i32);
        let scaled: Vec<f64> = point.iter().map(|value| value / scale).collect();
        let (pr, pi) = factor(set, &scaled);
        let next = (real * pr - imaginary * pi, real * pi + imaginary * pr);
        real = next.0;
        imaginary = next.1;
    }
    (real, imaginary)
}

fn transform(tag: &str, dimension: usize, point: &[f64]) {
    let set = digits(dimension);
    let tripled: Vec<f64> = point.iter().map(|value| value * 3.0).collect();
    let (ar, ai) = hat(&set, point, 60);
    let (br, bi) = hat(&set, &tripled, 60);
    let (pr, pi) = factor(&set, point);
    assert!((br - (pr * ar - pi * ai)).abs() < 1e-12);
    assert!((bi - (pr * ai + pi * ar)).abs() < 1e-12);
    let integral = point
        .iter()
        .all(|value| (value - value.round()).abs() < 1e-12);
    let size = (pr * pr + pi * pi).sqrt();
    assert_eq!(integral, (size - 1.0).abs() < 1e-12);
    let name: Vec<String> = point.iter().map(|value| format!("{value}")).collect();
    println!(
        "circle-crop transform {tag} t=({}) integral={integral} hat_mu={:.8} hat_mu_3t={:.8} P={size:.8}",
        name.join(","),
        (ar * ar + ai * ai).sqrt(),
        (br * br + bi * bi).sqrt()
    );
}

// MAIN

fn main() {
    println!("circle-crop generator: CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p circle-crop");
    println!("circle-crop convention: cells are indexed x in [0,3^L)^D, a cell counts when its centre x+1/2 lies in the closed ball, corner balls sit at the lattice corner 0 and centre balls at the grid centre 3^L/2");
    gauss(2, 5);
    gauss(3, 3);
    transform("carpet", 2, &[0.5, 0.0]);
    transform("carpet", 2, &[1.0, 0.0]);
    transform("sponge", 3, &[0.5, 0.0, 0.0]);
    transform("sponge", 3, &[1.0, 0.0, 0.0]);
    let carpet = corner("carpet", 2, &[5, 6, 7, 8, 9], 8);
    let sponge = corner("sponge", 3, &[3, 4, 5, 6], 20);
    let mut rows = carpet.0 + sponge.0;
    let banded = carpet.1 + sponge.1;
    rows += centre("carpet", 2, 6);
    rows += centre("carpet", 2, 7);
    rows += centre("sponge", 3, 4);
    rows += centre("sponge", 3, 5);
    println!("circle-crop totals rows={rows} banded={banded}");
}
