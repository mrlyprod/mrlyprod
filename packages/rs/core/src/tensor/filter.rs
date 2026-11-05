use super::{Buf, Dtype, Tensor, strides, unravel};
use crate::error::{Result, value_error};

fn source(
    row: usize,
    offset: &[isize],
    shape: &[usize],
    steps: &[usize],
    wrap: bool,
) -> Option<usize> {
    let mut flat = 0;
    for ((&n, &step), &o) in shape.iter().zip(steps).zip(offset) {
        let n = n as isize;
        let mut p = ((row / step) as isize % n) + o;
        if wrap {
            p = p.rem_euclid(n);
        } else if !(0..n).contains(&p) {
            return None;
        }
        flat += p as usize * step;
    }
    Some(flat)
}

fn window_counts(
    hits: &[u8],
    shape: &[usize],
    mask: &Tensor,
    side: &[usize],
    wrap: bool,
) -> Vec<u32> {
    let rank = shape.len() - 1;
    let (width, lane) = (shape[rank], side[rank]);
    let half = lane / 2;
    let span = width + lane;
    let mut prefix = vec![0u32; hits.len() / width * span];
    for (row, sums) in hits.chunks(width).zip(prefix.chunks_mut(span)) {
        let mut at = (width - half % width) % width;
        let mut total = 0;
        for (k, slot) in sums.iter_mut().enumerate().skip(1) {
            let hit = if wrap {
                let hit = row[at];
                at = if at + 1 == width { 0 } else { at + 1 };
                hit
            } else {
                (k - 1)
                    .checked_sub(half)
                    .and_then(|i| row.get(i))
                    .copied()
                    .unwrap_or(0)
            };
            total += u32::from(hit);
            *slot = total;
        }
    }
    let (outer, steps, mask_steps) = (
        &shape[..rank],
        strides(&shape[..rank]),
        strides(&side[..rank]),
    );
    let mut counts = vec![0u32; hits.len()];
    let mut offset = vec![0isize; rank];
    let mut runs = Vec::new();
    for m in 0..mask.size() / lane {
        runs.clear();
        let mut start = None;
        for i in 0..=lane {
            match (i < lane && mask.data.at(m * lane + i) == 1, start) {
                (true, None) => start = Some(i),
                (false, Some(s)) => {
                    runs.push((s, i));
                    start = None;
                }
                _ => {}
            }
        }
        if runs.is_empty() {
            continue;
        }
        for ((slot, &n), &step) in offset.iter_mut().zip(&side[..rank]).zip(&mask_steps) {
            *slot = ((m / step) % n) as isize - (n / 2) as isize;
        }
        for (p, out) in counts.chunks_mut(width).enumerate() {
            let Some(q) = source(p, &offset, outer, &steps, wrap) else {
                continue;
            };
            let sums = &prefix[q * span..(q + 1) * span];
            for &(s, e) in &runs {
                for ((c, &hi), &lo) in out
                    .iter_mut()
                    .zip(&sums[e..e + width])
                    .zip(&sums[s..s + width])
                {
                    *c += hi - lo;
                }
            }
        }
    }
    counts
}

impl Tensor {
    pub fn layers(&self, dtype: Dtype) -> Tensor {
        let shape = self.shape.clone();
        let mut out = Tensor::typed(shape.clone(), dtype);
        for flat in 0..out.size() {
            let idx = unravel(flat, &shape);
            let ring = idx
                .iter()
                .zip(&shape)
                .map(|(&i, &n)| {
                    let center = (n as f64 - 1.0) / 2.0;
                    (i as f64 - center).abs().floor() as i64
                })
                .max()
                .unwrap_or(0);
            out.data.put(flat, ring);
        }
        out
    }
    pub fn neighbors(&self, mask: &Tensor, target: u8, wrap: bool, dtype: Dtype) -> Result<Tensor> {
        if mask.shape.len() != self.shape.len() {
            return value_error("mask must have the same number of dimensions.");
        }
        if mask.shape.iter().any(|n| n.is_multiple_of(2)) {
            return value_error("Neighborhood (mask) dimensions must be odd.");
        }
        if target > 1 {
            return value_error("Bit to count (target) must be 0 or 1.");
        }
        if self.size() == 0 {
            return Ok(Tensor::typed(self.shape.clone(), dtype));
        }
        let (shape, side) = match self.shape.len() {
            0 => (vec![1], vec![1]),
            _ => (self.shape.clone(), mask.shape.clone()),
        };
        let counts = window_counts(&self.data.hits(target), &shape, mask, &side, wrap);
        if counts.iter().any(|&c| i64::from(c) > dtype.max()) {
            return value_error("neighbor count exceeds dtype range; widen the neighbors dtype.");
        }
        Ok(Tensor {
            data: Buf::counts(dtype, &counts),
            shape: self.shape.clone(),
        })
    }
    pub fn binarize(&self, threshold: u8) -> Tensor {
        let mut out = Tensor::new(self.shape.clone());
        for i in 0..self.size() {
            out.data
                .put(i, i64::from(self.data.at(i) >= threshold as i64));
        }
        out
    }
    pub fn otsu_threshold(&self) -> u8 {
        let mut hist = [0u64; 256];
        for i in 0..self.size() {
            hist[self.data.at(i).clamp(0, 255) as usize] += 1;
        }
        let total: u64 = hist.iter().sum();
        if total == 0 {
            return 0;
        }
        let sum_all: f64 = hist
            .iter()
            .enumerate()
            .map(|(v, &c)| v as f64 * c as f64)
            .sum();
        let mut sum_below = 0.0;
        let mut weight_below = 0u64;
        let mut best_variance = -1.0;
        let mut threshold = 0u8;
        for (level, &count) in hist.iter().enumerate() {
            weight_below += count;
            if weight_below == 0 {
                continue;
            }
            let weight_above = total - weight_below;
            if weight_above == 0 {
                break;
            }
            sum_below += level as f64 * count as f64;
            let mean_below = sum_below / weight_below as f64;
            let mean_above = (sum_all - sum_below) / weight_above as f64;
            let variance =
                weight_below as f64 * weight_above as f64 * (mean_below - mean_above).powi(2);
            if variance > best_variance {
                best_variance = variance;
                threshold = level as u8;
            }
        }
        threshold
    }
    pub fn binarize_otsu(&self) -> Tensor {
        self.binarize(self.otsu_threshold().saturating_add(1))
    }
    pub fn blur(&self, mask: &Tensor, wrap: bool) -> Result<Tensor> {
        if mask.shape.len() != self.shape.len() {
            return value_error("mask must have the same number of dimensions.");
        }
        if mask.shape.iter().any(|n| n.is_multiple_of(2)) {
            return value_error("Neighborhood (mask) dimensions must be odd.");
        }
        let center: Vec<usize> = mask.shape.iter().map(|&n| n / 2).collect();
        let mut offsets = Vec::new();
        for flat in 0..mask.size() {
            if mask.data.at(flat) != 0 {
                let idx = mask.multi(flat);
                offsets.push(
                    idx.iter()
                        .zip(&center)
                        .map(|(&i, &c)| i as isize - c as isize)
                        .collect::<Vec<isize>>(),
                );
            }
        }
        if offsets.is_empty() {
            return value_error("blur mask must have at least one nonzero cell.");
        }
        let mut out = Tensor::typed(self.shape.clone(), self.dtype());
        for flat in 0..self.size() {
            let idx = self.multi(flat);
            let mut sum: i64 = 0;
            let mut count: i64 = 0;
            for offset in &offsets {
                let mut source = Vec::with_capacity(idx.len());
                let mut inside = true;
                for axis in 0..idx.len() {
                    let n = self.shape[axis] as isize;
                    let mut p = idx[axis] as isize + offset[axis];
                    if wrap {
                        p = p.rem_euclid(n);
                    } else if p < 0 || p >= n {
                        inside = false;
                        break;
                    }
                    source.push(p as usize);
                }
                if inside {
                    sum += self.data.at(self.index(&source));
                    count += 1;
                }
            }
            let value = if count > 0 {
                (sum as f64 / count as f64).round() as i64
            } else {
                self.data.at(flat)
            };
            out.data.put(flat, value);
        }
        Ok(out)
    }
    pub fn perforate(&self, mask: &Tensor, value: u8) -> Result<Tensor> {
        if mask.shape.len() != self.shape.len() {
            return value_error("mask must have the same number of dimensions.");
        }
        for (axis, (&n, &m)) in self.shape.iter().zip(&mask.shape).enumerate() {
            if m == 0 || !n.is_multiple_of(m) {
                return value_error(format!(
                    "mask dimension {axis} must evenly tile the tensor dimension."
                ));
            }
        }
        let mut out = Tensor::typed(self.shape.clone(), self.dtype());
        for flat in 0..self.size() {
            let idx = self.multi(flat);
            let mask_idx: Vec<usize> = idx.iter().zip(&mask.shape).map(|(&i, &m)| i % m).collect();
            let hit = mask.data.at(mask.index(&mask_idx)) != 0;
            let value = if hit {
                value as i64
            } else {
                self.data.at(flat)
            };
            out.data.put(flat, value);
        }
        Ok(out)
    }
    pub fn count(&self, value: u8) -> usize {
        (0..self.size())
            .filter(|&i| self.data.at(i) == value as i64)
            .count()
    }
    pub fn exposed(&self) -> u128 {
        let dims = self.shape.clone();
        let rank = dims.len();
        let mut total: u128 = 0;
        for flat in 0..self.size() {
            if self.data.at(flat) == 0 {
                continue;
            }
            let mut rem = flat;
            let mut coord = Vec::with_capacity(rank);
            for axis in 0..rank {
                let stride: usize = dims[(axis + 1)..].iter().product();
                coord.push(rem / stride);
                rem %= stride;
            }
            for axis in 0..rank {
                if coord[axis] == 0 || {
                    let mut lo = coord.clone();
                    lo[axis] -= 1;
                    self.data.at(self.index(&lo)) == 0
                } {
                    total += 1;
                }
                if coord[axis] + 1 == dims[axis] || {
                    let mut hi = coord.clone();
                    hi[axis] += 1;
                    self.data.at(self.index(&hi)) == 0
                } {
                    total += 1;
                }
            }
        }
        total
    }
}
