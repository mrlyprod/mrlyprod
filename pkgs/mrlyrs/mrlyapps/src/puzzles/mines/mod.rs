use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::{json, Json};
use mrlymusic::cue;
use mrlyos::kernel::{drive, int, pick, App, Call, Effect, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::hex;

const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];
const SURFACES: [&str; 2] = ["grid", "canvas"];
const SKINS: [&str; 3] = ["tiles", "emojis", "digits"];
const TOOLS: [&str; 2] = ["dig", "flag"];
const MINE_ID: u8 = 10;

struct Set {
    cols: i64,
    rows: i64,
    mines: i64,
    reward_reveal: i64,
    reward_win: i64,
    reward_lose: i64,
    tile: i64,
    design: String,
    surface: String,
    skin: String,
}

impl Set {
    fn new() -> Set {
        Set {
            cols: 9,
            rows: 9,
            mines: 10,
            reward_reveal: 0,
            reward_win: 1000,
            reward_lose: -1000,
            tile: 3,
            design: "carpet".to_string(),
            surface: "grid".to_string(),
            skin: "emojis".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "cols" => int(&mut self.cols, value, (4, 30)),
            "rows" => int(&mut self.rows, value, (4, 30)),
            "mines" => int(&mut self.mines, value, (1, 200)),
            "tile" => int(&mut self.tile, value, (1, 8)),
            "reward_reveal" => int(&mut self.reward_reveal, value, (0, 1000)),
            "reward_win" => int(&mut self.reward_win, value, (0, 10000)),
            "reward_lose" => int(&mut self.reward_lose, value, (-10000, 0)),
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
                if !SKINS.contains(&v) {
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
            "cols": self.cols,
            "rows": self.rows,
            "mines": self.mines,
            "reward_reveal": self.reward_reveal,
            "reward_win": self.reward_win,
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

pub struct Mines {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    over: bool,
    won: Option<bool>,
    tool: String,
    mine: Vec<bool>,
    adj: Vec<u8>,
    shown: Vec<bool>,
    flagged: Vec<bool>,
    placed: bool,
    colors: Vec<[u8; 4]>,
    hidden: [u8; 4],
    mine_color: [u8; 4],
}

impl Default for Mines {
    fn default() -> Mines {
        Mines::new()
    }
}

impl Mines {
    pub fn new() -> Mines {
        let mut mines = Mines {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            over: false,
            won: None,
            tool: "dig".to_string(),
            mine: Vec::new(),
            adj: Vec::new(),
            shown: Vec::new(),
            flagged: Vec::new(),
            placed: false,
            colors: Vec::new(),
            hidden: [70, 70, 78, 255],
            mine_color: [220, 40, 40, 255],
        };
        mines.reset(0);
        mines
    }
    fn cols(&self) -> usize {
        self.set.cols as usize
    }
    fn rows(&self) -> usize {
        self.set.rows as usize
    }
    fn size(&self) -> usize {
        self.cols() * self.rows()
    }
    fn mine_count(&self) -> usize {
        (self.set.mines as usize).min(self.size().saturating_sub(1))
    }
    fn neighbors(&self, i: usize) -> Vec<usize> {
        let (cols, rows) = (self.cols(), self.rows());
        let (r, c) = (i / cols, i % cols);
        let mut out = Vec::new();
        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    out.push(nr as usize * cols + nc as usize);
                }
            }
        }
        out
    }
    fn place(&mut self, safe: usize) {
        let want = self.mine_count();
        let mut placed = 0;
        while placed < want {
            let at = self.rng.below(self.size());
            if at == safe || self.mine[at] {
                continue;
            }
            self.mine[at] = true;
            placed += 1;
        }
        for i in 0..self.size() {
            if self.mine[i] {
                continue;
            }
            self.adj[i] = self.neighbors(i).iter().filter(|&&n| self.mine[n]).count() as u8;
        }
        self.placed = true;
    }
    fn flood(&mut self, start: usize) -> u32 {
        let mut count = 0;
        let mut stack = vec![start];
        while let Some(i) = stack.pop() {
            if self.shown[i] || self.mine[i] {
                continue;
            }
            self.shown[i] = true;
            count += 1;
            if self.adj[i] == 0 {
                stack.extend(self.neighbors(i));
            }
        }
        count
    }
    fn all_safe_shown(&self) -> bool {
        (0..self.size()).all(|i| self.mine[i] || self.shown[i])
    }
    fn revealed_safe(&self) -> u64 {
        (0..self.size())
            .filter(|&i| self.shown[i] && !self.mine[i])
            .count() as u64
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
        self.won = None;
        let n = self.size();
        self.mine = vec![false; n];
        self.adj = vec![0; n];
        self.shown = vec![false; n];
        self.flagged = vec![false; n];
        self.placed = false;
        self.colors = (0..9).map(|_| self.palette()).collect();
        self.hidden = self.palette();
        self.mine_color = self.palette();
    }
    fn board_facts(&self) -> Vec<Vec<Json>> {
        let cols = self.cols();
        (0..self.rows())
            .map(|r| {
                (0..cols)
                    .map(|c| {
                        let i = r * cols + c;
                        if !self.shown[i] {
                            Json::Null
                        } else if self.mine[i] {
                            json!("mine")
                        } else {
                            json!(self.adj[i])
                        }
                    })
                    .collect()
            })
            .collect()
    }
    fn flags_facts(&self) -> Vec<Vec<bool>> {
        let cols = self.cols();
        (0..self.rows())
            .map(|r| (0..cols).map(|c| self.flagged[r * cols + c]).collect())
            .collect()
    }
    fn id_at(&self, i: usize) -> u8 {
        if !self.shown[i] {
            0
        } else if self.mine[i] {
            MINE_ID
        } else {
            1 + self.adj[i]
        }
    }
    fn target(&self, call: &Call) -> Result<usize, &'static str> {
        let cell = match call.arg("cell").as_i64() {
            Some(cell) => cell,
            None => {
                let (Some(x), Some(y)) = (call.arg("x").as_i64(), call.arg("y").as_i64()) else {
                    return Err("cell must be an integer");
                };
                if !(0..self.cols() as i64).contains(&x) || !(0..self.rows() as i64).contains(&y) {
                    return Err("cell out of range");
                }
                y * self.cols() as i64 + x
            }
        };
        if !(0..self.size() as i64).contains(&cell) {
            return Err("cell out of range");
        }
        Ok(cell as usize)
    }
    fn ids_facts(&self) -> Vec<Vec<u8>> {
        let cols = self.cols();
        (0..self.rows())
            .map(|r| (0..cols).map(|c| self.id_at(r * cols + c)).collect())
            .collect()
    }
    fn cells_fact(&self) -> Json {
        let mut pens = vec![hex(self.hidden)];
        pens.extend(self.colors.iter().map(|&c| hex(c)));
        pens.push(hex(self.mine_color));
        json!({
            "ids": self.ids_facts(),
            "skin": &self.set.skin,
            "pens": pens,
            "design": &self.set.design,
        })
    }
}

impl App for Mines {
    fn route(&self) -> &str {
        "mines"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("mines").emoji("💣").category("puzzles")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let flags = self.flagged.iter().filter(|&&f| f).count() as i64;
        json!({
            "score": self.revealed_safe(),
            "steps": self.steps,
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "tool": &self.tool,
            "remaining": self.mine_count() as i64 - flags,
            "board": self.board_facts(),
            "flags": self.flags_facts(),
            "won": self.won,
            "cells": self.cells_fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new("mines.reveal", json!({ "cell": "int" })));
            out.push(Verb::new("mines.flag", json!({ "cell": "int" })));
            out.push(Verb::new("mines.tool", json!({ "tool": "dig | flag" })));
        }
        out.push(Verb::new("mines.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "mines.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "mines.reveal" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let cell = match self.target(call) {
                    Ok(cell) => cell,
                    Err(note) => return Outcome::fail(note),
                };
                if self.flagged[cell] {
                    return Outcome::fail("cell is flagged");
                }
                if self.shown[cell] {
                    return Outcome::fail("illegal move");
                }
                self.steps += 1;
                if !self.placed {
                    self.place(cell);
                }
                if self.mine[cell] {
                    self.over = true;
                    self.won = Some(false);
                    for i in 0..self.size() {
                        if self.mine[i] {
                            self.shown[i] = true;
                        }
                    }
                    return Outcome::ok(json!({ "cell": cell, "hit": "mine" }))
                        .emit(Effect::new("sound", cue::payload("lose")));
                }
                let opened = self.flood(cell);
                if self.all_safe_shown() {
                    self.over = true;
                    self.won = Some(true);
                    return Outcome::ok(json!({ "cell": cell, "opened": opened }))
                        .emit(Effect::new("sound", cue::payload("win")));
                }
                Outcome::ok(json!({ "cell": cell, "opened": opened }))
                    .emit(Effect::new("sound", cue::payload("blip")))
            }
            "mines.flag" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let cell = match self.target(call) {
                    Ok(cell) => cell,
                    Err(note) => return Outcome::fail(note),
                };
                if self.shown[cell] {
                    return Outcome::fail("illegal move");
                }
                self.flagged[cell] = !self.flagged[cell];
                Outcome::ok(json!({ "cell": cell, "flagged": self.flagged[cell] }))
                    .emit(Effect::new("sound", cue::payload("blip")))
            }
            "mines.tool" => {
                let Some(tool) = call.arg("tool").as_str().filter(|t| TOOLS.contains(t)) else {
                    return Outcome::fail("tool must be dig or flag");
                };
                self.tool = tool.to_string();
                Outcome::ok(json!({ "tool": tool }))
            }
            "mines.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "mines.set" => {
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
            "steps": self.steps,
            "over": self.over,
            "won": self.won,
            "tool": &self.tool,
            "mine": self.mine.clone(),
            "adj": self.adj.clone(),
            "shown": self.shown.clone(),
            "flagged": self.flagged.clone(),
            "placed": self.placed,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let n = self.size();
        if let (Some(mine), Some(adj), Some(shown), Some(flagged)) = (
            state["mine"].as_array(),
            state["adj"].as_array(),
            state["shown"].as_array(),
            state["flagged"].as_array(),
        ) {
            if mine.len() == n && adj.len() == n && shown.len() == n && flagged.len() == n {
                let mine: Option<Vec<bool>> = mine.iter().map(|v| v.as_bool()).collect();
                let adj: Option<Vec<u8>> =
                    adj.iter().map(|v| v.as_u64().map(|n| n as u8)).collect();
                let shown: Option<Vec<bool>> = shown.iter().map(|v| v.as_bool()).collect();
                let flagged: Option<Vec<bool>> = flagged.iter().map(|v| v.as_bool()).collect();
                if let (Some(mine), Some(adj), Some(shown), Some(flagged)) =
                    (mine, adj, shown, flagged)
                {
                    self.mine = mine;
                    self.adj = adj;
                    self.shown = shown;
                    self.flagged = flagged;
                    self.placed = state["placed"].as_bool().unwrap_or(false);
                    self.over = state["over"].as_bool().unwrap_or(false);
                    self.won = state["won"].as_bool();
                    self.steps = state["steps"].as_u64().unwrap_or(0);
                }
            }
        }
        if let Some(tool) = state["tool"].as_str().filter(|t| TOOLS.contains(t)) {
            self.tool = tool.to_string();
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

    fn mines(seed: u64) -> Mines {
        seeded(Mines::new(), "mines.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = mines(9);
        let mut b = mines(9);
        for m in [&mut a, &mut b] {
            send(m, "mines.reveal", json!({ "cell": 40 }));
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn first_click_never_loses() {
        for seed in 0..20u64 {
            let mut m = mines(seed);
            let out = send(&mut m, "mines.reveal", json!({ "cell": 40 }));
            assert!(out.ok);
            assert!(!m.over);
        }
    }
    #[test]
    fn flag_toggles() {
        let mut m = mines(5);
        let out = send(&mut m, "mines.flag", json!({ "cell": 3 }));
        assert!(out.ok);
        assert_eq!(out.data["flagged"], json!(true));
        assert_eq!(m.state(&iden(), None)["flags"][0][3], json!(true));
        let out = send(&mut m, "mines.flag", json!({ "cell": 3 }));
        assert_eq!(out.data["flagged"], json!(false));
        assert!(!send(&mut m, "mines.reveal", json!({ "cell": -1 })).ok);
    }
    #[test]
    fn mine_loss_sets_over() {
        let mut m = mines(5);
        send(&mut m, "mines.reveal", json!({ "cell": 40 }));
        let mine_cell = (0..m.size()).find(|&i| m.mine[i]).unwrap();
        let out = send(&mut m, "mines.reveal", json!({ "cell": mine_cell }));
        assert!(out.ok);
        assert!(m.over);
        assert_eq!(m.state(&iden(), None)["won"], json!(false));
        assert!(!send(&mut m, "mines.reveal", json!({ "cell": 0 })).ok);
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut m = mines(5);
        send(&mut m, "mines.reveal", json!({ "cell": 40 }));
        let out = send(&mut m, "mines.reveal", json!({ "cell": 40 }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("illegal move"));
        assert!(!send(&mut m, "mines.reveal", json!({ "cell": 9999 })).ok);
        send(&mut m, "mines.flag", json!({ "cell": 41 }));
        assert!(!send(&mut m, "mines.reveal", json!({ "cell": 41 })).ok);
    }
    #[test]
    fn finished_round_rejects_play() {
        let mut m = mines(5);
        send(&mut m, "mines.reveal", json!({ "cell": 40 }));
        let mine_cell = (0..m.size()).find(|&i| m.mine[i]).unwrap();
        send(&mut m, "mines.reveal", json!({ "cell": mine_cell }));
        assert!(!send(&mut m, "mines.flag", json!({ "cell": 0 })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut m = Mines::new();
        let out = m.call(&iden(), &Call::new("mines.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(m.state(&iden(), None)["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut m = mines(4);
        send(&mut m, "mines.reveal", json!({ "cell": 0 }));
        let out = send(&mut m, "mines.set", json!({ "key": "cols", "value": 6 }));
        assert!(out.ok);
        let state = m.state(&iden(), None);
        assert_eq!(state["settings"]["cols"], json!(6));
        assert_eq!(state["steps"], json!(0));
        assert!(!send(&mut m, "mines.set", json!({ "key": "cols", "value": 999 })).ok);
        assert!(
            !send(
                &mut m,
                "mines.set",
                json!({ "key": "design", "value": "nope" })
            )
            .ok
        );
        assert!(!send(&mut m, "mines.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = mines(11);
        send(&mut a, "mines.reveal", json!({ "cell": 40 }));
        let mut b = Mines::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        let next = (0..a.size()).find(|&i| !a.shown[i] && !a.mine[i]).unwrap();
        for m in [&mut a, &mut b] {
            send(m, "mines.reveal", json!({ "cell": next }));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut m = Mines::new();
        m.load(&json!({ "seed": "soup", "mine": [1, 2, 3], "settings": 7 }));
        assert_eq!(m.state(&iden(), None)["steps"], json!(0));
        assert_eq!(m.state(&iden(), None)["seed"], json!(0));
    }
    #[test]
    fn state_does_not_leak_unrevealed_mines_but_save_restores_them() {
        let m = mines(9);
        let board = m.state(&iden(), None)["board"].clone();
        for row in board.as_array().unwrap() {
            for cell in row.as_array().unwrap() {
                assert!(cell.is_null());
            }
        }
        let saved = m.save();
        let mut restored = Mines::new();
        restored.load(&saved);
        assert_eq!(restored.mine, m.mine);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let m = mines(3);
        let names: Vec<String> = m.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "mines.reveal",
                "mines.flag",
                "mines.tool",
                "mines.reset",
                "mines.set"
            ]
        );
    }
    #[test]
    fn tool_switches_saves_and_rejects_garbage() {
        let mut m = mines(3);
        assert_eq!(m.state(&iden(), None)["tool"], json!("dig"));
        let out = send(&mut m, "mines.tool", json!({ "tool": "flag" }));
        assert!(out.ok);
        assert_eq!(m.state(&iden(), None)["tool"], json!("flag"));
        assert!(!send(&mut m, "mines.tool", json!({ "tool": "hammer" })).ok);
        let mut b = Mines::new();
        b.load(&m.save());
        assert_eq!(b.state(&iden(), None)["tool"], json!("flag"));
    }
    #[test]
    fn remaining_counts_down_with_flags() {
        let mut m = mines(5);
        assert_eq!(m.state(&iden(), None)["remaining"], json!(10));
        send(&mut m, "mines.flag", json!({ "cell": 3 }));
        assert_eq!(m.state(&iden(), None)["remaining"], json!(9));
        send(&mut m, "mines.flag", json!({ "cell": 3 }));
        assert_eq!(m.state(&iden(), None)["remaining"], json!(10));
    }
    #[test]
    fn reveal_accepts_grid_coordinates() {
        let mut a = mines(9);
        let mut b = mines(9);
        send(&mut a, "mines.reveal", json!({ "cell": 40 }));
        send(&mut b, "mines.reveal", json!({ "x": 4, "y": 4 }));
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert!(!send(&mut b, "mines.reveal", json!({ "x": 99, "y": 0 })).ok);
    }
    #[test]
    fn emojis_ride_either_surface() {
        let mut m = mines(4);
        for (key, value) in [("surface", "canvas"), ("skin", "emojis")] {
            assert!(send(&mut m, "mines.set", json!({ "key": key, "value": value })).ok);
        }
        let state = m.state(&iden(), None);
        assert_eq!(state["settings"]["surface"], json!("canvas"));
        assert_eq!(state["cells"]["skin"], json!("emojis"));
        let frame = mrlyui::skin::raster("mines", &state["cells"], 3, false).expect("a frame");
        assert_eq!(frame.width, 9 * mrlyui::skin::SYMBOL);
        assert!(
            !send(
                &mut m,
                "mines.set",
                json!({ "key": "skin", "value": "velvet" })
            )
            .ok
        );
    }
    #[test]
    fn from_json_keeps_every_combo() {
        let mut m = Mines::new();
        m.load(&json!({ "seed": 3, "settings": { "skin": "emojis", "surface": "canvas" } }));
        let settings = m.state(&iden(), None)["settings"].clone();
        assert_eq!(settings["surface"], json!("canvas"));
        assert_eq!(settings["skin"], json!("emojis"));
        let mut m = Mines::new();
        m.load(&json!({ "seed": 3, "settings": { "cols": 6 } }));
        let settings = m.state(&iden(), None)["settings"].clone();
        assert_eq!(settings["surface"], json!("grid"));
        assert_eq!(settings["skin"], json!("emojis"));
    }
    #[test]
    fn cells_pens_bind_hidden_counts_and_mine() {
        let mut m = mines(4);
        send(&mut m, "mines.reveal", json!({ "cell": 40 }));
        let cells = m.state(&iden(), None)["cells"].clone();
        let pens = cells["pens"].as_array().unwrap();
        assert_eq!(pens.len(), 11);
        assert_eq!(pens[0], json!(hex(m.hidden)));
        assert_eq!(pens[1], json!(hex(m.colors[0])));
        assert_eq!(pens[10], json!(hex(m.mine_color)));
        assert_eq!(cells["design"], json!("carpet"));
        let shown = (0..m.size()).find(|&i| m.shown[i]).unwrap();
        let (r, c) = (shown / m.cols(), shown % m.cols());
        assert_eq!(cells["ids"][r][c], json!(1 + m.adj[shown]));
    }
    #[test]
    fn moves_ring_the_right_cues() {
        let mut m = mines(5);
        let out = send(&mut m, "mines.reveal", json!({ "cell": 40 }));
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.effects[0].kind, "sound");
        let out = send(&mut m, "mines.flag", json!({ "cell": 0 }));
        assert_eq!(out.effects[0].data, mrlymusic::cue::payload("blip"));
        send(&mut m, "mines.flag", json!({ "cell": 0 }));
        let mine_cell = (0..m.size()).find(|&i| m.mine[i]).unwrap();
        let out = send(&mut m, "mines.reveal", json!({ "cell": mine_cell }));
        assert_eq!(out.effects[0].data, mrlymusic::cue::payload("lose"));
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let m = mines(5);
        let state = m.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["ids"].as_array().unwrap().len(), 9);
        assert_eq!(cells["ids"][0][0], json!(0));
        assert_eq!(cells["skin"], json!("emojis"));
        assert!(state.get("frame").is_none());
        assert!(state.get("skin").is_none());
        assert!(state.get("ids").is_none());
        assert!(state.get("sprites").is_none());
    }
}
