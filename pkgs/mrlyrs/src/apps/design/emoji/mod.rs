use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Emoji {
    category: String,
    work: String,
}

impl Default for Emoji {
    fn default() -> Emoji {
        Emoji::new()
    }
}

impl Emoji {
    pub fn new() -> Emoji {
        let category = crate::core::emoji::first().to_string();
        let work = crate::core::emoji::grid(&category)
            .first()
            .copied()
            .unwrap_or("😀")
            .to_string();
        Emoji { category, work }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "category" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if !crate::core::emoji::has(name) {
                    return Err("no such category");
                }
                self.category = name.to_string();
                Ok(json!(name))
            }
            "work" => {
                let g = value.as_str().ok_or("value must be a string")?;
                if !crate::core::emoji::is_grapheme(g) {
                    return Err("not a single grapheme");
                }
                self.work = g.to_string();
                Ok(json!(g))
            }
            _ => Err("no such key"),
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
            "work": self.work,
            "categories": crate::core::emoji::names(),
            "grid": crate::core::emoji::grid(&self.category),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new(
            "emoji.set",
            json!({ "key": "category | work", "value": "string" }),
        )]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "emoji.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "category": self.category, "work": self.work })
    }
    fn load(&mut self, state: &Json) {
        if let Some(c) = state["category"].as_str() {
            if crate::core::emoji::has(c) {
                self.category = c.to_string();
            }
        }
        if let Some(w) = state["work"].as_str() {
            if crate::core::emoji::is_grapheme(w) {
                self.work = w.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

    #[test]
    fn boot_is_a_valid_grapheme() {
        let e = Emoji::new();
        let state = e.state(&iden());
        assert_eq!(state["category"], json!(crate::core::emoji::first()));
        assert!(crate::core::emoji::is_grapheme(
            state["work"].as_str().unwrap()
        ));
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
    }
    #[test]
    fn set_work_accepts_any_grapheme() {
        let mut e = Emoji::new();
        assert!(send(&mut e, "emoji.set", json!({ "key": "work", "value": "🎉" })).ok);
        assert_eq!(e.state(&iden())["work"], json!("🎉"));
        assert!(send(&mut e, "emoji.set", json!({ "key": "work", "value": "❤️" })).ok);
        assert_eq!(e.state(&iden())["work"], json!("❤️"));
        assert!(
            !send(
                &mut e,
                "emoji.set",
                json!({ "key": "work", "value": "a b" })
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
        send(&mut a, "emoji.set", json!({ "key": "work", "value": "🍎" }));
        let mut b = Emoji::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let e = Emoji::new();
        let names: Vec<String> = e.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["emoji.set"]);
    }
}
