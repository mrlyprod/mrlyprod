#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use mrlycore::colors::ink;
use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::{json, Json};
use mrlymath::space::{axis_edges, solid, Pack, PITCH_MAX, SOLIDS, TURN};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use std::f64::consts::TAU;

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

/// One platonic solid on a turntable, its faces stepped into bands by an aimed light.
pub struct Solids {
    set: Set,
    rng: Rng,
    seed: u64,
    object: String,
    spin: i64,
    base: [u8; 4],
    dark: bool,
}

impl Default for Solids {
    fn default() -> Solids {
        Solids::new()
    }
}

impl Solids {
    /// Opens the app on the icosahedron, unturned, in the color seed zero rolls.
    pub fn new() -> Solids {
        let mut solids = Solids {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            object: "icosa".to_string(),
            spin: 0,
            base: [255, 255, 255, 255],
            dark: false,
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
        json!({ "program": "mesh", "route": "solids", "mesh": self.signature() })
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
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        json!({
            "seed": self.seed,
            "object": &self.object,
            "spin": self.spin,
            "settings": self.set.to_json(),
            "shade": self.shade(),
        })
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
    fn uniforms(&self) -> Option<Vec<f32>> {
        let rad = TAU / TURN as f64;
        let board = mrlycore::colors::board(self.dark);
        let mut u = vec![0.0; 24];
        u[4] = board[0] as f64 / 255.0;
        u[5] = board[1] as f64 / 255.0;
        u[6] = board[2] as f64 / 255.0;
        u[8] = self.base[0] as f64 / 255.0;
        u[9] = self.base[1] as f64 / 255.0;
        u[10] = self.base[2] as f64 / 255.0;
        u[11] = self.set.bands as f64;
        u[12] = self.spin as f64 * rad;
        u[19] = self.set.alpha as f64 / 255.0;
        u[20] = self.set.light_yaw as f64 * rad;
        u[21] = self.set.light_pitch as f64 * rad;
        Some(u.into_iter().map(|v| v as f32).collect())
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("solids.step", json!({ "n": "int" })),
            Verb::new(
                "solids.pick",
                json!({ "solid": "cube | tetra | octa | icosa" }),
            ),
            Verb::new("solids.reset", json!({ "seed": "int" })),
            Verb::new(
                "solids.set",
                json!({
                    "key": {
                        "bands": "int 2..8",
                        "speed": "int 0..16",
                        "alpha": "int 32..255",
                        "light_yaw": format!("int 0..{}", TURN - 1),
                        "light_pitch": format!("int -{PITCH_MAX}..{PITCH_MAX}"),
                        "edges": "bool",
                        "wireframe": "bool",
                        "axes": "bool",
                    },
                    "value": "of key",
                }),
            ),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
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
            "object": &self.object,
            "spin": self.spin,
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
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn solids(seed: u64) -> Solids {
        seeded(Solids::new(), "solids.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = solids(123);
        let mut b = solids(123);
        for s in [&mut a, &mut b] {
            send(s, "solids.pick", json!({ "solid": "octa" }));
            send(s, "solids.step", json!({ "n": 5 }));
        }
        assert_eq!(a.state(&iden(), None), b.state(&iden(), None));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn save_load_continues() {
        let mut a = solids(11);
        send(&mut a, "solids.pick", json!({ "solid": "tetra" }));
        send(&mut a, "solids.step", json!({ "n": 7 }));
        let mut b = Solids::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        for s in [&mut a, &mut b] {
            send(s, "solids.step", json!({ "n": 4 }));
        }
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut s = Solids::new();
        s.load(&json!({ "seed": "soup", "object": "sphere" }));
        assert_eq!(s.state(&iden(), None)["seed"], json!(0));
        assert_eq!(s.object, "icosa");
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
        let out = s.call(&iden(), &Call::new("solids.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let s = solids(3);
        let names: Vec<String> = s.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["solids.step", "solids.pick", "solids.reset", "solids.set"]
        );
    }
    #[test]
    fn looks_validate_and_change_the_pack() {
        let mut s = solids(9);
        let plain = s.geometry().unwrap();
        assert!(
            send(
                &mut s,
                "solids.set",
                json!({ "key": "edges", "value": true })
            )
            .ok
        );
        assert!(!send(&mut s, "solids.set", json!({ "key": "edges", "value": 3 })).ok);
        let edged = s.geometry().unwrap();
        assert!(edged[1] > plain[1]);
        assert!(
            send(
                &mut s,
                "solids.set",
                json!({ "key": "axes", "value": true })
            )
            .ok
        );
        assert!(s.geometry().unwrap()[1] > edged[1]);
    }
    #[test]
    fn alpha_makes_glass() {
        let mut s = solids(9);
        let plain = s.uniforms().unwrap();
        assert!(send(&mut s, "solids.set", json!({ "key": "alpha", "value": 96 })).ok);
        assert!(!send(&mut s, "solids.set", json!({ "key": "alpha", "value": 8 })).ok);
        assert_ne!(plain, s.uniforms().unwrap());
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
        assert_eq!(s.uniforms().unwrap().len(), 24);
    }
}
