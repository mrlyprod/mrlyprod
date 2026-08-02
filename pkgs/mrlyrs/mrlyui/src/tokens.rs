use mrlycore::colors::ROLLABLE;

// SHEET
pub const RUNGS: [(usize, usize); 7] = [
    (113, 160),
    (160, 226),
    (226, 320),
    (320, 452),
    (452, 640),
    (640, 904),
    (904, 1280),
];
pub const RUNG: usize = 3;
pub const FLOOR: usize = 1;
pub const WIDTH: usize = RUNGS[RUNG].0;
pub const HEIGHT: usize = RUNGS[RUNG].1;
pub const SCALE: usize = 3;

pub fn rung(at: usize) -> (usize, usize) {
    RUNGS[at.min(RUNGS.len() - 1)]
}

pub fn fits(bw: usize, bh: usize) -> usize {
    let mut at = FLOOR;
    for (i, (w, h)) in RUNGS.iter().enumerate() {
        if *w <= bw && *h <= bh {
            at = i.max(FLOOR);
        }
    }
    at
}

// TYPE
pub const LINE: usize = 7;
pub const LEAD: usize = 4;
pub const TEXT: usize = 1;
pub const TITLE: usize = 2;
pub const ROW: usize = LINE * TEXT + LEAD;

// SPACE
pub const UNIT: usize = 3;
pub const EDGE: usize = 1;
pub const RIM: usize = 2 * EDGE;
pub const GAP: usize = UNIT;
pub const PAD: usize = UNIT;
pub const TIGHT: usize = UNIT;
pub const SPLIT: usize = 2 * UNIT;
pub const INDENT: usize = 3 * UNIT;
pub const INSET: usize = (CONTROL - LINE) / 2;

// SIZE
pub const CONTROL: usize = 24;
pub const HEADER: usize = CONTROL;
pub const RULE: usize = 2 * UNIT;
pub const SYMBOL: usize = 16;
pub const ICON: usize = 2 * CONTROL;
pub const CANVAS: usize = 192;

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

// COLOR
pub const MUTED: f64 = 0.55;
pub const FAINT: f64 = 0.12;
pub const READ: f64 = 4.5;
pub const STEP: f64 = 0.05;

pub struct Theme {
    pub board: [u8; 4],
    pub ink: [u8; 4],
    pub muted: [u8; 4],
    pub faint: [u8; 4],
    pub accent: [u8; 4],
    pub pen: [u8; 4],
    pub width: usize,
    pub height: usize,
    pub round: usize,
    pub seed: u64,
}

impl Theme {
    pub fn new(app: &str, dark: bool) -> Theme {
        let board = crate::frame::board(dark);
        let ink = crate::frame::ink(dark);
        let c = ROLLABLE[(hash(app) % ROLLABLE.len() as u64) as usize];
        let accent = [c.r, c.g, c.b, c.a];
        let (width, height) = rung(RUNG);
        let seed = hash(app);
        Theme {
            board,
            ink,
            muted: crate::frame::mix(board, ink, MUTED),
            faint: crate::frame::mix(board, ink, FAINT),
            accent,
            pen: legible(accent, board, ink),
            width,
            height,
            round: 0,
            seed,
        }
    }
    pub fn rounded(mut self, key: usize) -> Theme {
        self.round = key.min(4) * UNIT;
        self
    }
    pub fn card(&self, n: usize) -> [u8; 4] {
        let at = (self.seed.wrapping_add(n as u64 * 7)) % ROLLABLE.len() as u64;
        let c = ROLLABLE[at as usize];
        [c.r, c.g, c.b, c.a]
    }
    pub fn sized(mut self, at: usize) -> Theme {
        let (width, height) = rung(at);
        self.width = width;
        self.height = height;
        self
    }
    pub fn content(&self) -> usize {
        self.width.saturating_sub(2 * PAD)
    }
    pub fn body(&self) -> usize {
        self.height * 16
    }
    pub fn panel(&self) -> usize {
        (self.width * 3 / 4).min(self.width.saturating_sub(4 * PAD))
    }
    pub fn canvas(&self) -> usize {
        self.height * CANVAS / HEIGHT
    }
}

fn legible(accent: [u8; 4], board: [u8; 4], ink: [u8; 4]) -> [u8; 4] {
    let mut pen = accent;
    let mut at = 0.0;
    while ratio(pen, board) < READ && at < 1.0 {
        at += STEP;
        pen = crate::frame::mix(accent, ink, at);
    }
    pen
}

fn luma(c: [u8; 4]) -> f64 {
    let lift = |v: u8| {
        let v = v as f64 / 255.0;
        match v <= 0.04045 {
            true => v / 12.92,
            false => ((v + 0.055) / 1.055).powf(2.4),
        }
    };
    0.2126 * lift(c[0]) + 0.7152 * lift(c[1]) + 0.0722 * lift(c[2])
}

pub fn ratio(a: [u8; 4], b: [u8; 4]) -> f64 {
    let (x, y) = (luma(a), luma(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

fn hash(text: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for byte in text.as_bytes() {
        h ^= *byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub fn accent_of(app: &str) -> [u8; 4] {
    let c = ROLLABLE[(hash(app) % ROLLABLE.len() as u64) as usize];
    [c.r, c.g, c.b, c.a]
}

pub fn contrast(fill: [u8; 4]) -> [u8; 4] {
    let black = [0, 0, 0, 255];
    let white = [255, 255, 255, 255];
    if ratio(fill, black) >= ratio(fill, white) {
        black
    } else {
        white
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pen_always_reads_on_its_board() {
        for dark in [false, true] {
            for app in [
                "snake",
                "twenty48",
                "menu",
                "settings",
                "lasers",
                "mandelbrot",
                "chess",
            ] {
                let t = Theme::new(app, dark);
                assert!(
                    ratio(t.pen, t.board) >= READ,
                    "{app} dark={dark} pen {} reads at {:.2}",
                    crate::frame::hex(t.pen),
                    ratio(t.pen, t.board)
                );
            }
        }
    }

    #[test]
    fn contrast_picks_whichever_ink_reads_better() {
        let black = [0, 0, 0, 255];
        let white = [255, 255, 255, 255];
        for c in mrlycore::colors::ROLLABLE {
            let fill = [c.r, c.g, c.b, c.a];
            let ink = contrast(fill);
            assert!(
                ratio(fill, ink) >= ratio(fill, if ink == black { white } else { black }),
                "{} took the worse ink",
                crate::frame::hex(fill)
            );
        }
        assert_eq!(contrast(white), black);
        assert_eq!(contrast(black), white);
    }
}
