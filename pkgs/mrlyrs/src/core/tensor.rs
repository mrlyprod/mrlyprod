#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtype {
    U8,
    U16,
    U32,
    I32,
}

impl Dtype {
    pub fn max(self) -> i64 {
        match self {
            Dtype::U8 => u8::MAX as i64,
            Dtype::U16 => u16::MAX as i64,
            Dtype::U32 => u32::MAX as i64,
            Dtype::I32 => i32::MAX as i64,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Buf {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    I32(Vec<i32>),
}

impl Buf {
    fn zeros(dtype: Dtype, n: usize) -> Buf {
        Buf::filled(dtype, n, 0)
    }
    fn filled(dtype: Dtype, n: usize, value: i64) -> Buf {
        match dtype {
            Dtype::U8 => Buf::U8(vec![value as u8; n]),
            Dtype::U16 => Buf::U16(vec![value as u16; n]),
            Dtype::U32 => Buf::U32(vec![value as u32; n]),
            Dtype::I32 => Buf::I32(vec![value as i32; n]),
        }
    }
    fn dtype(&self) -> Dtype {
        match self {
            Buf::U8(_) => Dtype::U8,
            Buf::U16(_) => Dtype::U16,
            Buf::U32(_) => Dtype::U32,
            Buf::I32(_) => Dtype::I32,
        }
    }
    fn len(&self) -> usize {
        match self {
            Buf::U8(v) => v.len(),
            Buf::U16(v) => v.len(),
            Buf::U32(v) => v.len(),
            Buf::I32(v) => v.len(),
        }
    }
    fn at(&self, i: usize) -> i64 {
        match self {
            Buf::U8(v) => v[i] as i64,
            Buf::U16(v) => v[i] as i64,
            Buf::U32(v) => v[i] as i64,
            Buf::I32(v) => v[i] as i64,
        }
    }
    fn put(&mut self, i: usize, value: i64) {
        match self {
            Buf::U8(v) => v[i] = value as u8,
            Buf::U16(v) => v[i] = value as u16,
            Buf::U32(v) => v[i] = value as u32,
            Buf::I32(v) => v[i] = value as i32,
        }
    }
    fn gather(&self, idx: &[usize]) -> Buf {
        match self {
            Buf::U8(v) => Buf::U8(idx.iter().map(|&i| v[i]).collect()),
            Buf::U16(v) => Buf::U16(idx.iter().map(|&i| v[i]).collect()),
            Buf::U32(v) => Buf::U32(idx.iter().map(|&i| v[i]).collect()),
            Buf::I32(v) => Buf::I32(idx.iter().map(|&i| v[i]).collect()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tensor {
    data: Buf,
    pub shape: Vec<usize>,
}

fn strides(shape: &[usize]) -> Vec<usize> {
    let mut out = vec![1; shape.len()];
    for axis in (0..shape.len().saturating_sub(1)).rev() {
        out[axis] = out[axis + 1] * shape[axis + 1];
    }
    out
}

fn unravel(flat: usize, shape: &[usize]) -> Vec<usize> {
    let s = strides(shape);
    let mut rem = flat;
    s.iter()
        .map(|&st| {
            let i = rem / st;
            rem %= st;
            i
        })
        .collect()
}

impl Tensor {
    pub fn new(shape: Vec<usize>) -> Self {
        Tensor::typed(shape, Dtype::U8)
    }
    pub fn full(shape: Vec<usize>, value: u8) -> Self {
        Tensor::filled(shape, value as i64, Dtype::U8)
    }
    pub fn typed(shape: Vec<usize>, dtype: Dtype) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: Buf::zeros(dtype, size),
            shape,
        }
    }
    pub fn filled(shape: Vec<usize>, value: i64, dtype: Dtype) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: Buf::filled(dtype, size, value),
            shape,
        }
    }
    pub fn of(data: Vec<u8>, shape: Vec<usize>) -> Self {
        Tensor {
            data: Buf::U8(data),
            shape,
        }
    }
    pub fn dtype(&self) -> Dtype {
        self.data.dtype()
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn bytes(&self) -> &[u8] {
        match &self.data {
            Buf::U8(v) => v,
            other => panic!("tensor is {:?}, not u8", other.dtype()),
        }
    }
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        match &mut self.data {
            Buf::U8(v) => v,
            other => panic!("tensor is {:?}, not u8", other.dtype()),
        }
    }
    pub fn at(&self, flat: usize) -> i64 {
        self.data.at(flat)
    }
    pub fn put(&mut self, flat: usize, value: i64) {
        self.data.put(flat, value);
    }
    pub fn sum(&self) -> u64 {
        (0..self.size()).map(|i| self.data.at(i) as u64).sum()
    }
    pub fn index(&self, multi: &[usize]) -> usize {
        let s = strides(&self.shape);
        multi.iter().zip(&s).map(|(m, st)| m * st).sum()
    }
    pub fn get(&self, multi: &[usize]) -> u8 {
        self.bytes()[self.index(multi)]
    }
    pub fn set(&mut self, multi: &[usize], value: u8) {
        let i = self.index(multi);
        self.bytes_mut()[i] = value;
    }
    pub fn kron(&self, other: &Tensor) -> Tensor {
        let shape: Vec<usize> = self
            .shape
            .iter()
            .zip(&other.shape)
            .map(|(a, b)| a * b)
            .collect();
        let mut out = Tensor::typed(shape.clone(), self.dtype());
        let sa = strides(&self.shape);
        let sb = strides(&other.shape);
        let so = strides(&shape);
        for flat in 0..out.size() {
            let mut ai = 0;
            let mut bi = 0;
            let mut rem = flat;
            for axis in 0..shape.len() {
                let idx = rem / so[axis];
                rem %= so[axis];
                ai += (idx / other.shape[axis]) * sa[axis];
                bi += (idx % other.shape[axis]) * sb[axis];
            }
            out.data.put(flat, self.data.at(ai) * other.data.at(bi));
        }
        out
    }
    pub fn fractal(&self, level: usize) -> Tensor {
        let mut out = self.clone();
        for _ in 1..level {
            out = out.kron(self);
        }
        out
    }
    fn multi(&self, flat: usize) -> Vec<usize> {
        unravel(flat, &self.shape)
    }
    fn remap(&self, shape: Vec<usize>, map: impl Fn(&[usize]) -> Vec<usize>) -> Tensor {
        let size: usize = shape.iter().product();
        let indices: Vec<usize> = (0..size)
            .map(|flat| self.index(&map(&unravel(flat, &shape))))
            .collect();
        Tensor {
            data: self.data.gather(&indices),
            shape,
        }
    }
    pub fn invert(&self) -> Tensor {
        let mut out = Tensor::typed(self.shape.clone(), self.dtype());
        for i in 0..self.size() {
            out.data.put(i, 1 - self.data.at(i));
        }
        out
    }
    pub fn flip(&self, axis: usize) -> Tensor {
        let n = self.shape[axis];
        self.remap(self.shape.clone(), |idx| {
            let mut src = idx.to_vec();
            src[axis] = n - 1 - src[axis];
            src
        })
    }
    pub fn transpose(&self, a: usize, b: usize) -> Tensor {
        let mut shape = self.shape.clone();
        shape.swap(a, b);
        self.remap(shape, |idx| {
            let mut src = idx.to_vec();
            src.swap(a, b);
            src
        })
    }
    pub fn rot90(&self, k: usize, axes: (usize, usize)) -> Tensor {
        let mut out = self.clone();
        for _ in 0..k % 4 {
            out = out.transpose(axes.0, axes.1).flip(axes.0);
        }
        out
    }
    pub fn pad(&self, count: usize, value: u8) -> Tensor {
        let shape: Vec<usize> = self.shape.iter().map(|&n| n + 2 * count).collect();
        let mut out = Tensor::filled(shape, value as i64, self.dtype());
        let so = strides(&out.shape);
        for flat in 0..self.size() {
            let idx = self.multi(flat);
            let target: usize = idx.iter().zip(&so).map(|(i, st)| (i + count) * st).sum();
            out.data.put(target, self.data.at(flat));
        }
        out
    }
    pub fn tile(&self, reps: &[usize]) -> Tensor {
        let shape: Vec<usize> = self.shape.iter().zip(reps).map(|(n, r)| n * r).collect();
        let inner = self.shape.clone();
        self.remap(shape, |idx| {
            idx.iter().zip(&inner).map(|(i, n)| i % n).collect()
        })
    }
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
    pub fn neighbors(
        &self,
        mask: &Tensor,
        target: u8,
        wrap: bool,
        dtype: Dtype,
    ) -> crate::core::Result<Tensor> {
        use crate::core::errors::value_error;
        if mask.shape.len() != self.shape.len() {
            return value_error("mask must have the same number of dimensions.");
        }
        if mask.shape.iter().any(|n| n.is_multiple_of(2)) {
            return value_error("Neighborhood (mask) dimensions must be odd.");
        }
        if target > 1 {
            return value_error("Bit to count (target) must be 0 or 1.");
        }
        let center: Vec<usize> = mask.shape.iter().map(|&n| n / 2).collect();
        let mut offsets = Vec::new();
        for flat in 0..mask.size() {
            if mask.data.at(flat) == 1 {
                let idx = mask.multi(flat);
                offsets.push(
                    idx.iter()
                        .zip(&center)
                        .map(|(&i, &c)| i as isize - c as isize)
                        .collect::<Vec<isize>>(),
                );
            }
        }
        let mut out = Tensor::typed(self.shape.clone(), dtype);
        for flat in 0..self.size() {
            let idx = self.multi(flat);
            let mut count: u32 = 0;
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
                if inside && self.get(&source) == target {
                    count += 1;
                }
            }
            if count as i64 > dtype.max() {
                return value_error(
                    "neighbor count exceeds dtype range; widen the neighbors dtype.",
                );
            }
            out.data.put(flat, count as i64);
        }
        Ok(out)
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
    pub fn blur(&self, mask: &Tensor, wrap: bool) -> crate::core::Result<Tensor> {
        use crate::core::errors::value_error;
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
    pub fn perforate(&self, mask: &Tensor, value: u8) -> crate::core::Result<Tensor> {
        use crate::core::errors::value_error;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kron_matches_numpy_semantics() {
        let a = Tensor::of(vec![1, 0, 0, 1], vec![2, 2]);
        let b = Tensor::of(vec![1, 1, 1, 1], vec![2, 2]);
        let k = a.kron(&b);
        assert_eq!(k.shape, vec![4, 4]);
        assert_eq!(k.sum(), 8);
        assert_eq!(k.get(&[0, 0]), 1);
        assert_eq!(k.get(&[0, 2]), 0);
        assert_eq!(k.get(&[3, 3]), 1);
    }
    #[test]
    fn fractal_sum_is_power() {
        let a = Tensor::of(vec![1, 1, 0, 1, 1, 0, 0, 0, 1], vec![3, 3]);
        let f = a.fractal(3);
        assert_eq!(f.shape, vec![27, 27]);
        assert_eq!(f.sum(), a.sum().pow(3));
    }
    #[test]
    fn rot90_matches_numpy() {
        let a = Tensor::of(vec![1, 2, 3, 4], vec![2, 2]);
        assert_eq!(a.rot90(1, (0, 1)).bytes(), &[2, 4, 1, 3]);
        assert_eq!(a.rot90(2, (0, 1)).bytes(), &[4, 3, 2, 1]);
        assert_eq!(a.rot90(4, (0, 1)), a);
    }
    #[test]
    fn pad_and_tile() {
        let a = Tensor::full(vec![2, 2], 1);
        let p = a.pad(1, 0);
        assert_eq!(p.shape, vec![4, 4]);
        assert_eq!(p.sum(), 4);
        assert_eq!(p.get(&[0, 0]), 0);
        assert_eq!(p.get(&[1, 1]), 1);
        let t = a.tile(&[2, 3]);
        assert_eq!(t.shape, vec![4, 6]);
        assert_eq!(t.sum(), 24);
    }
    #[test]
    fn layers_rings() {
        let l = Tensor::new(vec![5, 5]).layers(Dtype::U8);
        assert_eq!(l.get(&[2, 2]), 0);
        assert_eq!(l.get(&[1, 2]), 1);
        assert_eq!(l.get(&[0, 0]), 2);
        assert_eq!(l.get(&[4, 0]), 2);
    }
    #[test]
    fn neighbors_moore() {
        let mut mask = Tensor::full(vec![3, 3], 1);
        mask.set(&[1, 1], 0);
        let ones = Tensor::full(vec![3, 3], 1);
        let n = ones.neighbors(&mask, 1, false, Dtype::U8).unwrap();
        assert_eq!(n.get(&[1, 1]), 8);
        assert_eq!(n.get(&[0, 0]), 3);
        let w = ones.neighbors(&mask, 1, true, Dtype::U8).unwrap();
        assert_eq!(w.get(&[0, 0]), 8);
        assert!(ones
            .neighbors(&Tensor::full(vec![2, 2], 1), 1, false, Dtype::U8)
            .is_err());
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
    #[test]
    fn invert_round_trip() {
        let a = Tensor::of(vec![1, 0, 0, 1], vec![2, 2]);
        assert_eq!(a.invert().invert(), a);
        assert_eq!(a.invert().sum(), 2);
    }
    #[test]
    fn kron_3d() {
        let a = Tensor::full(vec![2, 2, 2], 1);
        let b = Tensor::full(vec![3, 3, 3], 1);
        let k = a.kron(&b);
        assert_eq!(k.shape, vec![6, 6, 6]);
        assert_eq!(k.sum(), 216);
    }
    #[test]
    fn binarize_thresholds_pointwise() {
        let a = Tensor::of(vec![0, 50, 128, 255], vec![2, 2]);
        let b = a.binarize(128);
        assert_eq!(b.bytes(), &[0, 0, 1, 1]);
    }
    #[test]
    fn binarize_is_idempotent() {
        let a = Tensor::of(vec![0, 50, 128, 255], vec![2, 2]);
        let once = a.binarize(1);
        let twice = once.binarize(1);
        assert_eq!(once, twice);
    }
    #[test]
    fn otsu_splits_bimodal_histogram() {
        let mut data = vec![10u8; 20];
        data.extend(vec![200u8; 20]);
        let a = Tensor::of(data, vec![40, 1]);
        let t = a.otsu_threshold();
        assert!((10..200).contains(&t));
        let b = a.binarize_otsu();
        assert_eq!(b.bytes()[0..20].iter().sum::<u8>(), 0);
        assert_eq!(b.bytes()[20..40].iter().sum::<u8>(), 20);
    }
    #[test]
    fn blur_box_smooths_a_spike() {
        let mut a = Tensor::new(vec![5, 5]);
        a.set(&[2, 2], 100);
        let mask = Tensor::full(vec![3, 3], 1);
        let b = a.blur(&mask, false).unwrap();
        assert!(b.get(&[2, 2]) < 100);
        assert!(b.get(&[2, 2]) > 0);
        assert_eq!(b.get(&[0, 0]), 0);
    }
    #[test]
    fn blur_preserves_mean_under_wrap() {
        let a = Tensor::of((0..25).map(|v| (v * 7) % 251).collect(), vec![5, 5]);
        let mask = Tensor::full(vec![3, 3], 1);
        let b = a.blur(&mask, true).unwrap();
        let mean_a: f64 = a.sum() as f64 / a.size() as f64;
        let mean_b: f64 = b.sum() as f64 / b.size() as f64;
        assert!((mean_a - mean_b).abs() < 1.0);
    }
    #[test]
    fn blur_rejects_mismatched_mask() {
        let a = Tensor::new(vec![4, 4]);
        assert!(a.blur(&Tensor::full(vec![2, 2], 1), false).is_err());
    }
    #[test]
    fn perforate_zero_mask_is_identity() {
        let a = Tensor::of(vec![1, 2, 3, 4], vec![2, 2]);
        let mask = Tensor::new(vec![2, 2]);
        let p = a.perforate(&mask, 9).unwrap();
        assert_eq!(p, a);
    }
    #[test]
    fn perforate_writes_masked_positions() {
        let a = Tensor::new(vec![4, 4]);
        let mask = Tensor::of(vec![1, 0, 0, 1], vec![2, 2]);
        let p = a.perforate(&mask, 7).unwrap();
        assert_eq!(p.get(&[0, 0]), 7);
        assert_eq!(p.get(&[0, 1]), 0);
        assert_eq!(p.get(&[1, 1]), 7);
        assert_eq!(p.sum(), 8 * 7);
    }
    #[test]
    fn perforate_rejects_non_tiling_mask() {
        let a = Tensor::new(vec![4, 4]);
        assert!(a.perforate(&Tensor::new(vec![3, 3]), 1).is_err());
    }
}
