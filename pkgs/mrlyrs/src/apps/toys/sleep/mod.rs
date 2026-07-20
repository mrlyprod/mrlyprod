use crate::core::rng::Rng;
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{hex, hex_of, solid_rect, Frame, Layer, Sprite};
use serde_json::{json, Value as Json};

fn slot(value: &Json) -> Result<[u8; 4], &'static str> {
    let s = value.as_str().ok_or("value must be a string")?;
    if let Some(code) = s.strip_prefix('#') {
        if (code.len() == 6 || code.len() == 8) && code.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(hex_of(s));
        }
        return Err("bad hex");
    }
    let c = crate::core::colors::named(s).map_err(|_| "unknown color")?;
    Ok([c.r, c.g, c.b, c.a])
}

struct Set {
    cols: i64,
    rows: i64,
    size: i64,
    speed: f64,
    scale: i64,
    palette: Vec<[u8; 4]>,
}

impl Set {
    fn new() -> Set {
        Set {
            cols: 32,
            rows: 24,
            size: 4,
            speed: 0.5,
            scale: 8,
            palette: vec![
                hex_of("#32cc58"),
                hex_of("#00d1bb"),
                hex_of("#1ec9f3"),
                hex_of("#f5a623"),
                hex_of("#ff5d73"),
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
            return Ok(json!(hex(self.palette[i])));
        }
        match key {
            "cols" | "rows" | "size" | "scale" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "cols" => (8, 64),
                    "rows" => (8, 48),
                    "size" => (2, 12),
                    _ => (2, 16),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "cols" => self.cols = n,
                    "rows" => self.rows = n,
                    "size" => self.size = n,
                    _ => self.scale = n,
                }
                Ok(json!(n))
            }
            "speed" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                if !(0.1..=2.0).contains(&n) {
                    return Err("out of range");
                }
                self.speed = n;
                Ok(json!(n))
            }
            "palette" => {
                let parsed: Vec<[u8; 4]> = match value {
                    Json::String(s) => s
                        .split([',', ' '])
                        .filter(|t| !t.is_empty())
                        .map(hex_of)
                        .collect(),
                    Json::Array(arr) => {
                        if !arr.iter().all(|v| v.is_string()) {
                            return Err("value must be hex strings");
                        }
                        arr.iter().filter_map(|v| v.as_str()).map(hex_of).collect()
                    }
                    _ => return Err("value must be hex strings"),
                };
                self.palette = parsed.clone();
                Ok(json!(parsed.iter().map(|c| hex(*c)).collect::<Vec<_>>()))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "cols": self.cols,
            "rows": self.rows,
            "size": self.size,
            "speed": self.speed,
            "scale": self.scale,
            "palette": self.palette.iter().map(|c| hex(*c)).collect::<Vec<_>>(),
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

pub struct Sleep {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: [u8; 4],
    dark: bool,
}

impl Default for Sleep {
    fn default() -> Sleep {
        Sleep::new()
    }
}

impl Sleep {
    pub fn new() -> Sleep {
        let mut sleep = Sleep {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            color: [255, 255, 255, 255],
            dark: false,
        };
        sleep.reset(0);
        sleep
    }
    fn pick(&mut self) -> [u8; 4] {
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
    fn max_x(&self) -> f64 {
        (self.set.cols - self.set.size).max(0) as f64
    }
    fn max_y(&self) -> f64 {
        (self.set.rows - self.set.size).max(0) as f64
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.color = self.pick();
        self.x = self.rng.unit() * self.max_x();
        self.y = self.rng.unit() * self.max_y();
        let speed = self.set.speed;
        self.vx = if self.rng.boolean() { speed } else { -speed };
        self.vy = if self.rng.boolean() { speed } else { -speed };
    }
    fn step_once(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        let (mx, my) = (self.max_x(), self.max_y());
        let mut bounced = false;
        if self.x <= 0.0 {
            self.x = 0.0;
            self.vx = self.vx.abs();
            bounced = true;
        } else if self.x >= mx {
            self.x = mx;
            self.vx = -self.vx.abs();
            bounced = true;
        }
        if self.y <= 0.0 {
            self.y = 0.0;
            self.vy = self.vy.abs();
            bounced = true;
        } else if self.y >= my {
            self.y = my;
            self.vy = -self.vy.abs();
            bounced = true;
        }
        if bounced {
            self.color = self.pick();
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
        let s = self.set.scale as usize;
        let w = self.set.cols as usize * s;
        let h = self.set.rows as usize * s;
        let span = self.set.size as usize * s;
        let mut frame = Frame::new(w, h, crate::ui::frame::board(self.dark));
        frame.push(Layer::Sprites(vec![Sprite::new(
            self.x * s as f64,
            self.y * s as f64,
            solid_rect(span, span, self.color),
        )]));
        frame
    }
}

impl App for Sleep {
    fn route(&self) -> &str {
        "sleep"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("sleep").emoji("💤").category("toys")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "steps": self.steps,
            "over": false,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("sleep.step", json!({ "n": "int" })),
            Verb::new("sleep.reset", json!({ "seed": "int" })),
            Verb::new("sleep.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "sleep.step" => {
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
            "sleep.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "sleep.set" => {
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
        Some(Call::new("sleep.step", json!({})))
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "x": self.x,
            "y": self.y,
            "vx": self.vx,
            "vy": self.vy,
            "color": self.color,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let Some(x) = state["x"].as_f64() {
            self.x = x;
        }
        if let Some(y) = state["y"].as_f64() {
            self.y = y;
        }
        if let Some(vx) = state["vx"].as_f64() {
            self.vx = vx;
        }
        if let Some(vy) = state["vy"].as_f64() {
            self.vy = vy;
        }
        if let Some(arr) = state["color"].as_array() {
            if arr.len() == 4 {
                let mut c = [0u8; 4];
                let mut ok = true;
                for (i, slot) in c.iter_mut().enumerate() {
                    match arr[i].as_u64() {
                        Some(v) => *slot = v as u8,
                        None => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    self.color = c;
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

    fn sleep(seed: u64) -> Sleep {
        seeded(Sleep::new(), "sleep.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = sleep(9);
        let mut b = sleep(9);
        for s in [&mut a, &mut b] {
            send(s, "sleep.step", json!({ "n": 120 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn stays_in_bounds() {
        let mut s = sleep(2);
        send(&mut s, "sleep.set", json!({ "key": "cols", "value": 20 }));
        send(&mut s, "sleep.set", json!({ "key": "rows", "value": 16 }));
        send(&mut s, "sleep.set", json!({ "key": "size", "value": 3 }));
        for _ in 0..500 {
            send(&mut s, "sleep.step", json!({}));
            assert!(s.x >= 0.0 && s.x <= (20 - 3) as f64 + 1e-6);
            assert!(s.y >= 0.0 && s.y <= (16 - 3) as f64 + 1e-6);
        }
    }
    #[test]
    fn step_counts_and_frame_skips() {
        let mut s = sleep(9);
        let out = send(&mut s, "sleep.step", json!({ "n": 5 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(5));
        assert_eq!(s.state(&iden())["steps"], json!(5));
        assert!(!send(&mut s, "sleep.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut s, "sleep.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn set_validates_and_resets() {
        let mut s = sleep(4);
        send(&mut s, "sleep.step", json!({ "n": 3 }));
        let out = send(&mut s, "sleep.set", json!({ "key": "cols", "value": 20 }));
        assert!(out.ok);
        let state = s.state(&iden());
        assert_eq!(state["settings"]["cols"], json!(20));
        assert_eq!(state["steps"], json!(0));
        assert!(!send(&mut s, "sleep.set", json!({ "key": "cols", "value": 999 })).ok);
        assert!(
            !send(
                &mut s,
                "sleep.set",
                json!({ "key": "speed", "value": "fast" })
            )
            .ok
        );
        assert!(!send(&mut s, "sleep.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn palette_accepts_hex_strings() {
        let mut s = sleep(4);
        let out = send(
            &mut s,
            "sleep.set",
            json!({ "key": "palette", "value": "#ff0000, #00ff00" }),
        );
        assert!(out.ok);
        assert_eq!(
            s.state(&iden())["settings"]["palette"],
            json!(["#ff0000", "#00ff00"])
        );
        let out = send(
            &mut s,
            "sleep.set",
            json!({ "key": "palette", "value": ["#0000ff"] }),
        );
        assert!(out.ok);
        assert_eq!(s.state(&iden())["settings"]["palette"], json!(["#0000ff"]));
        assert!(!send(&mut s, "sleep.set", json!({ "key": "palette", "value": 7 })).ok);
        assert!(
            !send(
                &mut s,
                "sleep.set",
                json!({ "key": "palette", "value": [7] })
            )
            .ok
        );
    }
    #[test]
    fn palette_slots_take_names_and_hex() {
        let mut s = sleep(4);
        assert!(
            send(
                &mut s,
                "sleep.set",
                json!({ "key": "palette.1", "value": "teal" })
            )
            .ok
        );
        assert_eq!(s.state(&iden())["settings"]["palette"][1], json!("#00cad8"));
        assert!(
            send(
                &mut s,
                "sleep.set",
                json!({ "key": "palette.0", "value": "#ff0000" })
            )
            .ok
        );
        assert!(
            !send(
                &mut s,
                "sleep.set",
                json!({ "key": "palette.9", "value": "teal" })
            )
            .ok
        );
        assert!(
            !send(
                &mut s,
                "sleep.set",
                json!({ "key": "palette.1", "value": "chartreuse" })
            )
            .ok
        );
        assert!(
            !send(
                &mut s,
                "sleep.set",
                json!({ "key": "palette.1", "value": "#zzz" })
            )
            .ok
        );
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = sleep(11);
        send(&mut a, "sleep.step", json!({ "n": 40 }));
        let mut b = Sleep::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for s in [&mut a, &mut b] {
            send(s, "sleep.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut s = Sleep::new();
        s.load(&json!({ "seed": "soup", "x": "nope", "settings": 7 }));
        assert_eq!(s.state(&iden())["steps"], json!(0));
        assert_eq!(s.state(&iden())["seed"], json!(0));
        let frame = s.state(&iden())["frame"].clone();
        assert!(!frame["rows"].as_array().unwrap().is_empty());
    }
    #[test]
    fn beat_steps_forever() {
        let mut s = sleep(3);
        send(&mut s, "sleep.step", json!({ "n": 500 }));
        assert_eq!(s.beat(), Some(Call::new("sleep.step", json!({}))));
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let s = sleep(5);
        let state = s.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
    }
}
