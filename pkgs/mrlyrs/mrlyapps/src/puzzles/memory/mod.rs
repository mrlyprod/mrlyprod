use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{bake, hex, motif_tile, solid_tile, Frame, Layer, TileSet};
use mrlyui::music::cue;
use serde_json::{json, Value as Json};

const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];
const SURFACES: [&str; 2] = ["grid", "canvas"];
const SKINS: [&str; 3] = ["tiles", "emojis", "digits"];
const VOID: u8 = 0;
const BACK: u8 = 1;
const LOOK: u64 = 16;

struct Set {
    pairs: i64,
    cols: i64,
    sudden: bool,
    reward_match: f64,
    reward_miss: f64,
    tile: i64,
    design: String,
    surface: String,
    skin: String,
}

impl Set {
    fn new() -> Set {
        Set {
            pairs: 4,
            cols: 3,
            sudden: false,
            reward_match: 1.0,
            reward_miss: 0.0,
            tile: 4,
            design: "carpet".to_string(),
            surface: "grid".to_string(),
            skin: "tiles".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "pairs" | "cols" | "tile" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "pairs" => (2, 8),
                    "cols" => (2, 8),
                    _ => (1, 8),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "pairs" => self.pairs = n,
                    "cols" => self.cols = n,
                    _ => self.tile = n,
                }
                Ok(json!(n))
            }
            "reward_match" | "reward_miss" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
                    "reward_match" => (0.0, 10.0),
                    _ => (-1.0, 0.0),
                };
                if n < min || n > max {
                    return Err("out of range");
                }
                match key {
                    "reward_match" => self.reward_match = n,
                    _ => self.reward_miss = n,
                }
                Ok(json!(n))
            }
            "sudden" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                self.sudden = on;
                Ok(json!(on))
            }
            "design" => {
                let d = value.as_str().ok_or("value must be a string")?;
                if !DESIGNS.contains(&d) {
                    return Err("no such option");
                }
                self.design = d.to_string();
                Ok(json!(d))
            }
            "surface" => {
                let v = value.as_str().ok_or("value must be a string")?;
                if !SURFACES.contains(&v) {
                    return Err("no such option");
                }
                if v == "canvas" && self.skin == "emojis" {
                    return Err("emojis need the grid");
                }
                self.surface = v.to_string();
                Ok(json!(v))
            }
            "skin" => {
                let v = value.as_str().ok_or("value must be a string")?;
                if !SKINS.contains(&v) {
                    return Err("no such option");
                }
                if v == "emojis" && self.surface == "canvas" {
                    return Err("emojis need the grid");
                }
                self.skin = v.to_string();
                Ok(json!(v))
            }
            _ => Err("no such key"),
        }
    }
    fn legal(&mut self) {
        if self.surface == "canvas" && self.skin == "emojis" {
            let fresh = Set::new();
            self.surface = fresh.surface;
            self.skin = fresh.skin;
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "pairs": self.pairs,
            "cols": self.cols,
            "sudden": self.sudden,
            "reward_match": self.reward_match,
            "reward_miss": self.reward_miss,
            "tile": self.tile,
            "design": self.design,
            "surface": self.surface,
            "skin": self.skin,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let _ = set.apply(key, val);
            }
        }
        set.legal();
        set
    }
}

pub struct Memory {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    over: bool,
    rounds: u64,
    score: u64,
    look: u64,
    faces: Vec<u8>,
    matched: Vec<bool>,
    first: Option<usize>,
    peek: Vec<usize>,
    colors: Vec<[u8; 4]>,
    back: [u8; 4],
    dark: bool,
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            over: false,
            rounds: 0,
            score: 0,
            look: 0,
            faces: Vec::new(),
            matched: Vec::new(),
            first: None,
            peek: Vec::new(),
            colors: Vec::new(),
            back: [70, 70, 78, 255],
            dark: false,
        };
        memory.reset(0);
        memory
    }
    fn pairs(&self) -> usize {
        self.set.pairs as usize
    }
    fn total(&self) -> usize {
        self.pairs() * 2 + 1
    }
    fn cols(&self) -> usize {
        self.set.cols as usize
    }
    fn rows(&self) -> usize {
        self.total().div_ceil(self.cols())
    }
    fn looking(&self) -> bool {
        self.look > 0
    }
    fn faceup(&self, i: usize) -> bool {
        self.looking() || self.matched[i] || self.first == Some(i) || self.peek.contains(&i)
    }
    fn won(&self) -> bool {
        self.matched.iter().filter(|&&m| m).count() == self.pairs() * 2
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn deal(&mut self) {
        let mut deck: Vec<u8> = Vec::with_capacity(self.total());
        for p in 0..self.pairs() {
            deck.push(p as u8);
            deck.push(p as u8);
        }
        deck.push(self.pairs() as u8);
        for i in (1..deck.len()).rev() {
            let j = self.rng.below(i + 1);
            deck.swap(i, j);
        }
        self.faces = deck;
        self.matched = vec![false; self.total()];
        self.first = None;
        self.peek = Vec::new();
        self.look = LOOK;
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.over = false;
        self.rounds = 0;
        self.score = 0;
        let faces = self.pairs() + 1;
        self.deal();
        self.colors = (0..faces).map(|_| self.palette()).collect();
        self.back = self.palette();
    }
    fn target(&self, call: &Call) -> Result<usize, &'static str> {
        let card = match call.arg("card").as_i64() {
            Some(card) => card,
            None => {
                let (Some(x), Some(y)) = (call.arg("x").as_i64(), call.arg("y").as_i64()) else {
                    return Err("card must be an integer");
                };
                if !(0..self.cols() as i64).contains(&x) || !(0..self.rows() as i64).contains(&y) {
                    return Err("card out of range");
                }
                y * self.cols() as i64 + x
            }
        };
        if !(0..self.total() as i64).contains(&card) {
            return Err("card out of range");
        }
        Ok(card as usize)
    }
    fn board_facts(&self) -> Vec<Vec<Json>> {
        let (cols, rows) = (self.cols(), self.rows());
        (0..rows)
            .map(|r| {
                (0..cols)
                    .map(|c| {
                        let p = r * cols + c;
                        if p >= self.total() {
                            json!("void")
                        } else if self.faceup(p) {
                            json!(self.faces[p])
                        } else {
                            Json::Null
                        }
                    })
                    .collect()
            })
            .collect()
    }
    fn matched_facts(&self) -> Vec<Vec<bool>> {
        let (cols, rows) = (self.cols(), self.rows());
        (0..rows)
            .map(|r| {
                (0..cols)
                    .map(|c| {
                        let p = r * cols + c;
                        p < self.total() && self.matched[p]
                    })
                    .collect()
            })
            .collect()
    }
    fn ids(&self) -> Tensor {
        let (cols, rows) = (self.cols(), self.rows());
        let mut grid = Tensor::new(vec![rows, cols]);
        for p in 0..self.total() {
            let id = if self.faceup(p) {
                2 + self.faces[p]
            } else {
                BACK
            };
            grid.set(&[p / cols, p % cols], id);
        }
        for p in self.total()..rows * cols {
            grid.set(&[p / cols, p % cols], VOID);
        }
        grid
    }
    fn tileset(&self) -> TileSet {
        let k = self.set.tile as usize;
        let clear = [0, 0, 0, 0];
        let d = self.set.design.as_str();
        let digits = self.set.skin == "digits";
        let mut tiles = vec![solid_tile(k, clear), solid_tile(k, self.back)];
        for (face, &color) in self.colors.iter().enumerate() {
            let mut tile = motif_tile(d, k, color, clear);
            if digits {
                bake(
                    &mut tile,
                    &(face + 1).to_string(),
                    k,
                    mrlyui::frame::board(self.dark),
                );
            }
            tiles.push(tile);
        }
        TileSet::new(k, tiles)
    }
    fn render(&self) -> Frame {
        let k = self.set.tile as usize;
        let mut frame = Frame::new(
            self.cols() * k,
            self.rows() * k,
            mrlyui::frame::board(self.dark),
        );
        frame.push(Layer::Tiles {
            ids: self.ids(),
            set: self.tileset(),
        });
        frame
    }
}

impl App for Memory {
    fn route(&self) -> &str {
        "memory"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("memory").emoji("🧠").category("puzzles")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "rounds": self.rounds,
            "look": self.look,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "board": self.board_facts(),
            "matched": self.matched_facts(),
            "colors": self.colors.iter().map(|&c| hex(c)).collect::<Vec<_>>(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over && !self.looking() {
            out.push(Verb::new("memory.flip", json!({ "card": "int" })));
        }
        out.push(Verb::new("memory.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "memory.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "memory.flip" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                if self.looking() {
                    return Outcome::fail("still looking");
                }
                let card = match self.target(call) {
                    Ok(card) => card,
                    Err(note) => return Outcome::fail(note),
                };
                self.peek.clear();
                if self.matched[card] || self.first == Some(card) {
                    return Outcome::fail("illegal move");
                }
                self.steps += 1;
                match self.first {
                    None => {
                        self.first = Some(card);
                        Outcome::ok(json!({ "card": card, "matched": false }))
                            .emit(Effect::new("sound", cue::payload("blip")))
                    }
                    Some(first) => {
                        self.first = None;
                        if self.faces[first] == self.faces[card] {
                            self.matched[first] = true;
                            self.matched[card] = true;
                            self.score += 1;
                            if self.won() {
                                self.rounds += 1;
                                self.deal();
                                return Outcome::ok(
                                    json!({ "card": card, "matched": true, "solved": true }),
                                )
                                .emit(Effect::new("sound", cue::payload("win")));
                            }
                            Outcome::ok(json!({ "card": card, "matched": true }))
                                .emit(Effect::new("sound", cue::payload("good")))
                        } else if self.set.sudden {
                            self.over = true;
                            self.peek = vec![first, card];
                            Outcome::ok(json!({ "card": card, "matched": false, "over": true }))
                                .emit(Effect::new("sound", cue::payload("lose")))
                        } else {
                            self.peek = vec![first, card];
                            Outcome::ok(json!({ "card": card, "matched": false }))
                                .emit(Effect::new("sound", cue::payload("bad")))
                        }
                    }
                }
            }
            "memory.tick" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                if self.look == 0 {
                    return Outcome::fail("nothing to tick");
                }
                self.look -= 1;
                Outcome::ok(json!({ "look": self.look }))
            }
            "memory.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "memory.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        let seed = self.seed;
                        self.reset(seed);
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        if !self.over && self.looking() {
            Some(Call::new("memory.tick", json!({})))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "over": self.over,
            "rounds": self.rounds,
            "score": self.score,
            "look": self.look,
            "faces": self.faces,
            "matched": self.matched,
            "first": self.first,
            "peek": self.peek,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let faces: Option<Vec<u8>> = state["faces"].as_array().and_then(|arr| {
            arr.iter()
                .map(|v| {
                    v.as_u64()
                        .filter(|&f| f <= self.pairs() as u64)
                        .map(|f| f as u8)
                })
                .collect()
        });
        if let (Some(faces), Some(arr)) = (faces, state["matched"].as_array()) {
            if faces.len() == self.total() && arr.len() == self.total() {
                if let Some(matched) = arr.iter().map(|v| v.as_bool()).collect::<Option<Vec<_>>>() {
                    self.faces = faces;
                    self.matched = matched;
                    self.first = state["first"].as_u64().and_then(|i| {
                        let i = i as usize;
                        (i < self.total()).then_some(i)
                    });
                    self.peek = state["peek"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_u64())
                                .map(|v| v as usize)
                                .collect()
                        })
                        .unwrap_or_default();
                    self.over = state["over"].as_bool().unwrap_or(false);
                    self.steps = state["steps"].as_u64().unwrap_or(0);
                    self.rounds = state["rounds"].as_u64().unwrap_or(0);
                    self.score = state["score"].as_u64().unwrap_or(0);
                    self.look = state["look"].as_u64().unwrap_or(0).min(LOOK);
                }
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
    use mrlyos::kernel::testkit::{iden, send};

    fn drain(m: &mut Memory) {
        while m.looking() {
            m.act(&iden(), &Call::new("memory.tick", json!({})));
        }
    }
    fn memory(seed: u64) -> Memory {
        let mut m = Memory::new();
        m.act(&iden(), &Call::new("memory.reset", json!({ "seed": seed })));
        drain(&mut m);
        m
    }

    #[test]
    fn seed_reproduces() {
        let mut a = memory(3);
        let mut b = memory(3);
        for m in [&mut a, &mut b] {
            send(m, "memory.flip", json!({ "card": 0 }));
            send(m, "memory.flip", json!({ "card": 1 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn successful_pair_flip_matches_and_scores() {
        let mut m = memory(3);
        let a = m.faces.iter().position(|&f| f == 0).unwrap();
        let b = m.faces.iter().rposition(|&f| f == 0).unwrap();
        send(&mut m, "memory.flip", json!({ "card": a }));
        let out = send(&mut m, "memory.flip", json!({ "card": b }));
        assert!(out.ok);
        assert_eq!(out.data["matched"], json!(true));
        assert_eq!(m.state(&iden())["score"], json!(1));
    }
    #[test]
    fn failed_pair_flip_peeks_then_hides() {
        let mut m = memory(3);
        let a = m.faces.iter().position(|&f| f == 0).unwrap();
        let c = m.faces.iter().position(|&f| f == 1).unwrap();
        send(&mut m, "memory.flip", json!({ "card": a }));
        let out = send(&mut m, "memory.flip", json!({ "card": c }));
        assert!(out.ok);
        assert_eq!(out.data["matched"], json!(false));
        assert!(m.faceup(a) && m.faceup(c));
        let other = (0..m.total()).find(|&i| i != a && i != c).unwrap();
        send(&mut m, "memory.flip", json!({ "card": other }));
        assert!(!m.faceup(a) && !m.faceup(c));
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut m = memory(3);
        send(&mut m, "memory.flip", json!({ "card": 0 }));
        let out = send(&mut m, "memory.flip", json!({ "card": 0 }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("illegal move"));
        assert!(!send(&mut m, "memory.flip", json!({ "card": 99 })).ok);
    }
    fn solve(m: &mut Memory) {
        for face in 0..m.pairs() as u8 {
            let a = m.faces.iter().position(|&f| f == face).unwrap();
            let b = m.faces.iter().rposition(|&f| f == face).unwrap();
            send(m, "memory.flip", json!({ "card": a }));
            send(m, "memory.flip", json!({ "card": b }));
        }
    }
    #[test]
    fn solving_a_round_deals_the_next_and_keeps_score() {
        let mut m = memory(3);
        solve(&mut m);
        let state = m.state(&iden());
        assert!(!m.over);
        assert_eq!(state["rounds"], json!(1));
        assert_eq!(state["score"], json!(4));
        assert!(m.looking());
        assert!(!send(&mut m, "memory.flip", json!({ "card": 0 })).ok);
        drain(&mut m);
        solve(&mut m);
        assert_eq!(m.state(&iden())["rounds"], json!(2));
        assert_eq!(m.state(&iden())["score"], json!(8));
    }
    #[test]
    fn sudden_death_ends_on_a_mismatch() {
        let mut m = memory(3);
        send(
            &mut m,
            "memory.set",
            json!({ "key": "sudden", "value": true }),
        );
        drain(&mut m);
        let a = m.faces.iter().position(|&f| f == 0).unwrap();
        let c = m.faces.iter().position(|&f| f == 1).unwrap();
        send(&mut m, "memory.flip", json!({ "card": a }));
        let out = send(&mut m, "memory.flip", json!({ "card": c }));
        assert!(out.ok);
        assert!(m.over);
        assert_eq!(out.effects[0].data, mrlyui::music::cue::payload("lose"));
        assert!(!send(&mut m, "memory.flip", json!({ "card": 0 })).ok);
    }
    #[test]
    fn look_phase_shows_then_hides_and_gates_flips() {
        let mut m = Memory::new();
        m.act(&iden(), &Call::new("memory.reset", json!({ "seed": 3 })));
        assert!(m.looking());
        assert_eq!(m.state(&iden())["look"], json!(LOOK));
        for row in m.state(&iden())["board"].as_array().unwrap() {
            for cell in row.as_array().unwrap() {
                assert!(!cell.is_null() || cell == &Json::Null);
            }
        }
        assert!(!send(&mut m, "memory.flip", json!({ "card": 0 })).ok);
        assert_eq!(m.beat(), Some(Call::new("memory.tick", json!({}))));
        drain(&mut m);
        assert!(!m.looking());
        assert_eq!(m.beat(), None);
        assert!(send(&mut m, "memory.flip", json!({ "card": 0 })).ok);
    }
    #[test]
    fn look_reveals_but_settled_hides() {
        let mut m = Memory::new();
        m.act(&iden(), &Call::new("memory.reset", json!({ "seed": 9 })));
        let shown = m.state(&iden())["board"].clone();
        assert!(shown
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|r| r.as_array().unwrap())
            .any(|c| c.is_number()));
        drain(&mut m);
        let board = m.state(&iden())["board"].clone();
        for row in board.as_array().unwrap() {
            for cell in row.as_array().unwrap() {
                assert!(cell.is_null() || cell == "void");
            }
        }
    }
    #[test]
    fn surface_and_skin_reject_emojis_off_the_grid() {
        let mut m = memory(4);
        assert!(
            send(
                &mut m,
                "memory.set",
                json!({ "key": "surface", "value": "canvas" })
            )
            .ok
        );
        let out = send(
            &mut m,
            "memory.set",
            json!({ "key": "skin", "value": "emojis" }),
        );
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("emojis need the grid"));
        assert!(
            send(
                &mut m,
                "memory.set",
                json!({ "key": "surface", "value": "grid" })
            )
            .ok
        );
        assert!(
            send(
                &mut m,
                "memory.set",
                json!({ "key": "skin", "value": "emojis" })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "memory.set",
                json!({ "key": "surface", "value": "canvas" })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "memory.set",
                json!({ "key": "skin", "value": "velvet" })
            )
            .ok
        );
    }
    #[test]
    fn from_json_resets_illegal_combos() {
        let mut m = Memory::new();
        m.load(&json!({ "seed": 3, "settings": { "skin": "emojis", "surface": "canvas" } }));
        let settings = m.state(&iden())["settings"].clone();
        assert!(!(settings["surface"] == json!("canvas") && settings["skin"] == json!("emojis")));
        let mut m = Memory::new();
        m.load(&json!({ "seed": 3, "settings": { "pairs": 6 } }));
        let settings = m.state(&iden())["settings"].clone();
        assert_eq!(settings["surface"], json!("grid"));
        assert_eq!(settings["skin"], json!("tiles"));
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut m = Memory::new();
        let out = m.act(&iden(), &Call::new("memory.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(m.state(&iden())["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut m = memory(4);
        send(&mut m, "memory.flip", json!({ "card": 0 }));
        let out = send(&mut m, "memory.set", json!({ "key": "pairs", "value": 6 }));
        assert!(out.ok);
        let state = m.state(&iden());
        assert_eq!(state["settings"]["pairs"], json!(6));
        assert_eq!(state["steps"], json!(0));
        assert!(!send(&mut m, "memory.set", json!({ "key": "pairs", "value": 99 })).ok);
        assert!(
            !send(
                &mut m,
                "memory.set",
                json!({ "key": "design", "value": "nope" })
            )
            .ok
        );
        assert!(!send(&mut m, "memory.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = memory(11);
        let x = a.faces.iter().position(|&f| f == 0).unwrap();
        let y = a.faces.iter().position(|&f| f == 1).unwrap();
        send(&mut a, "memory.flip", json!({ "card": x }));
        send(&mut a, "memory.flip", json!({ "card": y }));
        let mut b = Memory::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        let next = (0..a.total()).find(|&i| i != x && i != y).unwrap();
        for m in [&mut a, &mut b] {
            send(m, "memory.flip", json!({ "card": next }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut m = Memory::new();
        m.load(&json!({ "seed": "soup", "matched": [1, 2, 3], "settings": 7 }));
        assert_eq!(m.state(&iden())["steps"], json!(0));
        assert_eq!(m.state(&iden())["seed"], json!(0));
    }
    #[test]
    fn state_does_not_leak_hidden_cards_but_save_restores_them() {
        let m = memory(9);
        let board = m.state(&iden())["board"].clone();
        for row in board.as_array().unwrap() {
            for cell in row.as_array().unwrap() {
                assert!(cell.is_null() || cell == "void");
            }
        }
        let saved = m.save();
        let mut restored = Memory::new();
        restored.load(&saved);
        assert_eq!(restored.faces, m.faces);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let m = memory(3);
        let names: Vec<String> = m.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["memory.flip", "memory.reset", "memory.set"]);
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let m = memory(5);
        let state = m.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
    }
}
