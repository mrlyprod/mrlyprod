use crate::core::colors::ROLLABLE;
use crate::core::rng::Rng;
use crate::core::tensor::Tensor;
use crate::math::pick;
use crate::math::two::Cell2d;
use crate::music::cue;
use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{sprite_fact, Frame, Layer, TileSet};
use serde_json::{json, Value as Json};

const SURFACES: [&str; 2] = ["grid", "canvas"];
const SKINS: [&str; 2] = ["tiles", "digits"];

struct Set {
    options: i64,
    size: i64,
    length: i64,
    surface: String,
    skin: String,
    reward_right: f64,
    reward_wrong: f64,
}

impl Set {
    fn new() -> Set {
        Set {
            options: 3,
            size: 8,
            length: 10,
            surface: "grid".to_string(),
            skin: "tiles".to_string(),
            reward_right: 1.0,
            reward_wrong: 0.0,
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "options" | "size" | "length" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "options" => (2, 8),
                    "size" => (2, 16),
                    _ => (2, 32),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "options" => self.options = n,
                    "size" => self.size = n,
                    _ => self.length = n,
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
            "options": self.options,
            "size": self.size,
            "length": self.length,
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

pub struct Quiz {
    set: Set,
    rng: Rng,
    seed: u64,
    score: u64,
    steps: u64,
    over: bool,
    won: bool,
    target: usize,
    options: Vec<usize>,
    correct: usize,
    color: [u8; 4],
    dark: bool,
}

impl Default for Quiz {
    fn default() -> Quiz {
        Quiz::new()
    }
}

impl Quiz {
    pub fn new() -> Quiz {
        let mut quiz = Quiz {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            score: 0,
            steps: 0,
            over: false,
            won: false,
            target: 0,
            options: Vec::new(),
            correct: 0,
            color: [255, 255, 255, 255],
            dark: false,
        };
        quiz.reset(0);
        quiz
    }
    fn options_len(&self) -> usize {
        (self.set.options as usize).max(2)
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn roll(&mut self) {
        let vocab = pick::vocab(usize::MAX);
        let n = self.options_len().min(vocab.len());
        let mut bag = vocab.clone();
        for i in (1..bag.len()).rev() {
            let j = self.rng.below(i + 1);
            bag.swap(i, j);
        }
        let mut chosen: Vec<usize> = bag.into_iter().take(n).collect();
        self.target = chosen[self.rng.below(n)];
        for i in (1..chosen.len()).rev() {
            let j = self.rng.below(i + 1);
            chosen.swap(i, j);
        }
        self.correct = chosen.iter().position(|&c| c == self.target).unwrap();
        self.options = chosen;
        self.color = self.palette();
    }
    fn option_names(&self) -> Vec<String> {
        self.options.iter().map(|&c| pick::name(c).into()).collect()
    }
    fn face(&self) -> Cell2d {
        let k = self.set.size as usize;
        let fg = if self.set.skin == "digits" {
            crate::ui::frame::ink(self.dark)
        } else {
            self.color
        };
        pick::tile(self.target, k, fg, [0, 0, 0, 0])
    }
    fn tileset(&self) -> TileSet {
        TileSet::new(self.set.size as usize, vec![self.face()])
    }
    fn position(&self) -> u64 {
        if self.over {
            self.steps
        } else {
            self.steps + 1
        }
    }
    fn render(&self) -> Frame {
        let k = self.set.size as usize;
        let mut frame = Frame::new(k, k, crate::ui::frame::board(self.dark));
        frame.push(Layer::Tiles {
            ids: Tensor::new(vec![1, 1]),
            set: self.tileset(),
        });
        frame.say("", self.option_names());
        frame
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.score = 0;
        self.steps = 0;
        self.over = false;
        self.won = false;
        self.roll();
    }
}

impl App for Quiz {
    fn route(&self) -> &str {
        "quiz"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("quiz").emoji("❓").category("puzzles")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "won": self.won,
            "seed": self.seed,
            "position": self.position(),
            "total": self.set.length,
            "settings": self.set.to_json(),
            "options": self.option_names(),
            "sprite": sprite_fact(&self.face()),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new("quiz.answer", json!({ "text": "string" })));
        }
        out.push(Verb::new("quiz.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "quiz.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "quiz.answer" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let Some(text) = call.arg("text").as_str() else {
                    return Outcome::fail("text must be a string");
                };
                let names = self.option_names();
                let Some(idx) = names.iter().position(|n| n == text) else {
                    return Outcome::fail("no such option");
                };
                self.steps += 1;
                if idx != self.correct {
                    self.over = true;
                    return Outcome::ok(json!({ "correct": false }))
                        .emit(Effect::new("sound", cue::payload("bad")));
                }
                self.score += 1;
                if self.steps >= self.set.length as u64 {
                    self.over = true;
                    self.won = true;
                    return Outcome::ok(json!({ "correct": true, "won": true }))
                        .emit(Effect::new("sound", cue::payload("win")));
                }
                self.roll();
                Outcome::ok(json!({ "correct": true }))
                    .emit(Effect::new("sound", cue::payload("good")))
            }
            "quiz.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "quiz.set" => {
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
            "won": self.won,
            "target": self.target,
            "options": self.options,
            "correct": self.correct,
            "color": self.color,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let options: Option<Vec<usize>> = state["options"].as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as usize))
                .collect()
        });
        if let Some(options) = options {
            if !options.is_empty() {
                self.options = options;
                if let Some(target) = state["target"].as_u64() {
                    self.target = target as usize;
                }
                if let Some(correct) = state["correct"].as_u64() {
                    self.correct = (correct as usize).min(self.options.len() - 1);
                }
                if let Some(color) = state["color"].as_array() {
                    let bytes: Option<Vec<u8>> =
                        color.iter().map(|v| v.as_u64().map(|n| n as u8)).collect();
                    if let Some(bytes) = bytes {
                        if bytes.len() == 4 {
                            self.color = [bytes[0], bytes[1], bytes[2], bytes[3]];
                        }
                    }
                }
                self.score = state["score"].as_u64().unwrap_or(0);
                self.steps = state["steps"].as_u64().unwrap_or(0);
                self.over = state["over"].as_bool().unwrap_or(false);
                self.won = state["won"].as_bool().unwrap_or(false);
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
    use crate::os::kernel::testkit::{iden, seeded, send};

    fn quiz(seed: u64) -> Quiz {
        seeded(Quiz::new(), "quiz.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = quiz(7);
        let mut b = quiz(7);
        for q in [&mut a, &mut b] {
            let text = q.option_names()[q.correct].clone();
            send(q, "quiz.answer", json!({ "text": text }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn correct_answer_scores_and_rerolls() {
        let mut q = quiz(7);
        let text = q.option_names()[q.correct].clone();
        let out = send(&mut q, "quiz.answer", json!({ "text": text }));
        assert!(out.ok);
        assert_eq!(out.data["correct"], json!(true));
        assert_eq!(out.effects[0].data, cue::payload("good"));
        assert_eq!(q.score, 1);
        assert!(!q.over);
    }
    #[test]
    fn wrong_answer_ends_the_round_honestly() {
        let mut q = quiz(7);
        let names = q.option_names();
        let wrong = names[(q.correct + 1) % names.len()].clone();
        let out = send(&mut q, "quiz.answer", json!({ "text": wrong }));
        assert!(out.ok);
        assert_eq!(out.data["correct"], json!(false));
        assert_eq!(out.effects[0].data, cue::payload("bad"));
        assert!(q.over);
        assert!(!q.won);
        assert!(!send(&mut q, "quiz.answer", json!({ "text": wrong })).ok);
    }
    #[test]
    fn full_run_ends_in_a_win() {
        let mut q = quiz(7);
        send(&mut q, "quiz.set", json!({ "key": "length", "value": 2 }));
        let text = q.option_names()[q.correct].clone();
        let out = send(&mut q, "quiz.answer", json!({ "text": text }));
        assert!(out.ok);
        assert!(!q.over);
        let text = q.option_names()[q.correct].clone();
        let out = send(&mut q, "quiz.answer", json!({ "text": text }));
        assert!(out.ok);
        assert_eq!(out.data["won"], json!(true));
        assert_eq!(out.effects[0].data, cue::payload("win"));
        assert!(q.over);
        assert!(q.won);
        assert_eq!(q.score, 2);
        assert!(!send(&mut q, "quiz.answer", json!({ "text": text })).ok);
    }
    #[test]
    fn position_counts_against_the_total() {
        let mut q = quiz(7);
        let state = q.state(&iden());
        assert_eq!(state["position"], json!(1));
        assert_eq!(state["total"], json!(10));
        let text = q.option_names()[q.correct].clone();
        send(&mut q, "quiz.answer", json!({ "text": text }));
        assert_eq!(q.state(&iden())["position"], json!(2));
        let names = q.option_names();
        let wrong = names[(q.correct + 1) % names.len()].clone();
        send(&mut q, "quiz.answer", json!({ "text": wrong }));
        assert_eq!(q.state(&iden())["position"], json!(2));
    }
    #[test]
    fn state_masks_the_correct_answer() {
        let q = quiz(9);
        let state = q.state(&iden());
        assert!(state.get("target").is_none());
        assert!(state.get("correct").is_none());
        assert_eq!(state["options"].as_array().unwrap().len(), q.options.len());
        let keys: Vec<&str> = state["sprite"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, vec!["width", "height", "rows", "palette"]);
        let saved = q.save();
        assert!(saved.get("target").is_some());
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut q = quiz(1);
        assert!(
            !send(
                &mut q,
                "quiz.answer",
                json!({ "text": "not-a-real-option" })
            )
            .ok
        );
        assert!(!send(&mut q, "quiz.answer", json!({})).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut q = Quiz::new();
        let out = q.act(&iden(), &Call::new("quiz.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(q.state(&iden())["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut q = quiz(4);
        let text = q.option_names()[q.correct].clone();
        send(&mut q, "quiz.answer", json!({ "text": text }));
        let out = send(&mut q, "quiz.set", json!({ "key": "options", "value": 5 }));
        assert!(out.ok);
        let state = q.state(&iden());
        assert_eq!(state["settings"]["options"], json!(5));
        assert_eq!(state["steps"], json!(0));
        assert!(
            !send(
                &mut q,
                "quiz.set",
                json!({ "key": "options", "value": 999 })
            )
            .ok
        );
        assert!(!send(&mut q, "quiz.set", json!({ "key": "length", "value": 1 })).ok);
        assert!(
            !send(
                &mut q,
                "quiz.set",
                json!({ "key": "reward_right", "value": "lots" })
            )
            .ok
        );
        assert!(!send(&mut q, "quiz.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn look_keys_validate_without_resetting() {
        let mut q = quiz(4);
        let text = q.option_names()[q.correct].clone();
        send(&mut q, "quiz.answer", json!({ "text": text }));
        assert!(
            send(
                &mut q,
                "quiz.set",
                json!({ "key": "surface", "value": "canvas" })
            )
            .ok
        );
        assert!(
            send(
                &mut q,
                "quiz.set",
                json!({ "key": "skin", "value": "digits" })
            )
            .ok
        );
        let state = q.state(&iden());
        assert_eq!(state["settings"]["surface"], json!("canvas"));
        assert_eq!(state["settings"]["skin"], json!("digits"));
        assert_eq!(state["steps"], json!(1));
        assert!(
            !send(
                &mut q,
                "quiz.set",
                json!({ "key": "skin", "value": "emojis" })
            )
            .ok
        );
        assert!(
            !send(
                &mut q,
                "quiz.set",
                json!({ "key": "surface", "value": "paper" })
            )
            .ok
        );
    }
    #[test]
    fn old_saves_default_to_the_legacy_look() {
        let mut q = Quiz::new();
        q.load(&json!({ "seed": 3, "settings": { "options": 4 } }));
        let settings = q.state(&iden())["settings"].clone();
        assert_eq!(settings["options"], json!(4));
        assert_eq!(settings["surface"], json!("grid"));
        assert_eq!(settings["skin"], json!("tiles"));
        assert_eq!(settings["length"], json!(10));
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = quiz(11);
        let text = a.option_names()[a.correct].clone();
        send(&mut a, "quiz.answer", json!({ "text": text }));
        let mut b = Quiz::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for q in [&mut a, &mut b] {
            let text = q.option_names()[q.correct].clone();
            send(q, "quiz.answer", json!({ "text": text }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut q = Quiz::new();
        q.load(&json!({ "seed": "soup", "options": "nope", "settings": 7 }));
        assert_eq!(q.state(&iden())["steps"], json!(0));
        assert_eq!(q.state(&iden())["seed"], json!(0));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let q = quiz(3);
        let names: Vec<String> = q.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["quiz.answer", "quiz.reset", "quiz.set"]);
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let q = quiz(5);
        let state = q.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
    }
}
