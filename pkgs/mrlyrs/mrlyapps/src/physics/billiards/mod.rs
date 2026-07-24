use mrlycore::colors::{named, YELLOW};
use mrlycore::rng::Rng;
use mrlymath::physics::{Billiards as Sim, BilliardsConfig, Mask, Particle};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{self, Frame};
use mrlyui::shaders;
use serde_json::{json, Value as Json};

const WALL: [u8; 4] = [90, 90, 94, 255];
const DESIGNS: [&str; 5] = ["carpet", "net", "htree", "vtree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MASK_KEYS: [&str; 6] = ["design", "number", "level", "padding", "invert", "subpixel"];
const SUBPIXELS: [i64; 3] = [1, 2, 4];

fn too_many_cells(number: i64, level: i64, subpixel: i64) -> bool {
    match (number as u32).checked_pow(level as u32) {
        Some(cells) => (cells as i64) * subpixel > 256,
        None => true,
    }
}

struct Set {
    design: String,
    number: i64,
    level: i64,
    padding: i64,
    invert: bool,
    subpixel: i64,
    speed: f64,
    trail: f64,
    size: f64,
    count: i64,
    accent: String,
}

impl Set {
    fn new() -> Set {
        Set {
            design: "carpet".to_string(),
            number: 5,
            level: 2,
            padding: 8,
            invert: false,
            subpixel: 2,
            speed: 1.0,
            trail: 0.1,
            size: 1.5,
            count: 16,
            accent: "yellow".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "design" => {
                let d = value.as_str().ok_or("value must be a string")?;
                if !DESIGNS.contains(&d) {
                    return Err("no such option");
                }
                self.design = d.to_string();
                Ok(json!(d))
            }
            "number" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !NUMBERS.contains(&n) {
                    return Err("no such option");
                }
                if too_many_cells(n, self.level, self.subpixel) {
                    return Err("too many cells");
                }
                self.number = n;
                Ok(json!(n))
            }
            "level" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if n < 1 {
                    return Err("out of range");
                }
                if too_many_cells(self.number, n, self.subpixel) {
                    return Err("too many cells");
                }
                self.level = n;
                Ok(json!(n))
            }
            "padding" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !(0..=48).contains(&n) {
                    return Err("out of range");
                }
                self.padding = n;
                Ok(json!(n))
            }
            "invert" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                self.invert = on;
                Ok(json!(on))
            }
            "count" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !(1..=128).contains(&n) {
                    return Err("out of range");
                }
                self.count = n;
                Ok(json!(n))
            }
            "subpixel" => {
                let n = value
                    .as_i64()
                    .or_else(|| value.as_str().and_then(|s| s.parse::<i64>().ok()))
                    .ok_or("value must be an integer")?;
                if !SUBPIXELS.contains(&n) {
                    return Err("no such option");
                }
                if too_many_cells(self.number, self.level, n) {
                    return Err("too many cells");
                }
                self.subpixel = n;
                Ok(json!(n))
            }
            "accent" => {
                let name = value.as_str().ok_or("value must be a string")?;
                named(name).map_err(|_| "unknown color")?;
                self.accent = name.to_string();
                Ok(json!(name))
            }
            "speed" | "trail" | "size" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
                    "speed" => (0.5, 4.0),
                    "trail" => (0.02, 1.0),
                    _ => (1.0, 4.0),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "speed" => self.speed = n,
                    "trail" => self.trail = n,
                    _ => self.size = n,
                }
                Ok(json!(n))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "design": self.design,
            "number": self.number,
            "level": self.level,
            "padding": self.padding,
            "invert": self.invert,
            "subpixel": self.subpixel,
            "speed": self.speed,
            "trail": self.trail,
            "size": self.size,
            "count": self.count,
            "accent": self.accent,
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
    fn mask(&self) -> Mask {
        Mask::build(
            &self.design,
            self.number as usize,
            self.level as usize,
            self.padding as usize,
            self.subpixel as usize,
            self.invert,
        )
        .unwrap()
    }
    fn config(&self) -> BilliardsConfig {
        BilliardsConfig {
            speed: self.speed as f32,
            trail: self.trail as f32,
            size: self.size as f32,
            count: self.count as usize,
        }
    }
}

pub struct Billiards {
    set: Set,
    sim: Sim,
    play: bool,
    rng: Rng,
    seed: u64,
    intensity: Vec<f32>,
    dark: bool,
}

impl Default for Billiards {
    fn default() -> Billiards {
        Billiards::new()
    }
}

impl Billiards {
    pub fn new() -> Billiards {
        let set = Set::new();
        let sim = Sim::new(set.mask(), set.config(), 0);
        let mut billiards = Billiards {
            set,
            sim,
            play: true,
            rng: Rng::new(0),
            seed: 0,
            intensity: Vec::new(),
            dark: false,
        };
        billiards.reset(0);
        billiards
    }
    fn rebuild(&mut self) {
        let mask = self.set.mask();
        let (w, h) = (mask.width(), mask.height());
        self.sim = Sim::new(mask, self.set.config(), self.seed);
        self.intensity = vec![0.0; w * h];
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.rebuild();
    }
    fn stamp(&mut self, x: f32, y: f32) {
        let mask = self.sim.mask();
        let (w, h) = (mask.width() as i64, mask.height() as i64);
        let r = self.set.size as f32;
        let ri = r.ceil() as i64;
        let cx = x.floor() as i64;
        let cy = y.floor() as i64;
        for dy in -ri..=ri {
            for dx in -ri..=ri {
                let px = cx + dx;
                let py = cy + dy;
                if px < 0 || py < 0 || px >= w || py >= h {
                    continue;
                }
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist <= r {
                    let i = (py as usize) * (w as usize) + px as usize;
                    self.intensity[i] = 1.0;
                }
            }
        }
    }
    fn render(&self) -> Frame {
        let mask = self.sim.mask();
        let w = mask.width();
        let h = mask.height();
        let c = named(&self.set.accent).unwrap_or(YELLOW);
        let accent = [c.r, c.g, c.b, c.a];
        let board = mrlyui::frame::board(self.dark);
        let mut colors = vec![board; w * h];
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if mask.solid(x as f32, y as f32) {
                    colors[i] = WALL;
                    continue;
                }
                let t = self.intensity[i].clamp(0.0, 1.0) as f64;
                colors[i] = frame::mix(board, accent, t);
            }
        }
        frame::field(w, h, colors, board)
    }
    fn shade(&self) -> Json {
        let mask = self.sim.mask();
        let p = shaders::linear(mrlyui::frame::board(self.dark));
        let c = named(&self.set.accent).unwrap_or(YELLOW);
        let a = shaders::linear([c.r, c.g, c.b, c.a]);
        let mut u = vec![0.0; 16];
        u[4] = p[0];
        u[5] = p[1];
        u[6] = p[2];
        u[8] = a[0];
        u[9] = a[1];
        u[10] = a[2];
        u[12] = mask.width() as f64;
        u[13] = mask.height() as f64;
        json!({ "program": "billiards", "uniforms": u })
    }
    fn count(call: &Call, max: i64) -> Result<i64, &'static str> {
        match call.arg("n") {
            Json::Null => Ok(1),
            given => match given.as_i64() {
                Some(n) if (1..=max).contains(&n) => Ok(n),
                _ => Err("n out of range"),
            },
        }
    }
}

impl App for Billiards {
    fn route(&self) -> &str {
        "billiards"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("billiards").emoji("🎱").category("physics")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        let mask = self.sim.mask();
        json!({
            "settings": self.set.to_json(),
            "play": self.play,
            "seed": self.seed,
            "particles": self.sim.particles().len(),
            "grid": { "width": mask.width(), "height": mask.height() },
            "frame": self.render().fact(),
            "shade": self.shade(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("billiards.break", json!({ "x": "int", "y": "int" })),
            Verb::new("billiards.step", json!({ "n": "int" })),
            Verb::new("billiards.reset", json!({ "seed": "int" })),
            Verb::new("billiards.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "billiards.break" => {
                let (Some(x), Some(y)) = (call.arg("x").as_i64(), call.arg("y").as_i64()) else {
                    return Outcome::fail("x and y must be integers");
                };
                if self.sim.mask().solid(x as f32, y as f32) {
                    return Outcome::fail("that cell is wall");
                }
                self.sim.spawn(x as f32, y as f32);
                Outcome::ok(json!({ "x": x, "y": y, "particles": self.sim.particles().len() }))
            }
            "billiards.step" => {
                let n = match Billiards::count(call, 64) {
                    Ok(n) => n,
                    Err(note) => return Outcome::fail(note),
                };
                for _ in 0..n {
                    let trail = self.set.trail as f32;
                    for v in self.intensity.iter_mut() {
                        *v *= 1.0 - trail;
                    }
                    self.sim.step(1.0);
                    let positions: Vec<(f32, f32)> =
                        self.sim.particles().iter().map(|p| (p.x, p.y)).collect();
                    for (x, y) in positions {
                        self.stamp(x, y);
                    }
                }
                Outcome::ok(json!({ "particles": self.sim.particles().len() }))
            }
            "billiards.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "billiards.set" => {
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
                        if MASK_KEYS.contains(&key.as_str()) {
                            self.rebuild();
                        } else {
                            self.sim.set_config(self.set.config());
                        }
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
            Some(Call::new("billiards.step", json!({})))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        let particles: Vec<Json> = self
            .sim
            .particles()
            .iter()
            .map(|p| json!({ "x": p.x, "y": p.y, "vx": p.vx, "vy": p.vy }))
            .collect();
        json!({
            "settings": self.set.to_json(),
            "play": self.play,
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "particles": particles,
            "trail": self.intensity,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        if let Some(on) = state["play"].as_bool() {
            self.play = on;
        }
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
        if let Some(arr) = state["particles"].as_array() {
            let particles: Option<Vec<Particle>> = arr
                .iter()
                .map(|p| {
                    Some(Particle {
                        x: p["x"].as_f64()? as f32,
                        y: p["y"].as_f64()? as f32,
                        vx: p["vx"].as_f64()? as f32,
                        vy: p["vy"].as_f64()? as f32,
                    })
                })
                .collect();
            if let Some(particles) = particles {
                self.sim.load_particles(particles);
            }
        }
        if let Some(arr) = state["trail"].as_array() {
            if arr.len() == self.intensity.len() {
                let values: Option<Vec<f32>> =
                    arr.iter().map(|v| v.as_f64().map(|f| f as f32)).collect();
                if let Some(values) = values {
                    self.intensity = values;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn billiards(seed: u64) -> Billiards {
        seeded(Billiards::new(), "billiards.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = billiards(123);
        let mut b = billiards(123);
        for x in [&mut a, &mut b] {
            send(x, "billiards.break", json!({ "x": 20, "y": 20 }));
            send(x, "billiards.step", json!({ "n": 5 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn save_load_continues() {
        let mut a = billiards(11);
        send(&mut a, "billiards.break", json!({ "x": 20, "y": 20 }));
        send(&mut a, "billiards.step", json!({ "n": 4 }));
        let mut b = Billiards::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for x in [&mut a, &mut b] {
            send(x, "billiards.step", json!({ "n": 4 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut b = Billiards::new();
        b.load(&json!({ "seed": "soup", "settings": 7, "particles": "nope" }));
        assert_eq!(b.state(&iden())["seed"], json!(0));
        assert_eq!(b.state(&iden())["particles"], json!(0));
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut b = Billiards::new();
        let out = b.act(&iden(), &Call::new("billiards.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let b = billiards(3);
        let names: Vec<String> = b.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "billiards.break",
                "billiards.step",
                "billiards.reset",
                "billiards.set"
            ]
        );
    }
    #[test]
    fn break_fails_on_wall() {
        let mut b = billiards(1);
        assert!(!send(&mut b, "billiards.break", json!({ "x": -1, "y": 0 })).ok);
        assert!(!send(&mut b, "billiards.break", json!({ "x": "nope", "y": 0 })).ok);
    }
    #[test]
    fn break_fans_count_particles() {
        let mut b = billiards(1);
        send(
            &mut b,
            "billiards.set",
            json!({ "key": "count", "value": 4 }),
        );
        assert!(send(&mut b, "billiards.break", json!({ "x": 20, "y": 20 })).ok);
        assert_eq!(b.state(&iden())["particles"], json!(4));
    }
    #[test]
    fn set_validates_mask_and_config_keys() {
        let mut b = billiards(1);
        assert!(
            send(
                &mut b,
                "billiards.set",
                json!({ "key": "number", "value": 3 })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "number", "value": 4 })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "design", "value": "spiral" })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "volume", "value": 1 })
            )
            .ok
        );
        assert!(
            send(
                &mut b,
                "billiards.set",
                json!({ "key": "count", "value": 32 })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "count", "value": 999 })
            )
            .ok
        );
        assert!(
            send(
                &mut b,
                "billiards.set",
                json!({ "key": "play", "value": false })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "play", "value": "no" })
            )
            .ok
        );
    }
    #[test]
    fn accent_takes_any_named_color() {
        let mut b = billiards(1);
        for name in mrlycore::colors::NAMES {
            assert!(
                send(
                    &mut b,
                    "billiards.set",
                    json!({ "key": "accent", "value": name })
                )
                .ok
            );
        }
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "accent", "value": "chartreuse" })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "accent", "value": 7 })
            )
            .ok
        );
    }
    #[test]
    fn subpixel_validates_and_rebuilds() {
        let mut b = billiards(1);
        send(&mut b, "billiards.break", json!({ "x": 20, "y": 20 }));
        assert!(b.state(&iden())["particles"].as_u64().unwrap() > 0);
        assert!(
            send(
                &mut b,
                "billiards.set",
                json!({ "key": "subpixel", "value": 4 })
            )
            .ok
        );
        assert_eq!(b.state(&iden())["particles"], json!(0));
        assert_eq!(b.state(&iden())["settings"]["subpixel"], json!(4));
        assert!(
            send(
                &mut b,
                "billiards.set",
                json!({ "key": "subpixel", "value": "1" })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "subpixel", "value": 3 })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "billiards.set",
                json!({ "key": "subpixel", "value": "loud" })
            )
            .ok
        );
    }
    #[test]
    fn accent_and_subpixel_round_trip() {
        let mut a = billiards(2);
        send(
            &mut a,
            "billiards.set",
            json!({ "key": "accent", "value": "teal" }),
        );
        send(
            &mut a,
            "billiards.set",
            json!({ "key": "subpixel", "value": 4 }),
        );
        let mut b = Billiards::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn mask_key_change_rebuilds_and_clears_particles() {
        let mut b = billiards(1);
        send(&mut b, "billiards.break", json!({ "x": 20, "y": 20 }));
        assert!(b.state(&iden())["particles"].as_u64().unwrap() > 0);
        send(
            &mut b,
            "billiards.set",
            json!({ "key": "padding", "value": 4 }),
        );
        assert_eq!(b.state(&iden())["particles"], json!(0));
        assert_eq!(b.state(&iden())["settings"]["padding"], json!(4));
    }
    #[test]
    fn beat_gates_on_play() {
        let mut b = billiards(1);
        assert_eq!(b.beat(), Some(Call::new("billiards.step", json!({}))));
        send(
            &mut b,
            "billiards.set",
            json!({ "key": "play", "value": false }),
        );
        assert_eq!(b.beat(), None);
    }
    #[test]
    fn step_out_of_range_fails() {
        let mut b = billiards(1);
        assert!(!send(&mut b, "billiards.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut b, "billiards.step", json!({ "n": 65 })).ok);
    }
}
