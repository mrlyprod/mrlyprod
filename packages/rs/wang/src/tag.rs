pub const N: u8 = 0b1000;
pub const E: u8 = 0b0100;
pub const S: u8 = 0b0010;
pub const W: u8 = 0b0001;
pub const VOID: u8 = 0b0000;
pub const FILL: u8 = 0b1111;
pub const HALVES: [u8; 4] = [N | E, E | S, S | W, W | N];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Turn(usize),
    FlipX,
    FlipY,
    FlipMain,
    FlipAnti,
}

pub const GROUP: [Move; 8] = [
    Move::Turn(0),
    Move::Turn(1),
    Move::Turn(2),
    Move::Turn(3),
    Move::FlipX,
    Move::FlipY,
    Move::FlipMain,
    Move::FlipAnti,
];

impl Move {
    pub fn sides(self, [n, e, s, w]: [u8; 4]) -> [u8; 4] {
        match self {
            Move::Turn(k) => {
                let mut out = [n, e, s, w];
                out.rotate_left(k % 4);
                out
            }
            Move::FlipX => [n, w, s, e],
            Move::FlipY => [s, e, n, w],
            Move::FlipMain => [w, s, e, n],
            Move::FlipAnti => [e, n, w, s],
        }
    }
    pub fn tag(self, tag: u8) -> u8 {
        of(self.sides(sides(tag)))
    }
}

pub fn of(sides: [u8; 4]) -> u8 {
    sides
        .iter()
        .fold(0, |tag, &value| tag << 1 | u8::from(value != 0))
}

pub fn sides(tag: u8) -> [u8; 4] {
    [tag >> 3 & 1, tag >> 2 & 1, tag >> 1 & 1, tag & 1]
}

pub fn class(tag: u8) -> &'static str {
    match (tag & FILL).count_ones() {
        0 => "void",
        1 => "wedge",
        2 if HALVES.contains(&(tag & FILL)) => "half",
        2 => "hourglass",
        3 => "notch",
        _ => "fill",
    }
}

pub fn orbit(tag: u8) -> Vec<u8> {
    let mut out: Vec<u8> = GROUP.iter().map(|op| op.tag(tag)).collect();
    out.sort_unstable();
    out.dedup();
    out
}
