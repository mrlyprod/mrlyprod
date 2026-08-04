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

#[wasm_bindgen]
pub struct Handle {
    os: Os,
}

#[wasm_bindgen]
pub fn boot() -> Handle {
    Handle {
        os: mrlyweb::registry::boot(),
    }
}

#[wasm_bindgen]
pub fn list(handle: &Handle, shape: Option<String>) -> String {
    let shape = shape_of(shape);
    handle.os.list(shape.as_ref()).to_string()
}

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

#[wasm_bindgen]
pub fn read(handle: &Handle, path: &str, shape: Option<String>) -> String {
    let shape = shape_of(shape);
    text_of(handle.os.read(path, shape.as_ref()))
}
