#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The two-pen tile skin the flat designs are drawn with.
pub mod skin;

use mrlycore::colors::hex;
use mrlycore::colors::named;
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};
use mrlymath::bang::{bang, factory, universe_codes};
use mrlymath::name::Named;
use mrlymath::space::{Pack, TURN};
use mrlymath::three::{quads, Cell3d};
use mrlymath::two::Cell2d;
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use std::f64::consts::TAU;

const BASE: usize = 2;
const NUMBER: usize = 3;
const LEVEL_1D: usize = 6;
const LEVEL_2D: usize = 5;
const LEVEL_3D: usize = 2;
const FILL: &str = "yellow";
const VOID: &str = "black";
const LIGHT_YAW: i64 = 72;
const LIGHT_PITCH: i64 = 28;
const BANDS: i64 = 4;

/// The base-2 design catalog, open to one page at a time.
pub struct Bang {
    dimension: usize,
    index: usize,
    dark: bool,
}

impl Default for Bang {
    fn default() -> Bang {
        Bang::new()
    }
}

impl Bang {
    /// Opens the catalog at the first two-dimensional design.
    pub fn new() -> Bang {
        Bang {
            dimension: 2,
            index: 0,
            dark: false,
        }
    }
    fn int(value: &Json) -> Option<i64> {
        value
            .as_i64()
            .or_else(|| value.as_str().and_then(|s| s.parse::<i64>().ok()))
    }
    fn codes(&self) -> &'static [u128] {
        universe_codes(self.dimension)
    }
    fn code(&self) -> u128 {
        let codes = self.codes();
        codes[self.index % codes.len()]
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "dimension" => {
                let n = Bang::int(value).ok_or("value must be an integer")?;
                match n {
                    1..=3 => {
                        self.dimension = n as usize;
                        self.index = 0;
                        Ok(json!(n))
                    }
                    4 => Err("too big to walk, nothing to render"),
                    _ => Err("dimension must be 1, 2, or 3"),
                }
            }
            "base" => Err("base 2 is the whole catalog today"),
            _ => Err("no such key"),
        }
    }
    fn tensor(&self) -> Tensor {
        let code = self.code();
        match self.dimension {
            1 => factory::create(code, NUMBER, 1, BASE, LEVEL_1D).unwrap(),
            3 => factory::create(code, NUMBER, 3, BASE, LEVEL_3D).unwrap(),
            _ => factory::create(code, NUMBER, 2, BASE, LEVEL_2D).unwrap(),
        }
    }
    fn cells_fact(&self) -> Json {
        let tensor = self.tensor();
        let ids: Vec<Vec<u8>> = match self.dimension {
            1 => vec![tensor.bytes().iter().map(|&v| u8::from(v == 1)).collect()],
            _ => {
                let cell = Cell2d::new(tensor);
                let w = cell.width();
                cell.types()
                    .bytes()
                    .chunks(w)
                    .map(|row| row.iter().map(|&v| u8::from(v == 1)).collect())
                    .collect()
            }
        };
        let fill = named(FILL).unwrap();
        let void = named(VOID).unwrap();
        json!({
            "ids": ids,
            "skin": "tiles",
            "pens": [hex([void.r, void.g, void.b, 255]), hex([fill.r, fill.g, fill.b, 255])],
        })
    }
    fn signature(&self) -> String {
        self.code().to_string()
    }
    fn shade(&self) -> Json {
        json!({ "program": "mesh", "route": "bang", "mesh": self.signature() })
    }
}

impl App for Bang {
    fn route(&self) -> &str {
        "bang"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("bang").emoji("💥").category("math")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let code = self.code();
        let design = bang(self.dimension).design(code);
        let mut out = json!({
            "dimension": self.dimension,
            "base": BASE,
            "index": self.index,
            "count": self.codes().len(),
            "name": mrlymath::name::Bang::new(code, self.dimension, BASE).to_str(),
            "code": code.to_string(),
            "degree": design.degree(),
            "anf": design.anf(),
        });
        if self.dimension == 3 {
            out["shade"] = self.shade();
        } else {
            out["cells"] = self.cells_fact();
        }
        out
    }
    fn geometry(&self) -> Option<Vec<f32>> {
        if self.dimension != 3 {
            return None;
        }
        let cell = Cell3d::new(self.tensor());
        let mut pack = Pack::new();
        for quad in quads(&cell) {
            pack.quad(quad.verts, quad.normal);
        }
        Some(pack.buffer())
    }
    fn uniforms(&self) -> Option<Vec<f32>> {
        if self.dimension != 3 {
            return None;
        }
        let rad = TAU / TURN as f64;
        let fill = named(FILL).unwrap();
        let void = named(VOID).unwrap();
        let mut u = vec![0.0; 24];
        u[4] = void.r as f64 / 255.0;
        u[5] = void.g as f64 / 255.0;
        u[6] = void.b as f64 / 255.0;
        u[8] = fill.r as f64 / 255.0;
        u[9] = fill.g as f64 / 255.0;
        u[10] = fill.b as f64 / 255.0;
        u[11] = BANDS as f64;
        u[19] = 1.0;
        u[20] = LIGHT_YAW as f64 * rad;
        u[21] = LIGHT_PITCH as f64 * rad;
        Some(u.into_iter().map(|v| v as f32).collect())
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("bang.page", json!({ "dir": "next | prev" })),
            Verb::new(
                "bang.set",
                json!({
                    "key": { "dimension": "int 1..3" },
                    "value": "of key",
                }),
            ),
            Verb::new("bang.reset", json!({})),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "bang.page" => {
                let count = self.codes().len();
                let next = match call.arg("dir").as_str() {
                    Some("next") => (self.index + 1) % count,
                    Some("prev") => (self.index + count - 1) % count,
                    _ => return Outcome::fail("dir must be next or prev"),
                };
                self.index = next;
                Outcome::ok(json!({ "index": self.index }))
            }
            "bang.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "bang.reset" => {
                self.dimension = 2;
                self.index = 0;
                Outcome::ok(json!({}))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "dimension": self.dimension, "index": self.index })
    }
    fn load(&mut self, state: &Json) {
        let mut next = Bang::new();
        if let Some(n) = state["dimension"].as_u64() {
            if (1..=3).contains(&n) {
                next.dimension = n as usize;
            }
        }
        if let Some(i) = state["index"].as_u64() {
            next.index = i as usize;
        }
        if next.index >= universe_codes(next.dimension).len() {
            next.index = 0;
        }
        *self = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    #[test]
    fn set_validates() {
        let mut b = Bang::new();
        assert!(
            send(
                &mut b,
                "bang.set",
                json!({ "key": "dimension", "value": 3 })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "bang.set",
                json!({ "key": "dimension", "value": 4 })
            )
            .ok
        );
        assert!(
            !send(
                &mut b,
                "bang.set",
                json!({ "key": "dimension", "value": 7 })
            )
            .ok
        );
        assert!(!send(&mut b, "bang.set", json!({ "key": "base", "value": 2 })).ok);
        assert!(!send(&mut b, "bang.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn page_cycles() {
        let mut b = Bang::new();
        let count = universe_codes(2).len();
        assert_eq!(b.index, 0);
        send(&mut b, "bang.page", json!({ "dir": "prev" }));
        assert_eq!(b.index, count - 1);
        send(&mut b, "bang.page", json!({ "dir": "next" }));
        assert_eq!(b.index, 0);
        send(&mut b, "bang.page", json!({ "dir": "next" }));
        assert_eq!(b.index, 1);
        assert!(!send(&mut b, "bang.page", json!({ "dir": "sideways" })).ok);
    }
    #[test]
    fn dimension_change_resets_index() {
        let mut b = Bang::new();
        send(&mut b, "bang.page", json!({ "dir": "next" }));
        assert_eq!(b.index, 1);
        send(
            &mut b,
            "bang.set",
            json!({ "key": "dimension", "value": 3 }),
        );
        assert_eq!(b.dimension, 3);
        assert_eq!(b.index, 0);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Bang::new();
        send(
            &mut a,
            "bang.set",
            json!({ "key": "dimension", "value": 3 }),
        );
        send(&mut a, "bang.page", json!({ "dir": "next" }));
        let mut b = Bang::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut b = Bang::new();
        b.load(&json!({ "dimension": 4, "index": 9999 }));
        assert_eq!(b.dimension, 2);
        assert_eq!(b.index, 0);
        b.load(&json!({ "dimension": 1, "index": 999 }));
        assert_eq!(b.dimension, 1);
        assert_eq!(b.index, 0);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let b = Bang::new();
        let names: Vec<String> = b.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["bang.page", "bang.set", "bang.reset"]);
    }
    #[test]
    fn flat_universes_speak_cells_and_solid_ones_shade_mesh() {
        let mut b = Bang::new();
        for d in [1i64, 2] {
            send(
                &mut b,
                "bang.set",
                json!({ "key": "dimension", "value": d }),
            );
            let state = b.state(&iden(), None);
            let cells = &state["cells"];
            assert_eq!(cells["skin"], json!("tiles"));
            assert_eq!(cells["pens"], json!(["#000000", "#ffd100"]));
            let ids = cells["ids"].as_array().unwrap();
            assert!(!ids.is_empty());
            assert!(!ids[0].as_array().unwrap().is_empty());
            if d == 1 {
                assert_eq!(ids.len(), 1);
            } else {
                assert_eq!(ids.len(), ids[0].as_array().unwrap().len());
            }
            assert!(state["shade"].is_null());
            assert!(b.geometry().is_none());
            assert!(b.uniforms().is_none());
        }
        send(
            &mut b,
            "bang.set",
            json!({ "key": "dimension", "value": 3 }),
        );
        let state = b.state(&iden(), None);
        assert!(state["cells"].is_null());
        assert!(state["frame"].is_null());
        assert_eq!(state["shade"]["program"], json!("mesh"));
        assert_eq!(state["shade"]["route"], json!("bang"));
        assert_eq!(b.uniforms().unwrap().len(), 24);
        let mut solid = false;
        for _ in 0..universe_codes(3).len() {
            if b.geometry().unwrap()[0] > 0.0 {
                solid = true;
                break;
            }
            send(&mut b, "bang.page", json!({ "dir": "next" }));
        }
        assert!(solid);
        let held = b.state(&iden(), None)["shade"]["mesh"]
            .as_str()
            .unwrap()
            .to_string();
        send(&mut b, "bang.page", json!({ "dir": "next" }));
        let turned = b.state(&iden(), None);
        assert_ne!(turned["shade"]["mesh"].as_str().unwrap(), held);
    }
}
