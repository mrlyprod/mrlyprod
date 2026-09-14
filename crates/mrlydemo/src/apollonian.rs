use crate::Fault;
use mrlycore::{json, Json};
use mrlynum::apollonian as gasket;
use mrlynum::apollonian::{Circle, Packing};
use wasm_bindgen::prelude::*;

const STRIDE: usize = 5;

fn drawn(c: Circle) -> [f64; STRIDE] {
    let (x, y) = c.centre().unwrap_or((0.0, 0.0));
    let den = if gasket::is_ford(c) {
        ((c.k / 2) as f64).sqrt().round()
    } else {
        0.0
    };
    [x, y, c.radius().unwrap_or(0.0), c.k as f64, den]
}

fn sheet(circles: impl Iterator<Item = Circle>) -> Vec<f64> {
    let mut out = Vec::new();
    for c in circles.filter(|c| !c.is_line()) {
        out.extend_from_slice(&drawn(c));
    }
    out
}

fn line_tangent(p: &Packing) -> (usize, usize) {
    let on = p.circles.iter().filter(|c| gasket::on_line(**c)).count();
    let ford = p.circles.iter().filter(|c| gasket::is_ford(**c)).count();
    (on, ford)
}

/// Grows the named packing to the curvature cap and returns the circles it made, the root quadruple excluded, five doubles each: the centre, the radius, the curvature, and the denominator of the fraction the circle rests on when it is a Ford circle, zero when it is not. Lines carry no centre and are left out.
#[wasm_bindgen]
pub fn apollonian(root: &str, cap: u32) -> Result<Vec<f64>, Fault> {
    let p = gasket::grow(root, i64::from(cap))?;
    Ok(sheet(p.circles.into_iter()))
}

/// The finite circles of the named root quadruple, in the encoding of `apollonian`.
#[wasm_bindgen]
pub fn apollonian_root(root: &str) -> Result<Vec<f64>, Fault> {
    Ok(sheet(gasket::root(root)?.into_iter()))
}

/// The tangency points the packing rests on the line `y = 0`, ascending, four doubles each: the point, the numerator and the denominator of the reduced fraction it is, and the curvature of the circle resting there. Empty off the strip.
#[wasm_bindgen]
pub fn apollonian_touches(root: &str, cap: u32) -> Result<Vec<f64>, Fault> {
    let p = gasket::grow(root, i64::from(cap))?;
    let mut out = Vec::new();
    for t in gasket::touches(&p) {
        out.extend_from_slice(&[
            t.num as f64 / t.den as f64,
            t.num as f64,
            t.den as f64,
            t.k as f64,
        ]);
    }
    Ok(out)
}

/// Reads the packing: its root's curvatures, whether it is the strip, the census `N(T)` with the root excluded, the quadruples grown and how many failed one of the six invariants, the circles centred outside the period, the line-tangent circles and how many of them are Ford circles, the box the picture sits in, the local census exponent read over the two octaves below the cap, and the Farey stack of the depth read against the tangency points, as JSON.
#[wasm_bindgen]
pub fn apollonian_read(root: &str, cap: u32, order: usize) -> Result<String, Fault> {
    let cap = i64::from(cap);
    let p = gasket::grow(root, cap)?;
    let shadow = gasket::shadow(&p, order)?;
    let (on, ford) = line_tangent(&p);
    let roots: Vec<Json> = p.root.iter().map(|c| json!(c.k)).collect();
    let finite = p.root.iter().filter(|c| !c.is_line()).count();
    let back = cap / 4;
    let exponent = if back >= 2 {
        let old = gasket::grow(root, back)?.circles.len();
        if old > 0 && !p.circles.is_empty() {
            json!((p.circles.len() as f64 / old as f64).ln() / 4f64.ln())
        } else {
            Json::Null
        }
    } else {
        Json::Null
    };
    Ok(json!({
        "root": root,
        "curvatures": roots,
        "strip": p.strip,
        "cap": cap,
        "circles": p.circles.len(),
        "drawn": p.circles.len() + finite,
        "quads": p.quads,
        "broken": p.broken,
        "strayed": p.strayed,
        "line": on,
        "ford": ford,
        "frame": gasket::frame(&p).to_vec(),
        "exponent": exponent,
        "from": back,
        "shadow": {
            "order": shadow.order,
            "reach": shadow.reach,
            "covered": shadow.covered,
            "nodes": shadow.nodes,
            "touched": shadow.touched,
            "missed": shadow.missed,
            "offford": shadow.offford,
            "bright": shadow.bright as u64,
            "want": shadow.want as u64,
        },
    })
    .to_string())
}

/// The caps: the largest curvature a packing is grown to, the most circles one growth makes, the deepest the stack is read, and the root quadruples on offer, as JSON.
#[wasm_bindgen]
pub fn apollonian_caps() -> String {
    let roots: Vec<Json> = gasket::ROOTS.iter().map(|&name| json!(name)).collect();
    json!({
        "curvature": gasket::CURVATURE_CAP,
        "circles": gasket::CIRCLE_CAP,
        "order": gasket::ORDER_CAP,
        "roots": roots,
    })
    .to_string()
}
