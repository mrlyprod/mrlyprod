use crate::{EXTENSIBLE, FLOAT, PCM, Result};

const GUID_TAIL: [u8; 14] = [
    0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71,
];

struct Layout {
    tag: u16,
    channels: usize,
    rate: usize,
    bits: usize,
    frames: usize,
}

fn mask(channels: usize) -> u32 {
    match channels {
        1 => 0x4,
        2 => 0x3,
        3 => 0x7,
        4 => 0x33,
        5 => 0x37,
        6 => 0x3F,
        7 => 0x13F,
        8 => 0x63F,
        _ => 0,
    }
}

fn header(layout: &Layout) -> Result<Vec<u8>> {
    let Layout {
        tag,
        channels,
        rate,
        bits,
        frames,
    } = *layout;
    if channels == 0 || channels > u16::MAX as usize {
        return Err(format!("channels run from 1 to 65535, not {channels}."));
    }
    if rate == 0 || rate > u32::MAX as usize {
        return Err(format!(
            "rate must fit 32 bits and be positive, not {rate}."
        ));
    }
    let block = channels * bits / 8;
    if block > u16::MAX as usize {
        return Err(format!(
            "{channels} channels of {bits} bits pass 65535 bytes a frame."
        ));
    }
    let wide = tag == FLOAT || channels > 2 || bits > 16;
    let data = frames
        .checked_mul(block)
        .filter(|&size| size <= u32::MAX as usize - 128)
        .ok_or("a wav file holds under 4 GiB of samples.")?;
    let pad = data % 2;
    let fmt = if wide { 40 } else { 16 };
    let fact = if tag == FLOAT { 12 } else { 0 };
    let riff = 4 + 8 + fmt + fact + 8 + data + pad;
    let mut out = Vec::with_capacity(8 + riff);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(riff as u32).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&(fmt as u32).to_le_bytes());
    out.extend_from_slice(&(if wide { EXTENSIBLE } else { tag }).to_le_bytes());
    out.extend_from_slice(&(channels as u16).to_le_bytes());
    out.extend_from_slice(&(rate as u32).to_le_bytes());
    let bytes = (rate as u64 * block as u64).min(u32::MAX as u64) as u32;
    out.extend_from_slice(&bytes.to_le_bytes());
    out.extend_from_slice(&(block as u16).to_le_bytes());
    out.extend_from_slice(&(bits as u16).to_le_bytes());
    if wide {
        out.extend_from_slice(&22u16.to_le_bytes());
        out.extend_from_slice(&(bits as u16).to_le_bytes());
        out.extend_from_slice(&mask(channels).to_le_bytes());
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&GUID_TAIL);
    }
    if tag == FLOAT {
        out.extend_from_slice(b"fact");
        out.extend_from_slice(&4u32.to_le_bytes());
        out.extend_from_slice(&(frames as u32).to_le_bytes());
    }
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data as u32).to_le_bytes());
    Ok(out)
}

fn frames(len: usize, channels: usize) -> Result<usize> {
    if channels == 0 {
        return Err("channels run from 1 to 65535, not 0.".into());
    }
    if !len.is_multiple_of(channels) {
        return Err(format!(
            "{len} samples do not fill whole frames of {channels} channels."
        ));
    }
    Ok(len / channels)
}

fn finish(mut out: Vec<u8>) -> Vec<u8> {
    if out.len() % 2 == 1 {
        out.push(0);
    }
    out
}

fn whole(sample: f32, bits: usize) -> i64 {
    let top = (1i64 << (bits - 1)) - 1;
    let scaled = (sample as f64 * (top + 1) as f64).round();
    if scaled.is_nan() {
        return 0;
    }
    (scaled as i64).clamp(-top - 1, top)
}

pub fn encode(samples: &[f32], channels: usize, rate: usize, bits: usize) -> Result<Vec<u8>> {
    if !matches!(bits, 8 | 16 | 24 | 32) {
        return Err(format!("pcm bits are 8, 16, 24 or 32, not {bits}."));
    }
    let mut out = header(&Layout {
        tag: PCM,
        channels,
        rate,
        bits,
        frames: frames(samples.len(), channels)?,
    })?;
    out.reserve(samples.len() * bits / 8 + 1);
    for &sample in samples {
        let value = whole(sample, bits);
        match bits {
            8 => out.push((value + 128) as u8),
            16 => out.extend_from_slice(&(value as i16).to_le_bytes()),
            24 => out.extend_from_slice(&(value as i32).to_le_bytes()[..3]),
            _ => out.extend_from_slice(&(value as i32).to_le_bytes()),
        }
    }
    Ok(finish(out))
}

pub fn encode_float(samples: &[f32], channels: usize, rate: usize) -> Result<Vec<u8>> {
    let mut out = header(&Layout {
        tag: FLOAT,
        channels,
        rate,
        bits: 32,
        frames: frames(samples.len(), channels)?,
    })?;
    out.reserve(samples.len() * 4);
    for &sample in samples {
        out.extend_from_slice(&sample.to_le_bytes());
    }
    Ok(out)
}

fn next(state: &mut u64) -> f32 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state >> 40) as f32 / (1u64 << 24) as f32
}

pub fn dither(samples: &[f32], bits: usize, seed: u64) -> Result<Vec<f32>> {
    if !(2..=32).contains(&bits) {
        return Err(format!("dither bits run from 2 to 32, not {bits}."));
    }
    let step = 1.0 / (1u64 << (bits - 1)) as f32;
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    Ok(samples
        .iter()
        .map(|&sample| sample + (next(&mut state) - next(&mut state)) * step)
        .collect())
}
