#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use mrlycore::{json, Json};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};

const SHELF: usize = 24;

/// One shelved file, held as base64 text.
pub struct File {
    /// The name the file was shelved under.
    pub name: String,
    /// The media type, plain binary when none was given.
    pub mime: String,
    /// The content as base64 text.
    pub data: String,
    /// The moment the file arrived.
    pub tick: i64,
}

/// The files app: a capped shelf of recent files, newest first.
pub struct Files {
    items: Vec<File>,
}

impl Default for Files {
    fn default() -> Files {
        Files::new()
    }
}

impl Files {
    /// Builds an empty shelf.
    pub fn new() -> Files {
        Files { items: Vec::new() }
    }
}

fn bytes(data: &str) -> usize {
    let pad = data.bytes().rev().take_while(|&b| b == b'=').count();
    (data.len() / 4 * 3).saturating_sub(pad)
}

impl App for Files {
    fn route(&self) -> &str {
        "files"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("files").emoji("📁").category("system")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "count": self.items.len(),
            "files": self.items.iter().map(|f| json!({
                "name": &f.name,
                "mime": &f.mime,
                "uri": format!("data:{};base64,{}", f.mime, f.data),
                "size": bytes(&f.data),
                "tick": f.tick,
            })).collect::<Vec<_>>(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("files.drop", json!({ "index": "number" })),
            Verb::new("files.clear", json!({})),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "files.keep" => {
                let Some(name) = call.arg("name").as_str().filter(|s| !s.is_empty()) else {
                    return Outcome::fail("no name");
                };
                let Some(data) = call.arg("data").as_str().filter(|s| !s.is_empty()) else {
                    return Outcome::fail("no data");
                };
                let mime = call
                    .arg("mime")
                    .as_str()
                    .unwrap_or("application/octet-stream");
                self.items.insert(
                    0,
                    File {
                        name: name.to_string(),
                        mime: mime.to_string(),
                        data: data.to_string(),
                        tick: call.now.unwrap_or(0),
                    },
                );
                self.items.truncate(SHELF);
                Outcome::ok(json!({ "files": self.items.len() }))
            }
            "files.drop" => match call.arg("index").as_u64().map(|n| n as usize) {
                Some(i) if i < self.items.len() => {
                    self.items.remove(i);
                    Outcome::ok(json!({ "files": self.items.len() }))
                }
                _ => Outcome::fail("no such file"),
            },
            "files.clear" => {
                let count = self.items.len();
                self.items.clear();
                Outcome::ok(json!({ "cleared": count }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "files": self.items.iter().map(|f| json!({
                "name": &f.name,
                "mime": &f.mime,
                "data": &f.data,
                "tick": f.tick,
            })).collect::<Vec<_>>(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.items = state["files"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|f| {
                        Some(File {
                            name: f["name"].as_str()?.to_string(),
                            mime: f["mime"].as_str()?.to_string(),
                            data: f["data"].as_str()?.to_string(),
                            tick: f["tick"].as_i64().unwrap_or(0),
                        })
                    })
                    .take(SHELF)
                    .collect()
            })
            .unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    fn keep(f: &mut Files, name: &str) -> Outcome {
        f.call(
            &iden(),
            &Call::new(
                "files.keep",
                json!({ "name": name, "mime": "text/plain", "data": "aGVsbG8=" }),
            ),
        )
    }

    #[test]
    fn keep_prepends_and_truncates() {
        let mut f = Files::new();
        for n in 0..27 {
            assert!(keep(&mut f, &format!("f{n}.txt")).ok);
        }
        let state = f.state(&iden(), None);
        assert_eq!(state["count"], json!(24));
        assert_eq!(state["files"].as_array().unwrap().len(), 24);
        assert_eq!(state["files"][0]["name"], json!("f26.txt"));
    }
    #[test]
    fn keep_derives_uri_and_size() {
        let mut f = Files::new();
        keep(&mut f, "hi.txt");
        let state = f.state(&iden(), None);
        assert_eq!(
            state["files"][0]["uri"],
            json!("data:text/plain;base64,aGVsbG8=")
        );
        assert_eq!(state["files"][0]["size"], json!(5));
    }
    #[test]
    fn keep_stamps_the_tick() {
        let mut f = Files::new();
        let out = f.call(
            &iden(),
            &Call::new(
                "files.keep",
                json!({ "name": "a.txt", "mime": "text/plain", "data": "aGk=" }),
            )
            .at(5000),
        );
        assert!(out.ok);
        assert_eq!(f.state(&iden(), None)["files"][0]["tick"], json!(5000));
    }
    #[test]
    fn keep_requires_name_and_data() {
        let mut f = Files::new();
        assert!(!send(&mut f, "files.keep", json!({ "data": "aGk=" })).ok);
        assert!(!send(&mut f, "files.keep", json!({ "name": "a.txt" })).ok);
        assert!(!send(&mut f, "files.keep", json!({ "name": "a.txt", "data": "" })).ok);
        assert_eq!(f.state(&iden(), None)["count"], json!(0));
    }
    #[test]
    fn drop_removes_by_index() {
        let mut f = Files::new();
        keep(&mut f, "a.txt");
        keep(&mut f, "b.txt");
        let out = send(&mut f, "files.drop", json!({ "index": 0 }));
        assert!(out.ok);
        assert_eq!(f.state(&iden(), None)["files"][0]["name"], json!("a.txt"));
        assert!(!send(&mut f, "files.drop", json!({ "index": 9 })).ok);
    }
    #[test]
    fn clear_empties_the_shelf() {
        let mut f = Files::new();
        keep(&mut f, "a.txt");
        let out = send(&mut f, "files.clear", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["cleared"], json!(1));
        assert_eq!(f.state(&iden(), None)["count"], json!(0));
    }
    #[test]
    fn keep_stays_off_the_verb_surface() {
        let f = Files::new();
        let names: Vec<String> = f.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["files.drop", "files.clear"]);
    }
    #[test]
    fn save_load_roundtrips_and_filters_junk() {
        let mut a = Files::new();
        a.call(
            &iden(),
            &Call::new(
                "files.keep",
                json!({ "name": "a.txt", "mime": "text/plain", "data": "aGk=" }),
            )
            .at(5000),
        );
        let mut b = Files::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        let mut c = Files::new();
        c.load(&json!({ "files": [{ "name": 7, "mime": "x", "data": "y" }, "nope"] }));
        assert_eq!(c.state(&iden(), None)["count"], json!(0));
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Files::new(), "files.print", json!({})).ok);
    }
}
