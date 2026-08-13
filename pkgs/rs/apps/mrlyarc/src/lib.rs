#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The ten-pen tile skin the boards are drawn with.
pub mod skin;

mod corpus;

use corpus::{Grid, Pair, TaskData, SIDE};
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};
use mrlymath::saga;
use mrlymath::saga::solve;
use mrlyos::kernel::{int, pick, App, Call, Effect, Iden, Manifest, Outcome, Verb};

const SETS: [&str; 3] = ["one", "two", "three"];

const SPLITS: [&str; 2] = ["train", "eval"];

const INKS: [&str; 10] = [
    "#000000", "#0074d9", "#ff4136", "#2ecc40", "#ffdc00", "#aaaaaa", "#f012be", "#ff851b",
    "#7fdbff", "#870c25",
];

const LIVE: &str = "https://arcade.example/task";

const PAIRS: usize = 16;

const TESTS: usize = 8;

const NODES: usize = 2000;

struct Set {
    set: String,
    split: String,
    pen: i64,
}

impl Set {
    fn new() -> Set {
        Set {
            set: "one".to_string(),
            split: "train".to_string(),
            pen: 1,
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "set" => pick(&mut self.set, value, &SETS),
            "split" => pick(&mut self.split, value, &SPLITS),
            "pen" => int(&mut self.pen, value, (0, 9)),
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "set": &self.set,
            "split": &self.split,
            "pen": self.pen,
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

/// The arcade cabinet: grid tasks from two vendored corpora, a working grid and ten pens.
pub struct Arc {
    dial: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    index: usize,
    task: Option<TaskData>,
    taken: bool,
    pair: usize,
    test: usize,
    grid: Grid,
    solved: Vec<bool>,
    tries: Vec<u64>,
    session: Json,
    scorecard: Json,
}

impl Default for Arc {
    fn default() -> Arc {
        Arc::new()
    }
}

impl Arc {
    /// Opens the cabinet on set one's training split with nothing staged yet.
    pub fn new() -> Arc {
        Arc {
            dial: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            index: 0,
            task: None,
            taken: false,
            pair: 0,
            test: 0,
            grid: Grid::blank(3, 3),
            solved: Vec::new(),
            tries: Vec::new(),
            session: Json::Null,
            scorecard: Json::Null,
        }
    }
    fn score(&self) -> i64 {
        self.solved.iter().filter(|&&hit| hit).count() as i64
    }
    fn over(&self) -> bool {
        !self.solved.is_empty() && self.solved.iter().all(|&hit| hit)
    }
    fn stage(&mut self, task: TaskData, taken: bool) {
        self.pair = 0;
        self.test = 0;
        self.solved = vec![false; task.tests.len()];
        self.tries = vec![0; task.tests.len()];
        let input = &task.tests[0].input;
        self.grid = Grid::blank(input.w, input.h);
        self.task = Some(task);
        self.taken = taken;
    }
    fn unstage(&mut self) {
        self.task = None;
        self.taken = false;
        self.pair = 0;
        self.test = 0;
        self.solved = Vec::new();
        self.tries = Vec::new();
        self.grid = Grid::blank(3, 3);
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.index = 0;
        self.session = Json::Null;
        self.scorecard = Json::Null;
        self.unstage();
    }
    fn load_task(&mut self, want: Option<i64>) -> Result<Json, &'static str> {
        if self.dial.set == "three" {
            return Err("no session");
        }
        let shelf = corpus::corpus(&self.dial.set).ok_or("corrupt corpus")?;
        let list = match self.dial.split.as_str() {
            "eval" => &shelf.eval,
            _ => &shelf.train,
        };
        let len = list.len() as i64;
        if len == 0 {
            return Err("empty split");
        }
        let want = match want {
            Some(n) => n,
            None => self.rng.range(0, len - 1),
        };
        let index = want.rem_euclid(len) as usize;
        self.index = index;
        self.stage(list[index].clone(), false);
        Ok(json!({ "task": index as u64, "id": &list[index].id }))
    }
    fn validate(value: &Json) -> Result<TaskData, &'static str> {
        let train = value["train"].as_array().ok_or("bad task")?;
        let test = value["test"].as_array().ok_or("bad task")?;
        if train.is_empty() || train.len() > PAIRS || test.is_empty() || test.len() > TESTS {
            return Err("bad task");
        }
        let read = |list: &[Json]| -> Result<Vec<Pair>, &'static str> {
            list.iter()
                .map(|p| {
                    Ok(Pair {
                        input: Grid::from_json(&p["input"]).ok_or("bad task")?,
                        output: Grid::from_json(&p["output"]).ok_or("bad task")?,
                    })
                })
                .collect()
        };
        let id = value["id"]
            .as_str()
            .filter(|s| (1..=16).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_alphanumeric()))
            .unwrap_or("live")
            .to_string();
        Ok(TaskData {
            id,
            pairs: read(train)?,
            tests: read(test)?,
        })
    }
    fn door_json(task: &TaskData) -> Json {
        let fold = |list: &[Pair]| -> Vec<Json> {
            list.iter()
                .map(|p| json!({ "input": p.input.to_json(), "output": p.output.to_json() }))
                .collect()
        };
        json!({
            "id": &task.id,
            "train": fold(&task.pairs),
            "test": fold(&task.tests),
        })
    }
    fn parse_points(&self, value: &Json) -> Result<Vec<(usize, usize)>, &'static str> {
        let arr = value
            .as_array()
            .ok_or("points must be an array of [x, y] pairs")?;
        let (w, h) = (self.grid.w as i64, self.grid.h as i64);
        let mut out = Vec::with_capacity(arr.len());
        for p in arr {
            let pair = p
                .as_array()
                .filter(|pair| pair.len() == 2)
                .ok_or("points must be an array of [x, y] pairs")?;
            let x = pair[0]
                .as_i64()
                .ok_or("points must be an array of [x, y] pairs")?;
            let y = pair[1]
                .as_i64()
                .ok_or("points must be an array of [x, y] pairs")?;
            if !(0..w).contains(&x) || !(0..h).contains(&y) {
                return Err("point out of bounds");
            }
            out.push((x as usize, y as usize));
        }
        Ok(out)
    }
    fn task_fact(&self) -> Json {
        let Some(task) = &self.task else {
            return Json::Null;
        };
        let pairs: Vec<Json> = task
            .pairs
            .iter()
            .map(|p| json!({ "input": p.input.to_json(), "output": p.output.to_json() }))
            .collect();
        json!({
            "set": &self.dial.set,
            "split": &self.dial.split,
            "index": self.index as u64,
            "id": &task.id,
            "taken": self.taken,
            "pair": self.pair as u64,
            "pairs": pairs,
            "test": json!({
                "i": self.test as u64,
                "input": task.tests[self.test].input.to_json(),
                "tries": self.tries.get(self.test).copied().unwrap_or(0),
            }),
            "tests": task.tests.len() as u64,
            "solved": self.solved.clone(),
        })
    }
    fn cells_fact(&self) -> Json {
        json!({
            "ids": self.grid.to_json(),
            "skin": "tiles",
            "pens": INKS.to_vec(),
        })
    }
    fn library() -> Json {
        let mut out = Vec::new();
        for set in ["one", "two"] {
            let counts = corpus::corpus(set)
                .map(|c| (c.train.len(), c.eval.len()))
                .unwrap_or((0, 0));
            out.push(json!({ "set": set, "split": "train", "count": counts.0 as u64 }));
            out.push(json!({ "set": set, "split": "eval", "count": counts.1 as u64 }));
        }
        Json::Arr(out)
    }
}

fn tensor(grid: &Grid) -> Tensor {
    Tensor::of(grid.cells.clone(), vec![grid.h, grid.w])
}

fn grid_of(tensor: &Tensor) -> Grid {
    Grid {
        w: tensor.shape[1],
        h: tensor.shape[0],
        cells: tensor.bytes().to_vec(),
    }
}

impl App for Arc {
    fn route(&self) -> &str {
        "arc"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("arc").emoji("🧩").category("puzzles")
    }
    fn state(&self, _iden: &Iden, shape: Option<&Json>) -> Json {
        let mut out = json!({
            "score": self.score(),
            "over": self.over(),
            "steps": self.steps,
            "seed": self.seed,
            "settings": self.dial.to_json(),
            "size": json!({ "w": self.grid.w as u64, "h": self.grid.h as u64 }),
            "task": self.task_fact(),
            "live": json!({ "session": self.session.clone(), "scorecard": self.scorecard.clone() }),
            "cells": self.cells_fact(),
        });
        if shape.is_some_and(|s| s.get("library").is_some()) {
            out["library"] = Self::library();
        }
        out
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = vec![Verb::new("arc.load", json!({ "task": "int 0..999" }))];
        if self.task.is_some() {
            out.push(Verb::new("arc.pair", json!({ "i": "int 0..9" })));
            out.push(Verb::new("arc.test", json!({ "i": "int 0..3" })));
            out.push(Verb::new(
                "arc.size",
                json!({ "w": "int 1..30", "h": "int 1..30" }),
            ));
            out.push(Verb::new("arc.copy", json!({})));
            out.push(Verb::new("arc.clear", json!({})));
            out.push(Verb::new(
                "arc.fill",
                json!({ "x": "int 0..29", "y": "int 0..29", "pen": "int 0..9" }),
            ));
            out.push(Verb::new(
                "arc.paint",
                json!({ "points": "points 0..29 0..29" }),
            ));
            out.push(Verb::new("arc.submit", json!({})));
            out.push(Verb::new("arc.solve", json!({ "depth": "int 1..3" })));
        }
        out.push(Verb::new(
            "arc.set",
            json!({
                "key": {
                    "set": "one | two | three",
                    "split": "train | eval",
                    "pen": "0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9",
                },
                "value": "of key",
            }),
        ));
        if self.dial.set == "three" {
            out.push(Verb::new("arc.fetch", json!({})));
        }
        out.push(Verb::new("arc.reset", json!({ "seed": "int" })));
        out
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "arc.load" => match self.load_task(call.arg("task").as_i64()) {
                Ok(fact) => Outcome::ok(fact),
                Err(note) => Outcome::fail(note),
            },
            "arc.pair" => match &self.task {
                Some(task) => {
                    let len = task.pairs.len() as i64;
                    let i = call.arg("i").as_i64().unwrap_or(0).rem_euclid(len) as usize;
                    self.pair = i;
                    Outcome::ok(json!({ "pair": i as u64 }))
                }
                None => Outcome::fail("no task"),
            },
            "arc.test" => match &self.task {
                Some(task) => {
                    let len = task.tests.len() as i64;
                    let i = call.arg("i").as_i64().unwrap_or(0).rem_euclid(len) as usize;
                    let input = &task.tests[i].input;
                    let (w, h) = (input.w, input.h);
                    self.test = i;
                    self.grid = Grid::blank(w, h);
                    Outcome::ok(json!({ "test": i as u64 }))
                }
                None => Outcome::fail("no task"),
            },
            "arc.size" => {
                let w = call.arg("w").as_i64().unwrap_or(0);
                let h = call.arg("h").as_i64().unwrap_or(0);
                let side = 1..=SIDE as i64;
                if !side.contains(&w) || !side.contains(&h) {
                    return Outcome::fail("out of range");
                }
                let (w, h) = (w as usize, h as usize);
                let mut next = Grid::blank(w, h);
                for y in 0..h.min(self.grid.h) {
                    for x in 0..w.min(self.grid.w) {
                        next.cells[y * w + x] = self.grid.cells[y * self.grid.w + x];
                    }
                }
                self.grid = next;
                self.steps += 1;
                Outcome::ok(json!({ "w": w as u64, "h": h as u64 }))
            }
            "arc.copy" => match &self.task {
                Some(task) => {
                    self.grid = task.tests[self.test].input.clone();
                    self.steps += 1;
                    Outcome::ok(json!({ "w": self.grid.w as u64, "h": self.grid.h as u64 }))
                }
                None => Outcome::fail("no task"),
            },
            "arc.clear" => {
                self.grid = Grid::blank(self.grid.w, self.grid.h);
                self.steps += 1;
                Outcome::ok(json!({ "cleared": true }))
            }
            "arc.fill" => {
                let x = call.arg("x").as_i64().unwrap_or(-1);
                let y = call.arg("y").as_i64().unwrap_or(-1);
                let pen = call.arg("pen").as_i64().unwrap_or(-1);
                if !(0..=9).contains(&pen) {
                    return Outcome::fail("no such pen");
                }
                if !(0..self.grid.w as i64).contains(&x) || !(0..self.grid.h as i64).contains(&y) {
                    return Outcome::fail("out of bounds");
                }
                self.grid.cells[y as usize * self.grid.w + x as usize] = pen as u8;
                self.steps += 1;
                Outcome::ok(json!({ "x": x, "y": y, "pen": pen }))
            }
            "arc.paint" => match self.parse_points(call.arg("points")) {
                Ok(points) => {
                    let pen = self.dial.pen as u8;
                    for (x, y) in &points {
                        self.grid.cells[y * self.grid.w + x] = pen;
                    }
                    self.steps += 1;
                    Outcome::ok(json!({ "points": points.len() as u64 }))
                }
                Err(note) => Outcome::fail(note),
            },
            "arc.submit" => {
                let Some(task) = &self.task else {
                    return Outcome::fail("no task");
                };
                let hit = self.grid == task.tests[self.test].output;
                self.tries[self.test] += 1;
                self.steps += 1;
                if !hit {
                    return Outcome::fail("no match");
                }
                self.solved[self.test] = true;
                Outcome::ok(json!({
                    "test": self.test as u64,
                    "score": self.score(),
                    "over": self.over(),
                }))
            }
            "arc.solve" => {
                let Some(task) = &self.task else {
                    return Outcome::fail("no task");
                };
                let depth = call.arg("depth").as_i64().unwrap_or(2);
                if !(1..=3).contains(&depth) {
                    return Outcome::fail("out of range");
                }
                let pairs: Vec<(Tensor, Tensor)> = task
                    .pairs
                    .iter()
                    .map(|p| (tensor(&p.input), tensor(&p.output)))
                    .collect();
                let budget = solve::Budget {
                    depth: depth as usize,
                    nodes: NODES,
                };
                let Some(found) = solve::solve(&pairs, budget) else {
                    return Outcome::fail("no solution");
                };
                let input = tensor(&task.tests[self.test].input);
                match saga::out(&input, &found) {
                    Ok(answer) => {
                        self.grid = grid_of(&answer);
                        self.steps += 1;
                        Outcome::ok(json!({
                            "saga": found.name(),
                            "w": self.grid.w as u64,
                            "h": self.grid.h as u64,
                        }))
                    }
                    Err(_) => Outcome::fail("no solution"),
                }
            }
            "arc.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                let before = (self.dial.set.clone(), self.dial.split.clone());
                match self.dial.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        if (self.dial.set.clone(), self.dial.split.clone()) != before {
                            self.index = 0;
                            self.unstage();
                        }
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            "arc.take" => {
                let data = match call.arg("task") {
                    Json::Null => call.arg("data"),
                    given => given,
                };
                match Self::validate(data) {
                    Ok(task) => {
                        let fact = json!({
                            "id": &task.id,
                            "pairs": task.pairs.len() as u64,
                            "tests": task.tests.len() as u64,
                        });
                        self.stage(task, true);
                        Outcome::ok(fact)
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            "arc.fetch" => {
                if self.dial.set != "three" {
                    return Outcome::fail("not the live set");
                }
                Outcome::ok(json!({ "url": LIVE })).emit(
                    Effect::new("fetch", json!({ "url": LIVE, "as": "json" }))
                        .then(Call::new("arc.take", json!({}))),
                )
            }
            "arc.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "v": 1,
            "settings": self.dial.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "index": self.index as u64,
            "staged": self.task.is_some(),
            "taken": match &self.task {
                Some(task) if self.taken => Self::door_json(task),
                _ => Json::Null,
            },
            "pair": self.pair as u64,
            "test": self.test as u64,
            "grid": self.grid.to_json(),
            "solved": self.solved.clone(),
            "tries": self.tries.clone(),
            "session": self.session.clone(),
            "scorecard": self.scorecard.clone(),
        })
    }
    fn load(&mut self, state: &Json) {
        *self = Arc::new();
        self.dial = Set::from_json(&state["settings"]);
        self.seed = state["seed"].as_u64().unwrap_or(0);
        self.rng = Rng::new(self.seed);
        if state["staged"].as_bool() == Some(true) {
            if let Ok(task) = Self::validate(&state["taken"]) {
                self.stage(task, true);
            } else if self.dial.set != "three" {
                let index = state["index"].as_u64().unwrap_or(0) as i64;
                let _ = self.load_task(Some(index));
            }
        }
        if let Some(task) = &self.task {
            let tests = task.tests.len();
            self.pair = (state["pair"].as_u64().unwrap_or(0) as usize).min(task.pairs.len() - 1);
            self.test = (state["test"].as_u64().unwrap_or(0) as usize).min(tests - 1);
            if let Some(flags) = state["solved"].as_array() {
                let flags: Option<Vec<bool>> = flags.iter().map(Json::as_bool).collect();
                if let Some(flags) = flags.filter(|f| f.len() == tests) {
                    self.solved = flags;
                }
            }
            if let Some(counts) = state["tries"].as_array() {
                let counts: Option<Vec<u64>> = counts.iter().map(Json::as_u64).collect();
                if let Some(counts) = counts.filter(|c| c.len() == tests) {
                    self.tries = counts;
                }
            }
            if let Some(grid) = Grid::from_json(&state["grid"]) {
                self.grid = grid;
            }
        }
        self.steps = state["steps"].as_u64().unwrap_or(0);
        if !state["session"].is_null() {
            self.session = state["session"].clone();
        }
        if !state["scorecard"].is_null() {
            self.scorecard = state["scorecard"].clone();
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
    use mrlyos::kernel::Os;

    fn loaded() -> Arc {
        let mut arc = Arc::new();
        assert!(send(&mut arc, "arc.load", json!({ "task": 0 })).ok);
        arc
    }
    fn tiny() -> Json {
        json!({
            "id": "tiny1",
            "train": [{ "input": [[0, 1], [1, 0]], "output": [[1, 0], [0, 1]] }],
            "test": [{ "input": [[1, 1], [0, 0]], "output": [[0, 0], [1, 1]] }],
        })
    }

    #[test]
    fn both_corpora_load_with_their_counts() {
        let one = corpus::corpus("one").unwrap();
        assert_eq!(one.train.len(), 400);
        assert_eq!(one.eval.len(), 400);
        let two = corpus::corpus("two").unwrap();
        assert_eq!(two.train.len(), 1000);
        assert_eq!(two.eval.len(), 120);
        assert!(corpus::corpus("three").is_none());
    }
    #[test]
    fn every_vendored_task_is_legal() {
        for set in ["one", "two"] {
            let shelf = corpus::corpus(set).unwrap();
            for list in [&shelf.train, &shelf.eval] {
                for task in list {
                    assert_eq!(task.id.len(), 8, "{set} {}", task.id);
                    assert!(!task.pairs.is_empty() && !task.tests.is_empty());
                    for pair in task.pairs.iter().chain(task.tests.iter()) {
                        for grid in [&pair.input, &pair.output] {
                            assert!((1..=SIDE).contains(&grid.w));
                            assert!((1..=SIDE).contains(&grid.h));
                            assert_eq!(grid.cells.len(), grid.w * grid.h);
                            assert!(grid.cells.iter().all(|&cell| cell <= 9));
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn garbage_grids_parse_to_none() {
        assert!(Grid::from_json(&json!([[1, 2], [3]])).is_none());
        assert!(Grid::from_json(&json!("soup")).is_none());
        assert!(Grid::from_json(&json!([[10]])).is_none());
    }
    #[test]
    fn load_stages_a_task_by_index() {
        let arc = loaded();
        let state = arc.state(&iden(), None);
        assert_eq!(state["task"]["index"], json!(0));
        assert_eq!(state["task"]["set"], json!("one"));
        assert_eq!(state["task"]["id"], json!("007bbfb7"));
        assert_eq!(state["size"], json!({ "w": 3, "h": 3 }));
        assert_eq!(state["score"], json!(0));
        assert_eq!(state["over"], json!(false));
    }
    #[test]
    fn load_wraps_the_index_modulo_the_split() {
        let mut arc = Arc::new();
        let out = send(&mut arc, "arc.load", json!({ "task": 400 }));
        assert!(out.ok);
        assert_eq!(out.data["task"], json!(0));
        assert!(send(&mut arc, "arc.load", json!({ "task": 999 })).ok);
        assert_eq!(arc.index, 199);
    }
    #[test]
    fn load_without_an_index_rolls_one() {
        let mut a = Arc::new();
        let mut b = Arc::new();
        for arc in [&mut a, &mut b] {
            assert!(send(arc, "arc.reset", json!({ "seed": 5 })).ok);
            assert!(send(arc, "arc.load", json!({})).ok);
        }
        assert_eq!(a.index, b.index);
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
    }
    #[test]
    fn load_fails_honestly_in_the_live_set() {
        let mut arc = Arc::new();
        assert!(
            send(
                &mut arc,
                "arc.set",
                json!({ "key": "set", "value": "three" })
            )
            .ok
        );
        let out = send(&mut arc, "arc.load", json!({ "task": 0 }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no session"));
    }
    #[test]
    fn pair_and_test_stage_by_wrapped_index() {
        let mut arc = loaded();
        let pairs = arc.task.as_ref().unwrap().pairs.len() as i64;
        let out = send(&mut arc, "arc.pair", json!({ "i": pairs + 1 }));
        assert!(out.ok);
        assert_eq!(arc.pair, 1);
        assert!(send(&mut arc, "arc.test", json!({ "i": 4 })).ok);
        assert_eq!(arc.test, 0);
        let mut bare = Arc::new();
        assert!(!send(&mut bare, "arc.pair", json!({ "i": 0 })).ok);
        assert!(!send(&mut bare, "arc.test", json!({ "i": 0 })).ok);
    }
    #[test]
    fn copy_clear_and_size_shape_the_working_grid() {
        let mut arc = loaded();
        assert!(send(&mut arc, "arc.copy", json!({})).ok);
        let state = arc.state(&iden(), None);
        assert_eq!(state["cells"]["ids"], state["task"]["test"]["input"]);
        assert!(send(&mut arc, "arc.size", json!({ "w": 5, "h": 2 })).ok);
        assert_eq!(arc.grid.w, 5);
        assert_eq!(arc.grid.h, 2);
        assert_eq!(arc.grid.cells[0], 7);
        assert_eq!(arc.grid.cells[3], 0);
        assert!(send(&mut arc, "arc.clear", json!({})).ok);
        assert!(arc.grid.cells.iter().all(|&cell| cell == 0));
        assert!(!send(&mut arc, "arc.size", json!({ "w": 31, "h": 5 })).ok);
        assert!(!send(&mut arc, "arc.size", json!({ "w": 0, "h": 5 })).ok);
    }
    #[test]
    fn fill_and_paint_ink_the_grid() {
        let mut arc = loaded();
        assert!(send(&mut arc, "arc.fill", json!({ "x": 1, "y": 2, "pen": 4 })).ok);
        assert_eq!(arc.grid.cells[2 * 3 + 1], 4);
        assert!(send(&mut arc, "arc.set", json!({ "key": "pen", "value": 6 })).ok);
        assert!(send(&mut arc, "arc.paint", json!({ "points": [[0, 0], [2, 2]] })).ok);
        assert_eq!(arc.grid.cells[0], 6);
        assert_eq!(arc.grid.cells[8], 6);
    }
    #[test]
    fn out_of_range_paints_fail_with_a_note() {
        let mut arc = loaded();
        let out = send(&mut arc, "arc.fill", json!({ "x": 9, "y": 0, "pen": 1 }));
        assert_eq!(out.note.as_deref(), Some("out of bounds"));
        let out = send(&mut arc, "arc.fill", json!({ "x": 0, "y": 0, "pen": 12 }));
        assert_eq!(out.note.as_deref(), Some("no such pen"));
        let out = send(&mut arc, "arc.paint", json!({ "points": [[0, 9]] }));
        assert_eq!(out.note.as_deref(), Some("point out of bounds"));
        assert!(!send(&mut arc, "arc.paint", json!({ "points": "soup" })).ok);
        assert!(!send(&mut arc, "arc.paint", json!({ "points": [[1]] })).ok);
    }
    #[test]
    fn submit_scores_the_exact_match() {
        let mut arc = loaded();
        let target = arc.task.as_ref().unwrap().tests[0].output.clone();
        assert!(
            send(
                &mut arc,
                "arc.size",
                json!({ "w": target.w, "h": target.h })
            )
            .ok
        );
        for y in 0..target.h {
            for x in 0..target.w {
                let pen = target.cells[y * target.w + x];
                if pen != 0 {
                    assert!(send(&mut arc, "arc.fill", json!({ "x": x, "y": y, "pen": pen })).ok);
                }
            }
        }
        let out = send(&mut arc, "arc.submit", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["score"], json!(1));
        assert_eq!(out.data["over"], json!(true));
        let state = arc.state(&iden(), None);
        assert_eq!(state["score"], json!(1));
        assert_eq!(state["over"], json!(true));
    }
    #[test]
    fn submit_refuses_a_near_miss() {
        let mut arc = loaded();
        assert!(send(&mut arc, "arc.copy", json!({})).ok);
        let out = send(&mut arc, "arc.submit", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no match"));
        let state = arc.state(&iden(), None);
        assert_eq!(state["score"], json!(0));
        assert_eq!(state["task"]["test"]["tries"], json!(1));
    }
    #[test]
    fn take_ingests_a_task_given_as_data() {
        let mut arc = Arc::new();
        let out = send(&mut arc, "arc.take", json!({ "task": tiny() }));
        assert!(out.ok);
        assert_eq!(out.data["id"], json!("tiny1"));
        assert!(send(&mut arc, "arc.fill", json!({ "x": 0, "y": 1, "pen": 1 })).ok);
        assert!(send(&mut arc, "arc.fill", json!({ "x": 1, "y": 1, "pen": 1 })).ok);
        let out = send(&mut arc, "arc.submit", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["over"], json!(true));
    }
    #[test]
    fn take_validates_hard() {
        let mut arc = Arc::new();
        for bad in [
            json!({}),
            json!({ "task": { "train": [], "test": [] } }),
            json!({ "task": { "train": [{ "input": [[1]], "output": [[1]] }] } }),
            json!({ "task": {
                "train": [{ "input": [[1, 2], [3]], "output": [[1]] }],
                "test": [{ "input": [[1]], "output": [[1]] }],
            } }),
            json!({ "task": {
                "train": [{ "input": [[11]], "output": [[1]] }],
                "test": [{ "input": [[1]], "output": [[1]] }],
            } }),
        ] {
            let out = arc.call(&iden(), &Call::new("arc.take", bad));
            assert!(!out.ok);
            assert!(arc.task.is_none());
        }
    }
    #[test]
    fn switching_set_or_split_unstages_the_task() {
        let mut arc = loaded();
        assert!(
            send(
                &mut arc,
                "arc.set",
                json!({ "key": "split", "value": "eval" })
            )
            .ok
        );
        assert!(arc.task.is_none());
        assert!(send(&mut arc, "arc.load", json!({ "task": 3 })).ok);
        assert_eq!(arc.state(&iden(), None)["task"]["split"], json!("eval"));
        assert!(send(&mut arc, "arc.set", json!({ "key": "pen", "value": 5 })).ok);
        assert!(arc.task.is_some());
        assert!(
            !send(
                &mut arc,
                "arc.set",
                json!({ "key": "set", "value": "nine" })
            )
            .ok
        );
        assert!(!send(&mut arc, "arc.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn the_library_read_honors_the_shape_hint() {
        let arc = Arc::new();
        assert!(arc.state(&iden(), None).get("library").is_none());
        let shelf = arc.state(&iden(), Some(&json!({ "library": 1 })));
        let shelf = shelf["library"].as_array().unwrap();
        assert_eq!(shelf.len(), 4);
        assert_eq!(
            shelf[0],
            json!({ "set": "one", "split": "train", "count": 400 })
        );
        assert_eq!(
            shelf[3],
            json!({ "set": "two", "split": "eval", "count": 120 })
        );
    }
    #[test]
    fn fetch_is_advertised_only_in_the_live_set() {
        let mut arc = Arc::new();
        let names: Vec<String> = arc
            .actions(&iden())
            .iter()
            .map(|v| v.name.clone())
            .collect();
        assert_eq!(names, vec!["arc.load", "arc.set", "arc.reset"]);
        assert!(!send(&mut arc, "arc.fetch", json!({})).ok);
        assert!(
            send(
                &mut arc,
                "arc.set",
                json!({ "key": "set", "value": "three" })
            )
            .ok
        );
        let names: Vec<String> = arc
            .actions(&iden())
            .iter()
            .map(|v| v.name.clone())
            .collect();
        assert_eq!(names, vec!["arc.load", "arc.set", "arc.fetch", "arc.reset"]);
        let out = send(&mut arc, "arc.fetch", json!({}));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.effects[0].kind, "fetch");
        assert_eq!(out.effects[0].call.as_ref().unwrap().verb, "arc.take");
    }
    #[test]
    fn the_kernel_refuses_the_fetch_without_internet() {
        let mut os = Os::new(iden()).install(Box::new(Arc::new()));
        os.call(Call::new(
            "arc.set",
            json!({ "key": "set", "value": "three" }),
        ));
        let out = os.call(Call::new("arc.fetch", json!({})));
        assert!(out.ok);
        let env = os.envelope(None);
        assert!(env.effects.iter().all(|e| e.kind != "fetch"));
        assert!(env.notices.iter().any(|n| n.title == "refused"));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let arc = loaded();
        let names: Vec<String> = arc
            .actions(&iden())
            .iter()
            .map(|v| v.name.clone())
            .collect();
        assert_eq!(
            names,
            vec![
                "arc.load",
                "arc.pair",
                "arc.test",
                "arc.size",
                "arc.copy",
                "arc.clear",
                "arc.fill",
                "arc.paint",
                "arc.submit",
                "arc.solve",
                "arc.set",
                "arc.reset"
            ]
        );
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let arc = loaded();
        let state = arc.state(&iden(), None);
        let cells = &state["cells"];
        assert_eq!(cells["skin"], json!("tiles"));
        assert_eq!(cells["ids"].as_array().unwrap().len(), 3);
        let pens = cells["pens"].as_array().unwrap();
        assert_eq!(pens.len(), 10);
        assert!(pens.iter().all(|p| p.as_str().unwrap().starts_with('#')));
        assert!(state.get("frame").is_none());
        assert!(state.get("sprites").is_none());
        assert!(state.get("skin").is_none());
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut arc = loaded();
        let out = arc.call(&iden(), &Call::new("arc.reset", json!({})).at(4000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(4000));
        assert!(arc.task.is_none());
        assert_eq!(arc.state(&iden(), None)["steps"], json!(0));
    }
    #[test]
    fn save_load_roundtrips_mid_task() {
        let mut a = loaded();
        assert!(send(&mut a, "arc.copy", json!({})).ok);
        assert!(send(&mut a, "arc.fill", json!({ "x": 1, "y": 1, "pen": 4 })).ok);
        assert!(!send(&mut a, "arc.submit", json!({})).ok);
        let mut b = Arc::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        for arc in [&mut a, &mut b] {
            assert!(send(arc, "arc.load", json!({})).ok);
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn save_load_carries_a_taken_task() {
        let mut a = Arc::new();
        assert!(send(&mut a, "arc.take", json!({ "task": tiny() })).ok);
        assert!(send(&mut a, "arc.fill", json!({ "x": 0, "y": 0, "pen": 3 })).ok);
        let mut b = Arc::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.state(&iden(), None)["task"]["id"], json!("tiny1"));
    }
    #[test]
    fn solve_cracks_a_vendored_task() {
        let mut arc = Arc::new();
        assert!(send(&mut arc, "arc.load", json!({ "task": 15 })).ok);
        let out = send(&mut arc, "arc.solve", json!({ "depth": 2 }));
        assert!(out.ok);
        assert_eq!(out.data["saga"], json!("map0564312798"));
        let out = send(&mut arc, "arc.submit", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["score"], json!(1));
        assert_eq!(out.data["over"], json!(true));
    }
    #[test]
    fn solve_defaults_the_depth_and_paints_the_answer() {
        let mut arc = Arc::new();
        let shifted = json!({
            "id": "shifted1",
            "train": [{ "input": [[1, 2], [3, 4]], "output": [[2, 3], [4, 5]] }],
            "test": [{ "input": [[4, 3], [2, 1]], "output": [[5, 4], [3, 2]] }],
        });
        assert!(send(&mut arc, "arc.take", json!({ "task": shifted })).ok);
        let out = send(&mut arc, "arc.solve", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["saga"], json!("map0234556789"));
        assert_eq!(out.data["w"], json!(2));
        let state = arc.state(&iden(), None);
        assert_eq!(state["cells"]["ids"], json!([[5, 4], [3, 2]]));
        assert!(send(&mut arc, "arc.submit", json!({})).ok);
    }
    #[test]
    fn solve_reports_honestly_when_beaten() {
        let mut arc = Arc::new();
        let contradictory = json!({
            "id": "nosuch1",
            "train": [
                { "input": [[1]], "output": [[2]] },
                { "input": [[1]], "output": [[3]] },
            ],
            "test": [{ "input": [[1]], "output": [[2]] }],
        });
        assert!(send(&mut arc, "arc.take", json!({ "task": contradictory })).ok);
        let out = send(&mut arc, "arc.solve", json!({ "depth": 3 }));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no solution"));
        let out = send(&mut arc, "arc.solve", json!({ "depth": 9 }));
        assert_eq!(out.note.as_deref(), Some("out of range"));
        let mut bare = Arc::new();
        let out = send(&mut bare, "arc.solve", json!({ "depth": 1 }));
        assert_eq!(out.note.as_deref(), Some("no task"));
    }
    #[test]
    #[ignore]
    fn probe_the_corpus_for_solvable_tasks() {
        let shelf = corpus::corpus("one").unwrap();
        for (index, task) in shelf.train.iter().enumerate() {
            let pairs: Vec<(Tensor, Tensor)> = task
                .pairs
                .iter()
                .map(|p| (tensor(&p.input), tensor(&p.output)))
                .collect();
            let budget = solve::Budget {
                depth: 2,
                nodes: NODES,
            };
            let Some(found) = solve::solve(&pairs, budget) else {
                continue;
            };
            let clean = task.tests.iter().all(|t| {
                saga::out(&tensor(&t.input), &found)
                    .map(|answer| grid_of(&answer) == t.output)
                    .unwrap_or(false)
            });
            println!(
                "index {index} id {} saga {} tests {} clean {clean}",
                task.id,
                found.name(),
                task.tests.len()
            );
        }
    }
    #[test]
    fn load_survives_garbage() {
        let mut arc = Arc::new();
        arc.load(&json!({
            "settings": 7,
            "seed": "soup",
            "staged": true,
            "index": [1],
            "grid": [[1, 2], [3]],
            "solved": "yes",
        }));
        assert_eq!(arc.state(&iden(), None)["seed"], json!(0));
        assert_eq!(arc.state(&iden(), None)["steps"], json!(0));
        arc.load(&Json::Null);
        assert_eq!(arc.state(&iden(), None), Arc::new().state(&iden(), None));
    }
}
