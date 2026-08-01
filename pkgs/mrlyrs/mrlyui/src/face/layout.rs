use super::text;
use super::FaceInput;
use crate::tokens::{
    Theme, ACTIONS, BEAT, EDGE, GAP, HEADER, INDENT, LEAD, LINE, MARK, PAD, ROW, SPLIT, TEXT,
    TITLE, VERB, WIDTH,
};

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
        height: LINE * scale + LEAD,
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
        y: (HEADER - MARK) / 2,
        w: MARK,
        h: MARK,
        color: theme.accent,
    }];
    let mut right = WIDTH - PAD;
    if let Some(beat) = &input.beat {
        let name = text::truncate(beat, BEAT, TEXT);
        let bw = text::width(&name, TEXT);
        let bx = (WIDTH - PAD).saturating_sub(bw);
        ops.push(Op::Text {
            x: bx,
            y: (HEADER - LINE * TEXT) / 2,
            text: name,
            scale: TEXT,
            color: theme.muted,
        });
        right = bx.saturating_sub(PAD);
    }
    let tx = PAD + MARK + GAP;
    let title = text::truncate(&input.title, right.saturating_sub(tx), TITLE);
    ops.push(Op::Text {
        x: tx,
        y: (HEADER - LINE * TITLE) / 2,
        text: title,
        scale: TITLE,
        color: theme.ink,
    });
    ops.push(Op::Rect {
        x: 0,
        y: HEADER - 1,
        w: WIDTH,
        h: EDGE,
        color: theme.faint,
    });
    ops
}

pub(crate) fn action_bar(input: &FaceInput, theme: &Theme) -> Vec<Item> {
    if input.actions.is_empty() {
        return vec![row("no actions".to_string(), 0, TEXT, theme.muted)];
    }
    let mut items = Vec::new();
    for verb in input.actions.iter().take(ACTIONS) {
        let name_cut = text::truncate(&verb.name, VERB, TEXT);
        let name_w = text::width(&name_cut, TEXT);
        let mut ops = vec![Op::Text {
            x: PAD,
            y: 0,
            text: name_cut,
            scale: TEXT,
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
            let hx = PAD + name_w + SPLIT;
            let field = (WIDTH - PAD).saturating_sub(hx);
            ops.push(Op::Text {
                x: hx,
                y: 0,
                text: text::truncate(&hint, field, TEXT),
                scale: TEXT,
                color: theme.muted,
            });
        }
        items.push(Item { height: ROW, ops });
    }
    if input.actions.len() > ACTIONS {
        items.push(row(
            format!("+ {} more", input.actions.len() - ACTIONS),
            0,
            TEXT,
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
