use crate::{BACKGROUND, LIMIT, Movie, PREVIOUS, Result, lzw};

struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8]> {
        let end = self.at + n;
        let Some(out) = self.bytes.get(self.at..end) else {
            return Err(format!("the file ends inside {what}."));
        };
        self.at = end;
        Ok(out)
    }
    fn byte(&mut self, what: &str) -> Result<u8> {
        Ok(self.take(1, what)?[0])
    }
    fn word(&mut self, what: &str) -> Result<usize> {
        let pair = self.take(2, what)?;
        Ok(u16::from_le_bytes([pair[0], pair[1]]) as usize)
    }
    fn blocks(&mut self, what: &str, mut each: impl FnMut(&'a [u8])) -> Result<()> {
        loop {
            let n = self.byte(what)? as usize;
            if n == 0 {
                return Ok(());
            }
            each(self.take(n, what)?);
        }
    }
    fn table(&mut self, flags: u8, what: &str) -> Result<&'a [u8]> {
        if flags & 0x80 == 0 {
            return Ok(&[]);
        }
        self.take(3 << ((flags & 7) + 1), what)
    }
}

struct Frame {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    colors: [[u8; 4]; 256],
    indices: Vec<u8>,
}

type Control = (u16, u8, Option<u8>);

fn extension(at: &mut Cursor, control: &mut Option<Control>) -> Result<()> {
    let what = "an extension";
    let label = at.byte(what)?;
    let mut first: Option<&[u8]> = None;
    at.blocks(what, |block| {
        first.get_or_insert(block);
    })?;
    match (label, first) {
        (0xF9, Some(body)) if body.len() >= 4 => {
            let delay = u16::from_le_bytes([body[1], body[2]]);
            let transparent = (body[0] & 1 == 1).then_some(body[3]);
            *control = Some((delay, body[0] >> 2 & 7, transparent));
        }
        (0x01, _) => *control = None,
        _ => {}
    }
    Ok(())
}

fn image(at: &mut Cursor, global: &[u8], transparent: Option<u8>, what: &str) -> Result<Frame> {
    let left = at.word(what)?;
    let top = at.word(what)?;
    let width = at.word(what)?;
    let height = at.word(what)?;
    let flags = at.byte(what)?;
    let area = width * height;
    if area > LIMIT {
        return Err(format!(
            "{what} holds {area} pixels, over the limit of {LIMIT}."
        ));
    }
    let local = at.table(flags, what)?;
    let palette = if local.is_empty() { global } else { local };
    if palette.is_empty() {
        return Err(format!("{what} has no color table."));
    }
    let mut colors = [[0, 0, 0, 255]; 256];
    for (slot, color) in colors.iter_mut().zip(palette.chunks_exact(3)) {
        slot[..3].copy_from_slice(color);
    }
    if let Some(index) = transparent {
        colors[index as usize] = [0; 4];
    }
    let min_size = at.byte(what)?;
    let mut data = Vec::new();
    at.blocks(what, |block| data.extend_from_slice(block))?;
    let mut indices = lzw::unpack(&data, min_size, area).map_err(|e| format!("{what}: {e}"))?;
    if indices.len() < area {
        return Err(format!(
            "{what} holds {} of its {area} pixels.",
            indices.len()
        ));
    }
    if flags & 0x40 != 0 {
        let rows = [(0, 8), (4, 8), (2, 4), (1, 2)]
            .into_iter()
            .flat_map(|(start, step)| (start..height).step_by(step));
        let mut sorted = vec![0; area];
        for (stored, y) in rows.enumerate() {
            sorted[y * width..(y + 1) * width]
                .copy_from_slice(&indices[stored * width..(stored + 1) * width]);
        }
        indices = sorted;
    }
    Ok(Frame {
        left,
        top,
        width,
        height,
        colors,
        indices,
    })
}

fn spans(frame: &Frame, width: usize, height: usize) -> (usize, usize) {
    let across = frame.width.min(width.saturating_sub(frame.left));
    let down = frame.height.min(height.saturating_sub(frame.top));
    if across == 0 { (0, 0) } else { (across, down) }
}

fn clear(canvas: &mut [[u8; 4]], width: usize, height: usize, frame: &Frame) {
    let (across, down) = spans(frame, width, height);
    for y in frame.top..frame.top + down {
        let at = y * width + frame.left;
        canvas[at..at + across].fill([0; 4]);
    }
}

fn draw(canvas: &mut [[u8; 4]], width: usize, height: usize, frame: &Frame) {
    let (across, down) = spans(frame, width, height);
    for row in 0..down {
        let source = &frame.indices[row * frame.width..row * frame.width + across];
        let at = (frame.top + row) * width + frame.left;
        for (pixel, &index) in canvas[at..at + across].iter_mut().zip(source) {
            let color = frame.colors[index as usize];
            if color[3] != 0 {
                *pixel = color;
            }
        }
    }
}

pub fn decode(bytes: &[u8]) -> Result<Movie> {
    if bytes.len() < 6 || &bytes[..3] != b"GIF" {
        return Err("not a gif file.".into());
    }
    let mut at = Cursor { bytes, at: 6 };
    let what = "the header";
    let width = at.word(what)?;
    let height = at.word(what)?;
    let flags = at.byte(what)?;
    at.take(2, what)?;
    let global = at.table(flags, "the color table")?;
    let area = width * height;
    let mut canvas = Vec::new();
    let mut frames = Vec::new();
    let mut delays = Vec::new();
    let mut control = None;
    while let Some(&kind) = bytes.get(at.at) {
        at.at += 1;
        match kind {
            0x3B => break,
            0x21 => extension(&mut at, &mut control)?,
            0x2C => {
                let what = format!("frame {}", frames.len());
                if area.saturating_mul(frames.len() + 2) > LIMIT {
                    return Err(format!(
                        "{what} takes a {width} by {height} canvas over the limit of {LIMIT} pixels."
                    ));
                }
                let (delay, dispose, transparent) = control.take().unwrap_or((0, 0, None));
                let frame = image(&mut at, global, transparent, &what)?;
                canvas.resize(area, [0; 4]);
                if dispose == PREVIOUS {
                    let mut shown = canvas.clone();
                    draw(&mut shown, width, height, &frame);
                    frames.push(shown);
                } else {
                    draw(&mut canvas, width, height, &frame);
                    frames.push(canvas.clone());
                }
                if dispose == BACKGROUND {
                    clear(&mut canvas, width, height, &frame);
                }
                delays.push(delay);
            }
            other => {
                return Err(format!(
                    "unknown block 0x{other:02x} at byte {}.",
                    at.at - 1
                ));
            }
        }
    }
    if frames.is_empty() {
        return Err("the file holds no frames.".into());
    }
    Ok((width, height, frames, delays))
}
