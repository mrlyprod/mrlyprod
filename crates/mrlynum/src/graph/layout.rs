use super::models::Network;
use mrlycore::errors::{value_error, Result};
use mrlycore::Rng;

const SPAN: usize = 300;
const FLOOR: f64 = 0.02;
const GAIN: f64 = 0.1;
const JITTER: f64 = 0.05;
const APART: f64 = 1e-9;

/// A force-directed layout: every node repels every other, every branch pulls its ends together, and a cooling cap on the move per tick lets the lattice settle.
///
/// The forces are Fruchterman and Reingold's: repulsion `k^2 / d` between every pair, attraction
/// `d^2 / k` along every branch, with `k` the ideal length `extent / n^(1/dim)` read off the
/// starting box. The temperature caps the move of any node in one tick and cools linearly from
/// a tenth of the extent to a floor over the first ticks, then holds, so the layout keeps
/// creeping toward rest. The seed jitters the start so a symmetric lattice can fold.
///
/// ```
/// let square = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
/// let ring = [(0, 1), (1, 2), (2, 3), (3, 0)];
/// let mut layout = mrlynum::graph::Layout::new(&square, &ring, 2, 1).unwrap();
/// assert!(layout.step(500) < 1e-3);
/// ```
#[derive(Clone)]
pub struct Layout {
    dim: usize,
    positions: Vec<f64>,
    branches: Vec<(usize, usize)>,
    ideal: f64,
    heat: f64,
    tick: usize,
    energy: f64,
    moved: f64,
    rng: Rng,
}

impl Layout {
    /// Starts a layout from flat positions, `dim` floats per node, and the branch pairs, or an error when a branch points off the list.
    pub fn new(
        positions: &[f64],
        branches: &[(usize, usize)],
        dim: usize,
        seed: u64,
    ) -> Result<Layout> {
        if dim == 0 || !positions.len().is_multiple_of(dim) {
            return value_error("positions must hold dim floats per node.");
        }
        let count = positions.len() / dim;
        if branches.iter().any(|&(a, b)| a >= count || b >= count) {
            return value_error("a branch points past the last node.");
        }
        let mut extent: f64 = 0.0;
        for axis in 0..dim {
            let column = positions.iter().skip(axis).step_by(dim).copied();
            let low = column.clone().fold(f64::INFINITY, f64::min);
            let high = column.fold(f64::NEG_INFINITY, f64::max);
            extent = extent.max(high - low);
        }
        if extent <= 0.0 || extent.is_nan() {
            extent = 1.0;
        }
        let ideal = extent / (count.max(1) as f64).powf(1.0 / dim as f64);
        let mut rng = Rng::new(seed);
        let positions = positions
            .iter()
            .map(|&p| p + (rng.unit() - 0.5) * 2.0 * JITTER * ideal)
            .collect();
        Ok(Layout {
            dim,
            positions,
            branches: branches.to_vec(),
            ideal,
            heat: extent / 10.0,
            tick: 0,
            energy: 0.0,
            moved: 0.0,
            rng,
        })
    }
    /// Starts a layout from a network's own positions and branches.
    pub fn from_network(network: &Network, seed: u64) -> Result<Layout> {
        let positions: Vec<f64> = network
            .nodes
            .iter()
            .flat_map(|node| node.position.iter().copied())
            .collect();
        let branches: Vec<(usize, usize)> = network
            .branches
            .iter()
            .map(|b| (b.parent, b.child))
            .collect();
        Layout::new(&positions, &branches, network.dim, seed)
    }
    /// Returns the node count.
    pub fn nodes(&self) -> usize {
        self.positions.len() / self.dim
    }
    /// Returns the ideal branch length `k`.
    pub fn ideal(&self) -> f64 {
        self.ideal
    }
    /// Returns the cap on one node's move in the next tick.
    pub fn temperature(&self) -> f64 {
        let cooled = 1.0 - self.tick.min(SPAN) as f64 / SPAN as f64;
        self.heat * cooled.max(FLOOR)
    }
    /// Returns the ticks stepped so far.
    pub fn ticks(&self) -> usize {
        self.tick
    }
    /// Runs the ticks and returns the energy left: the mean net force per node in units of `k`.
    pub fn step(&mut self, ticks: usize) -> f64 {
        for _ in 0..ticks {
            self.tick_once();
        }
        self.energy
    }
    fn tick_once(&mut self) {
        let (dim, n, k) = (self.dim, self.nodes(), self.ideal);
        let mut push = vec![0.0; n * dim];
        let mut delta = vec![0.0; dim];
        for i in 0..n {
            for j in (i + 1)..n {
                let d = gap(&self.positions, dim, i, j, &mut delta, &mut self.rng);
                let f = k * k / (d * d);
                for a in 0..dim {
                    push[i * dim + a] += delta[a] * f;
                    push[j * dim + a] -= delta[a] * f;
                }
            }
        }
        for &(i, j) in &self.branches {
            let d = gap(&self.positions, dim, i, j, &mut delta, &mut self.rng);
            let f = d / k;
            for a in 0..dim {
                push[i * dim + a] -= delta[a] * f;
                push[j * dim + a] += delta[a] * f;
            }
        }
        let cap = self.temperature();
        let (mut force, mut moved) = (0.0, 0.0);
        for i in 0..n {
            let len = (0..dim)
                .map(|a| push[i * dim + a].powi(2))
                .sum::<f64>()
                .sqrt();
            force += len;
            if len == 0.0 {
                continue;
            }
            let reach = (len * GAIN).min(cap);
            moved += reach;
            for a in 0..dim {
                self.positions[i * dim + a] += push[i * dim + a] / len * reach;
            }
        }
        self.energy = force / (n.max(1) as f64 * k);
        self.moved = moved / n.max(1) as f64;
        self.tick += 1;
    }
    /// Returns the positions, `dim` floats per node.
    pub fn positions(&self) -> &[f64] {
        &self.positions
    }
    /// Returns the mean net force per node in units of `k` after the last tick.
    pub fn energy(&self) -> f64 {
        self.energy
    }
    /// Returns the mean distance a node moved in the last tick.
    pub fn moved(&self) -> f64 {
        self.moved
    }
}

fn gap(positions: &[f64], dim: usize, i: usize, j: usize, delta: &mut [f64], rng: &mut Rng) -> f64 {
    for a in 0..dim {
        delta[a] = positions[i * dim + a] - positions[j * dim + a];
    }
    let d = delta.iter().map(|x| x * x).sum::<f64>().sqrt();
    if d >= APART {
        return d;
    }
    for x in delta.iter_mut() {
        *x = (rng.unit() - 0.5) * APART;
    }
    delta
        .iter()
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt()
        .max(APART / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ring() -> Layout {
        let square = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let ring = [(0, 1), (1, 2), (2, 3), (3, 0)];
        Layout::new(&square, &ring, 2, 1).unwrap()
    }
    #[test]
    fn the_four_cycle_relaxes_to_a_square() {
        let mut layout = ring();
        assert!((layout.ideal() - 0.5).abs() < 1e-12);
        let rest = layout.step(500);
        assert!(rest < 1e-3, "energy {rest}");
        let p = layout.positions();
        let gaps: Vec<f64> = [(0, 1), (1, 2), (2, 3), (3, 0)]
            .iter()
            .map(|&(a, b)| {
                ((p[2 * a] - p[2 * b]).powi(2) + (p[2 * a + 1] - p[2 * b + 1]).powi(2)).sqrt()
            })
            .collect();
        let spread = gaps.iter().cloned().fold(0.0, f64::max)
            - gaps.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(spread < 1e-3, "gaps {gaps:?}");
        let side = 0.5 * 1.5f64.cbrt();
        assert!((gaps[0] - side).abs() < 1e-2, "side {}", gaps[0]);
        assert_eq!(layout.ticks(), 500);
    }
    #[test]
    fn the_seed_replays_and_the_cap_cools() {
        let mut a = ring();
        let mut b = ring();
        a.step(20);
        b.step(20);
        assert_eq!(a.positions(), b.positions());
        assert!(
            a.temperature()
                < Layout::new(&[0.0, 0.0, 1.0, 1.0], &[], 2, 1)
                    .unwrap()
                    .temperature()
        );
        assert!(a.moved() > 0.0);
    }
    #[test]
    fn the_faults_are_named() {
        assert!(Layout::new(&[0.0, 0.0, 1.0], &[], 2, 1).is_err());
        assert!(Layout::new(&[0.0, 0.0], &[(0, 1)], 2, 1).is_err());
        assert!(Layout::new(&[0.0, 0.0, 0.0, 0.0], &[(0, 1)], 2, 1)
            .unwrap()
            .step(3)
            .is_finite());
    }
}
