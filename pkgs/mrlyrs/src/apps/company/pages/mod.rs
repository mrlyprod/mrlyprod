use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Pages {
    slug: String,
    md: String,
    mode: String,
    status: String,
}

impl Default for Pages {
    fn default() -> Pages {
        Pages::new()
    }
}

impl Pages {
    pub fn new() -> Pages {
        Pages {
            slug: String::new(),
            md: String::new(),
            mode: "preview".to_string(),
            status: "empty".to_string(),
        }
    }
    fn source(&self) -> String {
        if self.slug.is_empty() || self.slug == "dummy" {
            return String::new();
        }
        format!("/cdn/pages/{}.md", self.slug)
    }
}

fn clean(slug: &str) -> bool {
    if slug.is_empty() {
        return false;
    }
    if slug.starts_with('/') || slug.ends_with('/') || slug.contains("//") {
        return false;
    }
    slug.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '-' || c == '_')
}

impl App for Pages {
    fn route(&self) -> &str {
        "pages"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("pages")
            .emoji("📄")
            .category("company")
            .internet()
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "slug": self.slug,
            "md": self.md,
            "mode": self.mode,
            "status": self.status,
            "source": self.source(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("pages.open", json!({ "slug": "string" })),
            Verb::new("pages.land", json!({ "data": "text" })),
            Verb::new("pages.flip", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "pages.open" => {
                let Some(slug) = call.arg("slug").as_str() else {
                    return Outcome::fail("no slug");
                };
                if !clean(slug) {
                    return Outcome::fail("bad slug");
                }
                self.slug = slug.to_string();
                self.mode = "preview".to_string();
                if slug == "dummy" {
                    self.md = include_str!("dummy.md").to_string();
                    self.status = "ready".to_string();
                    return Outcome::ok(json!({ "slug": self.slug }));
                }
                self.md.clear();
                self.status = "loading".to_string();
                let url = self.source();
                Outcome::ok(json!({ "slug": self.slug, "url": url })).emit(
                    Effect::new("fetch", json!({ "url": url, "as": "text" }))
                        .then(Call::new("pages.land", json!({}))),
                )
            }
            "pages.land" => {
                if self.status != "loading" {
                    return Outcome::fail("nothing loading");
                }
                if let Some(error) = call.arg("error").as_str() {
                    self.status = "error".to_string();
                    return Outcome::fail(error);
                }
                let Some(data) = call.arg("data").as_str() else {
                    return Outcome::fail("no text");
                };
                if data.trim_start().starts_with('<') {
                    self.status = "error".to_string();
                    return Outcome::fail("not found");
                }
                self.md = data.to_string();
                self.status = "ready".to_string();
                Outcome::ok(json!({ "chars": self.md.len() }))
            }
            "pages.flip" => {
                self.mode = if self.mode == "preview" {
                    "code".to_string()
                } else {
                    "preview".to_string()
                };
                Outcome::ok(json!({ "mode": self.mode }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "slug": self.slug,
            "md": self.md,
            "mode": self.mode,
            "status": self.status,
        })
    }
    fn load(&mut self, state: &Json) {
        self.slug = state["slug"].as_str().unwrap_or("").to_string();
        self.md = state["md"].as_str().unwrap_or("").to_string();
        self.mode = match state["mode"].as_str() {
            Some("preview") | Some("code") => state["mode"].as_str().unwrap().to_string(),
            _ => "preview".to_string(),
        };
        self.status = match state["status"].as_str() {
            Some("empty") | Some("loading") | Some("ready") | Some("error") => {
                state["status"].as_str().unwrap().to_string()
            }
            _ => "empty".to_string(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

    #[test]
    fn dummy_opens_offline() {
        let mut p = Pages::new();
        let out = send(&mut p, "pages.open", json!({ "slug": "dummy" }));
        assert!(out.ok);
        assert!(out.effects.is_empty());
        assert_eq!(out.data["slug"], json!("dummy"));
        let state = p.state(&iden());
        assert_eq!(state["status"], json!("ready"));
        assert_eq!(state["mode"], json!("preview"));
        assert_eq!(state["source"], json!(""));
        assert!(state["md"].as_str().unwrap().contains("# "));
    }
    #[test]
    fn open_emits_a_fetch() {
        let mut p = Pages::new();
        let out = send(&mut p, "pages.open", json!({ "slug": "privacy" }));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        let effect = &out.effects[0];
        assert_eq!(effect.kind, "fetch");
        assert_eq!(effect.data["url"], json!("/cdn/pages/privacy.md"));
        assert_eq!(effect.data["as"], json!("text"));
        assert_eq!(effect.call.as_ref().unwrap().verb, "pages.land");
        let state = p.state(&iden());
        assert_eq!(state["status"], json!("loading"));
        assert_eq!(state["source"], json!("/cdn/pages/privacy.md"));
    }
    #[test]
    fn land_stores_markdown() {
        let mut p = Pages::new();
        send(&mut p, "pages.open", json!({ "slug": "privacy" }));
        let out = send(&mut p, "pages.land", json!({ "data": "# Privacy\n\nHi." }));
        assert!(out.ok);
        assert_eq!(out.data["chars"], json!(14));
        let state = p.state(&iden());
        assert_eq!(state["status"], json!("ready"));
        assert_eq!(state["md"], json!("# Privacy\n\nHi."));
    }
    #[test]
    fn land_rejects_the_spa_shell() {
        let mut p = Pages::new();
        send(&mut p, "pages.open", json!({ "slug": "missing" }));
        let out = send(
            &mut p,
            "pages.land",
            json!({ "data": "<!doctype html><html></html>" }),
        );
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("not found"));
        assert_eq!(p.state(&iden())["status"], json!("error"));
    }
    #[test]
    fn land_error_fails_honestly() {
        let mut p = Pages::new();
        send(&mut p, "pages.open", json!({ "slug": "privacy" }));
        let out = send(&mut p, "pages.land", json!({ "error": "offline" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("offline"));
        let state = p.state(&iden());
        assert_eq!(state["status"], json!("error"));
        assert_eq!(state["md"], json!(""));
    }
    #[test]
    fn unrequested_land_fails() {
        let mut p = Pages::new();
        let out = send(&mut p, "pages.land", json!({ "data": "# hi" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("nothing loading"));
    }
    #[test]
    fn land_without_text_fails() {
        let mut p = Pages::new();
        send(&mut p, "pages.open", json!({ "slug": "privacy" }));
        let out = send(&mut p, "pages.land", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no text"));
    }
    #[test]
    fn flip_toggles_mode() {
        let mut p = Pages::new();
        let out = send(&mut p, "pages.flip", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["mode"], json!("code"));
        let back = send(&mut p, "pages.flip", json!({}));
        assert_eq!(back.data["mode"], json!("preview"));
    }
    #[test]
    fn bad_slugs_all_fail() {
        for slug in ["", "../x", "a//b", "/x", "x/", "a b", "x?y"] {
            let mut p = Pages::new();
            let out = send(&mut p, "pages.open", json!({ "slug": slug }));
            assert!(!out.ok, "slug {slug:?} should fail");
            assert_eq!(out.note.as_deref(), Some("bad slug"));
        }
    }
    #[test]
    fn open_without_slug_fails() {
        let mut p = Pages::new();
        let out = send(&mut p, "pages.open", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no slug"));
    }
    #[test]
    fn save_load_roundtrips() {
        let mut a = Pages::new();
        send(&mut a, "pages.open", json!({ "slug": "dummy" }));
        send(&mut a, "pages.flip", json!({}));
        let mut b = Pages::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
    }
    #[test]
    fn malformed_load_falls_back() {
        let mut p = Pages::new();
        p.load(&json!({ "slug": 7, "md": true, "mode": "wild", "status": "soup" }));
        let state = p.state(&iden());
        assert_eq!(state["slug"], json!(""));
        assert_eq!(state["md"], json!(""));
        assert_eq!(state["mode"], json!("preview"));
        assert_eq!(state["status"], json!("empty"));
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Pages::new(), "pages.close", json!({})).ok);
    }
    #[test]
    fn actions_offer_the_three_verbs() {
        let p = Pages::new();
        let names: Vec<String> = p.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["pages.open", "pages.land", "pages.flip"]);
    }
}
