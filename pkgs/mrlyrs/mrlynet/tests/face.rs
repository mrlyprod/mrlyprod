use mrlyos::kernel::{Call, Iden, Os};
use mrlyui::face::{FaceInput, FaceVerb};
use serde_json::{json, Value};
use std::fs;

fn boot() -> Os {
    let mut os = Os::new(Iden::new("guest"));
    for app in mrlynet::registry::catalogue() {
        os = os.install(app);
    }
    os
}

fn png_dims(png: &[u8]) -> (usize, usize) {
    let w = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
    let h = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
    (w as usize, h as usize)
}

fn seeded_snake(os: &mut Os) {
    os.act(Call::new("nav.open", json!({ "app": "snake" })));
    os.act(Call::new("snake.reset", json!({ "seed": 7 })));
    os.act(Call::new("snake.step", json!({ "n": 2 })));
}

#[test]
fn every_app_face_renders() {
    let mut os = boot();
    for route in os.catalogue() {
        os.open(&route).unwrap();
        let png = mrlynet::face::face_png(&os, &route).unwrap();
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n", "{route}");
        let (w, h) = png_dims(&png);
        assert_eq!(w, mrlyui::face::WIDTH * mrlyui::face::SCALE, "{route}");
        let range = mrlyui::face::MIN_HEIGHT * mrlyui::face::SCALE
            ..=mrlyui::face::MAX_HEIGHT * mrlyui::face::SCALE;
        assert!(range.contains(&h), "{route}");
        assert!(png.len() < 2 * 1024 * 1024, "{route}");
    }
}

#[test]
fn every_fixture_face_renders() {
    let dir = format!("{}/../../../apps/web/fixtures", env!("CARGO_MANIFEST_DIR"));
    let mut count = 0;
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let fixture: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let view = &fixture["view"];
        let app = view["app"].as_str().unwrap().to_string();
        let actions = view["actions"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|v| FaceVerb {
                name: v["verb"].as_str().unwrap_or("").to_string(),
                args: v["args"].clone(),
            })
            .collect();
        let input = FaceInput {
            app: app.clone(),
            title: app,
            params: view["params"].clone(),
            state: view["state"].clone(),
            actions,
            beat: view["beat"]["verb"].as_str().map(str::to_string),
            dark: false,
        };
        let png = mrlyui::face::face_png(&input).unwrap();
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n", "{}", path.display());
        count += 1;
    }
    assert!(count >= 40);
}

#[test]
fn faces_are_deterministic() {
    let a = boot();
    let b = boot();
    assert_eq!(
        mrlynet::face::face_png(&a, "menu").unwrap(),
        mrlynet::face::face_png(&b, "menu").unwrap()
    );
    let mut a = boot();
    seeded_snake(&mut a);
    let mut b = boot();
    seeded_snake(&mut b);
    assert_eq!(
        mrlynet::face::face_png(&a, "snake").unwrap(),
        mrlynet::face::face_png(&b, "snake").unwrap()
    );
}

#[test]
fn the_two_shots_split() {
    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "calculator" })));
    assert!(os.snapshot("calculator").is_err());
    assert!(mrlynet::face::face_png(&os, "calculator").is_ok());
}

#[test]
fn canvas_rgba_matches_the_fact() {
    let mut os = boot();
    seeded_snake(&mut os);
    let view = os.peek("snake").unwrap();
    let fw = view.state["frame"]["width"].as_u64().unwrap() as usize;
    let fh = view.state["frame"]["height"].as_u64().unwrap() as usize;
    let (w, h, buf) = mrlynet::face::canvas_rgba(&os, "snake").unwrap();
    assert_eq!((w, h), (fw, fh));
    assert_eq!(buf.len(), w * h * 4);
    assert!(mrlynet::face::canvas_rgba(&os, "calculator").is_err());
}
