#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The tile skins a board can wear: tiles, emojis and digits.
pub mod skin;

use crate::skin::CAP;
use mrlycore::audio;
use mrlycore::colors::hex;
use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::{json, Json};
use mrlyos::kernel::{drive, int, pick, App, Call, Effect, Iden, Manifest, Outcome, Verb};

const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];
const SURFACES: [&str; 2] = ["grid", "canvas"];
const DIRS: [&str; 4] = ["up", "down", "left", "right"];

struct Set {
    grid: i64,
    reward_merge: i64,
    reward_lose: i64,
    tile: i64,
    design: String,
    surface: String,
    skin: String,
}

impl Set {
    fn new() -> Set {
        Set {
            grid: 4,
            reward_merge: 1000,
            reward_lose: 0,
            tile: 3,
            design: "carpet".to_string(),
            surface: "grid".to_string(),
            skin: "digits".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "grid" => int(&mut self.grid, value, (2, 8)),
            "tile" => int(&mut self.tile, value, (1, 40)),
            "reward_merge" => int(&mut self.reward_merge, value, (0, 10000)),
            "reward_lose" => int(&mut self.reward_lose, value, (-100000, 0)),
            "design" => pick(&mut self.design, value, &DESIGNS),
            "surface" => {
                let v = value.as_str().ok_or("value must be a string")?;
                if !SURFACES.contains(&v) {
                    return Err("no such option");
                }
                self.surface = v.to_string();
                Ok(json!(v))
            }
            "skin" => {
                let v = value.as_str().ok_or("value must be a string")?;
                if !skin::VARIANTS.contains(&v) {
                    return Err("no such option");
                }
                self.skin = v.to_string();
                Ok(json!(v))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "grid": self.grid,
            "reward_merge": self.reward_merge,
            "reward_lose": self.reward_lose,
            "tile": self.tile,
            "design": &self.design,
            "surface": &self.surface,
            "skin": &self.skin,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        drive(value, |k, v| {
            let _ = set.apply(k, v);
        });
        set
    }
}

/// The 2048 board: a seeded round of slides, merges and spawns on a square grid.
pub struct Twenty48 {
    set: Set,
    rng: Rng,
    seed: u64,
    score: u64,
    steps: u64,
    over: bool,
    cells: Vec<u32>,
    last_spawn: Option<usize>,
    last_merges: Vec<usize>,
    colors: Vec<[u8; 4]>,
}

impl Default for Twenty48 {
    fn default() -> Twenty48 {
        Twenty48::new()
    }
}

impl Twenty48 {
    /// Opens a fresh round on seed zero, with two tiles already down.
    pub fn new() -> Twenty48 {
        let mut game = Twenty48 {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            score: 0,
            steps: 0,
            over: false,
            cells: Vec::new(),
            last_spawn: None,
            last_merges: Vec::new(),
            colors: Vec::new(),
        };
        game.reset(0);
        game
    }
    fn n(&self) -> usize {
        self.set.grid as usize
    }
    fn empties(&self) -> Vec<usize> {
        (0..self.cells.len())
            .filter(|&i| self.cells[i] == 0)
            .collect()
    }
    fn spawn(&mut self) {
        self.last_spawn = None;
        let empties = self.empties();
        if empties.is_empty() {
            return;
        }
        let at = *self.rng.choice(&empties);
        self.cells[at] = if self.rng.chance(0.9) { 2 } else { 4 };
        self.last_spawn = Some(at);
    }
    fn slide(row: &[u32]) -> (Vec<u32>, u32, Vec<usize>) {
        let packed: Vec<u32> = row.iter().copied().filter(|&v| v != 0).collect();
        let mut out: Vec<u32> = Vec::with_capacity(row.len());
        let mut score = 0;
        let mut hits = Vec::new();
        let mut i = 0;
        while i < packed.len() {
            if i + 1 < packed.len() && packed[i] == packed[i + 1] {
                let merged = packed[i] * 2;
                hits.push(out.len());
                out.push(merged);
                score += merged;
                i += 2;
            } else {
                out.push(packed[i]);
                i += 1;
            }
        }
        out.resize(row.len(), 0);
        (out, score, hits)
    }
    fn rotate(grid: &[u32], n: usize) -> Vec<u32> {
        let mut out = vec![0u32; n * n];
        for i in 0..n {
            for k in 0..n {
                out[i * n + k] = grid[(n - 1 - k) * n + i];
            }
        }
        out
    }
    fn shifted(grid: &[u32], n: usize, action: usize) -> (Vec<u32>, u32, bool, Vec<usize>) {
        let rot = match action {
            0 => 3,
            1 => 1,
            3 => 2,
            _ => 0,
        };
        let mut g = grid.to_vec();
        for _ in 0..rot {
            g = Twenty48::rotate(&g, n);
        }
        let mut score = 0;
        let mut merges: Vec<(usize, usize)> = Vec::new();
        for r in 0..n {
            let (row, s, hits) = Twenty48::slide(&g[r * n..(r + 1) * n]);
            g[r * n..(r + 1) * n].copy_from_slice(&row);
            score += s;
            merges.extend(hits.into_iter().map(|c| (r, c)));
        }
        for _ in 0..((4 - rot) % 4) {
            g = Twenty48::rotate(&g, n);
            merges = merges.iter().map(|&(a, b)| (b, n - 1 - a)).collect();
        }
        let changed = g != grid;
        let merges = merges.into_iter().map(|(r, c)| r * n + c).collect();
        (g, score, changed, merges)
    }
    fn stuck(&self) -> bool {
        if !self.empties().is_empty() {
            return false;
        }
        let n = self.n();
        (0..4).all(|a| !Twenty48::shifted(&self.cells, n, a).2)
    }
    fn coords(&self, i: usize) -> Json {
        let n = self.n();
        json!([i / n, i % n])
    }
    fn exponent(value: u32) -> u8 {
        if value == 0 {
            return 0;
        }
        let mut e = 0u8;
        let mut v = value;
        while v > 1 {
            v >>= 1;
            e += 1;
        }
        e
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.score = 0;
        self.steps = 0;
        self.over = false;
        let n = self.n();
        self.cells = vec![0; n * n];
        self.colors = (0..=CAP).map(|_| self.palette()).collect();
        self.spawn();
        self.spawn();
        self.last_spawn = None;
        self.last_merges = Vec::new();
    }
    fn board_facts(&self) -> Vec<Vec<u32>> {
        let n = self.n();
        (0..n)
            .map(|r| (0..n).map(|c| self.cells[r * n + c]).collect())
            .collect()
    }
    fn ids_facts(&self) -> Vec<Vec<u8>> {
        let n = self.n();
        (0..n)
            .map(|r| {
                (0..n)
                    .map(|c| Twenty48::exponent(self.cells[r * n + c]))
                    .collect()
            })
            .collect()
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn cells_fact(&self) -> Json {
        json!({
            "ids": self.ids_facts(),
            "skin": &self.set.skin,
            "pens": self.colors[1..].iter().map(|&c| hex(c)).collect::<Vec<_>>(),
            "design": &self.set.design,
        })
    }
}

impl App for Twenty48 {
    fn route(&self) -> &str {
        "twenty48"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("twenty48")
            .emoji("🔢")
            .title("2048")
            .category("puzzles")
            .key("up", Call::new("twenty48.slide", json!({ "dir": "up" })))
            .key(
                "down",
                Call::new("twenty48.slide", json!({ "dir": "down" })),
            )
            .key(
                "left",
                Call::new("twenty48.slide", json!({ "dir": "left" })),
            )
            .key(
                "right",
                Call::new("twenty48.slide", json!({ "dir": "right" })),
            )
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "board": self.board_facts(),
            "last_spawn": self.last_spawn.map(|i| self.coords(i)).unwrap_or(Json::Null),
            "last_merges": self.last_merges.iter().map(|&i| self.coords(i)).collect::<Vec<_>>(),
            "cells": self.cells_fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new(
                "twenty48.slide",
                json!({ "dir": "up | down | left | right" }),
            ));
        }
        out.push(Verb::new("twenty48.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "twenty48.set",
            json!({
                "key": {
                    "grid": "int 2..8",
                    "tile": "int 1..40",
                    "reward_merge": "int 0..10000",
                    "reward_lose": "int -100000..0",
                    "design": DESIGNS.join(" | "),
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
            "twenty48.slide" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let Some(dir) = call
                    .arg("dir")
                    .as_str()
                    .and_then(|d| DIRS.iter().position(|&x| x == d))
                else {
                    return Outcome::fail("dir must be up, down, left, or right");
                };
                let n = self.n();
                let (next, score, changed, merges) = Twenty48::shifted(&self.cells, n, dir);
                if !changed {
                    return Outcome::fail("illegal move");
                }
                let before = self.cells.iter().copied().max().unwrap_or(0);
                let peak = next.iter().copied().max().unwrap_or(0);
                self.cells = next;
                self.steps += 1;
                self.score += score as u64;
                self.last_merges = merges;
                self.spawn();
                if self.stuck() {
                    self.over = true;
                }
                let out = Outcome::ok(json!({ "dir": DIRS[dir], "merged": score }));
                if self.over {
                    out.emit(Effect::new("sound", audio::cue("lose")))
                } else if peak > before && peak == 2048 {
                    out.emit(Effect::new("sound", audio::cue("win")))
                } else if peak > before {
                    out.emit(Effect::new("sound", audio::cue("good")))
                } else {
                    out.emit(Effect::new("sound", audio::cue("blip")))
                }
            }
            "twenty48.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "twenty48.set" => {
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
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "cells": self.cells.clone(),
            "last_spawn": self.last_spawn,
            "last_merges": self.last_merges.clone(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let Some(arr) = state["cells"].as_array() {
            let n = self.n();
            if arr.len() == n * n {
                if let Some(cells) = arr.iter().map(|v| v.as_u64().map(|n| n as u32)).collect() {
                    self.cells = cells;
                    self.score = state["score"].as_u64().unwrap_or(0);
                    self.steps = state["steps"].as_u64().unwrap_or(0);
                    self.over = state["over"].as_bool().unwrap_or(false);
                    self.last_spawn = state["last_spawn"]
                        .as_u64()
                        .map(|i| i as usize)
                        .filter(|&i| i < n * n);
                    self.last_merges = state["last_merges"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_u64())
                                .map(|v| v as usize)
                                .filter(|&i| i < n * n)
                                .collect()
                        })
                        .unwrap_or_default();
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

    fn game(seed: u64) -> Twenty48 {
        seeded(Twenty48::new(), "twenty48.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = game(7);
        let mut b = game(7);
        for g in [&mut a, &mut b] {
            send(g, "twenty48.slide", json!({ "dir": "left" }));
            send(g, "twenty48.slide", json!({ "dir": "up" }));
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn slide_merges_and_scores() {
        let mut g = game(1);
        g.cells = vec![0; g.n() * g.n()];
        g.cells[0] = 2;
        g.cells[1] = 2;
        let out = send(&mut g, "twenty48.slide", json!({ "dir": "left" }));
        assert!(out.ok);
        assert_eq!(out.data["merged"], json!(4));
        assert_eq!(g.cells[0], 4);
        assert_eq!(g.state(&iden(), None)["score"], json!(4));
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut g = game(1);
        g.cells = vec![0; g.n() * g.n()];
        g.cells[0] = 2;
        let out = send(&mut g, "twenty48.slide", json!({ "dir": "left" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("illegal move"));
        assert!(!send(&mut g, "twenty48.slide", json!({ "dir": "north" })).ok);
    }
    #[test]
    fn finished_round_rejects_play() {
        let mut g = game(1);
        g.over = true;
        assert!(!send(&mut g, "twenty48.slide", json!({ "dir": "left" })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut g = Twenty48::new();
        let out = g.call(&iden(), &Call::new("twenty48.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(g.state(&iden(), None)["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut g = game(4);
        send(&mut g, "twenty48.slide", json!({ "dir": "left" }));
        let out = send(&mut g, "twenty48.set", json!({ "key": "grid", "value": 6 }));
        assert!(out.ok);
        let state = g.state(&iden(), None);
        assert_eq!(state["settings"]["grid"], json!(6));
        assert_eq!(state["steps"], json!(0));
        assert_eq!(state["score"], json!(0));
        assert!(
            !send(
                &mut g,
                "twenty48.set",
                json!({ "key": "grid", "value": 999 })
            )
            .ok
        );
        assert!(
            !send(
                &mut g,
                "twenty48.set",
                json!({ "key": "design", "value": "nope" })
            )
            .ok
        );
        assert!(
            !send(
                &mut g,
                "twenty48.set",
                json!({ "key": "volume", "value": 1 })
            )
            .ok
        );
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = game(11);
        send(&mut a, "twenty48.slide", json!({ "dir": "left" }));
        send(&mut a, "twenty48.slide", json!({ "dir": "up" }));
        let mut b = Twenty48::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        for g in [&mut a, &mut b] {
            send(g, "twenty48.slide", json!({ "dir": "right" }));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut g = Twenty48::new();
        g.load(&json!({ "seed": "soup", "cells": [1, 2, 3], "settings": 7 }));
        assert_eq!(g.state(&iden(), None)["steps"], json!(0));
        assert_eq!(g.state(&iden(), None)["seed"], json!(0));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let g = game(3);
        let names: Vec<String> = g.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["twenty48.slide", "twenty48.reset", "twenty48.set"]
        );
    }
    #[test]
    fn tile_setting_is_widened_to_forty() {
        let mut g = game(1);
        assert!(
            send(
                &mut g,
                "twenty48.set",
                json!({ "key": "tile", "value": 40 })
            )
            .ok
        );
        assert!(
            !send(
                &mut g,
                "twenty48.set",
                json!({ "key": "tile", "value": 41 })
            )
            .ok
        );
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let mut g = game(1);
        g.cells = vec![0; g.n() * g.n()];
        g.cells[0] = 8;
        let state = g.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["ids"][0][0], json!(3));
        assert_eq!(cells["ids"][0][1], json!(0));
        assert_eq!(cells["skin"], json!("digits"));
        assert_eq!(cells["design"], json!("carpet"));
        assert_eq!(cells["pens"].as_array().unwrap().len(), CAP);
        assert!(state.get("frame").is_none());
        assert!(state.get("skin").is_none());
        assert!(state.get("ids").is_none());
        assert_eq!(state["board"].as_array().unwrap().len(), 4);
    }
    #[test]
    fn last_spawn_and_merges_track_the_slide() {
        let mut g = game(1);
        assert_eq!(g.state(&iden(), None)["last_spawn"], Json::Null);
        g.cells = vec![0; g.n() * g.n()];
        g.cells[0] = 2;
        g.cells[1] = 2;
        send(&mut g, "twenty48.slide", json!({ "dir": "left" }));
        let state = g.state(&iden(), None);
        assert_eq!(state["last_merges"], json!([[0, 0]]));
        let spawn = state["last_spawn"].as_array().unwrap();
        let (r, c) = (spawn[0].as_u64().unwrap(), spawn[1].as_u64().unwrap());
        let v = state["board"][r as usize][c as usize].as_u64().unwrap();
        assert!(v == 2 || v == 4);
    }
    #[test]
    fn merges_map_back_through_rotations() {
        let mut g = game(1);
        let n = g.n();
        g.cells = vec![0; n * n];
        g.cells[2] = 2;
        g.cells[2 + n] = 2;
        send(&mut g, "twenty48.slide", json!({ "dir": "down" }));
        let state = g.state(&iden(), None);
        assert_eq!(state["last_merges"], json!([[3, 2]]));
        assert_eq!(state["board"][3][2], json!(4));
    }
    #[test]
    fn slides_ring_cues_and_facts_roundtrip() {
        let mut g = game(1);
        g.cells = vec![0; g.n() * g.n()];
        g.cells[0] = 2;
        g.cells[1] = 2;
        let out = send(&mut g, "twenty48.slide", json!({ "dir": "left" }));
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.effects[0].kind, "sound");
        assert_eq!(out.effects[0].data, mrlycore::audio::cue("good"));
        let mut b = Twenty48::new();
        b.load(&g.save());
        assert_eq!(b.state(&iden(), None), g.state(&iden(), None));
    }
    #[test]
    fn emojis_ride_either_surface() {
        let mut g = game(4);
        for (key, value) in [("surface", "canvas"), ("skin", "emojis")] {
            assert!(
                send(
                    &mut g,
                    "twenty48.set",
                    json!({ "key": key, "value": value })
                )
                .ok
            );
        }
        let state = g.state(&iden(), None);
        assert_eq!(state["settings"]["surface"], json!("canvas"));
        assert_eq!(state["settings"]["skin"], json!("emojis"));
        assert_eq!(state["cells"]["skin"], json!("emojis"));
        assert!(
            !send(
                &mut g,
                "twenty48.set",
                json!({ "key": "skin", "value": "velvet" })
            )
            .ok
        );
    }
    #[test]
    fn from_json_keeps_every_combo() {
        let mut g = Twenty48::new();
        g.load(&json!({ "seed": 3, "settings": { "skin": "emojis", "surface": "canvas" } }));
        let settings = g.state(&iden(), None)["settings"].clone();
        assert_eq!(settings["surface"], json!("canvas"));
        assert_eq!(settings["skin"], json!("emojis"));
        let mut g = Twenty48::new();
        g.load(&json!({ "seed": 3, "settings": { "grid": 5 } }));
        let settings = g.state(&iden(), None)["settings"].clone();
        assert_eq!(settings["surface"], json!("grid"));
        assert_eq!(settings["skin"], json!("digits"));
    }
}
