use mrlycore::json::Map;
use mrlycore::{json, Json};
use std::fs;

fn write(name: &str, value: &Json) {
    let dir = format!("{}/../../../apps/web/src/gen", env!("CARGO_MANIFEST_DIR"));
    fs::create_dir_all(&dir).unwrap();
    let path = format!("{dir}/{name}.json");
    fs::write(&path, value.to_string() + "\n").unwrap();
    println!("wrote {path}");
}

fn palette() -> Json {
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
}

fn shaders() -> Json {
    let mut out = Map::new();
    for (name, source) in mrlyui::shaders::all() {
        out.insert(name.to_string(), json!(source));
    }
    Json::Obj(out)
}

fn mark() -> Json {
    json!({
        "rows": mrlyui::mark::ROWS,
        "cols": mrlyui::mark::COLS,
        "fps": mrlyui::mark::FPS,
        "frames": mrlyui::mark::animation(),
    })
}

fn main() {
    write("palette", &palette());
    write("shaders", &shaders());
    write("mark", &mark());
    write("skins", &mrlyui::skin::corpus());
}
