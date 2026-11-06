use mrly_error::{Result, value_error};
use std::fmt::Write as _;

pub const CLEAR: char = '.';

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub palette: Vec<(char, [u8; 3])>,
    pub frames: Vec<Vec<u8>>,
}

impl Grid {
    fn ink(&self, kind: u8) -> Option<(char, [u8; 3])> {
        let at = usize::from(kind.checked_sub(1)?);
        self.palette.get(at).copied()
    }
    pub fn colors(&self, frame: usize) -> Vec<[u8; 4]> {
        let Some(frame) = self.frames.get(frame) else {
            return Vec::new();
        };
        frame
            .iter()
            .map(|&kind| match self.ink(kind) {
                Some((_, [r, g, b])) => [r, g, b, 255],
                None => [0, 0, 0, 0],
            })
            .collect()
    }
    pub fn unused(&self) -> Vec<char> {
        let mut seen = vec![false; self.palette.len() + 1];
        for &kind in self.frames.iter().flatten() {
            if let Some(mark) = seen.get_mut(usize::from(kind)) {
                *mark = true;
            }
        }
        self.palette
            .iter()
            .zip(&seen[1..])
            .filter(|(_, used)| !**used)
            .map(|(&(key, _), _)| key)
            .collect()
    }
}

fn color(word: &str) -> Option<[u8; 3]> {
    if word.len() != 6 || !word.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let value = u32::from_str_radix(word, 16).ok()?;
    Some([(value >> 16) as u8, (value >> 8) as u8, value as u8])
}

fn palette(lines: &mut std::str::Lines) -> Result<Vec<(char, [u8; 3])>> {
    let mut out: Vec<(char, [u8; 3])> = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut glyphs = line.chars();
        let (Some(key), Some(' ')) = (glyphs.next(), glyphs.next()) else {
            return value_error(format!("palette line '{line}' must read like 'a 1b1e2b'."));
        };
        let Some(rgb) = color(glyphs.as_str()) else {
            return value_error(format!("palette line '{line}' must read like 'a 1b1e2b'."));
        };
        if key == CLEAR || key.is_whitespace() {
            return value_error(format!("'{key}' cannot name a color."));
        }
        if let Some(&(twin, _)) = out.iter().find(|&&(k, c)| k == key || c == rgb) {
            return value_error(format!("palette line '{line}' repeats '{twin}'."));
        }
        out.push((key, rgb));
    }
    if out.is_empty() || out.len() > 255 {
        return value_error(format!(
            "a palette holds 1 to 255 colors, not {}.",
            out.len()
        ));
    }
    Ok(out)
}

fn frame(rows: &[&str], palette: &[(char, [u8; 3])], width: usize, at: usize) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(rows.len() * width);
    for (y, row) in rows.iter().enumerate() {
        let y = y + 1;
        let wide = row.chars().count();
        if wide != width {
            return value_error(format!("frame {at} row {y} is {wide} wide, not {width}."));
        }
        for glyph in row.chars() {
            if glyph == CLEAR {
                out.push(0);
                continue;
            }
            match palette.iter().position(|&(key, _)| key == glyph) {
                Some(found) => out.push(found as u8 + 1),
                None => {
                    return value_error(format!(
                        "frame {at} row {y} uses '{glyph}', which is not in the palette."
                    ));
                }
            }
        }
    }
    Ok(out)
}

pub fn parse(text: &str) -> Result<Grid> {
    let mut lines = text.lines();
    let palette = palette(&mut lines)?;
    let mut frames: Vec<Vec<u8>> = Vec::new();
    let mut rows: Vec<&str> = Vec::new();
    let (mut width, mut height) = (0, 0);
    for line in lines.chain(std::iter::once("")) {
        if !line.is_empty() {
            rows.push(line);
            continue;
        }
        if rows.is_empty() {
            continue;
        }
        if frames.is_empty() {
            (width, height) = (rows[0].chars().count(), rows.len());
        }
        if rows.len() != height {
            return value_error(format!(
                "frame {} is {} tall, not {height}.",
                frames.len() + 1,
                rows.len()
            ));
        }
        frames.push(frame(&rows, &palette, width, frames.len() + 1)?);
        rows.clear();
    }
    if frames.is_empty() {
        return value_error("a grid needs rows after the palette and a blank line.");
    }
    Ok(Grid {
        width,
        height,
        palette,
        frames,
    })
}

pub fn print(grid: &Grid) -> String {
    let mut out = String::new();
    for &(key, [r, g, b]) in &grid.palette {
        let _ = writeln!(out, "{key} {r:02x}{g:02x}{b:02x}");
    }
    for frame in &grid.frames {
        out.push('\n');
        for row in frame.chunks(grid.width.max(1)) {
            out.extend(
                row.iter()
                    .map(|&kind| grid.ink(kind).map_or(CLEAR, |(key, _)| key)),
            );
            out.push('\n');
        }
    }
    out
}
