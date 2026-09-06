use mrlyweb::tube::*;
use mrlyweb::two::two_grid;

const SWEEP: usize = 2001;

fn nearest(code: &str, number: usize, level: usize, base: usize) -> Vec<f64> {
    let grid = two_grid(code, number, level, 0, base).unwrap();
    let side = grid.width as usize;
    let filled: Vec<(usize, usize)> = (0..side * side)
        .filter(|&at| grid.types[at] != 0)
        .map(|at| (at / side, at % side))
        .collect();
    let mut out = Vec::with_capacity(side * side);
    for row in 0..side {
        for col in 0..side {
            let mut best = f64::MAX;
            for &(a, b) in &filled {
                let (dr, dc) = (row.abs_diff(a) as f64, col.abs_diff(b) as f64);
                best = best.min(dr * dr + dc * dc);
            }
            out.push(best.sqrt());
        }
    }
    out
}

fn holes(level: u32, eps: u64) -> u64 {
    let mut total = 8u64.pow(level);
    for m in 1..=level {
        let side = 3u64.pow(level - m);
        let one = if side <= 2 * eps {
            side * side
        } else {
            4 * eps * side - 4 * eps * eps
        };
        total += 8u64.pow(m - 1) * one;
    }
    total
}

fn band(q: usize, k: usize) -> (f64, f64) {
    let (mut low, mut high) = (f64::MAX, f64::MIN);
    for step in 0..SWEEP {
        let t = 1.0 / q as f64 + (1.0 - 1.0 / q as f64) * step as f64 / (SWEEP - 1) as f64;
        let g = tube_closed(q, k, t).unwrap();
        low = low.min(g);
        high = high.max(g);
    }
    (low, high)
}

#[test]
fn the_field_is_the_exact_distance_to_the_design() {
    for (code, number, level, base) in [("495", 3, 3, 3), ("127", 3, 3, 3), ("7", 2, 5, 2)] {
        let read = tube_distance(code, number, level, base).unwrap();
        let want = nearest(code, number, level, base);
        assert_eq!(read.len(), want.len());
        let worst = read
            .iter()
            .zip(&want)
            .map(|(&a, &b)| (f64::from(a) - b).abs())
            .fold(0.0f64, f64::max);
        assert!(worst < 1e-6, "{code} at level {level} drifts {worst}");
    }
    let carpet = tube_distance("495", 3, 6, 3).unwrap();
    assert_eq!(carpet.len(), 531441);
    assert_eq!(carpet.iter().filter(|&&v| v == 0.0).count(), 262144);
}

#[test]
fn the_tube_is_the_hole_sum_of_the_carpet() {
    assert_eq!(tube_volume("495", 3, 6, 3, 21.0).unwrap(), 478872.0);
    assert_eq!(478872.0 / 531441.0, 5912.0 / 6561.0);
    for (level, eps) in [(3, 1), (4, 2), (5, 7), (6, 21), (6, 40), (6, 121)] {
        let read = tube_volume("495", 3, level, 3, eps as f64).unwrap();
        assert_eq!(
            read,
            holes(level as u32, eps) as f64,
            "level {level} eps {eps}"
        );
    }
}

#[test]
fn the_closed_profile_is_the_carpets_limit() {
    let seam = 379.0 / 280.0;
    assert!((tube_closed(3, 8, 1.0 / 3.0).unwrap() - seam).abs() < 1e-12);
    assert!((tube_closed(3, 8, 1.0).unwrap() - seam).abs() < 1e-12);
    assert!(
        (tube_closed(3, 8, 0.5).unwrap() - (44.0 / 35.0) * 2f64.powf(2.0 - 8f64.ln() / 3f64.ln()))
            .abs()
            < 1e-12
    );
    for step in 1..40 {
        let t = step as f64 / 40.0;
        let g = tube_closed(3, 8, t).unwrap();
        assert!((g - tube_closed(3, 8, t / 3.0).unwrap()).abs() < 1e-12);
    }
    let (low, high) = band(3, 8);
    assert_eq!(
        format!("{high:.8},{low:.8},{:.6}", 100.0 * (high - low) / low),
        "1.35561708,1.35067021,0.366253"
    );
    assert!(high <= 1.35561708227 && low >= 1.3506702097);
    let (five_low, five_high) = band(5, 21);
    assert!(five_high > five_low);
    assert!((tube_closed(5, 21, 0.2).unwrap() - tube_closed(5, 21, 1.0).unwrap()).abs() < 1e-12);
    assert!(tube_closed(3, 3, 0.5).is_err());
    assert!(tube_closed(3, 8, 0.0).is_err());
}

#[test]
fn the_profile_climbs_onto_the_closed_form() {
    let pairs = tube_profile("495", 3, 6, 3, 9).unwrap();
    assert_eq!(pairs.len(), 18);
    assert_eq!(pairs[0], 3f32.ln());
    assert_eq!(pairs[1], 1.125);
    let deep = f64::from(pairs[17]);
    let limit = tube_closed(3, 8, (-f64::from(pairs[16])).exp()).unwrap();
    assert!(deep < limit && limit - deep < 0.01, "the tail reads {deep}");
    let runner = tube_profile("127", 3, 6, 3, 40).unwrap();
    assert_eq!(runner.len(), 80);
    assert!(runner.chunks(2).all(|pair| pair[1] > 0.0));
    assert!(tube_profile("495", 3, 2, 3, 40).unwrap().is_empty());
}

#[test]
fn the_class_is_the_isolated_interior_hole() {
    assert!(tube_class("495", 3, 3).unwrap());
    assert!(tube_class("7", 3, 2).unwrap());
    assert!(!tube_class("127", 3, 3).unwrap());
    assert!(!tube_class("9", 2, 2).unwrap());
    assert!(!tube_class("511", 3, 3).unwrap());
}

#[test]
fn the_grid_is_capped_and_the_empty_design_refused() {
    assert!(tube_distance("495", 3, 7, 3).is_err());
    assert!(tube_distance("0", 3, 3, 3).is_err());
    assert!(tube_volume("495", 3, 0, 3, 1.0).is_err());
}
