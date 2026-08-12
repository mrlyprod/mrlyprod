#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use mrlycore::audio;
use mrlycore::{json, Json};
use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};

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
            let midi = audio::ROOT + 12 * (k / 7) + audio::MAJOR[(k % 7) as usize];
            k += 1;
            Some(midi)
        })
        .collect()
}

/// A grid of G major keys, sounding for as long as each one is held.
pub struct Studio {
    held: Vec<i64>,
    wave: String,
}

impl Default for Studio {
    fn default() -> Studio {
        Studio::new()
    }
}

impl Studio {
    /// Opens the keyboard with nothing held and the wave defaulting to sine.
    pub fn new() -> Studio {
        Studio {
            held: Vec::new(),
            wave: "sine".to_string(),
        }
    }
    fn start(&self, midi: i64) -> Effect {
        Effect::new(
            "sound",
            json!({
                "op": "start",
                "id": format!("studio:{midi}"),
                "midi": midi,
                "freq": audio::freq(midi),
                "wave": &self.wave,
                "gain": audio::VOLUME,
            }),
        )
    }
    fn stop(midi: i64) -> Effect {
        Effect::new(
            "sound",
            json!({ "op": "stop", "id": format!("studio:{midi}") }),
        )
    }
}

impl App for Studio {
    fn route(&self) -> &str {
        "studio"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("studio").emoji("🎹").category("creativity")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let board: Vec<Json> = keys()
            .iter()
            .map(|slot| match slot {
                Some(midi) => json!({
                    "midi": *midi,
                    "name": audio::name(*midi),
                    "held": self.held.contains(midi),
                }),
                None => Json::Null,
            })
            .collect();
        json!({ "cols": 5, "keys": board, "held": self.held.clone() })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("studio.press", json!({ "midi": "number" })),
            Verb::new("studio.lift", json!({ "midi": "number" })),
            Verb::new("studio.silence", json!({})),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "studio.press" => {
                let midi = call.arg("midi").as_i64().unwrap_or(-1);
                if !keys().contains(&Some(midi)) {
                    return Outcome::fail("no such key");
                }
                if self.held.contains(&midi) {
                    return Outcome::fail("already held");
                }
                self.held.push(midi);
                Outcome::ok(json!({ "midi": midi, "held": self.held.clone() }))
                    .emit(self.start(midi))
            }
            "studio.lift" => {
                let midi = call.arg("midi").as_i64().unwrap_or(-1);
                if !self.held.contains(&midi) {
                    return Outcome::fail("not held");
                }
                self.held.retain(|&m| m != midi);
                Outcome::ok(json!({ "midi": midi, "held": self.held.clone() }))
                    .emit(Studio::stop(midi))
            }
            "studio.silence" => {
                let mut out = Outcome::ok(json!({ "held": [] }));
                for &midi in &self.held {
                    out = out.emit(Studio::stop(midi));
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
        json!({ "held": self.held.clone() })
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
        let mut studio = Studio::new();
        let out = studio.call(&iden, &Call::new("studio.press", json!({ "midi": 43 })));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        assert_eq!(
            out.effects[0].to_json(),
            json!({ "kind": "sound", "data": {
                "op": "start", "id": "studio:43", "midi": 43,
                "freq": audio::freq(43), "wave": "sine", "gain": 30,
            }})
        );
        assert_eq!(studio.state(&iden, None)["held"], json!([43]));
        let out = studio.call(&iden, &Call::new("studio.lift", json!({ "midi": 43 })));
        assert!(out.ok);
        assert_eq!(
            out.effects[0].to_json(),
            json!({ "kind": "sound", "data": { "op": "stop", "id": "studio:43" } })
        );
        assert_eq!(studio.state(&iden, None)["held"], json!([]));
    }
    #[test]
    fn bad_presses_fail_honestly() {
        let iden = Iden::new("aria");
        let mut studio = Studio::new();
        let out = studio.call(&iden, &Call::new("studio.press", json!({ "midi": 44 })));
        assert_eq!(out.note.as_deref(), Some("no such key"));
        studio.call(&iden, &Call::new("studio.press", json!({ "midi": 43 })));
        let out = studio.call(&iden, &Call::new("studio.press", json!({ "midi": 43 })));
        assert_eq!(out.note.as_deref(), Some("already held"));
    }
    #[test]
    fn an_orphan_lift_fails() {
        let iden = Iden::new("aria");
        let mut studio = Studio::new();
        let out = studio.call(&iden, &Call::new("studio.lift", json!({ "midi": 43 })));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("not held"));
    }
    #[test]
    fn wear_switches_the_wave() {
        let iden = Iden::new("aria");
        let mut studio = Studio::new();
        studio.wear(&json!({ "shared": { "settings": { "wave": "square" } } }));
        let out = studio.call(&iden, &Call::new("studio.press", json!({ "midi": 55 })));
        assert_eq!(out.effects[0].data["wave"], json!("square"));
        studio.wear(&json!({}));
        let out = studio.call(&iden, &Call::new("studio.press", json!({ "midi": 57 })));
        assert_eq!(out.effects[0].data["wave"], json!("sine"));
    }
    #[test]
    fn silence_stops_every_held_key() {
        let iden = Iden::new("aria");
        let mut studio = Studio::new();
        studio.call(&iden, &Call::new("studio.press", json!({ "midi": 43 })));
        studio.call(&iden, &Call::new("studio.press", json!({ "midi": 55 })));
        let out = studio.call(&iden, &Call::new("studio.silence", json!({})));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 2);
        assert_eq!(out.effects[1].data["id"], json!("studio:55"));
        assert_eq!(studio.state(&iden, None)["held"], json!([]));
    }
    #[test]
    fn save_load_roundtrips_and_filters() {
        let iden = Iden::new("aria");
        let mut a = Studio::new();
        a.call(&iden, &Call::new("studio.press", json!({ "midi": 43 })));
        a.call(&iden, &Call::new("studio.press", json!({ "midi": 60 })));
        let mut b = Studio::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden, None), a.state(&iden, None));
        let mut c = Studio::new();
        c.load(&json!({ "held": [43, 44, "x"] }));
        assert_eq!(c.state(&iden, None)["held"], json!([43]));
        c.load(&json!({}));
        assert_eq!(c.state(&iden, None)["held"], json!([]));
    }
    #[test]
    fn unknown_verb_fails() {
        let iden = Iden::new("aria");
        assert!(
            !Studio::new()
                .call(&iden, &Call::new("studio.tune", json!({})))
                .ok
        );
    }
}
