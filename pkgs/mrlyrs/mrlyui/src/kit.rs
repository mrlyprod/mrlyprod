use crate::frame;
use crate::tokens;
use mrlycore::json;
use mrlycore::ui::{Pick, Role};
use mrlycore::Json;

pub use mrlycore::ui::{Call, Node};

const ARG: &str = "value";
const DIRS: [&str; 4] = ["left", "up", "down", "right"];

// CALLS
pub fn call(verb: &str) -> Call {
    Call::new(verb, json!({}))
}

pub fn arg(verb: &str, key: &str, value: &str) -> Call {
    Call::new(verb, json!({ key: value }))
}

pub fn set(app: &str, key: &str) -> Call {
    Call::new(&format!("{app}.set"), json!({ "key": key }))
}

pub fn open(route: &str) -> Call {
    Call::new("nav.open", json!({ "app": route }))
}

// BOXES
pub fn page(children: Vec<Node>) -> Node {
    Node::column(children)
}

pub fn list(children: Vec<Node>) -> Node {
    Node::column(children)
}

pub fn card(children: Vec<Node>) -> Node {
    Node::group(children)
}

pub fn pills(children: Vec<Node>) -> Node {
    Node::wrap(children)
}

pub fn launch(children: Vec<Node>) -> Node {
    Node::grid(tokens::LAUNCH, children)
}

// TYPE
pub fn title(text: &str) -> Node {
    Node::text(text, Role::Title)
}

pub fn heading(text: &str) -> Node {
    Node::text(text, Role::Label)
}

pub fn meter(text: &str) -> Node {
    Node::text(text, Role::Note)
}

// CONTROLS
pub fn button(label: &str, call: Call) -> Node {
    Node::button(label, call)
}

pub fn item(text: &str, note: &str, call: Option<Call>) -> Node {
    Node::label(text, note, call)
}

pub fn toggle(label: &str, on: bool, call: Call) -> Node {
    Node::toggle(label, on, call, ARG)
}

pub fn range(label: &str, value: i64, min: i64, max: i64, step: i64, call: Call) -> Node {
    Node::range(label, value, min, max, step, call, ARG)
}

pub fn scaled(label: &str, value: i64, min: i64, max: i64, step: i64, by: i64, call: Call) -> Node {
    Node::range(label, value, min, max, step, call, ARG).scaled(by)
}

pub fn segments(label: &str, value: &str, options: Vec<String>, call: Call) -> Node {
    Node::choice(label, value, options, Pick::Segments, call, ARG)
}

pub fn cycle(label: &str, value: &str, options: Vec<String>, call: Call) -> Node {
    Node::choice(label, value, options, Pick::Cycle, call, ARG)
}

pub fn select(label: &str, value: &str, options: Vec<String>, call: Call) -> Node {
    Node::choice(label, value, options, Pick::Menu, call, ARG)
}

pub fn search(value: &str, hint: &str, call: Call, key: &str, enter: Option<Call>) -> Node {
    let field = Node::field(value, hint, call, key).live().icon("search");
    match enter {
        Some(go) => field.enter(go),
        None => field,
    }
}

pub fn color(value: [u8; 4], call: Call) -> Node {
    Node::field(&frame::hex(value), "#rrggbb", call, ARG).keys("colors")
}

// PIECES
pub fn icon(name: &str) -> Node {
    Node::symbol(name).sized(tokens::SYMBOL)
}

pub fn mark(name: &str) -> Node {
    Node::symbol(name).sized(tokens::SYMBOL).inked("accent")
}

pub fn board(fact: Json) -> Node {
    Node::image(fact)
}

pub fn dpad(glyphs: [&str; 4], verb: &str) -> Node {
    Node::grid(
        tokens::DPAD,
        glyphs
            .iter()
            .zip(DIRS)
            .map(|(glyph, dir)| Node::button(glyph, arg(verb, "dir", dir)))
            .collect(),
    )
}

pub fn over(head: &str, note: &str, label: &str, call: Call) -> Node {
    Node::group(vec![title(head), meter(note), button(label, call)])
}
