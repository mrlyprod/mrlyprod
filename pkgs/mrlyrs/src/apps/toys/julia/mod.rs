use crate::core::rng::Rng;
use crate::math::fractal::{self, presets, Viewport, Wayfinder};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{self, Frame};
use serde_json::{json, Value as Json};

const PRESETS: [&str; 7] = [
    "-0.4+0.6i",
    "-0.8+0.156i",
    "0.285+0.01i",
    "-0.727+0.189i",
    "-0.1+0.651i",
    "0.355+0.355i",
    "custom",
];

struct Set {
    width: i64,
    height: i64,
    depth: i64,
    preset: String,
    cre: f64,
    cim: f64,
    zoom: f64,
    cycle: i64,
    start: f64,
    band: f64,
    drift: f64,
    fade: i64,
    spin: f64,
    primary: [u8; 4],
    accent: [u8; 4],
}

impl Set {
    fn new() -> Set {
        Set {
            width: 100,
            height: 100,
            depth: 96,
            preset: "-0.4+0.6i".to_string(),
            cre: -0.4,
            cim: 0.6,
            zoom: 1.012,
            cycle: 240,
            start: 0.7,
            band: 10.0,
            drift: 0.4,
            fade: 24,
            spin: 0.0,
            primary: frame::hex_of("#000000"),
            accent: frame::hex_of("#ff5db1"),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "width" | "height" | "depth" | "cycle" | "fade" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "width" => (16, 512),
                    "height" => (16, 512),
                    "depth" => (16, 600),
                    "cycle" => (30, 3000),
                    _ => (0, 240),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "width" => self.width = n,
                    "height" => self.height = n,
                    "depth" => self.depth = n,
                    "cycle" => self.cycle = n,
                    _ => self.fade = n,
                }
                Ok(json!(n))
            }
            "cre" | "cim" | "zoom" | "start" | "band" | "drift" | "spin" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
                    "cre" => (-2.0, 2.0),
                    "cim" => (-2.0, 2.0),
                    "zoom" => (1.0, 1.05),
                    "start" => (0.25, 4.0),
                    "band" => (2.0, 64.0),
                    "drift" => (0.0, 4.0),
                    _ => (0.0, 0.05),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "cre" => self.cre = n,
                    "cim" => self.cim = n,
                    "zoom" => self.zoom = n,
                    "start" => self.start = n,
                    "band" => self.band = n,
                    "drift" => self.drift = n,
                    _ => self.spin = n,
                }
                Ok(json!(n))
            }
            "preset" => {
                let p = value.as_str().ok_or("value must be a string")?;
                if !PRESETS.contains(&p) {
                    return Err("no such option");
                }
                self.preset = p.to_string();
                Ok(json!(p))
            }
            "primary" | "accent" => {
                let s = value.as_str().ok_or("value must be a hex string")?;
                let c = frame::hex_of(s);
                match key {
                    "primary" => self.primary = c,
                    _ => self.accent = c,
                }
                Ok(json!(frame::hex(c)))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "width": self.width,
            "height": self.height,
            "depth": self.depth,
            "preset": self.preset,
            "cre": self.cre,
            "cim": self.cim,
            "zoom": self.zoom,
            "cycle": self.cycle,
            "start": self.start,
            "band": self.band,
            "drift": self.drift,
            "fade": self.fade,
            "spin": self.spin,
            "primary": frame::hex(self.primary),
            "accent": frame::hex(self.accent),
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

pub struct Julia {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    c: (f64, f64),
    start: Viewport,
    target: (f64, f64),
    zoom: f64,
    age: usize,
    phase: f64,
    rotation: f64,
    iters: Vec<i64>,
    gpu: bool,
}

impl Default for Julia {
    fn default() -> Julia {
        Julia::new()
    }
}

impl Julia {
    pub fn new() -> Julia {
        let mut julia = Julia {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            c: (0.0, 0.0),
            start: fractal::JULIA,
            target: (0.0, 0.0),
            zoom: 1.0,
            age: 0,
            phase: 0.0,
            rotation: 0.0,
            iters: Vec::new(),
            gpu: false,
        };
        julia.reset(0);
        julia
    }
    fn resolve_c(&self) -> (f64, f64) {
        if self.set.preset == "custom" {
            (self.set.cre, self.set.cim)
        } else {
            presets::preset(&self.set.preset)
                .map(|p| (p.re, p.im))
                .unwrap_or((self.set.cre, self.set.cim))
        }
    }
    fn begin(&mut self) {
        let (cx, cy) = fractal::JULIA.center();
        let expand = 1.0 / self.set.start.max(0.01);
        let hw = (fractal::JULIA.xmax - fractal::JULIA.xmin) * 0.5 * expand;
        let hh = (fractal::JULIA.ymax - fractal::JULIA.ymin) * 0.5 * expand;
        self.start =
            Viewport::around(cx, cy, hw, hh).fit(self.set.width as usize, self.set.height as usize);
        let wf = Wayfinder::Julia {
            cr: self.c.0,
            ci: self.c.1,
        };
        self.target = wf.pick(&self.start, &mut self.rng);
        self.zoom = 1.0;
        self.age = 0;
        self.rotation = 0.0;
        self.fill();
    }
    fn view(&self) -> Viewport {
        let hw = (self.start.xmax - self.start.xmin) * 0.5 / self.zoom;
        let hh = (self.start.ymax - self.start.ymin) * 0.5 / self.zoom;
        Viewport::around(self.target.0, self.target.1, hw, hh)
    }
    fn fill(&mut self) {
        let w = self.set.width as usize;
        let h = self.set.height as usize;
        let depth = self.set.depth;
        let (cr, ci) = self.c;
        let v = self.view();
        let center = v.center();
        let vw = v.xmax - v.xmin;
        let vh = v.ymax - v.ymin;
        self.iters = vec![0; w * h];
        for py in 0..h {
            let uy = (py as f64 + 0.5) / h as f64;
            for px in 0..w {
                let ux = (px as f64 + 0.5) / w as f64;
                let zr = v.xmin + ux * vw;
                let zi = v.ymax - uy * vh;
                let (zr, zi) = fractal::rotate(zr, zi, center, self.rotation);
                self.iters[py * w + px] = fractal::julia(zr, zi, cr, ci, depth);
            }
        }
    }
    fn fade(&self) -> f64 {
        let fade = self.set.fade;
        if fade == 0 {
            return 1.0;
        }
        let fade = fade as f64;
        let fin = (self.age as f64 / fade).min(1.0);
        let fout = ((self.set.cycle as usize).saturating_sub(self.age) as f64 / fade).min(1.0);
        fin.min(fout).max(0.0)
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.phase = 0.0;
        self.c = self.resolve_c();
        self.begin();
    }
    fn step_once(&mut self) {
        self.age += 1;
        self.zoom *= self.set.zoom.max(1.0);
        self.phase += self.set.drift;
        if self.set.spin != 0.0 {
            self.rotation += self.set.spin;
        }
        if self.age >= self.set.cycle as usize {
            self.begin();
        } else {
            self.fill();
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
        let w = self.set.width as usize;
        let h = self.set.height as usize;
        let depth = self.set.depth;
        let primary = self.set.primary;
        let accent = self.set.accent;
        let f = self.fade();
        let mut colors = vec![primary; w * h];
        for (slot, &it) in colors.iter_mut().zip(self.iters.iter()) {
            let c = fractal::shade(it, depth, self.phase, self.set.band, primary, accent);
            *slot = if f < 1.0 {
                frame::mix(primary, c, f)
            } else {
                c
            };
        }
        frame::field(w, h, colors, primary)
    }
    fn shade(&self) -> Json {
        let v = self.view();
        let p = self.set.primary;
        let a = self.set.accent;
        let mut u = vec![0.0; 24];
        u[2] = self.phase;
        u[4] = p[0] as f64 / 255.0;
        u[5] = p[1] as f64 / 255.0;
        u[6] = p[2] as f64 / 255.0;
        u[8] = a[0] as f64 / 255.0;
        u[9] = a[1] as f64 / 255.0;
        u[10] = a[2] as f64 / 255.0;
        u[11] = self.rotation;
        u[12] = v.xmin;
        u[13] = v.xmax;
        u[14] = v.ymin;
        u[15] = v.ymax;
        u[16] = self.c.0;
        u[17] = self.c.1;
        u[18] = self.set.depth as f64;
        u[19] = self.set.band;
        u[20] = self.fade();
        json!({ "program": "julia", "uniforms": u })
    }
}

impl App for Julia {
    fn route(&self) -> &str {
        "julia"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("julia").emoji("🌀").category("toys")
    }
    fn wear(&mut self, world: &Json) {
        self.gpu = world["shared"]["settings"]["render"] == "gpu";
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "steps": self.steps,
            "over": false,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "frame": if self.gpu {
                frame::empty_fact(self.set.width as usize, self.set.height as usize)
            } else {
                self.render().fact()
            },
            "shade": self.shade(),
        })
    }
    fn capture(&self, _iden: &Iden) -> Json {
        self.render().fact()
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("julia.step", json!({ "n": "int" })),
            Verb::new("julia.reset", json!({ "seed": "int" })),
            Verb::new("julia.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "julia.step" => {
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
            "julia.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "julia.set" => {
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
        Some(Call::new("julia.step", json!({})))
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "c": [self.c.0, self.c.1],
            "start": [self.start.xmin, self.start.xmax, self.start.ymin, self.start.ymax],
            "target": [self.target.0, self.target.1],
            "zoom": self.zoom,
            "age": self.age as u64,
            "phase": self.phase,
            "rotation": self.rotation,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let (Some(cre), Some(cim)) = (state["c"][0].as_f64(), state["c"][1].as_f64()) {
            self.c = (cre, cim);
        }
        if let (Some(xmin), Some(xmax), Some(ymin), Some(ymax)) = (
            state["start"][0].as_f64(),
            state["start"][1].as_f64(),
            state["start"][2].as_f64(),
            state["start"][3].as_f64(),
        ) {
            self.start = Viewport {
                xmin,
                xmax,
                ymin,
                ymax,
            };
        }
        if let (Some(tx), Some(ty)) = (state["target"][0].as_f64(), state["target"][1].as_f64()) {
            self.target = (tx, ty);
        }
        if let Some(zoom) = state["zoom"].as_f64() {
            self.zoom = zoom;
        }
        if let Some(age) = state["age"].as_u64() {
            self.age = age as usize;
        }
        if let Some(phase) = state["phase"].as_f64() {
            self.phase = phase;
        }
        if let Some(rotation) = state["rotation"].as_f64() {
            self.rotation = rotation;
        }
        self.steps = state["steps"].as_u64().unwrap_or(0);
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
        self.fill();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, seeded, send};

    fn julia(seed: u64) -> Julia {
        seeded(Julia::new(), "julia.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = julia(5);
        let mut b = julia(5);
        for s in [&mut a, &mut b] {
            send(s, "julia.step", json!({ "n": 40 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn preset_resolves() {
        let mut j = julia(1);
        let out = send(
            &mut j,
            "julia.set",
            json!({ "key": "preset", "value": "0.285+0.01i" }),
        );
        assert!(out.ok);
        assert_eq!(j.c, (0.285, 0.01));
    }
    #[test]
    fn custom_c() {
        let mut j = julia(1);
        send(
            &mut j,
            "julia.set",
            json!({ "key": "preset", "value": "custom" }),
        );
        send(&mut j, "julia.set", json!({ "key": "cre", "value": 0.1 }));
        send(&mut j, "julia.set", json!({ "key": "cim", "value": -0.2 }));
        assert_eq!(j.c, (0.1, -0.2));
    }
    #[test]
    fn step_counts_and_frame_skips() {
        let mut j = julia(9);
        let out = send(&mut j, "julia.step", json!({ "n": 5 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(5));
        assert_eq!(j.state(&iden())["steps"], json!(5));
        assert!(!send(&mut j, "julia.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut j, "julia.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn set_validates_and_resets() {
        let mut j = julia(4);
        send(&mut j, "julia.step", json!({ "n": 3 }));
        let out = send(&mut j, "julia.set", json!({ "key": "width", "value": 48 }));
        assert!(out.ok);
        let state = j.state(&iden());
        assert_eq!(state["settings"]["width"], json!(48));
        assert_eq!(state["steps"], json!(0));
        assert!(
            !send(
                &mut j,
                "julia.set",
                json!({ "key": "width", "value": 9999 })
            )
            .ok
        );
        assert!(
            !send(
                &mut j,
                "julia.set",
                json!({ "key": "spin", "value": "fast" })
            )
            .ok
        );
        assert!(!send(&mut j, "julia.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = julia(11);
        send(&mut a, "julia.step", json!({ "n": 300 }));
        let mut b = Julia::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for s in [&mut a, &mut b] {
            send(s, "julia.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut j = Julia::new();
        j.load(&json!({ "seed": "soup", "c": "nope", "settings": 7 }));
        assert_eq!(j.state(&iden())["steps"], json!(0));
        assert_eq!(j.state(&iden())["seed"], json!(0));
        let frame = j.state(&iden())["frame"].clone();
        assert!(!frame["rows"].as_array().unwrap().is_empty());
    }
    #[test]
    fn beat_steps_forever() {
        let mut j = julia(3);
        send(&mut j, "julia.step", json!({ "n": 500 }));
        assert_eq!(j.beat(), Some(Call::new("julia.step", json!({}))));
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let j = julia(5);
        let state = j.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
    }
    #[test]
    fn gpu_mode_skips_the_cpu_raster() {
        let mut j = julia(5);
        let cpu = j.state(&iden())["frame"].clone();
        assert!(!cpu["rows"].as_array().unwrap().is_empty());
        j.wear(&json!({ "shared": { "settings": { "render": "gpu" } } }));
        let gpu = j.state(&iden())["frame"].clone();
        assert_eq!(gpu["width"], cpu["width"]);
        assert_eq!(gpu["height"], cpu["height"]);
        assert!(gpu["rows"].as_array().unwrap().is_empty());
        assert!(gpu["palette"].as_array().unwrap().is_empty());
        assert_eq!(j.capture(&iden()), cpu);
    }
}
