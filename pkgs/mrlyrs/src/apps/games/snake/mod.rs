use crate::core::colors::ROLLABLE;
use crate::core::paint::Paint;
use crate::core::rng::Rng;
use crate::core::tensor::Tensor;
use crate::core::tile::{Design, Group, Source, Tile as Model};
use crate::music::cue;
use crate::os::kernel::{drive, flag, int, App, Call, Effect, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{probe, solid_tile, tile_cell, work_cell, Frame, Layer, TileSet};
use serde_json::{json, Value as Json};

const DIRS: [&str; 4] = ["up", "down", "left", "right"];
const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const CEILING: usize = 64;

fn opposite(a: usize, b: usize) -> bool {
    matches!((a, b), (0, 1) | (1, 0) | (2, 3) | (3, 2))
}

#[derive(Clone)]
struct Part {
    tile: Model,
    paint: Option<Paint>,
}

impl Part {
    fn motif(design: Design, invert: bool) -> Part {
        let mut tile = Model::new(Group::General).size(3, 3);
        tile.sources = vec![Source::Classic(design)];
        tile.numbers = vec![3];
        tile.levels = vec![1];
        tile.rotations = vec![0];
        tile.anti = vec![false];
        tile.factor = 3;
        tile.invert = invert;
        Part { tile, paint: None }
    }
    fn carpet() -> Part {
        Part::motif(Design::Carpet, false)
    }
    fn work(&self) -> Json {
        json!({
            "v": 1,
            "tile": self.tile.to_json(),
            "paint": self.paint.as_ref().map(|p| p.to_json()).unwrap_or(Json::Null),
        })
    }
    fn from_work(value: &Json) -> Result<Part, &'static str> {
        if !value.is_object() {
            return Err("value must be a work bundle");
        }
        let tile = Model::from_json(&value["tile"]).map_err(|_| "bad tile")?;
        if tile.max_size() > CEILING || !probe(&tile) {
            return Err("tile does not build");
        }
        let paint = match &value["paint"] {
            Json::Null => None,
            given => Some(Paint::from_json(given).map_err(|_| "bad paint")?),
        };
        Ok(Part { tile, paint })
    }
}

struct Set {
    grid: i64,
    apples: i64,
    wrap: bool,
    self_collision: bool,
    speed: i64,
    tile: i64,
    head: Part,
    body: Part,
    food: Part,
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
            head: Part::carpet(),
            body: Part::carpet(),
            food: Part::carpet(),
        }
    }
    fn legacy(&mut self, name: &str) -> Result<(), &'static str> {
        let part = match name {
            "carpet" => Part::motif(Design::Carpet, false),
            "net" => Part::motif(Design::Net, false),
            "vtree" => Part::motif(Design::Vtree, false),
            "htree" => Part::motif(Design::Htree, false),
            "solid" => Part::motif(Design::Void, true),
            _ => return Err("no such option"),
        };
        self.head = part.clone();
        self.body = part.clone();
        self.food = part;
        Ok(())
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "grid" => int(&mut self.grid, value, (5, 64)),
            "apples" => int(&mut self.apples, value, (1, 16)),
            "speed" => int(&mut self.speed, value, (1, 8)),
            "tile" => int(&mut self.tile, value, (1, 8)),
            "wrap" => flag(&mut self.wrap, value),
            "self_collision" => flag(&mut self.self_collision, value),
            "head" | "body" | "food" => {
                let part = Part::from_work(value)?;
                match key {
                    "head" => self.head = part,
                    "body" => self.body = part,
                    _ => self.food = part,
                }
                Ok(value.clone())
            }
            "design" => {
                let name = value.as_str().ok_or("value must be a string")?;
                self.legacy(name)?;
                Ok(json!(name))
            }
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
            "head": self.head.work(),
            "body": self.body.work(),
            "food": self.food.work(),
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
    dark: bool,
    tiles: TileSet,
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
            dark: false,
            tiles: TileSet::new(1, Vec::new()),
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
        self.retile();
    }
    fn retile(&mut self) {
        let k = self.set.tile as usize;
        let clear = [0, 0, 0, 0];
        let cell = |part: &Part, color: [u8; 4]| match &part.paint {
            Some(coating) => work_cell(&part.tile, coating, k, clear),
            None => tile_cell(&part.tile, k, color, clear),
        };
        self.tiles = TileSet::new(
            k,
            vec![
                solid_tile(k, clear),
                cell(&self.set.head, self.head_color),
                cell(&self.set.body, self.body_color),
                cell(&self.set.food, self.food_color),
            ],
        );
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
    fn render(&self) -> Frame {
        let k = self.set.tile as usize;
        let side = self.set.grid as usize * k;
        let mut frame = Frame::new(side, side, crate::ui::frame::board(self.dark));
        frame.push(Layer::Tiles {
            ids: self.ids(),
            set: self.tiles.clone(),
        });
        frame
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
        Manifest::new("snake").emoji("🐍").category("games")
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
            "dir": DIRS[self.dir],
            "board": self.board(),
            "frame": self.render().fact(),
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
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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
    use crate::os::kernel::testkit::{iden, seeded, send};

    fn snake(seed: u64) -> Snake {
        seeded(Snake::new(), "snake.reset", seed)
    }
    fn net_work() -> Json {
        let part = Part::motif(Design::Net, false);
        part.work()
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
        assert_eq!(a.state(&iden()), b.state(&iden()));
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
        assert_eq!(s.state(&iden())["steps"], json!(5));
        assert!(!send(&mut s, "snake.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut s, "snake.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut s = Snake::new();
        let out = s.act(&iden(), &Call::new("snake.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(s.state(&iden())["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut s = snake(4);
        send(&mut s, "snake.step", json!({ "n": 3 }));
        let out = send(&mut s, "snake.set", json!({ "key": "grid", "value": 8 }));
        assert!(out.ok);
        let state = s.state(&iden());
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
    fn work_keys_accept_bundles() {
        let mut s = snake(4);
        for key in ["head", "body", "food"] {
            let out = send(
                &mut s,
                "snake.set",
                json!({ "key": key, "value": net_work() }),
            );
            assert!(out.ok, "key {key}");
        }
        let settings = s.state(&iden())["settings"].clone();
        for key in ["head", "body", "food"] {
            assert_eq!(settings[key]["tile"]["sources"][0]["design"], "Net");
            assert_eq!(settings[key]["paint"], Json::Null);
        }
    }
    #[test]
    fn work_keys_reject_garbage() {
        let mut s = snake(4);
        assert!(
            !send(
                &mut s,
                "snake.set",
                json!({ "key": "head", "value": "soup" })
            )
            .ok
        );
        assert!(
            !send(
                &mut s,
                "snake.set",
                json!({ "key": "head", "value": { "tile": 7 } })
            )
            .ok
        );
        let mut giant = Part::carpet();
        giant.tile.numbers = vec![81];
        giant.tile.factor = 81;
        giant.tile.width = 81;
        giant.tile.height = 81;
        assert!(
            !send(
                &mut s,
                "snake.set",
                json!({ "key": "head", "value": giant.work() })
            )
            .ok
        );
        let sane = net_work();
        let broken = json!({ "v": 1, "tile": sane["tile"], "paint": { "edition": "Sparkle" } });
        assert!(
            !send(
                &mut s,
                "snake.set",
                json!({ "key": "head", "value": broken })
            )
            .ok
        );
    }
    #[test]
    fn painted_part_renders_via_coat() {
        let mut s = snake(4);
        let plain = s.tiles.tiles[1].clone();
        let work = json!({
            "v": 1,
            "tile": Part::carpet().tile.to_json(),
            "paint": {
                "v": 1,
                "edition": "Simple",
                "scheme": "Multicolor",
                "target": "Fill",
                "primary": "Black",
                "secondary": ["Red"],
                "shades": [],
            },
        });
        let out = send(&mut s, "snake.set", json!({ "key": "head", "value": work }));
        assert!(out.ok);
        let painted = s.tiles.tiles[1].clone();
        assert_ne!(plain.cell.colors, painted.cell.colors);
        let mut again = snake(4);
        send(
            &mut again,
            "snake.set",
            json!({ "key": "head", "value": work }),
        );
        assert_eq!(painted.cell.colors, again.tiles.tiles[1].cell.colors);
    }
    #[test]
    fn legacy_design_saves_migrate() {
        let mut s = Snake::new();
        s.load(&json!({ "seed": 3, "settings": { "design": "net" } }));
        let settings = s.state(&iden())["settings"].clone();
        for key in ["head", "body", "food"] {
            assert_eq!(settings[key]["tile"]["sources"][0]["design"], "Net");
        }
        let mut s = Snake::new();
        s.load(&json!({ "seed": 3, "settings": { "design": "solid" } }));
        let head = s.state(&iden())["settings"]["head"].clone();
        assert_eq!(head["tile"]["sources"][0]["design"], "Void");
        assert_eq!(head["tile"]["invert"], json!(true));
        let mut s = Snake::new();
        s.load(&json!({ "seed": 3, "settings": { "design": "sparkles" } }));
        let head = s.state(&iden())["settings"]["head"].clone();
        assert_eq!(head["tile"]["sources"][0]["design"], "Carpet");
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = snake(11);
        send(
            &mut a,
            "snake.set",
            json!({ "key": "head", "value": net_work() }),
        );
        send(&mut a, "snake.turn", json!({ "dir": "left" }));
        send(&mut a, "snake.step", json!({ "n": 4 }));
        let mut b = Snake::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for s in [&mut a, &mut b] {
            send(s, "snake.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut s = Snake::new();
        s.load(&json!({ "seed": "soup", "body": [[99, 0]], "settings": 7 }));
        assert_eq!(s.state(&iden())["steps"], json!(0));
        assert_eq!(s.state(&iden())["seed"], json!(0));
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
        assert_eq!(s.state(&iden())["settings"]["speed"], json!(5));
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
    fn state_carries_an_indexed_frame() {
        let s = snake(5);
        let state = s.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
        assert_eq!(state["board"].as_array().unwrap().len(), 16);
        assert!(DIRS.contains(&state["dir"].as_str().unwrap()));
        assert_eq!(state["settings"]["head"]["v"], json!(1));
    }
}
