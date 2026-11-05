use super::{Tensor, strides, unravel};
use crate::error::{Result, shape_error, value_error};

impl Tensor {
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
    pub fn flip(&self, axis: usize) -> Result<Tensor> {
        let n = self.shape[self.axis(axis)?];
        Ok(self.remap(self.shape.clone(), |idx| {
            let mut src = idx.to_vec();
            src[axis] = n - 1 - src[axis];
            src
        }))
    }
    pub fn transpose(&self, a: usize, b: usize) -> Result<Tensor> {
        let mut shape = self.shape.clone();
        shape.swap(self.axis(a)?, self.axis(b)?);
        Ok(self.remap(shape, |idx| {
            let mut src = idx.to_vec();
            src.swap(a, b);
            src
        }))
    }
    pub fn slice(&self, axis: usize, index: usize) -> Result<Tensor> {
        if axis >= self.shape.len() {
            return value_error("slice axis is past the tensor's rank.");
        }
        if index >= self.shape[axis] {
            return value_error("slice index is past the axis.");
        }
        let mut shape = self.shape.clone();
        shape.remove(axis);
        Ok(self.remap(shape, |idx| {
            let mut src = idx.to_vec();
            src.insert(axis, index);
            src
        }))
    }
    pub fn rot90(&self, k: usize, axes: (usize, usize)) -> Result<Tensor> {
        let (a, b) = (self.axis(axes.0)?, self.axis(axes.1)?);
        if a == b {
            return value_error(format!("rot90 needs two distinct axes, got {axes:?}."));
        }
        let mut out = self.clone();
        for _ in 0..k % 4 {
            out = out.transpose(a, b)?.flip(a)?;
        }
        Ok(out)
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
    pub fn tile(&self, reps: &[usize]) -> Result<Tensor> {
        if reps.len() != self.shape.len() {
            return shape_error(format!(
                "tile wants one count per axis, got {} for rank {}.",
                reps.len(),
                self.shape.len()
            ));
        }
        let shape: Vec<usize> = self.shape.iter().zip(reps).map(|(n, r)| n * r).collect();
        let inner = self.shape.clone();
        Ok(self.remap(shape, |idx| {
            idx.iter().zip(&inner).map(|(i, n)| i % n).collect()
        }))
    }
}
