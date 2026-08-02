use crate::tokens::{Theme, ACTIONS, CONTROL, EDGE, FULL, GAP, HEADER, PAD};
use mrlycore::errors::Result;
use mrlycore::ui::{Call, Node};
use mrlycore::Json;

mod dump;
pub mod keys;
mod layout;
mod md;
mod paint;
mod text;
mod tree;

pub use crate::tokens::{HEIGHT, SCALE, WIDTH};

pub struct FaceVerb {
    pub name: String,
    pub args: Json,
}

pub struct FaceInput {
    pub app: String,
    pub title: String,
    pub params: Json,
    pub state: Json,
    pub actions: Vec<FaceVerb>,
    pub beat: Option<String>,
    pub dark: bool,
    pub ui: Option<Node>,
    pub rung: usize,
    pub round: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edit {
    pub id: String,
    pub buffer: String,
    pub keys: Option<String>,
    pub shift: bool,
    pub page: usize,
}

impl Edit {
    pub fn new(id: String, buffer: String, keys: Option<String>) -> Edit {
        Edit {
            id,
            buffer,
            keys,
            shift: false,
            page: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiState {
    pub scroll: usize,
    pub edit: Option<Edit>,
    pub menu: Option<String>,
    pub hover: Option<Act>,
    pub ring: Option<Act>,
    pub bar: Option<Bar>,
    pub dock: Option<Edit>,
    pub shade: Option<String>,
    pub rise: u8,
    pub veil: u8,
}

impl Default for UiState {
    fn default() -> UiState {
        UiState {
            scroll: 0,
            edit: None,
            menu: None,
            hover: None,
            ring: None,
            bar: None,
            dock: None,
            shade: None,
            rise: FULL,
            veil: FULL,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bar {
    pub line: String,
    pub hints: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub act: Act,
}

impl Hit {
    pub fn holds(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Act {
    Tap {
        call: Call,
    },
    Slide {
        call: Call,
        arg: String,
        value: i64,
        min: i64,
        max: i64,
        step: i64,
    },
    Board {
        cols: usize,
        rows: usize,
        pw: usize,
        ph: usize,
        sunk: usize,
        tap: Option<Call>,
        drag: Option<Call>,
        turn: Option<Call>,
        zoom: Option<Call>,
        pan: Option<Call>,
    },
    Edit {
        id: String,
        value: String,
        live: bool,
        call: Call,
        arg: String,
        enter: Option<Call>,
        keys: Option<String>,
    },
    Menu {
        id: String,
    },
    Cap(keys::Tap),
    Fill {
        text: String,
    },
    Shut,
    Mute,
}

pub struct Scene {
    pub frame: crate::frame::Frame,
    pub hits: Vec<Hit>,
    pub binds: Vec<Act>,
    pub body: usize,
    pub window: usize,
    pub scroll: usize,
}

fn reveal(hits: &[Hit], ui: &UiState, body: usize, window: usize) -> usize {
    let rest = body.saturating_sub(window);
    let mut scroll = ui.scroll.min(rest);
    let Some(edit) = &ui.edit else {
        return scroll;
    };
    let found = hits
        .iter()
        .find(|hit| matches!(&hit.act, Act::Edit { id, .. } if *id == edit.id));
    let Some(hit) = found else {
        return scroll;
    };
    if hit.y + hit.h + GAP > scroll + window {
        scroll = hit.y + hit.h + GAP - window;
    }
    if hit.y.saturating_sub(GAP) < scroll {
        scroll = hit.y.saturating_sub(GAP);
    }
    scroll.min(rest)
}

fn clip(hit: Hit, scroll: usize, y0: usize, y1: usize) -> Option<Hit> {
    let top = hit.y as i64 - scroll as i64 + y0 as i64;
    let bottom = top + hit.h as i64;
    if bottom <= y0 as i64 || top >= y1 as i64 {
        return None;
    }
    let y = top.max(y0 as i64) as usize;
    let mut act = hit.act;
    if let Act::Board { sunk, .. } = &mut act {
        *sunk = (y as i64 - top) as usize;
    }
    Some(Hit {
        x: hit.x,
        y,
        w: hit.w,
        h: (bottom.min(y1 as i64) as usize) - y,
        act,
    })
}

fn overlays(
    queue: Vec<(Node, Act)>,
    theme: &Theme,
    ui: &UiState,
    sheet: &mut [[u8; 4]],
    hits: &mut Vec<Hit>,
) {
    let (width, height) = (theme.width, theme.height);
    let panel = theme.panel();
    let mut queue = queue;
    while !queue.is_empty() {
        for (node, scrim) in std::mem::take(&mut queue) {
            let lift = ui.veil.max(1) as usize;
            let shade = (70 * lift / FULL as usize) as u8;
            let veil = [theme.ink[0], theme.ink[1], theme.ink[2], shade];
            paint::paint_into(
                sheet,
                width,
                height,
                &[layout::Op::rect(0, 0, width, height, veil)],
            );
            hits.push(Hit {
                x: 0,
                y: 0,
                w: width,
                h: height,
                act: scrim,
            });
            let mut out = tree::Out::new();
            out.hover = ui.hover.clone();
            out.tint = theme.card(0);
            let ph = tree::lay(&node, 0, 0, panel, theme, ui, &mut out);
            let px = (width - panel) / 2;
            let sunk = (height / 16) * (FULL as usize - lift) / FULL as usize;
            let py = (height.saturating_sub(ph) / 2).max(HEADER + PAD) + sunk;
            let rim = 2 * PAD;
            let frame = vec![
                layout::Op::rect(
                    px - rim,
                    py.saturating_sub(rim),
                    panel + 2 * rim,
                    ph + 2 * rim,
                    theme.board,
                ),
                layout::Op::rect(
                    px - rim,
                    py.saturating_sub(rim),
                    panel + 2 * rim,
                    EDGE,
                    theme.faint,
                ),
                layout::Op::rect(
                    px - rim,
                    py + ph + rim - EDGE,
                    panel + 2 * rim,
                    EDGE,
                    theme.faint,
                ),
                layout::Op::rect(
                    px - rim,
                    py.saturating_sub(rim),
                    EDGE,
                    ph + 2 * rim,
                    theme.faint,
                ),
                layout::Op::rect(
                    px + panel + rim - EDGE,
                    py.saturating_sub(rim),
                    EDGE,
                    ph + 2 * rim,
                    theme.faint,
                ),
            ];
            paint::paint_into(sheet, width, height, &frame);
            paint::paint_into(sheet, width, height, &layout::shift(out.ops, px, py));
            for mut hit in out.hits {
                hit.x += px;
                hit.y += py;
                hits.push(hit);
            }
            queue.extend(out.overlays);
        }
    }
}

pub fn render(input: &FaceInput, ui: &UiState) -> Scene {
    let theme = Theme::new(&input.app, input.dark)
        .sized(input.rung)
        .rounded(input.round);
    let (width, height) = (theme.width, theme.height);
    let root = input.ui.clone().unwrap_or_else(|| dump::tree(input));
    let mut out = tree::Out::new();
    out.hover = ui.hover.clone();
    out.tint = theme.accent;
    let body = tree::lay(&root, PAD, 0, theme.content(), &theme, ui, &mut out);
    let body = body.clamp(1, theme.body());

    let mut sheet = vec![theme.board; width * height];
    paint::paint_into(&mut sheet, width, height, &layout::title_ops(input, &theme));

    let asking = ui.bar.as_ref();
    let docked = ui.dock.as_ref().or(ui.edit.as_ref());
    let strip =
        docked.map(|e| keys::strip(e.keys.as_deref().unwrap_or("text"), e.shift, e.page, &theme));
    let bar = if strip.is_some() || asking.is_some() {
        Vec::new()
    } else {
        layout::action_bar(input, &theme)
    };
    let asked = asking.map_or(0, |b| EDGE + PAD + (1 + b.hints.len()) * CONTROL + PAD);
    let full = strip.as_ref().map_or(0, |(_, _, h)| *h);
    let risen = full * ui.rise.max(1) as usize / FULL as usize;
    let bar_h = match (strip.is_some(), asking) {
        (true, _) => risen,
        (false, Some(_)) => asked,
        (false, None) => PAD + bar.iter().map(|i| i.height).sum::<usize>(),
    };
    let y0 = HEADER + PAD;
    let y1 = height.saturating_sub(bar_h + PAD);
    let window = y1.saturating_sub(y0);

    let scroll = reveal(&out.hits, ui, body, window);
    let seen = window.min(body.saturating_sub(scroll)).max(1);
    let mut canvas = vec![theme.board; width * seen];
    paint::band(&mut canvas, width, seen, &out.ops, scroll);
    for row in 0..window.min(body.saturating_sub(scroll)) {
        let src = row * width;
        let dst = (y0 + row) * width;
        sheet[dst..dst + width].copy_from_slice(&canvas[src..src + width]);
    }

    let binds: Vec<Act> = out
        .hits
        .iter()
        .filter(|hit| matches!(hit.act, Act::Edit { .. }))
        .map(|hit| hit.act.clone())
        .collect();
    let mut hits: Vec<Hit> = out
        .hits
        .into_iter()
        .filter_map(|hit| clip(hit, scroll, y0, y1))
        .collect();

    if body > window && window > 0 {
        let th = (window * window / body).clamp(CONTROL / 3, window);
        let ty = y0 + (window - th) * scroll / body.saturating_sub(window).max(1);
        paint::paint_into(
            &mut sheet,
            width,
            height,
            &[layout::Op::rect(
                width - 2 * EDGE,
                ty,
                2 * EDGE,
                th,
                theme.muted,
            )],
        );
    }

    match strip {
        Some((ops, caps, _)) => {
            let top = height - risen;
            paint::paint_into(&mut sheet, width, height, &layout::shift(ops, 0, top));
            for mut hit in caps {
                hit.y += top;
                if hit.y < height {
                    hits.push(hit);
                }
            }
        }
        None if asking.is_some() => {
            let ask = asking.expect("a bar");
            let top = height - bar_h;
            let mut ops = vec![layout::Op::rect(0, top, width, EDGE, theme.faint)];
            let mut y = top + EDGE + PAD;
            for (i, hint) in ask.hints.iter().enumerate() {
                let call = Call::new(hint, mrlycore::json!({}));
                ops.push(layout::Op::text(
                    PAD + CONTROL,
                    y + (CONTROL - crate::tokens::LINE) / 2,
                    hint.clone(),
                    1,
                    theme.muted,
                ));
                hits.push(Hit {
                    x: 0,
                    y,
                    w: width,
                    h: CONTROL,
                    act: Act::Fill { text: hint.clone() },
                });
                let _ = (i, call);
                y += CONTROL;
            }
            ops.push(layout::Op::text(
                PAD,
                y + (CONTROL - crate::tokens::LINE) / 2,
                "/".to_string(),
                1,
                theme.pen,
            ));
            let typed = ask.line.clone();
            let tw = crate::face::text::width(&typed, 1);
            ops.push(layout::Op::text(
                PAD + CONTROL,
                y + (CONTROL - crate::tokens::LINE) / 2,
                typed,
                1,
                theme.ink,
            ));
            ops.push(layout::Op::rect(
                PAD + CONTROL + tw + EDGE,
                y + (CONTROL - crate::tokens::LINE) / 2,
                EDGE,
                crate::tokens::LINE,
                theme.pen,
            ));
            paint::paint_into(&mut sheet, width, height, &ops);
        }
        None => {
            let mut bar_ops = vec![layout::Op::rect(
                0,
                height - bar_h,
                width,
                EDGE,
                theme.faint,
            )];
            let mut by = height - bar_h + PAD;
            for (i, item) in bar.into_iter().enumerate() {
                let h = item.height;
                bar_ops.extend(layout::shift(item.ops, 0, by));
                if let Some(verb) = input.actions.get(i) {
                    if i < ACTIONS && verb.args.as_object().is_none_or(|m| m.is_empty()) {
                        hits.push(Hit {
                            x: 0,
                            y: by,
                            w: width,
                            h,
                            act: Act::Tap {
                                call: Call::new(&verb.name, mrlycore::json!({})),
                            },
                        });
                    }
                }
                by += h;
            }
            paint::paint_into(&mut sheet, width, height, &bar_ops);
        }
    }

    overlays(out.overlays, &theme, ui, &mut sheet, &mut hits);

    if let Some(want) = &ui.ring {
        let scrim = hits
            .iter()
            .rposition(|hit| hit.w == width && hit.h == height)
            .map_or(0, |at| at + 1);
        if let Some(hit) = hits[scrim..].iter().find(|hit| hit.act == *want) {
            let rim = vec![
                layout::Op::rect(hit.x, hit.y, hit.w, EDGE, theme.pen),
                layout::Op::rect(
                    hit.x,
                    hit.y + hit.h.saturating_sub(EDGE),
                    hit.w,
                    EDGE,
                    theme.pen,
                ),
                layout::Op::rect(hit.x, hit.y, EDGE, hit.h, theme.pen),
                layout::Op::rect(
                    hit.x + hit.w.saturating_sub(EDGE),
                    hit.y,
                    EDGE,
                    hit.h,
                    theme.pen,
                ),
            ];
            paint::paint_into(&mut sheet, width, height, &rim);
        }
    }

    Scene {
        frame: crate::frame::field(width, height, sheet, theme.board),
        hits,
        binds,
        body,
        window,
        scroll,
    }
}

pub fn fit(bw: usize, bh: usize, fw: usize, fh: usize) -> (usize, usize, usize) {
    let scale = (bw / fw.max(1)).min(bh / fh.max(1)).max(1);
    let ox = bw.saturating_sub(fw * scale) / 2;
    let oy = bh.saturating_sub(fh * scale) / 2;
    (ox, oy, scale)
}

pub fn face(input: &FaceInput) -> crate::frame::Frame {
    render(input, &UiState::default()).frame
}

pub fn face_png(input: &FaceInput) -> Result<Vec<u8>> {
    let frame = face(input);
    let colors = frame.composite().cell.colors.unwrap_or_default();
    mrlycore::png(&colors, frame.width, frame.height, SCALE)
}

pub fn decode(fact: &Json) -> Option<(usize, usize, Vec<[u8; 4]>)> {
    let image = mrlycore::Image::from_json(fact).ok()?;
    if image.width == 0 || image.height == 0 {
        return None;
    }
    Some((image.width, image.height, image.colors()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::colors::ROLLABLE;
    use mrlycore::json;
    use mrlycore::ui::Pick;

    fn bare(app: &str, state: Json) -> FaceInput {
        FaceInput {
            app: app.to_string(),
            title: app.to_string(),
            params: json!({}),
            state,
            actions: Vec::new(),
            beat: None,
            dark: false,
            ui: None,
            rung: crate::tokens::RUNG,
            round: 0,
        }
    }

    fn png_dims(png: &[u8]) -> (u32, u32) {
        let w = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
        let h = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
        (w, h)
    }

    fn fnv(colors: &[[u8; 4]]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for px in colors {
            for byte in px {
                h ^= *byte as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        h
    }

    fn pinned_input() -> FaceInput {
        let fact = crate::frame::field(4, 4, vec![[255, 0, 0, 255]; 16], [255, 0, 0, 255]).fact();
        FaceInput {
            app: "pinned".to_string(),
            title: "pinned \u{1f3b9} face".to_string(),
            params: json!({ "slug": "dummy" }),
            state: json!({
                "frame": fact,
                "score": 12,
                "grid": [[1, 2, 3], [4, 5, 6]],
                "settings": { "pace": 4, "head": { "tile": 1, "paint": 2 } },
                "blob": "data:image/png;base64,AAAA",
                "md": "# Pin\n\nProse with **bold** and a [link](https://mrly.net) that wraps far enough to break lines.\n\n- one\n- two\n\n```\ncargo run\n```",
            }),
            actions: vec![
                FaceVerb {
                    name: "pin.step".to_string(),
                    args: json!({ "n": "int" }),
                },
                FaceVerb {
                    name: "pin.turn".to_string(),
                    args: json!({ "dir": "up|down|left|right" }),
                },
                FaceVerb {
                    name: "pin.reset".to_string(),
                    args: json!({}),
                },
            ],
            beat: Some("pin.step".to_string()),
            dark: true,
            ui: None,
            rung: crate::tokens::RUNG,
            round: 0,
        }
    }

    fn widget_input() -> FaceInput {
        let mut input = bare("widgets", Json::Null);
        input.ui = Some(Node::column(vec![
            Node::group(vec![
                Node::text("controls", mrlycore::ui::Role::Label),
                Node::button("play", Call::new("game.play", json!({}))),
                Node::toggle(
                    "wrap",
                    true,
                    Call::new("game.set", json!({ "key": "wrap" })),
                    "value",
                ),
                Node::choice(
                    "mode",
                    "grid",
                    vec!["grid".to_string(), "list".to_string()],
                    Pick::Segments,
                    Call::new("game.set", json!({ "key": "mode" })),
                    "value",
                ),
                Node::range(
                    "speed",
                    4,
                    1,
                    8,
                    1,
                    Call::new("game.set", json!({ "key": "speed" })),
                    "value",
                ),
                Node::field("", "search", Call::new("game.search", json!({})), "q"),
            ]),
            Node::grid(
                4,
                (0..4)
                    .map(|i| {
                        Node::cell(Some(Call::new("game.pick", json!({ "i": i })))).active(i == 0)
                    })
                    .collect(),
            ),
        ]));
        input
    }

    #[test]
    fn null_state_still_faces() {
        let frame = face(&bare("ghost", Json::Null));
        assert_eq!(frame.width, WIDTH);
        assert_eq!(frame.height, HEIGHT);
        let png = face_png(&bare("ghost", Json::Null)).unwrap();
        assert_eq!(&png[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn hostile_state_never_panics() {
        let mut deep = json!(1);
        for _ in 0..40 {
            deep = json!({ "d": deep });
        }
        let state = json!({
            "deep": deep,
            "frame": { "width": 999999, "height": 999999, "rows": [], "palette": [] },
            "many": (0..500).map(|i| json!({ "i": i })).collect::<Vec<_>>(),
            "emoji": "\u{1f600}\u{1f680}\u{1f9e0}",
            "long": "x".repeat(5000),
            "extremes": [0, -1, i64::MIN, i64::MAX],
        });
        let png = face_png(&bare("hostile", state)).unwrap();
        let (w, h) = png_dims(&png);
        assert_eq!(w as usize, WIDTH * SCALE);
        assert_eq!(h as usize, HEIGHT * SCALE);
    }

    #[test]
    fn decode_roundtrips_a_frame_fact() {
        let frame = crate::frame::field(
            2,
            2,
            vec![
                [255, 0, 0, 255],
                [0, 0, 0, 255],
                [0, 0, 0, 255],
                [255, 0, 0, 255],
            ],
            [0, 0, 0, 255],
        );
        let (w, h, pixels) = decode(&frame.fact()).unwrap();
        assert_eq!((w, h), (2, 2));
        assert_eq!(pixels[0], [255, 0, 0, 255]);
        assert_eq!(pixels[1], [0, 0, 0, 255]);
        assert_eq!(decode(&crate::frame::empty_fact(48, 48)), None);
        assert_eq!(decode(&json!(null)), None);
        let ragged = json!({ "width": 2, "height": 2, "rows": [[0]], "palette": ["#ffffff"] });
        assert_eq!(decode(&ragged), None);
    }

    #[test]
    fn the_canvas_lands_in_the_body() {
        let fact = crate::frame::field(4, 4, vec![[255, 0, 0, 255]; 16], [255, 0, 0, 255]).fact();
        let frame = face(&bare("solid", json!({ "frame": fact })));
        let colors = frame.composite().cell.colors.unwrap();
        assert!(colors.contains(&[255, 0, 0, 255]));
    }

    #[test]
    fn the_accent_is_stable_per_app() {
        let a = Theme::new("snake", false).accent;
        let b = Theme::new("snake", true).accent;
        assert_eq!(a, b);
        assert!(ROLLABLE.iter().any(|c| [c.r, c.g, c.b, c.a] == a));
    }

    #[test]
    fn faces_are_deterministic() {
        let a = face_png(&pinned_input()).unwrap();
        let b = face_png(&pinned_input()).unwrap();
        assert_eq!(a, b);
        let (w, _) = png_dims(&a);
        assert_eq!(w as usize, WIDTH * SCALE);
    }

    #[test]
    fn face_pixels_are_pinned() {
        let frame = face(&pinned_input());
        let colors = frame.composite().cell.colors.unwrap();
        assert_eq!(fnv(&colors), 16299222511607485507);
    }

    #[test]
    fn widgets_emit_hits() {
        let scene = render(&widget_input(), &UiState::default());
        let acts: Vec<&Act> = scene.hits.iter().map(|h| &h.act).collect();
        assert!(acts
            .iter()
            .any(|a| matches!(a, Act::Tap { call } if call.verb == "game.play")));
        assert!(acts.iter().any(
            |a| matches!(a, Act::Tap { call } if call.verb == "game.set" && call.args["value"] == false)
        ));
        assert!(acts.iter().any(
            |a| matches!(a, Act::Tap { call } if call.args["value"].as_str() == Some("list"))
        ));
        assert!(acts
            .iter()
            .any(|a| matches!(a, Act::Slide { min: 1, max: 8, .. })));
        assert!(acts
            .iter()
            .any(|a| matches!(a, Act::Edit { arg, .. } if arg == "q")));
        assert!(acts
            .iter()
            .any(|a| matches!(a, Act::Tap { call } if call.verb == "game.pick")));
        for hit in &scene.hits {
            assert!(hit.x + hit.w <= WIDTH);
            assert!(hit.y + hit.h <= HEIGHT);
        }
    }

    #[test]
    fn a_menu_opens_an_overlay() {
        let mut input = bare("chooser", Json::Null);
        input.ui = Some(Node::choice(
            "note",
            "c",
            vec!["c".to_string(), "d".to_string(), "e".to_string()],
            Pick::Menu,
            Call::new("piano.set", json!({ "key": "note" })),
            "value",
        ));
        let key = "piano.set:value:{\"key\":\"note\"}";
        let closed = render(&input, &UiState::default());
        assert!(closed
            .hits
            .iter()
            .any(|h| matches!(&h.act, Act::Menu { id } if id == key)));
        let open = UiState {
            menu: Some(key.to_string()),
            shade: Some(key.to_string()),
            ..UiState::default()
        };
        let opened = render(&input, &open);
        assert!(opened.hits.iter().any(|h| matches!(&h.act, Act::Shut)));
        assert!(opened.hits.iter().any(
            |h| matches!(&h.act, Act::Tap { call } if call.args["value"].as_str() == Some("e"))
        ));
    }

    #[test]
    fn sibling_fields_get_distinct_ids() {
        let mut input = bare("fractal", Json::Null);
        let set = |key: &str| Call::new("mandelbrot.set", json!({ "key": key }));
        input.ui = Some(Node::column(vec![
            Node::field("#000000", "#rrggbb", set("primary"), "value"),
            Node::field("#1ec9f3", "#rrggbb", set("accent"), "value"),
        ]));
        let scene = render(&input, &UiState::default());
        let ids: Vec<String> = scene
            .binds
            .iter()
            .filter_map(|a| match a {
                Act::Edit { id, .. } => Some(id.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1]);
    }

    #[test]
    fn editing_swaps_the_bar_for_keycaps() {
        let input = widget_input();
        let idle = render(&input, &UiState::default());
        assert!(!idle.hits.iter().any(|h| matches!(h.act, Act::Cap(_))));
        let id = idle
            .binds
            .iter()
            .find_map(|a| match a {
                Act::Edit { id, .. } => Some(id.clone()),
                _ => None,
            })
            .unwrap();
        let editing = UiState {
            edit: Some(Edit::new(id, String::new(), None)),
            ..UiState::default()
        };
        let scene = render(&input, &editing);
        let caps: Vec<&keys::Tap> = scene
            .hits
            .iter()
            .filter_map(|h| match &h.act {
                Act::Cap(tap) => Some(tap),
                _ => None,
            })
            .collect();
        assert_eq!(caps.len(), 41);
        assert!(caps.contains(&&keys::Tap::Char('q')));
        assert!(caps.contains(&&keys::Tap::Back));
        assert!(caps.contains(&&keys::Tap::Enter));
        assert!(scene.window < idle.window);
    }

    #[test]
    fn a_keyed_field_picks_its_board() {
        let mut input = bare("fractal", Json::Null);
        input.ui = Some(
            Node::field(
                "",
                "",
                Call::new("mandelbrot.set", json!({ "key": "primary" })),
                "value",
            )
            .keys("colors"),
        );
        let scene = render(&input, &UiState::default());
        let Some(Act::Edit { id, keys, .. }) = scene.binds.first().cloned() else {
            panic!()
        };
        assert_eq!(keys.as_deref(), Some("colors"));
        let editing = UiState {
            edit: Some(Edit::new(id, String::new(), keys)),
            ..UiState::default()
        };
        let opened = render(&input, &editing);
        let puts = opened
            .hits
            .iter()
            .filter(|h| matches!(&h.act, Act::Cap(keys::Tap::Put(_))))
            .count();
        assert_eq!(puts, 15);
    }

    #[test]
    fn a_symbol_takes_its_size_and_ink() {
        let shot = |node: Node| {
            let mut input = bare("iconed", Json::Null);
            input.ui = Some(node);
            let scene = render(&input, &UiState::default());
            (
                scene.body,
                fnv(&scene.frame.composite().cell.colors.unwrap_or_default()),
            )
        };
        let (plain_h, plain) = shot(Node::symbol("search"));
        let (big_h, big) = shot(Node::symbol("search").sized(2 * CONTROL));
        let (inked_h, inked) = shot(Node::symbol("search").inked("accent"));
        assert!(big_h > plain_h, "a bigger symbol takes more room");
        assert_eq!(inked_h, plain_h, "ink is not geometry");
        assert_ne!(plain, big);
        assert_ne!(plain, inked);
        let (_, again) = shot(Node::symbol("search").inked("ink"));
        assert_eq!(plain, again, "the default ink is the theme ink");
    }

    #[test]
    fn shift_swaps_the_board_under_the_same_field() {
        let input = widget_input();
        let id = render(&input, &UiState::default())
            .binds
            .iter()
            .find_map(|a| match a {
                Act::Edit { id, .. } => Some(id.clone()),
                _ => None,
            })
            .unwrap();
        let low = UiState {
            edit: Some(Edit::new(id.clone(), String::new(), None)),
            ..UiState::default()
        };
        let up = UiState {
            edit: Some(Edit {
                shift: true,
                ..Edit::new(id, String::new(), None)
            }),
            ..UiState::default()
        };
        let taps = |ui| {
            render(&input, ui)
                .hits
                .iter()
                .filter_map(|h| match &h.act {
                    Act::Cap(tap) => Some(tap.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        let (down, over) = (taps(&low), taps(&up));
        assert!(down.contains(&keys::Tap::Char('q')));
        assert!(!down.contains(&keys::Tap::Char('Q')));
        assert!(over.contains(&keys::Tap::Char('Q')));
        assert!(over.contains(&keys::Tap::Char('!')));
        assert_eq!(down.len(), over.len());
        assert!(down.contains(&keys::Tap::Shift));
        let pixels = |ui| {
            fnv(&render(&input, ui)
                .frame
                .composite()
                .cell
                .colors
                .unwrap_or_default())
        };
        assert_ne!(pixels(&low), pixels(&up));
    }

    #[test]
    fn focus_pulls_a_buried_field_into_view() {
        let mut input = bare("lister", Json::Null);
        let mut rows: Vec<Node> = (0..40)
            .map(|i| {
                Node::button(
                    &format!("row {i}"),
                    Call::new("list.pick", json!({ "i": i })),
                )
            })
            .collect();
        rows.push(Node::field(
            "",
            "last",
            Call::new("list.find", json!({})),
            "q",
        ));
        input.ui = Some(Node::column(rows));
        let idle = render(&input, &UiState::default());
        assert_eq!(idle.scroll, 0);
        let id = idle
            .binds
            .iter()
            .find_map(|a| match a {
                Act::Edit { id, .. } => Some(id.clone()),
                _ => None,
            })
            .unwrap();
        let focused = render(
            &input,
            &UiState {
                edit: Some(Edit::new(id.clone(), String::new(), None)),
                ..UiState::default()
            },
        );
        assert!(focused.scroll > 0);
        assert!(focused.scroll <= focused.body - focused.window);
        let seen = focused
            .hits
            .iter()
            .find(|h| matches!(&h.act, Act::Edit { id: at, .. } if *at == id))
            .expect("the focused field is on screen");
        assert!(seen.y + seen.h <= HEIGHT);
        assert_eq!(seen.h, CONTROL);
    }

    #[test]
    fn a_visible_field_never_moves_the_scroll() {
        let input = widget_input();
        let idle = render(&input, &UiState::default());
        let id = idle
            .binds
            .iter()
            .find_map(|a| match a {
                Act::Edit { id, .. } => Some(id.clone()),
                _ => None,
            })
            .unwrap();
        let focused = render(
            &input,
            &UiState {
                edit: Some(Edit::new(id, String::new(), None)),
                ..UiState::default()
            },
        );
        assert_eq!(focused.scroll, 0);
    }

    #[test]
    fn scroll_reaches_the_tail() {
        let mut input = bare("lister", Json::Null);
        input.ui = Some(Node::column(
            (0..40)
                .map(|i| {
                    Node::button(
                        &format!("row {i}"),
                        Call::new("list.pick", json!({ "i": i })),
                    )
                })
                .collect(),
        ));
        let flat = render(&input, &UiState::default());
        assert!(flat.body > flat.window);
        let deep = UiState {
            scroll: 100000,
            ..UiState::default()
        };
        let scrolled = render(&input, &deep);
        assert_eq!(scrolled.frame.height, HEIGHT);
    }

    #[test]
    fn a_focused_field_follows_the_tail_of_a_long_buffer() {
        let mut input = bare("typer", Json::Null);
        input.ui = Some(Node::field(
            "",
            "type",
            Call::new("typer.find", json!({})),
            "q",
        ));
        let id = render(&input, &UiState::default())
            .binds
            .iter()
            .find_map(|a| match a {
                Act::Edit { id, .. } => Some(id.clone()),
                _ => None,
            })
            .unwrap();
        let typed =
            |n: usize| -> String { (0..n).map(|i| (b'a' + (i % 26) as u8) as char).collect() };
        let shot = |n: usize| {
            fnv(&render(
                &input,
                &UiState {
                    edit: Some(Edit::new(id.clone(), typed(n), None)),
                    ..UiState::default()
                },
            )
            .frame
            .composite()
            .cell
            .colors
            .unwrap_or_default())
        };
        let long = shot(120);
        assert_ne!(long, shot(121), "the field froze past its width");
        assert_ne!(shot(60), long, "the field never moved");
    }

    #[test]
    fn the_body_only_rasters_the_band_it_shows() {
        let mut input = bare("lister", Json::Null);
        input.ui = Some(Node::column(
            (0..60)
                .map(|i| {
                    Node::button(
                        &format!("row {i}"),
                        Call::new("list.pick", json!({ "i": i })),
                    )
                })
                .collect(),
        ));
        let deep = render(
            &input,
            &UiState {
                scroll: 400,
                ..UiState::default()
            },
        );
        assert!(deep.scroll > 0, "the list never scrolled");
        assert_eq!(deep.frame.height, HEIGHT);
        let seen = deep.hits.iter().filter(|h| h.y < HEIGHT).count();
        assert!(seen > 0, "a scrolled body showed nothing");
    }

    #[test]
    fn every_rung_renders_a_whole_sheet() {
        for at in 0..crate::tokens::RUNGS.len() {
            let (want_w, want_h) = crate::tokens::rung(at);
            let mut input = widget_input();
            input.rung = at;
            let scene = render(&input, &UiState::default());
            assert_eq!(scene.frame.width, want_w, "rung {at} width");
            assert_eq!(scene.frame.height, want_h, "rung {at} height");
            assert!(scene.window > 0, "rung {at} has no body");
            for hit in &scene.hits {
                assert!(hit.x + hit.w <= want_w, "rung {at} hit ran off the side");
                assert!(hit.y + hit.h <= want_h, "rung {at} hit ran off the bottom");
            }
        }
    }

    #[test]
    fn the_ratio_holds_at_every_rung() {
        let root2 = 2.0f64.sqrt();
        let mut last = 0;
        for (w, h) in crate::tokens::RUNGS {
            let off = ((h as f64 / w as f64) - root2).abs() / root2;
            assert!(off < 0.003, "{w}x{h} is {:.2}% off root two", off * 100.0);
            assert!(w > last, "the rungs must climb");
            last = w;
        }
        assert_eq!(crate::tokens::rung(crate::tokens::RUNG), (WIDTH, HEIGHT));
        assert_eq!(crate::tokens::fits(WIDTH, HEIGHT), crate::tokens::RUNG);
        assert_eq!(
            crate::tokens::fits(0, 0),
            crate::tokens::FLOOR,
            "a tiny window drops the blit scale, never the sheet"
        );
        assert_eq!(
            crate::tokens::fits(9000, 9000),
            crate::tokens::RUNGS.len() - 1
        );
    }

    #[test]
    fn every_card_wears_its_own_colour() {
        let mut input = bare("carded", Json::Null);
        input.ui = Some(Node::column(
            (0..6)
                .map(|i| {
                    Node::group(vec![Node::text(
                        &format!("card {i}"),
                        mrlycore::ui::Role::Label,
                    )])
                })
                .collect(),
        ));
        let theme = Theme::new("carded", false);
        let seen: Vec<[u8; 4]> = (0..6).map(|i| theme.card(i)).collect();
        for (i, colour) in seen.iter().enumerate() {
            assert!(
                !seen[..i].contains(colour),
                "card {i} repeated a colour inside one screen"
            );
        }
        let scene = render(&input, &UiState::default());
        let colors = scene.frame.composite().cell.colors.unwrap_or_default();
        for (i, colour) in seen.iter().enumerate() {
            assert!(colors.contains(colour), "card {i} never reached the sheet");
        }
    }

    #[test]
    fn the_radius_key_rounds_the_corners() {
        let shot = |round: usize| {
            let mut input = widget_input();
            input.round = round;
            fnv(&render(&input, &UiState::default())
                .frame
                .composite()
                .cell
                .colors
                .unwrap_or_default())
        };
        let square = shot(0);
        let mut last = square;
        for key in 1..=4 {
            let now = shot(key);
            assert_ne!(now, last, "radius {key} drew the same box as {}", key - 1);
            last = now;
        }
        assert_eq!(square, shot(0), "the same key must draw the same box");
    }

    #[test]
    fn a_box_keeps_its_rim_inside_its_bounds() {
        let mut input = widget_input();
        input.round = 4;
        let scene = render(&input, &UiState::default());
        for hit in &scene.hits {
            assert!(hit.x + hit.w <= WIDTH, "a rounded box ran off the side");
            assert!(hit.y + hit.h <= HEIGHT, "a rounded box ran off the bottom");
        }
    }
}
