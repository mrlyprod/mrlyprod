use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn parse(name: &str) -> Option<Op> {
        match name {
            "add" => Some(Op::Add),
            "sub" => Some(Op::Sub),
            "mul" => Some(Op::Mul),
            "div" => Some(Op::Div),
            _ => None,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::Div => "div",
        }
    }
}

pub struct Calculator {
    display: String,
    previous: Option<f64>,
    operator: Option<Op>,
    waiting: bool,
    glyphs: bool,
}

impl Default for Calculator {
    fn default() -> Calculator {
        Calculator::new()
    }
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display: "0".to_string(),
            previous: None,
            operator: None,
            waiting: false,
            glyphs: false,
        }
    }
    pub fn display(&self) -> &str {
        &self.display
    }
    pub fn digit(&mut self, d: u8) {
        let c = char::from(b'0' + d % 10);
        if self.waiting {
            self.display = c.to_string();
            self.waiting = false;
        } else if self.display == "0" {
            self.display = c.to_string();
        } else {
            self.display.push(c);
        }
    }
    pub fn dot(&mut self) {
        if self.waiting {
            self.display = "0.".to_string();
            self.waiting = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }
    pub fn op(&mut self, op: Op) {
        if let (Some(prev), Some(cur)) = (self.previous, self.operator) {
            if !self.waiting {
                self.display = format(apply(prev, self.value(), cur));
            }
        }
        self.previous = Some(self.value());
        self.operator = Some(op);
        self.waiting = true;
    }
    pub fn equals(&mut self) {
        if let (Some(prev), Some(op)) = (self.previous, self.operator) {
            self.display = format(apply(prev, self.value(), op));
            self.previous = None;
            self.operator = None;
            self.waiting = true;
        }
    }
    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.previous = None;
        self.operator = None;
        self.waiting = false;
    }
    pub fn percent(&mut self) {
        self.display = format(self.value() / 100.0);
    }
    pub fn negate(&mut self) {
        self.display = format(-self.value());
    }
    fn value(&self) -> f64 {
        self.display.parse::<f64>().unwrap_or(f64::NAN)
    }
}

impl App for Calculator {
    fn route(&self) -> &str {
        "calculator"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("calculator").emoji("🧮").category("tools")
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("calculator.digit", json!({ "d": "u8" })),
            Verb::new("calculator.dot", json!({})),
            Verb::new("calculator.op", json!({ "op": "add | sub | mul | div" })),
            Verb::new("calculator.equals", json!({})),
            Verb::new("calculator.clear", json!({})),
            Verb::new("calculator.negate", json!({})),
            Verb::new("calculator.percent", json!({})),
            Verb::new("calculator.copy", json!({})),
        ]
    }
    fn state(&self, _iden: &Iden) -> Json {
        let mut out = self.save();
        if self.glyphs {
            out["glyph"] = crate::ui::frame::glyph_fact(&self.display);
        }
        out
    }
    fn wear(&mut self, world: &Json) {
        self.glyphs = world["shared"]["settings"]["font"] == "mrly";
    }
    fn save(&self) -> Json {
        json!({
            "display": self.display,
            "previous": self.previous,
            "operator": self.operator.map(|op| op.name()),
            "waiting": self.waiting,
        })
    }
    fn load(&mut self, state: &Json) {
        self.display = state["display"].as_str().unwrap_or("0").to_string();
        self.previous = state["previous"].as_f64();
        self.operator = state["operator"].as_str().and_then(Op::parse);
        self.waiting = state["waiting"].as_bool().unwrap_or(false);
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "calculator.digit" => match call.arg("d").as_u64() {
                Some(d) if d <= 9 => {
                    self.digit(d as u8);
                    Outcome::ok(json!({ "d": d }))
                }
                _ => Outcome::fail("no such digit"),
            },
            "calculator.dot" => {
                self.dot();
                Outcome::ok(json!({}))
            }
            "calculator.op" => match Op::parse(call.arg("op").as_str().unwrap_or("")) {
                Some(op) => {
                    self.op(op);
                    Outcome::ok(json!({ "op": call.arg("op") }))
                }
                None => Outcome::fail("no such op"),
            },
            "calculator.equals" => {
                self.equals();
                Outcome::ok(json!({ "display": self.display() }))
            }
            "calculator.clear" => {
                self.clear();
                Outcome::ok(json!({}))
            }
            "calculator.negate" => {
                self.negate();
                Outcome::ok(json!({ "display": self.display() }))
            }
            "calculator.percent" => {
                self.percent();
                Outcome::ok(json!({ "display": self.display() }))
            }
            "calculator.copy" => Outcome::ok(json!({ "display": self.display() }))
                .emit(Effect::new("copy", json!({ "text": self.display() }))),
            _ => Outcome::fail("unknown verb"),
        }
    }
}

fn apply(a: f64, b: f64, op: Op) -> f64 {
    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
        Op::Div => {
            if b == 0.0 {
                f64::NAN
            } else {
                a / b
            }
        }
    }
}

fn round(v: f64, sig: i32) -> f64 {
    let mut a = v.abs();
    let mut mag = 0;
    while a >= 10.0 {
        a /= 10.0;
        mag += 1;
    }
    while a < 1.0 {
        a *= 10.0;
        mag -= 1;
    }
    let factor = 10f64.powi(sig - 1 - mag);
    (v * factor).round() / factor
}

fn format(v: f64) -> String {
    if !v.is_finite() {
        return "Error".to_string();
    }
    if v == 0.0 {
        return "0".to_string();
    }
    format!("{}", round(v, 12))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(seq: &[&str]) -> String {
        let mut c = Calculator::new();
        for token in seq {
            match *token {
                "+" => c.op(Op::Add),
                "-" => c.op(Op::Sub),
                "*" => c.op(Op::Mul),
                "/" => c.op(Op::Div),
                "=" => c.equals(),
                "." => c.dot(),
                "ac" => c.clear(),
                "%" => c.percent(),
                "neg" => c.negate(),
                d => c.digit(d.parse::<u8>().unwrap()),
            }
        }
        c.display().to_string()
    }
    #[test]
    fn adds() {
        assert_eq!(run(&["2", "+", "3", "="]), "5");
    }
    #[test]
    fn chains_operators() {
        assert_eq!(run(&["2", "+", "3", "*"]), "5");
        assert_eq!(run(&["2", "+", "3", "*", "4", "="]), "20");
    }
    #[test]
    fn divides_and_floats() {
        assert_eq!(run(&["1", "/", "4", "="]), "0.25");
    }
    #[test]
    fn float_dust_is_trimmed() {
        assert_eq!(run(&["0", ".", "1", "+", "0", ".", "2", "="]), "0.3");
    }
    #[test]
    fn divide_by_zero_errors() {
        assert_eq!(run(&["5", "/", "0", "="]), "Error");
    }
    #[test]
    fn percent_and_negate() {
        assert_eq!(run(&["5", "0", "%"]), "0.5");
        assert_eq!(run(&["7", "neg"]), "-7");
    }
    #[test]
    fn clear_resets() {
        assert_eq!(run(&["9", "ac"]), "0");
    }
    #[test]
    fn leading_zero_replaced() {
        assert_eq!(run(&["0", "5"]), "5");
    }
    #[test]
    fn acts_by_verb() {
        let iden = Iden::new("aria");
        let mut c = Calculator::new();
        assert!(
            c.act(&iden, &Call::new("calculator.digit", json!({ "d": 6 })))
                .ok
        );
        assert!(
            c.act(&iden, &Call::new("calculator.op", json!({ "op": "mul" })))
                .ok
        );
        assert!(
            c.act(&iden, &Call::new("calculator.digit", json!({ "d": 7 })))
                .ok
        );
        assert!(c.act(&iden, &Call::new("calculator.equals", json!({}))).ok);
        assert_eq!(c.display(), "42");
        assert!(
            !c.act(&iden, &Call::new("calculator.op", json!({ "op": "pow" })))
                .ok
        );
        assert!(
            !c.act(&iden, &Call::new("calculator.digit", json!({ "d": 12 })))
                .ok
        );
    }
    #[test]
    fn save_load_roundtrips_mid_sum() {
        let mut a = Calculator::new();
        a.digit(2);
        a.op(Op::Add);
        a.digit(3);
        let mut b = Calculator::new();
        b.load(&a.save());
        b.equals();
        assert_eq!(b.display(), "5");
        let mut c = Calculator::new();
        c.load(&json!({ "display": 7 }));
        assert_eq!(c.display(), "0");
    }
    #[test]
    fn state_shows_the_display() {
        let mut c = Calculator::new();
        c.digit(4);
        c.digit(2);
        let state = c.state(&Iden::new("aria"));
        assert_eq!(state["display"], "42");
        assert_eq!(state, c.save());
    }
    #[test]
    fn copy_emits_the_clipboard_effect() {
        let iden = Iden::new("aria");
        let mut c = Calculator::new();
        c.digit(4);
        c.digit(2);
        let out = c.act(&iden, &Call::new("calculator.copy", json!({})));
        assert!(out.ok);
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.effects[0].kind, "copy");
        assert_eq!(out.effects[0].data["text"], json!("42"));
        assert_eq!(c.display(), "42");
    }
    #[test]
    fn worn_calculator_shows_the_glyph_face() {
        let iden = Iden::new("aria");
        let mut c = Calculator::new();
        c.wear(&json!({ "shared": { "settings": { "font": "mrly" } } }));
        c.digit(4);
        c.digit(2);
        assert_eq!(c.state(&iden)["glyph"]["text"], json!("42"));
    }
}
