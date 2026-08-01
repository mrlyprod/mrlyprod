use mrlycore::json::Map;
use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Goose, Os};
use wasm_bindgen::prelude::*;

fn build() -> Os {
    mrlyweb::registry::boot()
}

fn shape_of(text: Option<String>) -> Option<Json> {
    let text = text?;
    if text.trim().is_empty() {
        return None;
    }
    mrlycore::json::parse(&text).ok()
}

#[wasm_bindgen]
pub struct Handle {
    os: Os,
    goose: Option<(u64, Goose)>,
}

#[wasm_bindgen]
pub fn boot() -> Handle {
    Handle {
        os: build(),
        goose: None,
    }
}

#[wasm_bindgen]
pub fn act(handle: &mut Handle, req: &str) -> String {
    let parsed = mrlycore::json::parse(req).unwrap_or(json!({}));
    let verb = parsed["verb"].as_str().unwrap_or("").to_string();
    let args = if parsed["args"].is_object() {
        parsed["args"].clone()
    } else {
        json!({})
    };
    let mut call = Call::new(&verb, args);
    if let Some(now) = parsed["now"].as_i64() {
        call = call.at(now);
    }
    handle.os.act(call);
    frame(handle, None)
}

#[wasm_bindgen]
pub fn goose(handle: &mut Handle, seed: u64) -> String {
    if handle.goose.as_ref().map(|(s, _)| *s) != Some(seed) {
        handle.goose = Some((seed, Goose::new(seed)));
    }
    if let Some((_, goose)) = handle.goose.as_mut() {
        goose.step(&mut handle.os);
    }
    frame(handle, None)
}

#[wasm_bindgen]
pub fn frame(handle: &Handle, shape: Option<String>) -> String {
    let shape = shape_of(shape);
    handle.os.frame(shape.as_ref()).to_json().to_string()
}

#[wasm_bindgen]
pub fn geometry(handle: &Handle, app: &str) -> Option<Vec<f32>> {
    handle.os.geometry(app)
}

#[wasm_bindgen]
pub fn uniforms(handle: &Handle, app: &str) -> Option<Vec<f32>> {
    handle.os.uniforms(app)
}

#[wasm_bindgen]
pub fn peek(handle: &Handle, app: &str, shape: Option<String>) -> String {
    let shape = shape_of(shape);
    match handle.os.peek(app, shape.as_ref()) {
        Some(view) => view.to_json().to_string(),
        None => "null".to_string(),
    }
}

#[wasm_bindgen]
pub fn describe(shape: Option<String>) -> String {
    let shape = shape_of(shape);
    build().describe(shape.as_ref()).to_string()
}

#[wasm_bindgen]
pub fn palette() -> String {
    use mrlycore::colors::{BOARD_DARK, BOARD_LIGHT, NAMES, PALETTE};
    let mut hex = Map::new();
    for (name, color) in NAMES.iter().zip(PALETTE.iter()) {
        hex.insert(name.to_string(), json!(color.to_hex()));
    }
    json!({
        "names": NAMES.to_vec(),
        "hex": hex,
        "canvas": { "dark": BOARD_DARK.to_hex(), "light": BOARD_LIGHT.to_hex() },
    })
    .to_string()
}

#[wasm_bindgen]
pub fn html(md: &str) -> String {
    mrlycore::md::html(md)
}

#[wasm_bindgen]
pub fn shaders() -> String {
    let mut out = Map::new();
    for (name, source) in mrlyui::shaders::all() {
        out.insert(name.to_string(), json!(source));
    }
    Json::Obj(out).to_string()
}

#[wasm_bindgen]
pub fn face(handle: &Handle, app: &str) -> Option<Vec<u8>> {
    mrlyweb::face::face_rgba(&handle.os, app)
        .ok()
        .map(|(_, _, rgba)| rgba)
}

#[wasm_bindgen]
pub fn sheet() -> Vec<u32> {
    vec![mrlyui::face::WIDTH as u32, mrlyui::face::HEIGHT as u32]
}

#[wasm_bindgen]
pub fn mark() -> String {
    json!({
        "rows": mrlyui::mark::ROWS,
        "cols": mrlyui::mark::COLS,
        "fps": mrlyui::mark::FPS,
        "frames": mrlyui::mark::animation(),
    })
    .to_string()
}
