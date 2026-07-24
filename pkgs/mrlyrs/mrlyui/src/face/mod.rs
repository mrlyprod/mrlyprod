use mrlycore::colors::ROLLABLE;
use mrlycore::errors::Result;
use serde_json::Value as Json;

mod layout;
mod md;
mod paint;
mod text;

pub const WIDTH: usize = 320;
pub const SCALE: usize = 3;
pub const MIN_HEIGHT: usize = 160;
pub const MAX_HEIGHT: usize = 512;

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

pub fn face(input: &FaceInput) -> crate::frame::Frame {
    let theme = Theme::new(&input.app, input.dark);
    let (height, ops) = layout::layout(input, &theme);
    let colors = paint::paint(&ops, WIDTH, height, theme.board);
    crate::frame::field(WIDTH, height, colors, theme.board)
}

pub fn face_png(input: &FaceInput) -> Result<Vec<u8>> {
    let frame = face(input);
    let colors = frame.composite().cell.colors.unwrap_or_default();
    mrlycore::png(&colors, frame.width, frame.height, SCALE)
}

pub fn decode(fact: &Json) -> Option<(usize, usize, Vec<[u8; 4]>)> {
    let width = fact["width"].as_u64()? as usize;
    let height = fact["height"].as_u64()? as usize;
    if width == 0 || height == 0 {
        return None;
    }
    let rows = fact["rows"].as_array()?;
    if rows.len() != height {
        return None;
    }
    let palette: Vec<[u8; 4]> = fact["palette"]
        .as_array()?
        .iter()
        .map(|v| {
            v.as_str()
                .and_then(|hex| mrlycore::Color::from_hex(hex).ok())
                .map(|c| [c.r, c.g, c.b, c.a])
        })
        .collect::<Option<Vec<_>>>()?;
    let mut pixels = Vec::with_capacity(width * height);
    for row in rows {
        let row = row.as_array()?;
        if row.len() != width {
            return None;
        }
        for id in row {
            pixels.push(*palette.get(id.as_u64()? as usize)?);
        }
    }
    Some((width, height, pixels))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn bare(app: &str, state: Json) -> FaceInput {
        FaceInput {
            app: app.to_string(),
            title: app.to_string(),
            params: json!({}),
            state,
            actions: Vec::new(),
            beat: None,
            dark: false,
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
        }
    }

    #[test]
    fn null_state_still_faces() {
        let frame = face(&bare("ghost", Json::Null));
        assert_eq!(frame.width, WIDTH);
        assert!(frame.height >= MIN_HEIGHT && frame.height <= MAX_HEIGHT);
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
            "floats": [0.005, -1.5, 1e30],
        });
        let png = face_png(&bare("hostile", state)).unwrap();
        let (w, h) = png_dims(&png);
        assert_eq!(w as usize, WIDTH * SCALE);
        assert!(h as usize <= MAX_HEIGHT * SCALE);
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
        assert_eq!(fnv(&colors), 3902902881908127161);
    }
}
