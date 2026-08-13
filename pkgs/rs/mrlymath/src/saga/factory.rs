use super::ops::Op;
use super::{out, Saga, COLORS, RULES};
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;

const KINDS: usize = 10;

const POSE_SIDE: usize = 8;

const POSE_PENS: usize = 4;

const DENSITY: f64 = 0.55;

const TRIES: usize = 24;

/// The dials the factory draws sagas under.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dials {
    /// The most ops a drawn saga holds.
    pub ops: usize,
    /// The most reps one drawn op carries.
    pub reps: usize,
    /// The pens a drawn op may touch beyond the background.
    pub colors: usize,
}

impl Default for Dials {
    fn default() -> Dials {
        Dials::new()
    }
}

impl Dials {
    /// Builds the standard dials: up to three ops, double reps, four pens.
    pub fn new() -> Dials {
        Dials {
            ops: 3,
            reps: 2,
            colors: POSE_PENS,
        }
    }
}

/// Draws a short random saga under the dials, the same one for the same stream.
pub fn sample(rng: &mut Rng, dials: &Dials) -> Saga {
    let colors = dials.colors.clamp(1, COLORS - 1);
    let count = 1 + rng.below(dials.ops.max(1));
    let mut saga = Saga::new();
    for _ in 0..count {
        let op = match rng.below(KINDS) {
            0 => Op::Rotate {
                k: 1 + rng.below(3),
            },
            1 => Op::Reflect {
                vertical: rng.boolean(),
            },
            2 => Op::Transpose,
            3 => Op::Pad {
                count: 1,
                color: rng.below(colors + 1) as u8,
            },
            4 => {
                let (across, down) = *rng.choice(&[(2, 1), (1, 2), (2, 2)]);
                Op::Tile { across, down }
            }
            5 => Op::Scale { k: 2 },
            6 => {
                let mut map = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                for slot in map.iter_mut().take(colors + 1).skip(1) {
                    *slot = 1 + rng.below(colors) as u8;
                }
                Op::Recolor { map }
            }
            7 => Op::Translate {
                dx: rng.range(-1, 1),
                dy: rng.range(-1, 1),
                fill: 0,
            },
            8 => Op::Paint {
                x: rng.below(3),
                y: rng.below(3),
                color: 1 + rng.below(colors) as u8,
            },
            _ => {
                let (birth, survive) = *rng.choice(&RULES);
                Op::Step {
                    color: 1 + rng.below(colors) as u8,
                    birth: birth.to_vec(),
                    survive: survive.to_vec(),
                    wrap: false,
                }
            }
        };
        saga.push(op, 1 + rng.below(dials.reps.max(1)));
    }
    saga
}

/// Poses a saga as example pairs: draws noisy inputs, runs the saga over each
/// and emits input and output, skipping inputs the saga refuses, so a stubborn
/// saga may pose fewer pairs than asked.
pub fn pose(rng: &mut Rng, saga: &Saga, count: usize) -> Vec<(Tensor, Tensor)> {
    let mut pairs = Vec::with_capacity(count);
    for _ in 0..count {
        for attempt in 0..TRIES {
            let cap = if attempt < TRIES / 2 { POSE_SIDE } else { 4 };
            let input = seed_grid(rng, cap);
            if let Ok(output) = out(&input, saga) {
                pairs.push((input, output));
                break;
            }
        }
    }
    pairs
}

fn seed_grid(rng: &mut Rng, cap: usize) -> Tensor {
    let w = 3 + rng.below(cap - 2);
    let h = 3 + rng.below(cap - 2);
    let mut grid = Tensor::new(vec![h, w]);
    for i in 0..grid.size() {
        if rng.chance(DENSITY) {
            grid.put(i, 1 + rng.below(POSE_PENS) as i64);
        }
    }
    if grid.sum() == 0 {
        let r = rng.below(h);
        let c = rng.below(w);
        grid.set(&[r, c], 1);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::super::run;
    use super::*;

    #[test]
    fn sample_replays_per_seed() {
        let dials = Dials::new();
        let a = sample(&mut Rng::new(11), &dials);
        let b = sample(&mut Rng::new(11), &dials);
        assert_eq!(a, b);
        assert!((1..=dials.ops).contains(&a.steps.len()));
    }
    #[test]
    fn posed_pairs_obey_the_saga() {
        let mut rng = Rng::new(21);
        let dials = Dials::new();
        let saga = sample(&mut rng, &dials);
        let pairs = pose(&mut rng, &saga, 4);
        assert!(!pairs.is_empty());
        for (input, output) in &pairs {
            assert_eq!(&out(input, &saga).unwrap(), output);
            assert_eq!(run(input, &saga).unwrap().len(), 1 + saga.len());
        }
    }
    #[test]
    fn pose_replays_per_seed() {
        let saga = Saga::parse("rot1_padn1c2").unwrap();
        let a = pose(&mut Rng::new(5), &saga, 3);
        let b = pose(&mut Rng::new(5), &saga, 3);
        assert_eq!(a, b);
        assert_eq!(a.len(), 3);
    }
    #[test]
    fn every_sampled_saga_parses_back() {
        let dials = Dials::new();
        for seed in 0..40 {
            let saga = sample(&mut Rng::new(seed), &dials);
            assert_eq!(Saga::parse(&saga.name()).unwrap(), saga, "{}", saga.name());
        }
    }
}
