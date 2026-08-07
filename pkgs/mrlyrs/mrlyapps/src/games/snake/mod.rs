use mrlycore::colors::hex;
use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};
use mrlymusic::cue;
use mrlyos::kernel::{drive, flag, int, pick, App, Call, Effect, Iden, Manifest, Outcome, Verb};

const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];
const DIRS: [&str; 4] = ["up", "down", "left", "right"];
const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn opposite(a: usize, b: usize) -> bool {
    matches!((a, b), (0, 1) | (1, 0) | (2, 3) | (3, 2))
}

struct Set {
    grid: i64,
    apples: i64,
    wrap: bool,
    self_collision: bool,
    speed: i64,
    tile: i64,
    design: String,
}

impl Set {
    fn new() -> Set {
        Set {
            grid: 16,
            apples: 1,
            wrap: true,
            self_collision: true,
            speed: 1,
            tile: 3,
            design: "carpet".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "grid" => int(&mut self.grid, value, (5, 64)),
            "apples" => int(&mut self.apples, value, (1, 16)),
            "speed" => int(&mut self.speed, value, (1, 8)),
            "tile" => int(&mut self.tile, value, (1, 8)),
            "wrap" => flag(&mut self.wrap, value),
            "self_collision" => flag(&mut self.self_collision, value),
            "design" => pick(&mut self.design, value, &DESIGNS),
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "grid": self.grid,
            "apples": self.apples,
            "wrap": self.wrap,
            "self_collision": self.self_collision,
            "speed": self.speed,
            "tile": self.tile,
            "design": &self.design,
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

pub struct Snake {
    set: Set,
    rng: Rng,
    seed: u64,
    score: u64,
    steps: u64,
    over: bool,
    dir: usize,
    body: Vec<(i32, i32)>,
    foods: Vec<(i32, i32)>,
    head_color: [u8; 4],
    body_color: [u8; 4],
    food_color: [u8; 4],
}

impl Default for Snake {
    fn default() -> Snake {
        Snake::new()
    }
}

impl Snake {
    pub fn new() -> Snake {
        let mut snake = Snake {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            score: 0,
            steps: 0,
            over: false,
            dir: 3,
            body: Vec::new(),
            foods: Vec::new(),
            head_color: [255, 255, 255, 255],
            body_color: [200, 200, 200, 255],
            food_color: [255, 0, 0, 255],
        };
        snake.reset(0);
        snake
    }
    fn grid(&self) -> i32 {
        self.set.grid as i32
    }
    fn free(&self) -> Vec<(i32, i32)> {
        let g = self.grid();
        let mut out = Vec::new();
        for r in 0..g {
            for c in 0..g {
                let p = (r, c);
                if !self.body.contains(&p) && !self.foods.contains(&p) {
                    out.push(p);
                }
            }
        }
        out
    }
    fn spawn_food(&mut self) {
        let free = self.free();
        if free.is_empty() {
            return;
        }
        let pick = *self.rng.choice(&free);
        self.foods.push(pick);
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.head_color = self.palette();
        loop {
            self.body_color = self.palette();
            if self.body_color != self.head_color {
                break;
            }
        }
        loop {
            self.food_color = self.palette();
            if self.food_color != self.head_color && self.food_color != self.body_color {
                break;
            }
        }
        self.dir = self.rng.below(4);
        let center = self.grid() / 2;
        let (dr, dc) = DELTAS[self.dir];
        self.body = vec![(center, center), (center - dr, center - dc)];
        self.foods = Vec::new();
        for _ in 0..self.set.apples {
            self.spawn_food();
        }
        self.score = 0;
        self.steps = 0;
        self.over = false;
    }
    fn advance(&mut self, n: u64) -> u64 {
        let mut taken = 0;
        for _ in 0..n {
            if self.over {
                break;
            }
            self.step_once();
            taken += 1;
        }
        taken
    }
    fn step_once(&mut self) {
        let (dr, dc) = DELTAS[self.dir];
        let g = self.grid();
        let head = self.body[0];
        let mut nr = head.0 + dr;
        let mut nc = head.1 + dc;
        self.steps += 1;
        if self.set.wrap {
            nr = nr.rem_euclid(g);
            nc = nc.rem_euclid(g);
        } else if nr < 0 || nr >= g || nc < 0 || nc >= g {
            self.over = true;
            return;
        }
        let new_head = (nr, nc);
        let ate = self.foods.iter().position(|&f| f == new_head);
        let collide_with = if ate.is_some() {
            &self.body[..]
        } else {
            &self.body[..self.body.len() - 1]
        };
        if self.set.self_collision && collide_with.contains(&new_head) {
            self.over = true;
            return;
        }
        self.body.insert(0, new_head);
        if let Some(i) = ate {
            self.foods.remove(i);
            self.spawn_food();
            self.score += 1;
        } else {
            self.body.pop();
        }
    }
    fn ids(&self) -> Tensor {
        let g = self.set.grid as usize;
        let mut grid = Tensor::new(vec![g, g]);
        for &(r, c) in &self.foods {
            grid.set(&[r as usize, c as usize], 3);
        }
        for &(r, c) in &self.body {
            grid.set(&[r as usize, c as usize], 2);
        }
        if let Some(&(r, c)) = self.body.first() {
            grid.set(&[r as usize, c as usize], 1);
        }
        grid
    }
    fn board(&self) -> Vec<Vec<u8>> {
        let ids = self.ids();
        (0..ids.shape[0])
            .map(|r| (0..ids.shape[1]).map(|c| ids.get(&[r, c])).collect())
            .collect()
    }
    fn cells_fact(&self) -> Json {
        json!({
            "ids": self.board(),
            "skin": "tiles",
            "pens": [
                hex(self.head_color),
                hex(self.body_color),
                hex(self.food_color),
            ],
            "design": &self.set.design,
        })
    }
    fn cells(&self, value: &Json) -> Option<Vec<(i32, i32)>> {
        let g = self.grid() as i64;
        let mut out = Vec::new();
        for p in value.as_array()? {
            let r = p[0].as_i64()?;
            let c = p[1].as_i64()?;
            if !(0..g).contains(&r) || !(0..g).contains(&c) {
                return None;
            }
            out.push((r as i32, c as i32));
        }
        Some(out)
    }
}

impl App for Snake {
    fn route(&self) -> &str {
        "snake"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("snake")
            .emoji("🐍")
            .category("games")
            .key("up", Call::new("snake.turn", json!({ "dir": "up" })))
            .key("down", Call::new("snake.turn", json!({ "dir": "down" })))
            .key("left", Call::new("snake.turn", json!({ "dir": "left" })))
            .key("right", Call::new("snake.turn", json!({ "dir": "right" })))
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "dir": DIRS[self.dir],
            "board": self.board(),
            "cells": self.cells_fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new(
                "snake.turn",
                json!({ "dir": "up | down | left | right" }),
            ));
            out.push(Verb::new("snake.step", json!({ "n": "int" })));
        }
        out.push(Verb::new("snake.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "snake.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "snake.turn" => {
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
                if self.body.len() > 1 && opposite(dir, self.dir) {
                    return Outcome::fail("cannot reverse");
                }
                self.dir = dir;
                Outcome::ok(json!({ "dir": DIRS[dir] }))
            }
            "snake.step" => {
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
                let before = self.score;
                let taken = self.advance(n);
                let mut out = Outcome::ok(json!({
                    "steps": taken,
                    "score": self.score,
                    "over": self.over,
                }));
                if self.score > before {
                    out = out.emit(Effect::new("sound", cue::payload("blip")));
                }
                if self.over {
                    out = out.emit(Effect::new("sound", cue::payload("lose")));
                }
                out
            }
            "snake.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "snake.set" => {
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
            Some(Call::new("snake.step", json!({ "n": self.set.speed })))
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
            "dir": DIRS[self.dir],
            "body": self.body.iter().map(|&(r, c)| json!([r, c])).collect::<Vec<_>>(),
            "foods": self.foods.iter().map(|&(r, c)| json!([r, c])).collect::<Vec<_>>(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let (Some(body), Some(foods)) = (self.cells(&state["body"]), self.cells(&state["foods"]))
        {
            if !body.is_empty() {
                self.body = body;
                self.foods = foods;
                if let Some(dir) = state["dir"]
                    .as_str()
                    .and_then(|d| DIRS.iter().position(|&x| x == d))
                {
                    self.dir = dir;
                }
                self.score = state["score"].as_u64().unwrap_or(0);
                self.steps = state["steps"].as_u64().unwrap_or(0);
                self.over = state["over"].as_bool().unwrap_or(false);
                if let Some(pos) = state["pos"].as_u64() {
                    self.rng.seek(pos as u128);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn snake(seed: u64) -> Snake {
        seeded(Snake::new(), "snake.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = snake(123);
        let mut b = snake(123);
        for s in [&mut a, &mut b] {
            send(s, "snake.turn", json!({ "dir": "left" }));
            send(s, "snake.step", json!({ "n": 3 }));
            send(s, "snake.turn", json!({ "dir": "up" }));
            send(s, "snake.step", json!({}));
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn wall_death_ends_the_round() {
        let mut s = snake(2);
        send(
            &mut s,
            "snake.set",
            json!({ "key": "wrap", "value": false }),
        );
        let out = send(&mut s, "snake.step", json!({ "n": 1024 }));
        assert!(out.ok);
        assert!(s.over);
        assert!(out
            .effects
            .contains(&Effect::new("sound", cue::payload("lose"))));
        assert!(!send(&mut s, "snake.step", json!({})).ok);
        assert!(!send(&mut s, "snake.turn", json!({ "dir": "up" })).ok);
        assert_eq!(s.beat(), None);
    }
    #[test]
    fn reversal_fails_honestly() {
        let mut s = snake(7);
        let back = match DIRS[s.dir] {
            "up" => "down",
            "down" => "up",
            "left" => "right",
            _ => "left",
        };
        let before = s.dir;
        let out = send(&mut s, "snake.turn", json!({ "dir": back }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("cannot reverse"));
        assert_eq!(s.dir, before);
        assert!(!send(&mut s, "snake.turn", json!({ "dir": "north" })).ok);
    }
    #[test]
    fn step_counts_and_frame_skips() {
        let mut s = snake(9);
        let out = send(&mut s, "snake.step", json!({ "n": 5 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(5));
        assert_eq!(s.state(&iden(), None)["steps"], json!(5));
        assert!(!send(&mut s, "snake.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut s, "snake.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut s = Snake::new();
        let out = s.call(&iden(), &Call::new("snake.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(s.state(&iden(), None)["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut s = snake(4);
        send(&mut s, "snake.step", json!({ "n": 3 }));
        let out = send(&mut s, "snake.set", json!({ "key": "grid", "value": 8 }));
        assert!(out.ok);
        let state = s.state(&iden(), None);
        assert_eq!(state["settings"]["grid"], json!(8));
        assert_eq!(state["steps"], json!(0));
        assert_eq!(state["board"].as_array().unwrap().len(), 8);
        assert!(!send(&mut s, "snake.set", json!({ "key": "grid", "value": 999 })).ok);
        assert!(
            !send(
                &mut s,
                "snake.set",
                json!({ "key": "wrap", "value": "yes" })
            )
            .ok
        );
        assert!(!send(&mut s, "snake.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn design_dresses_the_cells() {
        let mut s = snake(4);
        let out = send(
            &mut s,
            "snake.set",
            json!({ "key": "design", "value": "net" }),
        );
        assert!(out.ok);
        let state = s.state(&iden(), None);
        assert_eq!(state["settings"]["design"], json!("net"));
        assert_eq!(state["cells"]["design"], json!("net"));
        assert!(
            !send(
                &mut s,
                "snake.set",
                json!({ "key": "design", "value": "sparkles" })
            )
            .ok
        );
    }
    #[test]
    fn legacy_design_saves_migrate() {
        let mut s = Snake::new();
        s.load(&json!({ "seed": 3, "settings": { "design": "net" } }));
        assert_eq!(s.state(&iden(), None)["settings"]["design"], json!("net"));
        let mut s = Snake::new();
        s.load(&json!({ "seed": 3, "settings": { "design": "sparkles" } }));
        assert_eq!(
            s.state(&iden(), None)["settings"]["design"],
            json!("carpet")
        );
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = snake(11);
        send(
            &mut a,
            "snake.set",
            json!({ "key": "design", "value": "net" }),
        );
        send(&mut a, "snake.turn", json!({ "dir": "left" }));
        send(&mut a, "snake.step", json!({ "n": 4 }));
        let mut b = Snake::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        for s in [&mut a, &mut b] {
            send(s, "snake.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut s = Snake::new();
        s.load(&json!({ "seed": "soup", "body": [[99, 0]], "settings": 7 }));
        assert_eq!(s.state(&iden(), None)["steps"], json!(0));
        assert_eq!(s.state(&iden(), None)["seed"], json!(0));
        assert!(!s.body.is_empty());
    }
    #[test]
    fn beat_steps_live_rounds() {
        let s = snake(3);
        assert_eq!(s.beat(), Some(Call::new("snake.step", json!({ "n": 1 }))));
    }
    #[test]
    fn speed_paces_the_beat() {
        let mut s = snake(3);
        let out = send(&mut s, "snake.set", json!({ "key": "speed", "value": 5 }));
        assert!(out.ok);
        assert_eq!(s.state(&iden(), None)["settings"]["speed"], json!(5));
        assert_eq!(s.beat(), Some(Call::new("snake.step", json!({ "n": 5 }))));
        assert!(!send(&mut s, "snake.set", json!({ "key": "speed", "value": 0 })).ok);
        assert!(!send(&mut s, "snake.set", json!({ "key": "speed", "value": 9 })).ok);
    }
    #[test]
    fn eating_blips() {
        let mut s = snake(3);
        let (dr, dc) = DELTAS[s.dir];
        let head = s.body[0];
        s.foods = vec![(head.0 + dr, head.1 + dc)];
        let out = send(&mut s, "snake.step", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["score"], json!(1));
        assert!(out
            .effects
            .contains(&Effect::new("sound", cue::payload("blip"))));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let s = snake(3);
        let names: Vec<String> = s.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["snake.turn", "snake.step", "snake.reset", "snake.set"]
        );
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let s = snake(5);
        let state = s.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["skin"], json!("tiles"));
        assert_eq!(cells["design"], json!("carpet"));
        assert_eq!(cells["ids"].as_array().unwrap().len(), 16);
        let pens = cells["pens"].as_array().unwrap();
        assert_eq!(pens.len(), 3);
        assert!(pens.iter().all(|p| p.as_str().unwrap().starts_with('#')));
        let head = s.body[0];
        assert_eq!(cells["ids"][head.0 as usize][head.1 as usize], json!(1));
        assert!(state.get("frame").is_none());
        assert!(state.get("sprites").is_none());
        assert_eq!(state["board"].as_array().unwrap().len(), 16);
        assert!(DIRS.contains(&state["dir"].as_str().unwrap()));
    }
}
