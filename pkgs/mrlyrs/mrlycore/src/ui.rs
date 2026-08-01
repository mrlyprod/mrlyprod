use crate::Json;

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub verb: String,
    pub args: Json,
}

impl Call {
    pub fn new(verb: &str, args: Json) -> Call {
        Call {
            verb: verb.to_string(),
            args,
        }
    }
    pub fn fill(&self, arg: &str, value: Json) -> Call {
        let mut call = self.clone();
        call.args[arg] = value;
        call
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    Title,
    Label,
    Note,
    Body,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pick {
    Segments,
    Cycle,
    Menu,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Column {
        children: Vec<Node>,
    },
    Row {
        children: Vec<Node>,
    },
    Grid {
        cols: usize,
        children: Vec<Node>,
    },
    Group {
        children: Vec<Node>,
    },
    Wrap {
        children: Vec<Node>,
    },
    Overlay {
        child: Box<Node>,
        close: Option<Call>,
    },
    Text {
        text: String,
        role: Role,
    },
    Rule,
    Image {
        fact: Json,
    },
    Symbol {
        value: String,
    },
    Doc {
        md: String,
    },
    Table {
        rows: Vec<Vec<String>>,
    },
    Button {
        label: String,
        call: Option<Call>,
        active: bool,
        color: Option<String>,
        big: bool,
    },
    Toggle {
        label: String,
        on: bool,
        call: Call,
        arg: String,
    },
    Choice {
        label: String,
        value: String,
        options: Vec<String>,
        pick: Pick,
        call: Call,
        arg: String,
    },
    Range {
        label: String,
        value: i64,
        min: i64,
        max: i64,
        step: i64,
        scale: i64,
        call: Call,
        arg: String,
    },
    Field {
        value: String,
        hint: String,
        live: bool,
        call: Call,
        arg: String,
        enter: Option<Call>,
    },
    Cell {
        on: bool,
        color: Option<String>,
        child: Option<Box<Node>>,
        call: Option<Call>,
    },
    Label {
        symbol: Option<String>,
        text: String,
        note: String,
        call: Option<Call>,
    },
    Canvas {
        fact: Json,
        grid: Option<(usize, usize)>,
        tap: Option<Call>,
        drag: Option<Call>,
        turn: Option<Call>,
        zoom: Option<Call>,
        pan: Option<Call>,
    },
}

impl Node {
    pub fn column(children: Vec<Node>) -> Node {
        Node::Column { children }
    }
    pub fn row(children: Vec<Node>) -> Node {
        Node::Row { children }
    }
    pub fn grid(cols: usize, children: Vec<Node>) -> Node {
        Node::Grid { cols, children }
    }
    pub fn group(children: Vec<Node>) -> Node {
        Node::Group { children }
    }
    pub fn wrap(children: Vec<Node>) -> Node {
        Node::Wrap { children }
    }
    pub fn overlay(child: Node, close: Option<Call>) -> Node {
        Node::Overlay {
            child: Box::new(child),
            close,
        }
    }
    pub fn text(text: &str, role: Role) -> Node {
        Node::Text {
            text: text.to_string(),
            role,
        }
    }
    pub fn image(fact: Json) -> Node {
        Node::Image { fact }
    }
    pub fn symbol(value: &str) -> Node {
        Node::Symbol {
            value: value.to_string(),
        }
    }
    pub fn doc(md: &str) -> Node {
        Node::Doc { md: md.to_string() }
    }
    pub fn table(rows: Vec<Vec<String>>) -> Node {
        Node::Table { rows }
    }
    pub fn button(label: &str, call: Call) -> Node {
        Node::Button {
            label: label.to_string(),
            call: Some(call),
            active: false,
            color: None,
            big: false,
        }
    }
    pub fn toggle(label: &str, on: bool, call: Call, arg: &str) -> Node {
        Node::Toggle {
            label: label.to_string(),
            on,
            call,
            arg: arg.to_string(),
        }
    }
    pub fn choice(
        label: &str,
        value: &str,
        options: Vec<String>,
        pick: Pick,
        call: Call,
        arg: &str,
    ) -> Node {
        Node::Choice {
            label: label.to_string(),
            value: value.to_string(),
            options,
            pick,
            call,
            arg: arg.to_string(),
        }
    }
    pub fn range(
        label: &str,
        value: i64,
        min: i64,
        max: i64,
        step: i64,
        call: Call,
        arg: &str,
    ) -> Node {
        Node::Range {
            label: label.to_string(),
            value,
            min,
            max,
            step,
            scale: 1,
            call,
            arg: arg.to_string(),
        }
    }
    pub fn field(value: &str, hint: &str, call: Call, arg: &str) -> Node {
        Node::Field {
            value: value.to_string(),
            hint: hint.to_string(),
            live: false,
            call,
            arg: arg.to_string(),
            enter: None,
        }
    }
    pub fn cell(call: Option<Call>) -> Node {
        Node::Cell {
            on: false,
            color: None,
            child: None,
            call,
        }
    }
    pub fn label(text: &str, note: &str, call: Option<Call>) -> Node {
        Node::Label {
            symbol: None,
            text: text.to_string(),
            note: note.to_string(),
            call,
        }
    }
    pub fn canvas(fact: Json, tap: Option<Call>, drag: Option<Call>) -> Node {
        Node::Canvas {
            fact,
            grid: None,
            tap,
            drag,
            turn: None,
            zoom: None,
            pan: None,
        }
    }
    pub fn active(mut self, is: bool) -> Node {
        match &mut self {
            Node::Button { active, .. } => *active = is,
            Node::Cell { on, .. } => *on = is,
            _ => {}
        }
        self
    }
    pub fn tint(mut self, hex: &str) -> Node {
        match &mut self {
            Node::Button { color, .. } | Node::Cell { color, .. } => *color = Some(hex.to_string()),
            _ => {}
        }
        self
    }
    pub fn big(mut self) -> Node {
        if let Node::Button { big, .. } = &mut self {
            *big = true;
        }
        self
    }
    pub fn live(mut self) -> Node {
        if let Node::Field { live, .. } = &mut self {
            *live = true;
        }
        self
    }
    pub fn enter(mut self, call: Call) -> Node {
        if let Node::Field { enter, .. } = &mut self {
            *enter = Some(call);
        }
        self
    }
    pub fn scaled(mut self, by: i64) -> Node {
        if let Node::Range { scale, .. } = &mut self {
            *scale = by.max(1);
        }
        self
    }
    pub fn badge(mut self, value: &str) -> Node {
        if let Node::Label { symbol, .. } = &mut self {
            *symbol = Some(value.to_string());
        }
        self
    }
    pub fn hold(mut self, node: Node) -> Node {
        if let Node::Cell { child, .. } = &mut self {
            *child = Some(Box::new(node));
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;

    #[test]
    fn fill_merges_the_named_arg() {
        let call = Call::new("snake.set", json!({ "key": "grid" }));
        let filled = call.fill("value", json!(12));
        assert_eq!(filled.args, json!({ "key": "grid", "value": 12 }));
        assert_eq!(call.args, json!({ "key": "grid" }));
    }

    #[test]
    fn flags_land_on_their_kind() {
        let swatch = Node::button("", Call::new("set", json!({})))
            .tint("#ff0000")
            .big()
            .active(true);
        match swatch {
            Node::Button {
                active, color, big, ..
            } => {
                assert!(active);
                assert!(big);
                assert_eq!(color.as_deref(), Some("#ff0000"));
            }
            _ => panic!(),
        }
        let plain = Node::text("hi", Role::Note).big();
        assert_eq!(plain, Node::text("hi", Role::Note));
    }
}
