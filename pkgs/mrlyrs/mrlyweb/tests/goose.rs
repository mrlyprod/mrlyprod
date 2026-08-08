use mrlyos::kernel::Os;
use mrlyweb::Goose;

fn boot() -> Os {
    mrlyweb::registry::boot("full")
}

fn playable() -> Vec<String> {
    mrlyweb::registry::catalogue()
        .iter()
        .filter(|app| ["games", "puzzles"].contains(&app.manifest().category.as_str()))
        .map(|app| app.route().to_string())
        .collect()
}

#[test]
fn goose_survives_every_game() {
    let apps = playable();
    assert!(apps.len() >= 10, "expected the games and puzzles shelf");
    for route in apps {
        let mut os = boot();
        os.open(&route).unwrap();
        let mut goose = Goose::new(7);
        let acted = (0..100).filter_map(|_| goose.step(&mut os)).count();
        assert!(acted > 0, "{route} never accepted a goose call");
    }
}

#[test]
fn goose_replays_exactly() {
    for route in ["snake", "twenty48", "mines"] {
        let run = |seed: u64| {
            let mut os = boot();
            os.open(route).unwrap();
            let mut goose = Goose::new(seed);
            (0..60)
                .filter_map(|_| goose.step(&mut os))
                .map(|call| call.to_json().to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(run(11), run(11), "{route} transcript drifted");
        assert_ne!(run(11), run(12), "{route} ignores its seed");
    }
}
