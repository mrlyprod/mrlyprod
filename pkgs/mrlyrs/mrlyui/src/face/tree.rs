use super::layout::{shift, Op, PAD, ROW};
use super::text;
use super::{Act, Hit, Theme, UiState};
use mrlycore::ui::{Call, Node, Pick, Role};
use mrlycore::{json, Color};

pub(crate) const GAP: usize = 4;
const BTN_H: usize = 18;
const LABEL_H: usize = 12;
const TOGGLE_H: usize = 14;
const FIELD_H: usize = 18;
const CANVAS_MAX_H: usize = 192;
const SEG_GAP: usize = 2;

pub(crate) struct Out {
    pub ops: Vec<Op>,
    pub hits: Vec<Hit>,
    pub overlays: Vec<(Node, Act)>,
}

impl Out {
    pub fn new() -> Out {
        Out {
            ops: Vec::new(),
            hits: Vec::new(),
            overlays: Vec::new(),
        }
    }
    fn absorb(&mut self, other: Out, dx: usize, dy: usize) {
        self.ops.extend(shift(other.ops, dx, dy));
        for mut hit in other.hits {
            hit.x += dx;
            hit.y += dy;
            self.hits.push(hit);
        }
        self.overlays.extend(other.overlays);
    }
}

fn id(call: &Call, arg: &str) -> String {
    format!("{}:{}:{}", call.verb, arg, call.args)
}

fn tint(hex: &str, fallback: [u8; 4]) -> [u8; 4] {
    Color::from_hex(hex).map_or(fallback, |c| [c.r, c.g, c.b, c.a])
}

pub(crate) fn contrast(fill: [u8; 4]) -> [u8; 4] {
    let luma = 0.299 * fill[0] as f64 + 0.587 * fill[1] as f64 + 0.114 * fill[2] as f64;
    if luma > 140.0 {
        [0, 0, 0, 255]
    } else {
        [255, 255, 255, 255]
    }
}

fn outline(out: &mut Out, x: usize, y: usize, w: usize, h: usize, thick: usize, color: [u8; 4]) {
    out.ops.push(Op::Rect {
        x,
        y,
        w,
        h: thick,
        color,
    });
    out.ops.push(Op::Rect {
        x,
        y: y + h.saturating_sub(thick),
        w,
        h: thick,
        color,
    });
    out.ops.push(Op::Rect {
        x,
        y,
        w: thick,
        h,
        color,
    });
    out.ops.push(Op::Rect {
        x: x + w.saturating_sub(thick),
        y,
        w: thick,
        h,
        color,
    });
}

fn line(out: &mut Out, txt: &str, x: usize, y: usize, w: usize, scale: usize, color: [u8; 4]) {
    let cut = text::truncate(txt, w, scale);
    if !cut.is_empty() {
        out.ops.push(Op::Text {
            x,
            y,
            text: cut,
            scale,
            color,
        });
    }
}

fn wrapped(out: &mut Out, txt: &str, x: usize, y: usize, w: usize, color: [u8; 4]) -> usize {
    let mut dy = 0;
    for piece in super::md::wrap(txt, w) {
        line(out, &piece, x, y + dy, w, 1, color);
        dy += ROW;
    }
    dy
}

fn stacked(
    children: &[Node],
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    ui: &UiState,
    out: &mut Out,
) -> usize {
    let mut dy = 0;
    for child in children {
        let mut scratch = Out::new();
        let h = lay(child, 0, 0, w, t, ui, &mut scratch);
        out.absorb(scratch, x, y + dy);
        dy += h + GAP;
    }
    dy.saturating_sub(GAP)
}

fn gridded(
    children: &[Node],
    cols: usize,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    ui: &UiState,
    out: &mut Out,
) -> usize {
    let cols = cols.max(1);
    let cw = (w.saturating_sub((cols - 1) * GAP)) / cols;
    let mut dy = 0;
    for band in children.chunks(cols) {
        let mut tall = 0;
        for (i, child) in band.iter().enumerate() {
            let mut scratch = Out::new();
            let h = lay(child, 0, 0, cw, t, ui, &mut scratch);
            out.absorb(scratch, x + i * (cw + GAP), y + dy);
            tall = tall.max(h);
        }
        dy += tall + GAP;
    }
    dy.saturating_sub(GAP)
}

fn slim(node: &Node, w: usize) -> Option<usize> {
    let natural = match node {
        Node::Button { label, .. } => text::width(label, 1) + 14,
        Node::Text { text: txt, .. } => text::width(txt, 1) + 2,
        Node::Symbol { value } => {
            if crate::symbol::known(value) {
                20
            } else {
                text::width(value, 2) + 4
            }
        }
        Node::Label {
            text: txt, note, ..
        } => text::width(txt, 1) + text::width(note, 1) + 12,
        Node::Cell { .. } => 20,
        _ => return None,
    };
    Some(natural.clamp(16, w))
}

fn picture(
    fact: &mrlycore::Json,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    out: &mut Out,
) -> (usize, Option<(usize, usize, usize, usize, usize)>) {
    if let Some((iw, ih, pixels)) = super::decode(fact) {
        let scale = (w / iw.max(1)).min(CANVAS_MAX_H / ih.max(1));
        if scale >= 1 {
            let px = x + (w - iw * scale) / 2;
            out.ops.push(Op::Image {
                x: px,
                y,
                w: iw,
                h: ih,
                scale,
                pixels,
            });
            return (ih * scale + 2, Some((px, y, iw * scale, ih * scale, scale)));
        }
    }
    let fw = fact["width"].as_u64().unwrap_or(0);
    let fh = fact["height"].as_u64().unwrap_or(0);
    line(out, &format!("frame {fw}x{fh}"), x, y + 2, w, 1, t.muted);
    (ROW, None)
}

fn button(node: &Node, x: usize, y: usize, w: usize, t: &Theme, out: &mut Out) -> usize {
    let Node::Button {
        label,
        call,
        active,
        color,
        big,
    } = node
    else {
        return 0;
    };
    let h = if *big { w } else { BTN_H };
    let fill = match (active, color) {
        (true, _) => t.accent,
        (false, Some(hex)) => tint(hex, t.faint),
        (false, None) => t.faint,
    };
    out.ops.push(Op::Rect {
        x,
        y,
        w,
        h,
        color: fill,
    });
    let ink = if *active || color.is_some() {
        contrast(fill)
    } else {
        t.ink
    };
    let cut = text::truncate(label, w.saturating_sub(8), 1);
    let tw = text::width(&cut, 1);
    line(
        out,
        &cut,
        x + (w.saturating_sub(tw)) / 2,
        y + (h.saturating_sub(7)) / 2,
        w,
        1,
        ink,
    );
    if let Some(call) = call {
        out.hits.push(Hit {
            x,
            y,
            w,
            h,
            act: Act::Tap { call: call.clone() },
        });
    }
    h
}

fn toggle(
    label: &str,
    on: bool,
    call: &Call,
    arg: &str,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    out: &mut Out,
) -> usize {
    line(out, label, x, y + 3, w.saturating_sub(28), 1, t.ink);
    let tx = x + w - 22;
    let fill = if on { t.accent } else { t.faint };
    out.ops.push(Op::Rect {
        x: tx,
        y: y + 2,
        w: 22,
        h: 10,
        color: fill,
    });
    let kx = if on { tx + 13 } else { tx + 1 };
    out.ops.push(Op::Rect {
        x: kx,
        y: y + 3,
        w: 8,
        h: 8,
        color: t.board,
    });
    out.hits.push(Hit {
        x,
        y,
        w,
        h: TOGGLE_H,
        act: Act::Tap {
            call: call.fill(arg, json!(!on)),
        },
    });
    TOGGLE_H
}

fn segments(
    value: &str,
    options: &[String],
    call: &Call,
    arg: &str,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    out: &mut Out,
) {
    let n = options.len().max(1);
    let cw = (w.saturating_sub((n - 1) * SEG_GAP)) / n;
    for (i, option) in options.iter().enumerate() {
        let ox = x + i * (cw + SEG_GAP);
        let on = option == value;
        let fill = if on { t.accent } else { t.faint };
        out.ops.push(Op::Rect {
            x: ox,
            y,
            w: cw,
            h: BTN_H,
            color: fill,
        });
        let ink = if on { contrast(fill) } else { t.ink };
        let cut = text::truncate(option, cw.saturating_sub(6), 1);
        let tw = text::width(&cut, 1);
        line(
            out,
            &cut,
            ox + (cw.saturating_sub(tw)) / 2,
            y + 5,
            cw,
            1,
            ink,
        );
        out.hits.push(Hit {
            x: ox,
            y,
            w: cw,
            h: BTN_H,
            act: Act::Tap {
                call: call.fill(arg, json!(option)),
            },
        });
    }
}

fn choice(
    node: &Node,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    ui: &UiState,
    out: &mut Out,
) -> usize {
    let Node::Choice {
        label,
        value,
        options,
        pick,
        call,
        arg,
    } = node
    else {
        return 0;
    };
    let mut dy = 0;
    if !label.is_empty() {
        line(out, label, x, y, w, 1, t.muted);
        dy += LABEL_H;
    }
    match pick {
        Pick::Segments => segments(value, options, call, arg, x, y + dy, w, t, out),
        Pick::Cycle => {
            out.ops.push(Op::Rect {
                x,
                y: y + dy,
                w,
                h: BTN_H,
                color: t.faint,
            });
            let cut = text::truncate(value, w.saturating_sub(24), 1);
            let tw = text::width(&cut, 1);
            line(
                out,
                &cut,
                x + (w.saturating_sub(tw)) / 2,
                y + dy + 5,
                w,
                1,
                t.ink,
            );
            line(out, ">", x + w - 10, y + dy + 5, 10, 1, t.muted);
            let at = options.iter().position(|o| o == value).unwrap_or(0);
            if let Some(next) = options.get((at + 1) % options.len().max(1)) {
                out.hits.push(Hit {
                    x,
                    y: y + dy,
                    w,
                    h: BTN_H,
                    act: Act::Tap {
                        call: call.fill(arg, json!(next)),
                    },
                });
            }
        }
        Pick::Menu => {
            out.ops.push(Op::Rect {
                x,
                y: y + dy,
                w,
                h: BTN_H,
                color: t.faint,
            });
            line(
                out,
                value,
                x + 5,
                y + dy + 5,
                w.saturating_sub(20),
                1,
                t.ink,
            );
            line(out, "v", x + w - 10, y + dy + 5, 10, 1, t.muted);
            let key = id(call, arg);
            out.hits.push(Hit {
                x,
                y: y + dy,
                w,
                h: BTN_H,
                act: Act::Menu { id: key.clone() },
            });
            if ui.menu.as_deref() == Some(key.as_str()) {
                let list = options
                    .iter()
                    .map(|o| Node::button(o, call.fill(arg, json!(o))).active(o == value))
                    .collect();
                out.overlays.push((Node::group(list), Act::Shut));
            }
        }
    }
    dy + BTN_H
}

fn range(node: &Node, x: usize, y: usize, w: usize, t: &Theme, out: &mut Out) -> usize {
    let Node::Range {
        label,
        value,
        min,
        max,
        step,
        scale,
        call,
        arg,
    } = node
    else {
        return 0;
    };
    let shown = if *scale > 1 {
        format!("{}", *value as f64 / *scale as f64)
    } else {
        value.to_string()
    };
    let vw = text::width(&shown, 1);
    line(out, label, x, y, w.saturating_sub(vw + 8), 1, t.muted);
    line(out, &shown, x + w.saturating_sub(vw), y, vw + 2, 1, t.ink);
    let ty = y + LABEL_H;
    let span = (max - min).max(1);
    let frac = ((*value - min).clamp(0, span)) as f64 / span as f64;
    out.ops.push(Op::Rect {
        x,
        y: ty + 4,
        w,
        h: 4,
        color: t.faint,
    });
    let filled = (w as f64 * frac) as usize;
    out.ops.push(Op::Rect {
        x,
        y: ty + 4,
        w: filled,
        h: 4,
        color: t.accent,
    });
    out.ops.push(Op::Rect {
        x: (x + filled).min(x + w.saturating_sub(3)),
        y: ty + 1,
        w: 3,
        h: 10,
        color: t.ink,
    });
    out.hits.push(Hit {
        x,
        y: ty,
        w,
        h: 12,
        act: Act::Slide {
            call: call.clone(),
            arg: arg.clone(),
            min: *min,
            max: *max,
            step: (*step).max(1),
        },
    });
    LABEL_H + 12
}

fn field(
    node: &Node,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    ui: &UiState,
    out: &mut Out,
) -> usize {
    let Node::Field {
        value,
        hint,
        live,
        call,
        arg,
        enter,
        keys,
    } = node
    else {
        return 0;
    };
    let key = id(call, arg);
    let focused = ui.edit.as_ref().filter(|e| e.id == key);
    let border = if focused.is_some() { t.accent } else { t.faint };
    outline(out, x, y, w, FIELD_H, 1, border);
    let shown = focused.map_or(value.as_str(), |e| e.buffer.as_str());
    if shown.is_empty() && focused.is_none() {
        line(out, hint, x + 5, y + 5, w.saturating_sub(10), 1, t.muted);
    } else {
        let cut = text::truncate(shown, w.saturating_sub(14), 1);
        line(out, &cut, x + 5, y + 5, w.saturating_sub(10), 1, t.ink);
        if focused.is_some() {
            out.ops.push(Op::Rect {
                x: x + 7 + text::width(&cut, 1),
                y: y + 4,
                w: 1,
                h: 9,
                color: t.accent,
            });
        }
    }
    out.hits.push(Hit {
        x,
        y,
        w,
        h: FIELD_H,
        act: Act::Edit {
            id: key,
            value: value.clone(),
            live: *live,
            call: call.clone(),
            arg: arg.clone(),
            enter: enter.clone(),
            keys: keys.clone(),
        },
    });
    FIELD_H
}

fn cell(
    node: &Node,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    ui: &UiState,
    out: &mut Out,
) -> usize {
    let Node::Cell {
        on,
        color,
        child,
        call,
    } = node
    else {
        return 0;
    };
    let fill = color.as_deref().map_or(t.faint, |hex| tint(hex, t.faint));
    out.ops.push(Op::Rect {
        x,
        y,
        w,
        h: w,
        color: fill,
    });
    if *on {
        outline(out, x, y, w, w, 2, t.accent);
    }
    if let Some(inner) = child {
        let mut scratch = Out::new();
        let iw = w.saturating_sub(8);
        let h = lay(inner, 0, 0, iw, t, ui, &mut scratch);
        out.absorb(scratch, x + 4, y + (w.saturating_sub(h)) / 2);
    }
    if let Some(call) = call {
        out.hits.push(Hit {
            x,
            y,
            w,
            h: w,
            act: Act::Tap { call: call.clone() },
        });
    }
    w
}

fn item(node: &Node, x: usize, y: usize, w: usize, t: &Theme, out: &mut Out) -> usize {
    let Node::Label {
        symbol,
        text: txt,
        note,
        call,
    } = node
    else {
        return 0;
    };
    let mut tx = x;
    if let Some(sym) = symbol {
        line(out, sym, x, y + 2, 12, 1, t.accent);
        tx += 14;
    }
    let (text_ink, note_ink) = if call.is_some() {
        (t.ink, t.muted)
    } else {
        (t.muted, t.ink)
    };
    let nw = text::width(note, 1);
    let nx = x + w.saturating_sub(nw);
    line(out, txt, tx, y + 2, nx.saturating_sub(tx + 6), 1, text_ink);
    line(out, note, nx, y + 2, nw + 2, 1, note_ink);
    if let Some(call) = call {
        out.hits.push(Hit {
            x,
            y,
            w,
            h: LABEL_H,
            act: Act::Tap { call: call.clone() },
        });
    }
    LABEL_H
}

pub(crate) fn lay(
    node: &Node,
    x: usize,
    y: usize,
    w: usize,
    t: &Theme,
    ui: &UiState,
    out: &mut Out,
) -> usize {
    match node {
        Node::Column { children } => stacked(children, x, y, w, t, ui, out),
        Node::Row { children } => gridded(children, children.len(), x, y, w, t, ui, out),
        Node::Grid { cols, children } => gridded(children, *cols, x, y, w, t, ui, out),
        Node::Group { children } => {
            let mut scratch = Out::new();
            let h = stacked(children, 0, 0, w.saturating_sub(12), t, ui, &mut scratch);
            outline(out, x, y, w, h + 12, 1, t.faint);
            out.absorb(scratch, x + 6, y + 6);
            h + 12
        }
        Node::Wrap { children } => {
            let mut cx = 0;
            let mut dy = 0;
            let mut tall = 0;
            for child in children {
                let cw = slim(child, w).unwrap_or(w);
                if cx > 0 && cx + cw > w {
                    cx = 0;
                    dy += tall + GAP;
                    tall = 0;
                }
                let mut scratch = Out::new();
                let h = lay(child, 0, 0, cw, t, ui, &mut scratch);
                out.absorb(scratch, x + cx, y + dy);
                cx += cw + GAP;
                tall = tall.max(h);
            }
            dy + tall
        }
        Node::Overlay { child, close } => {
            let act = close.clone().map_or(Act::Mute, |call| Act::Tap { call });
            out.overlays.push((child.as_ref().clone(), act));
            0
        }
        Node::Text { text: txt, role } => match role {
            Role::Title => {
                line(out, txt, x, y, w, 2, t.ink);
                16
            }
            Role::Label => {
                line(out, txt, x, y + 2, w, 1, t.accent);
                LABEL_H
            }
            Role::Note => wrapped(out, txt, x, y, w, t.muted),
            Role::Body => wrapped(out, txt, x, y, w, t.ink),
        },
        Node::Rule => {
            out.ops.push(Op::Rect {
                x,
                y: y + 2,
                w,
                h: 1,
                color: t.faint,
            });
            5
        }
        Node::Image { fact } => picture(fact, x, y, w, t, out).0,
        Node::Symbol { value } => {
            if let Some(sprite) = crate::symbol::sprite(value, 16, t.ink) {
                out.ops.push(Op::Image {
                    x: x + w.saturating_sub(16) / 2,
                    y: y + 1,
                    w: 16,
                    h: 16,
                    scale: 1,
                    pixels: sprite.to_vec(),
                });
            } else {
                let tw = text::width(value, 2);
                line(
                    out,
                    value,
                    x + (w.saturating_sub(tw)) / 2,
                    y + 2,
                    w,
                    2,
                    t.ink,
                );
            }
            18
        }
        Node::Doc { md } => {
            let mut dy = 0;
            for piece in super::md::items(md, t, w) {
                out.ops
                    .extend(shift(piece.ops, x.saturating_sub(PAD), y + dy));
                dy += piece.height;
            }
            dy
        }
        Node::Table { rows } => {
            let cols = rows.iter().map(Vec::len).max().unwrap_or(1).max(1);
            let cw = (w.saturating_sub((cols - 1) * GAP)) / cols;
            let mut dy = 0;
            for (r, cells) in rows.iter().enumerate() {
                let color = if r == 0 { t.accent } else { t.ink };
                for (c, cell_text) in cells.iter().enumerate() {
                    line(out, cell_text, x + c * (cw + GAP), y + dy, cw, 1, color);
                }
                dy += ROW;
            }
            dy
        }
        Node::Button { .. } => button(node, x, y, w, t, out),
        Node::Toggle {
            label,
            on,
            call,
            arg,
        } => toggle(label, *on, call, arg, x, y, w, t, out),
        Node::Choice { .. } => choice(node, x, y, w, t, ui, out),
        Node::Range { .. } => range(node, x, y, w, t, out),
        Node::Field { .. } => field(node, x, y, w, t, ui, out),
        Node::Cell { .. } => cell(node, x, y, w, t, ui, out),
        Node::Label { .. } => item(node, x, y, w, t, out),
        Node::Canvas {
            fact,
            grid,
            tap,
            drag,
            turn,
            zoom,
            pan,
        } => {
            let (h, placed) = picture(fact, x, y, w, t, out);
            if let Some((px, py, pw, ph, _)) = placed {
                let (cols, rows) = grid
                    .unwrap_or_else(|| super::decode(fact).map_or((1, 1), |(iw, ih, _)| (iw, ih)));
                out.hits.push(Hit {
                    x: px,
                    y: py,
                    w: pw,
                    h: ph,
                    act: Act::Board {
                        cols: cols.max(1),
                        rows: rows.max(1),
                        pw,
                        ph,
                        sunk: 0,
                        tap: tap.clone(),
                        drag: drag.clone(),
                        turn: turn.clone(),
                        zoom: zoom.clone(),
                        pan: pan.clone(),
                    },
                });
            }
            h
        }
    }
}
