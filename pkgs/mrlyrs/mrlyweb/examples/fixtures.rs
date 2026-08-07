use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use std::fs;

fn boot() -> Os {
    mrlyweb::registry::boot()
}

fn shoot(os: &Os, app: &str) -> Json {
    let cells = os.read(&format!("{app}/cells"), None).unwrap();
    mrlyui::skin::raster(app, &cells, 8, false)
        .unwrap()
        .image()
        .to_json()
}

fn write(name: &str, os: &Os) {
    let path = format!(
        "{}/../../../apps/web/fixtures/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = os.envelope(None).to_json().pretty();
    fs::write(&path, text + "\n").unwrap();
    println!("wrote {path}");
}

fn main() {
    let os = boot();
    write("menu", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "calculator" })));
    os.call(Call::new("calculator.digit", json!({ "d": 4 })));
    os.call(Call::new("calculator.digit", json!({ "d": 2 })));
    write("calculator", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "notes" })));
    for text in ["buy oat milk", "book the ferry", "read the franel paper"] {
        os.call(Call::new("notes.add", json!({ "text": text })));
    }
    write("notes", &os);

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
    write("settings", &os);

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
    write("ui", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "life" })));
    os.call(Call::new("life.step", json!({})));
    os.call(Call::new("life.step", json!({})));
    write("life", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "clock" })));
    os.call(Call::new("clock.tick", json!({})).at(1783600496000));
    write("clock", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "timer" })));
    os.call(Call::new("timer.start", json!({ "secs": 60 })).at(1783600496000));
    os.call(Call::new("timer.check", json!({})).at(1783600556000));
    write("timer", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "calendar" })));
    os.call(Call::new(
        "calendar.goto",
        json!({ "year": 2026, "month": 6 }),
    ));
    os.call(Call::new("calendar.flip", json!({ "n": -1 })));
    os.call(Call::new("calendar.today", json!({})).at(1783600496000));
    write("calendar", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "dice" })));
    os.call(Call::new("dice.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "dice.set",
        json!({ "key": "sides", "value": 20 }),
    ));
    os.call(Call::new("dice.roll", json!({})));
    os.call(Call::new("dice.roll", json!({})));
    write("dice", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "life" })));
    os.call(Call::new("sys.shot", json!({})));
    os.call(Call::new("nav.open", json!({ "app": "ttt" })));
    os.call(Call::new("ttt.reset", json!({ "seed": 7 })));
    os.call(Call::new("ttt.place", json!({ "cell": 4 })));
    let image = shoot(&os, "ttt");
    os.call(Call::new("sys.shot", json!({ "image": image })));
    os.call(Call::new("nav.open", json!({ "app": "photos" })));
    write("photos", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "snake" })));
    os.call(Call::new("snake.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "snake.set",
        json!({ "key": "design", "value": "net" }),
    ));
    os.call(Call::new("snake.turn", json!({ "dir": "left" })));
    os.call(Call::new("snake.step", json!({})));
    os.call(Call::new("snake.turn", json!({ "dir": "up" })));
    os.call(Call::new("snake.step", json!({ "n": 2 })));
    write("snake", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "julia" })));
    os.call(Call::new("julia.reset", json!({ "seed": 7 })));
    os.call(Call::new("julia.step", json!({ "n": 3 })));
    write("julia", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "mandelbrot" })));
    os.call(Call::new("mandelbrot.reset", json!({ "seed": 7 })));
    os.call(Call::new("mandelbrot.step", json!({ "n": 3 })));
    write("mandelbrot", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "matrix" })));
    os.call(Call::new("matrix.reset", json!({ "seed": 7 })));
    os.call(Call::new("matrix.step", json!({ "n": 3 })));
    write("matrix", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "sleep" })));
    os.call(Call::new("sleep.reset", json!({ "seed": 7 })));
    os.call(Call::new("sleep.step", json!({ "n": 3 })));
    write("sleep", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "ttt" })));
    os.call(Call::new("ttt.reset", json!({ "seed": 7 })));
    os.call(Call::new("ttt.place", json!({ "cell": 0 })));
    os.call(Call::new("ttt.place", json!({ "cell": 4 })));
    write("ttt", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "memory" })));
    os.call(Call::new("memory.reset", json!({ "seed": 7 })));
    for _ in 0..16 {
        os.call(Call::new("memory.tick", json!({})));
    }
    os.call(Call::new("memory.flip", json!({ "card": 0 })));
    os.call(Call::new("memory.flip", json!({ "card": 1 })));
    write("memory", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "mines" })));
    os.call(Call::new("mines.reset", json!({ "seed": 7 })));
    os.call(Call::new("mines.reveal", json!({ "cell": 40 })));
    os.call(Call::new("mines.flag", json!({ "cell": 0 })));
    write("mines", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "twenty48" })));
    os.call(Call::new("twenty48.reset", json!({ "seed": 7 })));
    os.call(Call::new("twenty48.slide", json!({ "dir": "left" })));
    os.call(Call::new("twenty48.slide", json!({ "dir": "up" })));
    write("twenty48", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "crush" })));
    os.call(Call::new("crush.reset", json!({ "seed": 7 })));
    os.call(Call::new("crush.move", json!({ "dir": "left" })));
    os.call(Call::new("crush.step", json!({ "n": 2 })));
    write("crush", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "tennis" })));
    os.call(Call::new("tennis.reset", json!({ "seed": 7 })));
    os.call(Call::new("tennis.move", json!({ "dir": "up" })));
    os.call(Call::new("tennis.step", json!({ "n": 3 })));
    write("tennis", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "escape" })));
    os.call(Call::new("escape.reset", json!({ "seed": 7 })));
    os.call(Call::new("escape.turn", json!({ "dir": "right" })));
    os.call(Call::new("escape.step", json!({ "n": 2 })));
    os.call(Call::new("escape.turn", json!({ "dir": "up" })));
    os.call(Call::new("escape.step", json!({})));
    write("escape", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "quiz" })));
    os.call(Call::new("quiz.reset", json!({ "seed": 7 })));
    os.call(Call::new("quiz.answer", json!({ "text": "grid" })));
    write("quiz", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "captcha" })));
    os.call(Call::new("captcha.reset", json!({ "seed": 7 })));
    os.call(Call::new("captcha.answer", json!({ "text": "node" })));
    write("captcha", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "pixel" })));
    os.call(Call::new("pixel.reset", json!({ "seed": 7 })));
    os.call(Call::new(
        "pixel.stroke",
        json!({ "points": [[1, 1], [2, 2]] }),
    ));
    os.call(Call::new("pixel.clear", json!({})));
    write("pixel", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "solids" })));
    os.call(Call::new("solids.reset", json!({ "seed": 7 })));
    os.call(Call::new("solids.pick", json!({ "solid": "octa" })));
    os.call(Call::new("solids.orbit", json!({ "dir": "left", "n": 2 })));
    os.call(Call::new("solids.step", json!({ "n": 4 })));
    write("solids", &os);

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
    write("two", &os);

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
    write("three", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "bang" })));
    os.call(Call::new("bang.page", json!({ "dir": "next" })));
    os.call(Call::new(
        "bang.set",
        json!({ "key": "dimension", "value": 3 }),
    ));
    os.call(Call::new("bang.page", json!({ "dir": "next" })));
    write("bang", &os);

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
    write("tile", &os);

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
    write("six", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "chess" })));
    os.call(Call::new("chess.reset", json!({ "seed": 7 })));
    os.call(Call::new("chess.move", json!({ "from": "e2", "to": "e4" })));
    os.call(Call::new("chess.move", json!({ "from": "e7", "to": "e5" })));
    os.call(Call::new("chess.select", json!({ "square": "g1" })));
    write("chess", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "font" })));
    os.call(Call::new("font.pick", json!({ "char": "a" })));
    os.call(Call::new("font.scramble", json!({})).at(7));
    os.call(Call::new("font.tick", json!({})));
    os.call(Call::new("font.tick", json!({})));
    write("font", &os);

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
    write("moire", &os);

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
    write("hash", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "colors" })));
    os.call(Call::new("colors.page", json!({ "dir": "next" })));
    os.call(Call::new(
        "colors.set",
        json!({ "key": "name", "value": "teal" }),
    ));
    write("colors", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "emoji" })));
    os.call(Call::new(
        "emoji.set",
        json!({ "key": "category", "value": "food" }),
    ));
    write("emoji", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "piano" })));
    os.call(Call::new("piano.press", json!({ "midi": 43 })));
    os.call(Call::new("piano.press", json!({ "midi": 55 })));
    os.call(Call::new("piano.lift", json!({ "midi": 43 })));
    write("piano", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "twenty48" })));
    os.call(Call::new("twenty48.reset", json!({ "seed": 7 })));
    os.call(Call::new("twenty48.slide", json!({ "dir": "left" })));
    let image = shoot(&os, "twenty48");
    os.call(Call::new("sys.shot", json!({ "image": image })));
    os.call(Call::new("nav.open", json!({ "app": "photos" })));
    write("shot", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "notes" })));
    os.call(Call::new("notes.add", json!({ "text": "buy oat milk" })).at(1783600496000));
    os.call(Call::new("notes.add", json!({ "text": "book the ferry" })));
    os.call(Call::new("nav.open", json!({ "app": "log" })));
    write("log", &os);

    let mut os = boot();
    os.call(Call::new("nav.open", json!({ "app": "colors" })));
    os.call(Call::new("colors.export", json!({})).at(1783600496000));
    os.call(Call::new("nav.open", json!({ "app": "files" })));
    write("files", &os);
}
