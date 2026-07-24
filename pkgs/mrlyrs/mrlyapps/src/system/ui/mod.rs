use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub const PICKS: [&str; 3] = ["alpha", "beta", "gamma"];

pub struct Ui {
    sample: String,
    overlay: bool,
    toggle: bool,
    pick: String,
    span: f64,
}

impl Default for Ui {
    fn default() -> Ui {
        Ui::new()
    }
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            sample: "the quick brown fox".to_string(),
            overlay: false,
            toggle: false,
            pick: "alpha".to_string(),
            span: 5.0,
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "sample" => {
                let sample = value.as_str().ok_or("value must be a string")?.trim();
                if sample.is_empty() {
                    return Err("empty value");
                }
                self.sample = sample.to_string();
                Ok(json!(sample))
            }
            "overlay" | "toggle" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                match key {
                    "overlay" => self.overlay = on,
                    _ => self.toggle = on,
                }
                Ok(json!(on))
            }
            "pick" => {
                let pick = value.as_str().ok_or("value must be a string")?;
                if !PICKS.contains(&pick) {
                    return Err("no such option");
                }
                self.pick = pick.to_string();
                Ok(json!(pick))
            }
            "span" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                if !(0.0..=10.0).contains(&n) {
                    return Err("out of range");
                }
                self.span = n;
                Ok(json!(n))
            }
            _ => Err("no such key"),
        }
    }
}

impl App for Ui {
    fn route(&self) -> &str {
        "ui"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("ui")
            .emoji("🧪")
            .title("specimen")
            .category("system")
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "sample": self.sample,
            "overlay": self.overlay,
            "toggle": self.toggle,
            "pick": self.pick,
            "span": self.span,
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new(
            "ui.set",
            json!({ "key": "string", "value": "any" }),
        )]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "ui.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_carries_every_control() {
        let m = Ui::new();
        let state = m.state(&Iden::new("aria"));
        assert_eq!(state["sample"], json!("the quick brown fox"));
        assert_eq!(state["overlay"], json!(false));
        assert_eq!(state["toggle"], json!(false));
        assert_eq!(state["pick"], json!("alpha"));
        assert_eq!(state["span"], json!(5.0));
    }
    #[test]
    fn act_applies_key_and_value() {
        let iden = Iden::new("aria");
        let mut m = Ui::new();
        let out = m.act(
            &iden,
            &Call::new("ui.set", json!({ "key": "pick", "value": "beta" })),
        );
        assert!(out.ok);
        assert_eq!(out.data, json!({ "key": "pick", "value": "beta" }));
        assert_eq!(m.state(&iden)["pick"], json!("beta"));
        let out = m.act(
            &iden,
            &Call::new("ui.set", json!({ "key": "volume", "value": 1 })),
        );
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no such key"));
    }
    #[test]
    fn apply_holds_the_bounds() {
        let mut m = Ui::new();
        assert_eq!(m.apply("pick", &json!("delta")), Err("no such option"));
        assert_eq!(m.apply("span", &json!(11)), Err("out of range"));
        assert_eq!(m.apply("span", &json!(-1)), Err("out of range"));
        assert_eq!(m.apply("sample", &json!("  ")), Err("empty value"));
        assert_eq!(
            m.apply("toggle", &json!("yes")),
            Err("value must be a bool")
        );
        assert_eq!(m.apply("sample", &json!(" trimmed ")), Ok(json!("trimmed")));
        assert_eq!(m.apply("span", &json!(0)), Ok(json!(0.0)));
    }
    #[test]
    fn a_gallery_saves_nothing() {
        let mut m = Ui::new();
        m.apply("toggle", &json!(true)).unwrap();
        assert_eq!(m.save(), Json::Null);
        m.load(&json!({ "toggle": true }));
        assert_eq!(m.state(&Iden::new("aria"))["toggle"], json!(true));
    }
}
