use crate::Result;
use crate::sum::crc32;
use crate::zlib::decompress;

pub(crate) const MAGIC: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const LIMIT: usize = 1 << 26;

const START: [(usize, usize, usize, usize); 7] = [
    (0, 0, 8, 8),
    (4, 0, 8, 8),
    (0, 4, 4, 8),
    (2, 0, 4, 4),
    (0, 2, 2, 4),
    (1, 0, 2, 2),
    (0, 1, 1, 2),
];

struct Head {
    width: usize,
    height: usize,
    depth: u8,
    kind: u8,
    laced: bool,
}

struct Pass {
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
    width: usize,
    height: usize,
}

struct Parts {
    head: Head,
    table: Vec<[u8; 4]>,
    key: Vec<u8>,
    raw: Vec<u8>,
}

pub(crate) fn bits(kind: u8, depth: u8) -> usize {
    let channels = match kind {
        0 | 3 => 1,
        4 => 2,
        2 => 3,
        _ => 4,
    };
    channels * depth as usize
}

pub(crate) fn stride(width: usize, bits: usize) -> usize {
    (width as u64 * bits as u64).div_ceil(8) as usize
}

impl Head {
    fn bits(&self) -> usize {
        bits(self.kind, self.depth)
    }
    fn stride(&self, width: usize) -> usize {
        stride(width, self.bits())
    }
    fn passes(&self) -> Vec<Pass> {
        let whole = [(0, 0, 1, 1)];
        let list: &[(usize, usize, usize, usize)] = if self.laced { &START } else { &whole };
        list.iter()
            .map(|&(x, y, dx, dy)| Pass {
                x,
                y,
                dx,
                dy,
                width: (self.width + dx - 1 - x) / dx,
                height: (self.height + dy - 1 - y) / dy,
            })
            .filter(|pass| pass.width > 0 && pass.height > 0)
            .collect()
    }
}

fn word(bytes: &[u8]) -> usize {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize
}

fn label(name: &[u8]) -> String {
    String::from_utf8_lossy(name).into_owned()
}

fn chunk(bytes: &[u8], at: usize) -> Result<(&[u8], &[u8], usize)> {
    let Some(head) = bytes.get(at..at + 8) else {
        return Err("the file ends before its IEND chunk.".into());
    };
    let (len, name) = (word(head), &head[4..]);
    if !name.iter().all(u8::is_ascii_alphabetic) {
        return Err("a chunk name is not four letters.".into());
    }
    let end = at + 8 + len.min(bytes.len() - at);
    let (Some(body), Some(sum)) = (bytes.get(at + 8..end), bytes.get(end..end + 4)) else {
        return Err(format!("the file ends inside chunk {}.", label(name)));
    };
    if crc32(&bytes[at + 4..end]) != word(sum) as u32 {
        return Err(format!("chunk {} fails its checksum.", label(name)));
    }
    Ok((name, body, end + 4))
}

fn walk(bytes: &[u8]) -> Result<Vec<(&[u8], &[u8])>> {
    if !bytes.starts_with(&MAGIC) {
        return Err("not a png file.".into());
    }
    let mut out = Vec::new();
    let mut at = MAGIC.len();
    loop {
        let (name, body, next) = chunk(bytes, at)?;
        out.push((name, body));
        if name == b"IEND" {
            return Ok(out);
        }
        at = next;
    }
}

fn head(name: &[u8], body: &[u8]) -> Result<Head> {
    if name != b"IHDR" {
        return Err(format!("the first chunk is {}, not IHDR.", label(name)));
    }
    let &[_, _, _, _, _, _, _, _, depth, kind, pack, filter, lace] = body else {
        return Err(format!(
            "the IHDR chunk holds {} bytes, not 13.",
            body.len()
        ));
    };
    let (width, height) = (word(body), word(&body[4..]));
    let limit = i32::MAX as usize;
    if width == 0 || height == 0 || width > limit || height > limit {
        return Err(format!("a size of {width} x {height} is not allowed."));
    }
    let fits = match kind {
        0 => matches!(depth, 1 | 2 | 4 | 8 | 16),
        3 => matches!(depth, 1 | 2 | 4 | 8),
        2 | 4 | 6 => matches!(depth, 8 | 16),
        _ => return Err(format!("colour type {kind} is not one of 0, 2, 3, 4, 6.")),
    };
    if !fits {
        return Err(format!(
            "colour type {kind} cannot hold {depth} bits a sample."
        ));
    }
    if pack != 0 || filter != 0 || lace > 1 {
        return Err(format!(
            "compression {pack}, filter {filter}, interlace {lace} is not a known method."
        ));
    }
    Ok(Head {
        width,
        height,
        depth,
        kind,
        laced: lace == 1,
    })
}

fn open(bytes: &[u8]) -> Result<Parts> {
    let list = walk(bytes)?;
    let head = head(list[0].0, list[0].1)?;
    if head
        .width
        .checked_mul(head.height)
        .is_none_or(|n| n > LIMIT)
    {
        return Err(format!(
            "{} x {} pixels is over the limit of {LIMIT}.",
            head.width, head.height
        ));
    }
    let (mut palette, mut key): (&[u8], &[u8]) = (&[], &[]);
    let mut data = Vec::new();
    for &(name, body) in &list[1..] {
        match name {
            b"IHDR" => return Err("a second IHDR chunk.".into()),
            b"PLTE" if body.is_empty() || body.len() % 3 != 0 || body.len() > 768 => {
                return Err(format!("the PLTE chunk holds {} bytes.", body.len()));
            }
            b"PLTE" => palette = body,
            b"tRNS" if data.is_empty() => key = body,
            b"IDAT" => data.extend_from_slice(body),
            _ if name[0] & 32 == 0 && name != b"IEND" => {
                return Err(format!("chunk {} is critical and unknown.", label(name)));
            }
            _ => {}
        }
    }
    if data.is_empty() {
        return Err("the file holds no image data.".into());
    }
    let wanted = match head.kind {
        0 => 2,
        2 => 6,
        _ => key.len(),
    };
    if !key.is_empty() && key.len() != wanted {
        return Err(format!(
            "the tRNS chunk holds {} bytes, not {wanted}.",
            key.len()
        ));
    }
    let mut table = Vec::new();
    if head.kind == 3 {
        if palette.is_empty() {
            return Err("a palette image has no PLTE chunk.".into());
        }
        for (index, color) in palette.chunks_exact(3).enumerate() {
            let alpha = key.get(index).copied().unwrap_or(255);
            table.push([color[0], color[1], color[2], alpha]);
        }
    }
    let size: usize = head
        .passes()
        .iter()
        .map(|pass| pass.height * (1 + head.stride(pass.width)))
        .sum();
    let raw = decompress(&data, size)?;
    if raw.len() != size {
        return Err(format!(
            "the image data holds {} bytes, not the {size} its size needs.",
            raw.len()
        ));
    }
    let low = ((1u16 << head.depth.min(8)) - 1) as u8;
    let key = match head.depth {
        16 => key.to_vec(),
        _ => key.chunks_exact(2).map(|pair| pair[1] & low).collect(),
    };
    Ok(Parts {
        head,
        table,
        key,
        raw,
    })
}

pub(crate) fn paeth(left: u8, up: u8, corner: u8) -> u8 {
    let (a, b, c) = (left as i16, up as i16, corner as i16);
    let (pa, pb, pc) = ((b - c).abs(), (a - c).abs(), (a + b - 2 * c).abs());
    if pa <= pb && pa <= pc {
        left
    } else if pb <= pc {
        up
    } else {
        corner
    }
}

fn undo<const N: usize>(kind: u8, prev: &[u8], row: &mut [u8]) {
    let (mut left, mut corner) = ([0u8; N], [0u8; N]);
    for (cell, up) in row.chunks_exact_mut(N).zip(prev.chunks_exact(N)) {
        for at in 0..N {
            let guess = match kind {
                1 => left[at],
                3 => ((left[at] as u16 + up[at] as u16) / 2) as u8,
                _ => paeth(left[at], up[at], corner[at]),
            };
            left[at] = cell[at].wrapping_add(guess);
        }
        cell.copy_from_slice(&left);
        corner.copy_from_slice(up);
    }
}

fn unfilter(kind: u8, step: usize, prev: &[u8], row: &mut [u8]) -> Result<()> {
    match (kind, step) {
        (0, _) => {}
        (2, _) => {
            for (value, up) in row.iter_mut().zip(prev) {
                *value = value.wrapping_add(*up);
            }
        }
        (1 | 3 | 4, 1) => undo::<1>(kind, prev, row),
        (1 | 3 | 4, 2) => undo::<2>(kind, prev, row),
        (1 | 3 | 4, 3) => undo::<3>(kind, prev, row),
        (1 | 3 | 4, 4) => undo::<4>(kind, prev, row),
        (1 | 3 | 4, 6) => undo::<6>(kind, prev, row),
        (1 | 3 | 4, _) => undo::<8>(kind, prev, row),
        _ => return Err(format!("row filter {kind} is not one of 0 to 4.")),
    }
    Ok(())
}

fn narrow(high: u8, low: u8) -> u8 {
    ((u16::from_be_bytes([high, low]) as u32 * 255 + 32895) >> 16) as u8
}

fn sample(row: &[u8], x: usize, depth: usize) -> usize {
    let bit = x * depth;
    (row[bit / 8] >> (8 - depth - bit % 8)) as usize & ((1 << depth) - 1)
}

fn expand(row: &[u8], parts: &Parts, out: &mut [[u8; 4]]) -> Result<()> {
    let (key, depth) = (parts.key.as_slice(), parts.head.depth as usize);
    let veil = |hit: bool| if hit { 0 } else { 255 };
    match (parts.head.kind, depth) {
        (6, 8) => {
            for (color, p) in out.iter_mut().zip(row.chunks_exact(4)) {
                *color = [p[0], p[1], p[2], p[3]];
            }
        }
        (6, _) => {
            for (color, p) in out.iter_mut().zip(row.chunks_exact(8)) {
                *color = [
                    narrow(p[0], p[1]),
                    narrow(p[2], p[3]),
                    narrow(p[4], p[5]),
                    narrow(p[6], p[7]),
                ];
            }
        }
        (4, 8) => {
            for (color, p) in out.iter_mut().zip(row.chunks_exact(2)) {
                *color = [p[0], p[0], p[0], p[1]];
            }
        }
        (4, _) => {
            for (color, p) in out.iter_mut().zip(row.chunks_exact(4)) {
                let gray = narrow(p[0], p[1]);
                *color = [gray, gray, gray, narrow(p[2], p[3])];
            }
        }
        (2, 8) => {
            for (color, p) in out.iter_mut().zip(row.chunks_exact(3)) {
                *color = [p[0], p[1], p[2], veil(key == p)];
            }
        }
        (2, _) => {
            for (color, p) in out.iter_mut().zip(row.chunks_exact(6)) {
                let rgb = [narrow(p[0], p[1]), narrow(p[2], p[3]), narrow(p[4], p[5])];
                *color = [rgb[0], rgb[1], rgb[2], veil(key == p)];
            }
        }
        (0, 16) => {
            for (color, p) in out.iter_mut().zip(row.chunks_exact(2)) {
                let gray = narrow(p[0], p[1]);
                *color = [gray, gray, gray, veil(key == p)];
            }
        }
        (0, _) => {
            let unit = 255 / ((1 << depth) - 1);
            for (x, color) in out.iter_mut().enumerate() {
                let value = sample(row, x, depth);
                let gray = (value * unit) as u8;
                *color = [gray, gray, gray, veil(key == [value as u8])];
            }
        }
        _ => {
            for (x, color) in out.iter_mut().enumerate() {
                let index = sample(row, x, depth);
                let Some(found) = parts.table.get(index) else {
                    return Err(format!(
                        "palette index {index} is past the {} palette colours.",
                        parts.table.len()
                    ));
                };
                *color = *found;
            }
        }
    }
    Ok(())
}

pub fn decode(bytes: &[u8]) -> Result<(usize, usize, Vec<[u8; 4]>)> {
    let mut parts = open(bytes)?;
    let (width, height) = (parts.head.width, parts.head.height);
    let step = parts.head.bits().div_ceil(8);
    let mut raw = std::mem::take(&mut parts.raw);
    let zeros = vec![0u8; parts.head.stride(width)];
    let mut colors = vec![[0u8; 4]; width * height];
    let mut line = Vec::new();
    let mut at = 0;
    for pass in parts.head.passes() {
        let stride = parts.head.stride(pass.width);
        for y in 0..pass.height {
            let (before, rest) = raw.split_at_mut(at + 1);
            let row = &mut rest[..stride];
            let prev = match y {
                0 => &zeros[..stride],
                _ => &before[at - stride..at],
            };
            unfilter(before[at], step, prev, row)?;
            at += 1 + stride;
            let top = (pass.y + y * pass.dy) * width;
            if !parts.head.laced {
                expand(row, &parts, &mut colors[top..top + width])?;
                continue;
            }
            line.resize(pass.width, [0; 4]);
            expand(row, &parts, &mut line)?;
            for (x, &color) in line.iter().enumerate() {
                colors[top + pass.x + x * pass.dx] = color;
            }
        }
    }
    Ok((width, height, colors))
}

pub fn header(bytes: &[u8]) -> Result<(usize, usize, u8, u8, bool)> {
    if !bytes.starts_with(&MAGIC) {
        return Err("not a png file.".into());
    }
    let (name, body, _) = chunk(bytes, MAGIC.len())?;
    let head = head(name, body)?;
    Ok((head.width, head.height, head.depth, head.kind, head.laced))
}
