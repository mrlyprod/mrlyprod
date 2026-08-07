use mrlycore::rng::Rng;
use mrlycore::{json, Json};
use mrlymusic::cue;
use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};

pub const SIDES: [u32; 7] = [2, 4, 6, 8, 10, 12, 20];

const SURFACES: [&str; 2] = ["grid", "canvas"];
const SKINS: [&str; 3] = ["tiles", "emojis", "digits"];
const HISTORY: usize = 8;

pub struct Dice {
    rng: Rng,
    seed: u64,
    steps: u64,
    sides: u32,
    face: u32,
    rolls: Vec<u32>,
    surface: String,
    skin: String,
}

impl Default for Dice {
    fn default() -> Dice {
        Dice::new()
    }
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            sides: 6,
            face: 1,
            rolls: Vec::new(),
            surface: "grid".to_string(),
            skin: "digits".to_string(),
        }
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.face = 1;
        self.rolls.clear();
    }
    fn cells_fact(&self) -> Json {
        json!({
            "ids": [[self.face]],
            "skin": &self.skin,
            "pens": [],
        })
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "sides" => {
                let n = value.as_u64().ok_or("value must be an integer")?;
                if !SIDES.contains(&(n as u32)) {
                    return Err("no such die");
                }
                self.sides = n as u32;
                Ok(json!(n))
            }
            "surface" | "skin" => {
                let s = value.as_str().ok_or("value must be a string")?;
                let legal: &[&str] = if key == "surface" { &SURFACES } else { &SKINS };
                if !legal.contains(&s) {
                    return Err("no such option");
                }
                match key {
                    "surface" => self.surface = s.to_string(),
                    _ => self.skin = s.to_string(),
                }
                Ok(json!(s))
            }
            _ => Err("no such key"),
        }
    }
}

impl App for Dice {
    fn route(&self) -> &str {
        "dice"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("dice").emoji("🎲").category("tools")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "steps": self.steps,
            "seed": self.seed,
            "settings": {
                "sides": self.sides,
                "surface": &self.surface,
                "skin": &self.skin,
            },
            "face": self.face,
            "nonce": self.steps,
            "rolls": self.rolls.clone(),
            "cells": self.cells_fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("dice.roll", json!({})),
            Verb::new("dice.reset", json!({ "seed": "int" })),
            Verb::new("dice.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "dice.roll" => {
                self.face = self.rng.range(1, self.sides as i64) as u32;
                self.steps += 1;
                self.rolls.insert(0, self.face);
                self.rolls.truncate(HISTORY);
                Outcome::ok(json!({ "face": self.face }))
                    .emit(Effect::new("sound", cue::payload("blip")))
            }
            "dice.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "dice.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        if key == "sides" {
                            let seed = self.seed;
                            self.reset(seed);
                        }
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": {
                "sides": self.sides,
                "surface": &self.surface,
                "skin": &self.skin,
            },
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "face": self.face,
            "rolls": self.rolls.clone(),
        })
    }
    fn load(&mut self, state: &Json) {
        if let Some(sides) = state["settings"]["sides"].as_u64() {
            if SIDES.contains(&(sides as u32)) {
                self.sides = sides as u32;
            }
        }
        self.surface = "grid".to_string();
        self.skin = "digits".to_string();
        if let Some(surface) = state["settings"]["surface"].as_str() {
            if SURFACES.contains(&surface) {
                self.surface = surface.to_string();
            }
        }
        if let Some(skin) = state["settings"]["skin"].as_str() {
            if SKINS.contains(&skin) {
                self.skin = skin.to_string();
            }
        }
        self.reset(state["seed"].as_u64().unwrap_or(0));
        self.steps = state["steps"].as_u64().unwrap_or(0);
        if let Some(face) = state["face"].as_u64() {
            if (1..=self.sides as u64).contains(&face) {
                self.face = face as u32;
            }
        }
        if let Some(arr) = state["rolls"].as_array() {
            self.rolls = arr
                .iter()
                .filter_map(|v| v.as_u64())
                .filter(|&r| r >= 1 && r <= self.sides as u64)
                .map(|r| r as u32)
                .take(HISTORY)
                .collect();
        }
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn dice(seed: u64) -> Dice {
        seeded(Dice::new(), "dice.reset", seed)
    }

    #[test]
    fn roll_within_range() {
        let mut d = dice(7);
        send(&mut d, "dice.set", json!({ "key": "sides", "value": 20 }));
        for _ in 0..1000 {
            let out = send(&mut d, "dice.roll", json!({}));
            assert!(out.ok);
            let face = out.data["face"].as_u64().unwrap();
            assert!((1..=20).contains(&face));
        }
    }
    #[test]
    fn seed_reproduces() {
        let mut a = dice(42);
        let mut b = dice(42);
        for _ in 0..32 {
            assert_eq!(
                send(&mut a, "dice.roll", json!({})).data,
                send(&mut b, "dice.roll", json!({})).data
            );
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn roll_blips() {
        let mut d = dice(7);
        let out = send(&mut d, "dice.roll", json!({}));
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.effects[0].kind, "sound");
        assert_eq!(out.effects[0].data, cue::payload("blip"));
    }
    #[test]
    fn nonce_tracks_the_roll_count() {
        let mut d = dice(3);
        assert_eq!(d.state(&iden(), None)["nonce"], json!(0));
        send(&mut d, "dice.roll", json!({}));
        send(&mut d, "dice.roll", json!({}));
        assert_eq!(d.state(&iden(), None)["nonce"], json!(2));
    }
    #[test]
    fn set_validates_the_seven() {
        let mut d = dice(7);
        for sides in SIDES {
            assert!(
                send(
                    &mut d,
                    "dice.set",
                    json!({ "key": "sides", "value": sides })
                )
                .ok
            );
        }
        let out = send(&mut d, "dice.set", json!({ "key": "sides", "value": 7 }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no such die"));
        assert!(
            !send(
                &mut d,
                "dice.set",
                json!({ "key": "sides", "value": "soup" })
            )
            .ok
        );
        assert!(!send(&mut d, "dice.set", json!({ "key": "color", "value": 6 })).ok);
    }
    #[test]
    fn look_keys_take_emojis_on_either_surface() {
        let mut d = dice(7);
        send(&mut d, "dice.roll", json!({}));
        for (key, value) in [("surface", "canvas"), ("skin", "emojis")] {
            assert!(send(&mut d, "dice.set", json!({ "key": key, "value": value })).ok);
        }
        let state = d.state(&iden(), None);
        assert_eq!(state["steps"], json!(1));
        assert_eq!(state["cells"]["skin"], json!("emojis"));
        let frame = mrlyui::skin::raster("dice", &state["cells"], 12, false).expect("a frame");
        assert_eq!(frame.width, mrlyui::skin::SYMBOL);
        assert!(!send(&mut d, "dice.set", json!({ "key": "skin", "value": "wax" })).ok);
    }
    #[test]
    fn set_sides_rebuilds_the_round() {
        let mut d = dice(7);
        send(&mut d, "dice.roll", json!({}));
        send(&mut d, "dice.roll", json!({}));
        send(&mut d, "dice.set", json!({ "key": "sides", "value": 12 }));
        let state = d.state(&iden(), None);
        assert_eq!(state["steps"], json!(0));
        assert_eq!(state["face"], json!(1));
        assert_eq!(state["rolls"], json!([]));
        assert_eq!(state["settings"]["sides"], json!(12));
        assert_eq!(state["seed"], json!(7));
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut d = Dice::new();
        let out = d.call(&iden(), &Call::new("dice.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(d.state(&iden(), None)["seed"], json!(5000));
    }
    #[test]
    fn rolls_remember_the_last_eight() {
        let mut d = dice(3);
        let mut faces = Vec::new();
        for _ in 0..12 {
            faces.push(send(&mut d, "dice.roll", json!({})).data["face"].clone());
        }
        faces.reverse();
        faces.truncate(8);
        let state = d.state(&iden(), None);
        assert_eq!(state["rolls"], Json::Arr(faces));
        assert_eq!(state["steps"], json!(12));
    }
    #[test]
    fn save_load_continues() {
        let mut a = dice(11);
        send(&mut a, "dice.set", json!({ "key": "sides", "value": 20 }));
        send(
            &mut a,
            "dice.set",
            json!({ "key": "skin", "value": "tiles" }),
        );
        for _ in 0..5 {
            send(&mut a, "dice.roll", json!({}));
        }
        let mut b = Dice::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        for d in [&mut a, &mut b] {
            send(d, "dice.roll", json!({}));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn old_saves_default_to_the_legacy_look() {
        let mut d = Dice::new();
        d.load(&json!({ "seed": 3, "settings": { "sides": 20 } }));
        let settings = d.state(&iden(), None)["settings"].clone();
        assert_eq!(settings["sides"], json!(20));
        assert_eq!(settings["surface"], json!("grid"));
        assert_eq!(settings["skin"], json!("digits"));
    }
    #[test]
    fn load_survives_garbage() {
        let mut d = Dice::new();
        d.load(&json!({ "seed": "soup", "settings": { "sides": 7 }, "face": 99, "rolls": 3 }));
        let state = d.state(&iden(), None);
        assert_eq!(state["seed"], json!(0));
        assert_eq!(state["settings"]["sides"], json!(6));
        assert_eq!(state["face"], json!(1));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let d = Dice::new();
        let names: Vec<String> = d.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["dice.roll", "dice.reset", "dice.set"]);
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Dice::new(), "dice.juggle", json!({})).ok);
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let mut d = dice(5);
        send(&mut d, "dice.roll", json!({}));
        let state = d.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["ids"][0][0], state["face"]);
        assert_eq!(cells["skin"], json!("digits"));
        assert_eq!(cells["pens"].as_array().unwrap().len(), 0);
        assert!(cells.get("design").is_none());
        assert!(state.get("frame").is_none());
        assert!(state.get("sprite").is_none());
        assert!(state.get("skin").is_none());
    }
}
