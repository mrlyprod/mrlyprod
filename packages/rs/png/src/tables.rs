pub(crate) const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];

pub(crate) const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];

pub(crate) const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];

pub(crate) const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

pub(crate) const ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

pub(crate) const LEN_SLOT: [u8; 259] = len_slots();

const DIST_SLOT: [u8; 512] = dist_slots();

const fn len_slots() -> [u8; 259] {
    let mut out = [0u8; 259];
    let mut slot = 0;
    while slot < 29 {
        let mut len = LEN_BASE[slot] as usize;
        let end = if slot == 28 {
            259
        } else {
            LEN_BASE[slot + 1] as usize
        };
        while len < end {
            out[len] = slot as u8;
            len += 1;
        }
        slot += 1;
    }
    out
}

const fn fold(dist: usize) -> usize {
    if dist <= 256 {
        dist - 1
    } else {
        256 + ((dist - 1) >> 7)
    }
}

const fn dist_slots() -> [u8; 512] {
    let mut out = [0u8; 512];
    let mut slot = 0;
    while slot < 30 {
        let mut dist = DIST_BASE[slot] as usize;
        let end = dist + (1 << DIST_EXTRA[slot]);
        while dist < end {
            out[fold(dist)] = slot as u8;
            dist += 1;
        }
        slot += 1;
    }
    out
}

pub(crate) fn dist_slot(dist: usize) -> usize {
    DIST_SLOT[fold(dist)] as usize
}

pub(crate) fn fixed() -> ([u8; 288], [u8; 32]) {
    let mut lit = [8u8; 288];
    lit[144..256].fill(9);
    lit[256..280].fill(7);
    (lit, [5; 32])
}
