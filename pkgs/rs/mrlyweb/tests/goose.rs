use mrlycore::json;
use mrlyos::kernel::{Os, Verb};
use mrlyweb::Goose;

fn boot() -> Os {
    mrlyweb::registry::boot("full")
}

fn routes() -> Vec<String> {
    mrlyweb::registry::catalogue()
        .iter()
        .map(|app| app.route().to_string())
        .collect()
}

fn actions(route: &str) -> Vec<Verb> {
    let mut os = boot();
    if os.open(route).is_err() {
        return Vec::new();
    }
    os.envelope(Some(&json!({})))
        .view
        .map(|view| view.actions)
        .unwrap_or_default()
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

#[test]
fn no_app_advertises_a_verb_the_goose_cannot_play() {
    let mut dark = Vec::new();
    for route in routes() {
        for verb in actions(&route) {
            if !Goose::plays(&verb) {
                dark.push(verb.name.clone());
            }
        }
    }
    assert!(dark.is_empty(), "silent verbs: {}", dark.join(", "));
}

#[test]
fn every_settings_surface_takes_a_goose_call() {
    let mut missed = Vec::new();
    for route in routes() {
        let set = format!("{route}.set");
        if !actions(&route).iter().any(|verb| verb.name == set) {
            continue;
        }
        let mut os = boot();
        os.open(&route).unwrap();
        let mut goose = Goose::new(7);
        let landed = (0..200)
            .filter_map(|_| goose.step(&mut os))
            .any(|call| call.verb == set);
        if !landed {
            missed.push(set);
        }
    }
    assert!(missed.is_empty(), "never landed: {}", missed.join(", "));
}
