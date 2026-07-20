use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Log {
    ring: Vec<Json>,
}

impl Default for Log {
    fn default() -> Log {
        Log::new()
    }
}

impl Log {
    pub fn new() -> Log {
        Log { ring: Vec::new() }
    }
}

impl App for Log {
    fn route(&self) -> &str {
        "log"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("log").emoji("📜").category("system")
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({ "entries": self.ring.iter().rev().cloned().collect::<Vec<_>>() })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new("log.export", json!({}))]
    }
    fn wear(&mut self, world: &Json) {
        self.ring = world["ring"].as_array().cloned().unwrap_or_default();
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "log.export" => {
                let mut md = String::from("# session\n\n");
                for entry in &self.ring {
                    let verb = entry["verb"].as_str().unwrap_or("");
                    let tick = entry["tick"].as_u64().unwrap_or(0);
                    md.push_str(&format!("{tick}. `{verb}`"));
                    let args = &entry["args"];
                    if !args.as_object().map(|o| o.is_empty()).unwrap_or(true) {
                        md.push_str(&format!(" `{args}`"));
                    }
                    md.push('\n');
                }
                let data = crate::core::base64(md.as_bytes());
                Outcome::ok(json!({ "name": "session.md" })).emit(Effect::new(
                    "file",
                    json!({ "name": "session.md", "mime": "text/markdown", "data": data }),
                ))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        Json::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> Json {
        json!({
            "apps": [],
            "ring": [
                { "verb": "nav.open", "args": { "app": "notes" }, "now": 5000, "tick": 1 },
                { "verb": "notes.add", "args": { "text": "milk" }, "now": 5000, "tick": 2 },
            ],
        })
    }

    #[test]
    fn wear_copies_the_ring() {
        let mut log = Log::new();
        log.wear(&world());
        assert_eq!(log.ring.len(), 2);
    }
    #[test]
    fn state_lists_entries_newest_first() {
        let mut log = Log::new();
        log.wear(&world());
        let state = log.state(&Iden::new("aria"));
        assert_eq!(state["entries"][0]["verb"], "notes.add");
        assert_eq!(state["entries"][0]["tick"], json!(2));
        assert_eq!(state["entries"][1]["verb"], "nav.open");
    }
    #[test]
    fn bare_wear_empties_the_log() {
        let mut log = Log::new();
        log.wear(&world());
        log.wear(&json!({}));
        assert_eq!(log.state(&Iden::new("aria"))["entries"], json!([]));
    }
    #[test]
    fn any_verb_fails() {
        let mut log = Log::new();
        assert!(
            !log.act(&Iden::new("aria"), &Call::new("log.fly", json!({})))
                .ok
        );
    }
    #[test]
    fn save_stays_null() {
        let mut log = Log::new();
        log.wear(&world());
        assert_eq!(log.save(), Json::Null);
    }
    #[test]
    fn export_emits_a_markdown_file() {
        let mut log = Log::new();
        log.wear(&world());
        let out = log.act(&Iden::new("aria"), &Call::new("log.export", json!({})));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        let effect = &out.effects[0];
        assert_eq!(effect.kind, "file");
        assert_eq!(effect.data["name"], json!("session.md"));
        assert_eq!(effect.data["mime"], json!("text/markdown"));
        let expected = "# session\n\n1. `nav.open` `{\"app\":\"notes\"}`\n2. `notes.add` `{\"text\":\"milk\"}`\n";
        assert_eq!(
            effect.data["data"],
            json!(crate::core::base64(expected.as_bytes()))
        );
    }
}
