use crate::{code_of, Fault};
use mrlycore::json;
use mrlymath::formulas::{self, six as hexagon};
use mrlymath::{six, three, two};
use mrlynum::graph::{self, census, roles, Layout as Relax, Network, Role};
use wasm_bindgen::prelude::*;

const LIMIT: u128 = 20000;
const DEEPEST: u32 = 40;
const ROOT3: f64 = 1.732_050_807_568_877_2;

fn side(number: usize, level: usize) -> Result<usize, Fault> {
    number
        .checked_pow(level as u32)
        .ok_or_else(|| Fault::new("that level is deeper than a side counts."))
}

fn dim_of(space: &str) -> Result<usize, Fault> {
    match space {
        "flat" | "hex" => Ok(2),
        "cube" => Ok(3),
        _ => Err(Fault::new(format!(
            "space {space:?} is none of \"flat\", \"cube\" and \"hex\"."
        ))),
    }
}

fn kind_of(space: &str, kind: &str) -> Result<(), Fault> {
    let kinds: &[&str] = if space == "hex" {
        &["core", "dual", "edge"]
    } else {
        &["core", "edge", "tunnel"]
    };
    if kinds.contains(&kind) {
        return Ok(());
    }
    Err(Fault::new(format!(
        "graph {kind:?} is none of {} in the {space} space.",
        kinds.join(", ")
    )))
}

fn bound(
    space: &str,
    code: &str,
    number: usize,
    level: u32,
    base: usize,
    kind: &str,
) -> Result<u128, Fault> {
    let dim = dim_of(space)?;
    kind_of(space, kind)?;
    let code = code_of(code)?;
    if space == "hex" {
        let side = side(number, level as usize)?;
        return Ok(match kind {
            "edge" => hexagon::solid_slice_vertices(side)?,
            _ => hexagon::grid_triangles(number, level),
        });
    }
    Ok(match kind {
        "core" => formulas::fill(code, number, dim, level, base)?,
        "tunnel" => formulas::void(code, number, dim, level, base)?,
        _ => formulas::fill(code, number, dim, level, base)? << dim,
    })
}

/// Bounds the node count of the design's graph in closed form, before any build: the fill for the core, the void for the tunnels, `2^dim` fills for the edges, and the hexagon's triangles or corners for a slice, as a decimal string.
#[wasm_bindgen]
pub fn graph_size(
    space: &str,
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    kind: &str,
) -> Result<String, Fault> {
    Ok(bound(space, code, number, level as u32, base, kind)?.to_string())
}

/// Returns the largest level, at least one, whose graph bound stays within the budget, so a slider stops before a build stalls.
#[wasm_bindgen]
pub fn graph_cap(
    space: &str,
    code: &str,
    number: usize,
    base: usize,
    kind: &str,
    budget: usize,
) -> Result<usize, Fault> {
    bound(space, code, number, 1, base, kind)?;
    let fits = |level: u32| {
        bound(space, code, number, level, base, kind).is_ok_and(|count| count <= budget as u128)
    };
    let mut level = 1;
    while level < DEEPEST && fits(level + 1) {
        level += 1;
    }
    Ok(level as usize)
}

fn network(
    space: &str,
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    kind: &str,
) -> Result<(Network, Option<i64>), Fault> {
    let nodes = bound(space, code, number, level as u32, base, kind)?;
    if nodes > LIMIT {
        return Err(Fault::new(format!(
            "up to {nodes} nodes is past the {LIMIT} this page walks; lower the level."
        )));
    }
    let code = code_of(code)?;
    match space {
        "flat" => {
            let cell = two::create(code, number, level, 0, base)?;
            let net = match kind {
                "core" => two::graph::core_graph(&cell)?,
                "edge" => two::graph::edge_graph(&cell)?,
                _ => two::graph::tunnel_graph(&cell)?,
            };
            Ok((net, Some(two::census(&cell)?.euler)))
        }
        "cube" => {
            let cell = three::create(code, number, level, base)?;
            let net = match kind {
                "core" => three::graph::core_graph(&cell)?,
                "edge" => three::graph::edge_graph(&cell)?,
                _ => three::graph::tunnel_graph(&cell)?,
            };
            Ok((net, Some(three::census(&cell)?.euler)))
        }
        _ => {
            let cell = six::cut(&three::create(code, number, level, base)?)?;
            let mut net = match kind {
                "core" => six::graph::slice_core_graph(&cell)?,
                "dual" => six::graph::slice_dual_graph(&cell)?,
                _ => six::graph::slice_edge_graph(&cell, Some(six::FILL))?,
            };
            for node in &mut net.nodes {
                node.position[0] *= 0.5;
                node.position[1] *= ROOT3 / 4.0;
            }
            Ok((net, Some(six::fills_only(&cell).euler)))
        }
    }
}

/// Lists the node positions of the design's graph: the dimension, the node count, then that many coordinates per node, a hex slice already at its true aspect with unit triangle sides.
#[wasm_bindgen]
pub fn graph_nodes(
    space: &str,
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    kind: &str,
) -> Result<Vec<f32>, Fault> {
    let (net, _) = network(space, code, number, level, base, kind)?;
    let mut out = vec![net.dim as f32, net.nodes.len() as f32];
    for node in &net.nodes {
        out.extend(node.position.iter().map(|&p| p as f32));
    }
    Ok(out)
}

/// Lists the branches of the design's graph as node index pairs.
#[wasm_bindgen]
pub fn graph_branches(
    space: &str,
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    kind: &str,
) -> Result<Vec<u32>, Fault> {
    let (net, _) = network(space, code, number, level, base, kind)?;
    Ok(net
        .branches
        .iter()
        .flat_map(|b| [b.parent as u32, b.child as u32])
        .collect())
}

/// Tags every node of the design's graph by degree: 0 alone, 1 a tip, 2 on a path, 3 a junction.
#[wasm_bindgen]
pub fn graph_roles(
    space: &str,
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    kind: &str,
) -> Result<Vec<u8>, Fault> {
    let (net, _) = network(space, code, number, level, base, kind)?;
    Ok(roles(&net)
        .iter()
        .map(|role| match role {
            Role::Alone => 0,
            Role::Tip => 1,
            Role::Through => 2,
            Role::Junction => 3,
        })
        .collect())
}

/// Takes the census of the design's graph: nodes, branches, tips, junctions, pieces, total length, box dimension, and the Euler number of the design the graph came from, as JSON.
#[wasm_bindgen]
pub fn graph_census(
    space: &str,
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    kind: &str,
) -> Result<String, Fault> {
    let (net, euler) = network(space, code, number, level, base, kind)?;
    let tally = census(&net);
    Ok(json!({
        "dim": net.dim,
        "nodes": tally.nodes,
        "branches": tally.branches,
        "tips": tally.tips,
        "junctions": tally.junctions,
        "components": tally.components,
        "length": tally.total_length,
        "box": tally.fractal_dimension,
        "euler": euler,
    })
    .to_string())
}

/// A force layout: the nodes push apart, the branches pull, and a cooling cap lets the lattice settle into a shape.
#[wasm_bindgen]
pub struct Layout {
    inner: Relax,
}

#[wasm_bindgen]
impl Layout {
    /// Starts from flat positions, `dim` floats per node, and the branch pairs, jittered by the seed.
    #[wasm_bindgen(constructor)]
    pub fn new(
        positions: &[f32],
        branches: &[u32],
        dim: usize,
        seed: u32,
    ) -> Result<Layout, Fault> {
        let positions: Vec<f64> = positions.iter().map(|&p| f64::from(p)).collect();
        let pairs: Vec<(usize, usize)> = branches
            .chunks(2)
            .map(|pair| (pair[0] as usize, *pair.get(1).unwrap_or(&pair[0]) as usize))
            .collect();
        Ok(Layout {
            inner: graph::Layout::new(&positions, &pairs, dim, u64::from(seed))?,
        })
    }
    /// Runs the ticks and returns the energy left, the mean net force per node in units of the ideal length.
    pub fn step(&mut self, ticks: usize) -> f64 {
        self.inner.step(ticks)
    }
    /// Returns the positions, `dim` floats per node.
    pub fn positions(&self) -> Vec<f32> {
        self.inner.positions().iter().map(|&p| p as f32).collect()
    }
    /// Returns the energy after the last tick.
    pub fn energy(&self) -> f64 {
        self.inner.energy()
    }
    /// Returns the mean distance a node moved in the last tick.
    pub fn moved(&self) -> f64 {
        self.inner.moved()
    }
    /// Returns the ticks stepped so far.
    pub fn ticks(&self) -> u32 {
        self.inner.ticks() as u32
    }
    /// Returns the cap on one node's move in the next tick.
    pub fn temperature(&self) -> f64 {
        self.inner.temperature()
    }
}
