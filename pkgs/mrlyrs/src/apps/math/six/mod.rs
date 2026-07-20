use crate::core::colors::{mix, named, Color, ALPHA, BLACK};
use crate::core::enums::Mode;
use crate::math::six::{
    cut, iso, paint as hex_paint, pro, triangles, Cell6d, FILL, GRID, LEFT, RIGHT, UP, VOID,
};
use crate::math::three::{carpet, census, net, void, xtree, ytree, ztree, Cell3d};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::Frame;
use crate::ui::raster::{paint, Tri};
use serde_json::{json, Value as Json};
use std::collections::HashMap;

const DESIGNS: [&str; 6] = ["carpet", "net", "xtree", "ytree", "ztree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MAX_CELLS: u32 = 16;
const VIEWS: [&str; 3] = ["iso", "pro", "cut"];
const SIZE: usize = 128;

struct Set {
    design: String,
    number: i64,
    level: i64,
    view: String,
    fill: String,
}

impl Set {
    fn new() -> Set {
        Set {
            design: "carpet".to_string(),
            number: 3,
            level: 2,
            view: "iso".to_string(),
            fill: "indigo".to_string(),
        }
    }
    fn cells(number: i64, level: i64) -> u32 {
        (number as u32).saturating_pow(level as u32)
    }
    fn int(value: &Json) -> Option<i64> {
        value
            .as_i64()
            .or_else(|| value.as_str().and_then(|s| s.parse::<i64>().ok()))
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "design" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if !DESIGNS.contains(&name) {
                    return Err("no such design");
                }
                self.design = name.to_string();
                Ok(json!(name))
            }
            "number" => {
                let n = Set::int(value).ok_or("value must be an integer")?;
                if !NUMBERS.contains(&n) {
                    return Err("number must be 3, 5, 7, or 9");
                }
                if Set::cells(n, self.level) > MAX_CELLS {
                    return Err("too many cells");
                }
                self.number = n;
                Ok(json!(n))
            }
            "level" => {
                let n = Set::int(value).ok_or("value must be an integer")?;
                if n < 1 {
                    return Err("level must be at least 1");
                }
                if Set::cells(self.number, n) > MAX_CELLS {
                    return Err("too many cells");
                }
                self.level = n;
                Ok(json!(n))
            }
            "view" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if !VIEWS.contains(&name) {
                    return Err("no such view");
                }
                self.view = name.to_string();
                Ok(json!(name))
            }
            "fill" => {
                let name = value.as_str().ok_or("value must be a string")?;
                named(name).map_err(|_| "unknown color")?;
                self.fill = name.to_string();
                Ok(json!(name))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "design": self.design,
            "number": self.number,
            "level": self.level,
            "view": self.view,
            "fill": self.fill,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(name) = value["design"].as_str() {
            if DESIGNS.contains(&name) {
                set.design = name.to_string();
            }
        }
        if let Some(n) = value["number"].as_i64() {
            if NUMBERS.contains(&n) {
                set.number = n;
            }
        }
        if let Some(n) = value["level"].as_i64() {
            if n >= 1 {
                set.level = n;
            }
        }
        if let Some(name) = value["view"].as_str() {
            if VIEWS.contains(&name) {
                set.view = name.to_string();
            }
        }
        if let Some(name) = value["fill"].as_str() {
            if named(name).is_ok() {
                set.fill = name.to_string();
            }
        }
        if Set::cells(set.number, set.level) > MAX_CELLS {
            let defaults = Set::new();
            set.level = defaults.level;
            if Set::cells(set.number, set.level) > MAX_CELLS {
                set.number = defaults.number;
            }
        }
        set
    }
}

pub struct Six {
    set: Set,
    dark: bool,
}

impl Default for Six {
    fn default() -> Six {
        Six::new()
    }
}

impl Six {
    pub fn new() -> Six {
        Six {
            set: Set::new(),
            dark: false,
        }
    }
    fn cube(&self) -> Cell3d {
        let n = self.set.number as usize;
        let l = self.set.level as usize;
        match self.set.design.as_str() {
            "carpet" => carpet(n, l),
            "net" => net(n, l),
            "xtree" => xtree(n, l),
            "ytree" => ytree(n, l),
            "ztree" => ztree(n, l),
            _ => void(n, l),
        }
        .unwrap()
    }
    fn hex(&self) -> Cell6d {
        let cube = self.cube();
        match self.set.view.as_str() {
            "pro" => pro(&cube),
            "cut" => cut(&cube),
            _ => iso(&cube),
        }
        .unwrap()
    }
    fn colors_map(&self) -> HashMap<u8, Vec<Color>> {
        let fill = named(&self.set.fill).unwrap();
        let mut map = HashMap::new();
        if self.set.view == "iso" {
            map.insert(UP, vec![fill]);
            map.insert(LEFT, vec![mix(fill, BLACK, 0.25).unwrap()]);
            map.insert(RIGHT, vec![mix(fill, BLACK, 0.5).unwrap()]);
        } else {
            map.insert(FILL, vec![fill]);
            map.insert(VOID, vec![ALPHA]);
        }
        map.insert(GRID, vec![ALPHA]);
        map
    }
    fn render(&self) -> Frame {
        let painted = hex_paint(self.hex(), Some(&self.colors_map()), Some(Mode::Type));
        let tris = triangles(&painted).unwrap();
        if tris.is_empty() {
            return paint(SIZE, SIZE, &[], crate::ui::frame::board(self.dark));
        }
        let xs = tris.iter().flat_map(|(p, _)| p.iter().map(|q| q.0));
        let ys = tris.iter().flat_map(|(p, _)| p.iter().map(|q| q.1));
        let (min_x, max_x) = (xs.clone().min().unwrap(), xs.max().unwrap());
        let (min_y, max_y) = (ys.clone().min().unwrap(), ys.max().unwrap());
        let span = (max_x - min_x).max(max_y - min_y).max(1) as f32;
        let scale = (SIZE as f32 * 0.9) / span;
        let (cx, cy) = ((min_x + max_x) as f32 / 2.0, (min_y + max_y) as f32 / 2.0);
        let mid = SIZE as f32 / 2.0;
        let raster: Vec<Tri> = tris
            .iter()
            .map(|(pts, rgba)| {
                let x: Vec<f32> = pts
                    .iter()
                    .map(|&(px, _)| mid + (px as f32 - cx) * scale)
                    .collect();
                let y: Vec<f32> = pts
                    .iter()
                    .map(|&(_, py)| mid + (py as f32 - cy) * scale)
                    .collect();
                Tri {
                    x: [x[0], x[1], x[2]],
                    y: [y[0], y[1], y[2]],
                    z: [1.0, 1.0, 1.0],
                    color: *rgba,
                }
            })
            .collect();
        paint(SIZE, SIZE, &raster, crate::ui::frame::board(self.dark))
    }
}

impl App for Six {
    fn route(&self) -> &str {
        "six"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("six").emoji("💠").category("math")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        let cube = self.cube();
        let side = cube.width().max(cube.height()).max(cube.depth());
        let filled = census::fills(&cube);
        let total = cube.width() * cube.height() * cube.depth();
        json!({
            "design": self.set.design,
            "index": DESIGNS.iter().position(|&d| d == self.set.design).unwrap_or(0),
            "count": DESIGNS.len(),
            "number": self.set.number,
            "level": self.set.level,
            "view": self.set.view,
            "fill": self.set.fill,
            "census": { "grid": side, "fill": filled, "void": total - filled },
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("six.page", json!({ "dir": "next | prev" })),
            Verb::new("six.set", json!({ "key": "string", "value": "any" })),
            Verb::new("six.reset", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "six.page" => {
                let idx = DESIGNS
                    .iter()
                    .position(|&d| d == self.set.design)
                    .unwrap_or(0);
                let next = match call.arg("dir").as_str() {
                    Some("next") => (idx + 1) % DESIGNS.len(),
                    Some("prev") => (idx + DESIGNS.len() - 1) % DESIGNS.len(),
                    _ => return Outcome::fail("dir must be next or prev"),
                };
                self.set.design = DESIGNS[next].to_string();
                Outcome::ok(json!({ "design": self.set.design }))
            }
            "six.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "six.reset" => {
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
        let mut s = Six::new();
        assert!(
            send(
                &mut s,
                "six.set",
                json!({ "key": "design", "value": "xtree" })
            )
            .ok
        );
        assert!(
            !send(
                &mut s,
                "six.set",
                json!({ "key": "design", "value": "sphere" })
            )
            .ok
        );
        assert!(send(&mut s, "six.set", json!({ "key": "level", "value": 1 })).ok);
        assert!(send(&mut s, "six.set", json!({ "key": "number", "value": 5 })).ok);
        assert!(!send(&mut s, "six.set", json!({ "key": "level", "value": 3 })).ok);
        assert!(send(&mut s, "six.set", json!({ "key": "view", "value": "pro" })).ok);
        assert!(
            !send(
                &mut s,
                "six.set",
                json!({ "key": "view", "value": "ortho" })
            )
            .ok
        );
        assert!(send(&mut s, "six.set", json!({ "key": "fill", "value": "pink" })).ok);
        assert!(
            !send(
                &mut s,
                "six.set",
                json!({ "key": "fill", "value": "beige" })
            )
            .ok
        );
        assert!(!send(&mut s, "six.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn page_cycles() {
        let mut s = Six::new();
        assert_eq!(s.set.design, "carpet");
        send(&mut s, "six.page", json!({ "dir": "next" }));
        assert_eq!(s.set.design, "net");
        assert_eq!(s.state(&iden())["index"], json!(1));
        assert_eq!(s.state(&iden())["count"], json!(6));
        send(&mut s, "six.page", json!({ "dir": "prev" }));
        send(&mut s, "six.page", json!({ "dir": "prev" }));
        assert_eq!(s.set.design, "void");
        assert_eq!(s.state(&iden())["index"], json!(5));
        assert!(!send(&mut s, "six.page", json!({ "dir": "sideways" })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Six::new();
        send(
            &mut a,
            "six.set",
            json!({ "key": "design", "value": "ztree" }),
        );
        send(&mut a, "six.set", json!({ "key": "view", "value": "cut" }));
        send(&mut a, "six.set", json!({ "key": "fill", "value": "mint" }));
        let mut b = Six::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut s = Six::new();
        s.load(&json!({ "design": "sphere", "number": 4, "level": 99, "view": "ortho" }));
        assert_eq!(s.set.design, "carpet");
        assert_eq!(s.set.number, 3);
        assert_eq!(s.set.level, 2);
        assert_eq!(s.set.view, "iso");
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let s = Six::new();
        let names: Vec<String> = s.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["six.page", "six.set", "six.reset"]);
    }
    #[test]
    fn frame_renders_every_view() {
        for v in VIEWS {
            let mut s = Six::new();
            send(&mut s, "six.set", json!({ "key": "view", "value": v }));
            let state = s.state(&iden());
            assert_eq!(
                state["frame"]["rows"].as_array().unwrap().len(),
                SIZE,
                "{v}"
            );
            assert!(
                state["frame"]["palette"].as_array().unwrap().len() > 1,
                "{v}"
            );
        }
    }
}
