use super::FaceInput;
use crate::tokens::LIST;
use mrlycore::ui::{Node, Role};
use mrlycore::Json;

pub(crate) fn scalar_text(value: &Json) -> String {
    match value {
        Json::Null => "null".to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Int(n) => n.to_string(),
        Json::Str(s) => {
            if s.starts_with("data:") {
                format!("[png {}b]", s.len())
            } else {
                s.clone()
            }
        }
        _ => String::new(),
    }
}

fn is_scalar(value: &Json) -> bool {
    !value.is_array() && !value.is_object()
}

fn is_grid(value: &Json) -> bool {
    let Some(items) = value.as_array() else {
        return false;
    };
    if items.is_empty() || !items.iter().all(Json::is_array) {
        return false;
    }
    items[0]
        .as_array()
        .is_some_and(|row| !row.is_empty() && row.iter().all(Json::is_number))
}

fn brief(value: &Json) -> String {
    match value {
        Json::Arr(items) => format!("[{}]", items.len()),
        Json::Obj(map) => {
            let mut parts: Vec<String> = map
                .iter()
                .take(3)
                .map(|(k, v)| {
                    if is_scalar(v) {
                        format!("{k}: {}", scalar_text(v))
                    } else {
                        format!("{k}: {}", brief(v))
                    }
                })
                .collect();
            if map.len() > 3 {
                parts.push("..".to_string());
            }
            parts.join(", ")
        }
        _ => scalar_text(value),
    }
}

fn inline_list(items: &[Json]) -> String {
    let parts: Vec<String> = items.iter().map(scalar_text).collect();
    format!("[{}]", parts.join(", "))
}

fn value_nodes(key: &str, value: &Json) -> Vec<Node> {
    if is_scalar(value) {
        return vec![Node::label(key, &scalar_text(value), None)];
    }
    if is_grid(value) {
        let rows = value.as_array().map_or(0, Vec::len);
        let cols = value[0].as_array().map_or(0, Vec::len);
        return vec![Node::label(key, &format!("{rows}x{cols}"), None)];
    }
    if let Some(items) = value.as_array() {
        if items.iter().all(is_scalar) {
            return vec![Node::label(key, &inline_list(items), None)];
        }
        let mut out = vec![Node::text(key, Role::Note)];
        for item in items.iter().take(LIST) {
            out.push(Node::label(&format!("  {}", brief(item)), "", None));
        }
        if items.len() > LIST {
            out.push(Node::text(
                &format!("  + {} more", items.len() - LIST),
                Role::Note,
            ));
        }
        return out;
    }
    let Some(map) = value.as_object() else {
        return Vec::new();
    };
    let mut out = vec![Node::text(key, Role::Note)];
    for (k, v) in map {
        let shown = if is_scalar(v) {
            scalar_text(v)
        } else {
            brief(v)
        };
        out.push(Node::label(&format!("  {k}"), &shown, None));
    }
    out
}

pub(crate) fn tree(input: &FaceInput) -> Node {
    let mut nodes = Vec::new();
    if let Some(params) = input.params.as_object() {
        for (k, v) in params {
            nodes.push(Node::label(k, &scalar_text(v), None));
        }
    }
    match &input.state {
        Json::Null => nodes.push(Node::text("no state", Role::Note)),
        Json::Obj(map) => {
            if let Some(fact) = map.get("frame").filter(|f| !f.is_null()) {
                nodes.push(Node::image(fact.clone()));
            }
            for (k, v) in map {
                if k == "frame" || k == "shade" || k == "md" {
                    continue;
                }
                nodes.extend(value_nodes(k, v));
            }
            if let Some(md) = map.get("md").and_then(Json::as_str) {
                nodes.push(Node::doc(md));
            }
        }
        other => nodes.extend(value_nodes("state", other)),
    }
    Node::column(nodes)
}
