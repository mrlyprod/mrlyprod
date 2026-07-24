use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{motif_tile, solid_tile, Frame, Layer, TileSet};
use mrlymusic::cue;
use serde_json::{json, Value as Json};

const MAP_CHOICES: [&str; 4] = ["random", "0", "1", "2"];
const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];
const DIRS: [&str; 4] = ["up", "down", "left", "right"];
const DELTAS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

const SIZE: usize = 11;
const WALL: u8 = 0;
const FLOOR: u8 = 2;
const GHOST: u8 = 3;
const DOOR: u8 = 4;

const MAPS: [&str; 3] = [
    "00000400000\
     03222222230\
     02020002020\
     02222222220\
     02020202020\
     42022122024\
     02020202020\
     02222222220\
     02020002020\
     03222222230\
     00000400000",
    "00000400000\
     03222222230\
     02002020020\
     02022222020\
     02220202220\
     42022122024\
     02220202220\
     02022222020\
     02002020020\
     03222222230\
     00000400000",
    "00000400000\
     03222222230\
     02020202020\
     02222222220\
     02020202020\
     42222122224\
     02020202020\
     02222222220\
     02020202020\
     03222222230\
     00000400000",
];

struct Set {
    map: String,
    ghost_ratio: i64,
    speed: i64,
    reward_food: f64,
    reward_win: f64,
    reward_lose: f64,
    reward_step: f64,
    tile: i64,
    design: String,
}

impl Set {
    fn new() -> Set {
        Set {
            map: "random".to_string(),
            ghost_ratio: 2,
            speed: 1,
            reward_food: 0.0,
            reward_win: 1.0,
            reward_lose: -1.0,
            reward_step: 0.0,
            tile: 3,
            design: "carpet".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "map" => {
                let m = value.as_str().ok_or("value must be a string")?;
                if !MAP_CHOICES.contains(&m) {
                    return Err("no such option");
                }
                self.map = m.to_string();
                Ok(json!(m))
            }
            "ghost_ratio" | "speed" | "tile" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "ghost_ratio" | "speed" => (1, 4),
                    _ => (1, 8),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "ghost_ratio" => self.ghost_ratio = n,
                    "speed" => self.speed = n,
                    _ => self.tile = n,
                }
                Ok(json!(n))
            }
            "reward_food" | "reward_win" | "reward_lose" | "reward_step" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
                    "reward_food" => (0.0, 1.0),
                    "reward_win" => (0.0, 10.0),
                    "reward_lose" => (-10.0, 0.0),
                    _ => (-1.0, 1.0),
                };
                if n < min || n > max {
                    return Err("out of range");
                }
                match key {
                    "reward_food" => self.reward_food = n,
                    "reward_win" => self.reward_win = n,
                    "reward_lose" => self.reward_lose = n,
                    _ => self.reward_step = n,
                }
                Ok(json!(n))
            }
            "design" => {
                let d = value.as_str().ok_or("value must be a string")?;
                if !DESIGNS.contains(&d) {
                    return Err("no such option");
                }
                self.design = d.to_string();
                Ok(json!(d))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "map": self.map,
            "ghost_ratio": self.ghost_ratio,
            "speed": self.speed,
            "reward_food": self.reward_food,
            "reward_win": self.reward_win,
            "reward_lose": self.reward_lose,
            "reward_step": self.reward_step,
            "tile": self.tile,
            "design": self.design,
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

pub struct Escape {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    score: u64,
    level: u64,
    held: Option<usize>,
    over: bool,
    escaped: Option<bool>,
    base: Vec<u8>,
    food: Vec<bool>,
    ghosts: Vec<(i32, i32)>,
    px: i32,
    py: i32,
    colors: [[u8; 4]; 6],
    dark: bool,
}

impl Default for Escape {
    fn default() -> Escape {
        Escape::new()
    }
}

impl Escape {
    pub fn new() -> Escape {
        let mut escape = Escape {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            score: 0,
            level: 1,
            held: None,
            over: false,
            escaped: None,
            base: Vec::new(),
            food: Vec::new(),
            ghosts: Vec::new(),
            px: 5,
            py: 5,
            colors: [[0, 0, 0, 255]; 6],
            dark: false,
        };
        escape.reset(0);
        escape
    }
    fn parse(idx: usize) -> Vec<u8> {
        MAPS[idx]
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect()
    }
    fn at(&self, x: i32, y: i32) -> u8 {
        self.base[y as usize * SIZE + x as usize]
    }
    fn in_bounds(x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < SIZE && y >= 0 && (y as usize) < SIZE
    }
    fn foods_left(&self) -> bool {
        self.food.iter().any(|&f| f)
    }
    fn ghost_pace(&self) -> u64 {
        (self.set.ghost_ratio as u64)
            .saturating_sub(self.level - 1)
            .max(1)
    }
    fn map_index(&self) -> usize {
        (0..MAPS.len())
            .find(|&i| Escape::parse(i) == self.base)
            .unwrap_or(0)
    }
    fn pac_walkable(&self, x: i32, y: i32) -> bool {
        if !Escape::in_bounds(x, y) {
            return false;
        }
        match self.at(x, y) {
            WALL => false,
            DOOR => !self.foods_left(),
            _ => true,
        }
    }
    fn ghost_walkable(&self, x: i32, y: i32) -> bool {
        Escape::in_bounds(x, y) && !matches!(self.at(x, y), WALL | DOOR)
    }
    fn distances(&self, tx: i32, ty: i32) -> Vec<i32> {
        let mut dist = vec![i32::MAX; SIZE * SIZE];
        let mut queue = vec![(tx, ty)];
        dist[ty as usize * SIZE + tx as usize] = 0;
        let mut head = 0;
        while head < queue.len() {
            let (x, y) = queue[head];
            head += 1;
            let d = dist[y as usize * SIZE + x as usize];
            for &(dx, dy) in &DELTAS {
                let (nx, ny) = (x + dx, y + dy);
                if self.ghost_walkable(nx, ny) {
                    let i = ny as usize * SIZE + nx as usize;
                    if dist[i] == i32::MAX {
                        dist[i] = d + 1;
                        queue.push((nx, ny));
                    }
                }
            }
        }
        dist
    }
    fn step_ghosts(&mut self) {
        let dist = self.distances(self.px, self.py);
        for i in 0..self.ghosts.len() {
            let (gx, gy) = self.ghosts[i];
            if dist[gy as usize * SIZE + gx as usize] == i32::MAX {
                continue;
            }
            let mut best = i32::MAX;
            for &(dx, dy) in &DELTAS {
                let (nx, ny) = (gx + dx, gy + dy);
                if Escape::in_bounds(nx, ny) {
                    best = best.min(dist[ny as usize * SIZE + nx as usize]);
                }
            }
            if best == i32::MAX {
                continue;
            }
            let mut moves: Vec<(i32, i32)> = Vec::new();
            for &(dx, dy) in &DELTAS {
                let (nx, ny) = (gx + dx, gy + dy);
                if Escape::in_bounds(nx, ny) && dist[ny as usize * SIZE + nx as usize] == best {
                    moves.push((nx, ny));
                }
            }
            if !moves.is_empty() {
                self.ghosts[i] = *self.rng.choice(&moves);
            }
        }
    }
    fn id_at(&self, x: i32, y: i32) -> u8 {
        if self.px == x && self.py == y {
            return 5;
        }
        if self.ghosts.iter().any(|&(gx, gy)| gx == x && gy == y) {
            return 4;
        }
        match self.at(x, y) {
            WALL => 1,
            DOOR => {
                if self.foods_left() {
                    1
                } else {
                    3
                }
            }
            _ => {
                if self.food[y as usize * SIZE + x as usize] {
                    2
                } else {
                    0
                }
            }
        }
    }
    fn ids(&self) -> Tensor {
        let mut grid = Tensor::new(vec![SIZE, SIZE]);
        for y in 0..SIZE {
            for x in 0..SIZE {
                grid.set(&[y, x], self.id_at(x as i32, y as i32));
            }
        }
        grid
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn tileset(&self) -> TileSet {
        let k = self.set.tile as usize;
        let clear = [0, 0, 0, 0];
        let d = self.set.design.as_str();
        TileSet::new(
            k,
            vec![
                solid_tile(k, clear),
                motif_tile(d, k, self.colors[1], clear),
                solid_tile(k, self.colors[2]),
                solid_tile(k, self.colors[3]),
                motif_tile("void", k, self.colors[4], clear),
                motif_tile("net", k, self.colors[5], clear),
            ],
        )
    }
    fn render(&self) -> Frame {
        let k = self.set.tile as usize;
        let side = SIZE * k;
        let mut frame = Frame::new(side, side, mrlyui::frame::board(self.dark));
        frame.push(Layer::Tiles {
            ids: self.ids(),
            set: self.tileset(),
        });
        frame
    }
    fn build(&mut self, idx: usize) {
        self.base = Escape::parse(idx);
        self.food = self.base.iter().map(|&t| t == FLOOR).collect();
        self.ghosts = (0..SIZE * SIZE)
            .filter(|&i| self.base[i] == GHOST)
            .map(|i| ((i % SIZE) as i32, (i / SIZE) as i32))
            .collect();
        self.px = 5;
        self.py = 5;
        self.held = None;
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        let idx = match self.set.map.as_str() {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            _ => self.rng.below(MAPS.len()),
        };
        self.build(idx);
        self.steps = 0;
        self.score = 0;
        self.level = 1;
        self.over = false;
        self.escaped = None;
        let picked: Vec<[u8; 4]> = (0..6).map(|_| self.palette()).collect();
        self.colors.copy_from_slice(&picked);
    }
    fn advance_level(&mut self) {
        self.level += 1;
        let idx = match self.set.map.as_str() {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            _ => {
                let current = self.map_index();
                let others: Vec<usize> = (0..MAPS.len()).filter(|&i| i != current).collect();
                *self.rng.choice(&others)
            }
        };
        self.build(idx);
    }
    fn step_once(&mut self) -> (bool, bool) {
        let Some(dir) = self.held else {
            return (false, false);
        };
        let (dx, dy) = DELTAS[dir];
        let (nx, ny) = (self.px + dx, self.py + dy);
        if self.pac_walkable(nx, ny) {
            self.px = nx;
            self.py = ny;
        }
        let cell = self.py as usize * SIZE + self.px as usize;
        let ate = self.food[cell];
        if ate {
            self.food[cell] = false;
            self.score += 1;
        }
        if self.at(self.px, self.py) == DOOR {
            self.advance_level();
            return (ate, true);
        }
        self.steps += 1;
        if self.steps.is_multiple_of(self.ghost_pace()) {
            self.step_ghosts();
        }
        if self
            .ghosts
            .iter()
            .any(|&(gx, gy)| gx == self.px && gy == self.py)
        {
            self.over = true;
            self.escaped = Some(false);
        }
        (ate, false)
    }
}

impl App for Escape {
    fn route(&self) -> &str {
        "escape"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("escape").emoji("👻").category("games")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "score": self.score,
            "steps": self.steps,
            "level": self.level,
            "ghost_pace": self.ghost_pace(),
            "held": self.held.map(|d| DIRS[d]),
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "escaped": self.escaped,
            "pos": [self.px, self.py],
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new(
                "escape.turn",
                json!({ "dir": "up | down | left | right" }),
            ));
            out.push(Verb::new("escape.step", json!({ "n": "int" })));
        }
        out.push(Verb::new("escape.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "escape.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "escape.turn" => {
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
                self.held = Some(dir);
                Outcome::ok(json!({ "dir": DIRS[dir] }))
            }
            "escape.step" => {
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
                let mut ate = false;
                let mut leveled = false;
                for _ in 0..n {
                    if self.over || self.held.is_none() {
                        break;
                    }
                    let (a, l) = self.step_once();
                    ate |= a;
                    leveled |= l;
                    taken += 1;
                }
                let mut out = Outcome::ok(json!({
                    "steps": taken,
                    "score": self.score,
                    "level": self.level,
                    "over": self.over,
                }));
                if ate {
                    out = out.emit(Effect::new("sound", cue::payload("blip")));
                }
                if leveled {
                    out = out.emit(Effect::new("sound", cue::payload("good")));
                }
                if self.over {
                    out = out.emit(Effect::new("sound", cue::payload("lose")));
                }
                out
            }
            "escape.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "escape.set" => {
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
            Some(Call::new("escape.step", json!({ "n": self.set.speed })))
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "score": self.score,
            "level": self.level,
            "held": self.held.map(|d| DIRS[d]),
            "over": self.over,
            "escaped": self.escaped,
            "base": self.base,
            "food": self.food,
            "ghosts": self.ghosts.iter().map(|&(x, y)| json!([x, y])).collect::<Vec<_>>(),
            "px": self.px,
            "py": self.py,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let base: Option<Vec<u8>> = state["base"].as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as u8))
                .collect()
        });
        let food: Option<Vec<bool>> = state["food"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_bool()).collect());
        let ghosts: Option<Vec<(i32, i32)>> = state["ghosts"].as_array().map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    let x = p.get(0)?.as_i64()?;
                    let y = p.get(1)?.as_i64()?;
                    Some((x as i32, y as i32))
                })
                .collect()
        });
        if let (Some(base), Some(food), Some(ghosts)) = (base, food, ghosts) {
            if base.len() == SIZE * SIZE && food.len() == SIZE * SIZE {
                self.base = base;
                self.food = food;
                self.ghosts = ghosts;
                if let (Some(px), Some(py)) = (state["px"].as_i64(), state["py"].as_i64()) {
                    self.px = px as i32;
                    self.py = py as i32;
                }
                self.steps = state["steps"].as_u64().unwrap_or(0);
                self.score = state["score"].as_u64().unwrap_or(0);
                self.level = state["level"].as_u64().unwrap_or(1).max(1);
                self.held = state["held"]
                    .as_str()
                    .and_then(|d| DIRS.iter().position(|&x| x == d));
                self.over = state["over"].as_bool().unwrap_or(false);
                self.escaped = state["escaped"].as_bool();
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

    fn escape(seed: u64) -> Escape {
        seeded(Escape::new(), "escape.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = escape(4);
        let mut b = escape(4);
        for e in [&mut a, &mut b] {
            send(e, "escape.turn", json!({ "dir": "right" }));
            send(e, "escape.step", json!({ "n": 2 }));
            send(e, "escape.turn", json!({ "dir": "up" }));
            send(e, "escape.step", json!({}));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn turn_holds_and_walls_stop() {
        let mut e = escape(4);
        send(&mut e, "escape.set", json!({ "key": "map", "value": "0" }));
        send(&mut e, "escape.turn", json!({ "dir": "right" }));
        let out = send(&mut e, "escape.step", json!({ "n": 2 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(2));
        assert_eq!((e.px, e.py), (7, 5));
        let out = send(&mut e, "escape.step", json!({}));
        assert!(out.ok);
        assert_eq!((e.px, e.py), (7, 5));
        assert_eq!(e.state(&iden())["held"], json!("right"));
    }
    #[test]
    fn steps_idle_until_first_turn() {
        let mut e = escape(4);
        let out = send(&mut e, "escape.step", json!({ "n": 5 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(0));
        assert_eq!(e.state(&iden())["steps"], json!(0));
        assert_eq!(e.state(&iden())["held"], Json::Null);
    }
    #[test]
    fn eating_blips() {
        let mut e = escape(4);
        send(&mut e, "escape.set", json!({ "key": "map", "value": "0" }));
        send(&mut e, "escape.turn", json!({ "dir": "right" }));
        let out = send(&mut e, "escape.step", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["score"], json!(1));
        assert!(out
            .effects
            .contains(&Effect::new("sound", cue::payload("blip"))));
    }
    #[test]
    fn door_advances_the_level() {
        let mut e = escape(4);
        send(&mut e, "escape.set", json!({ "key": "map", "value": "0" }));
        e.food = vec![false; SIZE * SIZE];
        e.score = 3;
        e.px = 5;
        e.py = 1;
        send(&mut e, "escape.turn", json!({ "dir": "up" }));
        let out = send(&mut e, "escape.step", json!({}));
        assert!(out.ok);
        assert_eq!(out.data["level"], json!(2));
        assert!(out
            .effects
            .contains(&Effect::new("sound", cue::payload("good"))));
        let state = e.state(&iden());
        assert_eq!(state["level"], json!(2));
        assert_eq!(state["score"], json!(3));
        assert_eq!(state["over"], json!(false));
        assert_eq!(state["pos"], json!([5, 5]));
        assert_eq!(state["held"], Json::Null);
        assert!(e.foods_left());
        assert_eq!(e.map_index(), 0);
    }
    #[test]
    fn progression_reproduces_and_swaps_the_maze() {
        let mut a = escape(9);
        let mut b = escape(9);
        let was = a.map_index();
        for e in [&mut a, &mut b] {
            e.food = vec![false; SIZE * SIZE];
            e.px = 5;
            e.py = 1;
            send(e, "escape.turn", json!({ "dir": "up" }));
            send(e, "escape.step", json!({}));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
        assert_eq!(a.state(&iden())["level"], json!(2));
        assert_ne!(a.map_index(), was);
    }
    #[test]
    fn ghosts_quicken_as_levels_climb() {
        let mut e = escape(4);
        send(
            &mut e,
            "escape.set",
            json!({ "key": "ghost_ratio", "value": 4 }),
        );
        assert_eq!(e.state(&iden())["ghost_pace"], json!(4));
        e.level = 2;
        assert_eq!(e.state(&iden())["ghost_pace"], json!(3));
        e.level = 9;
        assert_eq!(e.state(&iden())["ghost_pace"], json!(1));
        let mut slow = escape(4);
        send(
            &mut slow,
            "escape.set",
            json!({ "key": "map", "value": "0" }),
        );
        let parked = slow.ghosts.clone();
        send(&mut slow, "escape.turn", json!({ "dir": "right" }));
        send(&mut slow, "escape.step", json!({}));
        assert_eq!(slow.ghosts, parked);
        let mut quick = escape(4);
        send(
            &mut quick,
            "escape.set",
            json!({ "key": "map", "value": "0" }),
        );
        quick.level = 2;
        let parked = quick.ghosts.clone();
        send(&mut quick, "escape.turn", json!({ "dir": "right" }));
        send(&mut quick, "escape.step", json!({}));
        assert_ne!(quick.ghosts, parked);
    }
    #[test]
    fn caught_ends_the_run() {
        let mut e = escape(4);
        send(&mut e, "escape.set", json!({ "key": "map", "value": "0" }));
        e.ghosts = vec![(6, 5)];
        send(&mut e, "escape.turn", json!({ "dir": "right" }));
        let out = send(&mut e, "escape.step", json!({}));
        assert!(out.ok);
        assert!(e.over);
        assert_eq!(e.escaped, Some(false));
        assert!(out
            .effects
            .contains(&Effect::new("sound", cue::payload("lose"))));
        assert_eq!(e.beat(), None);
        assert!(!send(&mut e, "escape.step", json!({})).ok);
        assert!(!send(&mut e, "escape.turn", json!({ "dir": "up" })).ok);
    }
    #[test]
    fn speed_paces_the_beat() {
        let mut e = escape(3);
        assert_eq!(e.beat(), Some(Call::new("escape.step", json!({ "n": 1 }))));
        let out = send(&mut e, "escape.set", json!({ "key": "speed", "value": 3 }));
        assert!(out.ok);
        assert_eq!(e.state(&iden())["settings"]["speed"], json!(3));
        assert_eq!(e.beat(), Some(Call::new("escape.step", json!({ "n": 3 }))));
        assert!(!send(&mut e, "escape.set", json!({ "key": "speed", "value": 0 })).ok);
        assert!(!send(&mut e, "escape.set", json!({ "key": "speed", "value": 5 })).ok);
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut e = escape(1);
        assert!(!send(&mut e, "escape.turn", json!({ "dir": "north" })).ok);
        assert!(!send(&mut e, "escape.turn", json!({})).ok);
        assert!(!send(&mut e, "escape.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut e, "escape.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut e = Escape::new();
        let out = e.act(&iden(), &Call::new("escape.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(e.state(&iden())["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut e = escape(4);
        send(&mut e, "escape.turn", json!({ "dir": "right" }));
        send(&mut e, "escape.step", json!({}));
        let out = send(&mut e, "escape.set", json!({ "key": "map", "value": "1" }));
        assert!(out.ok);
        let state = e.state(&iden());
        assert_eq!(state["settings"]["map"], json!("1"));
        assert_eq!(state["steps"], json!(0));
        assert!(!send(&mut e, "escape.set", json!({ "key": "map", "value": "9" })).ok);
        assert!(
            !send(
                &mut e,
                "escape.set",
                json!({ "key": "ghost_ratio", "value": "fast" })
            )
            .ok
        );
        assert!(!send(&mut e, "escape.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = escape(11);
        send(&mut a, "escape.turn", json!({ "dir": "right" }));
        send(&mut a, "escape.step", json!({ "n": 2 }));
        let mut b = Escape::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for e in [&mut a, &mut b] {
            send(e, "escape.turn", json!({ "dir": "up" }));
            send(e, "escape.step", json!({}));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut e = Escape::new();
        e.load(&json!({ "seed": "soup", "base": [1, 2, 3], "settings": 7 }));
        assert_eq!(e.state(&iden())["steps"], json!(0));
        assert_eq!(e.state(&iden())["seed"], json!(0));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let e = escape(3);
        let names: Vec<String> = e.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["escape.turn", "escape.step", "escape.reset", "escape.set"]
        );
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let e = escape(5);
        let state = e.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
        assert_eq!(state["pos"], json!([5, 5]));
    }
}
