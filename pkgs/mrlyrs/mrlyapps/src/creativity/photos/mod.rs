use mrlycore::image::Image;
use mrlycore::{json, Json};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};

const WALL: usize = 12;

pub struct Photos {
    shots: u64,
    photos: Vec<Json>,
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
            photos: Vec::new(),
        }
    }
}

impl App for Photos {
    fn route(&self) -> &str {
        "photos"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("photos").emoji("📷").category("creativity")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "shots": self.shots,
            "photos": self.photos.clone(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new("photos.clear", json!({}))]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "photos.clear" => {
                let count = self.photos.len();
                self.photos.clear();
                Outcome::ok(json!({ "cleared": count }))
            }
            "photos.keep" => {
                let Ok(image) = Image::from_json(call.arg("image")) else {
                    return Outcome::fail("no image");
                };
                self.photos.insert(0, image.to_json());
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
            "photos": self.photos.clone(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.shots = state["shots"].as_u64().unwrap_or(0);
        self.photos = state["photos"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| Image::from_json(v).ok())
                    .map(|image| image.to_json())
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

    fn shot(n: usize) -> Json {
        json!({
            "width": 1,
            "height": 1,
            "rows": [[0]],
            "palette": [format!("#{:02x}00ff", n % 256)],
        })
    }

    #[test]
    fn keep_prepends_and_truncates() {
        let mut p = Photos::new();
        for n in 0..13 {
            let out = send(&mut p, "photos.keep", json!({ "image": shot(n) }));
            assert!(out.ok);
        }
        let state = p.state(&iden(), None);
        assert_eq!(state["photos"].as_array().unwrap().len(), 12);
        assert_eq!(state["photos"][0], shot(12));
        assert_eq!(state["shots"], json!(13));
    }
    #[test]
    fn keep_requires_an_image() {
        let mut p = Photos::new();
        let out = send(&mut p, "photos.keep", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no image"));
        let ragged = send(
            &mut p,
            "photos.keep",
            json!({ "image": { "width": 2, "height": 2, "rows": [[0]], "palette": ["#ffffff"] } }),
        );
        assert!(!ragged.ok);
        assert_eq!(
            p.state(&iden(), None)["photos"].as_array().unwrap().len(),
            0
        );
    }
    #[test]
    fn clear_empties_the_wall() {
        let mut p = Photos::new();
        send(&mut p, "photos.keep", json!({ "image": shot(1) }));
        let out = send(&mut p, "photos.clear", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["cleared"], json!(1));
        assert_eq!(
            p.state(&iden(), None)["photos"].as_array().unwrap().len(),
            0
        );
    }
    #[test]
    fn save_load_roundtrips() {
        let mut a = Photos::new();
        send(&mut a, "photos.keep", json!({ "image": shot(1) }));
        send(&mut a, "photos.keep", json!({ "image": shot(2) }));
        let mut b = Photos::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        let mut c = Photos::new();
        c.load(&json!({ "shots": "soup", "photos": ["javascript:alert(1)", { "width": 1 }] }));
        assert_eq!(c.state(&iden(), None)["shots"], json!(0));
        assert_eq!(
            c.state(&iden(), None)["photos"].as_array().unwrap().len(),
            0
        );
    }
    #[test]
    fn keep_stays_off_the_verb_surface() {
        let p = Photos::new();
        let names: Vec<String> = p.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["photos.clear"]);
    }
    #[test]
    fn the_wall_needs_no_internet() {
        assert!(!Photos::new().manifest().internet);
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Photos::new(), "photos.load", json!({})).ok);
        assert!(!send(&mut Photos::new(), "photos.print", json!({})).ok);
    }
}
