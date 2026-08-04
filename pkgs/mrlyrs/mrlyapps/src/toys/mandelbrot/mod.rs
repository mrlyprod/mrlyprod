use mrlycore::rng::Rng;
use mrlycore::trig::{FracIndex, N as TRIG_N};
use mrlycore::{json, Json};
use mrlymath::fractal::{self, Viewport, Wayfinder};
use mrlyos::kernel::{int, App, Call, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{self, Frame};
use std::f64::consts::TAU;

const MILLI: i64 = 1000;
const MICRO: i64 = 1_000_000;
const TURN_MILLI: i64 = TRIG_N as i64 * MILLI;

struct Set {
    width: i64,
    height: i64,
    depth: i64,
    zoom: i64,
    cycle: i64,
    start: i64,
    band: i64,
    drift: i64,
    fade: i64,
    spin: i64,
    primary: [u8; 4],
    accent: [u8; 4],
}

impl Set {
    fn new() -> Set {
        Set {
            width: 100,
            height: 100,
            depth: 96,
            zoom: 1012,
            cycle: 240,
            start: 500,
            band: 10_000,
            drift: 400,
            fade: 24,
            spin: 0,
            primary: frame::hex_of("#000000"),
            accent: frame::hex_of("#1ec9f3"),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "width" => int(&mut self.width, value, (16, 512)),
            "height" => int(&mut self.height, value, (16, 512)),
            "depth" => int(&mut self.depth, value, (16, 600)),
            "zoom" => int(&mut self.zoom, value, (1000, 1050)),
            "cycle" => int(&mut self.cycle, value, (30, 3000)),
            "start" => int(&mut self.start, value, (250, 4000)),
            "band" => int(&mut self.band, value, (2000, 64000)),
            "drift" => int(&mut self.drift, value, (0, 4000)),
            "fade" => int(&mut self.fade, value, (0, 240)),
            "spin" => int(&mut self.spin, value, (0, 50)),
            "primary" | "accent" => {
                let s = value.as_str().ok_or("value must be a color string")?;
                let c = mrlycore::colors::named(s)
                    .map_or_else(|_| frame::hex_of(s), |c| [c.r, c.g, c.b, c.a]);
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
    target: (i64, i64),
    zoom: i64,
    age: usize,
    phase: i64,
    rotation: i64,
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
            target: (0, 0),
            zoom: MICRO,
            age: 0,
            phase: 0,
            rotation: 0,
            iters: Vec::new(),
            gpu: false,
        };
        mandelbrot.reset(0);
        mandelbrot
    }
    fn begin(&mut self) {
        let (cx, cy) = fractal::MANDELBROT.center();
        let start = self.set.start.max(10) as i128;
        let vw = (fractal::MANDELBROT.xmax - fractal::MANDELBROT.xmin) as i128;
        let vh = (fractal::MANDELBROT.ymax - fractal::MANDELBROT.ymin) as i128;
        let hw = (vw * MILLI as i128 / (2 * start)) as i64;
        let hh = (vh * MILLI as i128 / (2 * start)) as i64;
        self.start =
            Viewport::around(cx, cy, hw, hh).fit(self.set.width as usize, self.set.height as usize);
        self.target = Wayfinder::Mandelbrot.pick(&self.start, &mut self.rng);
        self.zoom = MICRO;
        self.age = 0;
        self.rotation = 0;
        self.fill();
    }
    fn viewport(&self) -> Viewport {
        let vw = (self.start.xmax - self.start.xmin) as i128;
        let vh = (self.start.ymax - self.start.ymin) as i128;
        let hw = (vw * MICRO as i128 / (2 * self.zoom as i128)) as i64;
        let hh = (vh * MICRO as i128 / (2 * self.zoom as i128)) as i64;
        Viewport::around(self.target.0, self.target.1, hw, hh)
    }
    fn tilt(&self) -> (f64, f64) {
        let (c, s) = FracIndex::new(self.rotation as f32 / MILLI as f32).unit();
        (c as f64, s as f64)
    }
    fn angle(&self) -> f64 {
        let idx = FracIndex::new(self.rotation as f32 / MILLI as f32).index();
        idx as f64 * TAU / TRIG_N as f64
    }
    fn fill(&mut self) {
        let w = self.set.width as usize;
        let h = self.set.height as usize;
        let depth = self.set.depth;
        let [xmin, xmax, ymin, ymax] = self.viewport().reals();
        let center = ((xmin + xmax) * 0.5, (ymin + ymax) * 0.5);
        let vw = xmax - xmin;
        let vh = ymax - ymin;
        let (ca, sa) = self.tilt();
        self.iters = vec![0; w * h];
        for py in 0..h {
            let uy = (py as f64 + 0.5) / h as f64;
            for px in 0..w {
                let ux = (px as f64 + 0.5) / w as f64;
                let cr = xmin + ux * vw;
                let ci = ymax - uy * vh;
                let (cr, ci) = fractal::rotate(cr, ci, center, ca, sa);
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
        self.phase = 0;
        self.begin();
    }
    fn step_once(&mut self) {
        self.age += 1;
        let grown = self.zoom as i128 * self.set.zoom.max(MILLI) as i128 / MILLI as i128;
        self.zoom = grown.min(i64::MAX as i128) as i64;
        self.phase += self.set.drift;
        if self.set.spin != 0 {
            self.rotation = (self.rotation + self.set.spin).rem_euclid(TURN_MILLI);
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
        let phase = self.phase as f64 / MILLI as f64;
        let band = self.set.band as f64 / MILLI as f64;
        let mut colors = vec![primary; w * h];
        for (slot, &it) in colors.iter_mut().zip(self.iters.iter()) {
            let c = fractal::shade(it, depth, phase, band, primary, accent);
            *slot = if f < 1.0 {
                frame::mix(primary, c, f)
            } else {
                c
            };
        }
        frame::field(w, h, colors, primary)
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
    fn state(&self, _iden: &Iden, shape: Option<&Json>) -> Json {
        json!({
            "steps": self.steps,
            "over": false,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "frame": if self.gpu || !crate::asked(shape, "frame") {
                frame::empty_fact(self.set.width as usize, self.set.height as usize)
            } else {
                self.render().fact()
            },
            "shade": json!({ "program": "mandelbrot" }),
        })
    }
    fn capture(&self, _iden: &Iden) -> Json {
        self.render().fact()
    }
    fn uniforms(&self) -> Option<Vec<f32>> {
        let [xmin, xmax, ymin, ymax] = self.viewport().reals();
        let p = self.set.primary;
        let a = self.set.accent;
        let mut u = vec![0.0; 20];
        u[2] = self.phase as f64 / MILLI as f64;
        u[4] = p[0] as f64 / 255.0;
        u[5] = p[1] as f64 / 255.0;
        u[6] = p[2] as f64 / 255.0;
        u[8] = a[0] as f64 / 255.0;
        u[9] = a[1] as f64 / 255.0;
        u[10] = a[2] as f64 / 255.0;
        u[11] = self.angle();
        u[12] = xmin;
        u[13] = xmax;
        u[14] = ymin;
        u[15] = ymax;
        u[16] = self.set.depth as f64;
        u[17] = self.set.band as f64 / MILLI as f64;
        u[18] = self.fade();
        Some(u.into_iter().map(|v| v as f32).collect())
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("mandelbrot.step", json!({ "n": "int" })),
            Verb::new("mandelbrot.reset", json!({ "seed": "int" })),
            Verb::new("mandelbrot.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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
            state["start"][0].as_i64(),
            state["start"][1].as_i64(),
            state["start"][2].as_i64(),
            state["start"][3].as_i64(),
        ) {
            self.start = Viewport {
                xmin,
                xmax,
                ymin,
                ymax,
            };
        }
        if let (Some(tx), Some(ty)) = (state["target"][0].as_i64(), state["target"][1].as_i64()) {
            self.target = (tx, ty);
        }
        if let Some(zoom) = state["zoom"].as_i64() {
            self.zoom = zoom;
        }
        if let Some(age) = state["age"].as_u64() {
            self.age = age as usize;
        }
        if let Some(phase) = state["phase"].as_i64() {
            self.phase = phase;
        }
        if let Some(rotation) = state["rotation"].as_i64() {
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
    use mrlyos::kernel::testkit::{iden, seeded, send};

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
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
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
        assert_eq!(m.zoom, MICRO);
    }
    #[test]
    fn set_takes_color_names_and_hex() {
        let mut m = mandelbrot(2);
        let out = send(
            &mut m,
            "mandelbrot.set",
            json!({ "key": "primary", "value": "cyan" }),
        );
        assert!(out.ok);
        assert_eq!(
            m.state(&iden(), None)["settings"]["primary"],
            json!("#1ec9f3")
        );
        send(
            &mut m,
            "mandelbrot.set",
            json!({ "key": "accent", "value": "#ff8800" }),
        );
        assert_eq!(
            m.state(&iden(), None)["settings"]["accent"],
            json!("#ff8800")
        );
    }
    #[test]
    fn zoom_accumulates_in_micro() {
        let mut m = mandelbrot(2);
        send(&mut m, "mandelbrot.step", json!({ "n": 1 }));
        assert_eq!(m.zoom, 1_012_000);
    }
    #[test]
    fn default_view_frames_the_set() {
        let m = mandelbrot(2);
        let f = 1_000_000_000_000_000i64;
        assert_eq!(fractal::MANDELBROT.center(), (-f / 2, 0));
        assert_eq!(m.start.xmin, -7 * f / 2);
        assert_eq!(m.start.xmax, 5 * f / 2);
        assert_eq!(m.start.ymin, -3 * f);
        assert_eq!(m.start.ymax, 3 * f);
    }
    #[test]
    fn step_counts_and_frame_skips() {
        let mut m = mandelbrot(9);
        let out = send(&mut m, "mandelbrot.step", json!({ "n": 5 }));
        assert!(out.ok);
        assert_eq!(out.data["steps"], json!(5));
        assert_eq!(m.state(&iden(), None)["steps"], json!(5));
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
        let state = m.state(&iden(), None);
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
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        assert_eq!(b.save(), a.save());
        for s in [&mut a, &mut b] {
            send(s, "mandelbrot.step", json!({ "n": 6 }));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut m = Mandelbrot::new();
        m.load(&json!({ "seed": "soup", "start": "nope", "settings": 7 }));
        assert_eq!(m.state(&iden(), None)["steps"], json!(0));
        assert_eq!(m.state(&iden(), None)["seed"], json!(0));
        let frame = m.state(&iden(), None)["frame"].clone();
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
        let state = m.state(&iden(), None);
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
        let cpu = m.state(&iden(), None)["frame"].clone();
        assert!(!cpu["rows"].as_array().unwrap().is_empty());
        m.wear(&json!({ "shared": { "settings": { "render": "gpu" } } }));
        let gpu = m.state(&iden(), None)["frame"].clone();
        assert_eq!(gpu["width"], cpu["width"]);
        assert_eq!(gpu["height"], cpu["height"]);
        assert!(gpu["rows"].as_array().unwrap().is_empty());
        assert!(gpu["palette"].as_array().unwrap().is_empty());
        assert_eq!(m.capture(&iden()), cpu);
    }
    #[test]
    fn a_shape_without_frame_skips_the_cpu_raster() {
        let m = mandelbrot(5);
        let shape = json!({ "steps": 1 });
        let thin = m.state(&iden(), Some(&shape));
        assert!(thin["frame"]["rows"].as_array().unwrap().is_empty());
        assert_eq!(thin["steps"], json!(0));
        let asked = json!({ "frame": 1 });
        assert!(!m.state(&iden(), Some(&asked))["frame"]["rows"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}
