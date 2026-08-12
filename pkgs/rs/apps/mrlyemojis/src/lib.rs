#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use mrlycore::{json, Json};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};

/// The emoji catalog: categories, membership tests and grids.
pub mod data;

/// The emojis app: one open category and a capped library of favorites.
pub struct Emojis {
    category: String,
    library: Vec<String>,
}

impl Default for Emojis {
    fn default() -> Emojis {
        Emojis::new()
    }
}

impl Emojis {
    /// Opens the picker on the first category with the four seed favorites.
    pub fn new() -> Emojis {
        Emojis {
            category: crate::data::first().to_string(),
            library: seed(),
        }
    }
}

fn seed() -> Vec<String> {
    ["🍎", "🍏", "⭐", "🎲"]
        .iter()
        .map(|&e| e.to_string())
        .collect()
}

impl App for Emojis {
    fn route(&self) -> &str {
        "emojis"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("emojis").emoji("😀").category("design")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "category": &self.category,
            "categories": crate::data::names(),
            "grid": crate::data::grid(&self.category),
            "library": self.library.clone(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new(
                "emojis.set",
                json!({
                    "key": { "category": crate::data::names().join(" | ") },
                    "value": "of key",
                }),
            ),
            Verb::new("emojis.keep", json!({ "value": "string" })),
            Verb::new("emojis.drop", json!({ "value": "string" })),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "emojis.set" => {
                let key = call.arg("key").as_str().unwrap_or("");
                if key != "category" {
                    return Outcome::fail("no such key");
                }
                let Some(name) = call.arg("value").as_str() else {
                    return Outcome::fail("value must be a string");
                };
                if !crate::data::has(name) {
                    return Outcome::fail("no such category");
                }
                self.category = name.to_string();
                Outcome::ok(json!({ "key": key, "value": name }))
            }
            "emojis.keep" => {
                let value = call.arg("value").as_str().unwrap_or("");
                if !crate::data::known(value) {
                    return Outcome::fail("unknown emoji");
                }
                if self.library.iter().any(|v| v == value) {
                    return Outcome::fail("already kept");
                }
                if self.library.len() >= 24 {
                    return Outcome::fail("library is full");
                }
                self.library.push(value.to_string());
                Outcome::ok(json!({ "value": value }))
            }
            "emojis.drop" => {
                let value = call.arg("value").as_str().unwrap_or("");
                match self.library.iter().position(|v| v == value) {
                    Some(i) => {
                        self.library.remove(i);
                        Outcome::ok(json!({ "value": value }))
                    }
                    None => Outcome::fail("not in the library"),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "category": &self.category, "library": self.library.clone() })
    }
    fn load(&mut self, state: &Json) {
        if let Some(c) = state["category"].as_str() {
            if crate::data::has(c) {
                self.category = c.to_string();
            }
        }
        self.library = match state["library"].as_array() {
            Some(items) => {
                let mut library: Vec<String> = Vec::new();
                for item in items {
                    if let Some(value) = item.as_str() {
                        if crate::data::known(value)
                            && !library.iter().any(|v| v == value)
                            && library.len() < 24
                        {
                            library.push(value.to_string());
                        }
                    }
                }
                library
            }
            None => seed(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    #[test]
    fn boot_shows_the_first_category() {
        let e = Emojis::new();
        let state = e.state(&iden(), None);
        assert_eq!(state["category"], json!(crate::data::first()));
        assert!(!state["grid"].as_array().unwrap().is_empty());
    }
    #[test]
    fn set_category_switches_grid() {
        let mut e = Emojis::new();
        assert!(
            send(
                &mut e,
                "emojis.set",
                json!({ "key": "category", "value": "food" })
            )
            .ok
        );
        assert_eq!(e.state(&iden(), None)["category"], json!("food"));
        assert!(
            !send(
                &mut e,
                "emojis.set",
                json!({ "key": "category", "value": "nope" })
            )
            .ok
        );
        assert!(!send(&mut e, "emojis.set", json!({ "key": "nope", "value": "x" })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Emojis::new();
        send(
            &mut a,
            "emojis.set",
            json!({ "key": "category", "value": "food" }),
        );
        let mut b = Emojis::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut e = Emojis::new();
        e.load(&json!({ "category": "nope" }));
        assert_eq!(
            e.state(&iden(), None)["category"],
            json!(crate::data::first())
        );
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let e = Emojis::new();
        let names: Vec<String> = e.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["emojis.set", "emojis.keep", "emojis.drop"]);
    }
    #[test]
    fn library_seeds_are_known() {
        let e = Emojis::new();
        let library = e.state(&iden(), None)["library"].clone();
        assert_eq!(library, json!(["🍎", "🍏", "⭐", "🎲"]));
        for value in library.as_array().unwrap() {
            assert!(crate::data::known(value.as_str().unwrap()));
        }
    }
    #[test]
    fn keep_and_drop_round_trip() {
        let mut e = Emojis::new();
        assert!(send(&mut e, "emojis.keep", json!({ "value": "🍐" })).ok);
        assert!(e.state(&iden(), None)["library"]
            .as_array()
            .unwrap()
            .contains(&json!("🍐")));
        let dup = send(&mut e, "emojis.keep", json!({ "value": "🍐" }));
        assert!(!dup.ok);
        assert_eq!(dup.note.as_deref(), Some("already kept"));
        let unknown = send(&mut e, "emojis.keep", json!({ "value": "x" }));
        assert_eq!(unknown.note.as_deref(), Some("unknown emoji"));
        assert!(send(&mut e, "emojis.drop", json!({ "value": "🍐" })).ok);
        assert!(!e.state(&iden(), None)["library"]
            .as_array()
            .unwrap()
            .contains(&json!("🍐")));
        let gone = send(&mut e, "emojis.drop", json!({ "value": "🍐" }));
        assert_eq!(gone.note.as_deref(), Some("not in the library"));
    }
    #[test]
    fn library_caps_at_twenty_four() {
        let mut e = Emojis::new();
        for &value in crate::data::grid("food") {
            send(&mut e, "emojis.keep", json!({ "value": value }));
        }
        assert_eq!(
            e.state(&iden(), None)["library"].as_array().unwrap().len(),
            24
        );
        let full = send(&mut e, "emojis.keep", json!({ "value": "🌋" }));
        assert!(!full.ok);
        assert_eq!(full.note.as_deref(), Some("library is full"));
    }
    #[test]
    fn load_sanitizes_the_library() {
        let mut e = Emojis::new();
        e.load(&json!({ "library": "garbage" }));
        assert_eq!(
            e.state(&iden(), None)["library"],
            json!(["🍎", "🍏", "⭐", "🎲"])
        );
        e.load(&json!({ "library": ["🍎", "notemoji", "🍎", "⭐"] }));
        assert_eq!(e.state(&iden(), None)["library"], json!(["🍎", "⭐"]));
    }
}
