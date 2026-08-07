use mrlycore::colors::ROLLABLE;
use mrlycore::colors::{board, hex};
use mrlycore::{json, Json};
use mrlymath::crypto::hash::{digest, fingerprint_cell, Config, Digest, Rule};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};

const SIDE: usize = 64;
const MAX_TEXT: usize = 256;

pub struct Hash {
    text: String,
    rule: Rule,
    dark: bool,
}

impl Default for Hash {
    fn default() -> Hash {
        Hash::new()
    }
}

impl Hash {
    pub fn new() -> Hash {
        Hash {
            text: "mrly".to_string(),
            rule: Rule::Life,
            dark: false,
        }
    }
    fn config(&self) -> Config {
        Config {
            rule: self.rule,
            ..Config::default()
        }
    }
    fn compute(&self) -> Digest {
        digest(self.text.as_bytes(), &self.config()).unwrap()
    }
    fn cells_fact(&self) -> Json {
        let d = self.compute();
        let grid = fingerprint_cell(&d, SIDE);
        let bytes = d.to_bytes();
        let first = bytes.first().copied().unwrap_or(0) as usize;
        let ink = ROLLABLE[first % ROLLABLE.len()];
        let ids: Vec<Vec<u8>> = grid.chunks(SIDE).map(<[u8]>::to_vec).collect();
        json!({
            "ids": ids,
            "skin": "tiles",
            "pens": [hex(board(self.dark)), hex([ink.r, ink.g, ink.b, 255])],
        })
    }
}

impl App for Hash {
    fn route(&self) -> &str {
        "hash"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("hash").emoji("🧬").category("tools")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let d = self.compute();
        json!({
            "text": &self.text,
            "hex": d.hex(),
            "rule": self.rule.name(),
            "cells": self.cells_fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("hash.digest", json!({ "text": "string" })),
            Verb::new(
                "hash.set",
                json!({ "key": "rule", "value": "life | maze | replicator | anneal" }),
            ),
            Verb::new("hash.reset", json!({})),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "hash.digest" => {
                let text = call.arg("text").as_str().unwrap_or("").to_string();
                if text.is_empty() {
                    return Outcome::fail("text must not be empty");
                }
                if text.chars().count() > MAX_TEXT {
                    return Outcome::fail("text must be at most 256 characters");
                }
                self.text = text;
                Outcome::ok(json!({ "text": &self.text }))
            }
            "hash.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                if key != "rule" {
                    return Outcome::fail("no such key");
                }
                let name = call.arg("value").as_str().unwrap_or("");
                match Rule::parse(name) {
                    Some(rule) => {
                        self.rule = rule;
                        Outcome::ok(json!({ "key": "rule", "value": rule.name() }))
                    }
                    None => Outcome::fail("rule must be life, maze, replicator, or anneal"),
                }
            }
            "hash.reset" => {
                *self = Hash::new();
                Outcome::ok(json!({}))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "text": &self.text, "rule": self.rule.name() })
    }
    fn load(&mut self, state: &Json) {
        let mut next = Hash::new();
        if let Some(t) = state["text"].as_str() {
            if !t.is_empty() && t.chars().count() <= MAX_TEXT {
                next.text = t.to_string();
            }
        }
        if let Some(r) = state["rule"].as_str() {
            if let Some(rule) = Rule::parse(r) {
                next.rule = rule;
            }
        }
        *self = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    #[test]
    fn set_validates() {
        let mut h = Hash::new();
        assert!(
            send(
                &mut h,
                "hash.set",
                json!({ "key": "rule", "value": "maze" })
            )
            .ok
        );
        assert!(
            send(
                &mut h,
                "hash.set",
                json!({ "key": "rule", "value": "replicator" })
            )
            .ok
        );
        assert!(
            send(
                &mut h,
                "hash.set",
                json!({ "key": "rule", "value": "anneal" })
            )
            .ok
        );
        assert!(
            send(
                &mut h,
                "hash.set",
                json!({ "key": "rule", "value": "life" })
            )
            .ok
        );
        assert!(
            !send(
                &mut h,
                "hash.set",
                json!({ "key": "rule", "value": "spiral" })
            )
            .ok
        );
        assert!(!send(&mut h, "hash.set", json!({ "key": "text", "value": "hi" })).ok);
    }
    #[test]
    fn digest_validates() {
        let mut h = Hash::new();
        assert!(!send(&mut h, "hash.digest", json!({ "text": "" })).ok);
        let long = "x".repeat(257);
        assert!(!send(&mut h, "hash.digest", json!({ "text": long })).ok);
        let ok = "x".repeat(256);
        assert!(send(&mut h, "hash.digest", json!({ "text": ok })).ok);
    }
    #[test]
    fn digest_changes_the_cells() {
        let mut a = Hash::new();
        send(&mut a, "hash.digest", json!({ "text": "alice" }));
        let mut b = Hash::new();
        send(&mut b, "hash.digest", json!({ "text": "bob" }));
        assert_ne!(
            a.state(&iden(), None)["cells"],
            b.state(&iden(), None)["cells"]
        );
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Hash::new();
        send(
            &mut a,
            "hash.digest",
            json!({ "text": "counting universe" }),
        );
        send(
            &mut a,
            "hash.set",
            json!({ "key": "rule", "value": "maze" }),
        );
        let mut b = Hash::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut h = Hash::new();
        h.load(&json!({ "text": "", "rule": "spiral" }));
        assert_eq!(h.text, "mrly");
        assert_eq!(h.rule.name(), "life");
        h.load(&json!({ "text": "x".repeat(300), "rule": "anneal" }));
        assert_eq!(h.text, "mrly");
        assert_eq!(h.rule.name(), "anneal");
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let h = Hash::new();
        let names: Vec<String> = h.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["hash.digest", "hash.set", "hash.reset"]);
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let h = Hash::new();
        let state = h.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["skin"], json!("tiles"));
        assert_eq!(cells["ids"].as_array().unwrap().len(), SIDE);
        assert_eq!(cells["ids"][0].as_array().unwrap().len(), SIDE);
        let pens = cells["pens"].as_array().unwrap();
        assert_eq!(pens.len(), 2);
        assert!(pens.iter().all(|p| p.as_str().unwrap().starts_with('#')));
        assert!(state.get("frame").is_none());
    }
    #[test]
    fn darkmode_flips_the_empty_pen() {
        let mut h = Hash::new();
        let light = h.state(&iden(), None)["cells"]["pens"][0].clone();
        h.wear(&json!({ "shared": { "settings": { "darkmode": true } } }));
        let dark = h.state(&iden(), None)["cells"]["pens"][0].clone();
        assert_ne!(light, dark);
        assert_eq!(
            h.state(&iden(), None)["cells"]["ids"],
            Hash::new().state(&iden(), None)["cells"]["ids"]
        );
    }
}
