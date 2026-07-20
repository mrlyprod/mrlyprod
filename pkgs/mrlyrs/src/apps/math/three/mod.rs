use crate::core::colors::named;
use crate::math::space::{beam, Rig, Vec3, PAN_MAX, TURN};
use crate::math::three::{
    carpet, census, edge_graph, net, quads, void, xtree, ytree, ztree, Cell3d,
};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{board, ink, Frame};
use crate::ui::scene::{axes, axis_edges, Pack, Scene};
use serde_json::{json, Value as Json};
use std::f64::consts::TAU;

const DESIGNS: [&str; 6] = ["carpet", "net", "xtree", "ytree", "ztree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MAX_CELLS: u32 = 32;
const VIEWS: [&str; 3] = ["iso", "front", "top"];
const LIGHT_YAW: i64 = 72;
const LIGHT_PITCH: i64 = 28;
const BANDS: i64 = 4;

struct Set {
    design: String,
    number: i64,
    level: i64,
    view: String,
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
            view: "iso".to_string(),
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
            "design": self.design,
            "number": self.number,
            "level": self.level,
            "view": self.view,
            "fill": self.fill,
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

fn shades(base: [u8; 4], bands: i64) -> Vec<[u8; 4]> {
    (0..bands)
        .map(|k| {
            let t = (64 + 191 * k / (bands - 1)) as u32;
            let mix = |c: u8| (c as u32 * t / 255) as u8;
            [mix(base[0]), mix(base[1]), mix(base[2]), 255]
        })
        .collect()
}

pub struct Three {
    set: Set,
    rig: Rig,
    dark: bool,
    gpu: bool,
    detail: usize,
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
            rig: Three::posed("iso"),
            dark: false,
            gpu: false,
            detail: 96,
        }
    }
    fn posed(view: &str) -> Rig {
        let mut rig = Rig::new();
        rig.ortho = true;
        rig.view(view);
        rig
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
    fn scene(&self) -> Scene {
        let cell = self.cell();
        let light = beam(LIGHT_YAW, LIGHT_PITCH);
        let fill_c = named(&self.set.fill).unwrap();
        let base = [fill_c.r, fill_c.g, fill_c.b, 255];
        let bands = shades(base, BANDS);
        let alpha = self.set.alpha as u8;
        let mut scene = Scene::new();
        if !self.set.wireframe {
            for quad in quads(&cell) {
                let lit = (quad.normal.dot(light) + 1.0) * 0.5;
                let band = ((lit * BANDS as f32).floor() as i64).clamp(0, BANDS - 1);
                let mut color = bands[band as usize];
                color[3] = alpha;
                scene.quad(quad.verts, quad.normal, color);
            }
        }
        if self.set.edges || self.set.wireframe {
            for [a, b] in Three::lines(&cell) {
                scene.edge(a, b, ink(self.dark));
            }
        }
        if self.set.axes {
            axes(&mut scene, ink(self.dark));
        }
        scene
    }
    fn lines(cell: &Cell3d) -> Vec<[Vec3; 2]> {
        let grid = cell.types();
        let side = grid.shape[0].max(grid.shape[1]).max(grid.shape[2]) as f32;
        let half = side / 2.0;
        let point = |p: &[f64]| {
            Vec3::new(
                (p[0] as f32 - half) / half,
                (p[1] as f32 - half) / half,
                (p[2] as f32 - half) / half,
            )
        };
        let Ok(net) = edge_graph(cell) else {
            return Vec::new();
        };
        net.branches
            .iter()
            .map(|branch| {
                [
                    point(&net.nodes[branch.parent].position),
                    point(&net.nodes[branch.child].position),
                ]
            })
            .collect()
    }
    fn render_at(&self, size: usize) -> Frame {
        self.scene()
            .paint(&self.rig.camera(), size, board(self.dark))
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
        let rad = TAU / TURN as f64;
        let fill = named(&self.set.fill).unwrap();
        let board = crate::ui::frame::board(self.dark);
        let mut u = vec![0.0; 24];
        u[4] = board[0] as f64 / 255.0;
        u[5] = board[1] as f64 / 255.0;
        u[6] = board[2] as f64 / 255.0;
        u[8] = fill.r as f64 / 255.0;
        u[9] = fill.g as f64 / 255.0;
        u[10] = fill.b as f64 / 255.0;
        u[11] = BANDS as f64;
        u[13] = self.rig.yaw as f64 * rad;
        u[14] = self.rig.pitch as f64 * rad;
        u[15] = self.rig.dist as f64 / 4.0;
        u[16] = self.rig.pan[0] as f64 / 16.0;
        u[17] = self.rig.pan[1] as f64 / 16.0;
        u[18] = if self.rig.ortho { 1.0 } else { 0.0 };
        u[19] = self.set.alpha as f64 / 255.0;
        u[20] = LIGHT_YAW as f64 * rad;
        u[21] = LIGHT_PITCH as f64 * rad;
        json!({ "program": "mesh", "route": "three", "mesh": self.signature(), "uniforms": u })
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
        self.gpu = world["shared"]["settings"]["render"] == "gpu";
        self.detail = world["shared"]["settings"]["detail"]
            .as_f64()
            .unwrap_or(96.0) as usize;
    }
    fn state(&self, _iden: &Iden) -> Json {
        let cell = self.cell();
        let side = cell.width().max(cell.height()).max(cell.depth());
        let filled = census::fills(&cell);
        let total = cell.width() * cell.height() * cell.depth();
        json!({
            "design": self.set.design,
            "index": DESIGNS.iter().position(|&d| d == self.set.design).unwrap_or(0),
            "count": DESIGNS.len(),
            "number": self.set.number,
            "level": self.set.level,
            "view": self.set.view,
            "fill": self.set.fill,
            "alpha": self.set.alpha,
            "edges": self.set.edges,
            "wireframe": self.set.wireframe,
            "axes": self.set.axes,
            "camera": self.rig.to_json(),
            "census": { "grid": side, "fill": filled, "void": total - filled },
            "frame": self.render_at(if self.gpu { 96 } else { self.detail }).fact(),
            "shade": self.shade(),
        })
    }
    fn capture(&self, _iden: &Iden) -> Json {
        self.render_at(self.detail).fact()
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
            for [a, b] in Three::lines(&cell) {
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
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("three.page", json!({ "dir": "next | prev" })),
            Verb::new("three.turn", json!({ "dyaw": "int", "dpitch": "int" })),
            Verb::new("three.zoom", json!({ "dir": "in | out", "n": "int" })),
            Verb::new("three.pan", json!({ "dx": "int", "dy": "int" })),
            Verb::new("three.set", json!({ "key": "string", "value": "any" })),
            Verb::new("three.reset", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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
                Outcome::ok(json!({ "design": self.set.design }))
            }
            "three.turn" => {
                let dyaw = call.arg("dyaw").as_i64().unwrap_or(0);
                let dpitch = call.arg("dpitch").as_i64().unwrap_or(0);
                if dyaw.abs() > TURN || dpitch.abs() > TURN {
                    return Outcome::fail("delta out of range");
                }
                self.rig.orbit(dyaw, dpitch);
                Outcome::ok(json!({ "yaw": self.rig.yaw, "pitch": self.rig.pitch }))
            }
            "three.zoom" => {
                let n = match Three::count(call, 24) {
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
            "three.pan" => {
                let dx = call.arg("dx").as_i64().unwrap_or(0);
                let dy = call.arg("dy").as_i64().unwrap_or(0);
                if dx.abs() > 2 * PAN_MAX || dy.abs() > 2 * PAN_MAX {
                    return Outcome::fail("delta out of range");
                }
                self.rig.pan(dx, dy);
                Outcome::ok(json!({ "pan": self.rig.pan }))
            }
            "three.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                if key == "ortho" {
                    let Some(b) = call.arg("value").as_bool() else {
                        return Outcome::fail("value must be a boolean");
                    };
                    self.rig.ortho = b;
                    return Outcome::ok(json!({ "key": key, "value": b }));
                }
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        if key == "view" {
                            let ortho = self.rig.ortho;
                            self.rig = Three::posed(&self.set.view);
                            self.rig.ortho = ortho;
                        }
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            "three.reset" => {
                self.set = Set::new();
                self.rig = Three::posed("iso");
                Outcome::ok(json!({}))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "camera": self.rig.to_json(),
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.rig = Three::posed(&self.set.view);
        self.rig.load(&state["camera"]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, send};

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
                json!({ "key": "view", "value": "top" })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "three.set",
                json!({ "key": "view", "value": "side" })
            )
            .ok
        );
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
        assert_eq!(t.state(&iden())["index"], json!(1));
        assert_eq!(t.state(&iden())["count"], json!(6));
        send(&mut t, "three.page", json!({ "dir": "prev" }));
        send(&mut t, "three.page", json!({ "dir": "prev" }));
        assert_eq!(t.set.design, "void");
        assert_eq!(t.state(&iden())["index"], json!(5));
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
            json!({ "key": "view", "value": "front" }),
        );
        send(
            &mut a,
            "three.set",
            json!({ "key": "fill", "value": "purple" }),
        );
        let mut b = Three::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Three::new();
        t.load(&json!({ "design": "sphere", "number": 4, "level": 99, "view": "side" }));
        assert_eq!(t.set.design, "carpet");
        assert_eq!(t.set.number, 3);
        assert_eq!(t.set.level, 2);
        assert_eq!(t.set.view, "iso");
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = Three::new();
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "three.page",
                "three.turn",
                "three.zoom",
                "three.pan",
                "three.set",
                "three.reset"
            ]
        );
    }
    #[test]
    fn gestures_drive_the_rig_and_views_snap_it_back() {
        let mut t = Three::new();
        assert_eq!((t.rig.yaw, t.rig.pitch), (32, 20));
        assert!(t.rig.ortho);
        send(&mut t, "three.turn", json!({ "dyaw": 8, "dpitch": -4 }));
        assert_eq!((t.rig.yaw, t.rig.pitch), (40, 16));
        send(&mut t, "three.zoom", json!({ "dir": "out", "n": 4 }));
        assert_eq!(t.rig.dist, 16);
        send(&mut t, "three.pan", json!({ "dx": 2, "dy": 3 }));
        assert_eq!(t.rig.pan, [2, 3]);
        send(
            &mut t,
            "three.set",
            json!({ "key": "view", "value": "top" }),
        );
        assert_eq!((t.rig.yaw, t.rig.pitch, t.rig.pan), (0, 64, [0, 0]));
        assert!(
            send(
                &mut t,
                "three.set",
                json!({ "key": "ortho", "value": false })
            )
            .ok
        );
        assert!(!t.rig.ortho);
        let mut b = Three::new();
        b.load(&t.save());
        assert_eq!(b.state(&iden()), t.state(&iden()));
    }
    #[test]
    fn looks_validate_and_change_the_frame() {
        let mut t = Three::new();
        let plain = t.state(&iden())["frame"].clone();
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
        let edged = t.state(&iden())["frame"].clone();
        assert_ne!(plain, edged);
        assert!(
            send(
                &mut t,
                "three.set",
                json!({ "key": "wireframe", "value": true })
            )
            .ok
        );
        let wired = t.state(&iden())["frame"].clone();
        assert_ne!(edged, wired);
        assert!(send(&mut t, "three.set", json!({ "key": "alpha", "value": 96 })).ok);
        assert!(!send(&mut t, "three.set", json!({ "key": "alpha", "value": 300 })).ok);
        assert!(send(&mut t, "three.set", json!({ "key": "axes", "value": true })).ok);
    }
    #[test]
    fn frame_renders_a_lit_mesh() {
        let mut t = Three::new();
        send(&mut t, "three.reset", json!({}));
        let state = t.state(&iden());
        assert_eq!(state["frame"]["rows"].as_array().unwrap().len(), 96);
        assert!(state["frame"]["palette"].as_array().unwrap().len() > 2);
        assert!(state["census"]["fill"].as_u64().unwrap() > 0);
    }
    #[test]
    fn gpu_mode_keeps_a_small_real_frame() {
        let mut t = Three::new();
        t.wear(&json!({ "shared": { "settings": { "render": "gpu", "detail": 128.0 } } }));
        let gpu = t.state(&iden())["frame"].clone();
        assert_eq!(gpu["width"], json!(96));
        assert_eq!(gpu["rows"].as_array().unwrap().len(), 96);
        assert!(gpu["palette"].as_array().unwrap().len() > 2);
        let shot = t.capture(&iden());
        assert_eq!(shot["width"], json!(128));
        assert_eq!(shot["rows"].as_array().unwrap().len(), 128);
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
        send(&mut t, "three.turn", json!({ "dyaw": 8, "dpitch": -4 }));
        send(&mut t, "three.zoom", json!({ "dir": "out", "n": 4 }));
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
        assert_eq!(t.shade()["uniforms"].as_array().unwrap().len(), 24);
    }
}
