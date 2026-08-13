use mrlycore::data::Well;
use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use mrlyweb::registry;
use mrlyweb::Goose;

const BUDGET: usize = 8;

/// Returns one goose-trail well for every registry app with a playable verb.
pub fn wells() -> Vec<Box<dyn Well>> {
    let mut os = registry::boot("full");
    let mut out: Vec<Box<dyn Well>> = Vec::new();
    for route in os.catalogue() {
        if os.open(&route).is_err() {
            continue;
        }
        let Some(view) = os.envelope(Some(&json!({}))).view else {
            continue;
        };
        let playable = view
            .actions
            .iter()
            .any(|verb| !verb.name.ends_with(".reset") && Goose::plays(verb));
        if playable {
            out.push(Box::new(Trail::new(route)));
        }
    }
    out
}

struct Trail {
    route: String,
    name: String,
    about: String,
}

impl Trail {
    fn new(route: String) -> Trail {
        Trail {
            name: format!("trail_{route}"),
            about: format!("Seeded goose episodes on the {route} app, one row per accepted call."),
            route,
        }
    }
}

impl Well for Trail {
    fn name(&self) -> &str {
        &self.name
    }
    fn about(&self) -> &str {
        &self.about
    }
    fn pour(&self, seed: u64, count: usize) -> Vec<Json> {
        let mut os = registry::boot("full");
        if os.open(&self.route).is_err() {
            return Vec::new();
        }
        let mut goose = Goose::new(seed);
        let mut rows = Vec::new();
        for step in 0..count.saturating_mul(BUDGET) {
            if rows.len() >= count {
                break;
            }
            let Some(call) = goose.step(&mut os) else {
                continue;
            };
            rows.push(row(&self.route, seed, step, &call, &os));
        }
        rows
    }
}

fn row(route: &str, seed: u64, step: usize, call: &Call, os: &Os) -> Json {
    let shape = json!({ "score": 1, "over": 1 });
    let envelope = os.envelope(Some(&shape));
    let mut out = json!({
        "app": route,
        "seed": crate::press::clip(seed),
        "step": step,
        "verb": &call.verb,
        "args": call.args.clone(),
        "ok": envelope.last.as_ref().map(|last| last.ok).unwrap_or(true),
    });
    if let Some(note) = envelope.last.and_then(|last| last.note) {
        out["note"] = json!(note);
    }
    if let Some(view) = envelope.view {
        if let Some(score) = view.state.get("score") {
            out["score"] = score.clone();
        }
        if let Some(over) = view.state.get("over") {
            out["over"] = over.clone();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_playable_app_earns_a_trail() {
        let names: Vec<String> = wells().iter().map(|w| w.name().to_string()).collect();
        assert!(names.len() > 10, "only {} trails surfaced", names.len());
        assert!(names.iter().all(|name| name.starts_with("trail_")));
    }

    #[test]
    fn a_trail_replays_byte_identical() {
        let well = &wells()[0];
        let a: Vec<String> = well.pour(7, 3).iter().map(Json::to_string).collect();
        let b: Vec<String> = well.pour(7, 3).iter().map(Json::to_string).collect();
        assert_eq!(a, b);
        assert!(!a.is_empty());
    }

    #[test]
    fn trail_rows_carry_the_episode_facts() {
        let well = &wells()[0];
        for (i, row) in well.pour(3, 3).iter().enumerate() {
            assert!(row["app"].as_str().is_some());
            assert_eq!(row["seed"], 3);
            assert!(row["step"].as_i64().unwrap() >= i as i64);
            assert!(row["verb"].as_str().unwrap().contains('.'));
            assert!(row["args"].is_object());
            assert!(row["ok"].is_boolean());
        }
    }
}
