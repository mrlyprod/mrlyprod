use crate::{EXTENSIBLE, FLOAT, PCM, Result, Wav};

struct Fmt {
    tag: u16,
    channels: usize,
    rate: usize,
    block: usize,
    bits: usize,
}

fn u16_at(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn fmt(body: &[u8]) -> Result<Fmt> {
    if body.len() < 16 {
        return Err(format!(
            "the fmt chunk holds {} bytes, under 16.",
            body.len()
        ));
    }
    let mut tag = u16_at(body, 0);
    if tag == EXTENSIBLE {
        if body.len() < 40 {
            return Err("an extensible fmt chunk holds 40 bytes.".into());
        }
        tag = u16_at(body, 24);
    }
    let found = Fmt {
        tag,
        channels: u16_at(body, 2) as usize,
        rate: u32_at(body, 4) as usize,
        block: u16_at(body, 12) as usize,
        bits: u16_at(body, 14) as usize,
    };
    let known = match found.tag {
        PCM => matches!(found.bits, 8 | 16 | 24 | 32),
        FLOAT => matches!(found.bits, 32 | 64),
        _ => {
            return Err(format!(
                "format tag {} is neither pcm nor float.",
                found.tag
            ));
        }
    };
    if !known {
        return Err(format!("{} bits a sample is not supported.", found.bits));
    }
    if found.channels == 0 || found.rate == 0 {
        return Err("a wav file needs at least one channel and a positive rate.".into());
    }
    if found.block != found.channels * found.bits / 8 {
        return Err(format!(
            "block align {} does not match {} channels of {} bits.",
            found.block, found.channels, found.bits
        ));
    }
    Ok(found)
}

fn chunks(bytes: &[u8]) -> Result<(Fmt, &[u8])> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("not a riff wave file.".into());
    }
    let mut at = 12;
    let mut found: Option<Fmt> = None;
    while at + 8 <= bytes.len() {
        let name = &bytes[at..at + 4];
        let size = u32_at(bytes, at + 4) as usize;
        let start = at + 8;
        let rest = bytes.len() - start;
        if name == b"data" {
            let Some(found) = found else {
                return Err("the data chunk comes before any fmt chunk.".into());
            };
            let take = size.min(rest);
            let whole = take - take % found.block;
            return Ok((found, &bytes[start..start + whole]));
        }
        if size > rest {
            return Err(format!(
                "chunk {} claims {size} bytes with {rest} left.",
                String::from_utf8_lossy(name)
            ));
        }
        if name == b"fmt " {
            found = Some(fmt(&bytes[start..start + size])?);
        }
        at = start + size + size % 2;
    }
    Err(match found {
        Some(_) => "no data chunk.".into(),
        None => "no fmt chunk.".into(),
    })
}

fn sample(found: &Fmt, raw: &[u8]) -> f32 {
    match (found.tag, found.bits) {
        (FLOAT, 32) => f32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]),
        (FLOAT, _) => f64::from_le_bytes([
            raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
        ]) as f32,
        (_, 8) => (raw[0] as f32 - 128.0) / 128.0,
        (_, 16) => i16::from_le_bytes([raw[0], raw[1]]) as f32 / 32768.0,
        (_, 24) => (i32::from_le_bytes([0, raw[0], raw[1], raw[2]]) >> 8) as f32 / 8_388_608.0,
        _ => (i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) as f64 / 2_147_483_648.0) as f32,
    }
}

pub fn decode(bytes: &[u8]) -> Result<Wav> {
    let (found, data) = chunks(bytes)?;
    let width = found.bits / 8;
    Ok(Wav {
        rate: found.rate,
        channels: found.channels,
        bits: found.bits,
        float: found.tag == FLOAT,
        samples: data
            .chunks_exact(width)
            .map(|raw| sample(&found, raw))
            .collect(),
    })
}
