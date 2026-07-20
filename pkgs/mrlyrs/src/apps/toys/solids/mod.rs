use crate::core::colors::ROLLABLE;
use crate::core::rng::Rng;
use crate::math::space::{beam, solid, Mat3, Rig, PAN_MAX, PITCH_MAX, SOLIDS, TURN};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{board, ink, Frame};
use crate::ui::scene::{axes, axis_edges, Pack, Scene};
use serde_json::{json, Value as Json};
use std::f64::consts::TAU;

const ORBIT: i64 = 8;

struct Set {
    bands: i64,
    speed: i64,
    light_yaw: i64,
    light_pitch: i64,
    alpha: i64,
    edges: bool,
    wireframe: bool,
    axes: bool,
}

impl Set {
    fn new() -> Set {
        Set {
            bands: 6,
            speed: 2,
            light_yaw: 72,
            light_pitch: 28,
            alpha: 255,
            edges: false,
            wireframe: false,
            axes: false,
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "edges" | "wireframe" | "axes" => {
                let b = value.as_bool().ok_or("value must be a boolean")?;
                match key {
                    "edges" => self.edges = b,
                    "wireframe" => self.wireframe = b,
                    _ => self.axes = b,
                }
                Ok(json!(b))
            }
            _ => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "bands" => (2, 8),
                    "speed" => (0, 16),
                    "alpha" => (32, 255),
                    "light_yaw" => (0, TURN - 1),
                    "light_pitch" => (-PITCH_MAX, PITCH_MAX),
                    _ => return Err("no such key"),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "bands" => self.bands = n,
                    "speed" => self.speed = n,
                    "alpha" => self.alpha = n,
                    "light_yaw" => self.light_yaw = n,
                    _ => self.light_pitch = n,
                }
                Ok(json!(n))
            }
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "bands": self.bands,
            "speed": self.speed,
            "light_yaw": self.light_yaw,
            "light_pitch": self.light_pitch,
            "alpha": self.alpha,
            "edges": self.edges,
            "wireframe": self.wireframe,
            "axes": self.axes,
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

pub struct Solids {
    set: Set,
    rng: Rng,
    seed: u64,
    object: String,
    spin: i64,
    rig: Rig,
    base: [u8; 4],
    dark: bool,
    gpu: bool,
    detail: usize,
}

impl Default for Solids {
    fn default() -> Solids {
        Solids::new()
    }
}

impl Solids {
    pub fn new() -> Solids {
        let mut solids = Solids {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            object: "icosa".to_string(),
            spin: 0,
            rig: Rig::new(),
            base: [255, 255, 255, 255],
            dark: false,
            gpu: false,
            detail: 96,
        };
        solids.reset(0);
        solids
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        self.base = [c.r, c.g, c.b, 255];
        self.spin = 0;
        self.rig = Rig::new();
        self.rig.view("iso");
    }
    fn shades(&self) -> Vec<[u8; 4]> {
        let bands = self.set.bands;
        (0..bands)
            .map(|k| {
                let t = (64 + 191 * k / (bands - 1)) as u32;
                let mix = |c: u8| (c as u32 * t / 255) as u8;
                [mix(self.base[0]), mix(self.base[1]), mix(self.base[2]), 255]
            })
            .collect()
    }
    fn scene(&self) -> Scene {
        let mesh = solid(&self.object);
        let spin = Mat3::yaw(self.spin);
        let light = beam(self.set.light_yaw, self.set.light_pitch);
        let shades = self.shades();
        let alpha = self.set.alpha as u8;
        let mut scene = Scene::new();
        if !self.set.wireframe {
            for (i, face) in mesh.faces.iter().enumerate() {
                let normal = spin.apply(mesh.normals[i]);
                let verts = face.map(|v| spin.apply(mesh.verts[v]));
                let lit = (normal.dot(light) + 1.0) * 0.5;
                let band =
                    ((lit * self.set.bands as f32).floor() as i64).clamp(0, self.set.bands - 1);
                let mut color = shades[band as usize];
                color[3] = alpha;
                scene.face(verts, normal, color);
            }
        }
        if self.set.edges || self.set.wireframe {
            for [a, b] in mesh.edges() {
                scene.edge(
                    spin.apply(mesh.verts[a]),
                    spin.apply(mesh.verts[b]),
                    ink(self.dark),
                );
            }
        }
        if self.set.axes {
            axes(&mut scene, ink(self.dark));
        }
        scene
    }
    fn render_at(&self, size: usize) -> Frame {
        self.scene()
            .paint(&self.rig.camera(), size, board(self.dark))
    }
    fn signature(&self) -> String {
        format!(
            "{}:w{}e{}a{}d{}",
            self.object,
            self.set.wireframe as u8,
            self.set.edges as u8,
            self.set.axes as u8,
            self.dark as u8
        )
    }
    fn shade(&self) -> Json {
        let rad = TAU / TURN as f64;
        let board = crate::ui::frame::board(self.dark);
        let mut u = vec![0.0; 24];
        u[4] = board[0] as f64 / 255.0;
        u[5] = board[1] as f64 / 255.0;
        u[6] = board[2] as f64 / 255.0;
        u[8] = self.base[0] as f64 / 255.0;
        u[9] = self.base[1] as f64 / 255.0;
        u[10] = self.base[2] as f64 / 255.0;
        u[11] = self.set.bands as f64;
        u[12] = self.spin as f64 * rad;
        u[13] = self.rig.yaw as f64 * rad;
        u[14] = self.rig.pitch as f64 * rad;
        u[15] = self.rig.dist as f64 / 4.0;
        u[16] = self.rig.pan[0] as f64 / 16.0;
        u[17] = self.rig.pan[1] as f64 / 16.0;
        u[18] = if self.rig.ortho { 1.0 } else { 0.0 };
        u[19] = self.set.alpha as f64 / 255.0;
        u[20] = self.set.light_yaw as f64 * rad;
        u[21] = self.set.light_pitch as f64 * rad;
        json!({ "program": "mesh", "route": "solids", "mesh": self.signature(), "uniforms": u })
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

impl App for Solids {
    fn route(&self) -> &str {
        "solids"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("solids").emoji("🧊").category("toys")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
        self.gpu = world["shared"]["settings"]["render"] == "gpu";
        self.detail = world["shared"]["settings"]["detail"]
            .as_f64()
            .unwrap_or(96.0) as usize;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "seed": self.seed,
            "object": self.object,
            "spin": self.spin,
            "camera": self.rig.to_json(),
            "settings": self.set.to_json(),
            "frame": self.render_at(if self.gpu { 96 } else { self.detail }).fact(),
            "shade": self.shade(),
        })
    }
    fn capture(&self, _iden: &Iden) -> Json {
        self.render_at(self.detail).fact()
    }
    fn geometry(&self) -> Option<Vec<f32>> {
        let mesh = solid(&self.object);
        let mut pack = Pack::new();
        if !self.set.wireframe {
            for (i, face) in mesh.faces.iter().enumerate() {
                pack.face(face.map(|v| mesh.verts[v]), mesh.normals[i]);
            }
        }
        if self.set.edges || self.set.wireframe {
            for [a, b] in mesh.edges() {
                pack.line(mesh.verts[a], mesh.verts[b], true, ink(self.dark));
            }
        }
        if self.set.axes {
            for edge in axis_edges(ink(self.dark)) {
                pack.line(edge.ends[0], edge.ends[1], false, edge.color);
            }
        }
        Some(pack.buffer())
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new(
                "solids.orbit",
                json!({ "dir": "up | down | left | right", "n": "int" }),
            ),
            Verb::new("solids.turn", json!({ "dyaw": "int", "dpitch": "int" })),
            Verb::new("solids.zoom", json!({ "dir": "in | out", "n": "int" })),
            Verb::new("solids.pan", json!({ "dx": "int", "dy": "int" })),
            Verb::new("solids.step", json!({ "n": "int" })),
            Verb::new(
                "solids.pick",
                json!({ "solid": "cube | tetra | octa | icosa" }),
            ),
            Verb::new("solids.reset", json!({ "seed": "int" })),
            Verb::new("solids.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "solids.orbit" => {
                let n = match Solids::count(call, 32) {
                    Ok(n) => n,
                    Err(note) => return Outcome::fail(note),
                };
                match call.arg("dir").as_str() {
                    Some("left") => self.rig.orbit(-ORBIT * n, 0),
                    Some("right") => self.rig.orbit(ORBIT * n, 0),
                    Some("up") => self.rig.orbit(0, ORBIT * n),
                    Some("down") => self.rig.orbit(0, -ORBIT * n),
                    _ => return Outcome::fail("dir must be up, down, left, or right"),
                }
                Outcome::ok(json!({ "yaw": self.rig.yaw, "pitch": self.rig.pitch }))
            }
            "solids.turn" => {
                let dyaw = call.arg("dyaw").as_i64().unwrap_or(0);
                let dpitch = call.arg("dpitch").as_i64().unwrap_or(0);
                if dyaw.abs() > TURN || dpitch.abs() > TURN {
                    return Outcome::fail("delta out of range");
                }
                self.rig.orbit(dyaw, dpitch);
                Outcome::ok(json!({ "yaw": self.rig.yaw, "pitch": self.rig.pitch }))
            }
            "solids.pan" => {
                let dx = call.arg("dx").as_i64().unwrap_or(0);
                let dy = call.arg("dy").as_i64().unwrap_or(0);
                if dx.abs() > 2 * PAN_MAX || dy.abs() > 2 * PAN_MAX {
                    return Outcome::fail("delta out of range");
                }
                self.rig.pan(dx, dy);
                Outcome::ok(json!({ "pan": self.rig.pan }))
            }
            "solids.zoom" => {
                let n = match Solids::count(call, 24) {
                    Ok(n) => n,
                    Err(note) => return Outcome::fail(note),
                };
                match call.arg("dir").as_str() {
                    Some("in") => self.rig.zoom(-n),
                    Some("out") => self.rig.zoom(n),
                    _ => return Outcome::fail("dir must be in or out"),
                }
                Outcome::ok(json!({ "dist": self.rig.dist }))
            }
            "solids.step" => {
                let n = match Solids::count(call, 1024) {
                    Ok(n) => n,
                    Err(note) => return Outcome::fail(note),
                };
                self.spin = (self.spin + self.set.speed * n).rem_euclid(TURN);
                Outcome::ok(json!({ "spin": self.spin }))
            }
            "solids.pick" => {
                let Some(name) = call.arg("solid").as_str() else {
                    return Outcome::fail("solid must be a string");
                };
                if !SOLIDS.contains(&name) {
                    return Outcome::fail("no such solid");
                }
                self.object = name.to_string();
                Outcome::ok(json!({ "solid": name }))
            }
            "solids.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "solids.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                if key == "ortho" {
                    let Some(b) = call.arg("value").as_bool() else {
                        return Outcome::fail("value must be a boolean");
                    };
                    self.rig.ortho = b;
                    return Outcome::ok(json!({ "key": key, "value": b }));
                }
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        if self.set.speed > 0 {
            Some(Call::new("solids.step", json!({})))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "object": self.object,
            "spin": self.spin,
            "camera": self.rig.to_json(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let Some(name) = state["object"].as_str() {
            if SOLIDS.contains(&name) {
                self.object = name.to_string();
            }
        }
        if let Some(spin) = state["spin"].as_i64() {
            self.spin = spin.rem_euclid(TURN);
        }
        self.rig.load(&state["camera"]);
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::space::{DIST_MAX, DIST_MIN};
    use crate::os::kernel::testkit::{iden, seeded, send};

    fn solids(seed: u64) -> Solids {
        seeded(Solids::new(), "solids.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = solids(123);
        let mut b = solids(123);
        for s in [&mut a, &mut b] {
            send(s, "solids.pick", json!({ "solid": "octa" }));
            send(s, "solids.orbit", json!({ "dir": "left", "n": 3 }));
            send(s, "solids.step", json!({ "n": 5 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn save_load_continues() {
        let mut a = solids(11);
        send(&mut a, "solids.pick", json!({ "solid": "tetra" }));
        send(&mut a, "solids.orbit", json!({ "dir": "up", "n": 2 }));
        send(&mut a, "solids.step", json!({ "n": 7 }));
        let mut b = Solids::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        for s in [&mut a, &mut b] {
            send(s, "solids.step", json!({ "n": 4 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut s = Solids::new();
        s.load(&json!({ "seed": "soup", "object": "sphere", "camera": { "dist": 999 } }));
        assert_eq!(s.state(&iden())["seed"], json!(0));
        assert_eq!(s.object, "icosa");
        assert_eq!(s.rig.dist, 12);
    }
    #[test]
    fn orbit_wraps_and_clamps() {
        let mut s = solids(1);
        send(&mut s, "solids.orbit", json!({ "dir": "left", "n": 32 }));
        assert!((0..TURN).contains(&s.rig.yaw));
        send(&mut s, "solids.orbit", json!({ "dir": "up", "n": 32 }));
        assert_eq!(s.rig.pitch, PITCH_MAX);
        send(&mut s, "solids.orbit", json!({ "dir": "down", "n": 32 }));
        assert_eq!(s.rig.pitch, -PITCH_MAX);
        assert!(!send(&mut s, "solids.orbit", json!({ "dir": "north" })).ok);
        assert!(!send(&mut s, "solids.orbit", json!({ "dir": "up", "n": 0 })).ok);
    }
    #[test]
    fn zoom_clamps() {
        let mut s = solids(1);
        send(&mut s, "solids.zoom", json!({ "dir": "in", "n": 24 }));
        assert_eq!(s.rig.dist, DIST_MIN);
        send(&mut s, "solids.zoom", json!({ "dir": "out", "n": 24 }));
        assert_eq!(s.rig.dist, DIST_MAX);
        assert!(!send(&mut s, "solids.zoom", json!({ "dir": "sideways" })).ok);
    }
    #[test]
    fn step_spins_by_speed() {
        let mut s = solids(1);
        send(&mut s, "solids.set", json!({ "key": "speed", "value": 3 }));
        send(&mut s, "solids.step", json!({ "n": 4 }));
        assert_eq!(s.spin, 12);
        send(&mut s, "solids.step", json!({ "n": 100 }));
        assert!((0..TURN).contains(&s.spin));
    }
    #[test]
    fn pick_validates() {
        let mut s = solids(1);
        assert!(send(&mut s, "solids.pick", json!({ "solid": "cube" })).ok);
        assert!(!send(&mut s, "solids.pick", json!({ "solid": "sphere" })).ok);
        assert_eq!(s.object, "cube");
    }
    #[test]
    fn set_validates() {
        let mut s = solids(1);
        assert!(send(&mut s, "solids.set", json!({ "key": "bands", "value": 4 })).ok);
        assert!(!send(&mut s, "solids.set", json!({ "key": "bands", "value": 1 })).ok);
        assert!(!send(&mut s, "solids.set", json!({ "key": "volume", "value": 2 })).ok);
        assert!(
            !send(
                &mut s,
                "solids.set",
                json!({ "key": "bands", "value": "big" })
            )
            .ok
        );
    }
    #[test]
    fn beat_gates_on_speed() {
        let mut s = solids(1);
        assert_eq!(s.beat(), Some(Call::new("solids.step", json!({}))));
        send(&mut s, "solids.set", json!({ "key": "speed", "value": 0 }));
        assert_eq!(s.beat(), None);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut s = Solids::new();
        let out = s.act(&iden(), &Call::new("solids.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
    }
    #[test]
    fn every_solid_draws_something() {
        let mut s = solids(5);
        for name in SOLIDS {
            send(&mut s, "solids.pick", json!({ "solid": name }));
            let state = s.state(&iden());
            let palette = state["frame"]["palette"].as_array().unwrap().len();
            assert!(palette > 2, "{name} painted {palette} colors");
            assert_eq!(state["frame"]["rows"].as_array().unwrap().len(), 96);
        }
    }
    #[test]
    fn the_frame_is_deterministic() {
        let a = solids(42);
        let b = solids(42);
        assert_eq!(a.state(&iden())["frame"], b.state(&iden())["frame"]);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let s = solids(3);
        let names: Vec<String> = s.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "solids.orbit",
                "solids.turn",
                "solids.zoom",
                "solids.pan",
                "solids.step",
                "solids.pick",
                "solids.reset",
                "solids.set"
            ]
        );
    }
    #[test]
    fn turn_pan_and_ortho_drive_the_rig() {
        let mut s = solids(1);
        send(&mut s, "solids.turn", json!({ "dyaw": -5, "dpitch": 3 }));
        assert_eq!((s.rig.yaw, s.rig.pitch), (27, 23));
        assert!(!send(&mut s, "solids.turn", json!({ "dyaw": 999 })).ok);
        send(&mut s, "solids.pan", json!({ "dx": 4, "dy": -6 }));
        assert_eq!(s.rig.pan, [4, -6]);
        assert!(!send(&mut s, "solids.pan", json!({ "dx": 999 })).ok);
        assert!(
            send(
                &mut s,
                "solids.set",
                json!({ "key": "ortho", "value": true })
            )
            .ok
        );
        assert!(s.rig.ortho);
        assert!(!send(&mut s, "solids.set", json!({ "key": "ortho", "value": 2 })).ok);
        let mut b = Solids::new();
        b.load(&s.save());
        assert_eq!(b.state(&iden()), s.state(&iden()));
    }
    #[test]
    fn looks_validate_and_change_the_frame() {
        let mut s = solids(9);
        let plain = s.state(&iden())["frame"].clone();
        assert!(
            send(
                &mut s,
                "solids.set",
                json!({ "key": "edges", "value": true })
            )
            .ok
        );
        assert!(!send(&mut s, "solids.set", json!({ "key": "edges", "value": 3 })).ok);
        let edged = s.state(&iden())["frame"].clone();
        assert_ne!(plain, edged);
        assert!(
            send(
                &mut s,
                "solids.set",
                json!({ "key": "wireframe", "value": true })
            )
            .ok
        );
        let wired = s.state(&iden())["frame"].clone();
        assert_ne!(edged, wired);
        assert!(
            send(
                &mut s,
                "solids.set",
                json!({ "key": "axes", "value": true })
            )
            .ok
        );
        assert_ne!(wired, s.state(&iden())["frame"]);
    }
    #[test]
    fn alpha_makes_glass() {
        let mut s = solids(9);
        let plain = s.state(&iden())["frame"].clone();
        assert!(send(&mut s, "solids.set", json!({ "key": "alpha", "value": 96 })).ok);
        assert!(!send(&mut s, "solids.set", json!({ "key": "alpha", "value": 8 })).ok);
        assert_ne!(plain, s.state(&iden())["frame"]);
    }
    #[test]
    fn gpu_mode_keeps_a_small_real_frame() {
        let mut s = solids(5);
        s.wear(&json!({ "shared": { "settings": { "render": "gpu", "detail": 128.0 } } }));
        let gpu = s.state(&iden())["frame"].clone();
        assert_eq!(gpu["width"], json!(96));
        assert_eq!(gpu["height"], json!(96));
        assert_eq!(gpu["rows"].as_array().unwrap().len(), 96);
        assert!(gpu["palette"].as_array().unwrap().len() > 2);
        let shot = s.capture(&iden());
        assert_eq!(shot["width"], json!(128));
        assert_eq!(shot["rows"].as_array().unwrap().len(), 128);
    }
    #[test]
    fn detail_sizes_the_cpu_frame() {
        let mut s = solids(5);
        s.wear(&json!({ "shared": { "settings": { "detail": 64.0 } } }));
        let frame = s.state(&iden())["frame"].clone();
        assert_eq!(frame["width"], json!(64));
        assert_eq!(s.capture(&iden()), frame);
    }
    #[test]
    fn geometry_packs_the_picked_solid() {
        let mut s = solids(5);
        let buf = s.geometry().unwrap();
        assert_eq!(buf[0], (20 * 3 * 6) as f32);
        assert_eq!(buf[1], 0.0);
        send(&mut s, "solids.pick", json!({ "solid": "cube" }));
        assert_eq!(s.geometry().unwrap()[0], (12 * 3 * 6) as f32);
        send(
            &mut s,
            "solids.set",
            json!({ "key": "wireframe", "value": true }),
        );
        let wired = s.geometry().unwrap();
        assert_eq!(wired[0], 0.0);
        assert_eq!(wired[1], (12 * 2 * 8) as f32);
    }
    #[test]
    fn the_mesh_signature_tracks_only_geometry() {
        let mut s = solids(5);
        let sig = |s: &Solids| s.shade()["mesh"].as_str().unwrap().to_string();
        let held = sig(&s);
        send(&mut s, "solids.orbit", json!({ "dir": "left", "n": 2 }));
        send(&mut s, "solids.step", json!({ "n": 3 }));
        send(&mut s, "solids.set", json!({ "key": "bands", "value": 3 }));
        send(&mut s, "solids.set", json!({ "key": "alpha", "value": 96 }));
        assert_eq!(sig(&s), held);
        send(
            &mut s,
            "solids.set",
            json!({ "key": "edges", "value": true }),
        );
        assert_ne!(sig(&s), held);
        let edged = sig(&s);
        send(&mut s, "solids.pick", json!({ "solid": "tetra" }));
        assert_ne!(sig(&s), edged);
        assert_eq!(s.shade()["program"], json!("mesh"));
        assert_eq!(s.shade()["route"], json!("solids"));
        assert_eq!(s.shade()["uniforms"].as_array().unwrap().len(), 24);
    }
}
