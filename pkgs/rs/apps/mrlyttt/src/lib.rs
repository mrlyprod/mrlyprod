#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The mark skins: tiles, emojis and digits.
pub mod skin;

use mrlycore::audio;
use mrlycore::colors::hex;
use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::{json, Json};
use mrlyos::kernel::{int, App, Call, Effect, Iden, Manifest, Outcome, Verb};

const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];
const MARKS: [&str; 2] = ["x", "o"];
const OPPONENTS: [&str; 2] = ["off", "random"];
const SURFACES: [&str; 2] = ["grid", "canvas"];

const WINS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

struct Set {
    reward_win: i64,
    reward_illegal: i64,
    tile: i64,
    design: String,
    opponent: String,
    surface: String,
    skin: String,
}

impl Set {
    fn new() -> Set {
        Set {
            reward_win: 1000,
            reward_illegal: 0,
            tile: 4,
            design: "carpet".to_string(),
            opponent: "random".to_string(),
            surface: "grid".to_string(),
            skin: "tiles".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "tile" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !(1..=8).contains(&n) {
                    return Err("out of range");
                }
                self.tile = n;
                Ok(json!(n))
            }
            "reward_win" => int(&mut self.reward_win, value, (0, 10000)),
            "reward_illegal" => int(&mut self.reward_illegal, value, (-1000, 0)),
            "design" => {
                let d = value.as_str().ok_or("value must be a string")?;
                if !DESIGNS.contains(&d) {
                    return Err("no such option");
                }
                self.design = d.to_string();
                Ok(json!(d))
            }
            "opponent" => {
                let o = value.as_str().ok_or("value must be a string")?;
                if !OPPONENTS.contains(&o) {
                    return Err("no such option");
                }
                self.opponent = o.to_string();
                Ok(json!(o))
            }
            "surface" | "skin" => {
                let s = value.as_str().ok_or("value must be a string")?;
                let legal: &[&str] = if key == "surface" {
                    &SURFACES
                } else {
                    &skin::VARIANTS
                };
                if !legal.contains(&s) {
                    return Err("no such option");
                }
                match key {
                    "surface" => self.surface = s.to_string(),
                    _ => self.skin = s.to_string(),
                }
                Ok(json!(s))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "reward_win": self.reward_win,
            "reward_illegal": self.reward_illegal,
            "tile": self.tile,
            "design": &self.design,
            "opponent": &self.opponent,
            "surface": &self.surface,
            "skin": &self.skin,
        })
    }
    fn soak(&mut self, key: &str, value: &Json) {
        match key {
            "surface" => {
                if let Some(s) = value.as_str() {
                    if SURFACES.contains(&s) {
                        self.surface = s.to_string();
                    }
                }
            }
            "skin" => {
                if let Some(s) = value.as_str() {
                    if skin::VARIANTS.contains(&s) {
                        self.skin = s.to_string();
                    }
                }
            }
            _ => {
                let _ = self.apply(key, value);
            }
        }
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                set.soak(key, val);
            }
        }
        set
    }
}

/// The tic-tac-toe app: one seeded round on nine cells, played against the random opponent or a second player.
pub struct Ttt {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    over: bool,
    cells: [u8; 9],
    turn: u8,
    x_color: [u8; 4],
    o_color: [u8; 4],
}

impl Default for Ttt {
    fn default() -> Ttt {
        Ttt::new()
    }
}

impl Ttt {
    /// Deals an empty board on seed zero, x to move against the random opponent.
    pub fn new() -> Ttt {
        let mut ttt = Ttt {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            over: false,
            cells: [0; 9],
            turn: 1,
            x_color: [255, 255, 255, 255],
            o_color: [200, 200, 200, 255],
        };
        ttt.reset(0);
        ttt
    }
    fn winner(&self) -> u8 {
        for line in WINS {
            let a = self.cells[line[0]];
            if a != 0 && a == self.cells[line[1]] && a == self.cells[line[2]] {
                return a;
            }
        }
        0
    }
    fn full(&self) -> bool {
        self.cells.iter().all(|&c| c != 0)
    }
    fn empties(&self) -> Vec<usize> {
        (0..9).filter(|&i| self.cells[i] == 0).collect()
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.over = false;
        self.cells = [0; 9];
        self.turn = 1;
        self.x_color = self.palette();
        loop {
            self.o_color = self.palette();
            if self.o_color != self.x_color {
                break;
            }
        }
    }
    fn place(&mut self, cell: usize) -> bool {
        self.cells[cell] = self.turn;
        self.steps += 1;
        if self.winner() != 0 || self.full() {
            self.over = true;
            return true;
        }
        self.turn = if self.turn == 1 { 2 } else { 1 };
        false
    }
    fn board_facts(&self) -> Vec<Vec<Json>> {
        (0..3)
            .map(|r| {
                (0..3)
                    .map(|c| match self.cells[r * 3 + c] {
                        1 => json!("x"),
                        2 => json!("o"),
                        _ => Json::Null,
                    })
                    .collect()
            })
            .collect()
    }
    fn winner_fact(&self) -> Json {
        if !self.over {
            return Json::Null;
        }
        match self.winner() {
            1 => json!("x"),
            2 => json!("o"),
            _ => json!("draw"),
        }
    }
    fn ids(&self) -> Vec<Vec<u8>> {
        (0..3)
            .map(|r| (0..3).map(|c| self.cells[r * 3 + c]).collect())
            .collect()
    }
    fn cells_fact(&self) -> Json {
        json!({
            "ids": self.ids(),
            "skin": &self.set.skin,
            "pens": [hex(self.x_color), hex(self.o_color)],
            "design": &self.set.design,
        })
    }
    fn cue(&self, mover: u8) -> Effect {
        let name = if !self.over {
            "blip"
        } else {
            match self.winner() {
                0 => "blip",
                w if w == mover => "win",
                _ => "lose",
            }
        };
        Effect::new("sound", audio::cue(name))
    }
}

impl App for Ttt {
    fn route(&self) -> &str {
        "ttt"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("ttt")
            .emoji("⭕")
            .title("tic tac toe")
            .category("puzzles")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "score": 0,
            "steps": self.steps,
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "board": self.board_facts(),
            "winner": self.winner_fact(),
            "turn": MARKS[(self.turn - 1) as usize],
            "cells": self.cells_fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new("ttt.place", json!({ "cell": "int 0..8" })));
        }
        out.push(Verb::new("ttt.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "ttt.set",
            json!({
                "key": {
                    "tile": "int 1..8",
                    "reward_win": "int 0..10000",
                    "reward_illegal": "int -1000..0",
                    "design": DESIGNS.join(" | "),
                    "opponent": OPPONENTS.join(" | "),
                    "surface": SURFACES.join(" | "),
                    "skin": skin::VARIANTS.join(" | "),
                },
                "value": "of key",
            }),
        ));
        out
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "ttt.place" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let cell = match call.arg("cell").as_i64() {
                    Some(c) => c,
                    None => match (call.arg("x").as_i64(), call.arg("y").as_i64()) {
                        (Some(x), Some(y)) if (0..3).contains(&x) && (0..3).contains(&y) => {
                            y * 3 + x
                        }
                        _ => return Outcome::fail("cell must be an integer"),
                    },
                };
                if !(0..9).contains(&cell) {
                    return Outcome::fail("cell out of range");
                }
                let cell = cell as usize;
                if self.cells[cell] != 0 {
                    return Outcome::fail("illegal move");
                }
                let mover = self.turn;
                let mark = MARKS[(mover - 1) as usize];
                let ended = self.place(cell);
                if !ended && self.set.opponent == "random" {
                    let empties = self.empties();
                    if !empties.is_empty() {
                        let reply = *self.rng.choice(&empties);
                        let bot = self.turn;
                        self.place(reply);
                        return Outcome::ok(json!({ "cell": cell, "mark": mark, "reply": reply }))
                            .emit(self.cue(bot));
                    }
                }
                Outcome::ok(json!({ "cell": cell, "mark": mark })).emit(self.cue(mover))
            }
            "ttt.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "ttt.set" => {
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
            "steps": self.steps,
            "over": self.over,
            "cells": self.cells.to_vec(),
            "turn": self.turn,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let Some(arr) = state["cells"].as_array() {
            if arr.len() == 9 {
                let mut cells = [0u8; 9];
                let mut ok = true;
                for (i, v) in arr.iter().enumerate() {
                    match v.as_u64() {
                        Some(n) if n <= 2 => cells[i] = n as u8,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    self.cells = cells;
                    self.turn = match state["turn"].as_u64() {
                        Some(1) => 1,
                        Some(2) => 2,
                        _ => self.turn,
                    };
                    self.over = state["over"].as_bool().unwrap_or(false);
                    self.steps = state["steps"].as_u64().unwrap_or(0);
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
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn ttt(seed: u64) -> Ttt {
        seeded(Ttt::new(), "ttt.reset", seed)
    }
    fn hotseat(seed: u64) -> Ttt {
        let mut t = ttt(seed);
        t.call(
            &iden(),
            &Call::new("ttt.set", json!({ "key": "opponent", "value": "off" })),
        );
        t
    }

    #[test]
    fn seed_reproduces() {
        let mut a = hotseat(3);
        let mut b = hotseat(3);
        for t in [&mut a, &mut b] {
            send(t, "ttt.place", json!({ "cell": 0 }));
            send(t, "ttt.place", json!({ "cell": 4 }));
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn winner_detection_ends_the_round() {
        let mut t = hotseat(1);
        send(&mut t, "ttt.place", json!({ "cell": 0 }));
        send(&mut t, "ttt.place", json!({ "cell": 3 }));
        send(&mut t, "ttt.place", json!({ "cell": 1 }));
        send(&mut t, "ttt.place", json!({ "cell": 4 }));
        let out = send(&mut t, "ttt.place", json!({ "cell": 2 }));
        assert!(out.ok);
        assert!(t.over);
        assert_eq!(t.state(&iden(), None)["winner"], json!("x"));
        assert_eq!(out.effects[0].data, audio::cue("win"));
    }
    #[test]
    fn full_board_without_winner_is_a_draw() {
        let mut t = hotseat(1);
        for cell in [0, 1, 2, 4, 3, 5, 7, 6, 8] {
            send(&mut t, "ttt.place", json!({ "cell": cell }));
        }
        assert!(t.over);
        assert_eq!(t.state(&iden(), None)["winner"], json!("draw"));
    }
    #[test]
    fn random_opponent_replies_in_the_same_act() {
        let mut t = ttt(3);
        let out = send(&mut t, "ttt.place", json!({ "cell": 0 }));
        assert!(out.ok);
        assert!(out.data.get("reply").is_some());
        let state = t.state(&iden(), None);
        let flat: Vec<Json> = state["board"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|r| r.as_array().unwrap().clone())
            .collect();
        let played = flat.iter().filter(|v| !v.is_null()).count();
        assert_eq!(played, 2);
        assert_eq!(state["turn"], json!("x"));
    }
    #[test]
    fn random_opponent_is_deterministic_under_a_seed() {
        let mut a = ttt(9);
        let mut b = ttt(9);
        for t in [&mut a, &mut b] {
            send(t, "ttt.place", json!({ "cell": 4 }));
            send(t, "ttt.place", json!({ "cell": 0 }));
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
        assert_eq!(a.cells, b.cells);
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut t = hotseat(1);
        send(&mut t, "ttt.place", json!({ "cell": 0 }));
        let out = send(&mut t, "ttt.place", json!({ "cell": 0 }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("illegal move"));
        assert!(!send(&mut t, "ttt.place", json!({ "cell": 20 })).ok);
    }
    #[test]
    fn place_accepts_grid_coordinates() {
        let mut t = hotseat(1);
        let out = send(&mut t, "ttt.place", json!({ "x": 1, "y": 1 }));
        assert!(out.ok);
        assert_eq!(out.data["cell"], json!(4));
        assert!(!send(&mut t, "ttt.place", json!({ "x": 9, "y": 0 })).ok);
        assert!(!send(&mut t, "ttt.place", json!({})).ok);
    }
    #[test]
    fn finished_round_rejects_play() {
        let mut t = hotseat(1);
        for cell in [0, 1, 2, 4, 3, 5, 7, 6, 8] {
            send(&mut t, "ttt.place", json!({ "cell": cell }));
        }
        assert!(!send(&mut t, "ttt.place", json!({ "cell": 4 })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut t = Ttt::new();
        let out = t.call(&iden(), &Call::new("ttt.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(t.state(&iden(), None)["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut t = hotseat(4);
        send(&mut t, "ttt.place", json!({ "cell": 0 }));
        let out = send(&mut t, "ttt.set", json!({ "key": "tile", "value": 6 }));
        assert!(out.ok);
        let state = t.state(&iden(), None);
        assert_eq!(state["settings"]["tile"], json!(6));
        assert_eq!(state["steps"], json!(0));
        assert!(!send(&mut t, "ttt.set", json!({ "key": "tile", "value": 999 })).ok);
        assert!(
            !send(
                &mut t,
                "ttt.set",
                json!({ "key": "design", "value": "nope" })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "ttt.set",
                json!({ "key": "opponent", "value": "hard" })
            )
            .ok
        );
        assert!(!send(&mut t, "ttt.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn look_keys_validate_without_resetting() {
        let mut t = hotseat(4);
        send(&mut t, "ttt.place", json!({ "cell": 0 }));
        for (key, value) in [("surface", "canvas"), ("skin", "emojis")] {
            assert!(send(&mut t, "ttt.set", json!({ "key": key, "value": value })).ok);
        }
        let state = t.state(&iden(), None);
        assert_eq!(state["steps"], json!(1));
        assert_eq!(state["cells"]["skin"], json!("emojis"));
        assert!(!send(&mut t, "ttt.set", json!({ "key": "skin", "value": "wax" })).ok);
    }
    #[test]
    fn from_json_keeps_every_combo() {
        let set = Set::from_json(&json!({ "skin": "emojis", "surface": "canvas" }));
        assert_eq!(set.surface, "canvas");
        assert_eq!(set.skin, "emojis");
    }
    #[test]
    fn old_saves_default_to_the_legacy_look() {
        let mut t = Ttt::new();
        t.load(
            &json!({ "seed": 3, "settings": { "tile": 6 }, "cells": [0, 0, 0, 0, 0, 0, 0, 0, 0] }),
        );
        let settings = t.state(&iden(), None)["settings"].clone();
        assert_eq!(settings["tile"], json!(6));
        assert_eq!(settings["opponent"], json!("random"));
        assert_eq!(settings["surface"], json!("grid"));
        assert_eq!(settings["skin"], json!("tiles"));
    }
    #[test]
    fn cells_track_the_played_marks() {
        let mut t = hotseat(1);
        send(&mut t, "ttt.place", json!({ "cell": 0 }));
        let state = t.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["ids"][0][0], json!(1));
        assert_eq!(cells["ids"][0][1], json!(0));
        let pens = cells["pens"].as_array().unwrap();
        assert_eq!(pens.len(), 2);
        assert!(pens.iter().all(|p| p.as_str().unwrap().starts_with('#')));
        assert_ne!(pens[0], pens[1]);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = ttt(11);
        send(&mut a, "ttt.place", json!({ "cell": 0 }));
        let mut b = Ttt::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        for t in [&mut a, &mut b] {
            let cell = t.empties()[0];
            send(t, "ttt.place", json!({ "cell": cell }));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Ttt::new();
        t.load(&json!({ "seed": "soup", "cells": [9, 9, 9], "settings": 7 }));
        assert_eq!(t.state(&iden(), None)["steps"], json!(0));
        assert_eq!(t.state(&iden(), None)["seed"], json!(0));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = ttt(3);
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["ttt.place", "ttt.reset", "ttt.set"]);
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let t = ttt(5);
        let state = t.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["ids"].as_array().unwrap().len(), 3);
        assert_eq!(cells["skin"], json!("tiles"));
        assert_eq!(cells["design"], json!("carpet"));
        assert!(state.get("frame").is_none());
        assert!(state.get("sprites").is_none());
        assert_eq!(state["board"].as_array().unwrap().len(), 3);
    }
}
