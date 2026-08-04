use mrlymath::space::{Pack, MESH_WGSL, TURN};
use mrlymath::three::{
    carpet, net, quads, sheet as plate, sheet_edges, void, wires, xtree, ytree, ztree, Cell3d,
};
use std::f64::consts::TAU;
use wasm_bindgen::prelude::*;

const NUMBERS: [usize; 4] = [3, 5, 7, 9];
const MAX_CELLS: usize = 32;
const SEGMENTS: usize = 16;
const BANDS: f64 = 4.0;
const LIGHT_YAW: i64 = 72;
const LIGHT_PITCH: i64 = 28;
const INK: [u8; 4] = [0, 0, 0, 255];

// HELPERS

fn tint(bytes: &[u8]) -> [u8; 4] {
    let mut out = INK;
    for (slot, c) in out.iter_mut().zip(bytes.iter()) {
        *slot = *c;
    }
    out
}

fn rgb(bytes: &[u8]) -> [f64; 3] {
    let mut out = [0.0; 3];
    for (slot, c) in out.iter_mut().zip(bytes.iter()) {
        *slot = *c as f64 / 255.0;
    }
    out
}

fn snapped(number: u32) -> usize {
    *NUMBERS
        .iter()
        .min_by_key(|&&n| (n as i64 - number as i64).abs())
        .unwrap()
}

fn deep(number: usize, level: u32) -> usize {
    let mut level = level.max(1) as usize;
    while level > 1 && number.saturating_pow(level as u32) > MAX_CELLS {
        level -= 1;
    }
    level
}

fn grid(design: &str, number: u32, level: u32) -> Option<Cell3d> {
    let n = snapped(number);
    let l = deep(n, level);
    match design {
        "carpet" => carpet(n, l),
        "net" => net(n, l),
        "xtree" => xtree(n, l),
        "ytree" => ytree(n, l),
        "ztree" => ztree(n, l),
        _ => void(n, l),
    }
    .ok()
}

// MESHES

#[wasm_bindgen]
pub fn brick(design: &str, number: u32, level: u32, edges: bool, edge_rgba: &[u8]) -> Vec<f32> {
    let mut pack = Pack::new();
    if let Some(cell) = grid(design, number, level) {
        for quad in quads(&cell) {
            pack.quad(quad.verts, quad.normal);
        }
        if edges {
            for [a, b] in wires(&cell) {
                pack.line(a, b, true, tint(edge_rgba));
            }
        }
    }
    pack.buffer()
}

#[wasm_bindgen]
pub fn sheet(
    cols: u32,
    rows: u32,
    diameter: f32,
    thickness: f32,
    edges: bool,
    edge_rgba: &[u8],
) -> Vec<f32> {
    let mut pack = Pack::new();
    for quad in plate(cols as usize, rows as usize, diameter, thickness, SEGMENTS) {
        pack.quad(quad.verts, quad.normal);
    }
    if edges {
        for [a, b] in sheet_edges(thickness) {
            pack.line(a, b, true, tint(edge_rgba));
        }
    }
    pack.buffer()
}

// SHADING

#[wasm_bindgen]
pub fn uniforms(
    yaw: i32,
    pitch: i32,
    dist: f32,
    pan_x: f32,
    pan_y: f32,
    ortho: bool,
    alpha: u8,
    board_rgb: &[u8],
    fill_rgb: &[u8],
) -> Vec<f32> {
    let rad = TAU / TURN as f64;
    let board = rgb(board_rgb);
    let fill = rgb(fill_rgb);
    let mut u = vec![0.0f64; 24];
    u[4] = board[0];
    u[5] = board[1];
    u[6] = board[2];
    u[8] = fill[0];
    u[9] = fill[1];
    u[10] = fill[2];
    u[11] = BANDS;
    u[13] = yaw as f64 * rad;
    u[14] = pitch as f64 * rad;
    u[15] = dist as f64 / 4.0;
    u[16] = pan_x as f64 / 16.0;
    u[17] = pan_y as f64 / 16.0;
    u[18] = if ortho { 1.0 } else { 0.0 };
    u[19] = alpha as f64 / 255.0;
    u[20] = LIGHT_YAW as f64 * rad;
    u[21] = LIGHT_PITCH as f64 * rad;
    u.into_iter().map(|v| v as f32).collect()
}

#[wasm_bindgen]
pub fn shader() -> String {
    MESH_WGSL.to_string()
}

// RIG

#[wasm_bindgen]
pub fn turn() -> u32 {
    TURN as u32
}

#[wasm_bindgen]
pub fn pose() -> Vec<f32> {
    vec![32.0, 20.0, 12.0, 0.0, 0.0, 1.0]
}
