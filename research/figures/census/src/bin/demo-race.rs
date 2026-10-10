use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::{json, Json, Rng};
use mrlyrs::math::bang::Code;
use mrlyrs::math::two;

const NAME: &str = "demo-race";
const NUMBER: usize = 3;
const BASE: usize = 3;
const LEVEL: usize = 4;
const SIDE: usize = 81;
const WALKERS: usize = 300;
const STEPS: usize = 1200;
const SEED: u64 = 7;

struct Swarm {
    filled: Vec<bool>,
    home: usize,
    at: Vec<usize>,
}

impl Swarm {
    fn spread(&self) -> f64 {
        let (hr, hc) = ((self.home / SIDE) as f64, (self.home % SIDE) as f64);
        let total: f64 = self
            .at
            .iter()
            .map(|&flat| {
                let (r, c) = ((flat / SIDE) as f64 - hr, (flat % SIDE) as f64 - hc);
                r * r + c * c
            })
            .sum();
        (total / self.at.len() as f64).sqrt()
    }
}

fn race(code: u128, seed: u64) -> Result<Swarm> {
    let cell = two::create(Code::from(code), NUMBER, LEVEL, 0, BASE)?;
    let types = cell.types();
    let filled: Vec<bool> = (0..SIDE * SIDE)
        .map(|flat| types.get(&[flat / SIDE, flat % SIDE]).is_ok_and(|v| v != 0))
        .collect();
    let centre = ((SIDE - 1) / 2) as i64;
    let home = (0..SIDE * SIDE)
        .filter(|&flat| filled[flat])
        .min_by_key(|&flat| {
            let (r, c) = ((flat / SIDE) as i64 - centre, (flat % SIDE) as i64 - centre);
            r * r + c * c
        })
        .expect("a filled site");
    let mut rng = Rng::new(seed);
    let mut at = vec![home; WALKERS];
    for _ in 0..STEPS {
        for walker in at.iter_mut() {
            let (r, c) = (*walker / SIDE, *walker % SIDE);
            let (nr, nc) = match rng.below(4) {
                0 => (r + 1, c),
                1 => (r.wrapping_sub(1), c),
                2 => (r, c + 1),
                _ => (r, c.wrapping_sub(1)),
            };
            if nr < SIDE && nc < SIDE && filled[nr * SIDE + nc] {
                *walker = nr * SIDE + nc;
            }
        }
    }
    Ok(Swarm { filled, home, at })
}

fn record(swarm: &Swarm) -> Json {
    json!({"home": swarm.home, "spread": swarm.spread(), "at": swarm.at})
}

fn main() -> Result<()> {
    let fast = race(127, SEED)?;
    let slow = race(239, SEED + 777)?;
    for swarm in [&fast, &slow] {
        assert_eq!(swarm.filled.len(), 6561);
        assert_eq!(swarm.filled.iter().filter(|on| **on).count(), 2401);
        assert_eq!(swarm.at.len(), WALKERS);
        assert!(swarm.at.iter().all(|&flat| swarm.filled[flat]));
    }
    assert!(fast.spread() > slow.spread());
    save(NAME, &json!({"fast": record(&fast), "slow": record(&slow)}))
}
