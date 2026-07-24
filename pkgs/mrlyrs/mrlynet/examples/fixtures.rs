use mrlyos::kernel::{Call, Iden, Os};
use serde_json::json;
use std::fs;

const PNG: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

fn boot() -> Os {
    let mut os = Os::new(Iden::new("guest"));
    for app in mrlynet::registry::catalogue() {
        os = os.install(app);
    }
    os
}

fn write(name: &str, os: &Os) {
    let path = format!(
        "{}/../../../apps/web/fixtures/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = serde_json::to_string_pretty(&os.frame().to_json()).unwrap();
    fs::write(&path, text + "\n").unwrap();
    println!("wrote {path}");
}

fn main() {
    let os = boot();
    write("menu", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "calculator" })));
    os.act(Call::new("calculator.digit", json!({ "d": 4 })));
    os.act(Call::new("calculator.digit", json!({ "d": 2 })));
    write("calculator", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "notes" })));
    for text in ["buy oat milk", "book the ferry", "read the franel paper"] {
        os.act(Call::new("notes.add", json!({ "text": text })));
    }
    write("notes", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "settings" })));
    os.act(Call::new(
        "settings.set",
        json!({ "key": "launchpad", "value": "list" }),
    ));
    os.act(Call::new(
        "settings.set",
        json!({ "key": "radius", "value": 3 }),
    ));
    os.act(Call::new(
        "settings.set",
        json!({ "key": "scale", "value": 4 }),
    ));
    write("settings", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "ui" })));
    os.act(Call::new(
        "ui.set",
        json!({ "key": "pick", "value": "beta" }),
    ));
    os.act(Call::new(
        "ui.set",
        json!({ "key": "overlay", "value": true }),
    ));
    write("ui", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "life" })));
    os.act(Call::new("life.step", json!({})));
    os.act(Call::new("life.step", json!({})));
    write("life", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "clock" })));
    os.act(Call::new("clock.tick", json!({})).at(1783600496000));
    write("clock", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "timer" })));
    os.act(Call::new("timer.start", json!({ "minutes": 1 })).at(1783600496000));
    os.act(Call::new("timer.check", json!({})).at(1783600556000));
    write("timer", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "calendar" })));
    os.act(Call::new(
        "calendar.goto",
        json!({ "year": 2026, "month": 6 }),
    ));
    os.act(Call::new("calendar.flip", json!({ "n": -1 })));
    os.act(Call::new("calendar.today", json!({})).at(1783600496000));
    write("calendar", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "dice" })));
    os.act(Call::new("dice.reset", json!({ "seed": 7 })));
    os.act(Call::new(
        "dice.set",
        json!({ "key": "sides", "value": 20 }),
    ));
    os.act(Call::new("dice.roll", json!({})));
    os.act(Call::new("dice.roll", json!({})));
    write("dice", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "photos" })));
    os.act(Call::new("photos.load", json!({})));
    os.act(Call::new(
        "photos.land",
        json!({ "data": PNG, "mime": "image/png" }),
    ));
    os.act(Call::new("photos.load", json!({})));
    write("photos", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "snake" })));
    os.act(Call::new("snake.reset", json!({ "seed": 7 })));
    os.act(Call::new(
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
    os.act(Call::new("snake.turn", json!({ "dir": "left" })));
    os.act(Call::new("snake.step", json!({})));
    os.act(Call::new("snake.turn", json!({ "dir": "up" })));
    os.act(Call::new("snake.step", json!({ "n": 2 })));
    write("snake", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "julia" })));
    os.act(Call::new("julia.reset", json!({ "seed": 7 })));
    os.act(Call::new("julia.step", json!({ "n": 3 })));
    write("julia", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "mandelbrot" })));
    os.act(Call::new("mandelbrot.reset", json!({ "seed": 7 })));
    os.act(Call::new("mandelbrot.step", json!({ "n": 3 })));
    write("mandelbrot", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "matrix" })));
    os.act(Call::new("matrix.reset", json!({ "seed": 7 })));
    os.act(Call::new("matrix.step", json!({ "n": 3 })));
    write("matrix", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "sleep" })));
    os.act(Call::new("sleep.reset", json!({ "seed": 7 })));
    os.act(Call::new("sleep.step", json!({ "n": 3 })));
    write("sleep", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "ttt" })));
    os.act(Call::new("ttt.reset", json!({ "seed": 7 })));
    os.act(Call::new("ttt.place", json!({ "cell": 0 })));
    os.act(Call::new("ttt.place", json!({ "cell": 4 })));
    write("ttt", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "memory" })));
    os.act(Call::new("memory.reset", json!({ "seed": 7 })));
    for _ in 0..16 {
        os.act(Call::new("memory.tick", json!({})));
    }
    os.act(Call::new("memory.flip", json!({ "card": 0 })));
    os.act(Call::new("memory.flip", json!({ "card": 1 })));
    write("memory", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "mines" })));
    os.act(Call::new("mines.reset", json!({ "seed": 7 })));
    os.act(Call::new("mines.reveal", json!({ "cell": 40 })));
    os.act(Call::new("mines.flag", json!({ "cell": 0 })));
    write("mines", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "twenty48" })));
    os.act(Call::new("twenty48.reset", json!({ "seed": 7 })));
    os.act(Call::new("twenty48.slide", json!({ "dir": "left" })));
    os.act(Call::new("twenty48.slide", json!({ "dir": "up" })));
    write("twenty48", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "crush" })));
    os.act(Call::new("crush.reset", json!({ "seed": 7 })));
    os.act(Call::new("crush.move", json!({ "dir": "left" })));
    os.act(Call::new("crush.step", json!({ "n": 2 })));
    write("crush", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "tennis" })));
    os.act(Call::new("tennis.reset", json!({ "seed": 7 })));
    os.act(Call::new("tennis.move", json!({ "dir": "up" })));
    os.act(Call::new("tennis.step", json!({ "n": 3 })));
    write("tennis", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "escape" })));
    os.act(Call::new("escape.reset", json!({ "seed": 7 })));
    os.act(Call::new("escape.turn", json!({ "dir": "right" })));
    os.act(Call::new("escape.step", json!({ "n": 2 })));
    os.act(Call::new("escape.turn", json!({ "dir": "up" })));
    os.act(Call::new("escape.step", json!({})));
    write("escape", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "quiz" })));
    os.act(Call::new("quiz.reset", json!({ "seed": 7 })));
    os.act(Call::new("quiz.answer", json!({ "text": "grid" })));
    write("quiz", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "captcha" })));
    os.act(Call::new("captcha.reset", json!({ "seed": 7 })));
    os.act(Call::new("captcha.answer", json!({ "text": "node" })));
    write("captcha", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "pixel" })));
    os.act(Call::new("pixel.reset", json!({ "seed": 7 })));
    os.act(Call::new(
        "pixel.stroke",
        json!({ "points": [[1, 1], [2, 2]] }),
    ));
    os.act(Call::new("pixel.clear", json!({})));
    write("pixel", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "solids" })));
    os.act(Call::new("solids.reset", json!({ "seed": 7 })));
    os.act(Call::new("solids.pick", json!({ "solid": "octa" })));
    os.act(Call::new("solids.orbit", json!({ "dir": "left", "n": 2 })));
    os.act(Call::new("solids.step", json!({ "n": 4 })));
    write("solids", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "text" })));
    os.act(Call::new("text.page", json!({ "dir": "next" })));
    os.act(Call::new("text.set", json!({ "key": "level", "value": 3 })));
    write("text", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "two" })));
    os.act(Call::new(
        "two.set",
        json!({ "key": "design", "value": "net" }),
    ));
    os.act(Call::new("two.set", json!({ "key": "number", "value": 7 })));
    os.act(Call::new("two.set", json!({ "key": "level", "value": 2 })));
    os.act(Call::new(
        "two.set",
        json!({ "key": "fill", "value": "cyan" }),
    ));
    os.act(Call::new("two.page", json!({ "dir": "next" })));
    write("two", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "three" })));
    os.act(Call::new(
        "three.set",
        json!({ "key": "design", "value": "xtree" }),
    ));
    os.act(Call::new(
        "three.set",
        json!({ "key": "view", "value": "top" }),
    ));
    os.act(Call::new(
        "three.set",
        json!({ "key": "fill", "value": "orange" }),
    ));
    os.act(Call::new("three.page", json!({ "dir": "next" })));
    write("three", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "bang" })));
    os.act(Call::new("bang.page", json!({ "dir": "next" })));
    os.act(Call::new(
        "bang.set",
        json!({ "key": "dimension", "value": 3 }),
    ));
    os.act(Call::new("bang.page", json!({ "dir": "next" })));
    write("bang", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "tile" })));
    os.act(Call::new(
        "tile.set",
        json!({ "key": "group", "value": "Special" }),
    ));
    os.act(Call::new(
        "tile.set",
        json!({ "key": "catalog", "value": "Universe" }),
    ));
    os.act(Call::new("tile.paint", json!({ "seed": 7 })));
    os.act(Call::new(
        "tile.set",
        json!({ "key": "edition", "value": "Layers" }),
    ));
    write("tile", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "six" })));
    os.act(Call::new(
        "six.set",
        json!({ "key": "design", "value": "ztree" }),
    ));
    os.act(Call::new(
        "six.set",
        json!({ "key": "view", "value": "pro" }),
    ));
    os.act(Call::new(
        "six.set",
        json!({ "key": "fill", "value": "pink" }),
    ));
    os.act(Call::new("six.page", json!({ "dir": "prev" })));
    write("six", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "waves" })));
    os.act(Call::new("waves.reset", json!({ "seed": 7 })));
    os.act(Call::new("waves.set", json!({ "key": "gain", "value": 6 })));
    os.act(Call::new(
        "waves.set",
        json!({ "key": "damp", "value": 0.005 }),
    ));
    os.act(Call::new("waves.drop", json!({ "x": 2, "y": 2 })));
    os.act(Call::new("waves.step", json!({ "n": 3 })));
    write("waves", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "billiards" })));
    os.act(Call::new("billiards.reset", json!({ "seed": 7 })));
    os.act(Call::new(
        "billiards.set",
        json!({ "key": "count", "value": 8 }),
    ));
    os.act(Call::new(
        "billiards.set",
        json!({ "key": "speed", "value": 2.0 }),
    ));
    os.act(Call::new("billiards.break", json!({ "x": 2, "y": 2 })));
    os.act(Call::new("billiards.step", json!({ "n": 3 })));
    write("billiards", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "lasers" })));
    os.act(Call::new("lasers.reset", json!({ "seed": 7 })));
    os.act(Call::new(
        "lasers.set",
        json!({ "key": "rays", "value": 8 }),
    ));
    os.act(Call::new(
        "lasers.set",
        json!({ "key": "spread", "value": "narrow" }),
    ));
    os.act(Call::new("lasers.place", json!({ "x": 2, "y": 2 })));
    os.act(Call::new("lasers.step", json!({ "n": 3 })));
    write("lasers", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "chess" })));
    os.act(Call::new("chess.reset", json!({ "seed": 7 })));
    os.act(Call::new("chess.move", json!({ "from": "e2", "to": "e4" })));
    os.act(Call::new("chess.move", json!({ "from": "e7", "to": "e5" })));
    os.act(Call::new("chess.select", json!({ "square": "g1" })));
    write("chess", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "font" })));
    os.act(Call::new("font.pick", json!({ "char": "a" })));
    os.act(Call::new("font.scramble", json!({})).at(7));
    os.act(Call::new("font.tick", json!({})));
    os.act(Call::new("font.tick", json!({})));
    write("font", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "moire" })));
    os.act(Call::new(
        "moire.set",
        json!({ "key": "angle", "value": 180 }),
    ));
    os.act(Call::new(
        "moire.set",
        json!({ "key": "lattice", "value": "hex" }),
    ));
    write("moire", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "hash" })));
    os.act(Call::new(
        "hash.digest",
        json!({ "text": "counting universe" }),
    ));
    os.act(Call::new(
        "hash.set",
        json!({ "key": "rule", "value": "maze" }),
    ));
    write("hash", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "colors" })));
    os.act(Call::new("colors.page", json!({ "dir": "next" })));
    os.act(Call::new(
        "colors.set",
        json!({ "key": "name", "value": "teal" }),
    ));
    write("colors", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "emoji" })));
    os.act(Call::new(
        "emoji.set",
        json!({ "key": "category", "value": "food" }),
    ));
    write("emoji", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "piano" })));
    os.act(Call::new("piano.press", json!({ "midi": 43 })));
    os.act(Call::new("piano.press", json!({ "midi": 55 })));
    os.act(Call::new("piano.lift", json!({ "midi": 43 })));
    write("piano", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "two" })));
    os.act(Call::new("sys.shot", json!({})));
    os.act(Call::new("nav.open", json!({ "app": "photos" })));
    write("shot", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "extras" })));
    os.act(Call::new("extras.cycle", json!({})));
    write("extras", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "notes" })));
    os.act(Call::new("notes.add", json!({ "text": "buy oat milk" })).at(1783600496000));
    os.act(Call::new("notes.add", json!({ "text": "book the ferry" })));
    os.act(Call::new("nav.open", json!({ "app": "log" })));
    write("log", &os);

    let mut os = boot();
    os.act(Call::new("nav.open", json!({ "app": "colors" })));
    os.act(Call::new("colors.export", json!({})).at(1783600496000));
    os.act(Call::new("nav.open", json!({ "app": "files" })));
    write("files", &os);
}
