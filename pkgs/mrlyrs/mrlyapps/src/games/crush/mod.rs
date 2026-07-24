use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlyos::kernel::{drive, int, pick, real, App, Call, Effect, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{motif_tile, solid_tile, Frame, Layer, TileSet};
use mrlymusic::cue;
use serde_json::{json, Value as Json};

const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];

struct Set {
    cols: i64,
    rows: i64,
    kinds: i64,
    speed: i64,
    reward_crush: f64,
    tile: i64,
    design: String,
}

impl Set {
    fn new() -> Set {
        Set {
            cols: 9,
            rows: 9,
            kinds: 5,
            speed: 1,
            reward_crush: 1.0,
            tile: 3,
            design: "carpet".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "cols" => int(&mut self.cols, value, (4, 16)),
            "rows" => int(&mut self.rows, value, (4, 16)),
            "kinds" => int(&mut self.kinds, value, (2, 8)),
            "speed" => int(&mut self.speed, value, (1, 8)),
            "tile" => int(&mut self.tile, value, (1, 8)),
            "reward_crush" => real(&mut self.reward_crush, value, (0.0, 10.0)),
            "design" => pick(&mut self.design, value, &DESIGNS),
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "cols": self.cols,
            "rows": self.rows,
            "kinds": self.kinds,
            "speed": self.speed,
            "reward_crush": self.reward_crush,
            "tile": self.tile,
            "design": self.design,
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

pub struct Crush {
    set: Set,
    rng: Rng,
    seed: u64,
    score: u64,
    steps: u64,
    phase: u64,
    over: bool,
    cells: Vec<u8>,
    crushable: Vec<bool>,
    active: Option<(i32, i32, u8)>,
    colors: Vec<[u8; 4]>,
    crush_color: [u8; 4],
    dark: bool,
}

impl Default for Crush {
    fn default() -> Crush {
        Crush::new()
    }
}

impl Crush {
    pub fn new() -> Crush {
        let mut crush = Crush {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            score: 0,
            steps: 0,
            phase: 0,
            over: false,
            cells: Vec::new(),
            crushable: Vec::new(),
            active: None,
            colors: Vec::new(),
            crush_color: [245, 245, 245, 255],
            dark: false,
        };
        crush.reset(0);
        crush
    }
    fn cols(&self) -> i32 {
        self.set.cols as i32
    }
    fn rows(&self) -> i32 {
        self.set.rows as i32
    }
    fn d(&self) -> u8 {
        self.set.kinds as u8
    }
    fn idx(&self, x: i32, y: i32) -> usize {
        y as usize * self.cols() as usize + x as usize
    }
    fn can_move(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.cols() || y >= self.rows() {
            return false;
        }
        y < 0 || self.cells[self.idx(x, y)] == 0
    }
    fn spawn(&mut self) {
        let kind = self.rng.below(self.d() as usize) as u8 + 1;
        let x = self.rng.below(self.cols() as usize) as i32;
        if self.cells[self.idx(x, 0)] != 0 {
            self.over = true;
            self.active = None;
        } else {
            self.active = Some((x, 0, kind));
        }
    }
    fn neighbors(&self, i: usize) -> Vec<usize> {
        let (cols, rows) = (self.cols(), self.rows());
        let (x, y) = ((i % cols as usize) as i32, (i / cols as usize) as i32);
        let mut out = Vec::new();
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < cols && ny >= 0 && ny < rows {
                out.push(self.idx(nx, ny));
            }
        }
        out
    }
    fn mark_matches(&mut self) {
        let n = self.cells.len();
        let mut seen = vec![false; n];
        for start in 0..n {
            if self.cells[start] == 0 || seen[start] {
                continue;
            }
            let kind = self.cells[start];
            let mut group = vec![start];
            let mut stack = vec![start];
            seen[start] = true;
            while let Some(i) = stack.pop() {
                for nb in self.neighbors(i) {
                    if !seen[nb] && self.cells[nb] == kind {
                        seen[nb] = true;
                        group.push(nb);
                        stack.push(nb);
                    }
                }
            }
            if group.len() >= 3 {
                for i in group {
                    self.crushable[i] = true;
                }
            }
        }
    }
    fn crush(&mut self) -> u32 {
        let mut points = 0;
        for i in 0..self.cells.len() {
            if self.crushable[i] {
                self.cells[i] = 0;
                self.crushable[i] = false;
                points += 1;
            }
        }
        if points == 0 {
            return 0;
        }
        let (cols, rows) = (self.cols(), self.rows());
        for x in 0..cols {
            let mut write = rows - 1;
            for y in (0..rows).rev() {
                let i = self.idx(x, y);
                if self.cells[i] != 0 {
                    let w = self.idx(x, write);
                    self.cells[w] = self.cells[i];
                    if w != i {
                        self.cells[i] = 0;
                    }
                    write -= 1;
                }
            }
        }
        self.mark_matches();
        points
    }
    fn lock(&mut self) {
        if let Some((x, y, kind)) = self.active {
            let i = self.idx(x, y);
            self.cells[i] = kind;
        }
        self.active = None;
        self.mark_matches();
        let cols = self.cols();
        let top_full = (0..cols).any(|x| self.cells[self.idx(x, 0)] != 0);
        if top_full {
            self.over = true;
        } else {
            self.spawn();
        }
    }
    fn advance_or_lock(&mut self) {
        if self.over {
            return;
        }
        if let Some((x, y, kind)) = self.active {
            if self.can_move(x, y + 1) {
                self.active = Some((x, y + 1, kind));
            } else {
                self.lock();
            }
        }
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.score = 0;
        self.steps = 0;
        self.phase = 0;
        self.over = false;
        let n = (self.cols() * self.rows()) as usize;
        self.cells = vec![0; n];
        self.crushable = vec![false; n];
        self.active = None;
        self.colors = (0..=self.d() as usize).map(|_| self.palette()).collect();
        self.crush_color = self.palette();
        self.spawn();
    }
    fn board_facts(&self) -> Vec<Vec<u8>> {
        let (cols, rows) = (self.cols() as usize, self.rows() as usize);
        let mut out: Vec<Vec<u8>> = (0..rows)
            .map(|r| (0..cols).map(|c| self.cells[r * cols + c]).collect())
            .collect();
        if let Some((x, y, kind)) = self.active {
            out[y as usize][x as usize] = kind;
        }
        out
    }
    fn crushable_facts(&self) -> Vec<Vec<bool>> {
        let cols = self.cols() as usize;
        (0..self.rows() as usize)
            .map(|r| (0..cols).map(|c| self.crushable[r * cols + c]).collect())
            .collect()
    }
    fn active_facts(&self) -> Json {
        match self.active {
            Some((x, y, kind)) => json!({ "x": x, "y": y, "kind": kind }),
            None => Json::Null,
        }
    }
    fn id_at(&self, i: usize) -> u8 {
        let kind = self.cells[i];
        if kind == 0 {
            0
        } else if self.crushable[i] {
            self.d() + kind
        } else {
            kind
        }
    }
    fn ids(&self) -> Tensor {
        let (cols, rows) = (self.cols() as usize, self.rows() as usize);
        let mut grid = Tensor::new(vec![rows, cols]);
        for i in 0..self.cells.len() {
            grid.set(&[i / cols, i % cols], self.id_at(i));
        }
        if let Some((x, y, kind)) = self.active {
            grid.set(&[y as usize, x as usize], kind);
        }
        grid
    }
    fn tileset(&self) -> TileSet {
        let k = self.set.tile as usize;
        let clear = [0, 0, 0, 0];
        let dn = self.set.design.as_str();
        let d = self.d() as usize;
        let mut tiles = vec![solid_tile(k, clear)];
        for kind in 1..=d {
            tiles.push(motif_tile(dn, k, self.colors[kind], clear));
        }
        for _ in 1..=d {
            tiles.push(motif_tile(dn, k, self.crush_color, clear));
        }
        TileSet::new(k, tiles)
    }
    fn render(&self) -> Frame {
        let k = self.set.tile as usize;
        let mut frame = Frame::new(
            self.cols() as usize * k,
            self.rows() as usize * k,
            mrlyui::frame::board(self.dark),
        );
        frame.push(Layer::Tiles {
            ids: self.ids(),
            set: self.tileset(),
        });
        frame
    }
}

impl App for Crush {
    fn route(&self) -> &str {
        "crush"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("crush").emoji("🍬").category("games")
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
            "board": self.board_facts(),
            "crushable": self.crushable_facts(),
            "active": self.active_facts(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new("crush.move", json!({ "dir": "left | right" })));
            out.push(Verb::new("crush.drop", json!({})));
            out.push(Verb::new("crush.crush", json!({})));
            out.push(Verb::new("crush.step", json!({ "n": "int" })));
        }
        out.push(Verb::new("crush.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "crush.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "crush.move" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let (dir, dx) = match call.arg("dir").as_str() {
                    Some("left") => ("left", -1),
                    Some("right") => ("right", 1),
                    _ => return Outcome::fail("dir must be left or right"),
                };
                let Some((x, y, kind)) = self.active else {
                    return Outcome::fail("illegal move");
                };
                if !self.can_move(x + dx, y) {
                    return Outcome::fail("illegal move");
                }
                self.active = Some((x + dx, y, kind));
                self.steps += 1;
                self.advance_or_lock();
                let mut out = Outcome::ok(json!({ "dir": dir, "over": self.over }))
                    .emit(Effect::new("sound", cue::payload("blip")));
                if self.over {
                    out = out.emit(Effect::new("sound", cue::payload("lose")));
                }
                out
            }
            "crush.drop" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let Some((x, mut y, kind)) = self.active else {
                    return Outcome::fail("illegal move");
                };
                while self.can_move(x, y + 1) {
                    y += 1;
                }
                self.active = Some((x, y, kind));
                self.steps += 1;
                self.lock();
                let mut out = Outcome::ok(json!({ "landed": [x, y], "over": self.over }))
                    .emit(Effect::new("sound", cue::payload("blip")));
                if self.over {
                    out = out.emit(Effect::new("sound", cue::payload("lose")));
                }
                out
            }
            "crush.crush" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                if !self.crushable.iter().any(|&c| c) {
                    return Outcome::fail("nothing to crush");
                }
                let points = self.crush();
                self.score += points as u64;
                self.steps += 1;
                self.advance_or_lock();
                let mut out = Outcome::ok(json!({ "crushed": points, "over": self.over }))
                    .emit(Effect::new("sound", cue::payload("good")));
                if self.over {
                    out = out.emit(Effect::new("sound", cue::payload("lose")));
                }
                out
            }
            "crush.step" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let n = match call.arg("n") {
                    Json::Null => 1,
                    given => match given.as_u64() {
                        Some(n) if (1..=1024).contains(&n) => n,
                        _ => return Outcome::fail("n must be 1 to 1024"),
                    },
                };
                let mut taken = 0;
                for _ in 0..n {
                    if self.over {
                        break;
                    }
                    self.phase += 1;
                    if self.phase.is_multiple_of(2) {
                        self.advance_or_lock();
                        self.steps += 1;
                        taken += 1;
                    }
                }
                let mut out = Outcome::ok(json!({ "steps": taken, "over": self.over }));
                if self.over {
                    out = out.emit(Effect::new("sound", cue::payload("lose")));
                }
                out
            }
            "crush.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "crush.set" => {
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
        if self.over {
            None
        } else {
            Some(Call::new("crush.step", json!({ "n": self.set.speed })))
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "score": self.score,
            "steps": self.steps,
            "phase": self.phase,
            "over": self.over,
            "cells": self.cells,
            "crushable": self.crushable,
            "active": self.active.map(|(x, y, kind)| json!([x, y, kind])),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let n = (self.cols() * self.rows()) as usize;
        if let (Some(cells), Some(crushable)) =
            (state["cells"].as_array(), state["crushable"].as_array())
        {
            if cells.len() == n && crushable.len() == n {
                let cells: Option<Vec<u8>> =
                    cells.iter().map(|v| v.as_u64().map(|n| n as u8)).collect();
                let crushable: Option<Vec<bool>> = crushable.iter().map(|v| v.as_bool()).collect();
                if let (Some(cells), Some(crushable)) = (cells, crushable) {
                    self.cells = cells;
                    self.crushable = crushable;
                    self.active = state["active"].as_array().and_then(|a| {
                        if a.len() == 3 {
                            let x = a[0].as_i64()? as i32;
                            let y = a[1].as_i64()? as i32;
                            let kind = a[2].as_u64()? as u8;
                            Some((x, y, kind))
                        } else {
                            None
                        }
                    });
                    self.score = state["score"].as_u64().unwrap_or(0);
                    self.steps = state["steps"].as_u64().unwrap_or(0);
                    self.phase = state["phase"].as_u64().unwrap_or(0);
                    self.over = state["over"].as_bool().unwrap_or(false);
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

    fn crush(seed: u64) -> Crush {
        seeded(Crush::new(), "crush.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = crush(8);
        let mut b = crush(8);
        for c in [&mut a, &mut b] {
            send(c, "crush.step", json!({ "n": 3 }));
            send(c, "crush.drop", json!({}));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn move_shifts_the_active_column_when_legal() {
        let mut c = crush(8);
        let (x, _, _) = c.active.unwrap();
        let (dir, want) = if c.can_move(x + 1, 0) {
            ("right", x + 1)
        } else {
            ("left", x - 1)
        };
        let out = send(&mut c, "crush.move", json!({ "dir": dir }));
        assert!(out.ok);
        assert_eq!(c.active.unwrap().0, want);
        assert_eq!(c.active.unwrap().1, 1);
    }
    #[test]
    fn drop_locks_the_piece() {
        let mut c = crush(8);
        let (x, _, kind) = c.active.unwrap();
        let out = send(&mut c, "crush.drop", json!({}));
        assert!(out.ok);
        assert!(out
            .effects
            .contains(&Effect::new("sound", cue::payload("blip"))));
        let landed = c.idx(x, c.rows() - 1);
        assert_eq!(c.cells[landed], kind);
        assert!(c.active.is_some());
    }
    #[test]
    fn crush_clears_and_scores() {
        let mut c = crush(8);
        c.active = None;
        let r = c.rows() - 1;
        let (i0, i1, i2) = (c.idx(0, r), c.idx(1, r), c.idx(2, r));
        c.cells[i0] = 1;
        c.cells[i1] = 1;
        c.cells[i2] = 1;
        c.crushable = vec![false; c.cells.len()];
        c.mark_matches();
        c.active = Some((5, 0, 1));
        let out = send(&mut c, "crush.crush", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["crushed"], json!(3));
        assert!(out
            .effects
            .contains(&Effect::new("sound", cue::payload("good"))));
        assert_eq!(c.state(&iden())["score"], json!(3));
        assert_eq!(c.cells[i0], 0);
    }
    #[test]
    fn step_coasts_a_row_every_two_phases() {
        let mut c = crush(8);
        let (x, _, _) = c.active.unwrap();
        let out = send(&mut c, "crush.step", json!({ "n": 6 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(3));
        assert_eq!(c.active.unwrap(), (x, 3, c.active.unwrap().2));
        assert_eq!(c.state(&iden())["steps"], json!(3));
        let out = send(&mut c, "crush.step", json!({ "n": 1 }));
        assert_eq!(out.data["steps"], json!(0));
        assert_eq!(c.active.unwrap().1, 3);
        assert!(!send(&mut c, "crush.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut c, "crush.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn speed_paces_the_beat() {
        let mut c = crush(8);
        assert_eq!(c.beat(), Some(Call::new("crush.step", json!({ "n": 1 }))));
        let out = send(&mut c, "crush.set", json!({ "key": "speed", "value": 4 }));
        assert!(out.ok);
        assert_eq!(c.state(&iden())["settings"]["speed"], json!(4));
        assert_eq!(c.beat(), Some(Call::new("crush.step", json!({ "n": 4 }))));
        assert!(!send(&mut c, "crush.set", json!({ "key": "speed", "value": 0 })).ok);
        assert!(!send(&mut c, "crush.set", json!({ "key": "speed", "value": 9 })).ok);
        c.over = true;
        assert_eq!(c.beat(), None);
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut c = crush(8);
        let out = send(&mut c, "crush.move", json!({ "dir": "north" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("dir must be left or right"));
        c.active = Some((0, 0, 1));
        let out = send(&mut c, "crush.move", json!({ "dir": "left" }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("illegal move"));
        assert_eq!(c.active.unwrap(), (0, 0, 1));
        let out = send(&mut c, "crush.crush", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("nothing to crush"));
    }
    #[test]
    fn finished_round_rejects_play() {
        let mut c = crush(8);
        c.over = true;
        for (verb, args) in [
            ("crush.move", json!({ "dir": "left" })),
            ("crush.drop", json!({})),
            ("crush.crush", json!({})),
            ("crush.step", json!({})),
        ] {
            let out = send(&mut c, verb, args);
            assert!(!out.ok);
            assert_eq!(out.note.as_deref(), Some("round over, reset to continue"));
        }
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut c = Crush::new();
        let out = c.act(&iden(), &Call::new("crush.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(c.state(&iden())["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut c = crush(4);
        let out = send(&mut c, "crush.set", json!({ "key": "cols", "value": 6 }));
        assert!(out.ok);
        let state = c.state(&iden());
        assert_eq!(state["settings"]["cols"], json!(6));
        assert_eq!(state["steps"], json!(0));
        assert_eq!(state["score"], json!(0));
        assert!(!send(&mut c, "crush.set", json!({ "key": "cols", "value": 999 })).ok);
        assert!(
            !send(
                &mut c,
                "crush.set",
                json!({ "key": "design", "value": "nope" })
            )
            .ok
        );
        assert!(!send(&mut c, "crush.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = crush(11);
        send(&mut a, "crush.step", json!({ "n": 4 }));
        send(&mut a, "crush.drop", json!({}));
        let mut b = Crush::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for c in [&mut a, &mut b] {
            send(c, "crush.drop", json!({}));
            send(c, "crush.step", json!({ "n": 2 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut c = Crush::new();
        c.load(&json!({ "seed": "soup", "cells": [1, 2, 3], "settings": 7 }));
        assert_eq!(c.state(&iden())["steps"], json!(0));
        assert_eq!(c.state(&iden())["seed"], json!(0));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let c = crush(3);
        let names: Vec<String> = c.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "crush.move",
                "crush.drop",
                "crush.crush",
                "crush.step",
                "crush.reset",
                "crush.set"
            ]
        );
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let c = crush(5);
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
