use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Note {
    pub id: u64,
    pub text: String,
}

pub struct Notes {
    items: Vec<Note>,
    next: u64,
    query: String,
}

impl Default for Notes {
    fn default() -> Notes {
        Notes::new()
    }
}

impl Notes {
    pub fn new() -> Notes {
        Notes {
            items: Vec::new(),
            next: 1,
            query: String::new(),
        }
    }
    pub fn add(&mut self, text: &str) -> Option<u64> {
        let text = text.trim();
        if text.is_empty() {
            return None;
        }
        let id = self.next;
        self.next += 1;
        self.items.push(Note {
            id,
            text: text.to_string(),
        });
        Some(id)
    }
    pub fn edit(&mut self, id: u64, text: &str) -> bool {
        let text = text.trim();
        if text.is_empty() {
            return false;
        }
        match self.items.iter_mut().find(|n| n.id == id) {
            Some(note) => {
                note.text = text.to_string();
                true
            }
            None => false,
        }
    }
    pub fn remove(&mut self, id: u64) -> bool {
        match self.items.iter().position(|n| n.id == id) {
            Some(i) => {
                self.items.remove(i);
                true
            }
            None => false,
        }
    }
    pub fn get(&self, id: u64) -> Option<&Note> {
        self.items.iter().find(|n| n.id == id)
    }
    pub fn all(&self) -> &[Note] {
        &self.items
    }
    pub fn search(&mut self, q: &str) {
        self.query = q.trim().to_lowercase();
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn found(&self) -> Vec<&Note> {
        self.items
            .iter()
            .filter(|n| n.text.to_lowercase().contains(&self.query))
            .collect()
    }
    pub fn count(&self) -> usize {
        self.items.len()
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl App for Notes {
    fn route(&self) -> &str {
        "notes"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("notes").emoji("📝").category("creativity")
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "query": self.query,
            "found": self.found().iter().map(|n| json!({ "id": n.id, "text": n.text })).collect::<Vec<_>>(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("notes.add", json!({ "text": "string" })),
            Verb::new("notes.edit", json!({ "id": "u64", "text": "string" })),
            Verb::new("notes.remove", json!({ "id": "u64" })),
            Verb::new("notes.clear", json!({})),
            Verb::new("notes.search", json!({ "q": "string" })),
            Verb::new("notes.export", json!({})),
        ]
    }
    fn save(&self) -> Json {
        json!({
            "items": self.items.iter().map(|n| json!({ "id": n.id, "text": n.text })).collect::<Vec<_>>(),
            "next": self.next,
            "query": self.query,
        })
    }
    fn load(&mut self, state: &Json) {
        self.items = state["items"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|n| {
                        Some(Note {
                            id: n["id"].as_u64()?,
                            text: n["text"].as_str()?.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.next = state["next"].as_u64().unwrap_or(1);
        self.query = state["query"].as_str().unwrap_or("").to_string();
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "notes.add" => match self.add(call.arg("text").as_str().unwrap_or("")) {
                Some(id) => Outcome::ok(json!({ "id": id })),
                None => Outcome::fail("empty note"),
            },
            "notes.edit" => {
                let id = call.arg("id").as_u64().unwrap_or(0);
                if self.edit(id, call.arg("text").as_str().unwrap_or("")) {
                    Outcome::ok(json!({ "id": id }))
                } else {
                    Outcome::fail("no such note")
                }
            }
            "notes.remove" => {
                let id = call.arg("id").as_u64().unwrap_or(0);
                if self.remove(id) {
                    Outcome::ok(json!({ "id": id }))
                } else {
                    Outcome::fail("no such note")
                }
            }
            "notes.clear" => {
                self.clear();
                Outcome::ok(json!({}))
            }
            "notes.search" => {
                self.search(call.arg("q").as_str().unwrap_or(""));
                Outcome::ok(json!({ "q": self.query(), "found": self.found().len() }))
            }
            "notes.export" => {
                let text = self
                    .items
                    .iter()
                    .map(|n| n.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n");
                let data = mrlycore::base64(text.as_bytes());
                Outcome::ok(json!({ "name": "notes.txt" })).emit(Effect::new(
                    "file",
                    json!({ "name": "notes.txt", "mime": "text/plain", "data": data }),
                ))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds_and_lists() {
        let mut n = Notes::new();
        let a = n.add("milk").unwrap();
        let b = n.add("eggs").unwrap();
        assert_ne!(a, b);
        assert_eq!(n.count(), 2);
        assert_eq!(n.all()[0].text, "milk");
        assert_eq!(n.all()[1].text, "eggs");
    }
    #[test]
    fn trims_and_rejects_empty() {
        let mut n = Notes::new();
        assert_eq!(n.add("  hello  "), Some(1));
        assert_eq!(n.get(1).unwrap().text, "hello");
        assert_eq!(n.add("   "), None);
        assert_eq!(n.add(""), None);
        assert_eq!(n.count(), 1);
    }
    #[test]
    fn removes_by_id() {
        let mut n = Notes::new();
        let a = n.add("one").unwrap();
        n.add("two").unwrap();
        assert!(n.remove(a));
        assert!(!n.remove(a));
        assert_eq!(n.count(), 1);
        assert_eq!(n.all()[0].text, "two");
    }
    #[test]
    fn ids_stay_unique_after_remove() {
        let mut n = Notes::new();
        let a = n.add("a").unwrap();
        n.remove(a);
        let b = n.add("b").unwrap();
        assert_ne!(a, b);
    }
    #[test]
    fn edits_in_place() {
        let mut n = Notes::new();
        let a = n.add("draft").unwrap();
        assert!(n.edit(a, "final"));
        assert_eq!(n.get(a).unwrap().text, "final");
        assert!(!n.edit(a, "   "));
        assert!(!n.edit(999, "ghost"));
    }
    #[test]
    fn search_filters_found() {
        let mut n = Notes::new();
        n.add("buy oat milk").unwrap();
        n.add("book the ferry").unwrap();
        n.search("MILK");
        assert_eq!(n.found().len(), 1);
        assert_eq!(n.found()[0].text, "buy oat milk");
        n.search("");
        assert_eq!(n.found().len(), 2);
    }
    #[test]
    fn state_publishes_found_under_the_query() {
        let mut n = Notes::new();
        n.add("buy oat milk").unwrap();
        n.add("book the ferry").unwrap();
        n.search("milk");
        let state = n.state(&Iden::new("aria"));
        assert_eq!(state["query"], "milk");
        assert_eq!(state["found"], json!([{ "id": 1, "text": "buy oat milk" }]));
    }
    #[test]
    fn save_load_roundtrips() {
        let mut a = Notes::new();
        a.add("milk").unwrap();
        let gone = a.add("ghost").unwrap();
        a.add("ferry").unwrap();
        a.remove(gone);
        a.search("milk");
        let mut b = Notes::new();
        b.load(&a.save());
        let iden = Iden::new("aria");
        assert_eq!(b.state(&iden), a.state(&iden));
        assert_eq!(b.save(), a.save());
        assert_eq!(b.add("eggs"), Some(4));
    }
    #[test]
    fn load_survives_garbage() {
        let mut n = Notes::new();
        n.add("milk").unwrap();
        n.load(&json!({ "items": "nope" }));
        assert_eq!(n.count(), 0);
        assert_eq!(n.add("fresh"), Some(1));
    }
    #[test]
    fn clears_all() {
        let mut n = Notes::new();
        n.add("x").unwrap();
        n.add("y").unwrap();
        n.clear();
        assert_eq!(n.count(), 0);
    }
    #[test]
    fn export_emits_a_text_file() {
        let mut n = Notes::new();
        n.add("milk").unwrap();
        n.add("ferry").unwrap();
        let out = n.act(&Iden::new("aria"), &Call::new("notes.export", json!({})));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        let effect = &out.effects[0];
        assert_eq!(effect.kind, "file");
        assert_eq!(effect.data["name"], json!("notes.txt"));
        assert_eq!(effect.data["mime"], json!("text/plain"));
        assert_eq!(effect.data["data"], json!(mrlycore::base64(b"milk\nferry")));
    }
}
