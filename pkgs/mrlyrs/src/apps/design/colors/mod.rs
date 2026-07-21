use crate::core::colors::{NAMES, PALETTE};
use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Colors {
    index: usize,
}

impl Default for Colors {
    fn default() -> Colors {
        Colors::new()
    }
}

impl Colors {
    pub fn new() -> Colors {
        Colors { index: 0 }
    }
}

impl App for Colors {
    fn route(&self) -> &str {
        "colors"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("colors").emoji("🌈").category("design")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let color = PALETTE[self.index];
        json!({
            "index": self.index,
            "count": PALETTE.len(),
            "name": NAMES[self.index],
            "hex": color.to_hex(),
            "rgb": { "r": color.r, "g": color.g, "b": color.b },
            "palette": NAMES
                .iter()
                .zip(PALETTE.iter())
                .map(|(name, color)| json!({ "name": name, "hex": color.to_hex() }))
                .collect::<Vec<_>>(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("colors.page", json!({ "dir": "next | prev" })),
            Verb::new("colors.set", json!({ "key": "name", "value": "string" })),
            Verb::new("colors.reset", json!({})),
            Verb::new("colors.export", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "colors.page" => {
                let next = match call.arg("dir").as_str() {
                    Some("next") => (self.index + 1) % PALETTE.len(),
                    Some("prev") => (self.index + PALETTE.len() - 1) % PALETTE.len(),
                    _ => return Outcome::fail("dir must be next or prev"),
                };
                self.index = next;
                Outcome::ok(json!({ "name": NAMES[self.index] }))
            }
            "colors.set" => {
                let key = call.arg("key").as_str().unwrap_or("");
                if key != "name" {
                    return Outcome::fail("no such key");
                }
                let value = call.arg("value").as_str().unwrap_or("");
                match NAMES.iter().position(|&n| n == value) {
                    Some(i) => {
                        self.index = i;
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    None => Outcome::fail("unknown color name"),
                }
            }
            "colors.reset" => {
                self.index = 0;
                Outcome::ok(json!({}))
            }
            "colors.export" => {
                let palette: Vec<Json> = NAMES
                    .iter()
                    .zip(PALETTE.iter())
                    .map(|(name, color)| json!({ "name": name, "hex": color.to_hex() }))
                    .collect();
                let text = serde_json::to_string_pretty(&json!(palette)).unwrap_or_default();
                let data = crate::core::base64(text.as_bytes());
                Outcome::ok(json!({ "name": "palette.json" })).emit(Effect::new(
                    "file",
                    json!({ "name": "palette.json", "mime": "application/json", "data": data }),
                ))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "index": self.index })
    }
    fn load(&mut self, state: &Json) {
        self.index = state["index"]
            .as_u64()
            .filter(|&i| (i as usize) < PALETTE.len())
            .map(|i| i as usize)
            .unwrap_or(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

    #[test]
    fn set_validates() {
        let mut c = Colors::new();
        assert!(
            send(
                &mut c,
                "colors.set",
                json!({ "key": "name", "value": "teal" })
            )
            .ok
        );
        assert!(
            !send(
                &mut c,
                "colors.set",
                json!({ "key": "name", "value": "beige" })
            )
            .ok
        );
        assert!(
            !send(
                &mut c,
                "colors.set",
                json!({ "key": "hue", "value": "teal" })
            )
            .ok
        );
    }
    #[test]
    fn page_cycles() {
        let mut c = Colors::new();
        assert_eq!(c.index, 0);
        send(&mut c, "colors.page", json!({ "dir": "prev" }));
        assert_eq!(c.index, PALETTE.len() - 1);
        send(&mut c, "colors.page", json!({ "dir": "next" }));
        assert_eq!(c.index, 0);
        assert!(!send(&mut c, "colors.page", json!({ "dir": "sideways" })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Colors::new();
        send(
            &mut a,
            "colors.set",
            json!({ "key": "name", "value": "indigo" }),
        );
        let mut b = Colors::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut c = Colors::new();
        send(
            &mut c,
            "colors.set",
            json!({ "key": "name", "value": "gray" }),
        );
        c.load(&json!({ "index": 999 }));
        assert_eq!(c.index, 0);
        c.load(&json!({ "index": "nonsense" }));
        assert_eq!(c.index, 0);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let c = Colors::new();
        let names: Vec<String> = c.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["colors.page", "colors.set", "colors.reset", "colors.export"]
        );
    }
    #[test]
    fn export_emits_a_palette_file() {
        let mut c = Colors::new();
        let out = send(&mut c, "colors.export", json!({}));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        let effect = &out.effects[0];
        assert_eq!(effect.kind, "file");
        assert_eq!(effect.data["name"], json!("palette.json"));
        assert_eq!(effect.data["mime"], json!("application/json"));
        assert!(!effect.data["data"].as_str().unwrap().is_empty());
    }
    #[test]
    fn facts_align_with_palette() {
        let mut c = Colors::new();
        for (i, &name) in NAMES.iter().enumerate() {
            send(
                &mut c,
                "colors.set",
                json!({ "key": "name", "value": name }),
            );
            let state = c.state(&iden());
            assert_eq!(state["name"], json!(NAMES[i]));
            assert_eq!(state["hex"], json!(PALETTE[i].to_hex()));
        }
    }
}
