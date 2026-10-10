use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::{json, Rng};
use std::collections::{HashMap, HashSet};

const NAME: &str = "wiki-domino-tilings";
const ORDER: i64 = 32;
const SEED: u64 = 5;
const DOMINOES: usize = 1056;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Domino {
    x: i64,
    y: i64,
    flat: bool,
}

// SHUFFLE

fn inside(order: i64, x: i64, y: i64) -> bool {
    (2 * x + 1).abs() + (2 * y + 1).abs() <= 2 * order
}

fn cells(d: Domino) -> [(i64, i64); 2] {
    if d.flat {
        [(d.x, d.y), (d.x + 1, d.y)]
    } else {
        [(d.x, d.y), (d.x, d.y + 1)]
    }
}

fn leads(order: i64, x: i64, y: i64) -> bool {
    (x + y + order).rem_euclid(2) == 0
}

fn shuffle(order: i64, tiles: &[Domino], rng: &mut Rng) -> Vec<Domino> {
    let laid: HashSet<Domino> = tiles.iter().copied().collect();
    let mut doomed = HashSet::new();
    for &d in tiles {
        if leads(order, d.x, d.y) {
            let facing = if d.flat {
                Domino { y: d.y + 1, ..d }
            } else {
                Domino { x: d.x + 1, ..d }
            };
            if laid.contains(&facing) {
                doomed.insert(d);
                doomed.insert(facing);
            }
        }
    }
    let mut next: Vec<Domino> = tiles
        .iter()
        .filter(|d| !doomed.contains(d))
        .map(|&d| {
            let step = if leads(order, d.x, d.y) { 1 } else { -1 };
            if d.flat {
                Domino { y: d.y + step, ..d }
            } else {
                Domino { x: d.x + step, ..d }
            }
        })
        .collect();
    let grown = order + 1;
    let mut taken = HashSet::new();
    for &d in &next {
        for (x, y) in cells(d) {
            assert!(inside(grown, x, y));
            assert!(taken.insert((x, y)));
        }
    }
    for y in (-grown..grown).rev() {
        for x in -grown..grown {
            if !inside(grown, x, y) || taken.contains(&(x, y)) {
                continue;
            }
            assert!(leads(grown, x, y));
            for (u, v) in [(x, y), (x + 1, y), (x, y - 1), (x + 1, y - 1)] {
                assert!(inside(grown, u, v));
                assert!(taken.insert((u, v)));
            }
            if rng.boolean() {
                next.push(Domino { x, y, flat: true });
                next.push(Domino {
                    x,
                    y: y - 1,
                    flat: true,
                });
            } else {
                next.push(Domino {
                    x,
                    y: y - 1,
                    flat: false,
                });
                next.push(Domino {
                    x: x + 1,
                    y: y - 1,
                    flat: false,
                });
            }
        }
    }
    next
}

// COUNT

fn count(rows: &[Vec<bool>]) -> u64 {
    let width = rows[0].len();
    let mut states: HashMap<u64, u64> = HashMap::from([(0, 1)]);
    for row in rows {
        let mut next = HashMap::new();
        for (&mask, &ways) in &states {
            let mut stack = vec![(0, mask, 0u64)];
            while let Some((i, held, below)) = stack.pop() {
                if i == width {
                    *next.entry(below).or_insert(0) += ways;
                    continue;
                }
                let bit = 1u64 << i;
                if held & bit != 0 {
                    if row[i] {
                        stack.push((i + 1, held, below));
                    }
                    continue;
                }
                if !row[i] {
                    stack.push((i + 1, held, below));
                    continue;
                }
                stack.push((i + 1, held, below | bit));
                if i + 1 < width && row[i + 1] && held & (bit << 1) == 0 {
                    stack.push((i + 2, held, below));
                }
            }
        }
        states = next;
    }
    states.get(&0).copied().unwrap_or(0)
}

fn aztec(order: i64) -> Vec<Vec<bool>> {
    (-order..order)
        .map(|y| (-order..order).map(|x| inside(order, x, y)).collect())
        .collect()
}

// WRITE

fn main() -> Result<()> {
    for order in 1..=6 {
        assert_eq!(count(&aztec(order)), 1 << (order * (order + 1) / 2));
    }
    let fibonacci: Vec<u64> = (1..=10).map(|n| count(&vec![vec![true; 2]; n])).collect();
    assert_eq!(fibonacci, vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    assert_eq!(count(&vec![vec![true; 8]; 8]), 12_988_816);

    let mut rng = Rng::new(SEED);
    let mut tiles = Vec::new();
    for order in 0..ORDER {
        tiles = shuffle(order, &tiles, &mut rng);
    }
    assert_eq!((ORDER * (ORDER + 1)) as usize, DOMINOES);
    assert_eq!(tiles.len(), DOMINOES);
    let region: usize = aztec(ORDER).iter().flatten().filter(|&&on| on).count();
    assert_eq!(region, 2 * DOMINOES);
    let covered: HashSet<(i64, i64)> = tiles.iter().flat_map(|&d| cells(d)).collect();
    assert_eq!(covered.len(), region);
    assert!(tiles.iter().filter(|d| !d.flat).count().is_multiple_of(2));

    let rows: Vec<_> = tiles
        .iter()
        .map(|d| json!([d.x, d.y, i64::from(d.flat)]))
        .collect();
    save(NAME, &json!({"order": ORDER, "tiles": rows}))
}
