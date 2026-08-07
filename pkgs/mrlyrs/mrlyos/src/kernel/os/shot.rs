use super::Os;
use crate::kernel::app::{Call, Outcome};
use crate::kernel::envelope::Notice;
use mrlycore::image::Image;
use mrlycore::{json, Json};

impl Os {
    pub fn shot(&mut self, image: &Json) -> Outcome {
        let Some(app) = self.focused().map(|r| r.app.clone()) else {
            return Outcome::fail("no current app");
        };
        if app == "photos" {
            return Outcome::fail("nothing to shoot here");
        }
        if Image::from_json(image).is_err() {
            return Outcome::fail("bad image");
        }
        if let Some(pi) = self.find("photos") {
            let iden = self.iden.clone();
            let kept =
                self.apps[pi].call(&iden, &Call::new("photos.keep", json!({ "image": image })));
            if kept.ok {
                self.notices
                    .push(Notice::new("saved", "screenshot → photos", self.now));
            }
        }
        Outcome::ok(json!({ "shot": app }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::app::{App, Verb};
    use crate::kernel::iden::Iden;

    fn picture(shade: &str) -> Json {
        json!({ "width": 1, "height": 1, "rows": [[0]], "palette": [shade] })
    }

    struct Board;

    impl App for Board {
        fn route(&self) -> &str {
            "board"
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            Vec::new()
        }
        fn call(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
            Outcome::ok(json!({}))
        }
    }

    struct Album {
        kept: Vec<Json>,
    }

    impl App for Album {
        fn route(&self) -> &str {
            "photos"
        }
        fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
            json!({ "kept": self.kept.clone() })
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            Vec::new()
        }
        fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
            match call.verb.as_str() {
                "photos.keep" => {
                    self.kept.push(call.arg("image").clone());
                    Outcome::ok(json!({}))
                }
                _ => Outcome::fail("unknown verb"),
            }
        }
    }

    fn boot() -> Os {
        Os::new(Iden::new("aria"))
            .install(Box::new(Board))
            .install(Box::new(Album { kept: Vec::new() }))
    }

    fn kept(os: &Os) -> Json {
        os.read("photos/kept", None).unwrap()
    }

    #[test]
    fn a_carried_image_lands_in_photos_untouched() {
        let mut os = boot();
        os.open("board").unwrap();
        let out = os.call(Call::new(
            "sys.shot",
            json!({ "image": picture("#123456") }),
        ));
        assert!(out.ok);
        assert_eq!(out.data, json!({ "shot": "board" }));
        assert_eq!(kept(&os), json!([picture("#123456")]));
    }

    #[test]
    fn a_bad_image_fails_and_keeps_nothing() {
        let mut os = boot();
        os.open("board").unwrap();
        let out = os.call(Call::new("sys.shot", json!({ "image": { "width": 1 } })));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("bad image"));
        assert_eq!(kept(&os), json!([]));
    }

    #[test]
    fn a_missing_image_is_a_bad_image() {
        let mut os = boot();
        os.open("board").unwrap();
        let out = os.call(Call::new("sys.shot", json!({})));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("bad image"));
        assert_eq!(kept(&os), json!([]));
    }

    #[test]
    fn photos_refuses_its_own_mirror() {
        let mut os = boot();
        os.open("photos").unwrap();
        let out = os.call(Call::new(
            "sys.shot",
            json!({ "image": picture("#123456") }),
        ));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("nothing to shoot here"));
        assert_eq!(kept(&os), json!([]));
    }
}
