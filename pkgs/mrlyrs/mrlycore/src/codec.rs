use super::errors::{value_error, Result};

pub fn png(colors: &[[u8; 4]], width: usize, height: usize, scale: usize) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    if colors.len() != width * height {
        return value_error("colors length must equal width * height.");
    }
    let out_w = width * scale;
    let out_h = height * scale;
    let mut pixels = Vec::with_capacity(out_w * out_h * 4);
    for y in 0..out_h {
        for x in 0..out_w {
            pixels.extend_from_slice(&colors[(y / scale) * width + (x / scale)]);
        }
    }
    encode(&pixels, out_w, out_h)
}

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

const ADLER_MODULO: u32 = 65521;

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

fn encode(pixels: &[u8], width: usize, height: usize) -> Result<Vec<u8>> {
    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&(width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    let mut bytes = Vec::with_capacity(pixels.len() + height + 128);
    bytes.extend_from_slice(&PNG_SIGNATURE);
    chunk(&mut bytes, b"IHDR", &header);
    chunk(&mut bytes, b"IDAT", &zlib(&filter(pixels, width, height)));
    chunk(&mut bytes, b"IEND", &[]);
    Ok(bytes)
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], payload: &[u8]) {
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(payload);
    out.extend_from_slice(&crc32(&[kind, payload]).to_be_bytes());
}

fn filter(pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
    let stride = width * 4;
    let mut out = Vec::with_capacity(height * (stride + 1));
    for row in 0..height {
        out.push(0);
        out.extend_from_slice(&pixels[row * stride..(row + 1) * stride]);
    }
    out
}

fn zlib(data: &[u8]) -> Vec<u8> {
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

const CRC32_TABLE: [u32; 256] = crc32_table();

const fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut index = 0;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 {
                0xedb8_8320 ^ (value >> 1)
            } else {
                value >> 1
            };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

fn crc32(parts: &[&[u8]]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for part in parts {
        for &byte in *part {
            crc = CRC32_TABLE[((crc ^ byte as u32) & 0xff) as usize] ^ (crc >> 8);
        }
    }
    crc ^ 0xffff_ffff
}

fn adler32(bytes: &[u8]) -> u32 {
    let mut low = 1u32;
    let mut high = 0u32;
    for &byte in bytes {
        low = (low + byte as u32) % ADLER_MODULO;
        high = (high + low) % ADLER_MODULO;
    }
    high << 16 | low
}

pub fn unpng(bytes: &[u8]) -> Result<(usize, usize, Vec<[u8; 4]>)> {
    if bytes.len() < 8 || bytes[..8] != PNG_SIGNATURE {
        return value_error("not a png.");
    }
    let mut at = 8;
    let mut width = 0;
    let mut height = 0;
    let mut color = 0u8;
    let mut depth = 0usize;
    let mut headed = false;
    let mut palette: Vec<[u8; 4]> = Vec::new();
    let mut idat = Vec::new();
    while at + 12 <= bytes.len() {
        let len = u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
        let len = len as usize;
        if at + 12 + len > bytes.len() {
            return value_error("truncated png chunk.");
        }
        let kind = &bytes[at + 4..at + 8];
        let body = &bytes[at + 8..at + 8 + len];
        match kind {
            b"IHDR" => {
                if len != 13 || body[10] != 0 || body[11] != 0 || body[12] != 0 {
                    return value_error("unsupported png header.");
                }
                width = u32::from_be_bytes([body[0], body[1], body[2], body[3]]) as usize;
                height = u32::from_be_bytes([body[4], body[5], body[6], body[7]]) as usize;
                depth = body[8] as usize;
                color = body[9];
                headed = true;
            }
            b"PLTE" => palette = body.chunks(3).map(|c| [c[0], c[1], c[2], 255]).collect(),
            b"tRNS" => {
                for (i, &alpha) in body.iter().enumerate() {
                    if i < palette.len() {
                        palette[i][3] = alpha;
                    }
                }
            }
            b"IDAT" => idat.extend_from_slice(body),
            b"IEND" => break,
            _ => {}
        }
        at += 12 + len;
    }
    if !headed || width == 0 || height == 0 {
        return value_error("missing png header.");
    }
    let channels = match color {
        0 => 1,
        2 => 3,
        3 => 1,
        4 => 2,
        6 => 4,
        _ => return value_error("unsupported png color type."),
    };
    let packed = matches!(color, 0 | 3) && matches!(depth, 1 | 2 | 4);
    if depth != 8 && !packed {
        return value_error("unsupported png bit depth.");
    }
    let raw = inflate(&idat)?;
    let line = (width * channels * depth).div_ceil(8);
    if raw.len() != height * (line + 1) {
        return value_error("bad png data length.");
    }
    let bpp = (channels * depth).div_ceil(8);
    let data = unfilter(&raw, line, height, bpp)?;
    let sample = |y: usize, i: usize| {
        if depth == 8 {
            data[y * line + i]
        } else {
            let byte = data[y * line + i * depth / 8];
            byte >> (8 - depth - i * depth % 8) & ((1 << depth) - 1)
        }
    };
    let full = ((1u32 << depth) - 1) as u8;
    let mut colors = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let px = |c: usize| sample(y, x * channels + c);
            colors.push(match color {
                0 => {
                    let g = (px(0) as u32 * 255 / full as u32) as u8;
                    [g, g, g, 255]
                }
                2 => [px(0), px(1), px(2), 255],
                3 => match palette.get(px(0) as usize) {
                    Some(&entry) => entry,
                    None => return value_error("palette index out of range."),
                },
                4 => [px(0), px(0), px(0), px(1)],
                _ => [px(0), px(1), px(2), px(3)],
            });
        }
    }
    Ok((width, height, colors))
}

fn unfilter(raw: &[u8], stride: usize, height: usize, bpp: usize) -> Result<Vec<u8>> {
    let mut out = vec![0u8; stride * height];
    for row in 0..height {
        let kind = raw[row * (stride + 1)];
        let line = &raw[row * (stride + 1) + 1..(row + 1) * (stride + 1)];
        for x in 0..stride {
            let left = if x >= bpp {
                out[row * stride + x - bpp]
            } else {
                0
            };
            let up = if row > 0 {
                out[(row - 1) * stride + x]
            } else {
                0
            };
            let corner = if row > 0 && x >= bpp {
                out[(row - 1) * stride + x - bpp]
            } else {
                0
            };
            let guess = match kind {
                0 => 0,
                1 => left,
                2 => up,
                3 => ((left as u32 + up as u32) / 2) as u8,
                4 => paeth(left, up, corner),
                _ => return value_error("bad png filter."),
            };
            out[row * stride + x] = line[x].wrapping_add(guess);
        }
    }
    Ok(out)
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = a as i32 + b as i32 - c as i32;
    let pa = (p - a as i32).abs();
    let pb = (p - b as i32).abs();
    let pc = (p - c as i32).abs();
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

fn inflate(stream: &[u8]) -> Result<Vec<u8>> {
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

const CODE_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

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
                for _ in 0..3 + reader.take(2)? {
                    lengths.push(last);
                }
            }
            17 => {
                for _ in 0..3 + reader.take(3)? {
                    lengths.push(0);
                }
            }
            _ => {
                for _ in 0..11 + reader.take(7)? {
                    lengths.push(0);
                }
            }
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

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        let n = (b0 as u32) << 16 | (b1 as u32) << 8 | b2 as u32;
        out.push(BASE64_ALPHABET[(n >> 18 & 0x3f) as usize] as char);
        out.push(BASE64_ALPHABET[(n >> 12 & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 {
            BASE64_ALPHABET[(n >> 6 & 0x3f) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            BASE64_ALPHABET[(n & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn png_signature_and_scaled_size() {
        let colors = vec![
            [255, 0, 0, 255],
            [0, 255, 0, 255],
            [0, 0, 255, 255],
            [255, 255, 0, 255],
        ];
        let bytes = png(&colors, 2, 2, 4).unwrap();
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        assert_eq!(&bytes[16..20], &8u32.to_be_bytes());
        assert_eq!(&bytes[20..24], &8u32.to_be_bytes());
    }
    #[test]
    fn png_rejects_bad_inputs() {
        let colors = vec![[0, 0, 0, 255]];
        assert!(png(&colors, 1, 1, 0).is_err());
        assert!(png(&colors, 2, 2, 1).is_err());
    }
    #[test]
    fn png_has_expected_chunk_layout() {
        let bytes = png(&[[1, 2, 3, 4]], 1, 1, 1).unwrap();
        assert_eq!(&bytes[0..8], &PNG_SIGNATURE);
        assert_eq!(&bytes[8..12], &13u32.to_be_bytes());
        assert_eq!(&bytes[12..16], b"IHDR");
        assert_eq!(&bytes[16..20], &1u32.to_be_bytes());
        assert_eq!(&bytes[20..24], &1u32.to_be_bytes());
        assert_eq!(&bytes[24..29], &[8, 6, 0, 0, 0]);
        assert_eq!(
            &bytes[29..33],
            &crc32(&[b"IHDR", &bytes[16..29]]).to_be_bytes()
        );
        let idat_len = u32::from_be_bytes([bytes[33], bytes[34], bytes[35], bytes[36]]) as usize;
        assert_eq!(&bytes[37..41], b"IDAT");
        let payload = &bytes[41..41 + idat_len];
        assert_eq!(payload, &zlib(&[0, 1, 2, 3, 4])[..]);
        assert_eq!(inflate(payload).unwrap(), [0, 1, 2, 3, 4]);
        let crc_at = 41 + idat_len;
        assert_eq!(
            &bytes[crc_at..crc_at + 4],
            &crc32(&[b"IDAT", payload]).to_be_bytes()
        );
        let iend = crc_at + 4;
        assert_eq!(&bytes[iend..iend + 4], &0u32.to_be_bytes());
        assert_eq!(&bytes[iend + 4..iend + 8], b"IEND");
        assert_eq!(&bytes[iend + 8..iend + 12], &0xae42_6082u32.to_be_bytes());
        assert_eq!(bytes.len(), iend + 12);
    }
    #[test]
    fn crc32_matches_known_vectors() {
        assert_eq!(crc32(&[b"IEND"]), 0xae42_6082);
        assert_eq!(crc32(&[b""]), 0x0000_0000);
        assert_eq!(crc32(&[b"123456789"]), 0xcbf4_3926);
        assert_eq!(crc32(&[b"1234", b"56789"]), crc32(&[b"123456789"]));
    }
    #[test]
    fn adler32_matches_known_vectors() {
        assert_eq!(adler32(b"Wikipedia"), 0x11e6_0398);
        assert_eq!(adler32(b""), 0x0000_0001);
    }
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
    #[test]
    fn unpng_round_trips_the_encoder() {
        let colors = vec![
            [255, 0, 0, 255],
            [0, 255, 0, 128],
            [0, 0, 255, 0],
            [7, 8, 9, 10],
            [250, 251, 252, 253],
            [1, 1, 1, 255],
        ];
        let bytes = png(&colors, 3, 2, 1).unwrap();
        let (w, h, out) = unpng(&bytes).unwrap();
        assert_eq!((w, h), (3, 2));
        assert_eq!(out, colors);
    }
    #[test]
    fn unpng_round_trips_scaled_output() {
        let colors = vec![[9, 9, 9, 255], [0, 0, 0, 0]];
        let bytes = png(&colors, 2, 1, 3).unwrap();
        let (w, h, out) = unpng(&bytes).unwrap();
        assert_eq!((w, h), (6, 3));
        assert_eq!(out[0], [9, 9, 9, 255]);
        assert_eq!(out[5], [0, 0, 0, 0]);
        assert_eq!(out.len(), 18);
    }
    #[test]
    fn unpng_rejects_garbage() {
        assert!(unpng(&[]).is_err());
        assert!(unpng(b"not a png at all").is_err());
        let mut bytes = png(&[[1, 2, 3, 4]], 1, 1, 1).unwrap();
        bytes[25] = 16;
        assert!(unpng(&bytes).is_err());
    }
    #[test]
    fn base64_matches_rfc_4648_vectors() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
    #[test]
    fn base64_handles_binary() {
        assert_eq!(base64(&[0, 1, 2, 253, 254, 255]), "AAEC/f7/");
    }
}
