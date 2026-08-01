use super::text;
use super::{FaceInput, Theme, WIDTH};

pub(crate) const PAD: usize = 6;
pub(crate) const FIELD: usize = WIDTH - 2 * PAD;
pub(crate) const ROW: usize = 11;
pub(crate) const INDENT: usize = 10;
pub(crate) const TITLE_H: usize = 20;
pub(crate) const ACTION_CAP: usize = 8;

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

pub(crate) fn title_ops(input: &FaceInput, theme: &Theme) -> Vec<Op> {
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

pub(crate) fn action_bar(input: &FaceInput, theme: &Theme) -> Vec<Item> {
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

pub(crate) fn shift(ops: Vec<Op>, dx: usize, dy: usize) -> Vec<Op> {
    ops.into_iter()
        .map(|op| match op {
            Op::Rect { x, y, w, h, color } => Op::Rect {
                x: x + dx,
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
                x: x + dx,
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
                x: x + dx,
                y: y + dy,
                w,
                h,
                scale,
                pixels,
            },
        })
        .collect()
}
