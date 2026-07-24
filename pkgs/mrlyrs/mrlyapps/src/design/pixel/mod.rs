use mrlycore::colors::ROLLABLE;
use mrlycore::rng::Rng;
use mrlycore::tensor::Tensor;
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{motif_tile, solid_tile, Frame, Layer, TileSet};
use serde_json::{json, Value as Json};

const DESIGNS: [&str; 5] = ["carpet", "net", "vtree", "htree", "solid"];

struct Set {
    width: i64,
    height: i64,
    tile: i64,
    design: String,
}

impl Set {
    fn new() -> Set {
        Set {
            width: 24,
            height: 24,
            tile: 3,
            design: "carpet".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "width" | "height" | "tile" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "width" => (4, 64),
                    "height" => (4, 64),
                    _ => (1, 8),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "width" => self.width = n,
                    "height" => self.height = n,
                    _ => self.tile = n,
                }
                Ok(json!(n))
            }
            "design" => {
                let d = value.as_str().ok_or("value must be a string")?;
                if !DESIGNS.contains(&d) {
                    return Err("no such option");
                }
                self.design = d.to_string();
                Ok(json!(d))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "width": self.width,
            "height": self.height,
            "tile": self.tile,
            "design": self.design,
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

pub struct Pixel {
    set: Set,
    rng: Rng,
    seed: u64,
    steps: u64,
    canvas: Vec<bool>,
    ink_color: [u8; 4],
    dark: bool,
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::new()
    }
}

impl Pixel {
    pub fn new() -> Pixel {
        let mut pixel = Pixel {
            set: Set::new(),
            rng: Rng::new(0),
            seed: 0,
            steps: 0,
            canvas: Vec::new(),
            ink_color: [255, 255, 255, 255],
            dark: false,
        };
        pixel.reset(0);
        pixel
    }
    fn width(&self) -> usize {
        self.set.width as usize
    }
    fn height(&self) -> usize {
        self.set.height as usize
    }
    fn size(&self) -> usize {
        self.width() * self.height()
    }
    fn painted(&self) -> u64 {
        self.canvas.iter().filter(|&&p| p).count() as u64
    }
    fn palette(&mut self) -> [u8; 4] {
        let c = ROLLABLE[self.rng.below(ROLLABLE.len())];
        [c.r, c.g, c.b, 255]
    }
    fn parse_points(&self, value: &Json) -> Result<Vec<(usize, usize)>, &'static str> {
        let arr = value
            .as_array()
            .ok_or("points must be an array of [x, y] pairs")?;
        let (w, h) = (self.width() as i64, self.height() as i64);
        let mut out = Vec::with_capacity(arr.len());
        for p in arr {
            let pair = p
                .as_array()
                .ok_or("points must be an array of [x, y] pairs")?;
            if pair.len() != 2 {
                return Err("points must be an array of [x, y] pairs");
            }
            let x = pair[0]
                .as_i64()
                .ok_or("points must be an array of [x, y] pairs")?;
            let y = pair[1]
                .as_i64()
                .ok_or("points must be an array of [x, y] pairs")?;
            if !(0..w).contains(&x) || !(0..h).contains(&y) {
                return Err("point out of bounds");
            }
            out.push((x as usize, y as usize));
        }
        Ok(out)
    }
    fn canvas_facts(&self) -> Vec<Vec<bool>> {
        let w = self.width();
        (0..self.height())
            .map(|r| (0..w).map(|c| self.canvas[r * w + c]).collect())
            .collect()
    }
    fn ids(&self) -> Tensor {
        let (w, h) = (self.width(), self.height());
        let mut grid = Tensor::new(vec![h, w]);
        for r in 0..h {
            for c in 0..w {
                if self.canvas[r * w + c] {
                    grid.set(&[r, c], 1);
                }
            }
        }
        grid
    }
    fn tileset(&self) -> TileSet {
        let k = self.set.tile as usize;
        let clear = [0, 0, 0, 0];
        let d = self.set.design.as_str();
        TileSet::new(
            k,
            vec![
                solid_tile(k, clear),
                motif_tile(d, k, self.ink_color, clear),
            ],
        )
    }
    fn render(&self) -> Frame {
        let k = self.set.tile as usize;
        let mut frame = Frame::new(
            self.width() * k,
            self.height() * k,
            mrlyui::frame::board(self.dark),
        );
        frame.push(Layer::Tiles {
            ids: self.ids(),
            set: self.tileset(),
        });
        frame
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.steps = 0;
        self.canvas = vec![false; self.size()];
        self.ink_color = self.palette();
    }
    fn clear(&mut self) {
        self.canvas = vec![false; self.size()];
    }
}

impl App for Pixel {
    fn route(&self) -> &str {
        "pixel"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("pixel").emoji("🎨").category("design")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "score": 0,
            "steps": self.steps,
            "over": false,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "canvas": self.canvas_facts(),
            "painted": self.painted(),
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("pixel.stroke", json!({ "points": "[[x, y], ...]" })),
            Verb::new("pixel.clear", json!({})),
            Verb::new("pixel.reset", json!({ "seed": "int" })),
            Verb::new("pixel.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "pixel.stroke" => match self.parse_points(call.arg("points")) {
                Ok(points) => {
                    let w = self.width();
                    for (x, y) in &points {
                        self.canvas[y * w + x] = true;
                    }
                    self.steps += 1;
                    Outcome::ok(json!({ "points": points.len() }))
                }
                Err(note) => Outcome::fail(note),
            },
            "pixel.clear" => {
                self.clear();
                self.steps += 1;
                Outcome::ok(json!({ "cleared": true }))
            }
            "pixel.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "pixel.set" => {
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
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "steps": self.steps,
            "canvas": self.canvas,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let Some(arr) = state["canvas"].as_array() {
            let canvas: Option<Vec<bool>> = arr.iter().map(|v| v.as_bool()).collect();
            if let Some(canvas) = canvas {
                if canvas.len() == self.size() {
                    self.canvas = canvas;
                    self.steps = state["steps"].as_u64().unwrap_or(0);
                }
            }
        }
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
    }
}

#[cfg(test)]
mod app_tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, seeded, send};

    fn pixel(seed: u64) -> Pixel {
        seeded(Pixel::new(), "pixel.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = pixel(3);
        let mut b = pixel(3);
        for p in [&mut a, &mut b] {
            send(p, "pixel.stroke", json!({ "points": [[1, 1], [2, 2]] }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn stroke_draws_and_clear_wipes() {
        let mut p = pixel(1);
        let before = p.state(&iden())["frame"].clone();
        let out = send(
            &mut p,
            "pixel.stroke",
            json!({ "points": [[0, 0], [1, 0], [2, 0]] }),
        );
        assert!(out.ok);
        assert_eq!(out.data["points"], json!(3));
        let after = p.state(&iden())["frame"].clone();
        assert_ne!(before, after);
        assert_eq!(p.state(&iden())["painted"], json!(3));
        let out = send(&mut p, "pixel.clear", json!({}));
        assert!(out.ok);
        assert_eq!(p.state(&iden())["painted"], json!(0));
    }
    #[test]
    fn malformed_points_fail_honestly() {
        let mut p = pixel(1);
        assert!(!send(&mut p, "pixel.stroke", json!({ "points": "nope" })).ok);
        assert!(!send(&mut p, "pixel.stroke", json!({ "points": [[1]] })).ok);
        assert!(!send(&mut p, "pixel.stroke", json!({ "points": [[1, "x"]] })).ok);
        assert!(!send(&mut p, "pixel.stroke", json!({ "points": [[999, 0]] })).ok);
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut p = Pixel::new();
        let out = p.act(&iden(), &Call::new("pixel.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
        assert_eq!(p.state(&iden())["seed"], json!(5000));
    }
    #[test]
    fn set_validates_and_resets_the_round() {
        let mut p = pixel(4);
        send(&mut p, "pixel.stroke", json!({ "points": [[0, 0]] }));
        let out = send(&mut p, "pixel.set", json!({ "key": "width", "value": 8 }));
        assert!(out.ok);
        let state = p.state(&iden());
        assert_eq!(state["settings"]["width"], json!(8));
        assert_eq!(state["steps"], json!(0));
        assert_eq!(state["painted"], json!(0));
        assert!(!send(&mut p, "pixel.set", json!({ "key": "width", "value": 999 })).ok);
        assert!(
            !send(
                &mut p,
                "pixel.set",
                json!({ "key": "design", "value": "nope" })
            )
            .ok
        );
        assert!(!send(&mut p, "pixel.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn save_load_roundtrips_and_continues() {
        let mut a = pixel(11);
        send(
            &mut a,
            "pixel.stroke",
            json!({ "points": [[1, 1], [2, 2]] }),
        );
        let mut b = Pixel::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for p in [&mut a, &mut b] {
            send(p, "pixel.stroke", json!({ "points": [[3, 3]] }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut p = Pixel::new();
        p.load(&json!({ "seed": "soup", "canvas": [1, 2, 3], "settings": 7 }));
        assert_eq!(p.state(&iden())["steps"], json!(0));
        assert_eq!(p.state(&iden())["seed"], json!(0));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let p = pixel(3);
        let names: Vec<String> = p.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["pixel.stroke", "pixel.clear", "pixel.reset", "pixel.set"]
        );
    }
    #[test]
    fn state_carries_an_indexed_frame() {
        let p = pixel(5);
        let state = p.state(&iden());
        let palette = state["frame"]["palette"].as_array().unwrap();
        assert!(!palette.is_empty());
        let rows = state["frame"]["rows"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            state["frame"]["height"].as_u64().unwrap() as usize
        );
        assert_eq!(state["score"], json!(0));
        assert_eq!(state["over"], json!(false));
    }
}
