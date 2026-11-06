use crate::Result;
use crate::deflate::pack;
use crate::inflate::expand;
use crate::sum::adler32;

pub fn compress(bytes: &[u8], level: u8) -> Vec<u8> {
    let flags = match level {
        0 | 1 => 0x01,
        2..=5 => 0x5E,
        6 => 0x9C,
        _ => 0xDA,
    };
    let mut out = vec![0x78, flags];
    out.extend_from_slice(&pack(bytes, level));
    out.extend_from_slice(&adler32(bytes).to_be_bytes());
    out
}

pub fn decompress(bytes: &[u8], limit: usize) -> Result<Vec<u8>> {
    let [method, flags, body @ ..] = bytes else {
        return Err("a zlib stream starts with two header bytes.".into());
    };
    if method & 15 != 8 || method >> 4 > 7 {
        return Err(format!("zlib method byte {method:#04x} is not deflate."));
    }
    if !(*method as u32 * 256 + *flags as u32).is_multiple_of(31) {
        return Err("the zlib header fails its check.".into());
    }
    if flags & 32 != 0 {
        return Err("a zlib preset dictionary is not supported.".into());
    }
    let (out, used) = expand(body, limit)?;
    let Some(sum) = body.get(used..used + 4) else {
        return Err("the zlib stream ends before its checksum.".into());
    };
    if sum != adler32(&out).to_be_bytes() {
        return Err("the zlib checksum does not match the data.".into());
    }
    Ok(out)
}
