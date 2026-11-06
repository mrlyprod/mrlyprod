use crate::map::{self, Keep, Near, plain};
use crate::quant::{exact, median, opaque};
use crate::write::{Frame, assemble};
use crate::{BACKGROUND, KEEP, Result, lzw};

type Rect = (usize, usize, usize, usize);

struct Shot<'a> {
    pixels: &'a [u8],
    delay: u16,
}

fn shape(width: usize, height: usize, frames: usize, delays: usize) -> Result<()> {
    if !(1..=65535).contains(&width) || !(1..=65535).contains(&height) {
        return Err(format!(
            "a {width} by {height} gif is outside 1 to 65535 pixels a side."
        ));
    }
    if frames == 0 {
        return Err("a gif needs at least one frame.".into());
    }
    if delays != 1 && delays != frames {
        return Err(format!(
            "{delays} delays for {frames} frames: give one for each frame or one for all."
        ));
    }
    Ok(())
}

fn merge<'a>(frames: &'a [Vec<u8>], delays: &[u16]) -> Vec<Shot<'a>> {
    let mut shots: Vec<Shot> = Vec::with_capacity(frames.len());
    for (i, frame) in frames.iter().enumerate() {
        let delay = delays[if delays.len() == 1 { 0 } else { i }];
        if let Some(last) = shots.last_mut()
            && last.pixels == frame.as_slice()
            && let Some(sum) = last.delay.checked_add(delay)
        {
            last.delay = sum;
            continue;
        }
        shots.push(Shot {
            pixels: frame,
            delay,
        });
    }
    shots
}

fn union(a: Option<Rect>, b: Option<Rect>) -> Option<Rect> {
    match (a, b) {
        (Some((ax, ay, aw, ah)), Some((bx, by, bw, bh))) => {
            let (left, top) = (ax.min(bx), ay.min(by));
            let (right, bottom) = ((ax + aw).max(bx + bw), (ay + ah).max(by + bh));
            Some((left, top, right - left, bottom - top))
        }
        (one, None) | (None, one) => one,
    }
}

fn bounds(height: usize, mut differs: impl FnMut(usize) -> Option<(usize, usize)>) -> Option<Rect> {
    let mut found: Option<Rect> = None;
    for y in 0..height {
        if let Some((first, last)) = differs(y) {
            found = union(found, Some((first, y, last + 1 - first, 1)));
        }
    }
    found
}

fn changes(before: &[u8], after: &[u8], width: usize, height: usize) -> Option<Rect> {
    bounds(height, |y| {
        let (a, b) = (
            &before[y * width..(y + 1) * width],
            &after[y * width..(y + 1) * width],
        );
        if a == b {
            return None;
        }
        let first = a.iter().zip(b).position(|(p, q)| p != q)?;
        let last = width
            - 1
            - a.iter()
                .rev()
                .zip(b.iter().rev())
                .position(|(p, q)| p != q)?;
        Some((first, last))
    })
}

fn clears(now: &[u8], next: &[u8], width: usize, height: usize, clear: u8) -> Option<Rect> {
    bounds(height, |y| {
        let row = y * width..(y + 1) * width;
        let wiped = |x: &usize| now[row.start + x] != clear && next[row.start + x] == clear;
        let first = (0..width).find(wiped)?;
        let last = (0..width).rev().find(wiped)?;
        Some((first, last))
    })
}

fn optimise(
    width: usize,
    height: usize,
    mut palette: Vec<[u8; 3]>,
    clear: Option<u8>,
    frames: &[Vec<u8>],
    delays: &[u16],
) -> Vec<u8> {
    let shots = merge(frames, delays);
    let moving = shots.len() > 1;
    let spare = clear.or((moving && palette.len() < 256).then_some(palette.len() as u8));
    let mut canvas = vec![0; width * height];
    let mut sent = Vec::with_capacity(shots.len());
    let mut spent = false;
    for (i, shot) in shots.iter().enumerate() {
        let wipe = match (clear, shots.get(i + 1)) {
            (Some(c), Some(next)) => clears(shot.pixels, next.pixels, width, height, c),
            _ => None,
        };
        let rect = match i {
            0 => Some((0, 0, width, height)),
            _ => union(changes(&canvas, shot.pixels, width, height), wipe),
        };
        let (left, top, w, h) = rect.unwrap_or((0, 0, 1, 1));
        let mut whole = Vec::with_capacity(w * h);
        let mut holes = Vec::new();
        let hollow = spare.filter(|_| i > 0);
        for y in top..top + h {
            let row = y * width + left..y * width + left + w;
            whole.extend_from_slice(&shot.pixels[row.clone()]);
            if let Some(s) = hollow {
                let pairs = shot.pixels[row.clone()].iter().zip(&canvas[row]);
                holes.extend(pairs.map(|(&now, &was)| if now == was { s } else { now }));
            }
        }
        let mut transparent = clear.filter(|c| whole.contains(c));
        let (mut min_size, mut data) = lzw::pack(&whole);
        if let Some(s) = hollow {
            let tried = lzw::pack(&holes);
            if tried.1.len() < data.len() {
                spent |= Some(s) != clear;
                (transparent, (min_size, data)) = (Some(s), tried);
            }
        }
        sent.push(Frame {
            left,
            top,
            width: w,
            height: h,
            delay: if moving { shot.delay } else { 0 },
            dispose: match (moving, wipe) {
                (false, _) => 0,
                (true, None) => KEEP,
                (true, Some(_)) => BACKGROUND,
            },
            transparent,
            min_size,
            data,
        });
        canvas.copy_from_slice(shot.pixels);
        if let (Some(c), Some(_)) = (clear, wipe) {
            for y in top..top + h {
                canvas[y * width + left..y * width + left + w].fill(c);
            }
        }
    }
    if spent {
        palette.push([0, 0, 0]);
    }
    assemble(width, height, &palette, &sent)
}

pub fn encode(
    frames: &[Vec<u8>],
    width: usize,
    height: usize,
    palette: &[[u8; 4]],
    delays: &[u16],
) -> Result<Vec<u8>> {
    shape(width, height, frames.len(), delays.len())?;
    if palette.is_empty() || palette.len() > 256 {
        return Err(format!(
            "a palette holds 1 to 256 colors, not {}.",
            palette.len()
        ));
    }
    let mut clears = (0..palette.len()).filter(|&i| !opaque(palette[i]));
    let clear = clears.next().map(|i| i as u8);
    if clears.next().is_some() {
        return Err("a gif palette holds at most one clear color.".into());
    }
    for (i, frame) in frames.iter().enumerate() {
        if frame.len() != width * height {
            return Err(format!(
                "frame {i} holds {} indices for a {width} by {height} gif.",
                frame.len()
            ));
        }
        if let Some(&top) = frame.iter().max()
            && top as usize >= palette.len()
        {
            return Err(format!(
                "frame {i} uses index {top} with {} colors.",
                palette.len()
            ));
        }
    }
    Ok(optimise(
        width,
        height,
        palette.iter().map(|c| [c[0], c[1], c[2]]).collect(),
        clear,
        frames,
        delays,
    ))
}

pub fn encode_rgba(
    frames: &[Vec<[u8; 4]>],
    width: usize,
    height: usize,
    delays: &[u16],
    dither: bool,
) -> Result<Vec<u8>> {
    shape(width, height, frames.len(), delays.len())?;
    for (i, frame) in frames.iter().enumerate() {
        if frame.len() != width * height {
            return Err(format!(
                "frame {i} holds {} colors for a {width} by {height} gif.",
                frame.len()
            ));
        }
    }
    let views: Vec<&[[u8; 4]]> = frames.iter().map(Vec::as_slice).collect();
    let clear = frames.iter().any(|frame| frame.iter().any(|&p| !opaque(p)));
    let moving = frames.windows(2).any(|pair| pair[0] != pair[1]);
    let (palette, indexed) = match exact(&views, clear, 256 - clear as usize) {
        Some(found) => found,
        None => {
            let colors = median(&views, if clear || moving { 255 } else { 256 });
            let near = Near::new(&colors);
            let first = clear as u8;
            let mut indexed: Vec<Vec<u8>> = Vec::with_capacity(frames.len());
            for (i, frame) in frames.iter().enumerate() {
                let next = if dither {
                    let keep = i.checked_sub(1).map(|j| Keep {
                        source: &frames[j],
                        indices: &indexed[j],
                    });
                    map::dither(frame, width, &near, first, clear, keep)
                } else {
                    plain(frame, &near, first, clear)
                };
                indexed.push(next);
            }
            let mut palette = vec![[0, 0, 0]; first as usize];
            palette.extend(colors);
            (palette, indexed)
        }
    };
    Ok(optimise(
        width,
        height,
        palette,
        clear.then_some(0),
        &indexed,
        delays,
    ))
}
