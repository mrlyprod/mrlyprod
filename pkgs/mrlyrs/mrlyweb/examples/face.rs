use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use std::fs;

fn boot() -> Os {
    mrlyweb::registry::boot()
}

fn main() {
    let outdir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/face".to_string());
    fs::create_dir_all(&outdir).unwrap();
    let mut count = 0;
    for route in boot().catalogue() {
        let mut os = boot();
        os.open(&route).unwrap();
        match mrlyweb::face::face_png(&os, &route) {
            Ok(png) => {
                fs::write(format!("{outdir}/{route}.png"), png).unwrap();
                count += 1;
            }
            Err(e) => eprintln!("! {route}: {e}"),
        }
    }
    let live: Vec<(&str, Vec<(&str, Json)>)> = vec![
        (
            "snake",
            vec![
                ("snake.reset", json!({ "seed": 7 })),
                ("snake.turn", json!({ "dir": "left" })),
                ("snake.step", json!({})),
                ("snake.turn", json!({ "dir": "up" })),
                ("snake.step", json!({ "n": 2 })),
            ],
        ),
        (
            "solids",
            vec![
                ("solids.reset", json!({ "seed": 7 })),
                ("solids.pick", json!({ "solid": "octa" })),
                ("solids.orbit", json!({ "dir": "left", "n": 2 })),
                ("solids.step", json!({ "n": 4 })),
            ],
        ),
        (
            "mandelbrot",
            vec![
                ("mandelbrot.reset", json!({ "seed": 7 })),
                ("mandelbrot.step", json!({ "n": 3 })),
            ],
        ),
        (
            "piano",
            vec![
                ("piano.press", json!({ "midi": 43 })),
                ("piano.press", json!({ "midi": 55 })),
                ("piano.lift", json!({ "midi": 43 })),
            ],
        ),
        (
            "settings",
            vec![
                (
                    "settings.set",
                    json!({ "key": "launchpad", "value": "list" }),
                ),
                ("settings.set", json!({ "key": "radius", "value": 3 })),
                ("settings.set", json!({ "key": "scale", "value": 4 })),
            ],
        ),
        ("pages", vec![("pages.open", json!({ "slug": "dummy" }))]),
    ];
    for (route, script) in live {
        let mut os = boot();
        os.act(Call::new("nav.open", json!({ "app": route })));
        for (verb, args) in script {
            os.act(Call::new(verb, args));
        }
        match mrlyweb::face::face_png(&os, route) {
            Ok(png) => {
                fs::write(format!("{outdir}/{route}-live.png"), png).unwrap();
                count += 1;
            }
            Err(e) => eprintln!("! {route}: {e}"),
        }
    }
    println!("wrote {count} faces to {outdir}");
}
