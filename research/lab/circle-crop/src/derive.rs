use crate::transfer::{levels, masks_seen, rung, show};

// PATTERN LAW

fn mask_of(steps: [i64; 4], reach: usize) -> u16 {
    let mut mask = 0u16;
    for column in 0..reach.min(3) {
        let low = steps[column + 1].max(0);
        let high = steps[column].min(2);
        let mut row = low;
        while row <= high {
            mask |= 1 << (3 * column as u16 + row as u16);
            row += 1;
        }
    }
    mask
}

fn window(steps: [i64; 4]) -> Option<(i64, i64)> {
    let mut low = i64::MIN;
    let mut high = i64::MAX;
    for near in 0..4usize {
        for far in 0..4usize {
            if far > near {
                let gap = (far - near) as i64;
                low = low.max(6 * (steps[near] - steps[far] - 1) / gap);
            }
            if far < near {
                let gap = (near - far) as i64;
                high = high.min(6 * (steps[far] - steps[near] + 1) / gap);
            }
        }
    }
    let floor = low.max(0);
    if high > floor {
        Some((floor, high))
    } else {
        None
    }
}

// LINE ALPHABET

fn line_alphabet(span: i64) -> Vec<(u16, i64, i64)> {
    let mut found: Vec<Option<(i64, i64)>> = vec![None; 512];
    for a in -span..=span {
        for b in -span..=span {
            for c in -span..=span {
                for d in -span..=span {
                    let steps = [a, b, c, d];
                    let Some((low, high)) = window(steps) else {
                        continue;
                    };
                    let mask = mask_of(steps, 3);
                    if mask == 0 {
                        continue;
                    }
                    let slot = &mut found[mask as usize];
                    match slot {
                        None => *slot = Some((low, high)),
                        Some(range) => {
                            range.0 = range.0.min(low);
                            range.1 = range.1.max(high);
                        }
                    }
                }
            }
        }
    }
    (0..512)
        .filter_map(|mask| found[mask].map(|range| (mask as u16, range.0, range.1)))
        .collect()
}

// SHELL AGAINST THE LINE

fn compare(label: &str, radius: u64) {
    let deep = levels(radius);
    let seen = masks_seen(radius, 1, deep - 3);
    let all = masks_seen(radius, 1, deep);
    let alphabet = line_alphabet(9);
    let line: Vec<u16> = alphabet.iter().map(|entry| entry.0).collect();
    let outside: Vec<u16> = seen.iter().cloned().filter(|m| !line.contains(m)).collect();
    let missing: Vec<u16> = line.iter().cloned().filter(|m| !seen.contains(m)).collect();
    let curved: Vec<u16> = all.iter().cloned().filter(|m| !line.contains(m)).collect();
    assert!(outside.is_empty() && missing.is_empty());
    assert_eq!(curved.len(), 1);
    println!(
        "alphabet {} r={} line={} shell={} shell_outside_line=[{}] line_missing_from_shell=[{}] every_level_outside_line=[{}]",
        label,
        radius,
        line.len(),
        seen.len(),
        show(&outside),
        show(&missing),
        show(&curved)
    );
}

// FROZEN SLOPE

fn frozen(label: &str, radius: u64) {
    let deep = levels(radius);
    for level in 0..deep.saturating_sub(1) {
        let below = rung(radius, level);
        let here = rung(radius, level + 1);
        let scale = 3u64.pow(level) as f64;
        let far = (radius * radius) as f64;
        let top_here = here.high.len() - 2;
        let top_below = below.high.len() - 2;
        let mut columns = 0u64;
        let mut column_hits = 0u64;
        let mut boxes = 0u64;
        let mut faults = 0u64;
        let mut chord_faults = 0u64;
        let mut shallow = 0u64;
        let mut shallow_faults = 0u64;
        let mut worst = 0usize;
        for i in 0..=top_here {
            columns += 1;
            let edge = (3 * i) as f64 * scale;
            let height = (far - edge * edge).max(0.0).sqrt() / scale;
            if height <= 0.0 {
                continue;
            }
            let slope = (3 * i) as f64 / height;
            let far_edge = (3 * (i + 1)) as f64 * scale;
            let end = (far - far_edge * far_edge).max(0.0).sqrt() / scale;
            let chord = (height - end) / 3.0;
            let mut guess = [0i64; 4];
            let mut rope = [0i64; 4];
            guess[0] = below.high[3 * i] as i64;
            rope[0] = guess[0];
            for k in 1..4usize {
                guess[k] = (height - k as f64 * slope).floor() as i64;
                rope[k] = (height - k as f64 * chord).floor() as i64;
            }
            let reach = (top_below + 1).saturating_sub(3 * i).min(3);
            let mut exact = true;
            for k in 1..4usize {
                if 3 * i + k <= top_below + 1 && guess[k] != below.high[3 * i + k] as i64 {
                    exact = false;
                }
            }
            if exact {
                column_hits += 1;
            }
            let low = here.high[i + 1];
            let high = here.high[i];
            for y in low..=high {
                boxes += 1;
                let mut truth = [0i64; 4];
                for k in 0..4usize {
                    let column = 3 * i + k;
                    truth[k] = if column <= top_below + 1 {
                        below.high[column] as i64 - 3 * y as i64
                    } else {
                        -1
                    };
                }
                let shifted = [
                    guess[0] - 3 * y as i64,
                    guess[1] - 3 * y as i64,
                    guess[2] - 3 * y as i64,
                    guess[3] - 3 * y as i64,
                ];
                let roped = [
                    rope[0] - 3 * y as i64,
                    rope[1] - 3 * y as i64,
                    rope[2] - 3 * y as i64,
                    rope[3] - 3 * y as i64,
                ];
                let seen = mask_of(truth, reach);
                if seen != mask_of(shifted, reach) {
                    faults += 1;
                    worst = worst.max(i);
                }
                let missed = seen != mask_of(roped, reach);
                if missed {
                    chord_faults += 1;
                }
                if slope <= 1.0 {
                    shallow += 1;
                    if missed {
                        shallow_faults += 1;
                    }
                }
            }
        }
        println!(
            "frozen {} r={} level={} columns={} column_exact={} boxes={} tangent_faults={} tangent_rate={:.6} chord_faults={} chord_rate={:.6} shallow_boxes={} shallow_chord_faults={} shallow_rate={:.6} last_fault_column={} of {}",
            label,
            radius,
            level,
            columns,
            column_hits,
            boxes,
            faults,
            faults as f64 / boxes.max(1) as f64,
            chord_faults,
            chord_faults as f64 / boxes.max(1) as f64,
            shallow,
            shallow_faults,
            shallow_faults as f64 / shallow.max(1) as f64,
            worst,
            top_here
        );
    }
}

// CENTRE SEAT

fn centre(label: &str, radius: u64) {
    let deep = levels(radius);
    for level in 0..deep.saturating_sub(1) {
        let below = rung(radius, level);
        let here = rung(radius, level + 1);
        let top_here = here.high.len() - 2;
        let top_below = below.high.len() - 2;
        let mut boxes = 0u64;
        let mut seated = 0u64;
        for i in 0..=top_here {
            let low = here.high[i + 1];
            let high = here.high[i];
            for y in low..=high {
                boxes += 1;
                let column = 3 * i + 1;
                if column <= top_below
                    && below.high[column + 1] <= 3 * y + 1
                    && 3 * y + 1 <= below.high[column]
                {
                    seated += 1;
                }
            }
        }
        println!(
            "centre {} r={} level={} boxes={} centre_crossed={} rate={:.6} derived={:.6} ratio={:.6}",
            label,
            radius,
            level,
            boxes,
            seated,
            seated as f64 / boxes as f64,
            1.0 / 3.0,
            3.0 * seated as f64 / boxes as f64
        );
    }
}

// STRAIGHT LINE LADDER

fn ones(value: u64, depth: u32) -> u32 {
    let mut mask = 0u32;
    let mut left = value;
    for slot in 0..depth {
        if left % 3 == 1 {
            mask |= 1 << slot;
        }
        left /= 3;
    }
    mask
}

fn ladder(name: &str, num: i128, den: i128, off: i128, depth: u32) {
    let width = 3i128.pow(depth);
    let height = |x: i128| -> i128 { (num * (width - x) + off) / den };
    let mut total = 0u64;
    let mut alive = 0u64;
    let mut hits = vec![0u64; depth as usize];
    for x in 0..width {
        let column = ones(x as u64, depth);
        let low = height(x + 1);
        let high = height(x);
        for z in low..=high {
            total += 1;
            let both = column & ones(z as u64, depth);
            if both == 0 {
                alive += 1;
            }
            let mut left = both;
            while left != 0 {
                let slot = left.trailing_zeros() as usize;
                hits[slot] += 1;
                left &= left - 1;
            }
        }
    }
    let mut product = 1.0f64;
    for slot in 0..depth as usize {
        product *= 1.0 - hits[slot] as f64 / total as f64;
    }
    let survival = alive as f64 / total as f64;
    let psi = survival / product;
    let independent = (8.0f64 / 9.0).powi(depth as i32);
    println!(
        "line {} slope={:.12} shift={:.6} depth={} cells={} alive={} survival={:.9} marginal={:.9} psi={:.9} logpsi={:.6} rate={:.9} eight_ninths={:.9}",
        name,
        num as f64 / den as f64,
        off as f64 / den as f64,
        depth,
        total,
        alive,
        survival,
        product,
        psi,
        psi.ln(),
        survival.powf(1.0 / depth as f64),
        independent.powf(1.0 / depth as f64)
    );
}

// PASS

pub fn derive() {
    let alphabet = line_alphabet(9);
    let masks: Vec<u16> = alphabet.iter().map(|entry| entry.0).collect();
    for span in [12i64, 15] {
        let other: Vec<u16> = line_alphabet(span).iter().map(|entry| entry.0).collect();
        assert_eq!(other, masks);
    }
    println!(
        "line_alphabet states={} list=[{}]",
        masks.len(),
        show(&masks)
    );
    for radius in [6560u64, 19682, 12345] {
        compare("carpet", radius);
    }
    for radius in [6560u64, 19682] {
        frozen("carpet", radius);
        centre("carpet", radius);
    }
    let unit: i128 = 300000000000;
    let slopes: [(&str, i128, i128); 10] = [
        ("half", unit / 2, 0),
        ("half_shifted", unit / 2, 123456789012),
        ("third", unit / 3, 0),
        ("third_shifted", unit / 3, 123456789012),
        ("third_again", unit / 3, 271828182845),
        ("seventh", unit / 7, 0),
        ("seventh_shifted", unit / 7, 123456789012),
        ("root2m1", 124264068712, 123456789012),
        ("golden", 185410196625, 123456789012),
        ("quarterpi", 235619449020, 123456789012),
    ];
    for (name, num, off) in slopes {
        for depth in 4..=12u32 {
            ladder(name, num, unit, off, depth);
        }
    }
    for step in 2..=9u32 {
        let drift = unit / 3i128.pow(step);
        for depth in 4..=12u32 {
            ladder(
                &format!("near_third_3^-{step}"),
                unit / 3 + drift,
                unit,
                123456789012,
                depth,
            );
        }
    }
}
