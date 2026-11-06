use crate::huffman::{codes, lengths};
use crate::matcher::{Matcher, level};
use crate::route::Route;
use crate::tables::{
    DIST_BASE, DIST_EXTRA, LEN_BASE, LEN_EXTRA, LEN_SLOT, ORDER, dist_slot, fixed,
};

const SEGMENT: usize = 1 << 16;
const PART: usize = 1 << 30;
const DEPTH: u32 = 4;

#[derive(Clone)]
pub(crate) struct Tally {
    pub(crate) lit: [u32; 286],
    pub(crate) dist: [u32; 30],
    extra: u64,
    bytes: usize,
}

impl Tally {
    fn new() -> Tally {
        Tally {
            lit: [0; 286],
            dist: [0; 30],
            extra: 0,
            bytes: 0,
        }
    }
    pub(crate) fn of(tokens: &[u32]) -> Tally {
        let mut tally = Tally::new();
        for &token in tokens {
            tally.add(token);
        }
        tally
    }
    fn add(&mut self, token: u32) {
        if token < 256 {
            self.lit[token as usize] += 1;
            self.bytes += 1;
            return;
        }
        let (len, far) = ((token >> 16) as usize, (token & 0xFFFF) as usize);
        let (a, b) = (LEN_SLOT[len] as usize, dist_slot(far));
        self.lit[257 + a] += 1;
        self.dist[b] += 1;
        self.extra += (LEN_EXTRA[a] + DIST_EXTRA[b]) as u64;
        self.bytes += len;
    }
    fn less(&self, other: &Tally) -> Tally {
        let mut out = self.clone();
        for (count, taken) in out.lit.iter_mut().zip(other.lit) {
            *count -= taken;
        }
        for (count, taken) in out.dist.iter_mut().zip(other.dist) {
            *count -= taken;
        }
        out.extra -= other.extra;
        out.bytes -= other.bytes;
        out
    }
}

struct Plan {
    lit: Vec<u8>,
    dist: Vec<u8>,
    runs: Vec<(u8, u8)>,
    meta: Vec<u8>,
    lits: usize,
    dists: usize,
    metas: usize,
    bits: u64,
}

fn runs(lengths: &[u8]) -> Vec<(u8, u8)> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < lengths.len() {
        let value = lengths[at];
        let mut run = lengths[at..].iter().take_while(|&&v| v == value).count();
        at += run;
        if value == 0 {
            while run >= 11 {
                let step = run.min(138);
                out.push((18, (step - 11) as u8));
                run -= step;
            }
            if run >= 3 {
                out.push((17, (run - 3) as u8));
                run = 0;
            }
        } else {
            out.push((value, 0));
            run -= 1;
            while run >= 3 {
                let step = run.min(6);
                out.push((16, (step - 3) as u8));
                run -= step;
            }
        }
        out.extend(std::iter::repeat_n((value, 0), run));
    }
    out
}

fn spare(symbol: u8) -> u32 {
    match symbol {
        16 => 2,
        17 => 3,
        18 => 7,
        _ => 0,
    }
}

fn weigh(counts: &[u32], lengths: &[u8]) -> u64 {
    counts
        .iter()
        .zip(lengths)
        .map(|(&count, &len)| count as u64 * len as u64)
        .sum()
}

fn plan(tally: &Tally) -> Plan {
    let mut counts = tally.lit;
    counts[256] = 1;
    let lit = lengths(&counts, 15);
    let dist = lengths(&tally.dist, 15);
    let lits = lit
        .iter()
        .rposition(|&len| len > 0)
        .map_or(257, |at| at + 1);
    let dists = dist.iter().rposition(|&len| len > 0).map_or(1, |at| at + 1);
    let runs = runs(&[&lit[..lits], &dist[..dists]].concat());
    let mut used = [0u32; 19];
    for &(symbol, _) in &runs {
        used[symbol as usize] += 1;
    }
    let meta = lengths(&used, 7);
    let metas = (4..19).rev().find(|&n| meta[ORDER[n]] > 0).unwrap_or(3) + 1;
    let head: u64 = runs
        .iter()
        .map(|&(symbol, _)| (meta[symbol as usize] as u32 + spare(symbol)) as u64)
        .sum();
    let bits = 17
        + 3 * metas as u64
        + head
        + weigh(&counts, &lit)
        + weigh(&tally.dist, &dist)
        + tally.extra;
    Plan {
        lit,
        dist,
        runs,
        meta,
        lits,
        dists,
        metas,
        bits,
    }
}

fn flat(tally: &Tally) -> u64 {
    let (lit, dist) = fixed();
    3 + 7 + weigh(&tally.lit, &lit) + weigh(&tally.dist, &dist) + tally.extra
}

fn raw(bytes: usize) -> u64 {
    40 * bytes.div_ceil(65535).max(1) as u64 + 8 * bytes as u64
}

pub(crate) fn price(tally: &Tally) -> u64 {
    plan(tally).bits.min(flat(tally)).min(raw(tally.bytes))
}

struct Writer {
    out: Vec<u8>,
    hold: u64,
    count: u32,
}

impl Writer {
    #[inline]
    fn put(&mut self, value: u32, bits: u32) {
        self.hold |= (value as u64) << self.count;
        self.count += bits;
        if self.count >= 32 {
            self.out
                .extend_from_slice(&(self.hold as u32).to_le_bytes());
            self.hold >>= 32;
            self.count -= 32;
        }
    }
    fn align(&mut self) {
        while self.count > 0 {
            self.out.push(self.hold as u8);
            self.hold >>= 8;
            self.count = self.count.saturating_sub(8);
        }
    }
    fn stored(&mut self, raw: &[u8], last: bool) {
        let mut parts = raw.chunks(65535).peekable();
        loop {
            let part = parts.next().unwrap_or(&[]);
            let end = parts.peek().is_none();
            self.put((last && end) as u32, 3);
            self.align();
            let len = part.len() as u16;
            self.out.extend_from_slice(&len.to_le_bytes());
            self.out.extend_from_slice(&(!len).to_le_bytes());
            self.out.extend_from_slice(part);
            if end {
                break;
            }
        }
    }
    fn body(&mut self, tokens: &[u32], lit: &[u8], dist: &[u8]) {
        let (lit_code, dist_code) = (codes(lit), codes(dist));
        for &token in tokens {
            if token < 256 {
                self.put(lit_code[token as usize] as u32, lit[token as usize] as u32);
                continue;
            }
            let (len, far) = ((token >> 16) as usize, (token & 0xFFFF) as usize);
            let slot = LEN_SLOT[len] as usize;
            self.put(lit_code[257 + slot] as u32, lit[257 + slot] as u32);
            self.put(
                (len - LEN_BASE[slot] as usize) as u32,
                LEN_EXTRA[slot] as u32,
            );
            let slot = dist_slot(far);
            self.put(dist_code[slot] as u32, dist[slot] as u32);
            self.put(
                (far - DIST_BASE[slot] as usize) as u32,
                DIST_EXTRA[slot] as u32,
            );
        }
        self.put(lit_code[256] as u32, lit[256] as u32);
    }
    fn dynamic(&mut self, tokens: &[u32], plan: &Plan, last: bool) {
        self.put(last as u32 | 4, 3);
        self.put(plan.lits as u32 - 257, 5);
        self.put(plan.dists as u32 - 1, 5);
        self.put(plan.metas as u32 - 4, 4);
        for &symbol in &ORDER[..plan.metas] {
            self.put(plan.meta[symbol] as u32, 3);
        }
        let code = codes(&plan.meta);
        for &(symbol, value) in &plan.runs {
            self.put(
                code[symbol as usize] as u32,
                plan.meta[symbol as usize] as u32,
            );
            self.put(value as u32, spare(symbol));
        }
        self.body(tokens, &plan.lit, &plan.dist);
    }
    fn block(&mut self, bytes: &[u8], tokens: &[u32], last: bool) {
        let tally = Tally::of(tokens);
        let plan = plan(&tally);
        let (flat, plain) = (flat(&tally), raw(bytes.len()));
        if plain <= plan.bits.min(flat) {
            self.stored(bytes, last);
        } else if flat <= plan.bits {
            let (lit, dist) = fixed();
            self.put(last as u32 | 2, 3);
            self.body(tokens, &lit, &dist);
        } else {
            self.dynamic(tokens, &plan, last);
        }
    }
}

fn divide(tokens: &[u32], lo: usize, hi: usize, whole: &Tally, depth: u32, out: &mut Vec<usize>) {
    if depth == 0 {
        return;
    }
    let (mut a, mut b) = (lo, hi);
    let mut base = Tally::new();
    let mut best: Option<(u64, usize, Tally)> = None;
    while b - a >= 10 {
        let step = (b - a) / 10;
        let mut left = base.clone();
        let mut marks = Vec::with_capacity(9);
        for at in (1..10).map(|mark| a + mark * step) {
            for &token in &tokens[at - step..at] {
                left.add(token);
            }
            let cost = price(&left) + price(&whole.less(&left));
            marks.push((cost, at, left.clone()));
        }
        let pick = (0..9).min_by_key(|&mark| marks[mark].0).unwrap_or(0);
        if best.as_ref().is_some_and(|best| marks[pick].0 >= best.0) {
            break;
        }
        if pick < 8 {
            b = marks[pick + 1].1;
        }
        if pick > 0 {
            a = marks[pick - 1].1;
            base = marks[pick - 1].2.clone();
        }
        best = Some(marks.swap_remove(pick));
    }
    let Some((cost, at, left)) = best else {
        return;
    };
    if cost >= price(whole) {
        return;
    }
    divide(tokens, lo, at, &left, depth - 1, out);
    out.push(at);
    divide(tokens, at, hi, &whole.less(&left), depth - 1, out);
}

pub(crate) fn pack(data: &[u8], grade: u8) -> Vec<u8> {
    let mut writer = Writer {
        out: Vec::with_capacity(data.len() / 4 + 64),
        hold: 0,
        count: 0,
    };
    if grade == 0 {
        writer.stored(data, true);
        return writer.out;
    }
    let level = level(grade);
    let mut tokens = Vec::with_capacity(SEGMENT.min(data.len()));
    let mut block = Vec::new();
    let mut cuts = Vec::new();
    let mut route = Route::default();
    let parts = data.len().div_ceil(PART);
    for (index, part) in data.chunks(PART).enumerate() {
        let mut matcher = Matcher::new(part, &level);
        while matcher.at < part.len() {
            let start = matcher.at;
            if level.passes > 0 {
                matcher.gather(&mut route.index, &mut route.pairs);
                route.first(&part[start..matcher.at], &mut tokens);
            } else {
                tokens.clear();
                matcher.run(&mut tokens, SEGMENT);
            }
            cuts.clear();
            if level.split {
                divide(
                    &tokens,
                    0,
                    tokens.len(),
                    &Tally::of(&tokens),
                    DEPTH,
                    &mut cuts,
                );
            }
            cuts.push(tokens.len());
            let (mut from, mut first) = (start, 0);
            for &cut in &cuts {
                block.clear();
                block.extend_from_slice(&tokens[first..cut]);
                let size = Tally::of(&block).bytes;
                let bytes = &part[from..from + size];
                if level.passes > 0 {
                    route.refine(bytes, from - start, &mut block, level.passes);
                }
                let last = index + 1 == parts && from + size == part.len();
                writer.block(bytes, &block, last);
                (from, first) = (from + size, cut);
            }
        }
    }
    if data.is_empty() {
        writer.block(&[], &[], true);
    }
    writer.align();
    writer.out
}
