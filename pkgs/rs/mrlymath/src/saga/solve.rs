use super::ops::Op;
use super::{check, sample, Dials, Saga, RULES};
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;

const DEPTH_MAX: usize = 3;

const STEP_PENS: usize = 4;

/// The search allowance: the deepest saga tried and the nodes spent trying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Budget {
    /// The most ops a found saga may hold, capped at three.
    pub depth: usize,
    /// The most candidate applications the search spends.
    pub nodes: usize,
}

impl Default for Budget {
    fn default() -> Budget {
        Budget::new()
    }
}

impl Budget {
    /// Builds the standard allowance: depth three, two thousand nodes.
    pub fn new() -> Budget {
        Budget {
            depth: DEPTH_MAX,
            nodes: 2000,
        }
    }
}

/// Searches shortest-first for a saga reproducing every pair exactly, or None
/// when the budget runs dry or the pairs are empty or illegal. The whole trick
/// is that candidate ops are read off the pairs instead of enumerated blind:
/// recolor maps from matched cells, crops from bounding boxes and found
/// subgrids, tiles and scales from exact size ratios, pads from size deltas,
/// moves from shifted boxes, paints and floods from cell diffs, life steps
/// from a small named rule set, and the eight symmetries always.
pub fn solve(pairs: &[(Tensor, Tensor)], budget: Budget) -> Option<Saga> {
    if pairs.is_empty() {
        return None;
    }
    if pairs
        .iter()
        .any(|(a, b)| check(a).is_err() || check(b).is_err())
    {
        return None;
    }
    let currents: Vec<Tensor> = pairs.iter().map(|(a, _)| a.clone()).collect();
    let targets: Vec<Tensor> = pairs.iter().map(|(_, b)| b.clone()).collect();
    if currents == targets {
        return Some(Saga::new());
    }
    let mut nodes = budget.nodes;
    for depth in 1..=budget.depth.min(DEPTH_MAX) {
        if let Some(ops) = dive(&currents, &targets, depth, &mut nodes) {
            return Some(Saga::of(ops));
        }
        if nodes == 0 {
            return None;
        }
    }
    None
}

/// The identity floor: the empty saga, betting the answer is the input untouched.
pub fn identity() -> Option<Saga> {
    Some(Saga::new())
}

/// The constant floor: a saga stamping the train output seen most often,
/// or None when there are no pairs.
pub fn constant(pairs: &[(Tensor, Tensor)]) -> Option<Saga> {
    let mut best: Option<(&Tensor, usize)> = None;
    for (_, output) in pairs {
        let count = pairs.iter().filter(|(_, b)| b == output).count();
        if best.is_none_or(|(_, top)| count > top) {
            best = Some((output, count));
        }
    }
    let (output, _) = best?;
    Some(Saga::of(vec![Op::Stamp {
        w: output.shape[1],
        cells: output.bytes().to_vec(),
    }]))
}

/// The random floor: a blind saga drawn from the factory, ignoring pairs entirely.
pub fn random(rng: &mut Rng, dials: &Dials) -> Option<Saga> {
    Some(sample(rng, dials))
}

// SEARCH

fn dive(
    currents: &[Tensor],
    targets: &[Tensor],
    depth: usize,
    nodes: &mut usize,
) -> Option<Vec<Op>> {
    for op in candidates(currents, targets) {
        if *nodes == 0 {
            return None;
        }
        *nodes -= 1;
        let Some(next) = advance(currents, &op) else {
            continue;
        };
        if next == targets {
            return Some(vec![op]);
        }
        if depth > 1 && next != currents {
            if let Some(mut tail) = dive(&next, targets, depth - 1, nodes) {
                tail.insert(0, op);
                return Some(tail);
            }
        }
    }
    None
}

fn advance(currents: &[Tensor], op: &Op) -> Option<Vec<Tensor>> {
    currents.iter().map(|grid| op.apply(grid).ok()).collect()
}

fn candidates(currents: &[Tensor], targets: &[Tensor]) -> Vec<Op> {
    let mut out = vec![
        Op::Rotate { k: 1 },
        Op::Rotate { k: 2 },
        Op::Rotate { k: 3 },
        Op::Reflect { vertical: false },
        Op::Reflect { vertical: true },
        Op::Transpose,
    ];
    let mut offer = |op: Op| {
        if !out.contains(&op) {
            out.push(op);
        }
    };
    let same_shape = currents
        .iter()
        .zip(targets)
        .all(|(a, b)| a.shape == b.shape);
    if let Some(map) = recolor_map(currents, targets) {
        offer(Op::Recolor { map });
    }
    if let Some((across, down)) = ratios(currents, targets) {
        for k in 2..=across.max(down) {
            if across.is_multiple_of(k) && down.is_multiple_of(k) {
                offer(Op::Scale { k });
            }
        }
        for a in 1..=across {
            for d in 1..=down {
                if a * d > 1 && across.is_multiple_of(a) && down.is_multiple_of(d) {
                    offer(Op::Tile { across: a, down: d });
                }
            }
        }
    }
    if let Some(count) = padding(currents, targets) {
        for c in 1..=count {
            let ring = count - c;
            offer(Op::Pad {
                count: c,
                color: targets[0].get(&[ring, ring]),
            });
        }
    }
    if currents.iter().any(shrinks) {
        offer(Op::Autocrop);
    }
    if let Some((x, y, w, h)) = found_box(&currents[0], &targets[0]) {
        offer(Op::Crop { x, y, w, h });
    }
    if same_shape {
        for (dx, dy) in shifts(&currents[0], &targets[0]) {
            offer(Op::Translate { dx, dy, fill: 0 });
        }
        if let Some((x, y, color, uniform)) = diffs(&currents[0], &targets[0]) {
            offer(Op::Paint { x, y, color });
            if uniform {
                offer(Op::Flood { x, y, color });
            }
        }
        for color in pens(&currents[0]) {
            for (birth, survive) in RULES {
                offer(Op::Step {
                    color,
                    birth: birth.to_vec(),
                    survive: survive.to_vec(),
                    wrap: false,
                });
            }
        }
    }
    out
}

// CANDIDATE READERS

fn recolor_map(currents: &[Tensor], targets: &[Tensor]) -> Option<[u8; 10]> {
    let mut map = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut fixed = [false; 10];
    for (a, b) in currents.iter().zip(targets) {
        if a.shape != b.shape {
            return None;
        }
        for (&from, &to) in a.bytes().iter().zip(b.bytes()) {
            let slot = from as usize;
            if fixed[slot] {
                if map[slot] != to {
                    return None;
                }
            } else {
                map[slot] = to;
                fixed[slot] = true;
            }
        }
    }
    if map == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
        return None;
    }
    Some(map)
}

fn ratios(currents: &[Tensor], targets: &[Tensor]) -> Option<(usize, usize)> {
    let mut common: Option<(usize, usize)> = None;
    for (a, b) in currents.iter().zip(targets) {
        let (ah, aw) = (a.shape[0], a.shape[1]);
        let (bh, bw) = (b.shape[0], b.shape[1]);
        if !bw.is_multiple_of(aw) || !bh.is_multiple_of(ah) {
            return None;
        }
        let pair = (bw / aw, bh / ah);
        if *common.get_or_insert(pair) != pair {
            return None;
        }
    }
    common.filter(|&(across, down)| across >= 1 && down >= 1)
}

fn padding(currents: &[Tensor], targets: &[Tensor]) -> Option<usize> {
    let mut common: Option<usize> = None;
    for (a, b) in currents.iter().zip(targets) {
        let dw = b.shape[1].checked_sub(a.shape[1])?;
        let dh = b.shape[0].checked_sub(a.shape[0])?;
        if dw != dh || dw == 0 || !dw.is_multiple_of(2) {
            return None;
        }
        if *common.get_or_insert(dw / 2) != dw / 2 {
            return None;
        }
    }
    common
}

fn shrinks(grid: &Tensor) -> bool {
    let (gh, gw) = (grid.shape[0], grid.shape[1]);
    let edge = (0..gw).any(|c| grid.get(&[0, c]) != 0 || grid.get(&[gh - 1, c]) != 0)
        && (0..gh).any(|r| grid.get(&[r, 0]) != 0 || grid.get(&[r, gw - 1]) != 0);
    !edge && grid.sum() > 0
}

fn found_box(a: &Tensor, b: &Tensor) -> Option<(usize, usize, usize, usize)> {
    let (ah, aw) = (a.shape[0], a.shape[1]);
    let (bh, bw) = (b.shape[0], b.shape[1]);
    if bh > ah || bw > aw || (bh == ah && bw == aw) {
        return None;
    }
    for y in 0..=ah - bh {
        for x in 0..=aw - bw {
            let hit = (0..bh).all(|r| (0..bw).all(|c| a.get(&[y + r, x + c]) == b.get(&[r, c])));
            if hit {
                return Some((x, y, bw, bh));
            }
        }
    }
    None
}

fn shifts(a: &Tensor, b: &Tensor) -> Vec<(i64, i64)> {
    let mut out = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    if let (Some(from), Some(to)) = (corner(a), corner(b)) {
        let shift = (to.0 as i64 - from.0 as i64, to.1 as i64 - from.1 as i64);
        if shift != (0, 0) && !out.contains(&shift) {
            out.push(shift);
        }
    }
    out
}

fn corner(grid: &Tensor) -> Option<(usize, usize)> {
    let (gh, gw) = (grid.shape[0], grid.shape[1]);
    let mut best: Option<(usize, usize)> = None;
    for r in 0..gh {
        for c in 0..gw {
            if grid.get(&[r, c]) != 0 {
                let (bc, br) = best.unwrap_or((c, r));
                best = Some((bc.min(c), br.min(r)));
            }
        }
    }
    best
}

fn diffs(a: &Tensor, b: &Tensor) -> Option<(usize, usize, u8, bool)> {
    let (gh, gw) = (a.shape[0], a.shape[1]);
    let mut first: Option<(usize, usize, u8)> = None;
    let mut from: Option<u8> = None;
    let mut to: Option<u8> = None;
    let mut uniform = true;
    for r in 0..gh {
        for c in 0..gw {
            let (av, bv) = (a.get(&[r, c]), b.get(&[r, c]));
            if av == bv {
                continue;
            }
            if first.is_none() {
                first = Some((c, r, bv));
            }
            if *from.get_or_insert(av) != av || *to.get_or_insert(bv) != bv {
                uniform = false;
            }
        }
    }
    first.map(|(x, y, color)| (x, y, color, uniform))
}

fn pens(grid: &Tensor) -> Vec<u8> {
    let mut seen = [false; 10];
    for &cell in grid.bytes() {
        seen[cell as usize] = true;
    }
    (1..10)
        .filter(|&p| seen[p as usize])
        .map(|p| p as u8)
        .take(STEP_PENS)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::{from_rows, out, pose};
    use super::*;

    fn pairs_of(saga: &Saga, seed: u64, count: usize) -> Vec<(Tensor, Tensor)> {
        pose(&mut Rng::new(seed), saga, count)
    }

    #[test]
    fn a_recolor_falls_at_depth_one() {
        let saga = Saga::parse("map1023456789").unwrap();
        let pairs = pairs_of(&saga, 3, 3);
        let found = solve(&pairs, Budget::new()).unwrap();
        assert_eq!(found.len(), 1);
        for (input, output) in &pairs {
            assert_eq!(&out(input, &found).unwrap(), output);
        }
    }
    #[test]
    fn a_turned_scale_falls_at_depth_two() {
        let saga = Saga::parse("rot1_scalek2").unwrap();
        let pairs = pairs_of(&saga, 4, 3);
        let found = solve(&pairs, Budget::new()).unwrap();
        assert!(found.len() <= 2);
        for (input, output) in &pairs {
            assert_eq!(&out(input, &found).unwrap(), output);
        }
    }
    #[test]
    fn the_factory_loop_closes() {
        let dials = Dials {
            ops: 2,
            reps: 1,
            colors: 4,
        };
        let budget = Budget {
            depth: 2,
            nodes: 6000,
        };
        let mut solved = 0;
        for seed in 0..12 {
            let saga = sample(&mut Rng::new(seed), &dials);
            let pairs = pose(&mut Rng::new(seed + 100), &saga, 4);
            if pairs.len() < 4 {
                continue;
            }
            let Some(found) = solve(&pairs, budget) else {
                panic!("seed {seed} saga {} went unsolved", saga.name());
            };
            for (input, output) in &pairs {
                assert_eq!(&out(input, &found).unwrap(), output, "seed {seed}");
            }
            solved += 1;
        }
        assert!(solved >= 10, "only {solved} of the loop closed");
    }
    #[test]
    fn the_budget_is_respected() {
        let saga = Saga::parse("rot1_map1023456789").unwrap();
        let pairs = pairs_of(&saga, 7, 3);
        let starving = Budget { depth: 3, nodes: 2 };
        assert!(solve(&pairs, starving).is_none());
    }
    #[test]
    fn twin_inputs_with_twin_outputs_solve_as_identity() {
        let grid = from_rows(&[vec![1, 2], vec![3, 4]]).unwrap();
        let pairs = vec![(grid.clone(), grid)];
        assert_eq!(solve(&pairs, Budget::new()).unwrap(), Saga::new());
    }
    #[test]
    fn contradictions_and_junk_return_none() {
        let a = from_rows(&[vec![1]]).unwrap();
        let two = from_rows(&[vec![2]]).unwrap();
        let three = from_rows(&[vec![3]]).unwrap();
        let pairs = vec![(a.clone(), two), (a, three)];
        assert!(solve(&pairs, Budget::new()).is_none());
        assert!(solve(&[], Budget::new()).is_none());
        let wide = Tensor::typed(vec![2, 2], mrlycore::tensor::Dtype::U16);
        assert!(solve(&[(wide.clone(), wide)], Budget::new()).is_none());
    }
    #[test]
    fn the_floors_speak_the_same_contract() {
        assert_eq!(identity().unwrap(), Saga::new());
        let a = from_rows(&[vec![1]]).unwrap();
        let b = from_rows(&[vec![2, 2]]).unwrap();
        let c = from_rows(&[vec![3]]).unwrap();
        let pairs = vec![
            (a.clone(), b.clone()),
            (c.clone(), b.clone()),
            (b.clone(), c.clone()),
        ];
        let stamped = constant(&pairs).unwrap();
        assert_eq!(out(&a, &stamped).unwrap(), b);
        assert!(constant(&[]).is_none());
        let blind = random(&mut Rng::new(1), &Dials::new()).unwrap();
        assert_eq!(blind, sample(&mut Rng::new(1), &Dials::new()));
    }
}
