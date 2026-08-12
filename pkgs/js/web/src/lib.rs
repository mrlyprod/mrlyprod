use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use wasm_bindgen::prelude::*;

fn shape_of(text: Option<String>) -> Option<Json> {
    let text = text?;
    if text.trim().is_empty() {
        return None;
    }
    mrlycore::json::parse(&text).ok()
}

fn text_of(found: Option<Json>) -> String {
    match found {
        Some(json) => json.to_string(),
        None => "null".to_string(),
    }
}

/// The four-door handle on one booted world.
#[wasm_bindgen]
pub struct Handle {
    os: Os,
}

/// Boots the named loadout into a fresh world and returns its handle.
#[wasm_bindgen]
pub fn boot(loadout: &str) -> Handle {
    Handle {
        os: mrlyweb::registry::boot(loadout),
    }
}

/// Lists the version, the apps and every verb on offer as JSON text, pruned to the shape.
#[wasm_bindgen]
pub fn list(handle: &Handle, shape: Option<String>) -> String {
    let shape = shape_of(shape);
    handle.os.list(shape.as_ref()).to_string()
}

/// Runs a JSON call of verb, args and optional now, then returns the fresh whole envelope as JSON text.
#[wasm_bindgen]
pub fn call(handle: &mut Handle, req: &str) -> String {
    let parsed = mrlycore::json::parse(req).unwrap_or(json!({}));
    let verb = parsed["verb"].as_str().unwrap_or("").to_string();
    let args = if parsed["args"].is_object() {
        parsed["args"].clone()
    } else {
        json!({})
    };
    let mut made = Call::new(&verb, args);
    if let Some(now) = parsed["now"].as_i64() {
        made = made.at(now);
    }
    handle.os.call(made);
    text_of(handle.os.read("", None))
}

/// Reads the value at a slash path, from the whole envelope down to one drilled leaf, or "null" where nothing lives.
#[wasm_bindgen]
pub fn read(handle: &Handle, path: &str, shape: Option<String>) -> String {
    let shape = shape_of(shape);
    text_of(handle.os.read(path, shape.as_ref()))
}
