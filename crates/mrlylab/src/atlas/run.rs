use super::presets::{Mask, Preset, Rule, Seed};
use super::readings::{canonical, read, translate_of, Reading};
use mrlycore::errors::{value_error, Result};
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlymath::life::{animate, churn, design_mask, lattice_index, Boundary, Config, Fate};
use mrlymath::two::{create, Cell2d};
use std::collections::{BTreeMap, BTreeSet};

/// The record of one run.
#[derive(Clone, Debug)]
pub struct Run {
    /// The seed design.
    pub seed: Seed,
    /// The tessellation factor.
    pub tessellation: usize,
    /// The mask.
    pub mask: Mask,
    /// The index of the lattice the mask generates.
    pub mask_index: usize,
    /// The rule.
    pub rule: Rule,
    /// The boundary.
    pub boundary: Boundary,
    /// The fate at the generation cap.
    pub fate: Fate,
    /// The period: 1 for a fixed point, the cycle length for a loop, 0 at timeout.
    pub period: usize,
    /// The first generation of the settled frame or cycle, the cap at timeout.
    pub settled_at: usize,
    /// The log-log slope of the live bounding box side against the generation.
    pub box_growth: f64,
    /// The shift and lag of the last frame as a translate of an earlier one, timeouts only.
    pub mover: Option<(usize, usize, usize)>,
    /// The number of frames recorded.
    pub frames: usize,
    /// The mean churn over the run.
    pub churn: f64,
    /// The settled frame, one frame of the cycle for a loop, none when dead or timed out.
    pub settled: Option<Cell2d>,
    /// The settled frame's reading.
    pub reading: Option<Reading>,
    /// The settled frame's canonical bytes under the dihedral group and torus translation.
    pub key: Option<Vec<u8>>,
    /// The reading of the heatmap's median level set.
    pub heat: Reading,
}

impl Run {
    /// Returns the run's one-line label.
    pub fn label(&self) -> String {
        format!(
            "{}s{}l{} t{} {}[{}] {} {}",
            self.seed.code,
            self.seed.side,
            self.seed.level,
            self.tessellation,
            self.mask.name(),
            self.mask_index,
            self.rule.name(),
            format!("{:?}", self.boundary).to_lowercase()
        )
    }
}

/// The census of one preset.
#[derive(Clone, Debug)]
pub struct Census {
    /// The preset run.
    pub preset: Preset,
    /// The runs in sweep order.
    pub runs: Vec<Run>,
    /// The runs dropped because their mask duplicated an earlier one at the same seed and tessellation.
    pub duplicates: usize,
    /// The number of runs the sweep held before any sample.
    pub universe: usize,
    /// The sampling seed when the universe was cut, none when every run ran.
    pub sample: Option<u64>,
    /// The number of distinct settled frames up to the dihedral group and torus translation.
    pub distinct: usize,
}

fn board(seed: &Seed, tessellation: usize) -> Result<Cell2d> {
    Ok(create(seed.code, seed.side, seed.level, 0, 2)?.tile(tessellation, tessellation))
}

/// Resolves a mask against the tessellated board, the centre popped.
pub fn mask_tensor(mask: &Mask, board: &Cell2d) -> Result<Tensor> {
    match mask {
        Mask::Design { code, side, level } => design_mask(2, *code, *side, *level),
        Mask::Copy { inverted } => {
            let mut tensor = board.types().clone();
            if *inverted {
                tensor = tensor.invert();
            }
            let centre = (tensor.shape[0] - 1) / 2;
            tensor.set(&[centre, centre], 0);
            Ok(tensor)
        }
    }
}

fn bounding_side(grid: &Tensor) -> usize {
    let n = grid.shape[0];
    let (mut rmin, mut rmax, mut cmin, mut cmax) = (n, 0, n, 0);
    let mut any = false;
    for (i, &v) in grid.bytes().iter().enumerate() {
        if v != 0 {
            let (r, c) = (i / n, i % n);
            rmin = rmin.min(r);
            rmax = rmax.max(r);
            cmin = cmin.min(c);
            cmax = cmax.max(c);
            any = true;
        }
    }
    if !any {
        return 0;
    }
    (rmax - rmin + 1).max(cmax - cmin + 1)
}

fn growth(grids: &[Cell2d]) -> f64 {
    let points: Vec<(f64, f64)> = grids
        .iter()
        .enumerate()
        .map(|(t, g)| {
            (
                (t as f64 + 1.0).ln(),
                (bounding_side(g.types()).max(1) as f64).ln(),
            )
        })
        .collect();
    if points.len() < 2 {
        return 0.0;
    }
    let n = points.len() as f64;
    let (sx, sy) = points
        .iter()
        .fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y));
    let (sxx, sxy) = points
        .iter()
        .fold((0.0, 0.0), |(a, b), (x, y)| (a + x * x, b + x * y));
    let denominator = n * sxx - sx * sx;
    if denominator.abs() < 1e-12 {
        return 0.0;
    }
    (n * sxy - sx * sy) / denominator
}

/// Returns the median level set of a run's cumulative visit counts as one frame.
pub fn heat_frame(grids: &[Cell2d]) -> Cell2d {
    let shape = grids[0].types().shape.clone();
    let mut total = vec![0usize; grids[0].types().size()];
    for grid in grids {
        for (i, &v) in grid.types().bytes().iter().enumerate() {
            total[i] += v as usize;
        }
    }
    let mut positive: Vec<usize> = total.iter().copied().filter(|&v| v > 0).collect();
    positive.sort_unstable();
    let level = positive
        .get(positive.len() / 2)
        .copied()
        .unwrap_or(usize::MAX);
    let bytes: Vec<u8> = total.iter().map(|&v| u8::from(v >= level)).collect();
    Cell2d::new(Tensor::of(bytes, shape))
}

fn mover(grids: &[Cell2d]) -> Option<(usize, usize, usize)> {
    let last = grids.last()?.types();
    if last.sum() == 0 {
        return None;
    }
    for (k, earlier) in grids.iter().enumerate().rev().skip(1) {
        if let Some((dr, dc)) = translate_of(earlier.types(), last) {
            if dr != 0 || dc != 0 {
                return Some((dr, dc, grids.len() - 1 - k));
            }
        }
    }
    None
}

/// Runs one cell of the sweep and records it.
pub fn run_one(
    preset: &Preset,
    seed: &Seed,
    tessellation: usize,
    mask: &Mask,
    rule: &Rule,
    boundary: Boundary,
) -> Result<Run> {
    if preset.dimension != 2 {
        return value_error("the atlas runs plane presets only.");
    }
    if *rule == Rule::Every {
        return value_error("the full outer-totalistic space is declared, not run.");
    }
    let board = board(seed, tessellation)?;
    let side = board.types().shape[0];
    if side > preset.canvas || !(preset.canvas - side).is_multiple_of(2) {
        return value_error("the board must fit the canvas with an even margin.");
    }
    let tensor = mask_tensor(mask, &board)?;
    let mask_index = lattice_index(&tensor);
    let config = Config {
        mask: Cell2d::new(tensor),
        birth: rule.birth(),
        survive: rule.survive(),
        boundary,
        max_generations: preset.generations,
        grid_size: 1,
        padding: (preset.canvas - side) / 2,
    };
    let life = animate(&board, &config)?;
    let frames = life.grids.len();
    let (period, settled_at) = match life.fate {
        Fate::Dead | Fate::Alive => (1, frames - 1),
        Fate::Loop => (life.loop_length, frames - life.loop_length),
        Fate::Timeout => (0, preset.generations),
    };
    let settled_index = match life.fate {
        Fate::Alive => Some(frames - 1),
        Fate::Loop => (settled_at..frames).min_by_key(|&i| canonical(life.grids[i].types())),
        Fate::Dead | Fate::Timeout => None,
    };
    let settled = settled_index.map(|i| life.grids[i].clone());
    let reading = match settled_index {
        Some(i) => Some(read(
            &life.grids[i],
            i.checked_sub(1).map(|j| &life.grids[j]),
        )?),
        None => None,
    };
    let key = settled.as_ref().map(|frame| canonical(frame.types()));
    let mover = if life.fate == Fate::Timeout {
        mover(&life.grids)
    } else {
        None
    };
    let heat = read(&heat_frame(&life.grids), None)?;
    Ok(Run {
        seed: *seed,
        tessellation,
        mask: *mask,
        mask_index,
        rule: *rule,
        boundary,
        fate: life.fate,
        period,
        settled_at,
        box_growth: growth(&life.grids[..=settled_at.min(frames - 1)]),
        mover,
        frames,
        churn: churn(&life.grids),
        settled,
        reading,
        key,
        heat,
    })
}

type Cell = (Seed, usize, Mask, Rule, Boundary);

fn sweep(preset: &Preset) -> Result<(Vec<Cell>, usize)> {
    let mut cells = Vec::new();
    let mut duplicates = 0;
    for seed in &preset.seeds {
        for &t in &preset.tessellations {
            let board = board(seed, t)?;
            let mut seen: BTreeSet<Vec<u8>> = BTreeSet::new();
            for mask in &preset.masks {
                let tensor = mask_tensor(mask, &board)?;
                let mut signature = tensor.shape.iter().map(|&n| n as u8).collect::<Vec<u8>>();
                signature.extend_from_slice(tensor.bytes());
                if !seen.insert(signature) {
                    duplicates += preset.rules.len() * preset.boundaries.len();
                    continue;
                }
                for rule in &preset.rules {
                    for &boundary in &preset.boundaries {
                        cells.push((*seed, t, *mask, *rule, boundary));
                    }
                }
            }
        }
    }
    Ok((cells, duplicates))
}

/// Runs a preset's whole sweep, or a seeded sample of cap runs when the sweep is larger, and dedupes the settled frames.
pub fn census(preset: &Preset, cap: usize, sample_seed: u64) -> Result<Census> {
    let (mut cells, duplicates) = sweep(preset)?;
    let universe = cells.len();
    let sample = if universe > cap {
        let mut rng = Rng::new(sample_seed);
        for i in (1..cells.len()).rev() {
            cells.swap(i, rng.below(i + 1));
        }
        cells.truncate(cap);
        Some(sample_seed)
    } else {
        None
    };
    let mut runs = Vec::with_capacity(cells.len());
    for (seed, t, mask, rule, boundary) in &cells {
        runs.push(run_one(preset, seed, *t, mask, rule, *boundary)?);
    }
    let distinct = runs
        .iter()
        .filter_map(|run| run.key.as_ref())
        .collect::<BTreeSet<_>>()
        .len();
    Ok(Census {
        preset: preset.clone(),
        runs,
        duplicates,
        universe,
        sample,
        distinct,
    })
}

/// Tallies the fates per rule, in preset order, as (family, rule, [dead, alive, loop, timeout]).
pub fn fate_table(census: &Census) -> Vec<(&'static str, String, [usize; 4])> {
    let mut table: BTreeMap<usize, [usize; 4]> = BTreeMap::new();
    let position = |rule: &Rule| {
        census
            .preset
            .rules
            .iter()
            .position(|r| r == rule)
            .unwrap_or(usize::MAX)
    };
    for run in &census.runs {
        let slot = match run.fate {
            Fate::Dead => 0,
            Fate::Alive => 1,
            Fate::Loop => 2,
            Fate::Timeout => 3,
        };
        table.entry(position(&run.rule)).or_default()[slot] += 1;
    }
    table
        .into_iter()
        .map(|(i, counts)| {
            let rule = &census.preset.rules[i];
            (rule.family(), rule.name(), counts)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::presets::{mini, MOORE};
    use super::*;
    #[test]
    fn a_life_run_of_the_cross_seed_settles_and_the_copy_mask_pops_its_centre() {
        let preset = mini();
        let seed = Seed {
            code: 6,
            side: 3,
            level: 1,
        };
        let mask = Mask::Design {
            code: MOORE,
            side: 3,
            level: 1,
        };
        let run = run_one(&preset, &seed, 1, &mask, &Rule::Life, Boundary::Wrap).unwrap();
        assert_eq!(run.mask_index, 1);
        assert!(run.frames <= preset.generations);
        assert_eq!(run.heat.side, 27);
        let copy = mask_tensor(&Mask::Copy { inverted: true }, &board(&seed, 3).unwrap()).unwrap();
        assert_eq!(copy.shape, vec![9, 9]);
        assert_eq!(copy.get(&[4, 4]), 0);
        assert_eq!(copy.sum(), 9 * 5 - 1);
    }
}
