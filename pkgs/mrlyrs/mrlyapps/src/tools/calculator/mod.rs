use mrlycore::{json, Json};
use mrlyos::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};

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

const SCALE: i128 = 1_000_000_000_000;

const PLACES: u32 = 12;

const ERROR: &str = "Error";

pub struct Calculator {
    display: String,
    previous: Option<String>,
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
        if let (Some(prev), Some(cur)) = (self.previous.clone(), self.operator) {
            if !self.waiting {
                self.display = self.result(&prev, cur);
            }
        }
        self.previous = Some(canon(&self.display));
        self.operator = Some(op);
        self.waiting = true;
    }
    pub fn equals(&mut self) {
        if let (Some(prev), Some(op)) = (self.previous.clone(), self.operator) {
            self.display = self.result(&prev, op);
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
        self.display = match self.value() {
            Some(n) => format(divide(n, 100)),
            None => ERROR.to_string(),
        };
    }
    pub fn negate(&mut self) {
        self.display = match self.value() {
            Some(n) => format(-n),
            None => ERROR.to_string(),
        };
    }
    fn value(&self) -> Option<i128> {
        parse(&self.display)
    }
    fn result(&self, prev: &str, op: Op) -> String {
        match (parse(prev), self.value()) {
            (Some(a), Some(b)) => match apply(a, b, op) {
                Some(n) => format(n),
                None => ERROR.to_string(),
            },
            _ => ERROR.to_string(),
        }
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
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        let mut out = self.save();
        if self.glyphs {
            out["glyph"] = mrlyui::frame::glyph_fact(&self.display);
        }
        out
    }
    fn wear(&mut self, world: &Json) {
        self.glyphs = world["shared"]["settings"]["font"] == "mrly";
    }
    fn save(&self) -> Json {
        json!({
            "display": &self.display,
            "previous": self.previous.clone(),
            "operator": self.operator.map(|op| op.name()),
            "waiting": self.waiting,
        })
    }
    fn load(&mut self, state: &Json) {
        self.display = state["display"].as_str().unwrap_or("0").to_string();
        self.previous = state["previous"].as_str().map(|text| text.to_string());
        self.operator = state["operator"].as_str().and_then(Op::parse);
        self.waiting = state["waiting"].as_bool().unwrap_or(false);
    }
    fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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

fn apply(a: i128, b: i128, op: Op) -> Option<i128> {
    match op {
        Op::Add => a.checked_add(b),
        Op::Sub => a.checked_sub(b),
        Op::Mul => a.checked_mul(b).map(|p| divide(p, SCALE)),
        Op::Div => {
            if b == 0 {
                None
            } else {
                a.checked_mul(SCALE).map(|p| divide(p, b))
            }
        }
    }
}

fn divide(a: i128, b: i128) -> i128 {
    let q = a / b;
    let r = a % b;
    if r == 0 {
        return q;
    }
    if r.unsigned_abs() * 2 < b.unsigned_abs() {
        return q;
    }
    if (a < 0) != (b < 0) {
        q - 1
    } else {
        q + 1
    }
}

fn parse(text: &str) -> Option<i128> {
    let text = text.trim();
    let (neg, body) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (whole, frac) = match body.split_once('.') {
        Some((w, f)) => (w, f),
        None => (body, ""),
    };
    if whole.is_empty() && frac.is_empty() {
        return None;
    }
    if !whole
        .bytes()
        .chain(frac.bytes())
        .all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let mut n: i128 = 0;
    for b in whole.bytes() {
        n = n.checked_mul(10)?.checked_add((b - b'0') as i128)?;
    }
    n = n.checked_mul(SCALE)?;
    let mut kept: i128 = 0;
    let mut taken = 0u32;
    let mut round_up = false;
    for b in frac.bytes() {
        let d = (b - b'0') as i128;
        if taken < PLACES {
            kept = kept * 10 + d;
            taken += 1;
        } else if taken == PLACES {
            round_up = d >= 5;
            taken += 1;
        }
    }
    kept *= 10i128.pow(PLACES - taken.min(PLACES));
    if round_up {
        kept += 1;
    }
    n = n.checked_add(kept)?;
    Some(if neg { -n } else { n })
}

fn format(n: i128) -> String {
    let whole = n.unsigned_abs() / SCALE as u128;
    let frac = n.unsigned_abs() % SCALE as u128;
    let mut out = String::new();
    if n < 0 {
        out.push('-');
    }
    out.push_str(&whole.to_string());
    if frac != 0 {
        let mut digits = format!("{frac:012}");
        while digits.ends_with('0') {
            digits.pop();
        }
        out.push('.');
        out.push_str(&digits);
    }
    out
}

fn canon(text: &str) -> String {
    match parse(text) {
        Some(n) => format(n),
        None => ERROR.to_string(),
    }
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
    fn division_rounds_half_away_at_twelve_places() {
        assert_eq!(run(&["1", "/", "3", "="]), "0.333333333333");
        assert_eq!(run(&["2", "/", "3", "="]), "0.666666666667");
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
            c.call(&iden, &Call::new("calculator.digit", json!({ "d": 6 })))
                .ok
        );
        assert!(
            c.call(&iden, &Call::new("calculator.op", json!({ "op": "mul" })))
                .ok
        );
        assert!(
            c.call(&iden, &Call::new("calculator.digit", json!({ "d": 7 })))
                .ok
        );
        assert!(c.call(&iden, &Call::new("calculator.equals", json!({}))).ok);
        assert_eq!(c.display(), "42");
        assert!(
            !c.call(&iden, &Call::new("calculator.op", json!({ "op": "pow" })))
                .ok
        );
        assert!(
            !c.call(&iden, &Call::new("calculator.digit", json!({ "d": 12 })))
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
        let state = c.state(&Iden::new("aria"), None);
        assert_eq!(state["display"], "42");
        assert_eq!(state, c.save());
    }
    #[test]
    fn copy_emits_the_clipboard_effect() {
        let iden = Iden::new("aria");
        let mut c = Calculator::new();
        c.digit(4);
        c.digit(2);
        let out = c.call(&iden, &Call::new("calculator.copy", json!({})));
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
        assert_eq!(c.state(&iden, None)["glyph"]["text"], json!("42"));
    }
}
