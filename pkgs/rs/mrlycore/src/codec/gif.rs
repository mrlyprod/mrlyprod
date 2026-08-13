use crate::errors::{value_error, Result};
use crate::resample::block;
use std::collections::HashMap;

/// Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block.
///
/// The frames index one shared palette, the delay is in hundredths of a second, and the
/// first fully transparent palette entry becomes the frame's transparent color.
///
/// ```
/// let still = [0u8, 1, 1, 0];
/// let flip = [1u8, 0, 0, 1];
/// let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
/// let bytes = mrlycore::gif(&[&still[..], &flip[..]], &palette, 2, 2, 3, 8).unwrap();
/// assert_eq!(&bytes[0..6], b"GIF89a");
/// assert_eq!(bytes[bytes.len() - 1], 0x3b);
/// ```
pub fn gif(
    frames: &[&[u8]],
    palette: &[[u8; 4]],
    width: usize,
    height: usize,
    scale: usize,
    delay: usize,
) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    if width == 0 || height == 0 {
        return value_error("width and height must be at least 1.");
    }
    if frames.is_empty() {
        return value_error("gif needs at least one frame.");
    }
    if palette.is_empty() || palette.len() > 256 {
        return value_error("palette must hold 1 to 256 colors.");
    }
    let (out_w, out_h) = (width * scale, height * scale);
    if out_w > u16::MAX as usize || out_h > u16::MAX as usize {
        return value_error("gif size must fit in 16 bits.");
    }
    for frame in frames {
        if frame.len() != width * height {
            return value_error("every frame must hold width * height indices.");
        }
        if frame.iter().any(|&i| i as usize >= palette.len()) {
            return value_error("frame index out of palette range.");
        }
    }
    let bits = palette.len().next_power_of_two().trailing_zeros().max(1);
    let clear = 1usize << bits.max(2);
    let mut out = Vec::with_capacity(frames.len() * out_w * out_h / 2 + 1024);
    out.extend_from_slice(b"GIF89a");
    out.extend_from_slice(&(out_w as u16).to_le_bytes());
    out.extend_from_slice(&(out_h as u16).to_le_bytes());
    out.push(0xf0 | (bits - 1) as u8);
    out.extend_from_slice(&[0, 0]);
    for slot in 0..1usize << bits {
        let color = palette.get(slot).copied().unwrap_or([0, 0, 0, 0]);
        out.extend_from_slice(&color[..3]);
    }
    out.extend_from_slice(&[0x21, 0xff, 0x0b]);
    out.extend_from_slice(b"NETSCAPE2.0");
    out.extend_from_slice(&[0x03, 0x01, 0, 0, 0]);
    let clear_index = palette.iter().position(|c| c[3] == 0);
    for frame in frames {
        out.extend_from_slice(&[0x21, 0xf9, 0x04]);
        match clear_index {
            Some(_) => out.push(0x09),
            None => out.push(0x04),
        }
        out.extend_from_slice(&(delay.min(u16::MAX as usize) as u16).to_le_bytes());
        out.push(clear_index.unwrap_or(0) as u8);
        out.push(0);
        out.push(0x2c);
        out.extend_from_slice(&[0, 0, 0, 0]);
        out.extend_from_slice(&(out_w as u16).to_le_bytes());
        out.extend_from_slice(&(out_h as u16).to_le_bytes());
        out.push(0);
        out.push(clear.trailing_zeros() as u8);
        out.extend_from_slice(&lzw(&block(frame, width, height, scale), clear));
    }
    out.push(0x3b);
    Ok(out)
}

fn lzw(indices: &[u8], clear: usize) -> Vec<u8> {
    let root = clear.trailing_zeros();
    let (clear, end) = (clear as u16, clear as u16 + 1);
    let mut table: HashMap<(u16, u8), u16> = HashMap::new();
    let mut next = end + 1;
    let mut width = root + 1;
    let mut blocks = Blocks::new();
    blocks.code(clear, width);
    let mut prefix: Option<u16> = None;
    for &index in indices {
        let held = match prefix {
            None => {
                prefix = Some(u16::from(index));
                continue;
            }
            Some(held) => held,
        };
        if let Some(&code) = table.get(&(held, index)) {
            prefix = Some(code);
            continue;
        }
        blocks.code(held, width);
        if next < 4096 {
            if next == 1 << width && width < 12 {
                width += 1;
            }
            table.insert((held, index), next);
            next += 1;
        } else {
            blocks.code(clear, width);
            table.clear();
            next = end + 1;
            width = root + 1;
        }
        prefix = Some(u16::from(index));
    }
    if let Some(held) = prefix {
        blocks.code(held, width);
    }
    blocks.code(end, width);
    blocks.finish()
}

struct Blocks {
    out: Vec<u8>,
    run: Vec<u8>,
    acc: u32,
    filled: u32,
}

impl Blocks {
    fn new() -> Blocks {
        Blocks {
            out: Vec::new(),
            run: Vec::with_capacity(255),
            acc: 0,
            filled: 0,
        }
    }
    fn code(&mut self, code: u16, width: u32) {
        self.acc |= u32::from(code) << self.filled;
        self.filled += width;
        while self.filled >= 8 {
            let byte = self.acc as u8;
            self.byte(byte);
            self.acc >>= 8;
            self.filled -= 8;
        }
    }
    fn byte(&mut self, value: u8) {
        self.run.push(value);
        if self.run.len() == 255 {
            self.pack();
        }
    }
    fn pack(&mut self) {
        if !self.run.is_empty() {
            self.out.push(self.run.len() as u8);
            self.out.append(&mut self.run);
        }
    }
    fn finish(mut self) -> Vec<u8> {
        if self.filled > 0 {
            let byte = self.acc as u8;
            self.byte(byte);
        }
        self.pack();
        self.out.push(0);
        self.out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sub_blocks(bytes: &[u8], at: &mut usize) -> Vec<u8> {
        let mut data = Vec::new();
        loop {
            let len = bytes[*at] as usize;
            *at += 1;
            if len == 0 {
                return data;
            }
            data.extend_from_slice(&bytes[*at..*at + len]);
            *at += len;
        }
    }
    fn unlzw(data: &[u8], min_code: u32) -> Vec<u8> {
        let clear = 1u16 << min_code;
        let end = clear + 1;
        let fresh = |table: &mut Vec<Vec<u8>>| {
            table.truncate(clear as usize);
            table.push(Vec::new());
            table.push(Vec::new());
        };
        let mut table: Vec<Vec<u8>> = (0..clear).map(|i| vec![i as u8]).collect();
        fresh(&mut table);
        let mut width = min_code + 1;
        let mut out = Vec::new();
        let mut held: Option<u16> = None;
        let mut bit = 0usize;
        while bit + width as usize <= data.len() * 8 {
            let mut code = 0u16;
            for k in 0..width as usize {
                let taken = data[(bit + k) / 8] >> ((bit + k) % 8) & 1;
                code |= u16::from(taken) << k;
            }
            bit += width as usize;
            if code == clear {
                fresh(&mut table);
                width = min_code + 1;
                held = None;
                continue;
            }
            if code == end {
                break;
            }
            let entry = match table.get(code as usize) {
                Some(entry) if !entry.is_empty() => entry.clone(),
                _ => {
                    let mut grown = table[held.unwrap() as usize].clone();
                    grown.push(grown[0]);
                    grown
                }
            };
            out.extend_from_slice(&entry);
            if let Some(prefix) = held {
                let mut grown = table[prefix as usize].clone();
                grown.push(entry[0]);
                if table.len() < 4096 {
                    table.push(grown);
                }
            }
            held = Some(code);
            if table.len() == 1 << width && width < 12 {
                width += 1;
            }
        }
        out
    }
    fn ungif(bytes: &[u8]) -> (usize, usize, Vec<[u8; 3]>, Vec<Vec<u8>>) {
        assert_eq!(&bytes[0..6], b"GIF89a");
        let width = u16::from_le_bytes([bytes[6], bytes[7]]) as usize;
        let height = u16::from_le_bytes([bytes[8], bytes[9]]) as usize;
        let packed = bytes[10];
        let slots = 1usize << ((packed & 7) + 1);
        let mut at = 13;
        let palette: Vec<[u8; 3]> = (0..slots)
            .map(|i| {
                let base = at + i * 3;
                [bytes[base], bytes[base + 1], bytes[base + 2]]
            })
            .collect();
        at += slots * 3;
        let mut frames = Vec::new();
        while at < bytes.len() {
            match bytes[at] {
                0x21 => {
                    at += 2;
                    sub_blocks(bytes, &mut at);
                }
                0x2c => {
                    at += 10;
                    let min_code = u32::from(bytes[at]);
                    at += 1;
                    let data = sub_blocks(bytes, &mut at);
                    frames.push(unlzw(&data, min_code));
                }
                _ => break,
            }
        }
        assert_eq!(bytes[bytes.len() - 1], 0x3b);
        (width, height, palette, frames)
    }
    #[test]
    fn gif_round_trips_indexed_frames() {
        let first = [0u8, 1, 2, 1, 2, 0, 1, 0, 1, 1, 0, 2];
        let second = [2u8, 2, 0, 0, 1, 2, 1, 2, 0, 1, 1, 0];
        let palette = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 0]];
        let bytes = gif(&[&first[..], &second[..]], &palette, 4, 3, 1, 5).unwrap();
        let (w, h, table, frames) = ungif(&bytes);
        assert_eq!((w, h), (4, 3));
        assert_eq!(table.len(), 4);
        assert_eq!(&table[..3], &[[255, 0, 0], [0, 255, 0], [0, 0, 255]]);
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0], first);
        assert_eq!(frames[1], second);
    }
    #[test]
    fn gif_scales_every_pixel_into_a_block() {
        let frame = [0u8, 1, 1, 0];
        let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
        let bytes = gif(&[&frame[..]], &palette, 2, 2, 3, 0).unwrap();
        let (w, h, _, frames) = ungif(&bytes);
        assert_eq!((w, h), (6, 6));
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].len(), 36);
        for y in 0..6 {
            for x in 0..6 {
                let want = frame[(y / 3) * 2 + x / 3];
                assert_eq!(frames[0][y * 6 + x], want, "pixel {x},{y}");
            }
        }
    }
    #[test]
    fn gif_lays_out_the_animation_blocks() {
        let frame = [0u8, 1];
        let palette = [[1, 2, 3, 255], [4, 5, 6, 0]];
        let bytes = gif(&[&frame[..], &frame[..]], &palette, 2, 1, 1, 7).unwrap();
        assert_eq!(bytes[10], 0xf0);
        assert_eq!(&bytes[13..19], &[1, 2, 3, 4, 5, 6]);
        assert_eq!(&bytes[19..22], &[0x21, 0xff, 0x0b]);
        assert_eq!(&bytes[22..33], b"NETSCAPE2.0");
        assert_eq!(&bytes[33..38], &[0x03, 0x01, 0, 0, 0]);
        assert_eq!(&bytes[38..41], &[0x21, 0xf9, 0x04]);
        assert_eq!(bytes[41], 0x09);
        assert_eq!(&bytes[42..44], &7u16.to_le_bytes());
        assert_eq!(bytes[44], 1);
        assert_eq!(bytes[46], 0x2c);
        let opaque = gif(&[&frame[..]], &[[1, 2, 3, 255], [4, 5, 6, 255]], 2, 1, 1, 7).unwrap();
        assert_eq!(opaque[41], 0x04);
    }
    #[test]
    fn gif_survives_a_full_dictionary() {
        let mut random = crate::chacha::ChaCha8::from_u64(11);
        let palette: Vec<[u8; 4]> = (0..256).map(|i| [i as u8, 9, 9, 255]).collect();
        let frame: Vec<u8> = (0..128 * 128).map(|_| random.next_u32() as u8).collect();
        let bytes = gif(&[&frame[..]], &palette, 128, 128, 1, 4).unwrap();
        let (w, h, table, frames) = ungif(&bytes);
        assert_eq!((w, h), (128, 128));
        assert_eq!(table.len(), 256);
        assert_eq!(frames[0], frame);
    }
    #[test]
    fn gif_compresses_flat_frames() {
        let frame = vec![3u8; 256 * 256];
        let palette: Vec<[u8; 4]> = (0..8).map(|i| [i as u8 * 30, 0, 0, 255]).collect();
        let bytes = gif(&[&frame[..]], &palette, 256, 256, 1, 4).unwrap();
        assert!(bytes.len() < 2000, "flat frame took {} bytes", bytes.len());
        assert_eq!(ungif(&bytes).3[0], frame);
    }
    #[test]
    fn gif_rejects_bad_inputs() {
        let frame = [0u8, 1, 1, 0];
        let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
        assert!(gif(&[&frame[..]], &palette, 2, 2, 0, 5).is_err());
        assert!(gif(&[], &palette, 2, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &[], 2, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &palette, 3, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &palette, 2, 0, 1, 5).is_err());
        assert!(gif(&[&[0u8, 1, 2, 0][..]], &palette, 2, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &palette, 2, 2, 40000, 5).is_err());
    }
}
