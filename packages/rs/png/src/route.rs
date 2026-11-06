use crate::deflate::{Tally, price};
use crate::matcher::WHOLE;
use crate::tables::{DIST_EXTRA, LEN_EXTRA, LEN_SLOT, dist_slot, fixed};

const UNIT: u32 = 6;

struct Costs {
    lit: [u32; 256],
    len: [u32; 259],
    dist: [u32; 30],
}

fn log2(value: u32) -> u32 {
    let whole = value.ilog2();
    let mut scaled = (value as u64) << (31 - whole);
    let mut out = whole;
    for _ in 0..UNIT {
        scaled = (scaled * scaled) >> 31;
        let bit = scaled >> 32;
        scaled >>= bit;
        out = out << 1 | bit as u32;
    }
    out
}

impl Costs {
    fn of(lit: &[u32], dist: &[u32]) -> Costs {
        let mut costs = Costs {
            lit: [0; 256],
            len: [0; 259],
            dist: [0; 30],
        };
        costs.lit.copy_from_slice(&lit[..256]);
        for (len, cost) in costs.len.iter_mut().enumerate().skip(3) {
            let slot = LEN_SLOT[len] as usize;
            *cost = lit[257 + slot] + ((LEN_EXTRA[slot] as u32) << UNIT);
        }
        for (slot, cost) in costs.dist.iter_mut().enumerate() {
            *cost = dist[slot] + ((DIST_EXTRA[slot] as u32) << UNIT);
        }
        costs
    }
    fn flat() -> Costs {
        let (lit, dist) = fixed();
        Costs::of(
            &lit.map(|bits| (bits as u32) << UNIT),
            &dist.map(|bits| (bits as u32) << UNIT),
        )
    }
    fn fit(tally: &Tally) -> Costs {
        let price = |counts: &[u32]| -> Vec<u32> {
            let total = log2(counts.iter().sum::<u32>() + 1);
            counts
                .iter()
                .map(|&count| total - log2(count.max(1)))
                .collect()
        };
        Costs::of(&price(&tally.lit), &price(&tally.dist))
    }
}

#[derive(Default)]
pub(crate) struct Route {
    pub(crate) index: Vec<u32>,
    pub(crate) pairs: Vec<u32>,
    best: Vec<u64>,
}

impl Route {
    fn walk(&mut self, data: &[u8], base: usize, costs: &Costs, tokens: &mut Vec<u32>) {
        let size = data.len();
        self.best.clear();
        self.best.resize(size + 1, u64::MAX);
        self.best[0] = 0;
        for (at, &byte) in data.iter().enumerate() {
            let here = (self.best[at] >> 32) as u32;
            let plain = ((here + costs.lit[byte as usize]) as u64) << 32;
            self.best[at + 1] = self.best[at + 1].min(plain);
            let (from, to) = (self.index[base + at], self.index[base + at + 1]);
            let mut low = 3;
            for &pair in &self.pairs[from as usize..to as usize] {
                let (len, far) = ((pair >> 16 & 511) as usize, pair & 0xFFFF);
                let top = len.min(size - at);
                if pair & WHOLE != 0 {
                    low = top;
                }
                let reach = here + costs.dist[dist_slot(far as usize)];
                if low <= top {
                    let slots = &mut self.best[at + low..=at + top];
                    for (slot, (len, cost)) in slots.iter_mut().zip((low..).zip(&costs.len[low..]))
                    {
                        let step = (len as u32) << 16 | far;
                        *slot = (*slot).min(((reach + cost) as u64) << 32 | step as u64);
                    }
                }
                low = len + 1;
            }
        }
        tokens.clear();
        let mut at = size;
        while at > 0 {
            let step = self.best[at] as u32;
            tokens.push(if step == 0 { data[at - 1] as u32 } else { step });
            at -= (step >> 16).max(1) as usize;
        }
        tokens.reverse();
    }
    pub(crate) fn first(&mut self, data: &[u8], tokens: &mut Vec<u32>) {
        self.walk(data, 0, &Costs::flat(), tokens);
    }
    pub(crate) fn refine(
        &mut self,
        data: &[u8],
        base: usize,
        tokens: &mut Vec<u32>,
        passes: usize,
    ) {
        let mut tally = Tally::of(tokens);
        let mut best = price(&tally);
        let mut trial = Vec::new();
        for _ in 0..passes {
            self.walk(data, base, &Costs::fit(&tally), &mut trial);
            tally = Tally::of(&trial);
            let cost = price(&tally);
            if cost >= best {
                break;
            }
            let gain = best - cost;
            best = cost;
            std::mem::swap(tokens, &mut trial);
            if gain < best >> 10 {
                break;
            }
        }
    }
}
