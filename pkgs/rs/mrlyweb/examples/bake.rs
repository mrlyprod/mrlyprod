use mrlycore::json::Map;
use mrlycore::{json, Json};
use std::fs;

fn write(home: &str, name: &str, value: &Json) {
    let dir = format!("{}/../../../{home}", env!("CARGO_MANIFEST_DIR"));
    fs::create_dir_all(&dir).unwrap();
    let path = format!("{dir}/{name}.json");
    fs::write(&path, value.to_string() + "\n").unwrap();
    println!("wrote {path}");
}

const WEB: &str = "sites/web/src/gen";

const UI: &str = "pkgs/js/mrlyui/src/gen";

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
    for program in mrlyweb::registry::programs() {
        out.insert(
            program.name.to_string(),
            json!(mrlyweb::shaders::assemble(&program)),
        );
    }
    Json::Obj(out)
}

fn rigs() -> Json {
    let mut out = Map::new();
    for rig in mrlyweb::registry::rigs() {
        out.insert(
            rig.route.to_string(),
            json!({
                "yaw": rig.yaw,
                "pitch": rig.pitch,
                "dist": rig.dist,
                "pan": [rig.pan[0], rig.pan[1]],
                "ortho": rig.ortho,
            }),
        );
    }
    json!({ "turn": mrlycore::trig::N, "rigs": Json::Obj(out) })
}

fn mark() -> Json {
    const WORDMARK: &str = "MRLYPROD";
    let write = mrlyfont::animate(WORDMARK, 1);
    let anim = mrlyfont::cycle(&write, &mrlyfont::merge(WORDMARK, 1), mrlyfont::HOLD);
    json!({
        "rows": anim.rows,
        "cols": anim.cols,
        "fps": anim.fps,
        "frames": anim.frames,
    })
}

fn main() {
    write(WEB, "palette", &palette());
    write(WEB, "shaders", &shaders());
    write(WEB, "skins", &mrlyweb::registry::corpus());
    write(WEB, "rigs", &rigs());
    write(UI, "mark", &mark());
}
