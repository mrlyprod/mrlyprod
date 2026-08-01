use mrlycore::colors::ROLLABLE;

// SHEET
pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 452;
pub const SCALE: usize = 3;

// TYPE
pub const LINE: usize = 7;
pub const LEAD: usize = 4;
pub const TEXT: usize = 1;
pub const TITLE: usize = 2;
pub const ROW: usize = LINE * TEXT + LEAD;

// SPACE
pub const UNIT: usize = 3;
pub const EDGE: usize = 1;
pub const GAP: usize = UNIT;
pub const PAD: usize = UNIT;
pub const TIGHT: usize = UNIT;
pub const SPLIT: usize = 2 * UNIT;
pub const INDENT: usize = 3 * UNIT;
pub const RADIUS: usize = 0;
pub const INSET: usize = (CONTROL - LINE) / 2;
pub const CONTENT: usize = WIDTH - 2 * PAD;

// SIZE
pub const CONTROL: usize = 24;
pub const HEADER: usize = CONTROL;
pub const RULE: usize = 2 * UNIT;
pub const SYMBOL: usize = 16;
pub const CANVAS: usize = 192;
pub const PANEL: usize = 240;

// PARTS
pub const MARK: usize = CONTROL / 3;
pub const SLOT: usize = CONTROL / 2;
pub const CHEV: usize = CONTROL / 2;
pub const SWITCH_W: usize = CONTROL;
pub const SWITCH_H: usize = CONTROL / 2;
pub const KNOB: usize = SWITCH_H - 2 * EDGE;
pub const RAIL: usize = UNIT;
pub const THUMB: usize = UNIT;
pub const GRIP: usize = CONTROL / 2;
pub const CHROME: usize = 4 * UNIT;
pub const SLACK: usize = 4 * UNIT;
pub const STUB: usize = 2 * CONTROL / 3;
pub const TILE: usize = CONTROL;

// GRID
pub const LAUNCH: usize = 3;
pub const DPAD: usize = 4;

// MOTION
pub const PACE: i64 = 150;
pub const FULL: u8 = 255;

pub fn eased(start: i64, now: i64, pace: i64, out: bool) -> u8 {
    let span = pace.max(0);
    let gone = (now - start).clamp(0, span.max(1));
    let at = match span {
        0 => 1.0,
        _ => gone as f64 / span as f64,
    };
    let curve = match out {
        true => 1.0 - (1.0 - at).powi(3),
        false => at * at * at,
    };
    (curve * FULL as f64).round().clamp(0.0, FULL as f64) as u8
}

// CAP
pub const ACTIONS: usize = 8;
pub const LIST: usize = 12;
pub const BEAT: usize = 110;
pub const VERB: usize = 160;
pub const BODY: usize = HEIGHT * 16;

// COLOR
pub const MUTED: f64 = 0.55;
pub const FAINT: f64 = 0.12;
pub const LUMA: f64 = 140.0;

pub struct Theme {
    pub board: [u8; 4],
    pub ink: [u8; 4],
    pub muted: [u8; 4],
    pub faint: [u8; 4],
    pub accent: [u8; 4],
}

impl Theme {
    pub fn new(app: &str, dark: bool) -> Theme {
        let board = crate::frame::board(dark);
        let ink = crate::frame::ink(dark);
        let c = ROLLABLE[(hash(app) % ROLLABLE.len() as u64) as usize];
        Theme {
            board,
            ink,
            muted: crate::frame::mix(board, ink, MUTED),
            faint: crate::frame::mix(board, ink, FAINT),
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

pub fn contrast(fill: [u8; 4]) -> [u8; 4] {
    let luma = 0.299 * fill[0] as f64 + 0.587 * fill[1] as f64 + 0.114 * fill[2] as f64;
    if luma > LUMA {
        [0, 0, 0, 255]
    } else {
        [255, 255, 255, 255]
    }
}
