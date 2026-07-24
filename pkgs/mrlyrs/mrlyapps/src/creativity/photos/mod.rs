use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

const WALL: usize = 12;

pub struct Photos {
    shots: u64,
    waiting: u64,
    photos: Vec<String>,
}

impl Default for Photos {
    fn default() -> Photos {
        Photos::new()
    }
}

impl Photos {
    pub fn new() -> Photos {
        Photos {
            shots: 0,
            waiting: 0,
            photos: Vec::new(),
        }
    }
    fn url(&self) -> String {
        format!("https://picsum.photos/seed/mrly-{}/300/200", self.shots)
    }
}

impl App for Photos {
    fn route(&self) -> &str {
        "photos"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("photos")
            .emoji("📷")
            .category("creativity")
            .internet()
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "shots": self.shots,
            "waiting": self.waiting,
            "photos": self.photos,
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("photos.load", json!({})),
            Verb::new("photos.land", json!({ "data": "base64", "mime": "string" })),
            Verb::new("photos.clear", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "photos.load" => {
                self.shots += 1;
                self.waiting += 1;
                let url = self.url();
                Outcome::ok(json!({ "url": url })).emit(
                    Effect::new("fetch", json!({ "url": url, "as": "bytes" }))
                        .then(Call::new("photos.land", json!({}))),
                )
            }
            "photos.land" => {
                if self.waiting == 0 {
                    return Outcome::fail("nothing loading");
                }
                self.waiting -= 1;
                if let Some(error) = call.arg("error").as_str() {
                    return Outcome::fail(error);
                }
                let Some(data) = call.arg("data").as_str() else {
                    return Outcome::fail("no bytes");
                };
                let mime = call.arg("mime").as_str().unwrap_or("image/jpeg");
                self.photos.insert(0, format!("data:{mime};base64,{data}"));
                self.photos.truncate(WALL);
                Outcome::ok(json!({ "photos": self.photos.len() }))
            }
            "photos.clear" => {
                let count = self.photos.len();
                self.photos.clear();
                Outcome::ok(json!({ "cleared": count }))
            }
            "photos.keep" => {
                let Some(data) = call.arg("data").as_str().filter(|s| !s.is_empty()) else {
                    return Outcome::fail("no data");
                };
                let mime = call.arg("mime").as_str().unwrap_or("image/png");
                self.photos.insert(0, format!("data:{mime};base64,{data}"));
                self.photos.truncate(WALL);
                self.shots += 1;
                Outcome::ok(json!({ "photos": self.photos.len() }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "shots": self.shots,
            "waiting": self.waiting,
            "photos": self.photos,
        })
    }
    fn load(&mut self, state: &Json) {
        self.shots = state["shots"].as_u64().unwrap_or(0);
        self.waiting = state["waiting"].as_u64().unwrap_or(0);
        self.photos = state["photos"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .filter(|s| s.starts_with("data:"))
                    .map(String::from)
                    .take(WALL)
                    .collect()
            })
            .unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    const PNG: &str =
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

    #[test]
    fn load_emits_a_seeded_fetch() {
        let mut p = Photos::new();
        let out = send(&mut p, "photos.load", json!({}));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        let effect = &out.effects[0];
        assert_eq!(effect.kind, "fetch");
        assert_eq!(
            effect.data["url"],
            json!("https://picsum.photos/seed/mrly-1/300/200")
        );
        assert_eq!(effect.data["as"], json!("bytes"));
        assert_eq!(effect.call.as_ref().unwrap().verb, "photos.land");
        let again = send(&mut p, "photos.load", json!({}));
        assert_eq!(
            again.effects[0].data["url"],
            json!("https://picsum.photos/seed/mrly-2/300/200")
        );
    }
    #[test]
    fn landed_bytes_become_a_data_uri() {
        let mut p = Photos::new();
        send(&mut p, "photos.load", json!({}));
        let out = send(
            &mut p,
            "photos.land",
            json!({ "data": PNG, "mime": "image/png" }),
        );
        assert!(out.ok);
        let state = p.state(&iden());
        assert_eq!(state["waiting"], json!(0));
        assert_eq!(
            state["photos"][0],
            json!(format!("data:image/png;base64,{PNG}"))
        );
    }
    #[test]
    fn unrequested_land_fails() {
        let mut p = Photos::new();
        let out = send(&mut p, "photos.land", json!({ "data": PNG }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("nothing loading"));
    }
    #[test]
    fn failed_fetch_lands_honestly() {
        let mut p = Photos::new();
        send(&mut p, "photos.load", json!({}));
        let out = send(&mut p, "photos.land", json!({ "error": "offline" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("offline"));
        let state = p.state(&iden());
        assert_eq!(state["waiting"], json!(0));
        assert_eq!(state["photos"].as_array().unwrap().len(), 0);
    }
    #[test]
    fn empty_land_fails_but_settles() {
        let mut p = Photos::new();
        send(&mut p, "photos.load", json!({}));
        let out = send(&mut p, "photos.land", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no bytes"));
        assert_eq!(p.state(&iden())["waiting"], json!(0));
    }
    #[test]
    fn the_wall_holds_twelve() {
        let mut p = Photos::new();
        for _ in 0..15 {
            send(&mut p, "photos.load", json!({}));
            send(&mut p, "photos.land", json!({ "data": PNG }));
        }
        assert_eq!(p.state(&iden())["photos"].as_array().unwrap().len(), 12);
    }
    #[test]
    fn clear_empties_the_wall() {
        let mut p = Photos::new();
        send(&mut p, "photos.load", json!({}));
        send(&mut p, "photos.land", json!({ "data": PNG }));
        let out = send(&mut p, "photos.clear", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["cleared"], json!(1));
        assert_eq!(p.state(&iden())["photos"].as_array().unwrap().len(), 0);
    }
    #[test]
    fn save_load_roundtrips() {
        let mut a = Photos::new();
        send(&mut a, "photos.load", json!({}));
        send(
            &mut a,
            "photos.land",
            json!({ "data": PNG, "mime": "image/png" }),
        );
        send(&mut a, "photos.load", json!({}));
        let mut b = Photos::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        let mut c = Photos::new();
        c.load(&json!({ "shots": "soup", "photos": ["javascript:alert(1)"] }));
        assert_eq!(c.state(&iden())["shots"], json!(0));
        assert_eq!(c.state(&iden())["photos"].as_array().unwrap().len(), 0);
    }
    #[test]
    fn keep_prepends_and_truncates() {
        let mut p = Photos::new();
        for n in 0..13 {
            let out = send(&mut p, "photos.keep", json!({ "data": format!("shot{n}") }));
            assert!(out.ok);
        }
        let state = p.state(&iden());
        assert_eq!(state["photos"].as_array().unwrap().len(), 12);
        assert_eq!(state["photos"][0], json!("data:image/png;base64,shot12"));
        assert_eq!(state["shots"], json!(13));
    }
    #[test]
    fn keep_requires_data() {
        let mut p = Photos::new();
        let out = send(&mut p, "photos.keep", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no data"));
        let empty = send(&mut p, "photos.keep", json!({ "data": "" }));
        assert!(!empty.ok);
        assert_eq!(p.state(&iden())["photos"].as_array().unwrap().len(), 0);
    }
    #[test]
    fn keep_stays_off_the_verb_surface() {
        let p = Photos::new();
        let names: Vec<String> = p.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["photos.load", "photos.land", "photos.clear"]);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let p = Photos::new();
        let names: Vec<String> = p.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["photos.load", "photos.land", "photos.clear"]);
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Photos::new(), "photos.print", json!({})).ok);
    }
}
