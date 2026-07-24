use super::text;
use super::{FaceInput, Theme, MAX_HEIGHT, MIN_HEIGHT, WIDTH};
use serde_json::Value as Json;

pub(crate) const PAD: usize = 6;
pub(crate) const FIELD: usize = WIDTH - 2 * PAD;
pub(crate) const ROW: usize = 11;
pub(crate) const INDENT: usize = 10;
const TITLE_H: usize = 20;
const CANVAS_MAX_H: usize = 192;
const LIST_CAP: usize = 12;
const ACTION_CAP: usize = 8;

pub(crate) enum Op {
    Rect {
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: [u8; 4],
    },
    Text {
        x: usize,
        y: usize,
        text: String,
        scale: usize,
        color: [u8; 4],
    },
    Image {
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        scale: usize,
        pixels: Vec<[u8; 4]>,
    },
}

pub(crate) struct Item {
    pub height: usize,
    pub ops: Vec<Op>,
}

pub(crate) fn row(line: String, indent: usize, scale: usize, color: [u8; 4]) -> Item {
    let x = PAD + indent * INDENT;
    let field = (WIDTH - PAD).saturating_sub(x);
    let cut = text::truncate(&line, field, scale);
    Item {
        height: text::LINE * scale + 4,
        ops: vec![Op::Text {
            x,
            y: 0,
            text: cut,
            scale,
            color,
        }],
    }
}

fn kv_row(key: &str, value: String, indent: usize, theme: &Theme, value_color: [u8; 4]) -> Item {
    let x = PAD + indent * INDENT;
    let key_cut = text::truncate(key, 120, 1);
    let key_w = text::width(&key_cut, 1);
    let vx = x + key_w + 6;
    let field = (WIDTH - PAD).saturating_sub(vx);
    let value_cut = text::truncate(&value, field, 1);
    Item {
        height: ROW,
        ops: vec![
            Op::Text {
                x,
                y: 0,
                text: key_cut,
                scale: 1,
                color: theme.muted,
            },
            Op::Text {
                x: vx,
                y: 0,
                text: value_cut,
                scale: 1,
                color: value_color,
            },
        ],
    }
}

fn scalar_text(value: &Json) -> String {
    match value {
        Json::Null => "null".to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Number(n) => n.to_string(),
        Json::String(s) => {
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
        Json::Array(items) => format!("[{}]", items.len()),
        Json::Object(map) => {
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

fn value_items(key: &str, value: &Json, theme: &Theme) -> Vec<Item> {
    if is_scalar(value) {
        return vec![kv_row(key, scalar_text(value), 0, theme, theme.ink)];
    }
    if is_grid(value) {
        let rows = value.as_array().map_or(0, Vec::len);
        let cols = value[0].as_array().map_or(0, Vec::len);
        return vec![kv_row(key, format!("{rows}x{cols}"), 0, theme, theme.muted)];
    }
    if let Some(items) = value.as_array() {
        if items.iter().all(is_scalar) {
            return vec![kv_row(key, inline_list(items), 0, theme, theme.ink)];
        }
        let mut out = vec![row(key.to_string(), 0, 1, theme.muted)];
        for item in items.iter().take(LIST_CAP) {
            out.push(row(brief(item), 1, 1, theme.ink));
        }
        if items.len() > LIST_CAP {
            out.push(row(
                format!("+ {} more", items.len() - LIST_CAP),
                1,
                1,
                theme.muted,
            ));
        }
        return out;
    }
    let Some(map) = value.as_object() else {
        return Vec::new();
    };
    let mut out = vec![row(key.to_string(), 0, 1, theme.muted)];
    for (k, v) in map {
        if is_scalar(v) {
            out.push(kv_row(k, scalar_text(v), 1, theme, theme.ink));
        } else {
            out.push(kv_row(k, brief(v), 1, theme, theme.ink));
        }
    }
    out
}

fn canvas_items(state: &Json, theme: &Theme) -> Vec<Item> {
    let fact = &state["frame"];
    if fact.is_null() {
        return Vec::new();
    }
    if let Some((w, h, pixels)) = super::decode(fact) {
        let scale = (FIELD / w).min(CANVAS_MAX_H / h);
        if scale >= 1 {
            let x = PAD + (FIELD - w * scale) / 2;
            return vec![Item {
                height: h * scale + 4,
                ops: vec![Op::Image {
                    x,
                    y: 0,
                    w,
                    h,
                    scale,
                    pixels,
                }],
            }];
        }
    }
    let w = fact["width"].as_u64().unwrap_or(0);
    let h = fact["height"].as_u64().unwrap_or(0);
    vec![row(format!("frame {w}x{h}"), 0, 1, theme.muted)]
}

fn body_items(input: &FaceInput, theme: &Theme) -> Vec<Item> {
    let mut items = Vec::new();
    if let Some(params) = input.params.as_object() {
        for (k, v) in params {
            items.push(kv_row(k, scalar_text(v), 0, theme, theme.muted));
        }
    }
    match &input.state {
        Json::Null => items.push(row("no state".to_string(), 0, 1, theme.muted)),
        Json::Object(map) => {
            items.extend(canvas_items(&input.state, theme));
            for (k, v) in map {
                if k == "frame" || k == "shade" || k == "md" {
                    continue;
                }
                items.extend(value_items(k, v, theme));
            }
            if let Some(md) = map.get("md").and_then(Json::as_str) {
                items.extend(super::md::items(md, theme));
            }
        }
        other => items.extend(value_items("state", other, theme)),
    }
    items
}

fn title_ops(input: &FaceInput, theme: &Theme) -> Vec<Op> {
    let mut ops = vec![Op::Rect {
        x: PAD,
        y: 6,
        w: 8,
        h: 8,
        color: theme.accent,
    }];
    let mut right = WIDTH - PAD;
    if let Some(beat) = &input.beat {
        let name = text::truncate(beat, 110, 1);
        let bw = text::width(&name, 1);
        let bx = (WIDTH - PAD).saturating_sub(bw);
        ops.push(Op::Text {
            x: bx,
            y: 6,
            text: name,
            scale: 1,
            color: theme.muted,
        });
        right = bx.saturating_sub(6);
    }
    let tx = PAD + 12;
    let title = text::truncate(&input.title, right.saturating_sub(tx), 2);
    ops.push(Op::Text {
        x: tx,
        y: 3,
        text: title,
        scale: 2,
        color: theme.ink,
    });
    ops.push(Op::Rect {
        x: 0,
        y: TITLE_H - 1,
        w: WIDTH,
        h: 1,
        color: theme.faint,
    });
    ops
}

fn action_bar(input: &FaceInput, theme: &Theme) -> Vec<Item> {
    if input.actions.is_empty() {
        return vec![row("no actions".to_string(), 0, 1, theme.muted)];
    }
    let mut items = Vec::new();
    for verb in input.actions.iter().take(ACTION_CAP) {
        let name_cut = text::truncate(&verb.name, 160, 1);
        let name_w = text::width(&name_cut, 1);
        let mut ops = vec![Op::Text {
            x: PAD,
            y: 0,
            text: name_cut,
            scale: 1,
            color: theme.accent,
        }];
        let hint = verb
            .args
            .as_object()
            .map(|m| {
                m.iter()
                    .map(|(k, v)| {
                        format!(
                            "{k}:{}",
                            v.as_str().map_or_else(|| v.to_string(), str::to_string)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();
        if !hint.is_empty() {
            let hx = PAD + name_w + 8;
            let field = (WIDTH - PAD).saturating_sub(hx);
            ops.push(Op::Text {
                x: hx,
                y: 0,
                text: text::truncate(&hint, field, 1),
                scale: 1,
                color: theme.muted,
            });
        }
        items.push(Item { height: ROW, ops });
    }
    if input.actions.len() > ACTION_CAP {
        items.push(row(
            format!("+ {} more", input.actions.len() - ACTION_CAP),
            0,
            1,
            theme.muted,
        ));
    }
    items
}

fn shift(ops: Vec<Op>, dy: usize) -> Vec<Op> {
    ops.into_iter()
        .map(|op| match op {
            Op::Rect { x, y, w, h, color } => Op::Rect {
                x,
                y: y + dy,
                w,
                h,
                color,
            },
            Op::Text {
                x,
                y,
                text,
                scale,
                color,
            } => Op::Text {
                x,
                y: y + dy,
                text,
                scale,
                color,
            },
            Op::Image {
                x,
                y,
                w,
                h,
                scale,
                pixels,
            } => Op::Image {
                x,
                y: y + dy,
                w,
                h,
                scale,
                pixels,
            },
        })
        .collect()
}

pub(crate) fn layout(input: &FaceInput, theme: &Theme) -> (usize, Vec<Op>) {
    let mut ops = title_ops(input, theme);
    let bar = action_bar(input, theme);
    let bar_h = 6 + bar.iter().map(|i| i.height).sum::<usize>();
    let body = body_items(input, theme);
    let budget = MAX_HEIGHT.saturating_sub(TITLE_H + PAD + bar_h);
    let mut placed: Vec<Item> = Vec::new();
    let mut used = 0;
    let mut skipped = 0;
    for item in body {
        if skipped == 0 && used + item.height <= budget {
            used += item.height;
            placed.push(item);
        } else {
            skipped += 1;
        }
    }
    if skipped > 0 {
        while used + ROW > budget {
            match placed.pop() {
                Some(item) => {
                    used -= item.height;
                    skipped += 1;
                }
                None => break,
            }
        }
        let more = row(format!("+ {skipped} more"), 0, 1, theme.muted);
        used += more.height;
        placed.push(more);
    }
    let height = (TITLE_H + PAD + used + bar_h).clamp(MIN_HEIGHT, MAX_HEIGHT);
    let mut y = TITLE_H + PAD;
    for item in placed {
        let h = item.height;
        ops.extend(shift(item.ops, y));
        y += h;
    }
    let mut by = height - bar_h;
    ops.push(Op::Rect {
        x: 0,
        y: by,
        w: WIDTH,
        h: 1,
        color: theme.faint,
    });
    by += 6;
    for item in bar {
        let h = item.height;
        ops.extend(shift(item.ops, by));
        by += h;
    }
    (height, ops)
}
