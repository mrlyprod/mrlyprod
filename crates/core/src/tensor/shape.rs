use super::{Tensor, strides, unravel};
use crate::error::{Result, shape_error, value_error};

impl Tensor {
    /// Builds the Kronecker product of the two tensors.
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
    /// Folds the tensor into its level-fold Kronecker power.
    ///
    /// ```
    /// use mrly_core::tensor::Tensor;
    /// let f = Tensor::of(vec![1, 1, 0, 1], vec![2, 2]).unwrap().fractal(2);
    /// assert_eq!(f.shape, vec![4, 4]);
    /// assert_eq!(f.sum(), 9);
    /// ```
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
    /// Flips every element between zero and one.
    pub fn invert(&self) -> Tensor {
        let mut out = Tensor::typed(self.shape.clone(), self.dtype());
        for i in 0..self.size() {
            out.data.put(i, 1 - self.data.at(i));
        }
        out
    }
    /// Reverses the tensor along one axis.
    ///
    /// # Errors
    ///
    /// Errs when the axis is past the rank.
    pub fn flip(&self, axis: usize) -> Result<Tensor> {
        let n = self.shape[self.axis(axis)?];
        Ok(self.remap(self.shape.clone(), |idx| {
            let mut src = idx.to_vec();
            src[axis] = n - 1 - src[axis];
            src
        }))
    }
    /// Swaps two axes.
    ///
    /// # Errors
    ///
    /// Errs when either axis is past the rank.
    pub fn transpose(&self, a: usize, b: usize) -> Result<Tensor> {
        let mut shape = self.shape.clone();
        shape.swap(self.axis(a)?, self.axis(b)?);
        Ok(self.remap(shape, |idx| {
            let mut src = idx.to_vec();
            src.swap(a, b);
            src
        }))
    }
    /// Drops one axis by fixing it at an index.
    ///
    /// # Errors
    ///
    /// Errs when the axis is past the rank or the index is past the axis.
    ///
    /// ```
    /// let block = mrly_core::tensor::Tensor::full(vec![2, 3, 4], 1);
    /// assert_eq!(block.slice(2, 0).unwrap(), mrly_core::tensor::Tensor::full(vec![2, 3], 1));
    /// ```
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
    /// Rotates the tensor k quarter turns in the plane of two axes.
    ///
    /// # Errors
    ///
    /// Errs when the axes are not two distinct axes of the tensor.
    ///
    /// ```
    /// use mrly_core::tensor::Tensor;
    /// let a = Tensor::of(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    /// assert_eq!(a.rot90(1, (0, 1)).unwrap().bytes().unwrap(), &[2, 4, 1, 3]);
    /// assert!(a.rot90(1, (0, 2)).is_err());
    /// ```
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
    /// Wraps the tensor in a count-thick border of one value.
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
    /// Repeats the tensor the given number of times along each axis.
    ///
    /// # Errors
    ///
    /// Errs without one count per axis.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seeds;
    fn of(data: Vec<u8>, shape: Vec<usize>) -> Tensor {
        Tensor::of(data, shape).unwrap()
    }
    #[test]
    fn kron_matches_numpy_semantics() {
        let a = of(vec![1, 0, 0, 1], vec![2, 2]);
        let b = of(vec![1, 1, 1, 1], vec![2, 2]);
        let k = a.kron(&b);
        assert_eq!(k.shape, vec![4, 4]);
        assert_eq!(k.sum(), 8);
        assert_eq!(k.get(&[0, 0]).unwrap(), 1);
        assert_eq!(k.get(&[0, 2]).unwrap(), 0);
        assert_eq!(k.get(&[3, 3]).unwrap(), 1);
    }
    #[test]
    fn fractal_sum_is_power() {
        let a = of(vec![1, 1, 0, 1, 1, 0, 0, 0, 1], vec![3, 3]);
        let f = a.fractal(3);
        assert_eq!(f.shape, vec![27, 27]);
        assert_eq!(f.sum(), a.sum().pow(3));
    }
    #[test]
    fn rot90_matches_numpy() {
        let a = of(vec![1, 2, 3, 4], vec![2, 2]);
        assert_eq!(a.rot90(1, (0, 1)).unwrap().bytes().unwrap(), &[2, 4, 1, 3]);
        assert_eq!(a.rot90(2, (0, 1)).unwrap().bytes().unwrap(), &[4, 3, 2, 1]);
        assert_eq!(a.rot90(4, (0, 1)).unwrap(), a);
    }
    #[test]
    fn pad_and_tile() {
        let a = Tensor::full(vec![2, 2], 1);
        let p = a.pad(1, 0);
        assert_eq!(p.shape, vec![4, 4]);
        assert_eq!(p.sum(), 4);
        assert_eq!(p.get(&[0, 0]).unwrap(), 0);
        assert_eq!(p.get(&[1, 1]).unwrap(), 1);
        let t = a.tile(&[2, 3]).unwrap();
        assert_eq!(t.shape, vec![4, 6]);
        assert_eq!(t.sum(), 24);
    }
    #[test]
    fn invert_round_trip() {
        let a = of(vec![1, 0, 0, 1], vec![2, 2]);
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
    fn slice_commutes_with_the_fractal() {
        for (n, level) in [(3, 2), (5, 2), (3, 3)] {
            let seed = seeds::xtree_3d(n);
            for axis in 0..3 {
                let deep = seed.fractal(level).slice(axis, 0).unwrap();
                let flat = seed.slice(axis, 0).unwrap().fractal(level);
                assert_eq!(deep, flat);
            }
        }
    }
    #[test]
    fn slice_takes_an_index_and_rejects_a_bad_one() {
        let a = of(vec![0, 1, 2, 3, 4, 5], vec![2, 3]);
        assert_eq!(a.slice(0, 1).unwrap(), of(vec![3, 4, 5], vec![3]));
        assert_eq!(a.slice(1, 2).unwrap(), of(vec![2, 5], vec![2]));
        assert!(a.slice(2, 0).is_err());
        assert!(a.slice(1, 3).is_err());
    }
    #[test]
    fn refuses_rot90() {
        let a = of(vec![1, 2, 3, 4], vec![2, 2]);
        assert!(a.rot90(1, (0, 2)).is_err());
        assert!(a.rot90(1, (2, 0)).is_err());
        assert!(a.rot90(1, (1, 1)).is_err());
        assert!(a.flip(2).is_err());
        assert!(a.transpose(0, 2).is_err());
        assert!(a.transpose(2, 0).is_err());
    }
    #[test]
    fn refuses_tile() {
        let a = Tensor::full(vec![2, 2], 1);
        assert!(a.tile(&[2]).is_err());
        assert!(a.tile(&[2, 2, 2]).is_err());
        assert!(a.tile(&[]).is_err());
    }
}
