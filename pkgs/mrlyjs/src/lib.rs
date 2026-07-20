use mrly::os::kernel::{Call, Iden, Os};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

fn build() -> Os {
    let mut os = Os::new(Iden::new("guest"));
    for app in mrly::net::registry::catalogue() {
        os = os.install(app);
    }
    os
}

#[wasm_bindgen]
pub struct Handle {
    os: Os,
}

#[wasm_bindgen]
pub fn boot() -> Handle {
    Handle { os: build() }
}

#[wasm_bindgen]
pub fn act(handle: &mut Handle, req: &str) -> String {
    let parsed: Value = serde_json::from_str(req).unwrap_or(json!({}));
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
    frame(handle)
}

#[wasm_bindgen]
pub fn frame(handle: &Handle) -> String {
    handle.os.frame().to_json().to_string()
}

#[wasm_bindgen]
pub fn geometry(handle: &Handle, app: &str) -> Option<Vec<f32>> {
    handle.os.geometry(app)
}

#[wasm_bindgen]
pub fn peek(handle: &Handle, app: &str) -> String {
    match handle.os.peek(app) {
        Some(view) => view.to_json().to_string(),
        None => "null".to_string(),
    }
}

#[wasm_bindgen]
pub fn describe() -> String {
    build().describe().to_string()
}

#[wasm_bindgen]
pub fn palette() -> String {
    use mrly::core::colors::{BOARD_DARK, BOARD_LIGHT, NAMES, PALETTE};
    let mut hex = serde_json::Map::new();
    for (name, color) in NAMES.iter().zip(PALETTE.iter()) {
        hex.insert(name.to_string(), json!(color.to_hex()));
    }
    json!({
        "names": NAMES,
        "hex": hex,
        "canvas": { "dark": BOARD_DARK.to_hex(), "light": BOARD_LIGHT.to_hex() },
    })
    .to_string()
}

#[wasm_bindgen]
pub fn html(md: &str) -> String {
    mrly::core::md::html(md)
}

#[wasm_bindgen]
pub fn shaders() -> String {
    let mut out = serde_json::Map::new();
    for (name, source) in mrly::ui::shaders::all() {
        out.insert(name.to_string(), json!(source));
    }
    Value::Object(out).to_string()
}

#[wasm_bindgen]
pub fn mark() -> String {
    json!({
        "rows": mrly::ui::mark::ROWS,
        "cols": mrly::ui::mark::COLS,
        "fps": mrly::ui::mark::FPS,
        "frames": mrly::ui::mark::animation(),
    })
    .to_string()
}
