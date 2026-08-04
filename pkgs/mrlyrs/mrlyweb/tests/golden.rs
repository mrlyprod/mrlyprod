use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Iden, Os};
use std::fs;

fn boot() -> Os {
    mrlyweb::registry::boot()
}

fn fixture(name: &str) -> Json {
    let path = format!(
        "{}/../../../apps/web/fixtures/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    mrlycore::json::parse(&fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn shader_sources_are_complete() {
    let all = mrlyui::shaders::all();
    assert!(!all.is_empty());
    for (name, source) in all {
        assert!(source.contains("fn vs_main"), "{name} misses vs_main");
        assert!(source.contains("fn fs_main"), "{name} misses fs_main");
    }
}

#[test]
fn every_shade_resolves_a_program() {
    let iden = Iden::new("guest");
    let mut shaded = 0;
    for app in mrlyweb::registry::catalogue() {
        let state = app.state(&iden, None);
        let shade = &state["shade"];
        if shade.is_null() {
            continue;
        }
        let program = shade["program"].as_str().expect("shade.program");
        let floats = mrlyui::shaders::floats(program)
            .unwrap_or_else(|| panic!("{} names unknown program {program}", app.route()));
        assert_eq!(
            app.uniforms().expect("uniforms").len(),
            floats,
            "{} uniforms disagree with {program}",
            app.route()
        );
        shaded += 1;
    }
    assert!(shaded >= 2);
}

#[test]
fn menu_frame_is_golden() {
    let os = boot();
    assert_eq!(os.envelope(None).to_json(), fixture("menu"));
}

#[test]
fn calculator_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "calculator" })));
    os.call(Call::new("calculator.digit", json!({ "d": 4 })));
    os.call(Call::new("calculator.digit", json!({ "d": 2 })));
    assert_eq!(os.envelope(None).to_json(), fixture("calculator"));
}

#[test]
fn notes_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "notes" })));
    for text in ["buy oat milk", "book the ferry", "read the franel paper"] {
        os.call(Call::new("notes.add", json!({ "text": text })));
    }
    assert_eq!(os.envelope(None).to_json(), fixture("notes"));
}

#[test]
fn settings_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "settings" })));
    os.call(Call::new(
        "settings.set",
        json!({ "key": "launchpad", "value": "list" }),
    ));
    os.call(Call::new(
        "settings.set",
        json!({ "key": "radius", "value": 3 }),
    ));
    os.call(Call::new(
        "settings.set",
        json!({ "key": "scale", "value": 4 }),
    ));
    assert_eq!(os.envelope(None).to_json(), fixture("settings"));
}

#[test]
fn ui_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "ui" })));
    os.call(Call::new(
        "ui.set",
        json!({ "key": "pick", "value": "beta" }),
    ));
    os.call(Call::new(
        "ui.set",
        json!({ "key": "overlay", "value": true }),
    ));
    assert_eq!(os.envelope(None).to_json(), fixture("ui"));
}

#[test]
fn life_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "life" })));
    os.call(Call::new("life.step", json!({})));
    os.call(Call::new("life.step", json!({})));
    assert_eq!(os.envelope(None).to_json(), fixture("life"));
}

#[test]
fn clock_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "clock" })));
    os.call(Call::new("clock.tick", json!({})).at(1783600496000));
    assert_eq!(os.envelope(None).to_json(), fixture("clock"));
}

#[test]
fn timer_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "timer" })));
    os.call(Call::new("timer.start", json!({ "secs": 60 })).at(1783600496000));
    os.call(Call::new("timer.check", json!({})).at(1783600556000));
    assert_eq!(os.envelope(None).to_json(), fixture("timer"));
}

#[test]
fn calendar_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "calendar" })));
    os.call(Call::new(
        "calendar.goto",
        json!({ "year": 2026, "month": 6 }),
    ));
    os.call(Call::new("calendar.flip", json!({ "n": -1 })));
    os.call(Call::new("calendar.today", json!({})).at(1783600496000));
    assert_eq!(os.envelope(None).to_json(), fixture("calendar"));
}

#[test]
fn dice_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "dice" })));
    os.call(Call::new("dice.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "dice.set",
        json!({ "key": "sides", "value": 20 }),
    ));
    os.call(Call::new("dice.roll", json!({})));
    os.call(Call::new("dice.roll", json!({})));
    assert_eq!(os.envelope(None).to_json(), fixture("dice"));
}

#[test]
fn photos_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "life" })));
    os.call(Call::new("sys.shot", json!({})));
    os.call(Call::new("nav.open", json!({ "app": "two" })));
    os.call(Call::new("sys.shot", json!({})));
    os.call(Call::new("nav.open", json!({ "app": "photos" })));
    assert_eq!(os.envelope(None).to_json(), fixture("photos"));
}

#[test]
fn shot_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "two" })));
    os.call(Call::new("sys.shot", json!({})));
    os.call(Call::new("nav.open", json!({ "app": "photos" })));
    assert_eq!(os.envelope(None).to_json(), fixture("shot"));
}

#[test]
fn snake_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "snake" })));
    os.call(Call::new("snake.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "snake.set",
        json!({ "key": "head", "value": {
            "v": 1,
            "tile": {
                "v": 1, "group": "General", "factor": 3,
                "sources": [{ "design": "Net" }],
                "numbers": [3], "levels": [1], "rotations": [1], "anti": [false],
                "invert": false, "flip": false, "base": 2, "width": 3, "height": 3,
            },
            "paint": {
                "v": 1, "edition": "Simple", "scheme": "Multicolor", "target": "Fill",
                "primary": "Black", "secondary": ["Red"], "shades": [],
            },
        } }),
    ));
    os.call(Call::new("snake.turn", json!({ "dir": "left" })));
    os.call(Call::new("snake.step", json!({})));
    os.call(Call::new("snake.turn", json!({ "dir": "up" })));
    os.call(Call::new("snake.step", json!({ "n": 2 })));
    assert_eq!(os.envelope(None).to_json(), fixture("snake"));
}

#[test]
fn julia_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "julia" })));
    os.call(Call::new("julia.reset", json!({ "seed": 7 })));
    os.call(Call::new("julia.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("julia"));
}

#[test]
fn mandelbrot_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "mandelbrot" })));
    os.call(Call::new("mandelbrot.reset", json!({ "seed": 7 })));
    os.call(Call::new("mandelbrot.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("mandelbrot"));
}

#[test]
fn matrix_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "matrix" })));
    os.call(Call::new("matrix.reset", json!({ "seed": 7 })));
    os.call(Call::new("matrix.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("matrix"));
}

#[test]
fn sleep_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "sleep" })));
    os.call(Call::new("sleep.reset", json!({ "seed": 7 })));
    os.call(Call::new("sleep.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("sleep"));
}

#[test]
fn ttt_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "ttt" })));
    os.call(Call::new("ttt.reset", json!({ "seed": 7 })));
    os.call(Call::new("ttt.place", json!({ "cell": 0 })));
    os.call(Call::new("ttt.place", json!({ "cell": 4 })));
    assert_eq!(os.envelope(None).to_json(), fixture("ttt"));
}

#[test]
fn memory_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "memory" })));
    os.call(Call::new("memory.reset", json!({ "seed": 7 })));
    for _ in 0..16 {
        os.call(Call::new("memory.tick", json!({})));
    }
    os.call(Call::new("memory.flip", json!({ "card": 0 })));
    os.call(Call::new("memory.flip", json!({ "card": 1 })));
    assert_eq!(os.envelope(None).to_json(), fixture("memory"));
}

#[test]
fn mines_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "mines" })));
    os.call(Call::new("mines.reset", json!({ "seed": 7 })));
    os.call(Call::new("mines.reveal", json!({ "cell": 40 })));
    os.call(Call::new("mines.flag", json!({ "cell": 0 })));
    assert_eq!(os.envelope(None).to_json(), fixture("mines"));
}

#[test]
fn twenty48_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "twenty48" })));
    os.call(Call::new("twenty48.reset", json!({ "seed": 7 })));
    os.call(Call::new("twenty48.slide", json!({ "dir": "left" })));
    os.call(Call::new("twenty48.slide", json!({ "dir": "up" })));
    assert_eq!(os.envelope(None).to_json(), fixture("twenty48"));
}

#[test]
fn crush_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "crush" })));
    os.call(Call::new("crush.reset", json!({ "seed": 7 })));
    os.call(Call::new("crush.move", json!({ "dir": "left" })));
    os.call(Call::new("crush.step", json!({ "n": 2 })));
    assert_eq!(os.envelope(None).to_json(), fixture("crush"));
}

#[test]
fn tennis_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "tennis" })));
    os.call(Call::new("tennis.reset", json!({ "seed": 7 })));
    os.call(Call::new("tennis.move", json!({ "dir": "up" })));
    os.call(Call::new("tennis.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("tennis"));
}

#[test]
fn escape_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "escape" })));
    os.call(Call::new("escape.reset", json!({ "seed": 7 })));
    os.call(Call::new("escape.turn", json!({ "dir": "right" })));
    os.call(Call::new("escape.step", json!({ "n": 2 })));
    os.call(Call::new("escape.turn", json!({ "dir": "up" })));
    os.call(Call::new("escape.step", json!({})));
    assert_eq!(os.envelope(None).to_json(), fixture("escape"));
}

#[test]
fn quiz_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "quiz" })));
    os.call(Call::new("quiz.reset", json!({ "seed": 7 })));
    os.call(Call::new("quiz.answer", json!({ "text": "grid" })));
    assert_eq!(os.envelope(None).to_json(), fixture("quiz"));
}

#[test]
fn captcha_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "captcha" })));
    os.call(Call::new("captcha.reset", json!({ "seed": 7 })));
    os.call(Call::new("captcha.answer", json!({ "text": "node" })));
    assert_eq!(os.envelope(None).to_json(), fixture("captcha"));
}

#[test]
fn pixel_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "pixel" })));
    os.call(Call::new("pixel.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "pixel.stroke",
        json!({ "points": [[1, 1], [2, 2]] }),
    ));
    os.call(Call::new("pixel.clear", json!({})));
    assert_eq!(os.envelope(None).to_json(), fixture("pixel"));
}

#[test]
fn solids_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "solids" })));
    os.call(Call::new("solids.reset", json!({ "seed": 7 })));
    os.call(Call::new("solids.pick", json!({ "solid": "octa" })));
    os.call(Call::new("solids.orbit", json!({ "dir": "left", "n": 2 })));
    os.call(Call::new("solids.step", json!({ "n": 4 })));
    assert_eq!(os.envelope(None).to_json(), fixture("solids"));
}

#[test]
fn two_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "two" })));
    os.call(Call::new(
        "two.set",
        json!({ "key": "design", "value": "net" }),
    ));
    os.call(Call::new("two.set", json!({ "key": "number", "value": 7 })));
    os.call(Call::new("two.set", json!({ "key": "level", "value": 2 })));
    os.call(Call::new(
        "two.set",
        json!({ "key": "fill", "value": "cyan" }),
    ));
    os.call(Call::new("two.page", json!({ "dir": "next" })));
    assert_eq!(os.envelope(None).to_json(), fixture("two"));
}

#[test]
fn three_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "three" })));
    os.call(Call::new(
        "three.set",
        json!({ "key": "design", "value": "xtree" }),
    ));
    os.call(Call::new(
        "three.set",
        json!({ "key": "view", "value": "top" }),
    ));
    os.call(Call::new(
        "three.set",
        json!({ "key": "fill", "value": "orange" }),
    ));
    os.call(Call::new("three.page", json!({ "dir": "next" })));
    assert_eq!(os.envelope(None).to_json(), fixture("three"));
}

#[test]
fn bang_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "bang" })));
    os.call(Call::new("bang.page", json!({ "dir": "next" })));
    os.call(Call::new(
        "bang.set",
        json!({ "key": "dimension", "value": 3 }),
    ));
    os.call(Call::new("bang.page", json!({ "dir": "next" })));
    assert_eq!(os.envelope(None).to_json(), fixture("bang"));
}

#[test]
fn tile_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "tile" })));
    os.call(Call::new(
        "tile.set",
        json!({ "key": "group", "value": "Special" }),
    ));
    os.call(Call::new(
        "tile.set",
        json!({ "key": "catalog", "value": "Universe" }),
    ));
    os.call(Call::new("tile.paint", json!({ "seed": 7 })));
    os.call(Call::new(
        "tile.set",
        json!({ "key": "edition", "value": "Layers" }),
    ));
    assert_eq!(os.envelope(None).to_json(), fixture("tile"));
}

#[test]
fn six_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "six" })));
    os.call(Call::new(
        "six.set",
        json!({ "key": "design", "value": "ztree" }),
    ));
    os.call(Call::new(
        "six.set",
        json!({ "key": "view", "value": "pro" }),
    ));
    os.call(Call::new(
        "six.set",
        json!({ "key": "fill", "value": "pink" }),
    ));
    os.call(Call::new("six.page", json!({ "dir": "prev" })));
    assert_eq!(os.envelope(None).to_json(), fixture("six"));
}

#[test]
fn waves_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "waves" })));
    os.call(Call::new("waves.reset", json!({ "seed": 7 })));
    os.call(Call::new("waves.set", json!({ "key": "gain", "value": 6 })));
    os.call(Call::new("waves.set", json!({ "key": "damp", "value": 5 })));
    os.call(Call::new("waves.drop", json!({ "x": 2, "y": 2 })));
    os.call(Call::new("waves.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("waves"));
}

#[test]
fn billiards_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "billiards" })));
    os.call(Call::new("billiards.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "billiards.set",
        json!({ "key": "count", "value": 8 }),
    ));
    os.call(Call::new(
        "billiards.set",
        json!({ "key": "speed", "value": 200 }),
    ));
    os.call(Call::new("billiards.break", json!({ "x": 2, "y": 2 })));
    os.call(Call::new("billiards.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("billiards"));
}

#[test]
fn lasers_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "lasers" })));
    os.call(Call::new("lasers.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "lasers.set",
        json!({ "key": "rays", "value": 8 }),
    ));
    os.call(Call::new(
        "lasers.set",
        json!({ "key": "spread", "value": "narrow" }),
    ));
    os.call(Call::new("lasers.place", json!({ "x": 2, "y": 2 })));
    os.call(Call::new("lasers.step", json!({ "n": 3 })));
    assert_eq!(os.envelope(None).to_json(), fixture("lasers"));
}

#[test]
fn chess_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "chess" })));
    os.call(Call::new("chess.reset", json!({ "seed": 7 })));
    os.call(Call::new("chess.move", json!({ "from": "e2", "to": "e4" })));
    os.call(Call::new("chess.move", json!({ "from": "e7", "to": "e5" })));
    os.call(Call::new("chess.select", json!({ "square": "g1" })));
    assert_eq!(os.envelope(None).to_json(), fixture("chess"));
}

#[test]
fn font_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "font" })));
    os.call(Call::new("font.pick", json!({ "char": "a" })));
    os.call(Call::new("font.scramble", json!({})).at(7));
    os.call(Call::new("font.tick", json!({})));
    os.call(Call::new("font.tick", json!({})));
    assert_eq!(os.envelope(None).to_json(), fixture("font"));
}

#[test]
fn moire_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "moire" })));
    os.call(Call::new(
        "moire.set",
        json!({ "key": "angle", "value": 180 }),
    ));
    os.call(Call::new(
        "moire.set",
        json!({ "key": "lattice", "value": "hex" }),
    ));
    assert_eq!(os.envelope(None).to_json(), fixture("moire"));
}

#[test]
fn hash_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "hash" })));
    os.call(Call::new(
        "hash.digest",
        json!({ "text": "counting universe" }),
    ));
    os.call(Call::new(
        "hash.set",
        json!({ "key": "rule", "value": "maze" }),
    ));
    assert_eq!(os.envelope(None).to_json(), fixture("hash"));
}

#[test]
fn colors_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "colors" })));
    os.call(Call::new("colors.page", json!({ "dir": "next" })));
    os.call(Call::new(
        "colors.set",
        json!({ "key": "name", "value": "teal" }),
    ));
    assert_eq!(os.envelope(None).to_json(), fixture("colors"));
}

#[test]
fn emoji_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "emoji" })));
    os.call(Call::new(
        "emoji.set",
        json!({ "key": "category", "value": "food" }),
    ));
    assert_eq!(os.envelope(None).to_json(), fixture("emoji"));
}

#[test]
fn piano_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "piano" })));
    os.call(Call::new("piano.press", json!({ "midi": 43 })));
    os.call(Call::new("piano.press", json!({ "midi": 55 })));
    os.call(Call::new("piano.lift", json!({ "midi": 43 })));
    assert_eq!(os.envelope(None).to_json(), fixture("piano"));
}

#[test]
fn log_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "notes" })));
    os.call(Call::new("notes.add", json!({ "text": "buy oat milk" })).at(1783600496000));
    os.call(Call::new("notes.add", json!({ "text": "book the ferry" })));
    os.call(Call::new("nav.open", json!({ "app": "log" })));
    assert_eq!(os.envelope(None).to_json(), fixture("log"));
}

#[test]
fn files_frame_is_golden() {
    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "colors" })));
    os.call(Call::new("colors.export", json!({})).at(1783600496000));
    os.call(Call::new("nav.open", json!({ "app": "files" })));
    assert_eq!(os.envelope(None).to_json(), fixture("files"));
}
