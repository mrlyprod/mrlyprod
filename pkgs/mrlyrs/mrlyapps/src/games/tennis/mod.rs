use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};
use mrlymusic::cue;
use mrlyos::kernel::{int, App, Call, Effect, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{solid_rect, Frame, Layer, Sprite};

const DIRS: [&str; 4] = ["up", "down", "left", "right"];
const TICK_BASE_MAX: i64 = 12;
const TICK_SCORE_DIVISOR: i64 = 20;
const TICK_MIN: i64 = 4;
const MILLI: i64 = 1000;
const HALF: i64 = 500;
const EDGE: i64 = 50;
const NUDGE: i64 = 1;

struct Set {
    board: i64,
    paddle: i64,
    block: i64,
    rows: i64,
    physics: i64,
    speed: i64,
    scale: i64,
    reward_brick: i64,
    reward_death: i64,
    reward_step: i64,
}

impl Set {
    fn new() -> Set {
        Set {
            board: 18,
            paddle: 5,
            block: 2,
            rows: 4,
            physics: 340,
            speed: 4,
            scale: 8,
            reward_brick: 1000,
            reward_death: -1000,
            reward_step: 0,
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "board" | "paddle" | "block" | "rows" | "speed" | "scale" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "board" => (8, 40),
                    "paddle" => (2, 10),
                    "block" => (1, 6),
                    "rows" => (1, 10),
                    "speed" => (1, 8),
                    _ => (2, 16),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "board" => self.board = n,
                    "paddle" => self.paddle = n,
                    "block" => self.block = n,
                    "rows" => self.rows = n,
                    "speed" => self.speed = n,
                    _ => self.scale = n,
                }
                Ok(json!(n))
            }
            "physics" => int(&mut self.physics, value, (100, 900)),
            "reward_brick" => int(&mut self.reward_brick, value, (-10000, 10000)),
            "reward_death" => int(&mut self.reward_death, value, (-10000, 10000)),
            "reward_step" => int(&mut self.reward_step, value, (-1000, 1000)),
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "board": self.board,
            "paddle": self.paddle,
            "block": self.block,
            "rows": self.rows,
            "physics": self.physics,
            "speed": self.speed,
            "scale": self.scale,
            "reward_brick": self.reward_brick,
            "reward_death": self.reward_death,
            "reward_step": self.reward_step,
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

#[derive(Clone)]
struct Block {
    x: i64,
    y: i64,
    active: bool,
}

pub struct Tennis {
    set: Set,
    rng: Rng,
    seed: u64,
    score: u64,
    steps: u64,
    over: bool,
    dir: Option<usize>,
    bx: i64,
    by: i64,
    bdx: i64,
    bdy: i64,
    px: i64,
    py: i64,
    blocks: Vec<Block>,
    hits: i64,
    threshold: i64,
    ball_color: [u8; 4],
    paddle_color: [u8; 4],
    block_color: [u8; 4],
    board_color: [u8; 4],
}

impl Default for Tennis {
    fn default() -> Tennis {
        Tennis::new()
    }
}

impl Tennis {
    pub fn new() -> Tennis {
        let mut tennis = Tennis {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            score: 0,
            steps: 0,
            over: false,
            dir: None,
            bx: 0,
            by: 0,
            bdx: 0,
            bdy: 0,
            px: 0,
            py: 0,
            blocks: Vec::new(),
            hits: 0,
            threshold: TICK_MIN,
            ball_color: [255, 255, 255, 255],
            paddle_color: [200, 200, 200, 255],
            block_color: [255, 80, 80, 255],
            board_color: [10, 10, 14, 255],
        };
        tennis.reset(0);
        tennis
    }
    fn board(&self) -> i64 {
        self.set.board * MILLI
    }
    fn pw(&self) -> i64 {
        self.set.paddle * MILLI
    }
    fn paddle_min_y(&self) -> i64 {
        self.set.board * 6 / 10 * MILLI
    }
    fn per_row(&self) -> i64 {
        (self.set.board / self.set.block).max(1)
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn full_row(&mut self, y: i64) {
        let bw = self.set.block;
        for pos in 0..self.per_row() {
            self.blocks.push(Block {
                x: pos * bw,
                y,
                active: true,
            });
        }
    }
    fn partial_row(&mut self, y: i64) {
        let per = self.per_row();
        let min_blocks = (per + 1) / 2;
        let extra = self.rng.range(0, per - min_blocks);
        let count = (min_blocks + extra).min(per);
        let mut positions: Vec<i64> = (0..per).collect();
        for i in (1..positions.len()).rev() {
            let j = self.rng.below(i + 1);
            positions.swap(i, j);
        }
        positions.truncate(count as usize);
        positions.sort_unstable();
        let bw = self.set.block;
        for pos in positions {
            self.blocks.push(Block {
                x: pos * bw,
                y,
                active: true,
            });
        }
    }
    fn tick_blocks(&mut self) {
        self.blocks.retain(|b| b.active);
        let mut min_y = 1;
        for b in &mut self.blocks {
            b.y += 1;
            if b.y - 1 < min_y {
                min_y = b.y - 1;
            }
        }
        self.partial_row(min_y - 1);
    }
    fn next_threshold(&mut self) -> i64 {
        let base = (TICK_BASE_MAX - self.score as i64 / TICK_SCORE_DIVISOR).max(TICK_MIN);
        let jitter = self.rng.range(0, 2) - 1;
        (base + jitter).max(TICK_MIN)
    }
    fn move_paddle(&mut self) {
        let step = MILLI;
        match self.dir {
            Some(0) => self.py = (self.py - step).max(self.paddle_min_y()),
            Some(1) => self.py = (self.py + step).min(self.board() - MILLI),
            Some(2) => self.px = (self.px - step).max(0),
            Some(3) => self.px = (self.px + step).min(self.board() - self.pw()),
            _ => {}
        }
    }
    fn collide_blocks(&mut self) -> i64 {
        let (px, py) = (self.bx, self.by);
        let bw = self.set.block * MILLI;
        let mut best: Option<usize> = None;
        let mut best_dist = i64::MAX;
        for (i, b) in self.blocks.iter().enumerate() {
            if !b.active {
                continue;
            }
            let (bxf, byf) = (b.x * MILLI, b.y * MILLI);
            if self.bx < bxf + bw
                && self.bx + MILLI > bxf
                && self.by < byf + MILLI
                && self.by + MILLI > byf
            {
                let cx = px + HALF - (bxf + bw / 2);
                let cy = py + HALF - (byf + HALF);
                let dist = cx * cx + cy * cy;
                if dist < best_dist {
                    best_dist = dist;
                    best = Some(i);
                }
            }
        }
        let Some(i) = best else { return 0 };
        let (bxf, byf) = (self.blocks[i].x * MILLI, self.blocks[i].y * MILLI);
        self.blocks[i].active = false;
        let was_above = py + MILLI <= byf + EDGE;
        let was_below = py >= byf + MILLI - EDGE;
        let was_left = px + MILLI <= bxf + EDGE;
        let was_right = px >= bxf + bw - EDGE;
        if was_above && self.bdy > 0 {
            self.bdy = -self.bdy;
            self.by = byf - MILLI - NUDGE;
        } else if was_below && self.bdy < 0 {
            self.bdy = -self.bdy;
            self.by = byf + MILLI + NUDGE;
        } else if was_left && self.bdx > 0 {
            self.bdx = -self.bdx;
            self.bx = bxf - MILLI - NUDGE;
        } else if was_right && self.bdx < 0 {
            self.bdx = -self.bdx;
            self.bx = bxf + bw + NUDGE;
        } else {
            let center_x = self.bx + HALF;
            let center_y = self.by + HALF;
            let b_cx = bxf + bw / 2;
            let b_cy = byf + HALF;
            let ox = HALF + bw / 2 - (center_x - b_cx).abs();
            let oy = MILLI - (center_y - b_cy).abs();
            if ox < oy {
                self.bdx = -self.bdx;
                self.bx = if center_x < b_cx {
                    bxf - MILLI - NUDGE
                } else {
                    bxf + bw + NUDGE
                };
            } else {
                self.bdy = -self.bdy;
                self.by = if center_y < b_cy {
                    byf - MILLI - NUDGE
                } else {
                    byf + MILLI + NUDGE
                };
            }
        }
        1
    }
    fn ids(&self) -> Tensor {
        let n = self.set.board as usize;
        let mut grid = Tensor::new(vec![n, n]);
        let put = |g: &mut Tensor, x: i64, y: i64, v: u8| {
            if x >= 0 && y >= 0 && (x as usize) < n && (y as usize) < n {
                g.set(&[y as usize, x as usize], v);
            }
        };
        let bw = self.set.block;
        for b in &self.blocks {
            if b.active {
                for dx in 0..bw {
                    put(&mut grid, b.x + dx, b.y, 1);
                }
            }
        }
        let pw = self.set.paddle;
        let pxi = self.px.div_euclid(MILLI);
        let pyi = self.py.div_euclid(MILLI);
        for dx in 0..pw {
            put(&mut grid, pxi + dx, pyi, 2);
        }
        put(
            &mut grid,
            self.bx.div_euclid(MILLI),
            self.by.div_euclid(MILLI),
            3,
        );
        grid
    }
    fn board_facts(&self) -> Vec<Vec<u8>> {
        let ids = self.ids();
        (0..ids.shape[0])
            .map(|r| (0..ids.shape[1]).map(|c| ids.get(&[r, c])).collect())
            .collect()
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.ball_color = self.palette();
        loop {
            self.paddle_color = self.palette();
            if self.paddle_color != self.ball_color {
                break;
            }
        }
        loop {
            self.block_color = self.palette();
            if self.block_color != self.ball_color && self.block_color != self.paddle_color {
                break;
            }
        }
        self.px = (self.set.board - self.set.paddle) / 2 * MILLI;
        self.py = (self.set.board - 1) * MILLI;
        let speed = self.set.physics;
        self.bx = self.px + self.pw() / 2 - HALF;
        self.by = self.py - MILLI;
        let spin = self.rng.below(2 * MILLI as usize + 1) as i64 - MILLI;
        self.bdx = spin * speed / (2 * MILLI);
        self.bdy = -speed;
        self.blocks = Vec::new();
        for r in 0..self.set.rows {
            self.full_row(r);
        }
        self.hits = 0;
        self.score = 0;
        self.steps = 0;
        self.dir = None;
        self.threshold = self.next_threshold();
        self.over = false;
    }
    fn step_once(&mut self) -> bool {
        self.move_paddle();
        let mut hit_paddle = false;
        let speed = self.set.physics;
        let board = self.board();
        self.bx += self.bdx;
        self.by += self.bdy;
        if self.bx <= 0 {
            self.bx = 0;
            self.bdx = self.bdx.abs();
        } else if self.bx + MILLI >= board {
            self.bx = board - MILLI;
            self.bdx = -self.bdx.abs();
        }
        if self.by <= 0 {
            self.by = 0;
            self.bdy = self.bdy.abs();
        }
        if self.by + MILLI >= self.py
            && self.by < self.py + MILLI
            && self.bx + MILLI > self.px
            && self.bx < self.px + self.pw()
            && self.bdy > 0
        {
            let ball_center = self.bx + HALF;
            let paddle_center = self.px + self.pw() / 2;
            let reach = self.pw() / 2;
            let hit = ((ball_center - paddle_center) * MILLI / reach).clamp(-MILLI, MILLI);
            self.bdy = -speed;
            self.bdx = hit * speed / MILLI;
            self.by = self.py - MILLI;
            self.hits += 1;
            hit_paddle = true;
            if self.hits >= self.threshold {
                self.hits = 0;
                self.tick_blocks();
                self.threshold = self.next_threshold();
            }
        }
        self.steps += 1;
        if self.collide_blocks() > 0 {
            self.score += 1;
        }
        if self.by > board {
            self.over = true;
        }
        hit_paddle
    }
    fn advance(&mut self, n: u64) -> (u64, u64) {
        let mut taken = 0;
        let mut hits = 0;
        for _ in 0..n {
            if self.over {
                break;
            }
            if self.step_once() {
                hits += 1;
            }
            taken += 1;
        }
        (taken, hits)
    }
    fn render(&self) -> Frame {
        let s = self.set.scale as usize;
        let side = self.set.board as usize * s;
        let mut frame = Frame::new(side, side, self.board_color);
        let mut sprites = Vec::new();
        let bw = self.set.block as usize;
        for b in &self.blocks {
            if b.active {
                sprites.push(Sprite::new(
                    b.x as f64 * s as f64,
                    b.y as f64 * s as f64,
                    solid_rect(bw * s, s, self.block_color),
                ));
            }
        }
        let unit = MILLI as f64;
        sprites.push(Sprite::new(
            self.px as f64 * s as f64 / unit,
            self.py as f64 * s as f64 / unit,
            solid_rect(self.set.paddle as usize * s, s, self.paddle_color),
        ));
        sprites.push(Sprite::new(
            self.bx as f64 * s as f64 / unit,
            self.by as f64 * s as f64 / unit,
            solid_rect(s, s, self.ball_color),
        ));
        frame.push(Layer::Sprites(sprites));
        frame
    }
    fn blocks_json(&self) -> Json {
        json!(self
            .blocks
            .iter()
            .map(|b| json!([b.x, b.y, b.active]))
            .collect::<Vec<_>>())
    }
    fn blocks_from_json(value: &Json) -> Option<Vec<Block>> {
        let mut out = Vec::new();
        for entry in value.as_array()? {
            let arr = entry.as_array()?;
            if arr.len() != 3 {
                return None;
            }
            let x = arr[0].as_i64()?;
            let y = arr[1].as_i64()?;
            let active = arr[2].as_bool()?;
            out.push(Block { x, y, active });
        }
        Some(out)
    }
}

impl App for Tennis {
    fn route(&self) -> &str {
        "tennis"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("tennis")
            .emoji("🎾")
            .category("games")
            .key("left", Call::new("tennis.move", json!({ "dir": "left" })))
            .key("right", Call::new("tennis.move", json!({ "dir": "right" })))
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "score": self.score,
            "steps": self.steps,
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "dir": self.dir.map(|d| DIRS[d]),
            "board": self.board_facts(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new(
                "tennis.move",
                json!({ "dir": "up | down | left | right" }),
            ));
            out.push(Verb::new("tennis.step", json!({ "n": "int" })));
        }
        out.push(Verb::new("tennis.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "tennis.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "tennis.move" => {
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
                self.dir = Some(dir);
                Outcome::ok(json!({ "dir": DIRS[dir] }))
            }
            "tennis.step" => {
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
                let (taken, hits) = self.advance(n);
                let mut out = Outcome::ok(json!({
                    "steps": taken,
                    "score": self.score,
                    "over": self.over,
                }));
                if hits > 0 {
                    out = out.emit(Effect::new("sound", cue::payload("blip")));
                }
                if self.score > before {
                    out = out.emit(Effect::new("sound", cue::payload("good")));
                }
                if self.over {
                    out = out.emit(Effect::new("sound", cue::payload("lose")));
                }
                out
            }
            "tennis.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "tennis.set" => {
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
            Some(Call::new("tennis.step", json!({ "n": self.set.speed })))
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
            "dir": self.dir.map(|d| DIRS[d]),
            "bx": self.bx,
            "by": self.by,
            "bdx": self.bdx,
            "bdy": self.bdy,
            "px": self.px,
            "py": self.py,
            "blocks": self.blocks_json(),
            "hits": self.hits,
            "threshold": self.threshold,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let (Some(bx), Some(by), Some(bdx), Some(bdy), Some(px), Some(py), Some(blocks)) = (
            state["bx"].as_i64(),
            state["by"].as_i64(),
            state["bdx"].as_i64(),
            state["bdy"].as_i64(),
            state["px"].as_i64(),
            state["py"].as_i64(),
            Tennis::blocks_from_json(&state["blocks"]),
        ) {
            self.bx = bx;
            self.by = by;
            self.bdx = bdx;
            self.bdy = bdy;
            self.px = px;
            self.py = py;
            self.blocks = blocks;
            self.score = state["score"].as_u64().unwrap_or(0);
            self.steps = state["steps"].as_u64().unwrap_or(0);
            self.over = state["over"].as_bool().unwrap_or(false);
            self.hits = state["hits"].as_i64().unwrap_or(0);
            self.threshold = state["threshold"].as_i64().unwrap_or(TICK_MIN);
            self.dir = state["dir"]
                .as_str()
                .and_then(|d| DIRS.iter().position(|&x| x == d));
            if let Some(pos) = state["pos"].as_u64() {
                self.rng.seek(pos as u128);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn tennis(seed: u64) -> Tennis {
        seeded(Tennis::new(), "tennis.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = tennis(42);
        let mut b = tennis(42);
        for t in [&mut a, &mut b] {
            send(t, "tennis.move", json!({ "dir": "up" }));
            send(t, "tennis.step", json!({ "n": 20 }));
            send(t, "tennis.move", json!({ "dir": "down" }));
            send(t, "tennis.step", json!({ "n": 10 }));
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn ball_advances_and_beat_stops_at_over() {
        let mut t = tennis(3);
        let (bx0, by0) = (t.bx, t.by);
        let mut last = send(&mut t, "tennis.step", json!({}));
        assert!((t.bx, t.by) != (bx0, by0));
        assert_eq!(t.beat(), Some(Call::new("tennis.step", json!({ "n": 4 }))));
        let mut rounds = 0;
        while !t.over && rounds < 5000 {
            last = send(&mut t, "tennis.step", json!({ "n": 1 }));
            rounds += 1;
        }
        assert!(t.over, "ball never fell past the board");
        assert!(last
            .effects
            .contains(&Effect::new("sound", cue::payload("lose"))));
        assert_eq!(t.beat(), None);
        assert!(!send(&mut t, "tennis.step", json!({})).ok);
        assert!(!send(&mut t, "tennis.move", json!({ "dir": "up" })).ok);
    }
    #[test]
    fn speed_paces_the_beat() {
        let mut t = tennis(3);
        let out = send(&mut t, "tennis.set", json!({ "key": "speed", "value": 2 }));
        assert!(out.ok);
        assert_eq!(t.state(&iden(), None)["settings"]["speed"], json!(2));
        assert_eq!(t.beat(), Some(Call::new("tennis.step", json!({ "n": 2 }))));
        assert!(!send(&mut t, "tennis.set", json!({ "key": "speed", "value": 0 })).ok);
        assert!(!send(&mut t, "tennis.set", json!({ "key": "speed", "value": 9 })).ok);
    }
    #[test]
    fn physics_tunes_the_ball() {
        let mut t = tennis(3);
        let out = send(
            &mut t,
            "tennis.set",
            json!({ "key": "physics", "value": 500 }),
        );
        assert!(out.ok);
        assert_eq!(t.state(&iden(), None)["settings"]["physics"], json!(500));
        assert_eq!(t.bdy, -500);
        assert!(
            !send(
                &mut t,
                "tennis.set",
                json!({ "key": "physics", "value": 50 })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "tennis.set",
                json!({ "key": "physics", "value": 950 })
            )
            .ok
        );
    }
    #[test]
    fn left_and_right_move_the_paddle_with_wall_clamps() {
        let mut t = tennis(6);
        let start_px = t.px;
        let start_py = t.py;
        send(&mut t, "tennis.move", json!({ "dir": "left" }));
        send(&mut t, "tennis.step", json!({}));
        assert_eq!(t.px, start_px - 1000);
        assert_eq!(t.py, start_py);
        send(&mut t, "tennis.step", json!({ "n": 20 }));
        assert_eq!(t.px, 0);
        send(&mut t, "tennis.move", json!({ "dir": "right" }));
        send(&mut t, "tennis.step", json!({ "n": 30 }));
        assert_eq!(t.px, t.board() - t.pw());
        assert_eq!(t.py, start_py);
        assert!(!t.over);
    }
    #[test]
    fn up_and_down_move_the_paddle_with_clamps() {
        let mut t = tennis(8);
        let start_px = t.px;
        send(&mut t, "tennis.move", json!({ "dir": "up" }));
        send(&mut t, "tennis.step", json!({ "n": 10 }));
        assert_eq!(t.py, t.paddle_min_y());
        assert_eq!(t.px, start_px);
        send(&mut t, "tennis.move", json!({ "dir": "down" }));
        send(&mut t, "tennis.step", json!({ "n": 10 }));
        assert_eq!(t.py, t.board() - 1000);
        assert_eq!(t.px, start_px);
        assert!(!t.over);
    }
    #[test]
    fn step_with_no_prior_move_matches_no_paddle_intent() {
        let mut a = tennis(9);
        let mut b = tennis(9);
        send(&mut a, "tennis.step", json!({ "n": 5 }));
        for _ in 0..5 {
            b.step_once();
        }
        assert_eq!(a.px, b.px);
        assert_eq!(a.py, b.py);
        assert_eq!(a.bx, b.bx);
        assert_eq!(a.by, b.by);
    }
    #[test]
    fn illegal_move_fails_honestly() {
        let mut t = tennis(1);
        assert!(!send(&mut t, "tennis.move", json!({ "dir": "sideways" })).ok);
        assert!(!send(&mut t, "tennis.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut t, "tennis.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut t = Tennis::new();
        let out = t.call(&iden(), &Call::new("tennis.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(t.state(&iden(), None)["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut t = tennis(4);
        send(&mut t, "tennis.step", json!({ "n": 3 }));
        let out = send(&mut t, "tennis.set", json!({ "key": "board", "value": 12 }));
        assert!(out.ok);
        let state = t.state(&iden(), None);
        assert_eq!(state["settings"]["board"], json!(12));
        assert_eq!(state["steps"], json!(0));
        assert!(
            !send(
                &mut t,
                "tennis.set",
                json!({ "key": "board", "value": 999 })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "tennis.set",
                json!({ "key": "speed", "value": "fast" })
            )
            .ok
        );
        assert!(!send(&mut t, "tennis.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = tennis(11);
        send(&mut a, "tennis.move", json!({ "dir": "up" }));
        send(&mut a, "tennis.step", json!({ "n": 4 }));
        let mut b = Tennis::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        for t in [&mut a, &mut b] {
            send(t, "tennis.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Tennis::new();
        t.load(&json!({ "seed": "soup", "blocks": [[9, 9]], "settings": 7 }));
        assert_eq!(t.state(&iden(), None)["steps"], json!(0));
        assert_eq!(t.state(&iden(), None)["seed"], json!(0));
        assert!(!t.blocks.is_empty());
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = tennis(3);
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["tennis.move", "tennis.step", "tennis.reset", "tennis.set"]
        );
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let t = tennis(5);
        let state = t.state(&iden(), None);
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
        assert_eq!(state["board"].as_array().unwrap().len(), 18);
    }
}
