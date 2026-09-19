use crate::board::Board;
use crate::ink;
use crate::out;
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::Color;
use std::collections::HashMap;
use std::path::PathBuf;

// REEL

/// A strip of frames over one palette, printed as a gif that loops forever.
pub struct Reel {
    /// The width in pixels.
    pub width: usize,
    /// The height in pixels.
    pub height: usize,
    /// The colors every frame indexes, at most 256.
    pub palette: Vec<Color>,
    /// The frames, each the palette index of every pixel and a delay in centiseconds.
    pub frames: Vec<(Vec<u8>, u16)>,
}

impl Reel {
    /// Opens an empty reel of the given size.
    pub fn new(width: usize, height: usize) -> Reel {
        Reel {
            width,
            height,
            palette: Vec::new(),
            frames: Vec::new(),
        }
    }
    /// Indexes the board exactly against the palette, growing it, and holds it for a delay in centiseconds.
    pub fn add(&mut self, board: &Board, delay: u16) -> Result<()> {
        if board.width != self.width || board.height != self.height {
            return value_error(format!(
                "a {}x{} board does not fit a {}x{} reel.",
                board.width, board.height, self.width, self.height
            ));
        }
        let mut palette = self.palette.clone();
        let mut seen: HashMap<[u8; 3], u8> = palette
            .iter()
            .enumerate()
            .map(|(id, color)| ([color.r, color.g, color.b], id as u8))
            .collect();
        let mut indices = Vec::with_capacity(board.pixels.len());
        for pixel in &board.pixels {
            let rgb = [pixel[0], pixel[1], pixel[2]];
            let id = match seen.get(&rgb) {
                Some(&id) => id,
                None => {
                    if palette.len() == 256 {
                        return value_error("a reel holds at most 256 colors.");
                    }
                    let id = palette.len() as u8;
                    palette.push(Color::rgb(rgb[0], rgb[1], rgb[2]));
                    seen.insert(rgb, id);
                    id
                }
            };
            indices.push(id);
        }
        self.palette = palette;
        self.frames.push((indices, delay));
        Ok(())
    }
    /// Encodes the reel as a gif89a: one global color table, one graphic control per frame, looping forever.
    pub fn bytes(&self) -> Result<Vec<u8>> {
        let (width, height) = match (u16::try_from(self.width), u16::try_from(self.height)) {
            (Ok(width), Ok(height)) => (width, height),
            _ => return value_error("a reel is at most 65535 by 65535 pixels."),
        };
        if self.palette.len() > 256 {
            return value_error("a reel holds at most 256 colors.");
        }
        let slots = self.palette.len().max(2).next_power_of_two();
        let bits = slots.trailing_zeros();
        let mut out = Vec::new();
        out.extend_from_slice(b"GIF89a");
        out.extend_from_slice(&width.to_le_bytes());
        out.extend_from_slice(&height.to_le_bytes());
        out.extend_from_slice(&[0xF0 | (bits as u8 - 1), 0x00, 0x00]);
        for slot in 0..slots {
            let color = self
                .palette
                .get(slot)
                .copied()
                .unwrap_or(Color::rgb(0, 0, 0));
            out.extend_from_slice(&[color.r, color.g, color.b]);
        }
        out.extend_from_slice(&[0x21, 0xFF, 0x0B]);
        out.extend_from_slice(b"NETSCAPE2.0");
        out.extend_from_slice(&[0x03, 0x01, 0x00, 0x00, 0x00]);
        for (indices, delay) in &self.frames {
            out.extend_from_slice(&[0x21, 0xF9, 0x04, 0x00]);
            out.extend_from_slice(&delay.to_le_bytes());
            out.extend_from_slice(&[0x00, 0x00]);
            out.extend_from_slice(&[0x2C, 0x00, 0x00, 0x00, 0x00]);
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            out.push(0x00);
            let min = bits.max(2);
            out.push(min as u8);
            for block in lzw(indices, min).chunks(255) {
                out.push(block.len() as u8);
                out.extend_from_slice(block);
            }
            out.push(0x00);
        }
        out.push(0x3B);
        Ok(out)
    }
    /// Writes the reel to files/figures/<name>-<theme>.gif and announces the one line it printed.
    pub fn save(&self, name: &str) -> Result<PathBuf> {
        let name = format!("{name}-{}", ink::name());
        let folder = out::root().join("files").join("figures");
        std::fs::create_dir_all(&folder)
            .map_err(|e| MrlyError::Value(format!("cannot make {folder:?}: {e}")))?;
        let path = folder.join(format!("{name}.gif"));
        let bytes = self.bytes()?;
        std::fs::write(&path, bytes)
            .map_err(|e| MrlyError::Value(format!("cannot write {path:?}: {e}")))?;
        println!(
            "reel {name} {}x{} {} frames",
            self.width,
            self.height,
            self.frames.len()
        );
        Ok(path)
    }
}

// LZW

struct Stream {
    out: Vec<u8>,
    acc: u32,
    used: u32,
    width: u32,
}

impl Stream {
    fn emit(&mut self, code: u16, next: u16) {
        self.acc |= (code as u32) << self.used;
        self.used += self.width;
        while self.used >= 8 {
            self.out.push((self.acc & 0xFF) as u8);
            self.acc >>= 8;
            self.used -= 8;
        }
        if u32::from(next) >= 1 << self.width && self.width < 12 {
            self.width += 1;
        }
    }
    fn flush(&mut self) {
        if self.used > 0 {
            self.out.push((self.acc & 0xFF) as u8);
        }
    }
}

fn lzw(indices: &[u8], min: u32) -> Vec<u8> {
    let clear = 1u16 << min;
    let end = clear + 1;
    let mut stream = Stream {
        out: Vec::new(),
        acc: 0,
        used: 0,
        width: min + 1,
    };
    let mut table: HashMap<(u16, u8), u16> = HashMap::new();
    let mut next = clear + 2;
    stream.emit(clear, next);
    let mut prefix = match indices.first() {
        Some(&first) => u16::from(first),
        None => {
            stream.emit(end, next);
            stream.flush();
            return stream.out;
        }
    };
    for &index in &indices[1..] {
        if let Some(&code) = table.get(&(prefix, index)) {
            prefix = code;
            continue;
        }
        stream.emit(prefix, next);
        if next < 4096 {
            table.insert((prefix, index), next);
            next += 1;
        } else {
            stream.emit(clear, next);
            table.clear();
            next = clear + 2;
            stream.width = min + 1;
        }
        prefix = u16::from(index);
    }
    stream.emit(prefix, next);
    stream.emit(end, next);
    stream.flush();
    stream.out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn painted(pattern: &[u8], colors: &[Color]) -> Board {
        let mut board = Board::new(4, 4, colors[0]);
        for (at, &id) in pattern.iter().enumerate() {
            let color = colors[id as usize];
            board.pixels[at] = [color.r, color.g, color.b, color.a];
        }
        board
    }

    struct Reader<'a> {
        bytes: &'a [u8],
        at: usize,
    }

    impl Reader<'_> {
        fn byte(&mut self) -> u8 {
            self.at += 1;
            self.bytes[self.at - 1]
        }
        fn word(&mut self) -> u16 {
            u16::from_le_bytes([self.byte(), self.byte()])
        }
        fn blocks(&mut self) -> Vec<u8> {
            let mut out = Vec::new();
            loop {
                let len = self.byte() as usize;
                if len == 0 {
                    return out;
                }
                out.extend_from_slice(&self.bytes[self.at..self.at + len]);
                self.at += len;
            }
        }
    }

    fn inflate(data: &[u8], min: u32) -> Vec<u8> {
        let clear = 1usize << min;
        let roots = || {
            (0..clear + 2)
                .map(|id| vec![id as u8])
                .collect::<Vec<Vec<u8>>>()
        };
        let mut table = roots();
        let mut width = min + 1;
        let mut at = 0usize;
        let mut prev: Option<Vec<u8>> = None;
        let mut out = Vec::new();
        while at + width as usize <= data.len() * 8 {
            let mut code = 0usize;
            for bit in 0..width as usize {
                let pos = at + bit;
                code |= usize::from((data[pos / 8] >> (pos % 8)) & 1) << bit;
            }
            at += width as usize;
            if code == clear {
                table = roots();
                width = min + 1;
                prev = None;
                continue;
            }
            if code == clear + 1 {
                break;
            }
            let entry = match table.get(code) {
                Some(entry) => entry.clone(),
                None => {
                    let mut grown = prev.clone().unwrap();
                    grown.push(grown[0]);
                    grown
                }
            };
            out.extend_from_slice(&entry);
            if let Some(mut grown) = prev {
                grown.push(entry[0]);
                table.push(grown);
                if table.len() == 1 << width && width < 12 {
                    width += 1;
                }
            }
            prev = Some(entry);
        }
        out
    }

    fn decode(bytes: &[u8]) -> (Vec<Color>, Vec<(Vec<u8>, u16)>) {
        assert_eq!(&bytes[0..6], b"GIF89a");
        let mut read = Reader { bytes, at: 6 };
        let size = (read.word(), read.word());
        let packed = read.byte();
        read.byte();
        read.byte();
        let mut palette = Vec::new();
        for _ in 0..(2usize << (packed & 7)) {
            palette.push(Color::rgb(read.byte(), read.byte(), read.byte()));
        }
        let mut frames = Vec::new();
        let mut delay = 0u16;
        loop {
            match read.byte() {
                0x21 => {
                    let label = read.byte();
                    let data = read.blocks();
                    if label == 0xF9 {
                        delay = u16::from_le_bytes([data[1], data[2]]);
                    }
                }
                0x2C => {
                    assert_eq!((read.word(), read.word()), (0, 0));
                    assert_eq!((read.word(), read.word()), size);
                    assert_eq!(read.byte(), 0x00);
                    let min = u32::from(read.byte());
                    let indices = inflate(&read.blocks(), min);
                    assert_eq!(indices.len(), size.0 as usize * size.1 as usize);
                    frames.push((indices, delay));
                }
                0x3B => return (palette, frames),
                other => panic!("stray block {other:#x}"),
            }
        }
    }

    #[test]
    fn one_frame_encodes_to_the_bytes_of_the_spec() {
        let mut board = Board::new(3, 2, Color::rgb(255, 0, 0));
        board.pixels[2] = [0, 0, 255, 255];
        board.pixels[3] = [0, 0, 255, 255];
        let mut reel = Reel::new(3, 2);
        reel.add(&board, 7).unwrap();
        let mut want = Vec::new();
        want.extend_from_slice(b"GIF89a");
        want.extend_from_slice(&[0x03, 0x00, 0x02, 0x00, 0xF0, 0x00, 0x00]);
        want.extend_from_slice(&[0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF]);
        want.extend_from_slice(&[0x21, 0xFF, 0x0B]);
        want.extend_from_slice(b"NETSCAPE2.0");
        want.extend_from_slice(&[0x03, 0x01, 0x00, 0x00, 0x00]);
        want.extend_from_slice(&[0x21, 0xF9, 0x04, 0x00, 0x07, 0x00, 0x00, 0x00]);
        want.extend_from_slice(&[0x2C, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x02, 0x00, 0x00]);
        want.extend_from_slice(&[0x02, 0x03, 0x04, 0x12, 0x56, 0x00]);
        want.push(0x3B);
        assert_eq!(reel.bytes().unwrap(), want);
    }

    #[test]
    fn a_two_frame_reel_round_trips_through_a_decoder() {
        let colors = [
            Color::rgb(9, 9, 9),
            Color::rgb(200, 30, 40),
            Color::rgb(0, 80, 255),
        ];
        let first: Vec<u8> = (0..16u8).map(|at| at % 3).collect();
        let second: Vec<u8> = (0..16u8).map(|at| (at / 2) % 3).collect();
        let mut reel = Reel::new(4, 4);
        reel.add(&painted(&first, &colors), 4).unwrap();
        reel.add(&painted(&second, &colors), 11).unwrap();
        let (palette, frames) = decode(&reel.bytes().unwrap());
        assert_eq!(&palette[..3], &colors[..]);
        assert_eq!(frames, vec![(first, 4), (second, 11)]);
    }

    #[test]
    fn the_palette_refuses_a_two_hundred_and_fifty_seventh_color() {
        let mut board = Board::new(257, 1, Color::rgb(0, 0, 0));
        for at in 0..257 {
            board.pixels[at] = [(at / 256) as u8, (at % 256) as u8, 0, 255];
        }
        let mut reel = Reel::new(257, 1);
        assert!(reel.add(&board, 1).is_err());
        assert!(reel.palette.is_empty());
    }
}
