use crate::core::colors::named;
use crate::core::paint::Paint;
use crate::core::rng::Rng;
use crate::core::tensor::Tensor;
use crate::core::tile::{Design, Group, Source, Tile as Model};
use crate::math::life::{counts, entropy, next_grid, Boundary, Sequence};
use crate::math::two::tile as tile2d;
use crate::math::two::Cell2d;
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{field, probe, sample_types};
use serde_json::{json, Value as Json};
use std::collections::VecDeque;

const RING: usize = 512;
const CEILING: usize = 64;
const MASK_MAX: usize = 9;
const PATTERNS: [&str; 6] = ["seed", "clear", "soup", "glider", "pulsar", "pentomino"];
const GLIDER: [(usize, usize); 5] = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
const PENTOMINO: [(usize, usize); 5] = [(0, 1), (0, 2), (1, 0), (1, 1), (2, 1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fate {
    Dead,
    Still,
    Loop,
}

impl Fate {
    fn name(self) -> &'static str {
        match self {
            Fate::Dead => "dead",
            Fate::Still => "still",
            Fate::Loop => "loop",
        }
    }
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

fn built(part: &Part) -> Cell2d {
    match tile2d::build(&part.tile) {
        Ok(cell) if cell.width() > 0 && cell.height() > 0 => cell,
        _ => Cell2d::new(Tensor::full(vec![3, 3], 1)),
    }
}

fn seed_board(part: &Part, size: usize, tiling: usize, padding: usize) -> Cell2d {
    let base = built(part).tile(tiling, tiling).pad(padding, 0);
    Cell2d::new(sample_types(&base, size))
}

fn mask_tensor(part: &Part) -> Tensor {
    let cell = built(part);
    let side = cell.width().max(cell.height()).clamp(1, MASK_MAX);
    let k = if side.is_multiple_of(2) {
        side + 1
    } else {
        side
    };
    let mut t = sample_types(&cell, k);
    t.set(&[k / 2, k / 2], 0);
    t
}

fn stamp(size: usize, cells: &[(usize, usize)]) -> Cell2d {
    let mut t = Tensor::new(vec![size, size]);
    let span_r = cells.iter().map(|c| c.0).max().unwrap_or(0) + 1;
    let span_c = cells.iter().map(|c| c.1).max().unwrap_or(0) + 1;
    let off_r = size.saturating_sub(span_r) / 2;
    let off_c = size.saturating_sub(span_c) / 2;
    for &(r, c) in cells {
        if off_r + r < size && off_c + c < size {
            t.set(&[off_r + r, off_c + c], 1);
        }
    }
    Cell2d::new(t)
}

fn pulsar(size: usize) -> Cell2d {
    let mut t = Tensor::new(vec![size, size]);
    if size < 13 {
        return Cell2d::new(t);
    }
    let off = (size - 13) / 2;
    for &line in &[0usize, 5, 7, 12] {
        for &spot in &[2usize, 3, 4, 8, 9, 10] {
            t.set(&[off + line, off + spot], 1);
            t.set(&[off + spot, off + line], 1);
        }
    }
    Cell2d::new(t)
}

fn soup_board(size: usize, density: i64, seed: u64) -> Cell2d {
    let mut rng = Rng::new(seed);
    let p = density as f64 / 100.0;
    let mut t = Tensor::new(vec![size, size]);
    for y in 0..size {
        for x in 0..size {
            if rng.chance(p) {
                t.set(&[y, x], 1);
            }
        }
    }
    Cell2d::new(t)
}

fn parse_points(value: &Json, size: usize) -> Result<Vec<(usize, usize)>, &'static str> {
    let arr = value
        .as_array()
        .ok_or("points must be an array of [x, y] pairs")?;
    let s = size as i64;
    let mut out = Vec::with_capacity(arr.len());
    for p in arr {
        let pair = p
            .as_array()
            .ok_or("points must be an array of [x, y] pairs")?;
        if pair.len() != 2 {
            return Err("points must be an array of [x, y] pairs");
        }
        let x = pair[0]
            .as_i64()
            .ok_or("points must be an array of [x, y] pairs")?;
        let y = pair[1]
            .as_i64()
            .ok_or("points must be an array of [x, y] pairs")?;
        if !(0..s).contains(&x) || !(0..s).contains(&y) {
            return Err("point out of bounds");
        }
        out.push((x as usize, y as usize));
    }
    Ok(out)
}

fn read_counts(value: &Json, key: &str, fallback: &[usize]) -> Vec<usize> {
    match value[key].as_array() {
        Some(arr) => {
            let mut v: Vec<usize> = arr
                .iter()
                .filter_map(|x| x.as_u64().map(|n| n as usize))
                .collect();
            v.sort_unstable();
            v.dedup();
            v
        }
        None => fallback.to_vec(),
    }
}

struct Set {
    size: i64,
    wrap: bool,
    speed: i64,
    tiling: i64,
    padding: i64,
    density: i64,
    birth: Vec<usize>,
    survive: Vec<usize>,
    zeros: bool,
    ones: bool,
    seed: Part,
    mask: Part,
    fill: String,
    void: String,
}

impl Set {
    fn new() -> Set {
        Set {
            size: 32,
            wrap: true,
            speed: 1,
            tiling: 1,
            padding: 0,
            density: 32,
            birth: vec![3],
            survive: vec![2, 3],
            zeros: false,
            ones: false,
            seed: Part::carpet(),
            mask: Part::carpet(),
            fill: "green".to_string(),
            void: "black".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "size" | "speed" | "tiling" | "padding" | "density" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "size" => (8, 64),
                    "speed" => (1, 32),
                    "tiling" => (1, 8),
                    "padding" => (0, 8),
                    _ => (1, 99),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "size" => self.size = n,
                    "speed" => self.speed = n,
                    "tiling" => self.tiling = n,
                    "padding" => self.padding = n,
                    _ => self.density = n,
                }
                Ok(json!(n))
            }
            "wrap" | "zeros" | "ones" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                match key {
                    "wrap" => self.wrap = on,
                    "zeros" => self.zeros = on,
                    _ => self.ones = on,
                }
                Ok(json!(on))
            }
            "fill" | "void" => {
                let name = value.as_str().ok_or("value must be a string")?;
                named(name).map_err(|_| "unknown color")?;
                match key {
                    "fill" => self.fill = name.to_string(),
                    _ => self.void = name.to_string(),
                }
                Ok(json!(name))
            }
            "seed" | "mask" => {
                let part = Part::from_work(value)?;
                match key {
                    "seed" => self.seed = part,
                    _ => self.mask = part,
                }
                Ok(value.clone())
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "size": self.size,
            "wrap": self.wrap,
            "speed": self.speed,
            "tiling": self.tiling,
            "padding": self.padding,
            "density": self.density,
            "birth": self.birth,
            "survive": self.survive,
            "zeros": self.zeros,
            "ones": self.ones,
            "seed": self.seed.work(),
            "mask": self.mask.work(),
            "fill": self.fill,
            "void": self.void,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let _ = set.apply(key, val);
            }
        }
        set.birth = read_counts(value, "birth", &[3]);
        set.survive = read_counts(value, "survive", &[2, 3]);
        set
    }
}

pub struct Life {
    set: Set,
    mask: Tensor,
    max_neighbors: usize,
    pattern: String,
    timeline: VecDeque<Cell2d>,
    cursor: usize,
    origin: u64,
    running: bool,
    fate: Option<Fate>,
    period: usize,
    rng_seed: u64,
    strip_len: usize,
}

impl Default for Life {
    fn default() -> Life {
        Life::new()
    }
}

impl Life {
    pub fn new() -> Life {
        let set = Set::new();
        let mask = mask_tensor(&set.mask);
        let max_neighbors = mask.sum() as usize;
        let mut life = Life {
            set,
            mask,
            max_neighbors,
            pattern: "glider".to_string(),
            timeline: VecDeque::new(),
            cursor: 0,
            origin: 0,
            running: true,
            fate: None,
            period: 0,
            rng_seed: 0,
            strip_len: 0,
        };
        life.rebuild();
        life
    }
    fn current(&self) -> &Cell2d {
        &self.timeline[self.cursor]
    }
    fn generation(&self) -> u64 {
        self.origin + self.cursor as u64
    }
    fn population(&self) -> usize {
        self.current()
            .types()
            .bytes()
            .iter()
            .filter(|&&b| b == 1)
            .count()
    }
    fn recompute_mask(&mut self) {
        self.mask = mask_tensor(&self.set.mask);
        self.max_neighbors = self.mask.sum() as usize;
        self.clamp_rules();
    }
    fn clamp_rules(&mut self) {
        let max = self.max_neighbors;
        self.set.birth.retain(|&n| n <= max);
        self.set.survive.retain(|&n| n <= max);
    }
    fn build_pattern(&self, pattern: &str) -> Cell2d {
        let size = self.set.size as usize;
        match pattern {
            "seed" => seed_board(
                &self.set.seed,
                size,
                self.set.tiling as usize,
                self.set.padding as usize,
            ),
            "clear" => Cell2d::new(Tensor::new(vec![size, size])),
            "soup" => soup_board(size, self.set.density, self.rng_seed),
            "pulsar" => pulsar(size),
            "pentomino" => stamp(size, &PENTOMINO),
            _ => stamp(size, &GLIDER),
        }
    }
    fn set_board(&mut self, board: Cell2d) {
        self.timeline.clear();
        self.timeline.push_back(board);
        self.cursor = 0;
        self.origin = 0;
        self.fate = None;
        self.period = 0;
        self.strip_len = 0;
    }
    fn rebuild(&mut self) {
        let board = self.build_pattern(&self.pattern.clone());
        self.set_board(board);
    }
    fn reset(&mut self, pattern: &str) -> Result<(), &'static str> {
        if !PATTERNS.contains(&pattern) {
            return Err("no such pattern");
        }
        if pattern == "soup" {
            self.rng_seed = self.rng_seed.wrapping_add(1);
        }
        self.pattern = pattern.to_string();
        self.rebuild();
        Ok(())
    }
    fn fork(&mut self) {
        self.timeline.truncate(self.cursor + 1);
        self.fate = None;
        self.period = 0;
        self.strip_len = 0;
    }
    fn push(&mut self, next: Cell2d) {
        self.timeline.push_back(next);
        if self.timeline.len() > RING {
            self.timeline.pop_front();
            self.origin += 1;
        }
        self.cursor = self.timeline.len() - 1;
    }
    fn advance(&mut self, n: usize) -> usize {
        let boundary = if self.set.wrap {
            Boundary::Wrap
        } else {
            Boundary::Constant
        };
        let mut taken = 0;
        for _ in 0..n {
            if self.fate.is_some() {
                break;
            }
            if self.cursor + 1 < self.timeline.len() {
                self.cursor += 1;
                taken += 1;
                continue;
            }
            let current = self.timeline[self.cursor].clone();
            let next = match next_grid(
                &current,
                &self.set.birth,
                &self.set.survive,
                &self.mask,
                boundary,
            ) {
                Ok(g) => g,
                Err(_) => break,
            };
            if next.types() == current.types() {
                self.fate = Some(if current.types().sum() == 0 {
                    Fate::Dead
                } else {
                    Fate::Still
                });
                self.period = if self.fate == Some(Fate::Still) { 1 } else { 0 };
                break;
            }
            if let Some(j) = self.timeline.iter().position(|f| f.types() == next.types()) {
                self.period = self.timeline.len() - j;
                self.fate = Some(Fate::Loop);
                break;
            }
            self.push(next);
            taken += 1;
        }
        self.strip_len = taken;
        taken
    }
    fn rgba(&self, name: &str) -> [u8; 4] {
        let c = named(name).unwrap_or(crate::core::colors::BLACK);
        [c.r, c.g, c.b, 255]
    }
    fn render_cell(&self, cell: &Cell2d) -> crate::ui::frame::Frame {
        let size = self.set.size as usize;
        let fill = self.rgba(&self.set.fill);
        let void = self.rgba(&self.set.void);
        let colors: Vec<[u8; 4]> = cell
            .types()
            .bytes()
            .iter()
            .map(|&v| if v == 1 { fill } else { void })
            .collect();
        field(size, size, colors, void)
    }
    fn render(&self) -> crate::ui::frame::Frame {
        self.render_cell(self.current())
    }
    fn strip(&self) -> Option<Json> {
        if self.strip_len < 2 {
            return None;
        }
        let win = self.strip_len.min(self.cursor + 1);
        if win < 2 {
            return None;
        }
        let first = self.cursor + 1 - win;
        let cap = 8;
        let picks: Vec<usize> = if win <= cap {
            (first..=self.cursor).collect()
        } else {
            (0..cap)
                .map(|i| first + i * (win - 1) / (cap - 1))
                .collect()
        };
        let frames: Vec<Json> = picks
            .iter()
            .map(|&p| self.render_cell(&self.timeline[p]).fact())
            .collect();
        Some(Json::Array(frames))
    }
    fn saved_rows(&self) -> Vec<String> {
        let size = self.set.size as usize;
        let types = self.current().types();
        (0..size)
            .map(|y| {
                (0..size)
                    .map(|x| if types.get(&[y, x]) == 1 { '1' } else { '0' })
                    .collect()
            })
            .collect()
    }
    fn parse_board(&self, value: &Json) -> Option<Cell2d> {
        let size = self.set.size as usize;
        let rows = value.as_array()?;
        if rows.len() != size {
            return None;
        }
        let mut t = Tensor::new(vec![size, size]);
        for (y, row) in rows.iter().enumerate() {
            let line = row.as_str()?;
            if line.len() != size {
                return None;
            }
            for (x, c) in line.bytes().enumerate() {
                if c == b'1' {
                    t.set(&[y, x], 1);
                }
            }
        }
        Some(Cell2d::new(t))
    }
}

impl App for Life {
    fn route(&self) -> &str {
        "life"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("life").emoji("🧫").category("math")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let mut out = json!({
            "frame": self.render().fact(),
            "generation": self.generation(),
            "population": self.population(),
            "entropy": (entropy(self.current()) * 1000.0).round() / 1000.0,
            "fate": self.fate.map(|f| f.name()),
            "period": self.period,
            "running": self.running,
            "cursor": self.cursor,
            "length": self.timeline.len(),
            "max_neighbors": self.max_neighbors,
            "settings": self.set.to_json(),
        });
        if let Some(strip) = self.strip() {
            out["strip"] = strip;
        }
        out
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("life.step", json!({ "n": "int" })),
            Verb::new("life.run", json!({ "on": "bool" })),
            Verb::new("life.back", json!({})),
            Verb::new("life.start", json!({})),
            Verb::new("life.end", json!({})),
            Verb::new(
                "life.reset",
                json!({ "pattern": "seed | clear | soup | glider | pulsar | pentomino" }),
            ),
            Verb::new("life.set", json!({ "key": "string", "value": "any" })),
            Verb::new(
                "life.rule",
                json!({ "which": "birth | survive", "n": "int", "on": "bool" }),
            ),
            Verb::new(
                "life.fill",
                json!({ "which": "birth | survive", "seq": "string" }),
            ),
            Verb::new("life.paint", json!({ "points": "[[x, y], ...]" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "life.step" => {
                let n = match call.arg("n") {
                    Json::Null => 1,
                    given => match given.as_u64() {
                        Some(n) if (1..=1024).contains(&n) => n,
                        _ => return Outcome::fail("n must be 1 to 1024"),
                    },
                };
                let taken = self.advance(n as usize);
                Outcome::ok(json!({
                    "steps": taken,
                    "generation": self.generation(),
                    "population": self.population(),
                    "fate": self.fate.map(|f| f.name()),
                }))
            }
            "life.run" => match call.arg("on").as_bool() {
                Some(on) => {
                    self.running = on;
                    Outcome::ok(json!({ "running": on }))
                }
                None => Outcome::fail("on must be a bool"),
            },
            "life.back" => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                self.running = false;
                self.strip_len = 0;
                Outcome::ok(json!({ "cursor": self.cursor }))
            }
            "life.start" => {
                self.cursor = 0;
                self.running = false;
                self.strip_len = 0;
                Outcome::ok(json!({ "cursor": self.cursor }))
            }
            "life.end" => {
                self.cursor = self.timeline.len() - 1;
                self.strip_len = 0;
                Outcome::ok(json!({ "cursor": self.cursor }))
            }
            "life.reset" => {
                let pattern = call.arg("pattern").as_str().unwrap_or("");
                match self.reset(pattern) {
                    Ok(()) => {
                        Outcome::ok(json!({ "pattern": pattern, "population": self.population() }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            "life.set" => {
                self.strip_len = 0;
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        match key.as_str() {
                            "seed" => {
                                self.pattern = "seed".to_string();
                                self.rebuild();
                            }
                            "size" | "tiling" | "padding" | "density" => self.rebuild(),
                            "mask" => {
                                self.recompute_mask();
                                self.fork();
                            }
                            "wrap" => self.fork(),
                            _ => {}
                        }
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            "life.rule" => {
                let which = call.arg("which").as_str().unwrap_or("");
                if which != "birth" && which != "survive" {
                    return Outcome::fail("which must be birth or survive");
                }
                let n = match call.arg("n").as_u64() {
                    Some(n) => n as usize,
                    None => return Outcome::fail("n must be an integer"),
                };
                if n > self.max_neighbors {
                    return Outcome::fail("out of range");
                }
                let set = if which == "birth" {
                    &mut self.set.birth
                } else {
                    &mut self.set.survive
                };
                let present = set.contains(&n);
                let want = call.arg("on").as_bool().unwrap_or(!present);
                if want && !present {
                    set.push(n);
                    set.sort_unstable();
                } else if !want && present {
                    set.retain(|&x| x != n);
                }
                self.fork();
                Outcome::ok(json!({ "which": which, "n": n, "on": want }))
            }
            "life.fill" => {
                let which = call.arg("which").as_str().unwrap_or("");
                if which != "birth" && which != "survive" {
                    return Outcome::fail("which must be birth or survive");
                }
                let name = call.arg("seq").as_str().unwrap_or("");
                let seq = match Sequence::parse(name) {
                    Ok(seq) => seq,
                    Err(_) => return Outcome::fail("unknown sequence"),
                };
                let filled = match counts(seq, self.max_neighbors, self.set.zeros, self.set.ones) {
                    Ok(v) => v,
                    Err(_) => return Outcome::fail("could not build sequence"),
                };
                if which == "birth" {
                    self.set.birth = filled.clone();
                } else {
                    self.set.survive = filled.clone();
                }
                self.fork();
                Outcome::ok(json!({ "which": which, "seq": name, "counts": filled }))
            }
            "life.paint" => match parse_points(call.arg("points"), self.set.size as usize) {
                Ok(points) => {
                    self.fork();
                    let board = &mut self.timeline[self.cursor];
                    for &(x, y) in &points {
                        let v = board.types().get(&[y, x]);
                        board.cell.types.set(&[y, x], 1 - v);
                    }
                    Outcome::ok(json!({ "points": points.len() }))
                }
                Err(note) => Outcome::fail(note),
            },
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        if self.running && self.fate.is_none() {
            Some(Call::new("life.step", json!({ "n": self.set.speed })))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "board": self.saved_rows(),
            "pattern": self.pattern,
            "generation": self.generation(),
            "running": self.running,
            "seed": self.rng_seed,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.recompute_mask();
        self.rng_seed = state["seed"].as_u64().unwrap_or(0);
        self.running = state["running"].as_bool().unwrap_or(true);
        let pattern = state["pattern"].as_str().unwrap_or("glider");
        self.pattern = if PATTERNS.contains(&pattern) {
            pattern.to_string()
        } else {
            "glider".to_string()
        };
        match self.parse_board(&state["board"]) {
            Some(board) => {
                self.set_board(board);
                self.origin = state["generation"].as_u64().unwrap_or(0);
            }
            None => self.rebuild(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

    fn net_work() -> Json {
        Part::motif(Design::Net, false).work()
    }

    #[test]
    fn glider_default_keeps_population_and_advances() {
        let mut life = Life::new();
        assert_eq!(life.pattern, "glider");
        assert_eq!(life.population(), 5);
        send(&mut life, "life.step", json!({ "n": 4 }));
        assert_eq!(life.generation(), 4);
        assert_eq!(life.population(), 5);
    }
    #[test]
    fn glider_on_moore_is_b3s23() {
        let life = Life::new();
        assert_eq!(life.max_neighbors, 8);
        assert_eq!(life.set.birth, vec![3]);
        assert_eq!(life.set.survive, vec![2, 3]);
    }
    #[test]
    fn ring_replays_backward_and_forward() {
        let mut life = Life::new();
        send(&mut life, "life.step", json!({ "n": 8 }));
        let frontier = life.state(&iden())["frame"].clone();
        let len = life.timeline.len();
        assert_eq!(life.cursor, len - 1);
        send(&mut life, "life.back", json!({}));
        assert_eq!(life.cursor, len - 2);
        assert!(!life.running);
        send(&mut life, "life.start", json!({}));
        assert_eq!(life.cursor, 0);
        assert_eq!(life.generation(), 0);
        send(&mut life, "life.end", json!({}));
        assert_eq!(life.cursor, len - 1);
        assert_eq!(life.state(&iden())["frame"], frontier);
    }
    #[test]
    fn pulsar_settles_into_a_loop() {
        let mut life = Life::new();
        send(&mut life, "life.reset", json!({ "pattern": "pulsar" }));
        send(&mut life, "life.step", json!({ "n": 16 }));
        assert_eq!(life.state(&iden())["fate"], json!("loop"));
        assert_eq!(life.state(&iden())["period"], json!(3));
        assert_eq!(life.beat(), None);
    }
    #[test]
    fn clear_board_dies() {
        let mut life = Life::new();
        send(&mut life, "life.reset", json!({ "pattern": "clear" }));
        send(&mut life, "life.step", json!({ "n": 3 }));
        assert_eq!(life.state(&iden())["fate"], json!("dead"));
        assert_eq!(life.population(), 0);
    }
    #[test]
    fn a_still_life_reports_still() {
        let mut life = Life::new();
        send(&mut life, "life.reset", json!({ "pattern": "clear" }));
        send(
            &mut life,
            "life.paint",
            json!({ "points": [[10, 10], [11, 10], [10, 11], [11, 11]] }),
        );
        send(&mut life, "life.step", json!({ "n": 2 }));
        assert_eq!(life.state(&iden())["fate"], json!("still"));
        assert_eq!(life.population(), 4);
    }
    #[test]
    fn rule_toggle_changes_the_set_and_forks() {
        let mut life = Life::new();
        send(&mut life, "life.step", json!({ "n": 5 }));
        let out = send(&mut life, "life.rule", json!({ "which": "birth", "n": 2 }));
        assert!(out.ok);
        assert!(life.set.birth.contains(&2));
        assert_eq!(life.cursor + 1, life.timeline.len());
        let out = send(
            &mut life,
            "life.rule",
            json!({ "which": "birth", "n": 2, "on": false }),
        );
        assert!(out.ok);
        assert!(!life.set.birth.contains(&2));
        assert!(!send(&mut life, "life.rule", json!({ "which": "birth", "n": 99 })).ok);
    }
    #[test]
    fn fill_from_sequence_populates_a_set() {
        let mut life = Life::new();
        let out = send(
            &mut life,
            "life.fill",
            json!({ "which": "survive", "seq": "odds" }),
        );
        assert!(out.ok);
        assert_eq!(life.set.survive, vec![3, 5, 7]);
        send(
            &mut life,
            "life.set",
            json!({ "key": "ones", "value": true }),
        );
        send(
            &mut life,
            "life.fill",
            json!({ "which": "survive", "seq": "odds" }),
        );
        assert_eq!(life.set.survive, vec![1, 3, 5, 7]);
        assert!(
            !send(
                &mut life,
                "life.fill",
                json!({ "which": "survive", "seq": "nope" })
            )
            .ok
        );
    }
    #[test]
    fn picked_mask_reshapes_the_rule_space() {
        let mut life = Life::new();
        let out = send(
            &mut life,
            "life.set",
            json!({ "key": "mask", "value": net_work() }),
        );
        assert!(out.ok);
        let net = mask_tensor(&Part::motif(Design::Net, false));
        assert_eq!(life.max_neighbors, net.sum() as usize);
    }
    #[test]
    fn paint_forks_the_timeline_when_scrubbed() {
        let mut life = Life::new();
        send(&mut life, "life.step", json!({ "n": 10 }));
        send(&mut life, "life.back", json!({}));
        send(&mut life, "life.back", json!({}));
        let before = life.population();
        let cursor = life.cursor;
        let out = send(&mut life, "life.paint", json!({ "points": [[0, 0]] }));
        assert!(out.ok);
        assert_eq!(life.cursor, cursor);
        assert_eq!(life.cursor + 1, life.timeline.len());
        assert_eq!(life.population(), before + 1);
        assert_eq!(life.state(&iden())["fate"], Json::Null);
    }
    #[test]
    fn set_size_rebuilds_the_board() {
        let mut life = Life::new();
        send(&mut life, "life.step", json!({ "n": 5 }));
        let out = send(&mut life, "life.set", json!({ "key": "size", "value": 24 }));
        assert!(out.ok);
        assert_eq!(life.generation(), 0);
        assert_eq!(life.timeline.len(), 1);
        assert_eq!(life.current().width(), 24);
        assert!(
            !send(
                &mut life,
                "life.set",
                json!({ "key": "size", "value": 999 })
            )
            .ok
        );
    }
    #[test]
    fn seed_pick_switches_to_the_seed_pattern() {
        let mut life = Life::new();
        let out = send(
            &mut life,
            "life.set",
            json!({ "key": "seed", "value": net_work() }),
        );
        assert!(out.ok);
        assert_eq!(life.pattern, "seed");
        assert_eq!(life.timeline.len(), 1);
    }
    #[test]
    fn soup_reseeds_each_reset() {
        let mut a = Life::new();
        send(&mut a, "life.reset", json!({ "pattern": "soup" }));
        let first = a.saved_rows();
        send(&mut a, "life.reset", json!({ "pattern": "soup" }));
        assert_ne!(a.saved_rows(), first);
        let mut b = Life::new();
        send(&mut b, "life.reset", json!({ "pattern": "soup" }));
        assert_eq!(b.saved_rows(), first);
    }
    #[test]
    fn beat_gated_by_running_and_fate() {
        let mut life = Life::new();
        assert_eq!(life.beat(), Some(Call::new("life.step", json!({ "n": 1 }))));
        send(&mut life, "life.run", json!({ "on": false }));
        assert_eq!(life.beat(), None);
        send(&mut life, "life.run", json!({ "on": true }));
        send(&mut life, "life.set", json!({ "key": "speed", "value": 4 }));
        assert_eq!(life.beat(), Some(Call::new("life.step", json!({ "n": 4 }))));
    }
    #[test]
    fn save_load_roundtrips_and_keeps_stepping() {
        let mut a = Life::new();
        send(&mut a, "life.reset", json!({ "pattern": "soup" }));
        send(
            &mut a,
            "life.fill",
            json!({ "which": "survive", "seq": "odds" }),
        );
        send(&mut a, "life.step", json!({ "n": 6 }));
        send(&mut a, "life.run", json!({ "on": false }));
        let mut b = Life::new();
        b.load(&a.save());
        assert_eq!(b.saved_rows(), a.saved_rows());
        assert_eq!(b.generation(), a.generation());
        assert_eq!(b.set.survive, a.set.survive);
        assert!(!b.running);
        send(&mut a, "life.step", json!({}));
        send(&mut b, "life.step", json!({}));
        assert_eq!(b.saved_rows(), a.saved_rows());
    }
    #[test]
    fn load_survives_garbage() {
        let mut life = Life::new();
        life.load(&json!({ "settings": 7, "board": ["11"], "pattern": "cube" }));
        assert_eq!(life.pattern, "glider");
        assert_eq!(life.generation(), 0);
        assert_eq!(life.population(), 5);
    }
    #[test]
    fn state_publishes_the_explorer_surface() {
        let life = Life::new();
        let state = life.state(&iden());
        assert!(!state["frame"]["palette"].as_array().unwrap().is_empty());
        assert_eq!(state["max_neighbors"], json!(8));
        assert_eq!(state["settings"]["birth"], json!([3]));
        assert_eq!(state["settings"]["seed"]["v"], json!(1));
        assert_eq!(state["length"], json!(1));
    }
    #[test]
    fn unknown_verb_fails() {
        let mut life = Life::new();
        assert!(!send(&mut life, "life.fly", json!({})).ok);
        assert!(!send(&mut life, "life.reset", json!({ "pattern": "cube" })).ok);
    }
    #[test]
    fn multi_step_emits_a_strip_ending_at_the_frame() {
        let mut life = Life::new();
        send(&mut life, "life.step", json!({ "n": 6 }));
        let state = life.state(&iden());
        let strip = state["strip"].as_array().unwrap();
        assert!(strip.len() >= 2 && strip.len() <= 8);
        assert_eq!(strip.last().unwrap(), &state["frame"]);
    }
    #[test]
    fn strip_samples_evenly_and_caps_at_eight() {
        let mut life = Life::new();
        send(&mut life, "life.step", json!({ "n": 32 }));
        assert_eq!(life.generation(), 32);
        let state = life.state(&iden());
        let strip = state["strip"].as_array().unwrap();
        assert_eq!(strip.len(), 8);
        assert_eq!(strip.last().unwrap(), &state["frame"]);
    }
    #[test]
    fn a_settled_or_paused_app_omits_the_strip() {
        let fresh = Life::new();
        assert!(fresh.state(&iden()).get("strip").is_none());
        let mut single = Life::new();
        send(&mut single, "life.step", json!({ "n": 1 }));
        assert!(single.state(&iden()).get("strip").is_none());
        let mut settled = Life::new();
        send(&mut settled, "life.reset", json!({ "pattern": "clear" }));
        send(&mut settled, "life.step", json!({ "n": 4 }));
        assert!(settled.state(&iden()).get("strip").is_none());
    }
    #[test]
    fn scrubbing_back_drops_the_strip() {
        let mut life = Life::new();
        send(&mut life, "life.step", json!({ "n": 6 }));
        assert!(life.state(&iden()).get("strip").is_some());
        send(&mut life, "life.back", json!({}));
        assert!(life.state(&iden()).get("strip").is_none());
    }
}
