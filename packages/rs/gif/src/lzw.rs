use crate::Result;

const CODES: usize = 4096;

struct Bits {
    out: Vec<u8>,
    acc: u64,
    n: u32,
}

impl Bits {
    fn put(&mut self, code: u16, size: u32) {
        self.acc |= (code as u64) << self.n;
        self.n += size;
        if self.n >= 32 {
            self.out.extend_from_slice(&(self.acc as u32).to_le_bytes());
            self.acc >>= 32;
            self.n -= 32;
        }
    }
    fn finish(mut self) -> Vec<u8> {
        while self.n > 0 {
            self.out.push(self.acc as u8);
            self.acc >>= 8;
            self.n = self.n.saturating_sub(8);
        }
        self.out
    }
}

pub(crate) fn pack(indices: &[u8]) -> (u8, Vec<u8>) {
    let top = indices.iter().copied().max().unwrap_or(0);
    let min_size = (8 - top.leading_zeros() as u8).max(2);
    let mut bits = Bits {
        out: Vec::with_capacity(indices.len() / 4 + 16),
        acc: 0,
        n: 0,
    };
    let clear = 1u16 << min_size;
    let end = clear + 1;
    let alphabet = 1usize << min_size;
    let mut child = vec![0u16; CODES * alphabet];
    let mut taken: Vec<u32> = Vec::with_capacity(CODES);
    let mut size = min_size as u32 + 1;
    let mut next = end + 1;
    bits.put(clear, size);
    let Some((&first, rest)) = indices.split_first() else {
        bits.put(end, size);
        return (min_size, bits.finish());
    };
    let mut prefix = first as u16;
    for &byte in rest {
        let slot = prefix as usize * alphabet + byte as usize;
        let found = child[slot];
        if found != 0 {
            prefix = found;
            continue;
        }
        bits.put(prefix, size);
        if next as usize == CODES {
            bits.put(clear, size);
            for &at in &taken {
                child[at as usize] = 0;
            }
            taken.clear();
            size = min_size as u32 + 1;
            next = end + 1;
        } else {
            child[slot] = next;
            taken.push(slot as u32);
            next += 1;
            if next > 1 << size && size < 12 {
                size += 1;
            }
        }
        prefix = byte as u16;
    }
    bits.put(prefix, size);
    if next == 1 << size && size < 12 {
        size += 1;
    }
    bits.put(end, size);
    (min_size, bits.finish())
}

pub(crate) fn unpack(data: &[u8], min_size: u8, limit: usize) -> Result<Vec<u8>> {
    if !(2..=8).contains(&min_size) {
        return Err(format!("minimum code size {min_size} is outside 2 to 8."));
    }
    let clear = 1usize << min_size;
    let end = clear + 1;
    let mut start = vec![0usize; CODES];
    let mut len = vec![0usize; CODES];
    let mut out: Vec<u8> = Vec::with_capacity(limit.min(data.len().saturating_mul(64)));
    let mut size = min_size as u32 + 1;
    let mut next = end + 1;
    let mut prev: Option<(usize, usize)> = None;
    let (mut acc, mut n, mut at) = (0u64, 0u32, 0usize);
    while out.len() < limit {
        if n < size {
            if let Some(word) = data.get(at..at + 4) {
                acc |= (u32::from_le_bytes([word[0], word[1], word[2], word[3]]) as u64) << n;
                n += 32;
                at += 4;
            } else {
                while n < size && at < data.len() {
                    acc |= (data[at] as u64) << n;
                    n += 8;
                    at += 1;
                }
                if n < size {
                    break;
                }
            }
        }
        let code = (acc & ((1 << size) - 1)) as usize;
        acc >>= size;
        n -= size;
        if code == clear {
            size = min_size as u32 + 1;
            next = end + 1;
            prev = None;
            continue;
        }
        if code == end {
            break;
        }
        let here = out.len();
        if code < clear {
            out.push(code as u8);
        } else if code > end && code < next {
            out.extend_from_within(start[code]..start[code] + len[code]);
        } else if let Some((from, count)) = prev.filter(|_| code == next) {
            out.extend_from_within(from..from + count);
            out.push(out[from]);
        } else {
            return Err(format!("code {code} comes before the table reaches it."));
        }
        if let Some((from, count)) = prev
            && next < CODES
        {
            start[next] = from;
            len[next] = count + 1;
            next += 1;
            if next >= 1 << size && size < 12 {
                size += 1;
            }
        }
        prev = Some((here, out.len() - here));
    }
    out.truncate(limit);
    Ok(out)
}
