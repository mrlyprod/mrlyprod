use mrlycore::json;
use mrlyos::kernel::{Call, Os};
use mrlyui::face::{FaceInput, FaceVerb};
use std::fs;

fn boot() -> Os {
    mrlyweb::registry::boot()
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
        let png = mrlyweb::face::face_png(&os, &route).unwrap();
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n", "{route}");
        let (w, h) = png_dims(&png);
        assert_eq!(w, mrlyui::face::WIDTH * mrlyui::face::SCALE, "{route}");
        assert_eq!(h, mrlyui::face::HEIGHT * mrlyui::face::SCALE, "{route}");
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
        let fixture = mrlycore::json::parse(&fs::read_to_string(&path).unwrap()).unwrap();
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
            ui: None,
            rung: mrlyui::tokens::RUNG,
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
        mrlyweb::face::face_png(&a, "menu").unwrap(),
        mrlyweb::face::face_png(&b, "menu").unwrap()
    );
    let mut a = boot();
    seeded_snake(&mut a);
    let mut b = boot();
    seeded_snake(&mut b);
    assert_eq!(
        mrlyweb::face::face_png(&a, "snake").unwrap(),
        mrlyweb::face::face_png(&b, "snake").unwrap()
    );
}

#[test]
fn the_two_shots_split() {
    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "calculator" })));
    assert!(os.snapshot("calculator").is_err());
    assert!(mrlyweb::face::face_png(&os, "calculator").is_ok());
}

#[test]
fn canvas_rgba_matches_the_fact() {
    let mut os = boot();
    seeded_snake(&mut os);
    let view = os.peek("snake", None).unwrap();
    let fw = view.state["frame"]["width"].as_u64().unwrap() as usize;
    let fh = view.state["frame"]["height"].as_u64().unwrap() as usize;
    let (w, h, buf) = mrlyweb::face::canvas_rgba(&os, "snake").unwrap();
    assert_eq!((w, h), (fw, fh));
    assert_eq!(buf.len(), w * h * 4);
    assert!(mrlyweb::face::canvas_rgba(&os, "calculator").is_err());
}

#[test]
fn gpu_mode_still_shows_the_cpu_twin() {
    let mut driver = mrlyweb::drive::Driver::scripted(0);
    driver.open("julia");
    driver.act("julia.step", mrlycore::json!({ "n": 4 }));
    let cpu = driver.frame_fnv();
    driver.act(
        "settings.set",
        mrlycore::json!({ "key": "render", "value": "gpu" }),
    );
    driver.open("julia");
    assert_eq!(driver.frame_fnv(), cpu, "gpu mode blanked the face");
    let empty = mrlyui::face::decode(&mrlyui::frame::empty_fact(100, 100));
    assert!(empty.is_none(), "an empty fact must not decode");
}
