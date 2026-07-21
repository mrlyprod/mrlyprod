use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Emoji {
    category: String,
}

impl Default for Emoji {
    fn default() -> Emoji {
        Emoji::new()
    }
}

impl Emoji {
    pub fn new() -> Emoji {
        Emoji {
            category: crate::core::emoji::first().to_string(),
        }
    }
}

impl App for Emoji {
    fn route(&self) -> &str {
        "emoji"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("emoji").emoji("😀").category("design")
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "category": self.category,
            "categories": crate::core::emoji::names(),
            "grid": crate::core::emoji::grid(&self.category),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new(
            "emoji.set",
            json!({ "key": "category", "value": "string" }),
        )]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "emoji.set" => {
                let key = call.arg("key").as_str().unwrap_or("");
                if key != "category" {
                    return Outcome::fail("no such key");
                }
                let Some(name) = call.arg("value").as_str() else {
                    return Outcome::fail("value must be a string");
                };
                if !crate::core::emoji::has(name) {
                    return Outcome::fail("no such category");
                }
                self.category = name.to_string();
                Outcome::ok(json!({ "key": key, "value": name }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "category": self.category })
    }
    fn load(&mut self, state: &Json) {
        if let Some(c) = state["category"].as_str() {
            if crate::core::emoji::has(c) {
                self.category = c.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

    #[test]
    fn boot_shows_the_first_category() {
        let e = Emoji::new();
        let state = e.state(&iden());
        assert_eq!(state["category"], json!(crate::core::emoji::first()));
        assert!(!state["grid"].as_array().unwrap().is_empty());
    }
    #[test]
    fn set_category_switches_grid() {
        let mut e = Emoji::new();
        assert!(
            send(
                &mut e,
                "emoji.set",
                json!({ "key": "category", "value": "food" })
            )
            .ok
        );
        assert_eq!(e.state(&iden())["category"], json!("food"));
        assert!(
            !send(
                &mut e,
                "emoji.set",
                json!({ "key": "category", "value": "nope" })
            )
            .ok
        );
        assert!(!send(&mut e, "emoji.set", json!({ "key": "nope", "value": "x" })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Emoji::new();
        send(
            &mut a,
            "emoji.set",
            json!({ "key": "category", "value": "food" }),
        );
        let mut b = Emoji::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut e = Emoji::new();
        e.load(&json!({ "category": "nope" }));
        assert_eq!(
            e.state(&iden())["category"],
            json!(crate::core::emoji::first())
        );
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let e = Emoji::new();
        let names: Vec<String> = e.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["emoji.set"]);
    }
}
