use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlymath::pick;
use mrlymath::two::Cell2d;
use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{sprite_fact, Frame, Layer, TileSet};
use mrlymusic::cue;
use serde_json::{json, Value as Json};

const SURFACES: [&str; 2] = ["grid", "canvas"];
const SKINS: [&str; 2] = ["tiles", "digits"];

struct Set {
    cols: i64,
    rows: i64,
    size: i64,
    surface: String,
    skin: String,
    reward_right: f64,
    reward_wrong: f64,
}

impl Set {
    fn new() -> Set {
        Set {
            cols: 3,
            rows: 3,
            size: 8,
            surface: "grid".to_string(),
            skin: "tiles".to_string(),
            reward_right: 1.0,
            reward_wrong: 0.0,
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "cols" | "rows" | "size" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "cols" => (2, 5),
                    "rows" => (2, 5),
                    _ => (2, 16),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "cols" => self.cols = n,
                    "rows" => self.rows = n,
                    _ => self.size = n,
                }
                Ok(json!(n))
            }
            "surface" | "skin" => {
                let s = value.as_str().ok_or("value must be a string")?;
                let legal: &[&str] = if key == "surface" { &SURFACES } else { &SKINS };
                if !legal.contains(&s) {
                    return Err("no such option");
                }
                match key {
                    "surface" => self.surface = s.to_string(),
                    _ => self.skin = s.to_string(),
                }
                Ok(json!(s))
            }
            "reward_right" | "reward_wrong" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
                    "reward_right" => (0.0, 10.0),
                    _ => (-10.0, 0.0),
                };
                if n < min || n > max {
                    return Err("out of range");
                }
                match key {
                    "reward_right" => self.reward_right = n,
                    _ => self.reward_wrong = n,
                }
                Ok(json!(n))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "cols": self.cols,
            "rows": self.rows,
            "size": self.size,
            "surface": self.surface,
            "skin": self.skin,
            "reward_right": self.reward_right,
            "reward_wrong": self.reward_wrong,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let _ = set.apply(key, val);
            }
        }
        set
    }
}

pub struct Captcha {
    set: Set,
    rng: Rng,
    seed: u64,
    score: u64,
    steps: u64,
    over: bool,
    codes: Vec<usize>,
    colors: Vec<[u8; 4]>,
    target: usize,
    dark: bool,
}

impl Default for Captcha {
    fn default() -> Captcha {
        Captcha::new()
    }
}

impl Captcha {
    pub fn new() -> Captcha {
        let mut captcha = Captcha {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            score: 0,
            steps: 0,
            over: false,
            codes: Vec::new(),
            colors: Vec::new(),
            target: 0,
            dark: false,
        };
        captcha.reset(0);
        captcha
    }
    fn cols(&self) -> usize {
        self.set.cols as usize
    }
    fn rows(&self) -> usize {
        self.set.rows as usize
    }
    fn cells(&self) -> usize {
        self.cols() * self.rows()
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn roll(&mut self) {
        let n = self.cells();
        let mut bag = pick::vocab(usize::MAX);
        for i in (1..bag.len()).rev() {
            let j = self.rng.below(i + 1);
            bag.swap(i, j);
        }
        let hero = bag[0];
        let rest = &bag[1..];
        self.codes = (0..n).map(|_| rest[self.rng.below(rest.len())]).collect();
        self.target = self.rng.below(n);
        self.codes[self.target] = hero;
        self.colors = (0..n).map(|_| self.palette()).collect();
    }
    fn prompt(&self) -> String {
        pick::name(self.codes[self.target]).to_string()
    }
    fn face(&self, i: usize) -> Cell2d {
        let k = self.set.size as usize;
        let fg = if self.set.skin == "digits" {
            mrlyui::frame::ink(self.dark)
        } else {
            self.colors[i]
        };
        pick::tile(self.codes[i], k, fg, [0, 0, 0, 0])
    }
    fn sprites(&self) -> Vec<Json> {
        (0..self.cells())
            .map(|i| sprite_fact(&self.face(i)))
            .collect()
    }
    fn tileset(&self) -> TileSet {
        let k = self.set.size as usize;
        TileSet::new(k, (0..self.cells()).map(|i| self.face(i)).collect())
    }
    fn ids(&self) -> Tensor {
        let (cols, rows) = (self.cols(), self.rows());
        let mut grid = Tensor::new(vec![rows, cols]);
        for i in 0..self.cells() {
            grid.set(&[i / cols, i % cols], i as u8);
        }
        grid
    }
    fn render(&self) -> Frame {
        let k = self.set.size as usize;
        let mut frame = Frame::new(
            self.cols() * k,
            self.rows() * k,
            mrlyui::frame::board(self.dark),
        );
        frame.push(Layer::Tiles {
            ids: self.ids(),
            set: self.tileset(),
        });
        frame.say(self.prompt(), Vec::new());
        frame
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.score = 0;
        self.steps = 0;
        self.over = false;
        self.roll();
    }
    fn grade(&mut self, idx: usize) -> Outcome {
        self.steps += 1;
        if idx == self.target {
            self.score += 1;
            self.roll();
            Outcome::ok(json!({ "correct": true })).emit(Effect::new("sound", cue::payload("good")))
        } else {
            self.over = true;
            Outcome::ok(json!({ "correct": false })).emit(Effect::new("sound", cue::payload("bad")))
        }
    }
}

impl App for Captcha {
    fn route(&self) -> &str {
        "captcha"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("captcha").emoji("🧩").category("puzzles")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "prompt": self.prompt(),
            "sprites": self.sprites(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new("captcha.pick", json!({ "cell": "int" })));
            out.push(Verb::new("captcha.answer", json!({ "text": "string" })));
        }
        out.push(Verb::new("captcha.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "captcha.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "captcha.pick" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let (cols, rows) = (self.set.cols, self.set.rows);
                let cell = match call.arg("cell").as_i64() {
                    Some(c) => c,
                    None => match (call.arg("x").as_i64(), call.arg("y").as_i64()) {
                        (Some(x), Some(y)) if (0..cols).contains(&x) && (0..rows).contains(&y) => {
                            y * cols + x
                        }
                        _ => return Outcome::fail("cell must be an integer"),
                    },
                };
                if !(0..self.cells() as i64).contains(&cell) {
                    return Outcome::fail("cell out of range");
                }
                self.grade(cell as usize)
            }
            "captcha.answer" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let Some(text) = call.arg("text").as_str() else {
                    return Outcome::fail("text must be a string");
                };
                let Some(idx) = self.codes.iter().position(|&code| pick::name(code) == text) else {
                    return Outcome::fail("no such option");
                };
                self.grade(idx)
            }
            "captcha.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "captcha.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        if !matches!(key.as_str(), "surface" | "skin") {
                            let seed = self.seed;
                            self.reset(seed);
                        }
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "codes": self.codes,
            "target": self.target,
            "colors": self.colors.iter().map(|c| json!(c)).collect::<Vec<_>>(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let codes: Option<Vec<usize>> = state["codes"].as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as usize))
                .collect()
        });
        if let Some(codes) = codes {
            if codes.len() == self.cells() {
                self.codes = codes;
                if let Some(target) = state["target"].as_u64() {
                    self.target = (target as usize).min(self.codes.len() - 1);
                }
                let colors: Option<Vec<[u8; 4]>> = state["colors"].as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|c| {
                            let bytes: Option<Vec<u8>> = c
                                .as_array()?
                                .iter()
                                .map(|v| v.as_u64().map(|n| n as u8))
                                .collect();
                            let bytes = bytes?;
                            if bytes.len() == 4 {
                                Some([bytes[0], bytes[1], bytes[2], bytes[3]])
                            } else {
                                None
                            }
                        })
                        .collect()
                });
                if let Some(colors) = colors {
                    if colors.len() == self.codes.len() {
                        self.colors = colors;
                    }
                }
                self.score = state["score"].as_u64().unwrap_or(0);
                self.steps = state["steps"].as_u64().unwrap_or(0);
                self.over = state["over"].as_bool().unwrap_or(false);
            }
        }
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn captcha(seed: u64) -> Captcha {
        seeded(Captcha::new(), "captcha.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = captcha(7);
        let mut b = captcha(7);
        for c in [&mut a, &mut b] {
            let text = c.prompt();
            send(c, "captcha.answer", json!({ "text": text }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn correct_answer_scores_and_rerolls() {
        let mut c = captcha(7);
        let text = c.prompt();
        let out = send(&mut c, "captcha.answer", json!({ "text": text }));
        assert!(out.ok);
        assert_eq!(out.data["correct"], json!(true));
        assert_eq!(c.score, 1);
        assert!(!c.over);
    }
    #[test]
    fn wrong_answer_ends_the_round_honestly() {
        let mut c = captcha(7);
        let wrong_idx = (c.target + 1) % c.cells();
        let wrong = pick::name(c.codes[wrong_idx]).to_string();
        let out = send(&mut c, "captcha.answer", json!({ "text": wrong.clone() }));
        assert!(out.ok);
        assert_eq!(out.data["correct"], json!(false));
        assert!(c.over);
        assert!(!send(&mut c, "captcha.answer", json!({ "text": wrong })).ok);
    }
    #[test]
    fn pick_scores_and_misses_with_sounds() {
        let mut c = captcha(7);
        let target = c.target;
        let out = send(&mut c, "captcha.pick", json!({ "cell": target }));
        assert!(out.ok);
        assert_eq!(out.data["correct"], json!(true));
        assert_eq!(c.score, 1);
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.effects[0].kind, "sound");
        assert_eq!(out.effects[0].data, cue::payload("good"));
        let wrong = (c.target + 1) % c.cells();
        let out = send(&mut c, "captcha.pick", json!({ "cell": wrong }));
        assert!(out.ok);
        assert_eq!(out.data["correct"], json!(false));
        assert!(c.over);
        assert_eq!(out.effects[0].data, cue::payload("bad"));
        assert!(!send(&mut c, "captcha.pick", json!({ "cell": 0 })).ok);
    }
    #[test]
    fn pick_accepts_canvas_coordinates() {
        let mut c = captcha(7);
        let (x, y) = (c.target % c.cols(), c.target / c.cols());
        let out = send(&mut c, "captcha.pick", json!({ "x": x, "y": y }));
        assert!(out.ok);
        assert_eq!(out.data["correct"], json!(true));
        assert!(!send(&mut c, "captcha.pick", json!({ "x": 9, "y": 0 })).ok);
        assert!(!send(&mut c, "captcha.pick", json!({ "cell": 99 })).ok);
        assert!(!send(&mut c, "captcha.pick", json!({})).ok);
    }
    #[test]
    fn state_does_not_leak_the_target_cell() {
        let c = captcha(9);
        let state = c.state(&iden());
        assert!(state.get("codes").is_none());
        assert!(state.get("target").is_none());
        assert_eq!(state["prompt"], json!(pick::name(c.codes[c.target])));
        let sprites = state["sprites"].as_array().unwrap();
        assert_eq!(sprites.len(), c.cells());
        for sprite in sprites {
            let keys: Vec<&str> = sprite
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(keys, vec!["width", "height", "rows", "palette"]);
        }
        let saved = c.save();
        assert!(saved.get("codes").is_some());
        assert!(saved.get("target").is_some());
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut c = captcha(1);
        assert!(
            !send(
                &mut c,
                "captcha.answer",
                json!({ "text": "not-a-real-option" })
            )
            .ok
        );
        assert!(!send(&mut c, "captcha.answer", json!({})).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut c = Captcha::new();
        let out = c.act(&iden(), &Call::new("captcha.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(c.state(&iden())["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut c = captcha(4);
        let text = c.prompt();
        send(&mut c, "captcha.answer", json!({ "text": text }));
        let out = send(&mut c, "captcha.set", json!({ "key": "cols", "value": 4 }));
        assert!(out.ok);
        let state = c.state(&iden());
        assert_eq!(state["settings"]["cols"], json!(4));
        assert_eq!(state["steps"], json!(0));
        assert!(
            !send(
                &mut c,
                "captcha.set",
                json!({ "key": "cols", "value": 999 })
            )
            .ok
        );
        assert!(
            !send(
                &mut c,
                "captcha.set",
                json!({ "key": "reward_right", "value": "lots" })
            )
            .ok
        );
        assert!(
            !send(
                &mut c,
                "captcha.set",
                json!({ "key": "volume", "value": 1 })
            )
            .ok
        );
    }
    #[test]
    fn look_keys_validate_without_resetting() {
        let mut c = captcha(4);
        let text = c.prompt();
        send(&mut c, "captcha.answer", json!({ "text": text }));
        let out = send(
            &mut c,
            "captcha.set",
            json!({ "key": "surface", "value": "canvas" }),
        );
        assert!(out.ok);
        let out = send(
            &mut c,
            "captcha.set",
            json!({ "key": "skin", "value": "digits" }),
        );
        assert!(out.ok);
        let state = c.state(&iden());
        assert_eq!(state["settings"]["surface"], json!("canvas"));
        assert_eq!(state["settings"]["skin"], json!("digits"));
        assert_eq!(state["steps"], json!(1));
        assert_eq!(state["score"], json!(1));
        assert!(
            !send(
                &mut c,
                "captcha.set",
                json!({ "key": "skin", "value": "emojis" })
            )
            .ok
        );
        assert!(
            !send(
                &mut c,
                "captcha.set",
                json!({ "key": "surface", "value": "paper" })
            )
            .ok
        );
        assert!(!send(&mut c, "captcha.set", json!({ "key": "skin", "value": 3 })).ok);
    }
    #[test]
    fn digits_skin_bleaches_the_sprites() {
        let mut c = captcha(4);
        send(
            &mut c,
            "captcha.set",
            json!({ "key": "skin", "value": "digits" }),
        );
        let state = c.state(&iden());
        for sprite in state["sprites"].as_array().unwrap() {
            for color in sprite["palette"].as_array().unwrap() {
                let hex = color.as_str().unwrap();
                assert!(hex == "#000000" || hex == "#00000000");
            }
        }
    }
    #[test]
    fn old_saves_default_to_the_legacy_look() {
        let mut c = Captcha::new();
        c.load(&json!({ "seed": 3, "settings": { "cols": 4, "rows": 4 } }));
        let settings = c.state(&iden())["settings"].clone();
        assert_eq!(settings["cols"], json!(4));
        assert_eq!(settings["surface"], json!("grid"));
        assert_eq!(settings["skin"], json!("tiles"));
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = captcha(11);
        let text = a.prompt();
        send(&mut a, "captcha.answer", json!({ "text": text }));
        let mut b = Captcha::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for c in [&mut a, &mut b] {
            let text = c.prompt();
            send(c, "captcha.answer", json!({ "text": text }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut c = Captcha::new();
        c.load(&json!({ "seed": "soup", "codes": "nope", "settings": 7 }));
        assert_eq!(c.state(&iden())["steps"], json!(0));
        assert_eq!(c.state(&iden())["seed"], json!(0));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let c = captcha(3);
        let names: Vec<String> = c.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "captcha.pick",
                "captcha.answer",
                "captcha.reset",
                "captcha.set"
            ]
        );
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let c = captcha(5);
        let state = c.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
    }
}
