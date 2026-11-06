use crate::Result;
use crate::read::{MAGIC, bits, paeth, stride};
use crate::sum::crc32;
use crate::zlib::compress;
use std::cmp::Reverse;

pub const DEFAULT: u8 = 8;
pub const BEST: u8 = 9;

const QUICK: u8 = 1;
const THOROUGH: u8 = 7;
const ADAPT: u8 = 5;
const SAMPLE: usize = 1 << 16;
const BANDS: usize = 4;

struct Seen {
    keys: Vec<[u8; 4]>,
    hits: Vec<usize>,
    slots: Vec<u16>,
}

impl Seen {
    fn place(&self, color: [u8; 4]) -> usize {
        let mut at = (u32::from_le_bytes(color).wrapping_mul(0x9E37_79B1) >> 22) as usize;
        while self.slots[at] != 0 && self.keys[self.slots[at] as usize - 1] != color {
            at = (at + 1) % self.slots.len();
        }
        at
    }
}

struct Form {
    kind: u8,
    depth: u8,
    table: Vec<[u8; 4]>,
    rank: Vec<u8>,
}

impl Form {
    fn bits(&self) -> usize {
        bits(self.kind, self.depth)
    }
    fn stride(&self, width: usize) -> usize {
        stride(width, self.bits())
    }
    fn veils(&self) -> Vec<u8> {
        let last = self.table.iter().rposition(|color| color[3] < 255);
        let count = last.map_or(0, |at| at + 1);
        self.table[..count].iter().map(|color| color[3]).collect()
    }
    fn extra(&self) -> usize {
        let veils = self.veils().len();
        match self.table.len() {
            0 => 0,
            count => 12 + 3 * count + if veils > 0 { 12 + veils } else { 0 },
        }
    }
}

fn depth(count: usize) -> u8 {
    match count {
        0..=2 => 1,
        3..=4 => 2,
        5..=16 => 4,
        _ => 8,
    }
}

fn check(count: usize, width: usize, height: usize, what: &str) -> Result<()> {
    let limit = i32::MAX as usize;
    if width == 0 || height == 0 || width > limit || height > limit {
        return Err(format!("a size of {width} x {height} is not allowed."));
    }
    if Some(count) != width.checked_mul(height) {
        return Err(format!("{what} length must equal width * height."));
    }
    Ok(())
}

fn survey(colors: &[[u8; 4]]) -> (Option<Seen>, bool, bool) {
    let mut seen = Some(Seen {
        keys: Vec::new(),
        hits: Vec::new(),
        slots: vec![0; 1024],
    });
    let (mut gray, mut opaque) = (true, true);
    let (mut last, mut index) = (None, 0);
    for &color in colors {
        gray &= color[0] == color[1] && color[1] == color[2];
        opaque &= color[3] == 255;
        let Some(set) = &mut seen else {
            continue;
        };
        if last != Some(color) {
            let at = set.place(color);
            if set.slots[at] == 0 {
                if set.keys.len() == 256 {
                    seen = None;
                    continue;
                }
                set.keys.push(color);
                set.hits.push(0);
                set.slots[at] = set.keys.len() as u16;
            }
            (last, index) = (Some(color), set.slots[at] as usize - 1);
        }
        set.hits[index] += 1;
    }
    (seen, gray, opaque)
}

fn forms(seen: Option<&Seen>, gray: bool, opaque: bool) -> Vec<Form> {
    let kind = match (gray, opaque) {
        (true, true) => 0,
        (true, false) => 4,
        (false, true) => 2,
        (false, false) => 6,
    };
    let mut direct = Form {
        kind,
        depth: 8,
        table: Vec::new(),
        rank: Vec::new(),
    };
    let Some(seen) = seen else {
        return vec![direct];
    };
    if kind == 0 {
        let fits = |depth: &u8| {
            let unit = 255 / ((1u16 << depth) - 1) as u8;
            seen.keys.iter().all(|color| color[0] % unit == 0)
        };
        direct.depth = [1, 2, 4].into_iter().find(fits).unwrap_or(8);
    }
    let count = seen.keys.len();
    let depth = depth(count);
    if kind == 0 && direct.depth <= depth {
        return vec![direct];
    }
    let mut order: Vec<usize> = (0..count).collect();
    order.sort_by_key(|&at| {
        (
            seen.keys[at][3] == 255,
            Reverse(seen.hits[at]),
            seen.keys[at],
        )
    });
    let mut rank = vec![0u8; count];
    for (place, &at) in order.iter().enumerate() {
        rank[at] = place as u8;
    }
    let table = order.iter().map(|&at| seen.keys[at]).collect();
    let palette = Form {
        kind: 3,
        depth,
        table,
        rank,
    };
    vec![direct, palette]
}

fn squeeze<T: Copy>(
    items: &[T],
    width: usize,
    depth: u8,
    mut value: impl FnMut(T) -> u8,
) -> Vec<u8> {
    let (depth, stride) = (depth as usize, stride(width, depth as usize));
    let mut out = vec![0u8; stride * (items.len() / width)];
    for (row, line) in out.chunks_exact_mut(stride).zip(items.chunks_exact(width)) {
        for (x, &item) in line.iter().enumerate() {
            let bit = x * depth;
            row[bit / 8] |= value(item) << (8 - depth - bit % 8);
        }
    }
    out
}

fn rows(colors: &[[u8; 4]], width: usize, form: &Form, seen: Option<&Seen>) -> Vec<u8> {
    match form.kind {
        6 => colors.as_flattened().to_vec(),
        2 => {
            let mut out = vec![0u8; colors.len() * 3];
            for (cell, color) in out.chunks_exact_mut(3).zip(colors) {
                cell.copy_from_slice(&color[..3]);
            }
            out
        }
        4 => {
            let mut out = vec![0u8; colors.len() * 2];
            for (cell, color) in out.chunks_exact_mut(2).zip(colors) {
                (cell[0], cell[1]) = (color[0], color[3]);
            }
            out
        }
        _ => {
            let unit = 255 / ((1u16 << form.depth) - 1) as u8;
            let pick = |color: [u8; 4]| match seen {
                Some(seen) if form.kind == 3 => {
                    form.rank[seen.slots[seen.place(color)] as usize - 1]
                }
                _ => color[0] / unit,
            };
            let (mut last, mut value) = (colors[0], pick(colors[0]));
            squeeze(colors, width, form.depth, |color| {
                if color != last {
                    (last, value) = (color, pick(color));
                }
                value
            })
        }
    }
}

fn filter(kind: u8, step: usize, prev: &[u8], row: &[u8], out: &mut [u8]) {
    let (head, tail) = out.split_at_mut(step);
    let first = head.iter_mut().zip(row.iter().zip(prev));
    let rest = tail
        .iter_mut()
        .zip(row[step..].iter().zip(row))
        .zip(prev[step..].iter().zip(prev));
    match kind {
        0 => out.copy_from_slice(row),
        1 => {
            head.copy_from_slice(&row[..step]);
            for ((cell, (value, left)), _) in rest {
                *cell = value.wrapping_sub(*left);
            }
        }
        2 => {
            for (cell, (value, up)) in out.iter_mut().zip(row.iter().zip(prev)) {
                *cell = value.wrapping_sub(*up);
            }
        }
        3 => {
            for (cell, (value, up)) in first {
                *cell = value.wrapping_sub(up / 2);
            }
            for ((cell, (value, left)), (up, _)) in rest {
                *cell = value.wrapping_sub(((*left as u16 + *up as u16) / 2) as u8);
            }
        }
        _ => {
            for (cell, (value, up)) in first {
                *cell = value.wrapping_sub(*up);
            }
            for ((cell, (value, left)), (up, corner)) in rest {
                *cell = value.wrapping_sub(paeth(*left, *up, *corner));
            }
        }
    }
}

fn filtered(raw: &[u8], stride: usize, step: usize, mode: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len() + raw.len() / stride);
    let zeros = vec![0u8; stride];
    let (mut best, mut trial) = (zeros.clone(), zeros.clone());
    let mut prev = zeros.as_slice();
    for row in raw.chunks_exact(stride) {
        let mut kind = mode;
        if mode == ADAPT {
            let mut low = u64::MAX;
            for candidate in 0..ADAPT {
                filter(candidate, step, prev, row, &mut trial);
                let weight = |&value: &u8| (value as i8).unsigned_abs() as u64;
                let score: u64 = trial.iter().map(weight).sum();
                if score < low {
                    (low, kind) = (score, candidate);
                    std::mem::swap(&mut best, &mut trial);
                }
            }
        } else {
            filter(mode, step, prev, row, &mut best);
        }
        out.push(kind);
        out.extend_from_slice(&best);
        prev = row;
    }
    out
}

fn sample(raw: &[u8], stride: usize) -> Vec<u8> {
    let rows = raw.len() / stride;
    let band = (SAMPLE / BANDS / stride).max(4);
    if rows <= band * BANDS {
        return raw.to_vec();
    }
    let mut out = Vec::with_capacity(band * BANDS * stride);
    for at in 0..BANDS {
        let top = (rows - band) * at / (BANDS - 1);
        out.extend_from_slice(&raw[top * stride..(top + band) * stride]);
    }
    out
}

fn chunk(out: &mut Vec<u8>, name: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(name);
    out.extend_from_slice(body);
    let sum = crc32(&out[start..]);
    out.extend_from_slice(&sum.to_be_bytes());
}

fn file(form: &Form, width: usize, height: usize, raw: &[u8], mode: u8, level: u8) -> Vec<u8> {
    let data = filtered(raw, form.stride(width), form.bits().div_ceil(8), mode);
    let packed = compress(&data, level);
    let mut out = Vec::with_capacity(packed.len() + form.extra() + 57);
    out.extend_from_slice(&MAGIC);
    let mut head = [0u8; 13];
    head[..4].copy_from_slice(&(width as u32).to_be_bytes());
    head[4..8].copy_from_slice(&(height as u32).to_be_bytes());
    (head[8], head[9]) = (form.depth, form.kind);
    chunk(&mut out, b"IHDR", &head);
    if !form.table.is_empty() {
        let rgb: Vec<u8> = form.table.iter().flat_map(|c| [c[0], c[1], c[2]]).collect();
        chunk(&mut out, b"PLTE", &rgb);
        let veils = form.veils();
        if !veils.is_empty() {
            chunk(&mut out, b"tRNS", &veils);
        }
    }
    for part in packed.chunks(1 << 30) {
        chunk(&mut out, b"IDAT", part);
    }
    chunk(&mut out, b"IEND", &[]);
    out
}

pub fn encode(colors: &[[u8; 4]], width: usize, height: usize) -> Result<Vec<u8>> {
    encode_level(colors, width, height, DEFAULT)
}

pub fn encode_level(colors: &[[u8; 4]], width: usize, height: usize, level: u8) -> Result<Vec<u8>> {
    check(colors.len(), width, height, "colors")?;
    let (seen, gray, opaque) = survey(colors);
    let mut forms = forms(seen.as_ref(), gray, opaque);
    let cost = |form: &Form| form.stride(width) * height + form.extra();
    forms.sort_by_key(cost);
    let thorough = level >= THOROUGH;
    let mut tries = Vec::new();
    for (index, form) in forms.iter().enumerate() {
        if index > 0 && (!thorough || cost(form) > 2 * cost(&forms[0])) {
            break;
        }
        let modes: &[u8] = match (form.kind != 3 && form.depth == 8, thorough) {
            (false, _) => &[0],
            (true, false) => &[ADAPT],
            (true, true) => &[0, 1, 2, 3, 4, ADAPT],
        };
        tries.extend(modes.iter().map(|&mode| (index, mode)));
    }
    let mut best = (usize::MAX, tries[0].0, tries[0].1);
    let mut held = (usize::MAX, Vec::new());
    let mut probe = Vec::new();
    for &(index, mode) in tries.iter().filter(|_| tries.len() > 1) {
        let (form, stride) = (&forms[index], forms[index].stride(width));
        if held.0 != index {
            held = (index, rows(colors, width, form, seen.as_ref()));
            probe = sample(&held.1, stride);
        }
        let data = filtered(&probe, stride, form.bits().div_ceil(8), mode);
        let part = compress(&data, QUICK).len() as u64 * height as u64;
        let score = (part / (probe.len() / stride) as u64) as usize + form.extra();
        if score < best.0 {
            best = (score, index, mode);
        }
    }
    let form = &forms[best.1];
    if held.0 != best.1 {
        held.1 = rows(colors, width, form, seen.as_ref());
    }
    Ok(file(form, width, height, &held.1, best.2, level))
}

pub fn encode_indexed(
    indices: &[u8],
    width: usize,
    height: usize,
    palette: &[[u8; 4]],
) -> Result<Vec<u8>> {
    check(indices.len(), width, height, "indices")?;
    let count = palette.len();
    if count == 0 || count > 256 {
        return Err(format!("a palette holds 1 to 256 colours, not {count}."));
    }
    if let Some(at) = indices.iter().position(|&index| index as usize >= count) {
        return Err(format!(
            "index {} at pixel {at} is past the {count} palette colours.",
            indices[at]
        ));
    }
    let form = Form {
        kind: 3,
        depth: depth(count),
        table: palette.to_vec(),
        rank: Vec::new(),
    };
    let raw = squeeze(indices, width, form.depth, |index| index);
    Ok(file(&form, width, height, &raw, 0, DEFAULT))
}
