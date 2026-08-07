use mrlycore::colors::ink;
use mrlycore::colors::named;
use mrlycore::{json, Json};
use mrlymath::space::{axis_edges, Pack, TURN};
use mrlymath::three::{carpet, census, net, quads, void, wires, xtree, ytree, ztree, Cell3d};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use std::f64::consts::TAU;

const DESIGNS: [&str; 6] = ["carpet", "net", "xtree", "ytree", "ztree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MAX_CELLS: u32 = 32;
const LIGHT_YAW: i64 = 72;
const LIGHT_PITCH: i64 = 28;
const BANDS: i64 = 4;

struct Set {
    design: String,
    number: i64,
    level: i64,
    fill: String,
    alpha: i64,
    edges: bool,
    wireframe: bool,
    axes: bool,
}

impl Set {
    fn new() -> Set {
        Set {
            design: "carpet".to_string(),
            number: 3,
            level: 2,
            fill: "teal".to_string(),
            alpha: 255,
            edges: false,
            wireframe: false,
            axes: false,
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
            "fill" => {
                let name = value.as_str().ok_or("value must be a string")?;
                named(name).map_err(|_| "unknown color")?;
                self.fill = name.to_string();
                Ok(json!(name))
            }
            "alpha" => {
                let n = Set::int(value).ok_or("value must be an integer")?;
                if !(32..=255).contains(&n) {
                    return Err("out of range");
                }
                self.alpha = n;
                Ok(json!(n))
            }
            "edges" | "wireframe" | "axes" => {
                let b = value.as_bool().ok_or("value must be a boolean")?;
                match key {
                    "edges" => self.edges = b,
                    "wireframe" => self.wireframe = b,
                    _ => self.axes = b,
                }
                Ok(json!(b))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "design": &self.design,
            "number": self.number,
            "level": self.level,
            "fill": &self.fill,
            "alpha": self.alpha,
            "edges": self.edges,
            "wireframe": self.wireframe,
            "axes": self.axes,
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
        if let Some(name) = value["fill"].as_str() {
            if named(name).is_ok() {
                set.fill = name.to_string();
            }
        }
        if let Some(n) = value["alpha"].as_i64() {
            if (32..=255).contains(&n) {
                set.alpha = n;
            }
        }
        if let Some(b) = value["edges"].as_bool() {
            set.edges = b;
        }
        if let Some(b) = value["wireframe"].as_bool() {
            set.wireframe = b;
        }
        if let Some(b) = value["axes"].as_bool() {
            set.axes = b;
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

pub struct Three {
    set: Set,
    dark: bool,
}

impl Default for Three {
    fn default() -> Three {
        Three::new()
    }
}

impl Three {
    pub fn new() -> Three {
        Three {
            set: Set::new(),
            dark: false,
        }
    }
    fn cell(&self) -> Cell3d {
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
    fn signature(&self) -> String {
        format!(
            "{}:{}:{}:w{}e{}a{}d{}",
            self.set.design,
            self.set.number,
            self.set.level,
            self.set.wireframe as u8,
            self.set.edges as u8,
            self.set.axes as u8,
            self.dark as u8
        )
    }
    fn shade(&self) -> Json {
        json!({ "program": "mesh", "route": "three", "mesh": self.signature() })
    }
}

impl App for Three {
    fn route(&self) -> &str {
        "three"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("three").emoji("🧱").category("math")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let cell = self.cell();
        let side = cell.width().max(cell.height()).max(cell.depth());
        let filled = census::fills(&cell);
        let total = cell.width() * cell.height() * cell.depth();
        json!({
            "design": &self.set.design,
            "index": DESIGNS.iter().position(|&d| d == self.set.design).unwrap_or(0),
            "count": DESIGNS.len(),
            "number": self.set.number,
            "level": self.set.level,
            "fill": &self.set.fill,
            "alpha": self.set.alpha,
            "edges": self.set.edges,
            "wireframe": self.set.wireframe,
            "axes": self.set.axes,
            "census": { "grid": side, "fill": filled, "void": total - filled },
            "shade": self.shade(),
        })
    }
    fn geometry(&self) -> Option<Vec<f32>> {
        let cell = self.cell();
        let mut pack = Pack::new();
        if !self.set.wireframe {
            for quad in quads(&cell) {
                pack.quad(quad.verts, quad.normal);
            }
        }
        if self.set.edges || self.set.wireframe {
            for [a, b] in wires(&cell) {
                pack.line(a, b, true, ink(self.dark));
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
        let fill = named(&self.set.fill).unwrap();
        let board = mrlycore::colors::board(self.dark);
        let mut u = vec![0.0; 24];
        u[4] = board[0] as f64 / 255.0;
        u[5] = board[1] as f64 / 255.0;
        u[6] = board[2] as f64 / 255.0;
        u[8] = fill.r as f64 / 255.0;
        u[9] = fill.g as f64 / 255.0;
        u[10] = fill.b as f64 / 255.0;
        u[11] = BANDS as f64;
        u[19] = self.set.alpha as f64 / 255.0;
        u[20] = LIGHT_YAW as f64 * rad;
        u[21] = LIGHT_PITCH as f64 * rad;
        Some(u.into_iter().map(|v| v as f32).collect())
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("three.page", json!({ "dir": "next | prev" })),
            Verb::new("three.set", json!({ "key": "string", "value": "any" })),
            Verb::new("three.reset", json!({})),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "three.page" => {
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
                Outcome::ok(json!({ "design": &self.set.design }))
            }
            "three.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "three.reset" => {
                self.set = Set::new();
                Outcome::ok(json!({}))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    #[test]
    fn set_validates() {
        let mut t = Three::new();
        assert!(
            send(
                &mut t,
                "three.set",
                json!({ "key": "design", "value": "xtree" })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "three.set",
                json!({ "key": "design", "value": "sphere" })
            )
            .ok
        );
        assert!(send(&mut t, "three.set", json!({ "key": "number", "value": 5 })).ok);
        assert!(!send(&mut t, "three.set", json!({ "key": "number", "value": 6 })).ok);
        assert!(!send(&mut t, "three.set", json!({ "key": "level", "value": 3 })).ok);
        assert!(
            send(
                &mut t,
                "three.set",
                json!({ "key": "fill", "value": "orange" })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "three.set",
                json!({ "key": "fill", "value": "beige" })
            )
            .ok
        );
        assert!(!send(&mut t, "three.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn page_cycles() {
        let mut t = Three::new();
        assert_eq!(t.set.design, "carpet");
        send(&mut t, "three.page", json!({ "dir": "next" }));
        assert_eq!(t.set.design, "net");
        assert_eq!(t.state(&iden(), None)["index"], json!(1));
        assert_eq!(t.state(&iden(), None)["count"], json!(6));
        send(&mut t, "three.page", json!({ "dir": "prev" }));
        send(&mut t, "three.page", json!({ "dir": "prev" }));
        assert_eq!(t.set.design, "void");
        assert_eq!(t.state(&iden(), None)["index"], json!(5));
        assert!(!send(&mut t, "three.page", json!({ "dir": "sideways" })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Three::new();
        send(
            &mut a,
            "three.set",
            json!({ "key": "design", "value": "ztree" }),
        );
        send(
            &mut a,
            "three.set",
            json!({ "key": "fill", "value": "purple" }),
        );
        let mut b = Three::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Three::new();
        t.load(&json!({ "design": "sphere", "number": 4, "level": 99 }));
        assert_eq!(t.set.design, "carpet");
        assert_eq!(t.set.number, 3);
        assert_eq!(t.set.level, 2);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = Three::new();
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["three.page", "three.set", "three.reset"]);
    }
    #[test]
    fn looks_validate_and_change_the_pack() {
        let mut t = Three::new();
        let plain = t.geometry().unwrap();
        assert!(
            send(
                &mut t,
                "three.set",
                json!({ "key": "edges", "value": true })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "three.set",
                json!({ "key": "edges", "value": "yes" })
            )
            .ok
        );
        let edged = t.geometry().unwrap();
        assert!(edged[1] > plain[1]);
        assert!(send(&mut t, "three.set", json!({ "key": "alpha", "value": 96 })).ok);
        assert!(!send(&mut t, "three.set", json!({ "key": "alpha", "value": 300 })).ok);
        assert!(send(&mut t, "three.set", json!({ "key": "axes", "value": true })).ok);
    }
    #[test]
    fn the_census_counts_cells() {
        let mut t = Three::new();
        send(&mut t, "three.reset", json!({}));
        let state = t.state(&iden(), None);
        assert!(state["census"]["fill"].as_u64().unwrap() > 0);
        assert!(state["census"]["grid"].as_u64().unwrap() > 0);
    }
    #[test]
    fn geometry_packs_the_cell() {
        let mut t = Three::new();
        let buf = t.geometry().unwrap();
        assert!(buf[0] > 0.0);
        assert_eq!(buf[1], 0.0);
        send(
            &mut t,
            "three.set",
            json!({ "key": "wireframe", "value": true }),
        );
        let wired = t.geometry().unwrap();
        assert_eq!(wired[0], 0.0);
        assert!(wired[1] > 0.0);
    }
    #[test]
    fn the_mesh_signature_tracks_only_geometry() {
        let mut t = Three::new();
        let sig = |t: &Three| t.shade()["mesh"].as_str().unwrap().to_string();
        let held = sig(&t);
        send(
            &mut t,
            "three.set",
            json!({ "key": "fill", "value": "orange" }),
        );
        send(&mut t, "three.set", json!({ "key": "alpha", "value": 96 }));
        assert_eq!(sig(&t), held);
        send(
            &mut t,
            "three.set",
            json!({ "key": "design", "value": "xtree" }),
        );
        assert_ne!(sig(&t), held);
        let treed = sig(&t);
        send(&mut t, "three.set", json!({ "key": "number", "value": 5 }));
        assert_ne!(sig(&t), treed);
        assert_eq!(t.shade()["program"], json!("mesh"));
        assert_eq!(t.shade()["route"], json!("three"));
        assert_eq!(t.uniforms().unwrap().len(), 24);
    }
}
