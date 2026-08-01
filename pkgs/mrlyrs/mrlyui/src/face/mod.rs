use mrlycore::colors::ROLLABLE;
use mrlycore::errors::Result;
use mrlycore::ui::{Call, Node};
use mrlycore::Json;

mod dump;
mod layout;
mod md;
mod paint;
mod text;
mod tree;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 452;
pub const SCALE: usize = 3;
const BODY_CAP: usize = HEIGHT * 16;
const PANEL_W: usize = 240;

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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edit {
    pub id: String,
    pub buffer: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UiState {
    pub scroll: usize,
    pub edit: Option<Edit>,
    pub menu: Option<String>,
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
    },
    Menu {
        id: String,
    },
    Shut,
    Mute,
}

pub struct Scene {
    pub frame: crate::frame::Frame,
    pub hits: Vec<Hit>,
    pub body: usize,
    pub window: usize,
}

pub(crate) struct Theme {
    pub board: [u8; 4],
    pub ink: [u8; 4],
    pub muted: [u8; 4],
    pub faint: [u8; 4],
    pub accent: [u8; 4],
}

impl Theme {
    pub(crate) fn new(app: &str, dark: bool) -> Theme {
        let board = crate::frame::board(dark);
        let ink = crate::frame::ink(dark);
        let c = ROLLABLE[(hash(app) % ROLLABLE.len() as u64) as usize];
        Theme {
            board,
            ink,
            muted: crate::frame::mix(board, ink, 0.55),
            faint: crate::frame::mix(board, ink, 0.12),
            accent: [c.r, c.g, c.b, c.a],
        }
    }
}

fn hash(text: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for byte in text.as_bytes() {
        h ^= *byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
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
    let mut queue = queue;
    while !queue.is_empty() {
        for (node, scrim) in std::mem::take(&mut queue) {
            let veil = [theme.ink[0], theme.ink[1], theme.ink[2], 70];
            paint::paint_into(
                sheet,
                WIDTH,
                HEIGHT,
                &[layout::Op::Rect {
                    x: 0,
                    y: 0,
                    w: WIDTH,
                    h: HEIGHT,
                    color: veil,
                }],
            );
            hits.push(Hit {
                x: 0,
                y: 0,
                w: WIDTH,
                h: HEIGHT,
                act: scrim,
            });
            let mut out = tree::Out::new();
            let ph = tree::lay(&node, 0, 0, PANEL_W, theme, ui, &mut out);
            let px = (WIDTH - PANEL_W) / 2;
            let py = (HEIGHT.saturating_sub(ph) / 2).max(layout::TITLE_H + 4);
            let panel = vec![
                layout::Op::Rect {
                    x: px - 8,
                    y: py.saturating_sub(8),
                    w: PANEL_W + 16,
                    h: ph + 16,
                    color: theme.board,
                },
                layout::Op::Rect {
                    x: px - 8,
                    y: py.saturating_sub(8),
                    w: PANEL_W + 16,
                    h: 1,
                    color: theme.faint,
                },
                layout::Op::Rect {
                    x: px - 8,
                    y: py + ph + 7,
                    w: PANEL_W + 16,
                    h: 1,
                    color: theme.faint,
                },
                layout::Op::Rect {
                    x: px - 8,
                    y: py.saturating_sub(8),
                    w: 1,
                    h: ph + 16,
                    color: theme.faint,
                },
                layout::Op::Rect {
                    x: px + PANEL_W + 7,
                    y: py.saturating_sub(8),
                    w: 1,
                    h: ph + 16,
                    color: theme.faint,
                },
            ];
            paint::paint_into(sheet, WIDTH, HEIGHT, &panel);
            paint::paint_into(sheet, WIDTH, HEIGHT, &layout::shift(out.ops, px, py));
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
    let theme = Theme::new(&input.app, input.dark);
    let root = input.ui.clone().unwrap_or_else(|| dump::tree(input));
    let mut out = tree::Out::new();
    let body = tree::lay(&root, layout::PAD, 0, layout::FIELD, &theme, ui, &mut out);
    let body = body.clamp(1, BODY_CAP);

    let mut sheet = vec![theme.board; WIDTH * HEIGHT];
    paint::paint_into(&mut sheet, WIDTH, HEIGHT, &layout::title_ops(input, &theme));

    let bar = layout::action_bar(input, &theme);
    let bar_h = 6 + bar.iter().map(|i| i.height).sum::<usize>();
    let y0 = layout::TITLE_H + layout::PAD;
    let y1 = HEIGHT.saturating_sub(bar_h + 2);
    let window = y1.saturating_sub(y0);

    let scroll = ui.scroll.min(body.saturating_sub(window));
    let mut canvas = vec![theme.board; WIDTH * body];
    paint::paint_into(&mut canvas, WIDTH, body, &out.ops);
    for row in 0..window.min(body.saturating_sub(scroll)) {
        let src = (scroll + row) * WIDTH;
        let dst = (y0 + row) * WIDTH;
        sheet[dst..dst + WIDTH].copy_from_slice(&canvas[src..src + WIDTH]);
    }

    let mut hits: Vec<Hit> = out
        .hits
        .into_iter()
        .filter_map(|hit| clip(hit, scroll, y0, y1))
        .collect();

    if body > window && window > 0 {
        let th = (window * window / body).clamp(8, window);
        let ty = y0 + (window - th) * scroll / body.saturating_sub(window).max(1);
        paint::paint_into(
            &mut sheet,
            WIDTH,
            HEIGHT,
            &[layout::Op::Rect {
                x: WIDTH - 3,
                y: ty,
                w: 2,
                h: th,
                color: theme.muted,
            }],
        );
    }

    let mut bar_ops = vec![layout::Op::Rect {
        x: 0,
        y: HEIGHT - bar_h,
        w: WIDTH,
        h: 1,
        color: theme.faint,
    }];
    let mut by = HEIGHT - bar_h + 6;
    for (i, item) in bar.into_iter().enumerate() {
        let h = item.height;
        bar_ops.extend(layout::shift(item.ops, 0, by));
        if let Some(verb) = input.actions.get(i) {
            if i < layout::ACTION_CAP && verb.args.as_object().is_none_or(|m| m.is_empty()) {
                hits.push(Hit {
                    x: 0,
                    y: by,
                    w: WIDTH,
                    h,
                    act: Act::Tap {
                        call: Call::new(&verb.name, mrlycore::json!({})),
                    },
                });
            }
        }
        by += h;
    }
    paint::paint_into(&mut sheet, WIDTH, HEIGHT, &bar_ops);

    overlays(out.overlays, &theme, ui, &mut sheet, &mut hits);

    Scene {
        frame: crate::frame::field(WIDTH, HEIGHT, sheet, theme.board),
        hits,
        body,
        window,
    }
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
        assert_eq!(fnv(&colors), 9385373485591055497);
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
        let closed = render(&input, &UiState::default());
        assert!(closed
            .hits
            .iter()
            .any(|h| matches!(&h.act, Act::Menu { id } if id == "piano.set:value")));
        let open = UiState {
            menu: Some("piano.set:value".to_string()),
            ..UiState::default()
        };
        let opened = render(&input, &open);
        assert!(opened.hits.iter().any(|h| matches!(&h.act, Act::Shut)));
        assert!(opened.hits.iter().any(
            |h| matches!(&h.act, Act::Tap { call } if call.args["value"].as_str() == Some("e"))
        ));
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
}
