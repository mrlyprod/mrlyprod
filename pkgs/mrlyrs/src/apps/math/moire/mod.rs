use crate::core::colors::{gradient, BLACK, CYAN};
use crate::math::bang::{code_to_corners, corners_to_code};
use crate::math::moire::{layer, Field, Lattice, Layer, Spec};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{field, Frame};
use serde_json::{json, Value as Json};

const CODE: u128 = 7;
const BASE: usize = 2;
const DIM: usize = 2;
const NUMBER: i64 = 9;
const SIZE: usize = 100;
const LEVELS: usize = 9;
const OFFSET_MIN: i64 = -6;
const OFFSET_MAX: i64 = 6;
const ANGLES: [i64; 4] = [0, 90, 180, 270];
const LATTICES: [&str; 2] = ["square", "hex"];

fn rotate90(corners: &[Vec<u8>], base: usize) -> Vec<Vec<u8>> {
    let q = base as u8;
    corners.iter().map(|c| vec![c[1], q - 1 - c[0]]).collect()
}

fn rotate(spec: Spec, times: u8) -> Spec {
    let mut corners = code_to_corners(spec.code, spec.dimension, spec.base).unwrap();
    for _ in 0..times {
        corners = rotate90(&corners, spec.base);
    }
    let code = corners_to_code(&corners, spec.dimension, spec.base);
    Spec::new(code, spec.base, spec.dimension)
}

struct Set {
    offset: i64,
    angle: i64,
    lattice: String,
}

impl Set {
    fn new() -> Set {
        Set {
            offset: 4,
            angle: 90,
            lattice: "square".to_string(),
        }
    }
    fn int(value: &Json) -> Option<i64> {
        value
            .as_i64()
            .or_else(|| value.as_str().and_then(|s| s.parse::<i64>().ok()))
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "offset" => {
                let n = Set::int(value).ok_or("value must be an integer")?;
                if !(OFFSET_MIN..=OFFSET_MAX).contains(&n) {
                    return Err("offset must be between -6 and 6");
                }
                self.offset = n;
                Ok(json!(n))
            }
            "angle" => {
                let n = Set::int(value).ok_or("value must be an integer")?;
                if !ANGLES.contains(&n) {
                    return Err("angle must be 0, 90, 180, or 270");
                }
                self.angle = n;
                Ok(json!(n))
            }
            "lattice" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if !LATTICES.contains(&name) {
                    return Err("lattice must be square or hex");
                }
                self.lattice = name.to_string();
                Ok(json!(name))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "offset": self.offset,
            "angle": self.angle,
            "lattice": self.lattice,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(n) = value["offset"].as_i64() {
            if (OFFSET_MIN..=OFFSET_MAX).contains(&n) {
                set.offset = n;
            }
        }
        if let Some(n) = value["angle"].as_i64() {
            if ANGLES.contains(&n) {
                set.angle = n;
            }
        }
        if let Some(name) = value["lattice"].as_str() {
            if LATTICES.contains(&name) {
                set.lattice = name.to_string();
            }
        }
        set
    }
}

pub struct Moire {
    set: Set,
}

impl Default for Moire {
    fn default() -> Moire {
        Moire::new()
    }
}

impl Moire {
    pub fn new() -> Moire {
        Moire { set: Set::new() }
    }
    fn lattice(&self) -> Lattice {
        match self.set.lattice.as_str() {
            "hex" => Lattice::Hex,
            _ => Lattice::Square,
        }
    }
    fn interference(&self) -> Field {
        let spec = Spec::new(CODE, BASE, DIM);
        let lattice = self.lattice();
        let mut base = Layer::new(spec, NUMBER as usize);
        base.lattice = lattice;
        base.size = SIZE;
        let mask_a = layer(&base).unwrap();
        let times = (self.set.angle / 90).rem_euclid(4) as u8;
        let over_spec = rotate(spec, times);
        let number_b = (NUMBER + self.set.offset).max(1) as usize;
        let mut over = Layer::new(over_spec, number_b);
        over.lattice = lattice;
        over.size = SIZE;
        let mask_b = layer(&over).unwrap();
        let data: Vec<f32> = mask_a
            .iter()
            .zip(mask_b.iter())
            .map(|(&a, &b)| a as u8 as f32 + b as u8 as f32)
            .collect();
        Field::from_data(data, SIZE)
    }
    fn render(&self) -> Frame {
        let interference = self.interference();
        let norm = interference.normalized(false);
        let ramp = gradient(&[BLACK, CYAN], LEVELS).unwrap();
        let background = [BLACK.r, BLACK.g, BLACK.b, 255];
        let colors: Vec<[u8; 4]> = norm
            .iter()
            .map(|&v| {
                let idx = ((v * (LEVELS - 1) as f32).round() as usize).min(LEVELS - 1);
                let c = ramp[idx];
                [c.r, c.g, c.b, 255]
            })
            .collect();
        field(SIZE, SIZE, colors, background)
    }
}

impl App for Moire {
    fn route(&self) -> &str {
        "moire"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("moire").emoji("🌀").category("math")
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "offset": self.set.offset,
            "angle": self.set.angle,
            "lattice": self.set.lattice,
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("moire.set", json!({ "key": "string", "value": "any" })),
            Verb::new("moire.reset", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "moire.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "moire.reset" => {
                self.set = Set::new();
                Outcome::ok(json!({}))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        self.set.to_json()
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

    #[test]
    fn set_validates() {
        let mut m = Moire::new();
        assert!(send(&mut m, "moire.set", json!({ "key": "offset", "value": -3 })).ok);
        assert!(!send(&mut m, "moire.set", json!({ "key": "offset", "value": 99 })).ok);
        assert!(send(&mut m, "moire.set", json!({ "key": "angle", "value": 180 })).ok);
        assert!(!send(&mut m, "moire.set", json!({ "key": "angle", "value": 45 })).ok);
        assert!(
            send(
                &mut m,
                "moire.set",
                json!({ "key": "lattice", "value": "hex" })
            )
            .ok
        );
        assert!(
            !send(
                &mut m,
                "moire.set",
                json!({ "key": "lattice", "value": "tri" })
            )
            .ok
        );
        assert!(!send(&mut m, "moire.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Moire::new();
        send(&mut a, "moire.set", json!({ "key": "offset", "value": -2 }));
        send(&mut a, "moire.set", json!({ "key": "angle", "value": 270 }));
        send(
            &mut a,
            "moire.set",
            json!({ "key": "lattice", "value": "hex" }),
        );
        let mut b = Moire::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut m = Moire::new();
        m.load(&json!({ "offset": 99, "angle": 45, "lattice": "tri" }));
        assert_eq!(m.set.offset, 4);
        assert_eq!(m.set.angle, 90);
        assert_eq!(m.set.lattice, "square");
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let m = Moire::new();
        let names: Vec<String> = m.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["moire.set", "moire.reset"]);
    }
    #[test]
    fn frame_renders() {
        let m = Moire::new();
        let state = m.state(&iden());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(rows.len(), SIZE);
        assert_eq!(rows[0].as_array().unwrap().len(), SIZE);
        assert!(state["frame"]["palette"].as_array().unwrap().len() >= 2);
    }
    #[test]
    fn knobs_change_the_frame() {
        let base = Moire::new().state(&iden())["frame"].clone();
        let mut a = Moire::new();
        send(&mut a, "moire.set", json!({ "key": "offset", "value": -5 }));
        assert_ne!(a.state(&iden())["frame"], base);
        let mut b = Moire::new();
        send(&mut b, "moire.set", json!({ "key": "angle", "value": 180 }));
        assert_ne!(b.state(&iden())["frame"], base);
        let mut c = Moire::new();
        send(
            &mut c,
            "moire.set",
            json!({ "key": "lattice", "value": "hex" }),
        );
        assert_ne!(c.state(&iden())["frame"], base);
    }
}
