use crate::{code_of, Fault};
use mrlycore::{json, Json};
use mrlymath::six;
use mrlymath::three;
use mrlymath::two;
use mrlynum::graph::{census, largest_component, Network};
use mrlynum::spectrum as spectra;
use wasm_bindgen::prelude::*;

const LIMIT: usize = 1100;
const TOLERANCE: f64 = 1e-9;
const TOP: usize = 8;

fn graph_of(
    kind: &str,
    code: &str,
    number: usize,
    level: usize,
) -> Result<(Network, usize), Fault> {
    let code = code_of(code)?;
    match kind {
        "flat" => {
            let whole = two::graph::core_graph(&two::create(code, number, level, 0, 2)?)?;
            let pieces = census::components(&whole);
            Ok((whole, pieces))
        }
        "slice" => {
            let cell = six::cut(&three::create(code, number, level, 2)?)?;
            let whole = six::graph::slice_core_graph(&cell)?;
            let pieces = census::components(&whole);
            Ok((largest_component(&whole), pieces))
        }
        _ => Err(Fault::new(format!(
            "kind {kind:?} is neither \"flat\" nor \"slice\"."
        ))),
    }
}

/// Diagonalises the Laplacian of a design's graph: the spectrum, its clusters, the pinned multiplicities and the spectral exponent, as JSON.
///
/// The kind is `"flat"` for the cell graph of a two-dimensional design or `"slice"` for the
/// giant piece of a cube's diagonal section, whose whole-section piece count is reported
/// alongside. The Laplacian is normalised or combinatorial, clustering runs at `1e-9`, and
/// anything over 1100 nodes is refused. The fit is the log-log intercept and slope the
/// exponent doubles, over the first `fitted` of the `stair` points the page draws.
#[wasm_bindgen]
pub fn spectrum(
    kind: &str,
    code: &str,
    number: usize,
    level: usize,
    normalised: bool,
    window: f64,
) -> Result<String, Fault> {
    let (network, components) = graph_of(kind, code, number, level)?;
    let nodes = network.nodes.len();
    if nodes > LIMIT {
        return Err(Fault::new(format!(
            "{nodes} nodes is past the {LIMIT} the spectrum allows."
        )));
    }
    let eigenvalues = spectra::laplacian_spectrum(&network, normalised)?;
    let groups = spectra::clusters(&eigenvalues, TOLERANCE);
    let fit = spectra::spectral_fit(&eigenvalues, window);
    let repeated: usize = groups.iter().filter(|g| g.1 > 1).map(|g| g.1).sum();
    let fraction = if nodes == 0 {
        0.0
    } else {
        (repeated as f64 / nodes as f64 * 10000.0).round() / 10000.0
    };
    let root = 30f64.sqrt() / 6.0;
    let top: Vec<Vec<Json>> = groups
        .iter()
        .rev()
        .take(TOP)
        .map(|(value, size)| vec![json!(value), json!(size)])
        .collect();
    Ok(json!({
        "nodes": nodes,
        "edges": network.branches.len(),
        "components": components,
        "eigenvalues": eigenvalues,
        "distinct": groups.len(),
        "classes": groups.iter().filter(|g| g.1 > 1).count(),
        "repeated": fraction,
        "one": spectra::multiplicity(&eigenvalues, 1.0, TOLERANCE),
        "pair": [
            spectra::multiplicity(&eigenvalues, 1.0 - root, TOLERANCE),
            spectra::multiplicity(&eigenvalues, 1.0 + root, TOLERANCE),
        ],
        "exponent": fit.map(|(_, slope, _)| 2.0 * slope),
        "fit": fit.map(|(a, b, _)| vec![a, b]),
        "fitted": fit.map(|(_, _, count)| count),
        "stair": spectra::spectral_points(&eigenvalues),
        "top": top,
    })
    .to_string())
}
