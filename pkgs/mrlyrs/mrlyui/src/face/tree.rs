use super::layout::{shift, Op};
use super::text;
use super::{Act, Hit, UiState};
use crate::tokens::{
    contrast, Theme, CANVAS, CHEV, CHROME, CONTROL, EDGE, GAP, GLYPH, GRIP, INSET, KNOB, LINE, PAD,
    RAIL, ROW, RULE, SLACK, SLOT, SPLIT, STUB, SWITCH_H, SWITCH_W, SYMBOL, TEXT, THUMB, TIGHT,
    TILE, TITLE, UNIT,
};
use mrlycore::ui::{Call, Node, Pick, Role};
use mrlycore::{json, Color};

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
        line(out, &piece, x, y + dy, w, TEXT, color);
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
        Node::Button { label, .. } => text::width(label, TEXT) + CHROME,
        Node::Text { text: txt, .. } => text::width(txt, TEXT) + TIGHT,
        Node::Symbol { value } => {
            if crate::symbol::known(value) {
                SYMBOL + 2 * TIGHT
            } else {
                text::width(value, TITLE) + 2 * TIGHT
            }
        }
        Node::Label {
            text: txt, note, ..
        } => text::width(txt, TEXT) + text::width(note, TEXT) + SLACK,
        Node::Cell { .. } => TILE,
        _ => return None,
    };
    Some(natural.clamp(STUB, w))
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
        let scale = (w / iw.max(1)).min(CANVAS / ih.max(1));
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
            return (
                ih * scale + TIGHT,
                Some((px, y, iw * scale, ih * scale, scale)),
            );
        }
    }
    let fw = fact["width"].as_u64().unwrap_or(0);
    let fh = fact["height"].as_u64().unwrap_or(0);
    line(
        out,
        &format!("frame {fw}x{fh}"),
        x,
        y + (ROW - LINE * TEXT) / 2,
        w,
        TEXT,
        t.muted,
    );
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
    let h = if *big { w } else { CONTROL };
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
    let cut = text::truncate(label, w.saturating_sub(2 * GAP), TEXT);
    let tw = text::width(&cut, TEXT);
    line(
        out,
        &cut,
        x + (w.saturating_sub(tw)) / 2,
        y + (h.saturating_sub(LINE)) / 2,
        w,
        TEXT,
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
    line(
        out,
        label,
        x,
        y + (CONTROL - LINE) / 2,
        w.saturating_sub(SWITCH_W + PAD),
        TEXT,
        t.ink,
    );
    let tx = x + w - SWITCH_W;
    let ty = y + (CONTROL - SWITCH_H) / 2;
    let fill = if on { t.accent } else { t.faint };
    out.ops.push(Op::Rect {
        x: tx,
        y: ty,
        w: SWITCH_W,
        h: SWITCH_H,
        color: fill,
    });
    let kx = if on {
        tx + SWITCH_W - KNOB - EDGE
    } else {
        tx + EDGE
    };
    out.ops.push(Op::Rect {
        x: kx,
        y: ty + EDGE,
        w: KNOB,
        h: KNOB,
        color: t.board,
    });
    out.hits.push(Hit {
        x,
        y,
        w,
        h: CONTROL,
        act: Act::Tap {
            call: call.fill(arg, json!(!on)),
        },
    });
    CONTROL
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
    let cw = (w.saturating_sub((n - 1) * TIGHT)) / n;
    for (i, option) in options.iter().enumerate() {
        let ox = x + i * (cw + TIGHT);
        let on = option == value;
        let fill = if on { t.accent } else { t.faint };
        out.ops.push(Op::Rect {
            x: ox,
            y,
            w: cw,
            h: CONTROL,
            color: fill,
        });
        let ink = if on { contrast(fill) } else { t.ink };
        let cut = text::truncate(option, cw.saturating_sub(PAD), TEXT);
        let tw = text::width(&cut, TEXT);
        line(
            out,
            &cut,
            ox + (cw.saturating_sub(tw)) / 2,
            y + INSET,
            cw,
            TEXT,
            ink,
        );
        out.hits.push(Hit {
            x: ox,
            y,
            w: cw,
            h: CONTROL,
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
        line(out, label, x, y, w, TEXT, t.muted);
        dy += CONTROL;
    }
    match pick {
        Pick::Segments => segments(value, options, call, arg, x, y + dy, w, t, out),
        Pick::Cycle => {
            out.ops.push(Op::Rect {
                x,
                y: y + dy,
                w,
                h: CONTROL,
                color: t.faint,
            });
            let cut = text::truncate(value, w.saturating_sub(2 * (CHEV + TIGHT)), TEXT);
            let tw = text::width(&cut, TEXT);
            line(
                out,
                &cut,
                x + (w.saturating_sub(tw)) / 2,
                y + dy + INSET,
                w,
                TEXT,
                t.ink,
            );
            line(out, ">", x + w - CHEV, y + dy + INSET, CHEV, TEXT, t.muted);
            let at = options.iter().position(|o| o == value).unwrap_or(0);
            if let Some(next) = options.get((at + 1) % options.len().max(1)) {
                out.hits.push(Hit {
                    x,
                    y: y + dy,
                    w,
                    h: CONTROL,
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
                h: CONTROL,
                color: t.faint,
            });
            line(
                out,
                value,
                x + INSET,
                y + dy + INSET,
                w.saturating_sub(INSET + CHEV + TIGHT),
                TEXT,
                t.ink,
            );
            line(out, "v", x + w - CHEV, y + dy + INSET, CHEV, TEXT, t.muted);
            let key = id(call, arg);
            out.hits.push(Hit {
                x,
                y: y + dy,
                w,
                h: CONTROL,
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
    dy + CONTROL
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
    let vw = text::width(&shown, TEXT);
    line(
        out,
        label,
        x,
        y + UNIT,
        w.saturating_sub(vw + SPLIT),
        TEXT,
        t.muted,
    );
    line(
        out,
        &shown,
        x + w.saturating_sub(vw),
        y + UNIT,
        vw + TIGHT,
        TEXT,
        t.ink,
    );
    let ty = y + UNIT + LINE * TEXT;
    let band = CONTROL.saturating_sub(UNIT + LINE * TEXT);
    let span = (max - min).max(1);
    let frac = ((*value - min).clamp(0, span)) as f64 / span as f64;
    out.ops.push(Op::Rect {
        x,
        y: ty + (band - RAIL) / 2,
        w,
        h: RAIL,
        color: t.faint,
    });
    let filled = (w as f64 * frac) as usize;
    out.ops.push(Op::Rect {
        x,
        y: ty + (band - RAIL) / 2,
        w: filled,
        h: RAIL,
        color: t.accent,
    });
    out.ops.push(Op::Rect {
        x: (x + filled).min(x + w.saturating_sub(THUMB)),
        y: ty + (band.saturating_sub(GRIP)) / 2,
        w: THUMB,
        h: GRIP,
        color: t.ink,
    });
    out.hits.push(Hit {
        x,
        y,
        w,
        h: CONTROL,
        act: Act::Slide {
            call: call.clone(),
            arg: arg.clone(),
            min: *min,
            max: *max,
            step: (*step).max(1),
        },
    });
    CONTROL
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
    outline(out, x, y, w, CONTROL, EDGE, border);
    let shown = focused.map_or(value.as_str(), |e| e.buffer.as_str());
    if shown.is_empty() && focused.is_none() {
        line(
            out,
            hint,
            x + INSET,
            y + INSET,
            w.saturating_sub(2 * INSET),
            TEXT,
            t.muted,
        );
    } else {
        let cut = text::truncate(shown, w.saturating_sub(2 * INSET + GAP), TEXT);
        line(
            out,
            &cut,
            x + INSET,
            y + INSET,
            w.saturating_sub(2 * INSET),
            TEXT,
            t.ink,
        );
        if focused.is_some() {
            out.ops.push(Op::Rect {
                x: x + INSET + TIGHT + text::width(&cut, TEXT),
                y: y + (CONTROL - LINE - TIGHT) / 2,
                w: EDGE,
                h: LINE + TIGHT,
                color: t.accent,
            });
        }
    }
    out.hits.push(Hit {
        x,
        y,
        w,
        h: CONTROL,
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
    CONTROL
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
        outline(out, x, y, w, w, 2 * EDGE, t.accent);
    }
    if let Some(inner) = child {
        let mut scratch = Out::new();
        let iw = w.saturating_sub(2 * GAP);
        let h = lay(inner, 0, 0, iw, t, ui, &mut scratch);
        out.absorb(scratch, x + GAP, y + (w.saturating_sub(h)) / 2);
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
        line(
            out,
            sym,
            x,
            y + (CONTROL - LINE * TEXT) / 2,
            SLOT,
            TEXT,
            t.accent,
        );
        tx += SLOT + TIGHT;
    }
    let (text_ink, note_ink) = if call.is_some() {
        (t.ink, t.muted)
    } else {
        (t.muted, t.ink)
    };
    let nw = text::width(note, TEXT);
    let nx = x + w.saturating_sub(nw);
    line(
        out,
        txt,
        tx,
        y + (CONTROL - LINE * TEXT) / 2,
        nx.saturating_sub(tx + PAD),
        TEXT,
        text_ink,
    );
    line(
        out,
        note,
        nx,
        y + (CONTROL - LINE * TEXT) / 2,
        nw + TIGHT,
        TEXT,
        note_ink,
    );
    if let Some(call) = call {
        out.hits.push(Hit {
            x,
            y,
            w,
            h: CONTROL,
            act: Act::Tap { call: call.clone() },
        });
    }
    CONTROL
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
            let h = stacked(
                children,
                0,
                0,
                w.saturating_sub(2 * PAD),
                t,
                ui,
                &mut scratch,
            );
            outline(out, x, y, w, h + 2 * PAD, EDGE, t.faint);
            out.absorb(scratch, x + PAD, y + PAD);
            h + 2 * PAD
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
                line(out, txt, x, y, w, TITLE, t.ink);
                LINE * TITLE + TIGHT
            }
            Role::Label => {
                line(
                    out,
                    txt,
                    x,
                    y + (CONTROL - LINE * TEXT) / 2,
                    w,
                    TEXT,
                    t.accent,
                );
                CONTROL
            }
            Role::Note => wrapped(out, txt, x, y, w, t.muted),
            Role::Body => wrapped(out, txt, x, y, w, t.ink),
        },
        Node::Rule => {
            out.ops.push(Op::Rect {
                x,
                y: y + (RULE - EDGE) / 2,
                w,
                h: EDGE,
                color: t.faint,
            });
            RULE
        }
        Node::Image { fact } => picture(fact, x, y, w, t, out).0,
        Node::Symbol { value } => {
            if let Some(sprite) = crate::symbol::sprite(value, SYMBOL, t.ink) {
                out.ops.push(Op::Image {
                    x: x + w.saturating_sub(SYMBOL) / 2,
                    y: y + (GLYPH - SYMBOL) / 2,
                    w: SYMBOL,
                    h: SYMBOL,
                    scale: 1,
                    pixels: sprite.to_vec(),
                });
            } else {
                let tw = text::width(value, TITLE);
                line(
                    out,
                    value,
                    x + (w.saturating_sub(tw)) / 2,
                    y + (GLYPH - LINE * TITLE) / 2,
                    w,
                    TITLE,
                    t.ink,
                );
            }
            GLYPH
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
                    line(out, cell_text, x + c * (cw + GAP), y + dy, cw, TEXT, color);
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
