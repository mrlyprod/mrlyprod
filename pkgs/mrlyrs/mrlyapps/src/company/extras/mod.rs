use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub const SOCIALS: [(&str, &str); 3] = [
    ("instagram", "https://instagram.com/mrlyprod"),
    ("reddit", "https://reddit.com/r/mrlyprod"),
    ("twitter", "https://twitter.com/mrlyprod"),
];

pub const ACTIONS: [(&str, &str); 2] = [
    (
        "donate",
        "https://donate.stripe.com/dRm3cu3XLfHj19e6WW5kk00",
    ),
    ("help", "mailto:help@mrlyprod.com"),
];

pub const LINES: [&str; 5] = [
    "this is the way",
    "why is the secret",
    "#mrlywear #wearmrly",
    "#mrlyshop #shopmrly",
    "powered by mrly",
];

pub const PAGES: [(&str, &str); 2] = [("privacy", "/privacy"), ("terms", "/terms")];

pub const COPYRIGHT: &str = "copyright \u{a9} 2026 mrlyprod, inc. all rights reserved.";

pub struct Extras {
    index: usize,
}

impl Default for Extras {
    fn default() -> Extras {
        Extras::new()
    }
}

impl Extras {
    pub fn new() -> Extras {
        Extras { index: 0 }
    }
    fn links(pairs: &[(&str, &str)]) -> Json {
        json!(pairs
            .iter()
            .map(|(name, url)| json!({ "name": name, "url": url }))
            .collect::<Vec<Json>>())
    }
}

impl App for Extras {
    fn route(&self) -> &str {
        "extras"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("extras").emoji("🔗").category("company")
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "socials": Extras::links(&SOCIALS),
            "actions": Extras::links(&ACTIONS),
            "pages": Extras::links(&PAGES),
            "cycle": { "lines": LINES, "index": self.index },
            "copyright": COPYRIGHT,
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new("extras.cycle", json!({}))]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "extras.cycle" => {
                self.index = (self.index + 1) % LINES.len();
                Outcome::ok(json!({ "index": self.index }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "index": self.index })
    }
    fn load(&mut self, state: &Json) {
        if let Some(index) = state["index"].as_u64() {
            if (index as usize) < LINES.len() {
                self.index = index as usize;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    #[test]
    fn actions_offer_the_natural_verb() {
        let e = Extras::new();
        let names: Vec<String> = e.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["extras.cycle"]);
    }
    #[test]
    fn cycle_advances_and_wraps() {
        let mut e = Extras::new();
        for i in 1..LINES.len() {
            let out = send(&mut e, "extras.cycle", json!({}));
            assert!(out.ok);
            assert_eq!(out.data["index"], json!(i));
        }
        let out = send(&mut e, "extras.cycle", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["index"], json!(0));
    }
    #[test]
    fn state_carries_the_fixed_content() {
        let e = Extras::new();
        let state = e.state(&iden());
        assert_eq!(state["socials"][0]["name"], json!("instagram"));
        assert_eq!(
            state["socials"][0]["url"],
            json!("https://instagram.com/mrlyprod")
        );
        assert_eq!(state["socials"][2]["name"], json!("twitter"));
        assert_eq!(state["actions"][0]["name"], json!("donate"));
        assert_eq!(state["actions"][1]["name"], json!("help"));
        assert_eq!(
            state["actions"][1]["url"],
            json!("mailto:help@mrlyprod.com")
        );
        assert_eq!(state["pages"][0]["name"], json!("privacy"));
        assert_eq!(state["pages"][1]["url"], json!("/terms"));
        assert_eq!(state["cycle"]["lines"][0], json!("this is the way"));
        assert_eq!(state["cycle"]["index"], json!(0));
        assert_eq!(state["copyright"], json!(COPYRIGHT));
    }
    #[test]
    fn save_load_continues() {
        let mut a = Extras::new();
        send(&mut a, "extras.cycle", json!({}));
        send(&mut a, "extras.cycle", json!({}));
        let mut b = Extras::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        send(&mut a, "extras.cycle", json!({}));
        send(&mut b, "extras.cycle", json!({}));
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut e = Extras::new();
        e.load(&json!({ "index": "soup" }));
        assert_eq!(e.state(&iden())["cycle"]["index"], json!(0));
        e.load(&json!({ "index": 99 }));
        assert_eq!(e.state(&iden())["cycle"]["index"], json!(0));
        e.load(&json!({ "index": 3 }));
        assert_eq!(e.state(&iden())["cycle"]["index"], json!(3));
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Extras::new(), "extras.jump", json!({})).ok);
    }
}
