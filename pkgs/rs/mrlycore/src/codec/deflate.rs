use super::png::adler32;
use crate::errors::{value_error, Result};

const WINDOW: usize = 32768;

const MIN_MATCH: usize = 3;

const MAX_MATCH: usize = 258;

const HASH_BITS: u32 = 15;

const MAX_CHAIN: usize = 128;

const LENGTH_BASE: [u32; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];

const LENGTH_EXTRA: [u32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];

const DIST_BASE: [u32; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];

const DIST_EXTRA: [u32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

const CODE_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

/// Compresses bytes into a zlib stream that inflate reads back whole.
///
/// ```
/// let stream = mrlycore::codec::deflate(b"honk honk honk honk");
/// assert_eq!(mrlycore::codec::inflate(&stream).unwrap(), b"honk honk honk honk");
/// ```
pub fn deflate(data: &[u8]) -> Vec<u8> {
    zlib(data)
}

pub(super) fn zlib(data: &[u8]) -> Vec<u8> {
    let mut writer = BitWriter::new(data.len() / 8 + 64);
    writer.bits(1, 1);
    writer.bits(1, 2);
    let mut head = vec![u32::MAX; 1 << HASH_BITS];
    let mut prev = vec![u32::MAX; data.len()];
    let mut i = 0;
    while i < data.len() {
        let (len, dist) = longest_match(data, i, &head, &prev);
        if len >= MIN_MATCH {
            emit_match(&mut writer, len as u32, dist as u32);
            for k in i..i + len {
                insert(data, k, &mut head, &mut prev);
            }
            i += len;
        } else {
            literal(&mut writer, u32::from(data[i]));
            insert(data, i, &mut head, &mut prev);
            i += 1;
        }
    }
    literal(&mut writer, 256);
    let mut out = writer.finish();
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn longest_match(data: &[u8], i: usize, head: &[u32], prev: &[u32]) -> (usize, usize) {
    if i + MIN_MATCH > data.len() {
        return (0, 0);
    }
    let floor = i.saturating_sub(WINDOW);
    let most = (data.len() - i).min(MAX_MATCH);
    let mut best_len = 0;
    let mut best_dist = 0;
    let mut candidate = head[hash3(data, i)];
    let mut chain = 0;
    while candidate != u32::MAX && candidate as usize >= floor && chain < MAX_CHAIN {
        let j = candidate as usize;
        let mut len = 0;
        while len < most && data[j + len] == data[i + len] {
            len += 1;
        }
        if len > best_len {
            best_len = len;
            best_dist = i - j;
            if len == most {
                break;
            }
        }
        candidate = prev[j];
        chain += 1;
    }
    if best_len >= MIN_MATCH {
        (best_len, best_dist)
    } else {
        (0, 0)
    }
}

fn insert(data: &[u8], k: usize, head: &mut [u32], prev: &mut [u32]) {
    if k + MIN_MATCH <= data.len() {
        let h = hash3(data, k);
        prev[k] = head[h];
        head[h] = k as u32;
    }
}

fn hash3(data: &[u8], i: usize) -> usize {
    let x = u32::from(data[i]) | u32::from(data[i + 1]) << 8 | u32::from(data[i + 2]) << 16;
    (x.wrapping_mul(0x9e37_79b1) >> (32 - HASH_BITS)) as usize
}

fn literal(writer: &mut BitWriter, symbol: u32) {
    match symbol {
        0..=143 => writer.huffman(0x30 + symbol, 8),
        144..=255 => writer.huffman(0x190 + symbol - 144, 9),
        256..=279 => writer.huffman(symbol - 256, 7),
        _ => writer.huffman(0xc0 + symbol - 280, 8),
    }
}

fn emit_match(writer: &mut BitWriter, len: u32, dist: u32) {
    let mut s = 28;
    while LENGTH_BASE[s] > len {
        s -= 1;
    }
    literal(writer, 257 + s as u32);
    writer.bits(len - LENGTH_BASE[s], LENGTH_EXTRA[s]);
    let mut d = 29;
    while DIST_BASE[d] > dist {
        d -= 1;
    }
    writer.huffman(d as u32, 5);
    writer.bits(dist - DIST_BASE[d], DIST_EXTRA[d]);
}

struct BitWriter {
    out: Vec<u8>,
    acc: u64,
    filled: u32,
}

impl BitWriter {
    fn new(capacity: usize) -> BitWriter {
        let mut out = Vec::with_capacity(capacity);
        out.extend_from_slice(&[0x78, 0x01]);
        BitWriter {
            out,
            acc: 0,
            filled: 0,
        }
    }
    fn bits(&mut self, value: u32, count: u32) {
        self.acc |= u64::from(value) << self.filled;
        self.filled += count;
        while self.filled >= 8 {
            self.out.push(self.acc as u8);
            self.acc >>= 8;
            self.filled -= 8;
        }
    }
    fn huffman(&mut self, code: u32, count: u32) {
        let mut reversed = 0;
        for bit in 0..count {
            reversed |= (code >> bit & 1) << (count - 1 - bit);
        }
        self.bits(reversed, count);
    }
    fn finish(mut self) -> Vec<u8> {
        if self.filled > 0 {
            self.out.push(self.acc as u8);
        }
        self.out
    }
}

/// Decompresses a zlib stream to its bytes, or an error for a broken or truncated one.
///
/// ```
/// assert!(mrlycore::codec::inflate(b"not a zlib stream").is_err());
/// ```
pub fn inflate(stream: &[u8]) -> Result<Vec<u8>> {
    if stream.len() < 6 || stream[0] & 0x0f != 8 || stream[1] & 0x20 != 0 {
        return value_error("bad zlib stream.");
    }
    let mut reader = BitReader {
        data: &stream[2..stream.len() - 4],
        byte: 0,
        bit: 0,
    };
    let mut out = Vec::new();
    loop {
        let last = reader.take(1)?;
        match reader.take(2)? {
            0 => stored(&mut reader, &mut out)?,
            1 => block(&mut reader, &mut out, &fixed_lengths(), &[5u8; 30])?,
            2 => {
                let (lit, dist) = dynamic_lengths(&mut reader)?;
                block(&mut reader, &mut out, &lit, &dist)?;
            }
            _ => return value_error("bad deflate block."),
        }
        if last == 1 {
            break;
        }
    }
    if stream[stream.len() - 4..] != adler32(&out).to_be_bytes() {
        return value_error("bad zlib checksum.");
    }
    Ok(out)
}

fn stored(reader: &mut BitReader, out: &mut Vec<u8>) -> Result<()> {
    reader.align();
    let low = reader.raw()?;
    let high = reader.raw()?;
    let len = u16::from_le_bytes([low, high]);
    let nlow = reader.raw()?;
    let nhigh = reader.raw()?;
    if len ^ u16::from_le_bytes([nlow, nhigh]) != 0xffff {
        return value_error("bad stored block.");
    }
    for _ in 0..len {
        out.push(reader.raw()?);
    }
    Ok(())
}

fn block(reader: &mut BitReader, out: &mut Vec<u8>, lit: &[u8], dist: &[u8]) -> Result<()> {
    let lit = huffman(lit);
    let dist = huffman(dist);
    loop {
        let symbol = decode(reader, &lit)?;
        if symbol == 256 {
            return Ok(());
        }
        if symbol < 256 {
            out.push(symbol as u8);
            continue;
        }
        let s = symbol as usize - 257;
        if s >= LENGTH_BASE.len() {
            return value_error("bad length symbol.");
        }
        let len = (LENGTH_BASE[s] + reader.take(LENGTH_EXTRA[s])?) as usize;
        let d = decode(reader, &dist)? as usize;
        if d >= DIST_BASE.len() {
            return value_error("bad distance symbol.");
        }
        let far = (DIST_BASE[d] + reader.take(DIST_EXTRA[d])?) as usize;
        if far > out.len() {
            return value_error("bad distance.");
        }
        let from = out.len() - far;
        for k in 0..len {
            out.push(out[from + k]);
        }
    }
}

fn fixed_lengths() -> Vec<u8> {
    let mut lengths = vec![8u8; 288];
    for length in &mut lengths[144..256] {
        *length = 9;
    }
    for length in &mut lengths[256..280] {
        *length = 7;
    }
    lengths
}

fn dynamic_lengths(reader: &mut BitReader) -> Result<(Vec<u8>, Vec<u8>)> {
    let hlit = reader.take(5)? as usize + 257;
    let hdist = reader.take(5)? as usize + 1;
    let hclen = reader.take(4)? as usize + 4;
    let mut meta = [0u8; 19];
    for &slot in CODE_ORDER.iter().take(hclen) {
        meta[slot] = reader.take(3)? as u8;
    }
    let codes = huffman(&meta);
    let mut lengths = Vec::with_capacity(hlit + hdist);
    while lengths.len() < hlit + hdist {
        let symbol = decode(reader, &codes)?;
        match symbol {
            0..=15 => lengths.push(symbol as u8),
            16 => {
                let Some(&last) = lengths.last() else {
                    return value_error("bad length repeat.");
                };
                lengths.extend(std::iter::repeat_n(last, (3 + reader.take(2)?) as usize));
            }
            17 => lengths.extend(std::iter::repeat_n(0, (3 + reader.take(3)?) as usize)),
            _ => lengths.extend(std::iter::repeat_n(0, (11 + reader.take(7)?) as usize)),
        }
    }
    if lengths.len() != hlit + hdist {
        return value_error("bad length counts.");
    }
    let dist = lengths.split_off(hlit);
    Ok((lengths, dist))
}

struct Huffman {
    counts: [u16; 16],
    symbols: Vec<u16>,
}

fn huffman(lengths: &[u8]) -> Huffman {
    let mut counts = [0u16; 16];
    for &length in lengths {
        counts[length as usize] += 1;
    }
    counts[0] = 0;
    let mut offsets = [0u16; 16];
    for length in 1..16 {
        offsets[length] = offsets[length - 1] + counts[length - 1];
    }
    let mut symbols = vec![0u16; offsets[15] as usize + counts[15] as usize];
    for (symbol, &length) in lengths.iter().enumerate() {
        if length > 0 {
            symbols[offsets[length as usize] as usize] = symbol as u16;
            offsets[length as usize] += 1;
        }
    }
    Huffman { counts, symbols }
}

fn decode(reader: &mut BitReader, huff: &Huffman) -> Result<u32> {
    let mut code = 0u32;
    let mut first = 0u32;
    let mut index = 0u32;
    for length in 1..16 {
        code |= reader.take(1)?;
        let count = u32::from(huff.counts[length]);
        if code < first + count {
            return Ok(u32::from(huff.symbols[(index + code - first) as usize]));
        }
        index += count;
        first = (first + count) << 1;
        code <<= 1;
    }
    value_error("bad huffman code.")
}

struct BitReader<'a> {
    data: &'a [u8],
    byte: usize,
    bit: u32,
}

impl BitReader<'_> {
    fn take(&mut self, count: u32) -> Result<u32> {
        let mut value = 0;
        for i in 0..count {
            if self.byte >= self.data.len() {
                return value_error("truncated deflate stream.");
            }
            let bit = u32::from(self.data[self.byte]) >> self.bit & 1;
            value |= bit << i;
            self.bit += 1;
            if self.bit == 8 {
                self.bit = 0;
                self.byte += 1;
            }
        }
        Ok(value)
    }
    fn align(&mut self) {
        if self.bit > 0 {
            self.bit = 0;
            self.byte += 1;
        }
    }
    fn raw(&mut self) -> Result<u8> {
        if self.byte >= self.data.len() {
            return value_error("truncated deflate stream.");
        }
        let value = self.data[self.byte];
        self.byte += 1;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zlib_handles_empty_data() {
        assert_eq!(zlib(&[]), vec![0x78, 0x01, 0x03, 0x00, 0, 0, 0, 1]);
    }
    #[test]
    fn zlib_round_trips() {
        let mut random = crate::chacha::ChaCha8::from_u64(7);
        let noise: Vec<u8> = (0..200_000).map(|_| random.next_u32() as u8).collect();
        let mut far = noise[..50_000].to_vec();
        far.extend_from_slice(&noise[..50_000]);
        let stripes: Vec<u8> = (0..90_000u32).map(|i| (i / 400 % 7) as u8).collect();
        let cases: Vec<Vec<u8>> = vec![
            vec![],
            b"a".to_vec(),
            b"abcabcabcabcabcabc".to_vec(),
            vec![0; 70_000],
            stripes,
            far,
            noise,
        ];
        for data in &cases {
            assert_eq!(&inflate(&zlib(data)).unwrap(), data);
        }
    }
    #[test]
    fn zlib_compresses_flat_data() {
        let data = vec![9u8; 100_000];
        let stream = zlib(&data);
        assert!(stream.len() < 1000);
        assert_eq!(inflate(&stream).unwrap(), data);
    }
    #[test]
    fn inflate_reads_dynamic_huffman_streams() {
        let mut data: Vec<u8> = (0..96u32).map(|i| ((i * 7 + i / 5) % 251) as u8).collect();
        data.extend_from_slice(b"the door opens the door opens the door opens");
        let stream = [
            120, 218, 99, 96, 231, 19, 149, 81, 209, 54, 178, 116, 240, 240, 15, 139, 77, 201, 41,
            174, 106, 236, 152, 48, 125, 222, 210, 53, 91, 118, 31, 58, 121, 225, 198, 253, 103,
            111, 191, 48, 114, 240, 139, 201, 170, 234, 24, 91, 57, 122, 6, 132, 199, 165, 230,
            150, 84, 55, 117, 78, 156, 49, 127, 217, 218, 173, 123, 14, 159, 186, 120, 243, 193,
            243, 119, 95, 153, 56, 5, 196, 229, 212, 116, 77, 172, 157, 188, 2, 35, 226, 211, 242,
            74, 107, 154, 187, 38, 205, 92, 176, 124, 221, 182, 146, 140, 84, 133, 148, 252, 252,
            34, 133, 252, 130, 212, 188, 98, 5, 188, 92, 0, 94, 114, 59, 28,
        ];
        assert_eq!(inflate(&stream).unwrap(), data);
    }
    #[test]
    fn inflate_reads_stored_blocks() {
        let stream = [
            120, 1, 1, 25, 0, 230, 255, 115, 116, 111, 114, 101, 100, 32, 98, 121, 116, 101, 115,
            32, 112, 97, 115, 115, 32, 116, 104, 114, 111, 117, 103, 104, 127, 143, 9, 209,
        ];
        assert_eq!(inflate(&stream).unwrap(), b"stored bytes pass through");
    }
    #[test]
    fn inflate_rejects_broken_streams() {
        assert!(inflate(&[]).is_err());
        assert!(inflate(&[0x78, 0x01, 0x03]).is_err());
        let mut stream = zlib(b"checksummed");
        let at = stream.len() - 1;
        stream[at] ^= 1;
        assert!(inflate(&stream).is_err());
    }
}
