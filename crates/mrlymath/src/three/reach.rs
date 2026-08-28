use super::models::Cell3d;
use mrlycore::errors::{value_error, Result};
use mrlycore::rng::Rng;
use std::collections::HashSet;

const DIRS: [[isize; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

/// The tally of one diffusion run over a cube's wall faces.
#[derive(Clone, Debug, PartialEq)]
pub struct Reach {
    /// The count of wall faces between an inner void and a fill.
    pub faces: usize,
    /// The count of wall faces some walker reacted on.
    pub used: usize,
    /// The used share of the wall faces.
    pub utilisation: f64,
    /// The share of walkers that reacted before exiting or stalling.
    pub reacted: f64,
    /// The count of open entry sites on the first face.
    pub entries: usize,
}

fn shape_of(cell: &Cell3d) -> [usize; 3] {
    let grid = cell.types();
    [grid.shape[0], grid.shape[1], grid.shape[2]]
}

fn shift(site: [usize; 3], step: &[isize; 3], shape: &[usize; 3]) -> Option<[usize; 3]> {
    let mut next = [0usize; 3];
    for axis in 0..3 {
        let moved = site[axis] as isize + step[axis];
        if moved < 0 || moved >= shape[axis] as isize {
            return None;
        }
        next[axis] = moved as usize;
    }
    Some(next)
}

/// Returns every wall face as a void site and the direction index of its filled neighbor.
pub fn walls(cell: &Cell3d) -> Vec<[usize; 4]> {
    let grid = cell.types();
    let shape = shape_of(cell);
    let mut out = Vec::new();
    for x in 0..shape[0] {
        for y in 0..shape[1] {
            for z in 0..shape[2] {
                if grid.get(&[x, y, z]) != 0 {
                    continue;
                }
                for (d, step) in DIRS.iter().enumerate() {
                    if let Some(site) = shift([x, y, z], step, &shape) {
                        if grid.get(&site) != 0 {
                            out.push([x, y, z, d]);
                        }
                    }
                }
            }
        }
    }
    out
}

fn doors(cell: &Cell3d) -> Vec<[usize; 3]> {
    let grid = cell.types();
    let shape = shape_of(cell);
    let mut out = Vec::new();
    for y in 0..shape[1] {
        for z in 0..shape[2] {
            if grid.get(&[0, y, z]) == 0 {
                out.push([0, y, z]);
            }
        }
    }
    out
}

/// Walks seeded reactant in from the open first face and tallies the wall faces it reaches.
///
/// Each walker enters at a random void site of the first face and steps at random. A step
/// back out through that face exits, any other step off the cube reflects, and a step into
/// a fill reacts with the given probability, marking the wall face, or reflects otherwise.
pub fn explore(
    cell: &Cell3d,
    p_react: f64,
    walkers: usize,
    max_steps: usize,
    seed: u64,
) -> Result<Reach> {
    if walkers == 0 {
        return value_error("the walk needs at least one walker.");
    }
    let grid = cell.types();
    let shape = shape_of(cell);
    let faces = walls(cell);
    if faces.is_empty() {
        return value_error("the design offers no wall faces.");
    }
    let entry = doors(cell);
    if entry.is_empty() {
        return value_error("the first face offers no entry site.");
    }
    let mut rng = Rng::new(seed);
    let mut hit: HashSet<[usize; 4]> = HashSet::new();
    let mut reacted = 0usize;
    for _ in 0..walkers {
        let mut site = entry[rng.below(entry.len())];
        for _ in 0..max_steps {
            let d = rng.below(6);
            let step = &DIRS[d];
            let Some(next) = shift(site, step, &shape) else {
                if site[0] as isize + step[0] < 0 {
                    break;
                }
                continue;
            };
            if grid.get(&next) != 0 {
                if rng.chance(p_react) {
                    hit.insert([site[0], site[1], site[2], d]);
                    reacted += 1;
                    break;
                }
                continue;
            }
            site = next;
        }
    }
    Ok(Reach {
        faces: faces.len(),
        used: hit.len(),
        utilisation: hit.len() as f64 / faces.len() as f64,
        reacted: reacted as f64 / walkers as f64,
        entries: entry.len(),
    })
}

/// Sweeps the reaction probability and pairs each value with its utilisation, same seed each run.
pub fn sweep(
    cell: &Cell3d,
    probabilities: &[f64],
    walkers: usize,
    max_steps: usize,
    seed: u64,
) -> Result<Vec<(f64, f64)>> {
    probabilities
        .iter()
        .map(|&p| Ok((p, explore(cell, p, walkers, max_steps, seed)?.utilisation)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three::{carpet, census, net, ones, void};

    fn hull(cell: &Cell3d) -> u128 {
        let grid = cell.types();
        let shape = shape_of(cell);
        let mut count = 0u128;
        for x in 0..shape[0] {
            for y in 0..shape[1] {
                for z in 0..shape[2] {
                    if grid.get(&[x, y, z]) == 0 {
                        continue;
                    }
                    for (site, size) in [(x, shape[0]), (y, shape[1]), (z, shape[2])] {
                        if site == 0 {
                            count += 1;
                        }
                        if site + 1 == size {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    #[test]
    fn walls_complete_the_censused_surface() {
        for cell in [
            carpet(3, 1).unwrap(),
            carpet(3, 2).unwrap(),
            net(3, 1).unwrap(),
            void(2, 1).unwrap(),
            void(2, 2).unwrap(),
        ] {
            let inner = walls(&cell).len() as u128;
            assert_eq!(inner + hull(&cell), census::surface(&cell));
        }
    }

    #[test]
    fn a_solid_block_of_eight_shows_only_its_hull() {
        let cell = ones(2, 1).unwrap();
        assert_eq!(census::fills(&cell), 8);
        assert_eq!(census::surface(&cell), 24);
        assert!(walls(&cell).is_empty());
        assert!(explore(&cell, 0.5, 8, 8, 0).is_err());
    }

    #[test]
    fn the_carpet_cube_offers_twenty_four_walls() {
        let cell = carpet(3, 1).unwrap();
        assert_eq!(walls(&cell).len(), 24);
        assert_eq!(census::surface(&cell), 72);
        assert_eq!(explore(&cell, 0.5, 16, 32, 0).unwrap().entries, 1);
    }

    #[test]
    fn a_fixed_seed_repeats_exactly() {
        let cell = carpet(3, 2).unwrap();
        let a = explore(&cell, 0.5, 64, 128, 9).unwrap();
        let b = explore(&cell, 0.5, 64, 128, 9).unwrap();
        assert_eq!(a, b);
        assert!(a.used <= a.faces);
        assert!((0.0..=1.0).contains(&a.utilisation));
        assert!((0.0..=1.0).contains(&a.reacted));
    }

    #[test]
    fn faster_reactions_screen_the_interior() {
        let cell = carpet(3, 1).unwrap();
        let curve = sweep(&cell, &[0.05, 0.5, 0.9], 800, 400, 0).unwrap();
        for pair in curve.windows(2) {
            assert!(pair[0].1 >= pair[1].1, "{curve:?}");
        }
        assert!(curve[0].1 > curve[2].1, "{curve:?}");
    }
}
