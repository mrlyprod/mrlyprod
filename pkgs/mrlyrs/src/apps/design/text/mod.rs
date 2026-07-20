use crate::math::two::{carpet, htree, net, void, vtree, Cell2d};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

const DESIGNS: [&str; 5] = ["carpet", "net", "htree", "vtree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MAX_CELLS: u32 = 2500;

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
            number: 3,
            level: 2,
            fill: "🍎".to_string(),
            void: "🍏".to_string(),
        }
    }
    fn side(number: i64, level: i64) -> u32 {
        (number as u32).saturating_pow(level as u32)
    }
    fn cells(number: i64, level: i64) -> u32 {
        let side = Set::side(number, level);
        side.saturating_mul(side)
    }
    fn int(value: &Json) -> Option<i64> {
        value
            .as_i64()
            .or_else(|| value.as_str().and_then(|s| s.parse::<i64>().ok()))
    }
    fn one_char(value: &str) -> bool {
        value.chars().count() == 1
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
                    return Err("over the 2500-cell budget");
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
                    return Err("over the 2500-cell budget");
                }
                self.level = n;
                Ok(json!(n))
            }
            "fill" | "void" => {
                let ch = value.as_str().ok_or("value must be a string")?;
                if !Set::one_char(ch) {
                    return Err("one character");
                }
                match key {
                    "fill" => self.fill = ch.to_string(),
                    _ => self.void = ch.to_string(),
                }
                Ok(json!(ch))
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
        if let Some(ch) = value["fill"].as_str() {
            if Set::one_char(ch) {
                set.fill = ch.to_string();
            }
        }
        if let Some(ch) = value["void"].as_str() {
            if Set::one_char(ch) {
                set.void = ch.to_string();
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

pub struct Text {
    set: Set,
}

impl Default for Text {
    fn default() -> Text {
        Text::new()
    }
}

impl Text {
    pub fn new() -> Text {
        Text { set: Set::new() }
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
    fn grid(&self) -> Vec<String> {
        let cell = self.cell();
        let w = cell.width();
        let h = cell.height();
        let fill_ch = self.set.fill.chars().next().unwrap();
        let void_ch = self.set.void.chars().next().unwrap();
        let bytes = cell.types().bytes();
        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| {
                        if bytes[y * w + x] == 1 {
                            fill_ch
                        } else {
                            void_ch
                        }
                    })
                    .collect::<String>()
            })
            .collect()
    }
}

impl App for Text {
    fn route(&self) -> &str {
        "text"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("text").emoji("🍎").category("design")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let side = Set::side(self.set.number, self.set.level) as usize;
        json!({
            "design": self.set.design,
            "number": self.set.number,
            "level": self.set.level,
            "fill": self.set.fill,
            "void": self.set.void,
            "cols": side,
            "rows": side,
            "grid": self.grid(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("text.page", json!({ "dir": "next | prev" })),
            Verb::new("text.set", json!({ "key": "string", "value": "any" })),
            Verb::new("text.reset", json!({})),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "text.page" => {
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
            "text.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "text.reset" => {
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
        let mut t = Text::new();
        assert!(
            send(
                &mut t,
                "text.set",
                json!({ "key": "design", "value": "net" })
            )
            .ok
        );
        assert!(
            !send(
                &mut t,
                "text.set",
                json!({ "key": "design", "value": "sphere" })
            )
            .ok
        );
        assert!(send(&mut t, "text.set", json!({ "key": "number", "value": 7 })).ok);
        assert!(!send(&mut t, "text.set", json!({ "key": "number", "value": 4 })).ok);
        assert!(!send(&mut t, "text.set", json!({ "key": "level", "value": 0 })).ok);
        assert!(!send(&mut t, "text.set", json!({ "key": "level", "value": 3 })).ok);
        assert!(send(&mut t, "text.set", json!({ "key": "fill", "value": "x" })).ok);
        assert!(!send(&mut t, "text.set", json!({ "key": "fill", "value": "xy" })).ok);
        assert!(!send(&mut t, "text.set", json!({ "key": "volume", "value": 1 })).ok);
    }
    #[test]
    fn page_cycles() {
        let mut t = Text::new();
        assert_eq!(t.set.design, "carpet");
        send(&mut t, "text.page", json!({ "dir": "next" }));
        assert_eq!(t.set.design, "net");
        send(&mut t, "text.page", json!({ "dir": "prev" }));
        send(&mut t, "text.page", json!({ "dir": "prev" }));
        assert_eq!(t.set.design, "void");
        assert!(!send(&mut t, "text.page", json!({ "dir": "sideways" })).ok);
    }
    #[test]
    fn budget_blocks_over_2500_cells() {
        let mut t = Text::new();
        assert!(!send(&mut t, "text.set", json!({ "key": "number", "value": 9 })).ok);
        assert!(send(&mut t, "text.set", json!({ "key": "level", "value": 1 })).ok);
        assert!(send(&mut t, "text.set", json!({ "key": "number", "value": 9 })).ok);
        assert!(!send(&mut t, "text.set", json!({ "key": "level", "value": 2 })).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let mut a = Text::new();
        send(
            &mut a,
            "text.set",
            json!({ "key": "design", "value": "htree" }),
        );
        send(&mut a, "text.set", json!({ "key": "number", "value": 5 }));
        send(&mut a, "text.set", json!({ "key": "level", "value": 2 }));
        send(&mut a, "text.set", json!({ "key": "fill", "value": "#" }));
        let mut b = Text::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = Text::new();
        t.load(&json!({
            "design": "sphere",
            "number": 4,
            "level": -1,
            "fill": "xy",
            "void": "zz",
        }));
        assert_eq!(t.set.design, "carpet");
        assert_eq!(t.set.number, 3);
        assert_eq!(t.set.level, 2);
        assert_eq!(t.set.fill, "🍎");
        assert_eq!(t.set.void, "🍏");

        let mut recovers_level = Text::new();
        recovers_level.load(&json!({ "number": 5, "level": 4 }));
        assert_eq!(recovers_level.set.number, 5);
        assert_eq!(recovers_level.set.level, 2);

        let mut recovers_number = Text::new();
        recovers_number.load(&json!({ "number": 9, "level": 4 }));
        assert_eq!(recovers_number.set.number, 3);
        assert_eq!(recovers_number.set.level, 2);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = Text::new();
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(names, vec!["text.page", "text.set", "text.reset"]);
    }
    #[test]
    fn grid_paints_fill_and_void() {
        let mut t = Text::new();
        send(
            &mut t,
            "text.set",
            json!({ "key": "design", "value": "net" }),
        );
        let state = t.state(&iden());
        let side = state["cols"].as_u64().unwrap() as usize;
        let rows = state["grid"].as_array().unwrap();
        assert_eq!(rows.len(), side);
        let mut saw_fill = false;
        let mut saw_void = false;
        for row in rows {
            let row = row.as_str().unwrap();
            assert_eq!(row.chars().count(), side);
            if row.contains('🍎') {
                saw_fill = true;
            }
            if row.contains('🍏') {
                saw_void = true;
            }
        }
        assert!(saw_fill);
        assert!(saw_void);
    }
}
