use crate::tables::{DIST_BASE, DIST_EXTRA, LEN_BASE, LEN_EXTRA, ORDER, fixed};

pub(crate) type Fault<T> = std::result::Result<T, &'static str>;

const CUT: &str = "the deflate stream ends early.";
const OVER: &str = "the inflated data runs past the size limit.";
const SLACK: usize = 280;

const LIT: u32 = 0;
const LEN: u32 = 1 << 5;
const END: u32 = 2 << 5;
const SUB: u32 = 3 << 5;
const BAD: u32 = 4 << 5;
const KIND: u32 = 7 << 5;

struct Bits<'a> {
    data: &'a [u8],
    at: usize,
    hold: u64,
    count: u32,
}

impl Bits<'_> {
    #[inline(always)]
    fn fill(&mut self) {
        if self.count < 56 {
            if let Some(word) = self.data.get(self.at..self.at + 8) {
                let word = u64::from_le_bytes([
                    word[0], word[1], word[2], word[3], word[4], word[5], word[6], word[7],
                ]);
                self.hold |= word << self.count;
                self.at += ((63 - self.count) >> 3) as usize;
                self.count |= 56;
            } else {
                while self.count < 56 && self.at < self.data.len() {
                    self.hold |= (self.data[self.at] as u64) << self.count;
                    self.at += 1;
                    self.count += 8;
                }
            }
        }
    }
    #[inline(always)]
    fn skip(&mut self, bits: u32) -> Fault<()> {
        if bits > self.count {
            return Err(CUT);
        }
        self.hold >>= bits;
        self.count -= bits;
        Ok(())
    }
    #[inline(always)]
    fn peek(&self, bits: u32) -> usize {
        (self.hold & ((1 << bits) - 1)) as usize
    }
    #[inline(always)]
    fn take(&mut self, bits: u32) -> Fault<usize> {
        if self.count < bits {
            self.fill();
        }
        let value = self.peek(bits);
        self.skip(bits)?;
        Ok(value)
    }
    fn rewind(&mut self) {
        self.at -= (self.count / 8) as usize;
        self.hold = 0;
        self.count = 0;
    }
}

#[derive(Default)]
struct Table {
    slots: Vec<u32>,
    bits: u32,
}

impl Table {
    fn build(
        &mut self,
        lengths: &[u8],
        primary: u32,
        entry: fn(usize) -> u32,
        lone: bool,
    ) -> Fault<()> {
        let mut count = [0u32; 16];
        for &len in lengths {
            count[len as usize] += 1;
        }
        count[0] = 0;
        let max = (1..16).rev().find(|&bits| count[bits] > 0).unwrap_or(0) as u32;
        let mut left = 1i32;
        let mut next = [0u32; 16];
        let mut code = 0;
        for bits in 1..16 {
            left = (left << 1) - count[bits] as i32;
            if left < 0 {
                return Err("a huffman code has too many short codes.");
            }
            code = (code + count[bits - 1]) << 1;
            next[bits] = code;
        }
        if left > 0 && !(lone && max <= 1) {
            return Err("a huffman code leaves bit patterns unused.");
        }
        self.bits = primary.min(max.max(1));
        let size = 1usize << self.bits;
        let wide = max.saturating_sub(self.bits);
        self.slots.clear();
        self.slots.resize(size, 0);
        for (symbol, &len) in lengths.iter().enumerate() {
            if len == 0 {
                continue;
            }
            let len = len as u32;
            let turned = (next[len as usize].reverse_bits() >> (32 - len)) as usize;
            next[len as usize] += 1;
            let slot = entry(symbol);
            if len <= self.bits {
                let slot = if slot == BAD { 0 } else { slot | len };
                for at in (turned..size).step_by(1 << len) {
                    self.slots[at] = slot;
                }
                continue;
            }
            let low = turned & (size - 1);
            if self.slots[low] & KIND != SUB {
                let at = self.slots.len();
                self.slots.resize(at + (1 << wide), 0);
                self.slots[low] = SUB | self.bits | wide << 8 | (at as u32) << 16;
            }
            let at = (self.slots[low] >> 16) as usize;
            let slot = if slot == BAD {
                0
            } else {
                slot | (len - self.bits)
            };
            for high in ((turned >> self.bits)..1 << wide).step_by(1 << (len - self.bits)) {
                self.slots[at + high] = slot;
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn read(&self, bits: &mut Bits) -> Fault<u32> {
        let mut slot = self.slots[bits.peek(self.bits)];
        if slot & KIND == SUB {
            bits.skip(slot & 31)?;
            slot = self.slots[(slot >> 16) as usize + bits.peek(slot >> 8 & 15)];
        }
        if slot == 0 {
            return Err("a huffman code is not in its table.");
        }
        bits.skip(slot & 31)?;
        Ok(slot)
    }
}

fn plain(symbol: usize) -> u32 {
    LIT | (symbol as u32) << 16
}

fn lit_entry(symbol: usize) -> u32 {
    match symbol {
        0..256 => plain(symbol),
        256 => END,
        257..286 => {
            LEN | (LEN_EXTRA[symbol - 257] as u32) << 8 | (LEN_BASE[symbol - 257] as u32) << 16
        }
        _ => BAD,
    }
}

fn dist_entry(symbol: usize) -> u32 {
    match symbol {
        0..30 => LEN | (DIST_EXTRA[symbol] as u32) << 8 | (DIST_BASE[symbol] as u32) << 16,
        _ => BAD,
    }
}

struct Out {
    data: Vec<u8>,
    at: usize,
    limit: usize,
}

impl Out {
    #[inline(always)]
    fn reserve(&mut self, room: usize) {
        let want = self.at + room;
        if self.data.len() < want {
            let most = self.limit.saturating_add(SLACK);
            self.data
                .resize((self.data.len() * 2).max(want).min(most), 0);
        }
    }
    #[inline(always)]
    fn push(&mut self, slot: u32) -> Fault<()> {
        if self.at >= self.limit {
            return Err(OVER);
        }
        self.data[self.at] = (slot >> 16) as u8;
        self.at += 1;
        Ok(())
    }
    #[inline(always)]
    fn copy(&mut self, len: usize, far: usize) -> Fault<()> {
        if far > self.at {
            return Err("a match reaches back before the start of the data.");
        }
        if len > self.limit - self.at {
            return Err(OVER);
        }
        let (from, to) = (self.at - far, self.at);
        if far >= 16 && len <= 16 {
            self.data.copy_within(from..from + 16, to);
        } else if far == 1 {
            let byte = self.data[from];
            self.data[to..to + len].fill(byte);
        } else if far < 16 && len <= 32 {
            for at in to..to + len {
                self.data[at] = self.data[at - far];
            }
        } else {
            let mut done = 0;
            while done < len {
                let step = (far + done).min(len - done);
                self.data.copy_within(from..from + step, to + done);
                done += step;
            }
        }
        self.at += len;
        Ok(())
    }
}

fn stored(bits: &mut Bits, out: &mut Out) -> Fault<()> {
    bits.skip(bits.count % 8)?;
    bits.rewind();
    let Some(head) = bits.data.get(bits.at..bits.at + 4) else {
        return Err(CUT);
    };
    let len = u16::from_le_bytes([head[0], head[1]]);
    if len != !u16::from_le_bytes([head[2], head[3]]) {
        return Err("a stored block has a length that fails its check.");
    }
    let start = bits.at + 4;
    let Some(body) = bits.data.get(start..start + len as usize) else {
        return Err(CUT);
    };
    if body.len() > out.limit - out.at {
        return Err(OVER);
    }
    out.reserve(body.len());
    out.data[out.at..out.at + body.len()].copy_from_slice(body);
    out.at += body.len();
    bits.at = start + body.len();
    Ok(())
}

fn dynamic(bits: &mut Bits, meta: &mut Table, lit: &mut Table, dist: &mut Table) -> Fault<()> {
    let lits = bits.take(5)? + 257;
    let dists = bits.take(5)? + 1;
    let metas = bits.take(4)? + 4;
    if lits > 286 || dists > 30 {
        return Err("a dynamic block names too many codes.");
    }
    let mut short = [0u8; 19];
    for &symbol in &ORDER[..metas] {
        short[symbol] = bits.take(3)? as u8;
    }
    meta.build(&short, 7, plain, false)?;
    let mut lengths = [0u8; 316];
    let total = lits + dists;
    let mut at = 0;
    while at < total {
        bits.fill();
        let symbol = meta.read(bits)? >> 16;
        let (value, run) = match symbol {
            0..16 => (symbol as u8, 1),
            16 if at == 0 => return Err("a code length repeats with nothing before it."),
            16 => (lengths[at - 1], 3 + bits.take(2)?),
            17 => (0, 3 + bits.take(3)?),
            _ => (0, 11 + bits.take(7)?),
        };
        if at + run > total {
            return Err("a code length run goes past the last code.");
        }
        lengths[at..at + run].fill(value);
        at += run;
    }
    if lengths[256] == 0 {
        return Err("a dynamic block has no end code.");
    }
    lit.build(&lengths[..lits], 10, lit_entry, true)?;
    dist.build(&lengths[lits..total], 8, dist_entry, true)
}

fn block(bits: &mut Bits, lit: &Table, dist: &Table, out: &mut Out) -> Fault<()> {
    loop {
        out.reserve(SLACK);
        bits.fill();
        let mut slot = lit.read(bits)?;
        for _ in 0..2 {
            if slot & KIND != LIT {
                break;
            }
            out.push(slot)?;
            slot = lit.read(bits)?;
        }
        if slot & KIND == LIT {
            out.push(slot)?;
            continue;
        }
        if slot & KIND == END {
            return Ok(());
        }
        bits.fill();
        let extra = bits.peek(slot >> 8 & 15);
        bits.skip(slot >> 8 & 15)?;
        let len = (slot >> 16) as usize + extra;
        let slot = dist.read(bits)?;
        let extra = bits.peek(slot >> 8 & 15);
        bits.skip(slot >> 8 & 15)?;
        out.copy(len, (slot >> 16) as usize + extra)?;
    }
}

pub(crate) fn expand(data: &[u8], limit: usize) -> Fault<(Vec<u8>, usize)> {
    let mut bits = Bits {
        data,
        at: 0,
        hold: 0,
        count: 0,
    };
    let first = limit.min(data.len().saturating_mul(4));
    let mut out = Out {
        data: vec![0; first.saturating_add(SLACK)],
        at: 0,
        limit,
    };
    let (mut meta, mut lit, mut dist) = (Table::default(), Table::default(), Table::default());
    let (mut flat_lit, mut flat_dist) = (Table::default(), Table::default());
    loop {
        let last = bits.take(1)? == 1;
        match bits.take(2)? {
            0 => stored(&mut bits, &mut out)?,
            1 => {
                if flat_lit.slots.is_empty() {
                    let (lits, dists) = fixed();
                    flat_lit.build(&lits, 10, lit_entry, false)?;
                    flat_dist.build(&dists, 8, dist_entry, false)?;
                }
                block(&mut bits, &flat_lit, &flat_dist, &mut out)?;
            }
            2 => {
                dynamic(&mut bits, &mut meta, &mut lit, &mut dist)?;
                block(&mut bits, &lit, &dist, &mut out)?;
            }
            _ => return Err("a deflate block has the reserved type 3."),
        }
        if last {
            break;
        }
    }
    out.data.truncate(out.at);
    Ok((out.data, bits.at - (bits.count / 8) as usize))
}
