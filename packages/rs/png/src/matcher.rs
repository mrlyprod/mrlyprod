pub(crate) const WHOLE: u32 = 1 << 31;

const WINDOW: usize = 32768;
const SPAN: usize = 1 << 21;
const NEAR: usize = 8;
const ROOM: usize = 1 << 22;

pub(crate) struct Level {
    chain: usize,
    good: usize,
    lazy: usize,
    pub(crate) split: bool,
    pub(crate) passes: usize,
}

pub(crate) fn level(level: u8) -> Level {
    let (chain, good, lazy) = match level {
        0 | 1 => (4, 4, 0),
        2 => (16, 4, 0),
        3 => (64, 4, 0),
        4 => (64, 4, 4),
        5 => (128, 8, 16),
        6 => (256, 8, 32),
        7 => (512, 16, 64),
        8 => (1024, 32, 258),
        _ => (4096, 32, 0),
    };
    Level {
        chain,
        good,
        lazy,
        split: level >= 7,
        passes: if level >= 9 { 8 } else { 0 },
    }
}

pub(crate) struct Matcher<'a> {
    data: &'a [u8],
    pub(crate) at: usize,
    short: Vec<u32>,
    head: Vec<u32>,
    prev: Vec<u16>,
    run: usize,
    level: &'a Level,
}

#[inline]
fn same(data: &[u8], from: usize, at: usize, most: usize) -> usize {
    let (a, b) = (&data[from..from + most], &data[at..at + most]);
    let mut len = 0;
    for (x, y) in a.chunks_exact(8).zip(b.chunks_exact(8)) {
        let x = u64::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]);
        let y = u64::from_le_bytes([y[0], y[1], y[2], y[3], y[4], y[5], y[6], y[7]]);
        if x != y {
            return len + ((x ^ y).trailing_zeros() / 8) as usize;
        }
        len += 8;
    }
    while len < most && a[len] == b[len] {
        len += 1;
    }
    len
}

#[inline(always)]
fn link(table: &mut [u32], key: u32, at: usize) -> usize {
    let slot = (key.wrapping_mul(0x9E37_79B1) >> 17) as usize;
    let gap = at + 1 - table[slot] as usize;
    table[slot] = at as u32 + 1;
    if gap > at || gap > WINDOW { 0 } else { gap }
}

impl<'a> Matcher<'a> {
    pub(crate) fn new(data: &'a [u8], level: &'a Level) -> Matcher<'a> {
        Matcher {
            data,
            at: 0,
            short: vec![0; WINDOW],
            head: vec![0; WINDOW],
            prev: vec![0; WINDOW],
            run: 0,
            level,
        }
    }
    #[inline(always)]
    fn insert(&mut self, at: usize) -> (usize, usize) {
        let data = self.data;
        if at >= self.run {
            let rest = data[at..].iter().take_while(|&&byte| byte == data[at]);
            self.run = at + rest.count();
        }
        let run = self.run - at;
        let last = if run > 3 {
            run.min(255) as u8
        } else {
            data.get(at + 3).copied().unwrap_or(0)
        };
        let three = u32::from_le_bytes([data[at], data[at + 1], data[at + 2], 0]);
        let near = link(&mut self.short, three, at);
        let gap = link(&mut self.head, three | (last as u32) << 24, at);
        self.prev[at % WINDOW] = gap as u16;
        (near, gap)
    }
    #[inline]
    fn find(
        &mut self,
        at: usize,
        floor: usize,
        mut tries: usize,
        cap: usize,
        mut each: impl FnMut(usize, usize),
    ) -> (usize, usize) {
        let data = self.data;
        let most = (data.len() - at).min(258);
        if most < 3 {
            return (0, 0);
        }
        let (near, mut gap) = self.insert(at);
        let (mut len, mut best) = (floor, 0);
        if near != 0 && near <= cap {
            let found = same(data, at - near, at, most);
            if found > len && (found > 3 || near <= 4096) {
                (len, best) = (found, near);
                each(len, near);
            }
        }
        let mut far = gap;
        while gap != 0 && far <= cap && tries > 0 && len < most {
            let from = at - far;
            if data[from + len] == data[at + len] {
                let found = same(data, from, at, most);
                if found > len {
                    (len, best) = (found, far);
                    each(len, far);
                }
            }
            if far == WINDOW {
                break;
            }
            gap = self.prev[from % WINDOW] as usize;
            far += gap;
            tries -= 1;
        }
        (len, best)
    }
    pub(crate) fn run(&mut self, tokens: &mut Vec<u32>, cap: usize) {
        let data = self.data;
        let chain = self.level.chain;
        while self.at < data.len() && tokens.len() < cap {
            let (mut len, mut far) = self.find(self.at, 2, chain, WINDOW, |_, _| {});
            let mut seen = self.at + 1;
            while far != 0 && len < self.level.lazy {
                let tries = if len < self.level.good {
                    chain
                } else {
                    chain / 4
                };
                let (longer, further) = self.find(self.at + 1, len, tries, WINDOW, |_, _| {});
                seen = self.at + 2;
                if further == 0 {
                    break;
                }
                tokens.push(data[self.at] as u32);
                self.at += 1;
                (len, far) = (longer, further);
            }
            if far == 0 {
                tokens.push(data[self.at] as u32);
                self.at += 1;
                continue;
            }
            tokens.push((len << 16 | far) as u32);
            self.at += len;
            for at in seen..self.at.min(data.len().saturating_sub(2)) {
                self.insert(at);
            }
        }
    }
    pub(crate) fn gather(&mut self, index: &mut Vec<u32>, pairs: &mut Vec<u32>) {
        let data = self.data;
        let end = data.len().min(self.at + SPAN);
        index.clear();
        pairs.clear();
        let (mut reach, mut held) = (0, 0);
        while self.at < end && pairs.len() < ROOM {
            index.push(pairs.len() as u32);
            let inside = self.at + 258 <= reach;
            let (tries, cap) = match inside {
                true => (NEAR, held - 1),
                false => (self.level.chain, WINDOW),
            };
            let record = |len, far| pairs.push((len << 16 | far) as u32);
            let (len, far) = self.find(self.at, 2, tries, cap, record);
            if len == 258 {
                held = far;
                let most = data.len().min(end + 258) - self.at;
                reach = self.at + same(data, self.at - far, self.at, most);
            } else if inside {
                pairs.push((258 << 16 | held) as u32);
            }
            if let (true, Some(pair)) = (inside, pairs.last_mut()) {
                *pair |= WHOLE;
            }
            self.at += 1;
        }
        index.push(pairs.len() as u32);
    }
}
