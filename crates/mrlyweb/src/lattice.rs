use mrlycore::{json, Json};
use mrlynum::lattice;
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
