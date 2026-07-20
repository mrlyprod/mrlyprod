use crate::core::rng::Rng;
use crate::math::fractal::{self, Viewport, Wayfinder};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{self, Frame};
use serde_json::{json, Value as Json};

struct Set {
    width: i64,
    height: i64,
    depth: i64,
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
            zoom: 1.012,
            cycle: 240,
            start: 0.5,
            band: 10.0,
            drift: 0.4,
            fade: 24,
            spin: 0.0,
            primary: frame::hex_of("#000000"),
            accent: frame::hex_of("#1ec9f3"),
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
            "zoom" | "start" | "band" | "drift" | "spin" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
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
                    "zoom" => self.zoom = n,
                    "start" => self.start = n,
                    "band" => self.band = n,
                    "drift" => self.drift = n,
                    _ => self.spin = n,
                }
                Ok(json!(n))
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

pub struct Mandelbrot {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    start: Viewport,
    target: (f64, f64),
    zoom: f64,
    age: usize,
    phase: f64,
    rotation: f64,
    iters: Vec<i64>,
    gpu: bool,
}

impl Default for Mandelbrot {
    fn default() -> Mandelbrot {
        Mandelbrot::new()
    }
}

impl Mandelbrot {
    pub fn new() -> Mandelbrot {
        let mut mandelbrot = Mandelbrot {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            start: fractal::MANDELBROT,
            target: (0.0, 0.0),
            zoom: 1.0,
            age: 0,
            phase: 0.0,
            rotation: 0.0,
            iters: Vec::new(),
            gpu: false,
        };
        mandelbrot.reset(0);
        mandelbrot
    }
    fn begin(&mut self) {
        let (cx, cy) = fractal::MANDELBROT.center();
        let expand = 1.0 / self.set.start.max(0.01);
        let hw = (fractal::MANDELBROT.xmax - fractal::MANDELBROT.xmin) * 0.5 * expand;
        let hh = (fractal::MANDELBROT.ymax - fractal::MANDELBROT.ymin) * 0.5 * expand;
        self.start =
            Viewport::around(cx, cy, hw, hh).fit(self.set.width as usize, self.set.height as usize);
        self.target = Wayfinder::Mandelbrot.pick(&self.start, &mut self.rng);
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
        let v = self.view();
        let center = v.center();
        let vw = v.xmax - v.xmin;
        let vh = v.ymax - v.ymin;
        self.iters = vec![0; w * h];
        for py in 0..h {
            let uy = (py as f64 + 0.5) / h as f64;
            for px in 0..w {
                let ux = (px as f64 + 0.5) / w as f64;
                let cr = v.xmin + ux * vw;
                let ci = v.ymax - uy * vh;
                let (cr, ci) = fractal::rotate(cr, ci, center, self.rotation);
                self.iters[py * w + px] = fractal::mandelbrot(cr, ci, depth);
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
        let mut u = vec![0.0; 20];
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
        u[16] = self.set.depth as f64;
        u[17] = self.set.band;
        u[18] = self.fade();
        json!({ "program": "mandelbrot", "uniforms": u })
    }
}

impl App for Mandelbrot {
    fn route(&self) -> &str {
        "mandelbrot"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("mandelbrot").emoji("🌌").category("toys")
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
            Verb::new("mandelbrot.step", json!({ "n": "int" })),
            Verb::new("mandelbrot.reset", json!({ "seed": "int" })),
            Verb::new("mandelbrot.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "mandelbrot.step" => {
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
            "mandelbrot.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "mandelbrot.set" => {
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
        Some(Call::new("mandelbrot.step", json!({})))
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
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

    fn mandelbrot(seed: u64) -> Mandelbrot {
        seeded(Mandelbrot::new(), "mandelbrot.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = mandelbrot(5);
        let mut b = mandelbrot(5);
        for s in [&mut a, &mut b] {
            send(s, "mandelbrot.step", json!({ "n": 40 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn cycle_restarts() {
        let mut m = mandelbrot(2);
        send(
            &mut m,
            "mandelbrot.set",
            json!({ "key": "cycle", "value": 30 }),
        );
        send(&mut m, "mandelbrot.step", json!({ "n": 30 }));
        assert_eq!(m.age, 0);
        assert_eq!(m.zoom, 1.0);
    }
    #[test]
    fn step_counts_and_frame_skips() {
        let mut m = mandelbrot(9);
        let out = send(&mut m, "mandelbrot.step", json!({ "n": 5 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(5));
        assert_eq!(m.state(&iden())["steps"], json!(5));
        assert!(!send(&mut m, "mandelbrot.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut m, "mandelbrot.step", json!({ "n": 2000 })).ok);
    }
    #[test]
    fn set_validates_and_resets() {
        let mut m = mandelbrot(4);
        send(&mut m, "mandelbrot.step", json!({ "n": 3 }));
        let out = send(
            &mut m,
            "mandelbrot.set",
            json!({ "key": "width", "value": 48 }),
        );
        assert!(out.ok);
        let state = m.state(&iden());
        assert_eq!(state["settings"]["width"], json!(48));
        assert_eq!(state["steps"], json!(0));
        assert!(
            !send(
                &mut m,
                "mandelbrot.set",
                json!({ "key": "width", "value": 9999 })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "mandelbrot.set",
                json!({ "key": "spin", "value": "fast" })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "mandelbrot.set",
                json!({ "key": "volume", "value": 1 })
            )
            .ok
        );
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = mandelbrot(11);
        send(&mut a, "mandelbrot.step", json!({ "n": 300 }));
        let mut b = Mandelbrot::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for s in [&mut a, &mut b] {
            send(s, "mandelbrot.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut m = Mandelbrot::new();
        m.load(&json!({ "seed": "soup", "start": "nope", "settings": 7 }));
        assert_eq!(m.state(&iden())["steps"], json!(0));
        assert_eq!(m.state(&iden())["seed"], json!(0));
        let frame = m.state(&iden())["frame"].clone();
        assert!(!frame["rows"].as_array().unwrap().is_empty());
    }
    #[test]
    fn beat_steps_forever() {
        let mut m = mandelbrot(3);
        send(&mut m, "mandelbrot.step", json!({ "n": 500 }));
        assert_eq!(m.beat(), Some(Call::new("mandelbrot.step", json!({}))));
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let m = mandelbrot(5);
        let state = m.state(&iden());
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
        let mut m = mandelbrot(5);
        let cpu = m.state(&iden())["frame"].clone();
        assert!(!cpu["rows"].as_array().unwrap().is_empty());
        m.wear(&json!({ "shared": { "settings": { "render": "gpu" } } }));
        let gpu = m.state(&iden())["frame"].clone();
        assert_eq!(gpu["width"], cpu["width"]);
        assert_eq!(gpu["height"], cpu["height"]);
        assert!(gpu["rows"].as_array().unwrap().is_empty());
        assert!(gpu["palette"].as_array().unwrap().is_empty());
        assert_eq!(m.capture(&iden()), cpu);
    }
}
