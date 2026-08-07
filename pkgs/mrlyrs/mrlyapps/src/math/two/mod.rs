use mrlycore::colors::hex;
use mrlycore::colors::named;
use mrlycore::{json, Json};
use mrlymath::two::{carpet, fills, htree, net, void, vtree, Cell2d};
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use mrlyui::skin::pixel::PENS;

const DESIGNS: [&str; 5] = ["carpet", "net", "htree", "vtree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MAX_SIDE: u32 = 256;

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
    fn side(number: i64, level: i64) -> u32 {
        (number as u32).saturating_pow(level as u32)
    }
    fn fits(number: i64, level: i64) -> bool {
        Set::side(number, level) <= MAX_SIDE
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
            "fill" | "void" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if named(name).is_err() {
                    return Err("unknown color");
                }
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
            "fill": &self.fill,
            "void": &self.void,
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
    fn ink(name: &str) -> [u8; 4] {
        named(name)
            .map(|c| [c.r, c.g, c.b, 255])
            .unwrap_or([0, 0, 0, 255])
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
    fn cells_fact(&self, cell: &Cell2d) -> Json {
        let mut pens = vec![Two::ink(&self.set.void), Two::ink(&self.set.fill)];
        pens.resize(PENS, [0, 0, 0, 0]);
        json!({
            "ids": self.ids(cell),
            "skin": "tiles",
            "pens": pens.iter().map(|&p| hex(p)).collect::<Vec<_>>(),
        })
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
        json!({
            "settings": self.set.to_json(),
            "index": DESIGNS.iter().position(|&d| d == self.set.design).unwrap_or(0),
            "count": DESIGNS.len(),
            "census": { "grid": side, "fill": filled, "void": total - filled },
            "cells": self.cells_fact(&cell),
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
        assert!(!send(&mut t, "two.set", json!({ "key": "fill", "value": "🍎" })).ok);
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
    fn budgets_cap_the_side() {
        let mut t = Two::new();
        assert!(send(&mut t, "two.set", json!({ "key": "number", "value": 3 })).ok);
        assert!(send(&mut t, "two.set", json!({ "key": "level", "value": 5 })).ok);
        assert!(!send(&mut t, "two.set", json!({ "key": "level", "value": 6 })).ok);
        assert!(!send(&mut t, "two.set", json!({ "key": "number", "value": 9 })).ok);
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
        send(
            &mut a,
            "two.set",
            json!({ "key": "void", "value": "white" }),
        );
        let mut b = Two::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden(), None), a.state(&iden(), None));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Two::new();
        t.load(&json!({ "design": "sphere", "number": 4, "level": -1, "fill": "beige" }));
        assert_eq!(t.set.design, "carpet");
        assert_eq!(t.set.number, 5);
        assert_eq!(t.set.level, 2);
        assert_eq!(t.set.fill, "red");
        let mut g = Two::new();
        g.load(&json!({ "skin": "glyphs", "number": 9, "level": 3, "fill": "🍎", "void": "🍏" }));
        assert_eq!(g.set.number, 9);
        assert_eq!(g.set.level, 2);
        assert_eq!(g.set.fill, "red");
        assert_eq!(g.set.void, "black");
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = Two::new();
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["two.page", "two.set", "two.reset"]);
    }
    #[test]
    fn state_carries_the_cells_fact() {
        let mut t = Two::new();
        send(&mut t, "two.set", json!({ "key": "fill", "value": "cyan" }));
        let state = t.state(&iden(), None);
        let side = state["census"]["grid"].as_u64().unwrap() as usize;
        let cells = &state["cells"];
        assert_eq!(cells["skin"], json!("tiles"));
        let rows = cells["ids"].as_array().unwrap();
        assert_eq!(rows.len(), side);
        assert_eq!(rows[0].as_array().unwrap().len(), side);
        let pens = cells["pens"].as_array().unwrap();
        assert_eq!(pens.len(), 16);
        assert_eq!(pens[0], json!("#000000"));
        assert_eq!(pens[1], json!("#1ec9f3"));
        assert!(state.get("frame").is_none());
        assert!(state.get("skin").is_none());
        assert!(state.get("ids").is_none());
        assert!(state["census"]["fill"].as_u64().unwrap() > 0);
        let flat: Vec<u64> = rows
            .iter()
            .flat_map(|row| row.as_array().unwrap().iter().map(|v| v.as_u64().unwrap()))
            .collect();
        assert!(flat.contains(&0));
        assert!(flat.contains(&1));
    }
}
