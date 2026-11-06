use crate::quant::{EMPTY, key, opaque};

pub(crate) struct Near {
    colors: Vec<[i32; 3]>,
    starts: Vec<u32>,
    list: Vec<u8>,
}

impl Near {
    pub(crate) fn new(palette: &[[u8; 3]]) -> Near {
        let colors: Vec<[i32; 3]> = palette.iter().map(|c| c.map(|v| v as i32)).collect();
        let reach = |v: i32, low: i32| {
            let close = (low - v).max(v - low - 15).max(0);
            let far = (v - low).abs().max((v - low - 15).abs());
            (close * close, far * far)
        };
        let spans: Vec<[[(i32, i32); 16]; 3]> = colors
            .iter()
            .map(|c| c.map(|v| std::array::from_fn(|k| reach(v, 16 * k as i32))))
            .collect();
        let mut starts = Vec::with_capacity(4097);
        let mut list = Vec::new();
        for cell in 0..4096 {
            let [r, g, b] = [cell >> 8, cell >> 4 & 15, cell & 15];
            let bound = spans
                .iter()
                .map(|s| s[0][r].1 + s[1][g].1 + s[2][b].1)
                .min()
                .unwrap_or(0);
            starts.push(list.len() as u32);
            for (i, s) in spans.iter().enumerate() {
                if s[0][r].0 + s[1][g].0 + s[2][b].0 <= bound {
                    list.push(i as u8);
                }
            }
        }
        starts.push(list.len() as u32);
        Near {
            colors,
            starts,
            list,
        }
    }
    pub(crate) fn find(&self, [r, g, b]: [i32; 3]) -> u8 {
        let cell = ((r >> 4) << 8 | (g >> 4) << 4 | b >> 4) as usize;
        let mut best = (i32::MAX, 0);
        for &i in &self.list[self.starts[cell] as usize..self.starts[cell + 1] as usize] {
            let [cr, cg, cb] = self.colors[i as usize];
            let d = (r - cr).pow(2) + (g - cg).pow(2) + (b - cb).pow(2);
            if d < best.0 {
                best = (d, i);
            }
        }
        best.1
    }
}

pub(crate) fn plain(frame: &[[u8; 4]], near: &Near, first: u8, clear: bool) -> Vec<u8> {
    let mut cache = vec![(EMPTY, 0u8); 4096];
    frame
        .iter()
        .map(|&pixel| {
            if clear && !opaque(pixel) {
                return 0;
            }
            let k = key(pixel);
            let slot = (k.wrapping_mul(0x9E37_79B1) >> 20) as usize;
            if cache[slot].0 != k {
                cache[slot] = (
                    k,
                    near.find([pixel[0], pixel[1], pixel[2]].map(|v| v as i32)),
                );
            }
            cache[slot].1 + first
        })
        .collect()
}

pub(crate) struct Keep<'a> {
    pub(crate) source: &'a [[u8; 4]],
    pub(crate) indices: &'a [u8],
}

pub(crate) fn dither(
    frame: &[[u8; 4]],
    width: usize,
    near: &Near,
    first: u8,
    clear: bool,
    keep: Option<Keep>,
) -> Vec<u8> {
    let height = frame.len() / width;
    let mut out = vec![0u8; frame.len()];
    let mut here = vec![[0i32; 3]; width + 2];
    let mut below = vec![[0i32; 3]; width + 2];
    for y in 0..height {
        let forward = y % 2 == 0;
        for step in 0..width {
            let x = if forward { step } else { width - 1 - step };
            let at = y * width + x;
            let pixel = frame[at];
            if clear && !opaque(pixel) {
                continue;
            }
            let source = [pixel[0], pixel[1], pixel[2]].map(|v| v as i32);
            let (index, error) = match &keep {
                Some(keep) if keep.source[at] == pixel => {
                    let index = keep.indices[at];
                    let color = near.colors[(index - first) as usize];
                    (index, [0, 1, 2].map(|a| source[a] - color[a]))
                }
                _ => {
                    let carried = here[x + 1];
                    let want = [0, 1, 2].map(|a| (source[a] + carried[a] / 16).clamp(0, 255));
                    let found = near.find(want);
                    let color = near.colors[found as usize];
                    (found + first, [0, 1, 2].map(|a| want[a] - color[a]))
                }
            };
            out[at] = index;
            let (ahead, behind) = if forward { (x + 2, x) } else { (x, x + 2) };
            for a in 0..3 {
                here[ahead][a] += 7 * error[a];
                below[behind][a] += 3 * error[a];
                below[x + 1][a] += 5 * error[a];
                below[ahead][a] += error[a];
            }
        }
        std::mem::swap(&mut here, &mut below);
        below.fill([0; 3]);
    }
    out
}
