use crate::{ink, Fault, Pixels};
use mrlycore::{json, Json};
use mrlynum::factor::gcd;
use mrlynum::{lattice, series};
use wasm_bindgen::prelude::*;

/// Walks the Farey sequence of the order: each node's numerator, denominator and brightness, as JSON.
#[wasm_bindgen]
pub fn farey(order: usize) -> String {
    let nodes: Vec<Json> = lattice::farey(order)
        .iter()
        .map(|node| json!([node.num, node.den, node.brightness]))
        .collect();
    Json::Array(nodes).to_string()
}

/// Sieves the totients of zero through the limit.
#[wasm_bindgen]
pub fn totients(limit: usize) -> Vec<u32> {
    lattice::totients(limit).iter().map(|&v| v as u32).collect()
}

/// Reads the Farey stack of the order: the nodes the walk lit, one plus the totients summed, whether the two agree, and the primes as the scales of maximal novelty, as JSON.
#[wasm_bindgen]
pub fn farey_novelty(order: usize) -> String {
    let phi = lattice::totients(order);
    let novel = 1 + phi.iter().skip(1).sum::<u64>();
    let lit = lattice::farey(order).len() as u64;
    let primes: Vec<usize> = (2..=order).filter(|&n| phi[n] == n as u64 - 1).collect();
    json!({
        "lit": lit,
        "novel": novel,
        "match": lit == novel,
        "primes": primes,
    })
    .to_string()
}

// VISIBLE

const WINDOW: usize = 4000;
const SHEET: usize = 2048;
const STOPS: usize = 512;

fn window(n: usize) -> Result<usize, Fault> {
    if n == 0 || n > WINDOW {
        return Err(Fault::new(format!(
            "the window must be between 1 and {WINDOW}."
        )));
    }
    Ok(n)
}

fn depth(dimension: u32) -> Result<u32, Fault> {
    if !(2..=8).contains(&dimension) {
        return Err(Fault::new("the dimension must be between 2 and 8."));
    }
    Ok(dimension)
}

fn shade(layer: usize, layers: bool) -> [u8; 4] {
    if layer == 1 {
        return ink::BLUE;
    }
    if !layers {
        return ink::FAINT;
    }
    let t = 1.0 / layer as f64;
    let step = |ground: u8, tone: u8| {
        (f64::from(ground) + (f64::from(tone) - f64::from(ground)) * t).round() as u8
    };
    [
        step(ink::DEEP[0], ink::DIM[0]),
        step(ink::DEEP[1], ink::DIM[1]),
        step(ink::DEEP[2], ink::DIM[2]),
        255,
    ]
}

/// Reads the window the stack lights in the dimension: the lit points, their density, the limit one over zeta, the constant the count recovers, the value it walks to and the gap between them, as JSON.
#[wasm_bindgen]
pub fn visible_read(n: usize, dimension: u32) -> Result<String, Fault> {
    let (n, dimension) = (window(n)?, depth(dimension)?);
    let lit = series::visible(n, dimension);
    let total = (n as u128).pow(dimension);
    let constant = lattice::recovered(n, dimension);
    let even = dimension.is_multiple_of(2);
    let truth = if even {
        std::f64::consts::PI
    } else {
        lattice::zeta_whole(dimension)
    };
    Ok(json!({
        "n": n,
        "dimension": dimension,
        "lit": lit.to_string(),
        "total": total.to_string(),
        "density": lit as f64 / total as f64,
        "limit": lattice::visible_density(dimension),
        "name": if even { "pi".to_string() } else { format!("zeta({dimension})") },
        "constant": constant,
        "truth": truth,
        "error": (constant - truth).abs(),
    })
    .to_string())
}

/// Paints the corner window of the plane lattice at the pixel side asked for, the origin at the lower left: a point of coprime coordinates in blue, a hidden point in the dim of the stack layer that owns it, flat when the layers are off.
#[wasm_bindgen]
pub fn visible_pixels(n: usize, side: usize, layers: bool) -> Result<Pixels, Fault> {
    let n = window(n)?;
    if !(16..=SHEET).contains(&side) {
        return Err(Fault::new(format!(
            "the side must be between 16 and {SHEET} pixels."
        )));
    }
    let mut colors = Vec::with_capacity(side * side);
    for py in 0..side {
        let b = n - py * n / side;
        for px in 0..side {
            let a = px * n / side + 1;
            colors.push(shade(gcd(a, b), layers));
        }
    }
    Ok(Pixels::of(side, side, colors))
}

/// Walks the window up to n at the count of stops and returns each stop as a window and the constant its count recovers, two numbers a stop, so the approach can be drawn.
#[wasm_bindgen]
pub fn visible_walk(n: usize, dimension: u32, stops: usize) -> Result<Vec<f64>, Fault> {
    let (n, dimension) = (window(n)?, depth(dimension)?);
    if !(2..=STOPS).contains(&stops) {
        return Err(Fault::new(format!(
            "the stops must be between 2 and {STOPS}."
        )));
    }
    let stops = stops.min(n);
    let mut out = Vec::with_capacity(stops * 2);
    for k in 1..=stops {
        let at = (n * k / stops).max(1);
        out.push(at as f64);
        out.push(lattice::recovered(at, dimension));
    }
    Ok(out)
}
