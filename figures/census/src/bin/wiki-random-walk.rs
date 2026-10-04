use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::{json, Rng};

const NAME: &str = "wiki-random-walk";
const SIDE: i64 = 64;
const STEPS: usize = 1024;
const HOME: i64 = SIDE / 2;

fn walk(seed: u64) -> Vec<(i64, i64)> {
    let mut rng = Rng::new(seed);
    let mut at = (HOME, HOME);
    let mut trace = Vec::with_capacity(STEPS + 1);
    trace.push(at);
    for _ in 0..STEPS {
        let next = match rng.below(4) {
            0 => (at.0 + 1, at.1),
            1 => (at.0 - 1, at.1),
            2 => (at.0, at.1 + 1),
            _ => (at.0, at.1 - 1),
        };
        at = next;
        trace.push(at);
    }
    trace
}

fn inside(trace: &[(i64, i64)]) -> bool {
    trace
        .iter()
        .all(|&(x, y)| (0..SIDE).contains(&x) && (0..SIDE).contains(&y))
}

fn reach(trace: &[(i64, i64)]) -> f64 {
    let last = trace[trace.len() - 1];
    let (dx, dy) = ((last.0 - HOME) as f64, (last.1 - HOME) as f64);
    dx.hypot(dy)
}

fn main() -> Result<()> {
    let typical = (STEPS as f64).sqrt();
    let mut seed = 1u64;
    let mut trace = walk(seed);
    while !inside(&trace) || (reach(&trace) - typical).abs() > 0.08 * typical {
        seed += 1;
        assert!(seed < 20000);
        trace = walk(seed);
    }
    assert_eq!(trace.len(), STEPS + 1);
    assert_eq!(typical, 32.0);
    let visited: std::collections::HashSet<(i64, i64)> = trace.iter().copied().collect();
    assert!(visited.len() > 300 && visited.len() < STEPS);
    let points: Vec<_> = trace.iter().map(|&(x, y)| json!([x, y])).collect();
    save(NAME, &json!({"typical": typical, "trace": points}))
}
