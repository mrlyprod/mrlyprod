#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use mrlycore::colors::ink;
use mrlycore::colors::named;
use mrlycore::colors::NAMES;
use mrlycore::tile;
use mrlycore::{json, Json};
use mrlymath::graph::{census, roles, Network, Role};
use mrlymath::space::{axis_edges, Pack, Vec3, TURN};
use mrlymath::three::{
    carpet, core_graph, edge_graph, net, quads, tunnel_graph, void, xtree, ytree, ztree, Cell3d,
};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use std::f64::consts::TAU;

const DESIGNS: [&str; 6] = ["carpet", "net", "xtree", "ytree", "ztree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const KINDS: [&str; 3] = ["core", "tunnel", "edge"];
const MAX_CELLS: usize = 27;
const MILLI: i64 = 1000;
const FEMTO: i64 = 1_000_000_000_000_000;
const ARM: f32 = 0.35;
const LIGHT_YAW: i64 = 72;
const LIGHT_PITCH: i64 = 28;
const BANDS: i64 = 4;

fn tint(role: Role) -> [u8; 4] {
    let color = match role {
        Role::Alone => named("cyan"),
        Role::Tip => named("orange"),
        Role::Through => named("gray"),
        Role::Junction => named("purple"),
    }
    .unwrap();
    [color.r, color.g, color.b, 255]
}

struct Set {
    design: String,
    number: i64,
    level: i64,
    kind: String,
    fill: String,
    alpha: i64,
    particles: bool,
    segments: bool,
    ghost: bool,
    axes: bool,
}

impl Set {
    fn new() -> Set {
        Set {
            design: "carpet".to_string(),
            number: 3,
            level: 2,
            kind: "core".to_string(),
            fill: "teal".to_string(),
            alpha: 96,
            particles: true,
            segments: true,
            ghost: false,
            axes: false,
        }
    }
    fn fits(number: i64, level: i64) -> bool {
        tile::size(number, level).is_some_and(|side| side <= MAX_CELLS)
    }
    fn depth(number: i64) -> i64 {
        (1i64..)
            .take_while(|&level| Set::fits(number, level))
            .last()
            .unwrap_or(1)
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
                if !Set::fits(n, self.level) {
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
                if !Set::fits(self.number, n) {
                    return Err("too many cells");
                }
                self.level = n;
                Ok(json!(n))
            }
            "kind" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if !KINDS.contains(&name) {
                    return Err("no such kind");
                }
                self.kind = name.to_string();
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
            "particles" | "segments" | "ghost" | "axes" => {
                let b = value.as_bool().ok_or("value must be a boolean")?;
                match key {
                    "particles" => self.particles = b,
                    "segments" => self.segments = b,
                    "ghost" => self.ghost = b,
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
            "kind": &self.kind,
            "fill": &self.fill,
            "alpha": self.alpha,
            "particles": self.particles,
            "segments": self.segments,
            "ghost": self.ghost,
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
        if let Some(name) = value["kind"].as_str() {
            if KINDS.contains(&name) {
                set.kind = name.to_string();
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
        if let Some(b) = value["particles"].as_bool() {
            set.particles = b;
        }
        if let Some(b) = value["segments"].as_bool() {
            set.segments = b;
        }
        if let Some(b) = value["ghost"].as_bool() {
            set.ghost = b;
        }
        if let Some(b) = value["axes"].as_bool() {
            set.axes = b;
        }
        if !Set::fits(set.number, set.level) {
            let defaults = Set::new();
            set.level = defaults.level;
            if !Set::fits(set.number, set.level) {
                set.number = defaults.number;
            }
        }
        set
    }
}

/// The skeleton viewer: one of six designs, one of three extractors, and the look the network wears.
pub struct Graph {
    set: Set,
    dark: bool,
}

impl Default for Graph {
    fn default() -> Graph {
        Graph::new()
    }
}

impl Graph {
    /// Opens the viewer on the core network of a 3 by 3 carpet two levels deep, drawn in glass teal.
    pub fn new() -> Graph {
        Graph {
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
    fn network(&self, cell: &Cell3d) -> Network {
        match self.set.kind.as_str() {
            "tunnel" => tunnel_graph(cell),
            "edge" => edge_graph(cell),
            _ => core_graph(cell),
        }
        .unwrap()
    }
    fn signature(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:p{}s{}g{}a{}d{}",
            self.set.design,
            self.set.number,
            self.set.level,
            self.set.kind,
            self.set.fill,
            self.set.particles as u8,
            self.set.segments as u8,
            self.set.ghost as u8,
            self.set.axes as u8,
            self.dark as u8
        )
    }
    fn shade(&self) -> Json {
        json!({ "program": "mesh", "route": "graph", "mesh": self.signature() })
    }
}

impl App for Graph {
    fn route(&self) -> &str {
        "graph"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("graph").emoji("🕸️").category("math")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let cell = self.cell();
        let network = self.network(&cell);
        let counted = census::census(&network);
        json!({
            "design": &self.set.design,
            "index": DESIGNS.iter().position(|&d| d == self.set.design).unwrap_or(0),
            "count": DESIGNS.len(),
            "number": self.set.number,
            "level": self.set.level,
            "kind": &self.set.kind,
            "fill": &self.set.fill,
            "alpha": self.set.alpha,
            "particles": self.set.particles,
            "segments": self.set.segments,
            "ghost": self.set.ghost,
            "axes": self.set.axes,
            "census": {
                "nodes": counted.nodes,
                "branches": counted.branches,
                "tips": counted.tips,
                "junctions": counted.junctions,
                "components": counted.components,
                "length": (counted.total_length * MILLI as f64) as i64,
                "dimension": (counted.fractal_dimension * FEMTO as f64) as i64,
            },
            "shade": self.shade(),
        })
    }
    fn geometry(&self) -> Option<Vec<f32>> {
        let cell = self.cell();
        let side = cell.width().max(cell.height()).max(cell.depth()) as f32;
        let half = side / 2.0;
        let network = self.network(&cell);
        let point = |p: &[f64]| {
            Vec3::new(
                (p[2] as f32 - half) / half,
                (p[1] as f32 - half) / half,
                (p[0] as f32 - half) / half,
            )
        };
        let mut pack = Pack::new();
        if self.set.ghost {
            for quad in quads(&cell) {
                pack.quad(quad.verts, quad.normal);
            }
        }
        if self.set.segments {
            let fill = named(&self.set.fill).unwrap();
            let paint = [fill.r, fill.g, fill.b, 255];
            for branch in &network.branches {
                pack.line(
                    point(&network.nodes[branch.parent].position),
                    point(&network.nodes[branch.child].position),
                    true,
                    paint,
                );
            }
        }
        if self.set.particles {
            let arm = ARM / half;
            for (node, role) in network.nodes.iter().zip(roles(&network)) {
                let c = point(&node.position);
                let paint = tint(role);
                pack.line(
                    Vec3::new(c.x - arm, c.y, c.z),
                    Vec3::new(c.x + arm, c.y, c.z),
                    true,
                    paint,
                );
                pack.line(
                    Vec3::new(c.x, c.y - arm, c.z),
                    Vec3::new(c.x, c.y + arm, c.z),
                    true,
                    paint,
                );
                pack.line(
                    Vec3::new(c.x, c.y, c.z - arm),
                    Vec3::new(c.x, c.y, c.z + arm),
                    true,
                    paint,
                );
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
            Verb::new("graph.page", json!({ "dir": "next | prev" })),
            Verb::new(
                "graph.set",
                json!({
                    "key": {
                        "design": DESIGNS.join(" | "),
                        "number": NUMBERS.map(|n| n.to_string()).join(" | "),
                        "level": format!("int 1..{}", Set::depth(self.set.number)),
                        "kind": KINDS.join(" | "),
                        "fill": NAMES.join(" | "),
                        "alpha": "int 32..255",
                        "particles": "bool",
                        "segments": "bool",
                        "ghost": "bool",
                        "axes": "bool",
                    },
                    "value": "of key",
                }),
            ),
            Verb::new("graph.reset", json!({})),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "graph.page" => {
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
            "graph.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "graph.reset" => {
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

    fn dial(graph: &mut Graph, key: &str, value: Json) -> bool {
        send(graph, "graph.set", json!({ "key": key, "value": value })).ok
    }

    fn card(graph: &Graph) -> Json {
        graph.state(&iden(), None)["census"].clone()
    }

    #[test]
    fn set_validates() {
        let mut g = Graph::new();
        assert!(dial(&mut g, "design", json!("xtree")));
        assert!(!dial(&mut g, "design", json!("sphere")));
        assert!(dial(&mut g, "number", json!(5)));
        assert!(!dial(&mut g, "number", json!(6)));
        assert!(!dial(&mut g, "level", json!(3)));
        assert!(dial(&mut g, "kind", json!("tunnel")));
        assert!(!dial(&mut g, "kind", json!("spine")));
        assert!(dial(&mut g, "fill", json!("orange")));
        assert!(!dial(&mut g, "fill", json!("beige")));
        assert!(!dial(&mut g, "volume", json!(1)));
        assert!(!dial(&mut g, "level", json!(4294967296i64)));
        assert_eq!(g.set.level, 2);
    }

    #[test]
    fn the_bones_ride_the_solid_axes() {
        let mut g = Graph::new();
        assert!(dial(&mut g, "design", json!("xtree")));
        assert!(dial(&mut g, "level", json!(1)));
        assert!(dial(&mut g, "particles", json!(false)));
        let buf = g.geometry().unwrap();
        let ends = buf[1] as usize / 8;
        let spread = |axis: usize| {
            let mut seen: Vec<i64> = (0..ends)
                .map(|i| (buf[2 + i * 8 + axis] * 1000.0) as i64)
                .collect();
            seen.sort_unstable();
            seen.dedup();
            seen.len()
        };
        assert_eq!([spread(0), spread(1), spread(2)], [3, 2, 2]);
    }

    #[test]
    fn page_cycles() {
        let mut g = Graph::new();
        assert_eq!(g.set.design, "carpet");
        send(&mut g, "graph.page", json!({ "dir": "next" }));
        assert_eq!(g.set.design, "net");
        assert_eq!(g.state(&iden(), None)["index"], json!(1));
        assert_eq!(g.state(&iden(), None)["count"], json!(6));
        send(&mut g, "graph.page", json!({ "dir": "prev" }));
        send(&mut g, "graph.page", json!({ "dir": "prev" }));
        assert_eq!(g.set.design, "void");
        assert_eq!(g.state(&iden(), None)["index"], json!(5));
        assert!(!send(&mut g, "graph.page", json!({ "dir": "sideways" })).ok);
    }

    #[test]
    fn save_load_round_trips() {
        let mut a = Graph::new();
        dial(&mut a, "design", json!("ztree"));
        dial(&mut a, "kind", json!("edge"));
        dial(&mut a, "fill", json!("purple"));
        let mut b = Graph::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }

    #[test]
    fn load_survives_garbage() {
        let mut g = Graph::new();
        g.load(&json!({
            "settings": {
                "design": "sphere",
                "number": 4,
                "level": 99,
                "kind": "spine",
                "fill": "beige",
                "alpha": 900,
            }
        }));
        assert_eq!(g.set.design, "carpet");
        assert_eq!(g.set.number, 3);
        assert_eq!(g.set.level, 2);
        assert_eq!(g.set.kind, "core");
        assert_eq!(g.set.fill, "teal");
        assert_eq!(g.set.alpha, 96);
    }

    #[test]
    fn a_wrapped_level_never_reaches_the_grid() {
        let mut g = Graph::new();
        assert!(!dial(&mut g, "level", json!(4294967298i64)));
        assert!(!dial(&mut g, "level", json!("4294967298")));
        assert!(!dial(&mut g, "level", json!(33)));
        assert!(!dial(&mut g, "level", json!(64)));
        assert_eq!(g.set.level, 2);
        g.load(&json!({ "settings": { "number": 3, "level": 4294967298i64 } }));
        assert_eq!(g.set.level, 2);
    }

    #[test]
    fn actions_offer_the_natural_verbs() {
        let g = Graph::new();
        let names: Vec<String> = g.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["graph.page", "graph.set", "graph.reset"]);
    }

    #[test]
    fn the_census_counts_the_network() {
        let g = Graph::new();
        let counted = card(&g);
        assert_eq!(counted["nodes"], json!(400));
        assert!(counted["branches"].as_u64().unwrap() > 0);
        assert_eq!(counted["components"], json!(1));
    }

    #[test]
    fn the_tunnel_of_a_carpet_is_a_star() {
        let mut g = Graph::new();
        assert!(dial(&mut g, "level", json!(1)));
        assert!(dial(&mut g, "kind", json!("tunnel")));
        let counted = card(&g);
        assert_eq!(counted["nodes"], json!(7));
        assert_eq!(counted["junctions"], json!(1));
        assert_eq!(counted["tips"], json!(6));
    }

    #[test]
    fn the_void_is_all_alone() {
        let mut g = Graph::new();
        assert!(dial(&mut g, "design", json!("void")));
        assert!(dial(&mut g, "level", json!(1)));
        let counted = card(&g);
        assert_eq!(counted["nodes"], json!(9));
        assert_eq!(counted["branches"], json!(0));
        assert_eq!(counted["components"], json!(9));
        let cell = g.cell();
        assert!(roles(&g.network(&cell)).iter().all(|&r| r == Role::Alone));
    }

    #[test]
    fn an_empty_pack_is_legal() {
        let mut g = Graph::new();
        for key in ["particles", "segments", "ghost", "axes"] {
            assert!(dial(&mut g, key, json!(false)));
        }
        assert_eq!(g.geometry().unwrap(), vec![0.0, 0.0]);
    }

    #[test]
    fn looks_validate_and_change_the_pack() {
        let mut g = Graph::new();
        let whole = g.geometry().unwrap();
        assert_eq!(whole[0], 0.0);
        assert!(whole[1] > 0.0);
        assert!(dial(&mut g, "particles", json!(false)));
        let bare = g.geometry().unwrap();
        assert!(bare[1] > 0.0 && bare[1] < whole[1]);
        assert!(dial(&mut g, "ghost", json!(true)));
        assert!(g.geometry().unwrap()[0] > 0.0);
        assert!(!dial(&mut g, "particles", json!("yes")));
        assert!(dial(&mut g, "alpha", json!(96)));
        assert!(!dial(&mut g, "alpha", json!(300)));
    }

    #[test]
    fn the_mesh_signature_tracks_only_geometry() {
        let mut g = Graph::new();
        let sig = |g: &Graph| g.shade()["mesh"].as_str().unwrap().to_string();
        let held = sig(&g);
        dial(&mut g, "alpha", json!(128));
        assert_eq!(sig(&g), held);
        dial(&mut g, "fill", json!("orange"));
        assert_ne!(sig(&g), held);
        let painted = sig(&g);
        dial(&mut g, "design", json!("xtree"));
        assert_ne!(sig(&g), painted);
        let treed = sig(&g);
        dial(&mut g, "kind", json!("tunnel"));
        assert_ne!(sig(&g), treed);
        assert_eq!(g.shade()["program"], json!("mesh"));
        assert_eq!(g.shade()["route"], json!("graph"));
        assert_eq!(g.uniforms().unwrap().len(), 24);
    }

    #[test]
    fn the_census_crosses_at_its_scales() {
        let g = Graph::new();
        let counted = card(&g);
        let branches = counted["branches"].as_i64().unwrap();
        assert_eq!(counted["length"], json!(branches * MILLI));
        assert_eq!(counted["dimension"], json!(2_253_563_989_299_303i64));
    }
}
