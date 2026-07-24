use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use mrlymusic::render::VOLUME;
use mrlymusic::theory;
use serde_json::{json, Value as Json};

const GRID: [u8; 25] = [
    1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1,
];

fn keys() -> Vec<Option<i64>> {
    let mut k: i64 = 0;
    GRID.iter()
        .map(|&cell| {
            if cell == 0 {
                return None;
            }
            let midi = theory::ROOT + 12 * (k / 7) + theory::MAJOR[(k % 7) as usize];
            k += 1;
            Some(midi)
        })
        .collect()
}

pub struct Piano {
    held: Vec<i64>,
    wave: String,
}

impl Default for Piano {
    fn default() -> Piano {
        Piano::new()
    }
}

impl Piano {
    pub fn new() -> Piano {
        Piano {
            held: Vec::new(),
            wave: "sine".to_string(),
        }
    }
    fn start(&self, midi: i64) -> Effect {
        Effect::new(
            "sound",
            json!({
                "op": "start",
                "id": format!("piano:{midi}"),
                "midi": midi,
                "freq": theory::freq(midi),
                "wave": self.wave,
                "gain": VOLUME,
            }),
        )
    }
    fn stop(midi: i64) -> Effect {
        Effect::new(
            "sound",
            json!({ "op": "stop", "id": format!("piano:{midi}") }),
        )
    }
}

impl App for Piano {
    fn route(&self) -> &str {
        "piano"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("piano").emoji("🎹").category("creativity")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let cells: Vec<Json> = keys()
            .iter()
            .map(|slot| match slot {
                Some(midi) => json!({
                    "midi": midi,
                    "name": theory::name(*midi),
                    "held": self.held.contains(midi),
                }),
                None => Json::Null,
            })
            .collect();
        json!({ "cols": 5, "cells": cells, "held": self.held })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("piano.press", json!({ "midi": "number" })),
            Verb::new("piano.lift", json!({ "midi": "number" })),
            Verb::new("piano.silence", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "piano.press" => {
                let midi = call.arg("midi").as_i64().unwrap_or(-1);
                if !keys().contains(&Some(midi)) {
                    return Outcome::fail("no such key");
                }
                if self.held.contains(&midi) {
                    return Outcome::fail("already held");
                }
                self.held.push(midi);
                Outcome::ok(json!({ "midi": midi, "held": self.held })).emit(self.start(midi))
            }
            "piano.lift" => {
                let midi = call.arg("midi").as_i64().unwrap_or(-1);
                if !self.held.contains(&midi) {
                    return Outcome::fail("not held");
                }
                self.held.retain(|&m| m != midi);
                Outcome::ok(json!({ "midi": midi, "held": self.held })).emit(Piano::stop(midi))
            }
            "piano.silence" => {
                let mut out = Outcome::ok(json!({ "held": [] }));
                for &midi in &self.held {
                    out = out.emit(Piano::stop(midi));
                }
                self.held.clear();
                out
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn wear(&mut self, world: &Json) {
        self.wave = world["shared"]["settings"]["wave"]
            .as_str()
            .unwrap_or("sine")
            .to_string();
    }
    fn save(&self) -> Json {
        json!({ "held": self.held })
    }
    fn load(&mut self, state: &Json) {
        let valid = keys();
        self.held = state["held"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(Json::as_i64)
                    .filter(|m| valid.contains(&Some(*m)))
                    .collect()
            })
            .unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grid_hangs_three_octaves() {
        let all: Vec<i64> = keys().into_iter().flatten().collect();
        assert_eq!(all.len(), 21);
        assert_eq!(all[0], 43);
        assert_eq!(all[7], 55);
        assert_eq!(all[20], 78);
        assert_eq!(keys().len(), 25);
    }
    #[test]
    fn press_and_lift_roundtrip_with_effects() {
        let iden = Iden::new("aria");
        let mut piano = Piano::new();
        let out = piano.act(&iden, &Call::new("piano.press", json!({ "midi": 43 })));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        assert_eq!(
            out.effects[0].to_json(),
            json!({ "kind": "sound", "data": {
                "op": "start", "id": "piano:43", "midi": 43,
                "freq": theory::freq(43), "wave": "sine", "gain": 0.3,
            }})
        );
        assert_eq!(piano.state(&iden)["held"], json!([43]));
        let out = piano.act(&iden, &Call::new("piano.lift", json!({ "midi": 43 })));
        assert!(out.ok);
        assert_eq!(
            out.effects[0].to_json(),
            json!({ "kind": "sound", "data": { "op": "stop", "id": "piano:43" } })
        );
        assert_eq!(piano.state(&iden)["held"], json!([]));
    }
    #[test]
    fn bad_presses_fail_honestly() {
        let iden = Iden::new("aria");
        let mut piano = Piano::new();
        let out = piano.act(&iden, &Call::new("piano.press", json!({ "midi": 44 })));
        assert_eq!(out.note.as_deref(), Some("no such key"));
        piano.act(&iden, &Call::new("piano.press", json!({ "midi": 43 })));
        let out = piano.act(&iden, &Call::new("piano.press", json!({ "midi": 43 })));
        assert_eq!(out.note.as_deref(), Some("already held"));
    }
    #[test]
    fn an_orphan_lift_fails() {
        let iden = Iden::new("aria");
        let mut piano = Piano::new();
        let out = piano.act(&iden, &Call::new("piano.lift", json!({ "midi": 43 })));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("not held"));
    }
    #[test]
    fn wear_switches_the_wave() {
        let iden = Iden::new("aria");
        let mut piano = Piano::new();
        piano.wear(&json!({ "shared": { "settings": { "wave": "square" } } }));
        let out = piano.act(&iden, &Call::new("piano.press", json!({ "midi": 55 })));
        assert_eq!(out.effects[0].data["wave"], json!("square"));
        piano.wear(&json!({}));
        let out = piano.act(&iden, &Call::new("piano.press", json!({ "midi": 57 })));
        assert_eq!(out.effects[0].data["wave"], json!("sine"));
    }
    #[test]
    fn silence_stops_every_held_key() {
        let iden = Iden::new("aria");
        let mut piano = Piano::new();
        piano.act(&iden, &Call::new("piano.press", json!({ "midi": 43 })));
        piano.act(&iden, &Call::new("piano.press", json!({ "midi": 55 })));
        let out = piano.act(&iden, &Call::new("piano.silence", json!({})));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 2);
        assert_eq!(out.effects[1].data["id"], json!("piano:55"));
        assert_eq!(piano.state(&iden)["held"], json!([]));
    }
    #[test]
    fn save_load_roundtrips_and_filters() {
        let iden = Iden::new("aria");
        let mut a = Piano::new();
        a.act(&iden, &Call::new("piano.press", json!({ "midi": 43 })));
        a.act(&iden, &Call::new("piano.press", json!({ "midi": 60 })));
        let mut b = Piano::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden), a.state(&iden));
        let mut c = Piano::new();
        c.load(&json!({ "held": [43, 44, "x"] }));
        assert_eq!(c.state(&iden)["held"], json!([43]));
        c.load(&json!({}));
        assert_eq!(c.state(&iden)["held"], json!([]));
    }
    #[test]
    fn unknown_verb_fails() {
        let iden = Iden::new("aria");
        assert!(
            !Piano::new()
                .act(&iden, &Call::new("piano.tune", json!({})))
                .ok
        );
    }
}
