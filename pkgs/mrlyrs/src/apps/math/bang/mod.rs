use crate::core::colors::named;
use crate::core::tensor::Tensor;
use crate::math::bang::{bang, factory, universe_codes};
use crate::math::space::{beam, project, view, Vec3};
use crate::math::three::{quads, Cell3d};
use crate::math::two::Cell2d;
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{field, Frame};
use crate::ui::raster::{paint, Tri};
use serde_json::{json, Value as Json};

const BASE: usize = 2;
const NUMBER: usize = 3;
const LEVEL_1D: usize = 6;
const LEVEL_2D: usize = 5;
const LEVEL_3D: usize = 2;
const FILL: &str = "yellow";
const VOID: &str = "black";
const SIZE_3D: usize = 128;
const LIGHT_YAW: i64 = 72;
const LIGHT_PITCH: i64 = 28;
const VIEW_YAW: i64 = 32;
const VIEW_PITCH: i64 = 20;
const BANDS: i64 = 4;
const DIST: f32 = 3.0;

fn shades(base: [u8; 4], bands: i64) -> Vec<[u8; 4]> {
    (0..bands)
        .map(|k| {
            let t = (64 + 191 * k / (bands - 1)) as u32;
            let mix = |c: u8| (c as u32 * t / 255) as u8;
            [mix(base[0]), mix(base[1]), mix(base[2]), 255]
        })
        .collect()
}

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
    fn render_1d(&self, tensor: Tensor) -> Frame {
        let side = tensor.shape[0];
        let fill_c = named(FILL).unwrap();
        let void_c = named(VOID).unwrap();
        let fill = [fill_c.r, fill_c.g, fill_c.b, 255];
        let empty = [void_c.r, void_c.g, void_c.b, 255];
        let colors: Vec<[u8; 4]> = tensor
            .bytes()
            .iter()
            .map(|&v| if v == 1 { fill } else { empty })
            .collect();
        field(side, 1, colors, empty)
    }
    fn render_2d(&self, tensor: Tensor) -> Frame {
        let cell = Cell2d::new(tensor);
        let (w, h) = (cell.width(), cell.height());
        let fill_c = named(FILL).unwrap();
        let void_c = named(VOID).unwrap();
        let fill = [fill_c.r, fill_c.g, fill_c.b, 255];
        let empty = [void_c.r, void_c.g, void_c.b, 255];
        let colors: Vec<[u8; 4]> = cell
            .types()
            .bytes()
            .iter()
            .map(|&v| if v == 1 { fill } else { empty })
            .collect();
        field(w, h, colors, empty)
    }
    fn render_3d(&self, tensor: Tensor) -> Frame {
        let cell = Cell3d::new(tensor);
        let eye = view(VIEW_YAW, VIEW_PITCH);
        let light = beam(LIGHT_YAW, LIGHT_PITCH);
        let focal = SIZE_3D as f32 * 1.2;
        let fill_c = named(FILL).unwrap();
        let base = [fill_c.r, fill_c.g, fill_c.b, 255];
        let bands = shades(base, BANDS);
        let mut tris = Vec::new();
        for quad in quads(&cell) {
            let n_eye = eye.apply(quad.normal);
            let us: Vec<Vec3> = quad.verts.iter().map(|&v| eye.apply(v)).collect();
            if n_eye.x * us[0].x + n_eye.y * us[0].y + n_eye.z * (us[0].z - DIST) >= 0.0 {
                continue;
            }
            let lit = (quad.normal.dot(light) + 1.0) * 0.5;
            let band = ((lit * BANDS as f32).floor() as i64).clamp(0, BANDS - 1);
            let color = bands[band as usize];
            for tri in [[0usize, 1, 2], [0, 2, 3]] {
                let ps: Option<Vec<[f32; 3]>> = tri
                    .iter()
                    .map(|&idx| project(us[idx], DIST, SIZE_3D as f32, focal))
                    .collect();
                let Some(ps) = ps else { continue };
                tris.push(Tri {
                    x: [ps[0][0], ps[1][0], ps[2][0]],
                    y: [ps[0][1], ps[1][1], ps[2][1]],
                    z: [ps[0][2], ps[1][2], ps[2][2]],
                    color,
                });
            }
        }
        paint(SIZE_3D, SIZE_3D, &tris, crate::ui::frame::board(self.dark))
    }
    fn render(&self) -> Frame {
        let tensor = self.tensor();
        match self.dimension {
            1 => self.render_1d(tensor),
            3 => self.render_3d(tensor),
            _ => self.render_2d(tensor),
        }
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
    fn state(&self, _iden: &Iden) -> Json {
        let code = self.code();
        let design = bang(self.dimension).design(code);
        json!({
            "dimension": self.dimension,
            "base": BASE,
            "index": self.index,
            "count": self.codes().len(),
            "name": factory::name(code, self.dimension, BASE),
            "code": code.to_string(),
            "degree": design.degree(),
            "anf": design.anf(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("bang.page", json!({ "dir": "next | prev" })),
            Verb::new("bang.set", json!({ "key": "string", "value": "any" })),
            Verb::new("bang.reset", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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
    use crate::os::kernel::testkit::{iden, send};

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
        assert_eq!(b.state(&iden()), a.state(&iden()));
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
    fn frame_renders_the_grid() {
        let mut b = Bang::new();
        for d in [1i64, 2, 3] {
            send(
                &mut b,
                "bang.set",
                json!({ "key": "dimension", "value": d }),
            );
            let state = b.state(&iden());
            let rows = state["frame"]["rows"].as_array().unwrap();
            assert!(!rows.is_empty());
            assert!(!rows[0].as_array().unwrap().is_empty());
        }
    }
}
