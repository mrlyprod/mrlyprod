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
    /// Numbers every position by its concentric ring out from the center.
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
    /// Counts each position's masked neighbors holding the target bit.
    ///
    /// # Errors
    ///
    /// Errs on a mask of another rank or an even side, a target above one, or a count past the dtype.
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
    /// Maps every element to one at or above the threshold, zero below.
    pub fn binarize(&self, threshold: u8) -> Tensor {
        let mut out = Tensor::new(self.shape.clone());
        for i in 0..self.size() {
            out.data
                .put(i, i64::from(self.data.at(i) >= threshold as i64));
        }
        out
    }
    /// Returns the Otsu threshold splitting the histogram at greatest variance.
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
    /// Binarizes at one above the Otsu threshold.
    pub fn binarize_otsu(&self) -> Tensor {
        self.binarize(self.otsu_threshold().saturating_add(1))
    }
    /// Averages every position over its masked neighborhood, rounded.
    ///
    /// # Errors
    ///
    /// Errs on a mask of another rank, an even side, or a mask with nothing on.
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
    /// Stamps the value wherever the tiled mask is nonzero.
    ///
    /// # Errors
    ///
    /// Errs on a mask of another rank or a side that does not evenly tile the tensor.
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
    /// Counts the cells holding one value.
    pub fn count(&self, value: u8) -> usize {
        (0..self.size())
            .filter(|&i| self.data.at(i) == value as i64)
            .count()
    }
    /// Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seeds;
    fn of(data: Vec<u8>, shape: Vec<usize>) -> Tensor {
        Tensor::of(data, shape).unwrap()
    }
    #[test]
    fn exposed_is_perimeter_in_2d() {
        assert_eq!(seeds::ones_2d(1).exposed(), 4);
        assert_eq!(seeds::ones_2d(3).exposed(), 12);
        assert_eq!(seeds::carpet_2d(3).exposed(), 16);
    }
    #[test]
    fn exposed_is_surface_in_3d() {
        assert_eq!(seeds::ones_3d(1).exposed(), 6);
        assert_eq!(seeds::ones_3d(2).exposed(), 24);
    }
    #[test]
    fn count_tallies_one_value() {
        let c = seeds::carpet_2d(3);
        assert_eq!(c.count(1), 8);
        assert_eq!(c.count(0), 1);
    }
    #[test]
    fn layers_rings() {
        let l = Tensor::new(vec![5, 5]).layers(Dtype::U8);
        assert_eq!(l.get(&[2, 2]).unwrap(), 0);
        assert_eq!(l.get(&[1, 2]).unwrap(), 1);
        assert_eq!(l.get(&[0, 0]).unwrap(), 2);
        assert_eq!(l.get(&[4, 0]).unwrap(), 2);
    }
    #[test]
    fn neighbors_moore() {
        let mut mask = Tensor::full(vec![3, 3], 1);
        mask.set(&[1, 1], 0).unwrap();
        let ones = Tensor::full(vec![3, 3], 1);
        let n = ones.neighbors(&mask, 1, false, Dtype::U8).unwrap();
        assert_eq!(n.get(&[1, 1]).unwrap(), 8);
        assert_eq!(n.get(&[0, 0]).unwrap(), 3);
        let w = ones.neighbors(&mask, 1, true, Dtype::U8).unwrap();
        assert_eq!(w.get(&[0, 0]).unwrap(), 8);
        assert!(
            ones.neighbors(&Tensor::full(vec![2, 2], 1), 1, false, Dtype::U8)
                .is_err()
        );
    }
    #[test]
    fn neighbors_wide_dtype_survives_large_mask() {
        let mask = Tensor::full(vec![33, 33], 1);
        let grid = Tensor::full(vec![40, 40], 1);
        assert!(grid.neighbors(&mask, 1, true, Dtype::U8).is_err());
        let wide = grid.neighbors(&mask, 1, true, Dtype::U16).unwrap();
        assert_eq!(wide.dtype(), Dtype::U16);
        assert_eq!(wide.at(wide.index(&[20, 20])), 33 * 33);
    }
    fn naive(grid: &Tensor, mask: &Tensor, target: u8, wrap: bool, dtype: Dtype) -> Result<Tensor> {
        let offsets: Vec<Vec<isize>> = (0..mask.size())
            .filter(|&m| mask.at(m) == 1)
            .map(|m| {
                let idx = mask.multi(m);
                let half = mask.shape.iter().map(|&n| (n / 2) as isize);
                idx.iter().zip(half).map(|(&i, c)| i as isize - c).collect()
            })
            .collect();
        let mut out = Tensor::typed(grid.shape.clone(), dtype);
        for flat in 0..grid.size() {
            let idx = grid.multi(flat);
            let mut count = 0;
            for offset in &offsets {
                let source: Option<Vec<usize>> = idx
                    .iter()
                    .zip(offset)
                    .zip(&grid.shape)
                    .map(|((&i, &o), &n)| {
                        let (p, n) = (i as isize + o, n as isize);
                        match wrap {
                            true => Some(p.rem_euclid(n) as usize),
                            false => (0..n).contains(&p).then_some(p as usize),
                        }
                    })
                    .collect();
                if source.is_some_and(|s| grid.at(grid.index(&s)) == i64::from(target)) {
                    count += 1;
                }
            }
            if count > dtype.max() {
                return value_error(
                    "neighbor count exceeds dtype range; widen the neighbors dtype.",
                );
            }
            out.put(flat, count);
        }
        Ok(out)
    }
    #[test]
    fn neighbors_match_the_naive_count() {
        let mut rng = crate::rng::Rng::new(3);
        let mut noise = |shape: Vec<usize>| {
            let data: Vec<u8> = (0..shape.iter().product::<usize>())
                .map(|_| [0, 1, 1, 2][rng.below(4)])
                .collect();
            match rng.below(4) {
                0 => Tensor::u8(data, shape),
                1 => Tensor::u16(data.iter().map(|&v| v.into()).collect(), shape),
                2 => Tensor::u32(data.iter().map(|&v| v.into()).collect(), shape),
                _ => Tensor::i32(data.iter().map(|&v| v.into()).collect(), shape),
            }
            .unwrap()
        };
        let mut checked = 0;
        for rank in 0..=3 {
            for round in 0..24 {
                let shape = (0..rank).map(|axis| (round * 7 + axis * 5) % 9).collect();
                let side = (0..rank).map(|axis| 2 * ((round + axis) % 6) + 1).collect();
                let (grid, mask) = (noise(shape), noise(side));
                for (wrap, target) in [(false, 0), (false, 1), (true, 0), (true, 1)] {
                    for dtype in [Dtype::U8, Dtype::U16, Dtype::U32, Dtype::I32] {
                        let fast = grid.neighbors(&mask, target, wrap, dtype);
                        let slow = naive(&grid, &mask, target, wrap, dtype);
                        assert_eq!(
                            fast.map_err(|e| e.to_string()),
                            slow.map_err(|e| e.to_string()),
                            "rank {rank} round {round} wrap {wrap} target {target} {dtype:?}"
                        );
                        checked += 1;
                    }
                }
            }
        }
        assert_eq!(checked, 4 * 24 * 16);
    }
    #[test]
    fn binarize_thresholds_pointwise() {
        let a = of(vec![0, 50, 128, 255], vec![2, 2]);
        let b = a.binarize(128);
        assert_eq!(b.bytes().unwrap(), &[0, 0, 1, 1]);
    }
    #[test]
    fn otsu_splits_bimodal_histogram() {
        let mut data = vec![10u8; 20];
        data.extend(vec![200u8; 20]);
        let a = of(data, vec![40, 1]);
        let t = a.otsu_threshold();
        assert!((10..200).contains(&t));
        let b = a.binarize_otsu();
        assert_eq!(b.bytes().unwrap()[0..20].iter().sum::<u8>(), 0);
        assert_eq!(b.bytes().unwrap()[20..40].iter().sum::<u8>(), 20);
    }
    #[test]
    fn blur_preserves_mean_under_wrap() {
        let a = of((0..25).map(|v| (v * 7) % 251).collect(), vec![5, 5]);
        let mask = Tensor::full(vec![3, 3], 1);
        let b = a.blur(&mask, true).unwrap();
        let mean_a: f64 = a.sum() as f64 / a.size() as f64;
        let mean_b: f64 = b.sum() as f64 / b.size() as f64;
        assert!((mean_a - mean_b).abs() < 1.0);
    }
    #[test]
    fn perforate_zero_mask_is_identity() {
        let a = of(vec![1, 2, 3, 4], vec![2, 2]);
        let mask = Tensor::new(vec![2, 2]);
        let p = a.perforate(&mask, 9).unwrap();
        assert_eq!(p, a);
    }
    #[test]
    fn perforate_writes_masked_positions() {
        let a = Tensor::new(vec![4, 4]);
        let mask = of(vec![1, 0, 0, 1], vec![2, 2]);
        let p = a.perforate(&mask, 7).unwrap();
        assert_eq!(p.get(&[0, 0]).unwrap(), 7);
        assert_eq!(p.get(&[0, 1]).unwrap(), 0);
        assert_eq!(p.get(&[1, 1]).unwrap(), 7);
        assert_eq!(p.sum(), 8 * 7);
    }
}
