use super::models::Network;
use mrly_core::Rng;
use mrly_core::error::{Result, value_error};

const SPAN: usize = 300;
const FLOOR: f64 = 0.02;
const GAIN: f64 = 0.1;
const JITTER: f64 = 0.05;
const APART: f64 = 1e-9;

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
    pub fn nodes(&self) -> usize {
        self.positions.len() / self.dim
    }
    pub fn ideal(&self) -> f64 {
        self.ideal
    }
    pub fn temperature(&self) -> f64 {
        let cooled = 1.0 - self.tick.min(SPAN) as f64 / SPAN as f64;
        self.heat * cooled.max(FLOOR)
    }
    pub fn ticks(&self) -> usize {
        self.tick
    }
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
    pub fn positions(&self) -> &[f64] {
        &self.positions
    }
    pub fn energy(&self) -> f64 {
        self.energy
    }
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
