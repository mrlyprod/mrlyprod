use mrlycore::{json, Json};
use mrlymath::two::{carpet, fills, htree, net, void, vtree, Cell2d};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use mrlyui::frame::{field, Frame};
use mrlyui::skin::{two as dress, Ink, Skin};

const DESIGNS: [&str; 5] = ["carpet", "net", "htree", "vtree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MAX_SIDE: u32 = 256;
const MAX_CELLS: u32 = 2500;

struct Set {
    design: String,
    number: i64,
    level: i64,
    skin: String,
    fill: String,
    void: String,
}

impl Set {
    fn new() -> Set {
        let skin = dress::MODES[0];
        let (fill, void) = dress::defaults(skin);
        Set {
            design: "carpet".to_string(),
            number: 5,
            level: 2,
            skin: skin.to_string(),
            fill: fill.to_string(),
            void: void.to_string(),
        }
    }
    fn side(number: i64, level: i64) -> u32 {
        (number as u32).saturating_pow(level as u32)
    }
    fn fits(skin: &str, number: i64, level: i64) -> bool {
        let side = Set::side(number, level);
        match dress::glyphs(skin) {
            true => side.saturating_mul(side) <= MAX_CELLS,
            false => side <= MAX_SIDE,
        }
    }
    fn int(value: &Json) -> Option<i64> {
        value
            .as_i64()
            .or_else(|| value.as_str().and_then(|s| s.parse::<i64>().ok()))
    }
    fn dress(&mut self, skin: &str) {
        let (fill, void) = dress::defaults(skin);
        self.skin = skin.to_string();
        self.fill = fill.to_string();
        self.void = void.to_string();
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
                if !Set::fits(&self.skin, n, self.level) {
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
                if !Set::fits(&self.skin, self.number, n) {
                    return Err("too many cells");
                }
                self.level = n;
                Ok(json!(n))
            }
            "skin" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if !dress::MODES.contains(&name) {
                    return Err("no such skin");
                }
                if !Set::fits(name, self.number, self.level) {
                    return Err("too many cells");
                }
                if name != self.skin {
                    self.dress(name);
                }
                Ok(json!(name))
            }
            "fill" | "void" => {
                let name = value.as_str().ok_or("value must be a string")?;
                dress::accepts(&self.skin, name)?;
                match key {
                    "fill" => self.fill = name.to_string(),
                    _ => self.void = name.to_string(),
                }
                Ok(json!(name))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "design": &self.design,
            "number": self.number,
            "level": self.level,
            "skin": &self.skin,
            "fill": &self.fill,
            "void": &self.void,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(name) = value["skin"].as_str() {
            if dress::MODES.contains(&name) {
                set.dress(name);
            }
        }
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
            if dress::accepts(&set.skin, name).is_ok() {
                set.fill = name.to_string();
            }
        }
        if let Some(name) = value["void"].as_str() {
            if dress::accepts(&set.skin, name).is_ok() {
                set.void = name.to_string();
            }
        }
        if !Set::fits(&set.skin, set.number, set.level) {
            let defaults = Set::new();
            set.level = defaults.level;
            if !Set::fits(&set.skin, set.number, set.level) {
                set.number = defaults.number;
            }
        }
        set
    }
}

pub struct Two {
    set: Set,
}

impl Default for Two {
    fn default() -> Two {
        Two::new()
    }
}

impl Two {
    pub fn new() -> Two {
        Two { set: Set::new() }
    }
    fn cell(&self) -> Cell2d {
        let n = self.set.number as usize;
        let l = self.set.level as usize;
        match self.set.design.as_str() {
            "carpet" => carpet(n, l),
            "net" => net(n, l),
            "htree" => htree(n, l),
            "vtree" => vtree(n, l),
            _ => void(n, l),
        }
        .unwrap()
    }
    fn skin(&self) -> Skin {
        dress::skin(&self.set.skin, &self.set.fill, &self.set.void)
    }
    fn glyphs(&self) -> bool {
        dress::glyphs(&self.set.skin)
    }
    fn ids(&self, cell: &Cell2d) -> Vec<Vec<u8>> {
        let w = cell.width();
        let bytes = cell.types().bytes();
        (0..cell.height())
            .map(|y| {
                (0..w)
                    .map(|x| u8::from(bytes[y * w + x] == 1))
                    .collect::<Vec<u8>>()
            })
            .collect()
    }
    fn render(&self, cell: &Cell2d) -> Frame {
        let (w, h) = (cell.width(), cell.height());
        let skin = self.skin();
        let clear = [0, 0, 0, 0];
        let paint = |role: usize| match skin.visuals.get(role).and_then(|v| v.bg.clone()) {
            Some(Ink::Hex(color)) => color,
            _ => clear,
        };
        let (empty, fill) = (paint(0), paint(1));
        let colors: Vec<[u8; 4]> = cell
            .types()
            .bytes()
            .iter()
            .map(|&v| if v == 1 { fill } else { empty })
            .collect();
        field(w, h, colors, empty)
    }
}

impl App for Two {
    fn route(&self) -> &str {
        "two"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("two").emoji("🔳").category("math")
    }
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let cell = self.cell();
        let side = cell.width().max(cell.height());
        let filled = fills(&cell);
        let total = cell.width() * cell.height();
        let glyphs = self.glyphs();
        json!({
            "settings": self.set.to_json(),
            "index": DESIGNS.iter().position(|&d| d == self.set.design).unwrap_or(0),
            "count": DESIGNS.len(),
            "census": { "grid": side, "fill": filled, "void": total - filled },
            "skin": self.skin().to_json(),
            "ids": match glyphs {
                true => json!(self.ids(&cell)),
                false => Json::Null,
            },
            "frame": match glyphs {
                true => Json::Null,
                false => self.render(&cell).fact(),
            },
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("two.page", json!({ "dir": "next | prev" })),
            Verb::new("two.set", json!({ "key": "string", "value": "any" })),
            Verb::new("two.reset", json!({})),
        ]
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "two.page" => {
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
            "two.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "two.reset" => {
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
    use mrlyos::kernel::testkit::{iden, send};

    fn glyphed() -> Two {
        let mut t = Two::new();
        send(
            &mut t,
            "two.set",
            json!({ "key": "skin", "value": "glyphs" }),
        );
        t
    }

    #[test]
    fn set_validates() {
        let mut t = Two::new();
        assert!(
            send(
                &mut t,
                "two.set",
                json!({ "key": "design", "value": "net" })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "two.set",
                json!({ "key": "design", "value": "sphere" })
            )
            .ok
        );
        assert!(send(&mut t, "two.set", json!({ "key": "number", "value": 7 })).ok);
        assert!(!send(&mut t, "two.set", json!({ "key": "number", "value": 4 })).ok);
        assert!(send(&mut t, "two.set", json!({ "key": "level", "value": 2 })).ok);
        assert!(!send(&mut t, "two.set", json!({ "key": "level", "value": 0 })).ok);
        assert!(!send(&mut t, "two.set", json!({ "key": "level", "value": 5 })).ok);
        assert!(send(&mut t, "two.set", json!({ "key": "fill", "value": "blue" })).ok);
        assert!(
            !send(
                &mut t,
                "two.set",
                json!({ "key": "fill", "value": "beige" })
            )
            .ok
        );
        assert!(!send(&mut t, "two.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn page_cycles() {
        let mut t = Two::new();
        assert_eq!(t.set.design, "carpet");
        send(&mut t, "two.page", json!({ "dir": "next" }));
        assert_eq!(t.set.design, "net");
        assert_eq!(t.state(&iden(), None)["index"], json!(1));
        assert_eq!(t.state(&iden(), None)["count"], json!(5));
        send(&mut t, "two.page", json!({ "dir": "prev" }));
        send(&mut t, "two.page", json!({ "dir": "prev" }));
        assert_eq!(t.set.design, "void");
        assert_eq!(t.state(&iden(), None)["index"], json!(4));
        assert!(!send(&mut t, "two.page", json!({ "dir": "sideways" })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Two::new();
        send(
            &mut a,
            "two.set",
            json!({ "key": "design", "value": "htree" }),
        );
        send(&mut a, "two.set", json!({ "key": "number", "value": 9 }));
        send(&mut a, "two.set", json!({ "key": "level", "value": 2 }));
        send(&mut a, "two.set", json!({ "key": "fill", "value": "cyan" }));
        let mut b = Two::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
        let mut c = glyphed();
        send(&mut c, "two.set", json!({ "key": "fill", "value": "#" }));
        let mut d = Two::new();
        d.load(&c.save());
        assert_eq!(d.state(&iden(), None), c.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Two::new();
        t.load(&json!({ "design": "sphere", "number": 4, "level": -1, "fill": "beige" }));
        assert_eq!(t.set.design, "carpet");
        assert_eq!(t.set.number, 5);
        assert_eq!(t.set.level, 2);
        assert_eq!(t.set.skin, "solid");
        assert_eq!(t.set.fill, "red");
        let mut g = Two::new();
        g.load(&json!({ "skin": "glyphs", "number": 9, "level": 3, "fill": "xy" }));
        assert_eq!(g.set.number, 5);
        assert_eq!(g.set.level, 2);
        assert_eq!(g.set.fill, "🍎");
        assert_eq!(g.set.void, "🍏");
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = Two::new();
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["two.page", "two.set", "two.reset"]);
    }
    #[test]
    fn frame_renders_the_grid() {
        let mut t = Two::new();
        send(&mut t, "two.reset", json!({}));
        let state = t.state(&iden(), None);
        let side = state["census"]["grid"].as_u64().unwrap() as usize;
        assert_eq!(state["frame"]["rows"].as_array().unwrap().len(), side);
        assert_eq!(state["frame"]["rows"][0].as_array().unwrap().len(), side);
        assert!(state["frame"]["palette"].as_array().unwrap().len() >= 2);
        assert!(state["census"]["fill"].as_u64().unwrap() > 0);
        assert!(state["ids"].is_null());
        assert!(state["skin"][1]["bg"].is_string());
        assert!(state["skin"][1]["face"].is_null());
    }
    #[test]
    fn glyph_skin_swaps_the_alphabet() {
        let mut t = glyphed();
        assert_eq!(t.set.fill, "🍎");
        assert_eq!(t.set.void, "🍏");
        assert!(send(&mut t, "two.set", json!({ "key": "fill", "value": "x" })).ok);
        assert!(!send(&mut t, "two.set", json!({ "key": "fill", "value": "xy" })).ok);
        assert!(!send(&mut t, "two.set", json!({ "key": "fill", "value": "red" })).ok);
        assert!(
            !send(
                &mut t,
                "two.set",
                json!({ "key": "skin", "value": "velvet" })
            )
            .ok
        );
        assert!(
            send(
                &mut t,
                "two.set",
                json!({ "key": "skin", "value": "solid" })
            )
            .ok
        );
        assert_eq!(t.set.fill, "red");
    }
    #[test]
    fn glyph_skin_emits_ids_and_faces() {
        let mut t = glyphed();
        send(
            &mut t,
            "two.set",
            json!({ "key": "design", "value": "net" }),
        );
        let state = t.state(&iden(), None);
        let side = state["census"]["grid"].as_u64().unwrap() as usize;
        let rows = state["ids"].as_array().unwrap();
        assert_eq!(rows.len(), side);
        assert_eq!(rows[0].as_array().unwrap().len(), side);
        assert!(state["frame"].is_null());
        assert_eq!(state["skin"][0]["face"]["value"], json!("🍏"));
        assert_eq!(state["skin"][1]["face"]["value"], json!("🍎"));
        assert_eq!(state["skin"][1]["face"]["as"], json!("emoji"));
        let flat: Vec<u64> = rows
            .iter()
            .flat_map(|row| row.as_array().unwrap().iter().map(|v| v.as_u64().unwrap()))
            .collect();
        assert!(flat.contains(&0));
        assert!(flat.contains(&1));
    }
    #[test]
    fn budgets_differ_by_skin() {
        let mut t = Two::new();
        send(&mut t, "two.set", json!({ "key": "number", "value": 9 }));
        assert!(send(&mut t, "two.set", json!({ "key": "level", "value": 2 })).ok);
        assert!(
            !send(
                &mut t,
                "two.set",
                json!({ "key": "skin", "value": "glyphs" })
            )
            .ok
        );
        let mut g = glyphed();
        assert!(send(&mut g, "two.set", json!({ "key": "number", "value": 7 })).ok);
        assert!(!send(&mut g, "two.set", json!({ "key": "number", "value": 9 })).ok);
        assert!(!send(&mut g, "two.set", json!({ "key": "level", "value": 3 })).ok);
    }
}
