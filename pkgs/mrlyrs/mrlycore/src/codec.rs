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
        assert_eq!(inflate(payload), [0, 1, 2, 3, 4]);
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
            assert_eq!(&inflate(&zlib(data)), data);
        }
    }
    #[test]
    fn zlib_compresses_flat_data() {
        let data = vec![9u8; 100_000];
        let stream = zlib(&data);
        assert!(stream.len() < 1000);
        assert_eq!(inflate(&stream), data);
    }
    fn inflate(stream: &[u8]) -> Vec<u8> {
        assert_eq!(&stream[..2], &[0x78, 0x01]);
        let mut reader = BitReader {
            data: &stream[2..stream.len() - 4],
            byte: 0,
            bit: 0,
        };
        assert_eq!(reader.take(1), 1);
        assert_eq!(reader.take(2), 1);
        let mut out = Vec::new();
        loop {
            let symbol = read_symbol(&mut reader);
            if symbol == 256 {
                break;
            }
            if symbol < 256 {
                out.push(symbol as u8);
                continue;
            }
            let s = (symbol - 257) as usize;
            let len = (LENGTH_BASE[s] + reader.take(LENGTH_EXTRA[s])) as usize;
            let mut code = 0;
            for _ in 0..5 {
                code = code << 1 | reader.take(1);
            }
            let d = code as usize;
            let dist = (DIST_BASE[d] + reader.take(DIST_EXTRA[d])) as usize;
            let from = out.len() - dist;
            for k in 0..len {
                out.push(out[from + k]);
            }
        }
        assert_eq!(&stream[stream.len() - 4..], &adler32(&out).to_be_bytes());
        out
    }
    fn read_symbol(reader: &mut BitReader) -> u32 {
        let mut code = 0;
        for _ in 0..7 {
            code = code << 1 | reader.take(1);
        }
        if code <= 0x17 {
            return 256 + code;
        }
        code = code << 1 | reader.take(1);
        if (0x30..=0xbf).contains(&code) {
            return code - 0x30;
        }
        if (0xc0..=0xc7).contains(&code) {
            return 280 + code - 0xc0;
        }
        code = code << 1 | reader.take(1);
        144 + code - 0x190
    }
    struct BitReader<'a> {
        data: &'a [u8],
        byte: usize,
        bit: u32,
    }
    impl BitReader<'_> {
        fn take(&mut self, count: u32) -> u32 {
            let mut value = 0;
            for i in 0..count {
                let bit = u32::from(self.data[self.byte]) >> self.bit & 1;
                value |= bit << i;
                self.bit += 1;
                if self.bit == 8 {
                    self.bit = 0;
                    self.byte += 1;
                }
            }
            value
        }
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
