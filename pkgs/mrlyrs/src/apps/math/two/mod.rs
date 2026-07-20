use crate::core::colors::named;
use crate::math::two::{carpet, fills, htree, net, void, vtree, Cell2d};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame::{field, Frame};
use serde_json::{json, Value as Json};

const DESIGNS: [&str; 5] = ["carpet", "net", "htree", "vtree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MAX_CELLS: u32 = 256;

struct Set {
    design: String,
    number: i64,
    level: i64,
    fill: String,
    void: String,
}

impl Set {
    fn new() -> Set {
        Set {
            design: "carpet".to_string(),
            number: 5,
            level: 2,
            fill: "red".to_string(),
            void: "black".to_string(),
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
            "fill" | "void" => {
                let name = value.as_str().ok_or("value must be a string")?;
                named(name).map_err(|_| "unknown color")?;
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
            "design": self.design,
            "number": self.number,
            "level": self.level,
            "fill": self.fill,
            "void": self.void,
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
        if let Some(name) = value["void"].as_str() {
            if named(name).is_ok() {
                set.void = name.to_string();
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
    fn render(&self) -> Frame {
        let cell = self.cell();
        let (w, h) = (cell.width(), cell.height());
        let fill_c = named(&self.set.fill).unwrap();
        let void_c = named(&self.set.void).unwrap();
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
}

impl App for Two {
    fn route(&self) -> &str {
        "two"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("two").emoji("🔳").category("math")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let cell = self.cell();
        let side = cell.width().max(cell.height());
        let filled = fills(&cell);
        let total = cell.width() * cell.height();
        json!({
            "design": self.set.design,
            "index": DESIGNS.iter().position(|&d| d == self.set.design).unwrap_or(0),
            "count": DESIGNS.len(),
            "number": self.set.number,
            "level": self.set.level,
            "fill": self.set.fill,
            "void": self.set.void,
            "census": { "grid": side, "fill": filled, "void": total - filled },
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("two.page", json!({ "dir": "next | prev" })),
            Verb::new("two.set", json!({ "key": "string", "value": "any" })),
            Verb::new("two.reset", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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
                Outcome::ok(json!({ "design": self.set.design }))
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
    use crate::os::kernel::testkit::{iden, send};

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
        assert_eq!(t.state(&iden())["index"], json!(1));
        assert_eq!(t.state(&iden())["count"], json!(5));
        send(&mut t, "two.page", json!({ "dir": "prev" }));
        send(&mut t, "two.page", json!({ "dir": "prev" }));
        assert_eq!(t.set.design, "void");
        assert_eq!(t.state(&iden())["index"], json!(4));
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
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Two::new();
        t.load(&json!({ "design": "sphere", "number": 4, "level": -1, "fill": "beige" }));
        assert_eq!(t.set.design, "carpet");
        assert_eq!(t.set.number, 5);
        assert_eq!(t.set.level, 2);
        assert_eq!(t.set.fill, "red");
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
        let state = t.state(&iden());
        let side = state["census"]["grid"].as_u64().unwrap() as usize;
        assert_eq!(state["frame"]["rows"].as_array().unwrap().len(), side);
        assert_eq!(state["frame"]["rows"][0].as_array().unwrap().len(), side);
        assert!(state["frame"]["palette"].as_array().unwrap().len() >= 2);
        assert!(state["census"]["fill"].as_u64().unwrap() > 0);
    }
}
