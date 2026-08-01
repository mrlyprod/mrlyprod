use mrlycore::colors::ROLLABLE;

// SHEET
pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 452;
pub const SCALE: usize = 3;

// SPACE
pub const PAD: usize = 6;
pub const GAP: usize = 4;
pub const TIGHT: usize = 2;
pub const INDENT: usize = 10;
pub const RADIUS: usize = 0;
pub const CONTENT: usize = WIDTH - 2 * PAD;

// TYPE
pub const LINE: usize = 7;
pub const LEAD: usize = 4;
pub const TEXT: usize = 1;
pub const TITLE: usize = 2;
pub const ROW: usize = LINE * TEXT + LEAD;

// SIZE
pub const CONTROL: usize = 18;
pub const TOGGLE: usize = 14;
pub const LABEL: usize = 12;
pub const HEADER: usize = 20;
pub const SYMBOL: usize = 16;
pub const CANVAS: usize = 192;
pub const PANEL: usize = 240;

// MOTION
pub const PACE: u64 = 0;

// CAP
pub const ACTIONS: usize = 8;
pub const LIST: usize = 12;
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
