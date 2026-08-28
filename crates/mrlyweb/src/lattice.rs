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
