use crate::core::rng::Rng;
use crate::core::tensor::Tensor;
use crate::font;
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{self, Frame};
use serde_json::{json, Value as Json};

fn slot(value: &Json) -> Result<[u8; 4], &'static str> {
    let s = value.as_str().ok_or("value must be a string")?;
    if let Some(code) = s.strip_prefix('#') {
        if (code.len() == 6 || code.len() == 8) && code.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(frame::hex_of(s));
        }
        return Err("bad hex");
    }
    let c = crate::core::colors::named(s).map_err(|_| "unknown color")?;
    Ok([c.r, c.g, c.b, c.a])
}

struct Set {
    cols: i64,
    rows: i64,
    speed: i64,
    trail: i64,
    charset: String,
    palette: Vec<[u8; 4]>,
}

impl Set {
    fn new() -> Set {
        Set {
            cols: 32,
            rows: 24,
            speed: 1,
            trail: 8,
            charset: "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            palette: vec![
                frame::hex_of("#32cc58"),
                frame::hex_of("#00d1bb"),
                frame::hex_of("#1ec9f3"),
            ],
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        if let Some(index) = key.strip_prefix("palette.") {
            let i: usize = index.parse().map_err(|_| "bad palette index")?;
            if i >= self.palette.len() {
                return Err("palette index out of range");
            }
            self.palette[i] = slot(value)?;
            return Ok(json!(frame::hex(self.palette[i])));
        }
        match key {
            "cols" | "rows" | "speed" | "trail" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "cols" => (4, 128),
                    "rows" => (4, 96),
                    "speed" => (1, 4),
                    _ => (2, 32),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "cols" => self.cols = n,
                    "rows" => self.rows = n,
                    "speed" => self.speed = n,
                    _ => self.trail = n,
                }
                Ok(json!(n))
            }
            "charset" => {
                let s = value.as_str().ok_or("value must be a string")?;
                self.charset = s.to_string();
                Ok(json!(s))
            }
            "palette" => {
                let parsed: Vec<[u8; 4]> = match value {
                    Json::String(s) => s
                        .split([',', ' '])
                        .filter(|t| !t.is_empty())
                        .map(frame::hex_of)
                        .collect(),
                    Json::Array(arr) => {
                        if !arr.iter().all(|v| v.is_string()) {
                            return Err("value must be hex strings");
                        }
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(frame::hex_of)
                            .collect()
                    }
                    _ => return Err("value must be hex strings"),
                };
                self.palette = parsed.clone();
                Ok(json!(parsed
                    .iter()
                    .map(|c| frame::hex(*c))
                    .collect::<Vec<_>>()))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "cols": self.cols,
            "rows": self.rows,
            "speed": self.speed,
            "trail": self.trail,
            "charset": self.charset,
            "palette": self.palette.iter().map(|c| frame::hex(*c)).collect::<Vec<_>>(),
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

pub struct Matrix {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    play: bool,
    cache: Vec<Tensor>,
    gw: usize,
    gh: usize,
    heads: Vec<i32>,
    colors: Vec<[u8; 4]>,
    ages: Tensor,
    glyphs: Tensor,
    dark: bool,
}

impl Default for Matrix {
    fn default() -> Matrix {
        Matrix::new()
    }
}

impl Matrix {
    pub fn new() -> Matrix {
        let mut matrix = Matrix {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            play: true,
            cache: Vec::new(),
            gw: 1,
            gh: 1,
            heads: Vec::new(),
            colors: Vec::new(),
            ages: Tensor::new(vec![1, 1]),
            glyphs: Tensor::new(vec![1, 1]),
            dark: false,
        };
        matrix.reset(0);
        matrix
    }
    fn build_cache(&mut self) {
        let mut lists: Vec<Vec<Vec<u8>>> = Vec::new();
        for ch in self.set.charset.chars() {
            if let Some(g) = font::glyph(ch) {
                lists.push(font::to_lists(&g));
            }
        }
        self.gh = lists.iter().map(|l| l.len()).max().unwrap_or(1).max(1);
        self.gw = lists
            .iter()
            .flat_map(|l| l.iter().map(|r| r.len()))
            .max()
            .unwrap_or(1)
            .max(1);
        self.cache = lists
            .iter()
            .map(|l| {
                let mut t = Tensor::new(vec![self.gh, self.gw]);
                for (y, row) in l.iter().enumerate() {
                    for (x, &v) in row.iter().enumerate() {
                        if v != 0 {
                            t.set(&[y, x], 1);
                        }
                    }
                }
                t
            })
            .collect();
        if self.cache.is_empty() {
            self.cache.push(Tensor::full(vec![self.gh, self.gw], 1));
        }
    }
    fn column_color(&mut self) -> [u8; 4] {
        if self.set.palette.is_empty() {
            [
                self.rng.range(40, 255) as u8,
                self.rng.range(40, 255) as u8,
                self.rng.range(40, 255) as u8,
                255,
            ]
        } else {
            *self.rng.choice(&self.set.palette.clone())
        }
    }
    fn glyph(&mut self) -> u8 {
        self.rng.below(self.cache.len()) as u8
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.build_cache();
        let cols = self.set.cols as usize;
        let rows = self.set.rows;
        self.ages = Tensor::new(vec![rows as usize, cols]);
        self.glyphs = Tensor::new(vec![rows as usize, cols]);
        self.heads = (0..cols)
            .map(|_| -(self.rng.below(rows as usize) as i32))
            .collect();
        self.colors = (0..cols).map(|_| self.column_color()).collect();
    }
    fn step_once(&mut self) {
        let cols = self.set.cols as usize;
        let rows = self.set.rows as i32;
        let speed = self.set.speed.max(1) as i32;
        let trail = self.set.trail as u8;
        for c in 0..cols {
            for r in 0..rows as usize {
                let age = self.ages.get(&[r, c]);
                if age > 0 {
                    let next = age as i32 + speed;
                    if next > trail as i32 {
                        self.ages.set(&[r, c], 0);
                    } else {
                        self.ages.set(&[r, c], next as u8);
                    }
                }
            }
            self.heads[c] += speed;
            let head = self.heads[c];
            if head >= 0 && head < rows {
                let g = self.glyph();
                self.ages.set(&[head as usize, c], 1);
                self.glyphs.set(&[head as usize, c], g);
            }
            if head >= rows && self.rng.chance(0.025) {
                self.heads[c] = 0;
                self.colors[c] = self.column_color();
            }
        }
    }
    fn advance(&mut self, n: u64) -> u64 {
        for _ in 0..n {
            self.step_once();
        }
        self.steps += n;
        n
    }
    fn render(&self) -> Frame {
        let cols = self.set.cols as usize;
        let rows = self.set.rows as usize;
        let w = cols * self.gw;
        let h = rows * self.gh;
        let trail = self.set.trail as usize;
        let board = crate::ui::frame::board(self.dark);
        let mut colors = vec![board; w * h];
        for r in 0..rows {
            for c in 0..cols {
                let age = self.ages.get(&[r, c]);
                if age == 0 {
                    continue;
                }
                let gi = self.glyphs.get(&[r, c]) as usize % self.cache.len();
                let mask = &self.cache[gi];
                let faded = fade(self.colors[c], age, trail, board);
                for ty in 0..self.gh {
                    for tx in 0..self.gw {
                        if mask.get(&[ty, tx]) == 1 {
                            let x = c * self.gw + tx;
                            let y = r * self.gh + ty;
                            colors[y * w + x] = faded;
                        }
                    }
                }
            }
        }
        frame::field(w, h, colors, board)
    }
}

fn fade(color: [u8; 4], age: u8, trail: usize, bg: [u8; 4]) -> [u8; 4] {
    let t = (age.saturating_sub(1)) as f64 / trail.max(1) as f64;
    let factor = (1.0 - t).clamp(0.0, 1.0);
    frame::mix(bg, color, factor)
}

impl App for Matrix {
    fn route(&self) -> &str {
        "matrix"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("matrix").emoji("🟩").category("toys")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "steps": self.steps,
            "over": false,
            "seed": self.seed,
            "play": self.play,
            "settings": self.set.to_json(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("matrix.step", json!({ "n": "int" })),
            Verb::new("matrix.reset", json!({ "seed": "int" })),
            Verb::new("matrix.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "matrix.step" => {
                let n = match call.arg("n") {
                    Json::Null => 1,
                    given => match given.as_u64() {
                        Some(n) if (1..=1024).contains(&n) => n,
                        _ => return Outcome::fail("n must be 1 to 1024"),
                    },
                };
                let taken = self.advance(n);
                Outcome::ok(json!({ "steps": taken }))
            }
            "matrix.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "matrix.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                if key == "play" {
                    return match call.arg("value").as_bool() {
                        Some(on) => {
                            self.play = on;
                            Outcome::ok(json!({ "key": "play", "value": on }))
                        }
                        None => Outcome::fail("value must be a bool"),
                    };
                }
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        let seed = self.seed;
                        self.reset(seed);
                        self.play = false;
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        if self.play {
            Some(Call::new("matrix.step", json!({})))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "play": self.play,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "heads": self.heads,
            "colors": self.colors,
            "ages": self.ages.bytes(),
            "glyphs": self.glyphs.bytes(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.play = state["play"].as_bool().unwrap_or(true);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let cols = self.set.cols as usize;
        let rows = self.set.rows as usize;
        if let Some(heads) = state["heads"].as_array() {
            let parsed: Option<Vec<i32>> =
                heads.iter().map(|v| v.as_i64().map(|n| n as i32)).collect();
            if let Some(h) = parsed {
                if h.len() == cols {
                    self.heads = h;
                }
            }
        }
        if let Some(colors) = state["colors"].as_array() {
            let parsed: Option<Vec<[u8; 4]>> = colors
                .iter()
                .map(|c| {
                    let a = c.as_array()?;
                    if a.len() != 4 {
                        return None;
                    }
                    let mut out = [0u8; 4];
                    for (i, slot) in out.iter_mut().enumerate() {
                        *slot = a[i].as_u64()? as u8;
                    }
                    Some(out)
                })
                .collect();
            if let Some(c) = parsed {
                if c.len() == cols {
                    self.colors = c;
                }
            }
        }
        if let Some(bytes) = state["ages"].as_array() {
            let parsed: Option<Vec<u8>> =
                bytes.iter().map(|v| v.as_u64().map(|n| n as u8)).collect();
            if let Some(b) = parsed {
                if b.len() == rows * cols {
                    self.ages = Tensor::of(b, vec![rows, cols]);
                }
            }
        }
        if let Some(bytes) = state["glyphs"].as_array() {
            let parsed: Option<Vec<u8>> =
                bytes.iter().map(|v| v.as_u64().map(|n| n as u8)).collect();
            if let Some(b) = parsed {
                if b.len() == rows * cols {
                    self.glyphs = Tensor::of(b, vec![rows, cols]);
                }
            }
        }
        self.steps = state["steps"].as_u64().unwrap_or(0);
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, seeded, send};

    fn matrix(seed: u64) -> Matrix {
        seeded(Matrix::new(), "matrix.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = matrix(5);
        let mut b = matrix(5);
        for s in [&mut a, &mut b] {
            send(s, "matrix.step", json!({ "n": 50 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn ages_stay_in_bounds() {
        let mut m = matrix(3);
        send(&mut m, "matrix.set", json!({ "key": "trail", "value": 6 }));
        send(&mut m, "matrix.step", json!({ "n": 200 }));
        assert!(m.ages.bytes().iter().all(|&a| a <= 6));
    }
    #[test]
    fn step_counts_and_frame_skips() {
        let mut m = matrix(9);
        let out = send(&mut m, "matrix.step", json!({ "n": 5 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(5));
        assert_eq!(m.state(&iden())["steps"], json!(5));
        assert!(!send(&mut m, "matrix.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut m, "matrix.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn set_validates_and_resets() {
        let mut m = matrix(4);
        send(&mut m, "matrix.step", json!({ "n": 3 }));
        let out = send(&mut m, "matrix.set", json!({ "key": "cols", "value": 8 }));
        assert!(out.ok);
        let state = m.state(&iden());
        assert_eq!(state["settings"]["cols"], json!(8));
        assert_eq!(state["steps"], json!(0));
        assert!(!send(&mut m, "matrix.set", json!({ "key": "cols", "value": 999 })).ok);
        assert!(!send(&mut m, "matrix.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn set_pauses_the_animation() {
        let mut m = matrix(4);
        assert_eq!(m.beat(), Some(Call::new("matrix.step", json!({}))));
        send(&mut m, "matrix.set", json!({ "key": "cols", "value": 8 }));
        assert_eq!(m.beat(), None);
        assert_eq!(m.state(&iden())["play"], json!(false));
    }
    #[test]
    fn play_toggles_without_resetting() {
        let mut m = matrix(4);
        send(&mut m, "matrix.step", json!({ "n": 3 }));
        assert!(
            send(
                &mut m,
                "matrix.set",
                json!({ "key": "play", "value": false })
            )
            .ok
        );
        assert_eq!(m.beat(), None);
        assert_eq!(m.state(&iden())["steps"], json!(3));
        assert!(
            send(
                &mut m,
                "matrix.set",
                json!({ "key": "play", "value": true })
            )
            .ok
        );
        assert_eq!(m.beat(), Some(Call::new("matrix.step", json!({}))));
        assert_eq!(m.state(&iden())["steps"], json!(3));
        assert!(
            !send(
                &mut m,
                "matrix.set",
                json!({ "key": "play", "value": "no" })
            )
            .ok
        );
    }
    #[test]
    fn palette_accepts_hex_strings() {
        let mut m = matrix(4);
        let out = send(
            &mut m,
            "matrix.set",
            json!({ "key": "palette", "value": "#ff0000 #00ff00" }),
        );
        assert!(out.ok);
        assert_eq!(
            m.state(&iden())["settings"]["palette"],
            json!(["#ff0000", "#00ff00"])
        );
        let out = send(
            &mut m,
            "matrix.set",
            json!({ "key": "palette", "value": ["#0000ff"] }),
        );
        assert!(out.ok);
        assert_eq!(m.state(&iden())["settings"]["palette"], json!(["#0000ff"]));
        assert!(
            !send(
                &mut m,
                "matrix.set",
                json!({ "key": "palette", "value": 7 })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "matrix.set",
                json!({ "key": "palette", "value": [7] })
            )
            .ok
        );
    }
    #[test]
    fn palette_slots_take_names_and_hex() {
        let mut m = matrix(4);
        let out = send(
            &mut m,
            "matrix.set",
            json!({ "key": "palette.1", "value": "teal" }),
        );
        assert!(out.ok);
        assert_eq!(m.state(&iden())["settings"]["palette"][1], json!("#00cad8"));
        assert!(
            send(
                &mut m,
                "matrix.set",
                json!({ "key": "palette.0", "value": "#ff0000" })
            )
            .ok
        );
        assert_eq!(m.state(&iden())["settings"]["palette"][0], json!("#ff0000"));
        assert!(
            !send(
                &mut m,
                "matrix.set",
                json!({ "key": "palette.9", "value": "teal" })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "matrix.set",
                json!({ "key": "palette.1", "value": "chartreuse" })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "matrix.set",
                json!({ "key": "palette.1", "value": "#zzz" })
            )
            .ok
        );
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = matrix(11);
        send(&mut a, "matrix.step", json!({ "n": 40 }));
        let mut b = Matrix::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for s in [&mut a, &mut b] {
            send(s, "matrix.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut m = Matrix::new();
        m.load(&json!({ "seed": "soup", "heads": "nope", "settings": 7 }));
        assert_eq!(m.state(&iden())["steps"], json!(0));
        assert_eq!(m.state(&iden())["seed"], json!(0));
        let frame = m.state(&iden())["frame"].clone();
        assert!(!frame["rows"].as_array().unwrap().is_empty());
    }
    #[test]
    fn beat_steps_forever() {
        let mut m = matrix(3);
        send(&mut m, "matrix.step", json!({ "n": 500 }));
        assert_eq!(m.beat(), Some(Call::new("matrix.step", json!({}))));
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let m = matrix(5);
        let state = m.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
    }
}
