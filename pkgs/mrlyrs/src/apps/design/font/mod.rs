use crate::core::rng::Rng;
use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Font {
    order: Vec<char>,
    at: usize,
    reveal: Option<Reveal>,
    library: Vec<char>,
}

struct Reveal {
    pixels: Vec<usize>,
    shown: usize,
}

impl Default for Font {
    fn default() -> Font {
        Font::new()
    }
}

impl Font {
    pub fn new() -> Font {
        Font {
            order: crate::font::supported(),
            at: 0,
            reveal: None,
            library: seed(),
        }
    }
}

fn seed() -> Vec<char> {
    vec!['A', 'a', '0', '#']
}

impl App for Font {
    fn route(&self) -> &str {
        "font"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("font").emoji("🔤").category("design")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let c = self.order[self.at];
        let g = crate::font::glyph(c).unwrap();
        let width = g.width();
        let height = g.height();
        let rows = match &self.reveal {
            Some(reveal) => {
                let mut grid = vec![vec![0u8; width]; height];
                for &idx in reveal.pixels.iter().take(reveal.shown) {
                    grid[idx / width][idx % width] = 1;
                }
                grid
            }
            None => crate::font::to_lists(&g),
        };
        json!({
            "char": c.to_string(),
            "name": crate::font::name_of(c),
            "index": self.at,
            "total": self.order.len(),
            "revealing": self.reveal.is_some(),
            "glyph": { "text": c.to_string(), "width": width, "height": height, "rows": rows },
            "library": self.library.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("font.page", json!({ "dir": "next | prev" })),
            Verb::new("font.pick", json!({ "char": "string" })),
            Verb::new("font.scramble", json!({})),
            Verb::new("font.tick", json!({})),
            Verb::new(
                "font.export",
                json!({ "format": "json | ttf | woff | woff2" }),
            ),
            Verb::new("font.keep", json!({ "char": "string" })),
            Verb::new("font.drop", json!({ "char": "string" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "font.page" => {
                self.at = match call.arg("dir").as_str() {
                    Some("next") => (self.at + 1) % self.order.len(),
                    Some("prev") => (self.at + self.order.len() - 1) % self.order.len(),
                    _ => return Outcome::fail("dir must be next or prev"),
                };
                self.reveal = None;
                Outcome::ok(json!({ "char": self.order[self.at].to_string() }))
            }
            "font.pick" => {
                let Some(s) = call.arg("char").as_str() else {
                    return Outcome::fail("no such glyph");
                };
                let mut chars = s.chars();
                let (Some(c), None) = (chars.next(), chars.next()) else {
                    return Outcome::fail("no such glyph");
                };
                let Some(pos) = self.order.iter().position(|&x| x == c) else {
                    return Outcome::fail("no such glyph");
                };
                self.at = pos;
                self.reveal = None;
                Outcome::ok(json!({ "char": c.to_string() }))
            }
            "font.scramble" => {
                let c = self.order[self.at];
                let g = crate::font::glyph(c).unwrap();
                let width = g.width();
                let rows = crate::font::to_lists(&g);
                let mut pixels: Vec<usize> = Vec::new();
                for (y, row) in rows.iter().enumerate() {
                    for (x, &v) in row.iter().enumerate() {
                        if v == 1 {
                            pixels.push(y * width + x);
                        }
                    }
                }
                let seed = call.now.unwrap_or(0).max(0) as u64;
                let mut rng = Rng::new(seed);
                for i in (1..pixels.len()).rev() {
                    let j = rng.below(i + 1);
                    pixels.swap(i, j);
                }
                let n = pixels.len();
                self.reveal = Some(Reveal { pixels, shown: 0 });
                Outcome::ok(json!({ "pixels": n }))
            }
            "font.tick" => {
                let Some(reveal) = self.reveal.as_mut() else {
                    return Outcome::fail("nothing revealing");
                };
                reveal.shown += 1;
                let shown = reveal.shown;
                let done = shown >= reveal.pixels.len();
                if done {
                    self.reveal = None;
                }
                Outcome::ok(json!({ "shown": shown, "done": done }))
            }
            "font.export" => {
                let (bytes, mime): (&[u8], &str) = match call.arg("format").as_str() {
                    Some("json") => (
                        include_bytes!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/../../files/mrlyfont/MrlyFont.json"
                        )),
                        "application/json",
                    ),
                    Some("ttf") => (
                        include_bytes!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/../../files/mrlyfont/MrlyFont.ttf"
                        )),
                        "font/ttf",
                    ),
                    Some("woff") => (
                        include_bytes!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/../../files/mrlyfont/MrlyFont.woff"
                        )),
                        "font/woff",
                    ),
                    Some("woff2") => (
                        include_bytes!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/../../files/mrlyfont/MrlyFont.woff2"
                        )),
                        "font/woff2",
                    ),
                    _ => return Outcome::fail("no such format"),
                };
                let name = format!("MrlyFont.{}", call.arg("format").as_str().unwrap());
                let data = crate::core::base64(bytes);
                Outcome::ok(json!({ "name": name.clone() })).emit(Effect::new(
                    "file",
                    json!({ "name": name, "mime": mime, "data": data }),
                ))
            }
            "font.keep" => {
                let arg = call.arg("char");
                let c = if arg.is_null() {
                    self.order[self.at]
                } else {
                    let Some(s) = arg.as_str() else {
                        return Outcome::fail("unknown char");
                    };
                    let mut chars = s.chars();
                    let (Some(c), None) = (chars.next(), chars.next()) else {
                        return Outcome::fail("unknown char");
                    };
                    c
                };
                if crate::font::glyph(c).is_none() {
                    return Outcome::fail("unknown char");
                }
                if self.library.contains(&c) {
                    return Outcome::fail("already kept");
                }
                if self.library.len() >= 24 {
                    return Outcome::fail("library is full");
                }
                self.library.push(c);
                Outcome::ok(json!({ "char": c.to_string() }))
            }
            "font.drop" => {
                let Some(s) = call.arg("char").as_str() else {
                    return Outcome::fail("not in the library");
                };
                let mut chars = s.chars();
                let (Some(c), None) = (chars.next(), chars.next()) else {
                    return Outcome::fail("not in the library");
                };
                match self.library.iter().position(|&x| x == c) {
                    Some(i) => {
                        self.library.remove(i);
                        Outcome::ok(json!({ "char": c.to_string() }))
                    }
                    None => Outcome::fail("not in the library"),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        if self.reveal.is_some() {
            Some(Call::new("font.tick", json!({})))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        json!({
            "char": self.order[self.at].to_string(),
            "library": self.library.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.reveal = None;
        if let Some(s) = state["char"].as_str() {
            let mut chars = s.chars();
            if let (Some(c), None) = (chars.next(), chars.next()) {
                if let Some(pos) = self.order.iter().position(|&x| x == c) {
                    self.at = pos;
                }
            }
        }
        self.library = match state["library"].as_array() {
            Some(items) => {
                let mut library: Vec<char> = Vec::new();
                for item in items {
                    if let Some(s) = item.as_str() {
                        let mut chars = s.chars();
                        if let (Some(c), None) = (chars.next(), chars.next()) {
                            if crate::font::glyph(c).is_some()
                                && !library.contains(&c)
                                && library.len() < 24
                            {
                                library.push(c);
                            }
                        }
                    }
                }
                library
            }
            None => seed(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

    #[test]
    fn next_and_prev_wrap() {
        let mut f = Font::new();
        let total = f.order.len();
        let out = send(&mut f, "font.page", json!({ "dir": "next" }));
        assert!(out.ok);
        assert_eq!(f.state(&iden())["index"], json!(1));
        let mut f = Font::new();
        let out = send(&mut f, "font.page", json!({ "dir": "prev" }));
        assert!(out.ok);
        assert_eq!(f.state(&iden())["index"], json!(total - 1));
    }
    #[test]
    fn pick_finds_and_rejects() {
        let mut f = Font::new();
        let out = send(&mut f, "font.pick", json!({ "char": "a" }));
        assert!(out.ok);
        assert_eq!(f.state(&iden())["char"], json!("a"));
        let out = send(&mut f, "font.pick", json!({ "char": "zz" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no such glyph"));
        let out = send(&mut f, "font.pick", json!({ "char": "€" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no such glyph"));
        assert_eq!(f.state(&iden())["char"], json!("a"));
    }
    #[test]
    fn scramble_and_tick_reveal_progressively() {
        let mut f = Font::new();
        send(&mut f, "font.pick", json!({ "char": "a" }));
        let out = f.act(&iden(), &Call::new("font.scramble", json!({})).at(42));
        assert!(out.ok);
        let pixels = out.data["pixels"].as_u64().unwrap();
        assert!(pixels > 0);
        for i in 1..=pixels {
            let tick = send(&mut f, "font.tick", json!({}));
            assert!(tick.ok);
            assert_eq!(tick.data["shown"], json!(i));
            let state = f.state(&iden());
            let sum: u64 = state["glyph"]["rows"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|r| r.as_array().unwrap())
                .map(|v| v.as_u64().unwrap())
                .sum();
            assert_eq!(sum, i);
            if i == pixels {
                assert_eq!(tick.data["done"], json!(true));
                assert_eq!(state["revealing"], json!(false));
            } else {
                assert_eq!(tick.data["done"], json!(false));
                assert_eq!(state["revealing"], json!(true));
            }
        }
        let g = crate::font::glyph('a').unwrap();
        assert_eq!(
            f.state(&iden())["glyph"]["rows"],
            json!(crate::font::to_lists(&g))
        );
    }
    #[test]
    fn tick_without_reveal_fails() {
        let mut f = Font::new();
        let out = send(&mut f, "font.tick", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("nothing revealing"));
    }
    #[test]
    fn beat_only_while_revealing() {
        let mut f = Font::new();
        assert!(f.beat().is_none());
        f.act(&iden(), &Call::new("font.scramble", json!({})).at(1));
        assert_eq!(
            f.beat().unwrap().to_json(),
            json!({ "verb": "font.tick", "args": {} })
        );
    }
    #[test]
    fn reveal_clears_on_navigation() {
        let mut f = Font::new();
        f.act(&iden(), &Call::new("font.scramble", json!({})).at(1));
        assert_eq!(f.state(&iden())["revealing"], json!(true));
        send(&mut f, "font.page", json!({ "dir": "next" }));
        assert_eq!(f.state(&iden())["revealing"], json!(false));
        f.act(&iden(), &Call::new("font.scramble", json!({})).at(1));
        send(&mut f, "font.page", json!({ "dir": "prev" }));
        assert_eq!(f.state(&iden())["revealing"], json!(false));
        f.act(&iden(), &Call::new("font.scramble", json!({})).at(1));
        send(&mut f, "font.pick", json!({ "char": "b" }));
        assert_eq!(f.state(&iden())["revealing"], json!(false));
    }
    #[test]
    fn reveal_survives_unrelated_failures() {
        let mut f = Font::new();
        f.act(&iden(), &Call::new("font.scramble", json!({})).at(1));
        send(&mut f, "font.pick", json!({ "char": "zz" }));
        assert_eq!(f.state(&iden())["revealing"], json!(true));
    }
    #[test]
    fn save_load_roundtrips() {
        let mut a = Font::new();
        send(&mut a, "font.pick", json!({ "char": "z" }));
        let mut b = Font::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden())["char"], json!("z"));
    }
    #[test]
    fn load_survives_garbage() {
        let mut f = Font::new();
        f.load(&json!({ "char": "€" }));
        assert_eq!(f.state(&iden())["index"], json!(0));
        f.load(&json!({ "char": 5 }));
        assert_eq!(f.state(&iden())["index"], json!(0));
        f.load(&json!({}));
        assert_eq!(f.state(&iden())["index"], json!(0));
    }
    #[test]
    fn load_clears_reveal() {
        let mut f = Font::new();
        f.act(&iden(), &Call::new("font.scramble", json!({})).at(1));
        f.load(&json!({ "char": "a" }));
        assert_eq!(f.state(&iden())["revealing"], json!(false));
    }
    #[test]
    fn state_carries_name_and_total() {
        let f = Font::new();
        let state = f.state(&iden());
        assert_eq!(state["total"], json!(108));
        assert_eq!(state["name"], json!(crate::font::name_of(f.order[0])));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let f = Font::new();
        let names: Vec<String> = f.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "font.page",
                "font.pick",
                "font.scramble",
                "font.tick",
                "font.export",
                "font.keep",
                "font.drop"
            ]
        );
    }
    #[test]
    fn library_seeds_are_supported() {
        let f = Font::new();
        let library = f.state(&iden())["library"].clone();
        assert_eq!(library, json!(["A", "a", "0", "#"]));
        for value in library.as_array().unwrap() {
            let c = value.as_str().unwrap().chars().next().unwrap();
            assert!(crate::font::glyph(c).is_some());
        }
    }
    #[test]
    fn keep_defaults_to_current_char() {
        let mut f = Font::new();
        send(&mut f, "font.pick", json!({ "char": "Z" }));
        assert!(send(&mut f, "font.keep", json!({})).ok);
        assert!(f.state(&iden())["library"]
            .as_array()
            .unwrap()
            .contains(&json!("Z")));
    }
    #[test]
    fn keep_and_drop_round_trip() {
        let mut f = Font::new();
        assert!(send(&mut f, "font.keep", json!({ "char": "Q" })).ok);
        assert!(f.state(&iden())["library"]
            .as_array()
            .unwrap()
            .contains(&json!("Q")));
        let dup = send(&mut f, "font.keep", json!({ "char": "Q" }));
        assert!(!dup.ok);
        assert_eq!(dup.note.as_deref(), Some("already kept"));
        let unknown = send(&mut f, "font.keep", json!({ "char": "€" }));
        assert_eq!(unknown.note.as_deref(), Some("unknown char"));
        assert!(send(&mut f, "font.drop", json!({ "char": "Q" })).ok);
        assert!(!f.state(&iden())["library"]
            .as_array()
            .unwrap()
            .contains(&json!("Q")));
        let gone = send(&mut f, "font.drop", json!({ "char": "Q" }));
        assert_eq!(gone.note.as_deref(), Some("not in the library"));
    }
    #[test]
    fn load_sanitizes_the_library() {
        let mut f = Font::new();
        f.load(&json!({ "library": "garbage" }));
        assert_eq!(f.state(&iden())["library"], json!(["A", "a", "0", "#"]));
        f.load(&json!({ "library": ["A", "€", "A", "0"] }));
        assert_eq!(f.state(&iden())["library"], json!(["A", "0"]));
    }
    #[test]
    fn export_emits_the_requested_asset() {
        let mut f = Font::new();
        let out = send(&mut f, "font.export", json!({ "format": "woff2" }));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        let effect = &out.effects[0];
        assert_eq!(effect.kind, "file");
        assert_eq!(effect.data["name"], json!("MrlyFont.woff2"));
        assert_eq!(effect.data["mime"], json!("font/woff2"));
        assert!(!effect.data["data"].as_str().unwrap().is_empty());
        assert!(!send(&mut f, "font.export", json!({ "format": "otf" })).ok);
        assert!(!send(&mut f, "font.export", json!({})).ok);
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Font::new(), "font.fly", json!({})).ok);
    }
}
