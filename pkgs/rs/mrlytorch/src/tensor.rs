use crate::ops;
use crate::ops::{Map, Reduce, Zip};
use crate::Result;

/// The padding modes a convolution can take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pad {
    /// No padding, so the output shrinks by the kernel span.
    Valid,
    /// Zero padding that keeps the output the input's size.
    Same,
}

impl Pad {
    /// Returns the display name.
    pub fn name(self) -> &'static str {
        match self {
            Pad::Valid => "valid",
            Pad::Same => "same",
        }
    }

    /// Parses a display name back into its mode, or an error for an unknown name.
    pub fn parse(text: &str) -> Result<Pad> {
        match text {
            "valid" => Ok(Pad::Valid),
            "same" => Ok(Pad::Same),
            _ => Err("no such padding"),
        }
    }

    /// Lists every mode.
    pub fn all() -> [Pad; 2] {
        [Pad::Valid, Pad::Same]
    }
}

/// A dense row-major f32 tensor.
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    /// The flat values in row-major order.
    pub data: Vec<f32>,
    /// The extent of each axis.
    pub shape: Vec<usize>,
}

impl Tensor {
    /// Wraps flat data in a shape, or an error when the data does not fill it.
    pub fn new(data: Vec<f32>, shape: Vec<usize>) -> Result<Tensor> {
        if data.len() != shape.iter().product::<usize>() {
            return Err("data does not fill the shape");
        }
        Ok(Tensor { data, shape })
    }

    /// Builds a tensor of zeros.
    pub fn zeros(shape: &[usize]) -> Tensor {
        Tensor::full(shape, 0.0)
    }

    /// Builds a tensor holding one value everywhere.
    pub fn full(shape: &[usize], value: f32) -> Tensor {
        Tensor {
            data: vec![value; shape.iter().product()],
            shape: shape.to_vec(),
        }
    }

    /// Stacks equal-length rows into a 2d tensor, or an error for empty or uneven rows.
    pub fn from_rows(rows: &[Vec<f32>]) -> Result<Tensor> {
        if rows.is_empty() || rows[0].is_empty() {
            return Err("rows are empty");
        }
        let width = rows[0].len();
        let mut data = Vec::with_capacity(rows.len() * width);
        for row in rows {
            if row.len() != width {
                return Err("rows have uneven lengths");
            }
            data.extend_from_slice(row);
        }
        Ok(Tensor {
            data,
            shape: vec![rows.len(), width],
        })
    }

    /// Counts the values held.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns the row-major stride of each axis.
    pub fn strides(&self) -> Vec<usize> {
        let mut strides = vec![1; self.shape.len()];
        for axis in (0..self.shape.len().saturating_sub(1)).rev() {
            strides[axis] = strides[axis + 1] * self.shape[axis + 1];
        }
        strides
    }

    /// Reads one value at a multi-index, or panics outside the shape.
    pub fn at(&self, index: &[usize]) -> f32 {
        assert_eq!(index.len(), self.shape.len(), "index rank differs");
        let mut flat = 0;
        for ((&i, &side), stride) in index.iter().zip(&self.shape).zip(self.strides()) {
            assert!(i < side, "index outside the shape");
            flat += i * stride;
        }
        self.data[flat]
    }

    fn zip_with(&self, other: &Tensor, op: Zip) -> Result<Tensor> {
        if self.shape != other.shape {
            return Err("shapes differ");
        }
        let mut out = Tensor::zeros(&self.shape);
        ops::zip(op, &self.data, &other.data, &mut out.data);
        Ok(out)
    }

    /// Adds another tensor elementwise, or an error when shapes differ.
    #[allow(clippy::should_implement_trait)]
    pub fn add(&self, other: &Tensor) -> Result<Tensor> {
        self.zip_with(other, Zip::Add)
    }

    /// Subtracts another tensor elementwise, or an error when shapes differ.
    #[allow(clippy::should_implement_trait)]
    pub fn sub(&self, other: &Tensor) -> Result<Tensor> {
        self.zip_with(other, Zip::Sub)
    }

    /// Multiplies another tensor elementwise, or an error when shapes differ.
    #[allow(clippy::should_implement_trait)]
    pub fn mul(&self, other: &Tensor) -> Result<Tensor> {
        self.zip_with(other, Zip::Mul)
    }

    /// Multiplies every value by a factor.
    pub fn scale(&self, factor: f32) -> Tensor {
        let mut out = Tensor::zeros(&self.shape);
        ops::map(Map::Scale(factor), &self.data, &mut out.data);
        out
    }

    /// Multiplies two 2d tensors, or an error when the shapes do not chain.
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor> {
        if self.shape.len() != 2 || other.shape.len() != 2 {
            return Err("matmul needs two 2d tensors");
        }
        if self.shape[1] != other.shape[0] {
            return Err("inner sizes differ");
        }
        let (m, k, n) = (self.shape[0], self.shape[1], other.shape[1]);
        let mut out = Tensor::zeros(&[m, n]);
        ops::gemm(&self.data, &other.data, m, k, n, &mut out.data);
        Ok(out)
    }

    /// Swaps the axes of a 2d tensor, or an error for any other rank.
    pub fn transpose(&self) -> Result<Tensor> {
        if self.shape.len() != 2 {
            return Err("transpose needs a 2d tensor");
        }
        let (h, w) = (self.shape[0], self.shape[1]);
        let mut out = Tensor::zeros(&[w, h]);
        for y in 0..h {
            for x in 0..w {
                out.data[x * h + y] = self.data[y * w + x];
            }
        }
        Ok(out)
    }

    /// Rewraps the data in a new shape, or an error when the size would change.
    pub fn reshape(&self, shape: &[usize]) -> Result<Tensor> {
        if self.data.len() != shape.iter().product::<usize>() {
            return Err("reshape changes the size");
        }
        Ok(Tensor {
            data: self.data.clone(),
            shape: shape.to_vec(),
        })
    }

    /// Sums every value in a fixed order.
    pub fn sum(&self) -> f32 {
        ops::reduce(Reduce::Sum, &self.data)
    }

    /// Averages every value, or zero for an empty tensor.
    pub fn mean(&self) -> f32 {
        if self.data.is_empty() {
            0.0
        } else {
            self.sum() / self.data.len() as f32
        }
    }

    /// Returns the flat index of the first largest value, or zero for an empty tensor.
    pub fn argmax(&self) -> usize {
        let mut best = 0;
        for (i, &v) in self.data.iter().enumerate() {
            if v > self.data[best] {
                best = i;
            }
        }
        best
    }

    /// Convolves a 2d tensor with a 2d kernel, or an error for bad ranks or spans.
    pub fn conv2d(&self, kernel: &Tensor, pad: Pad) -> Result<Tensor> {
        if self.shape.len() != 2 || kernel.shape.len() != 2 {
            return Err("conv2d needs two 2d tensors");
        }
        let (h, w) = (self.shape[0], self.shape[1]);
        let (kh, kw) = (kernel.shape[0], kernel.shape[1]);
        match pad {
            Pad::Valid => {
                if kh > h || kw > w {
                    return Err("kernel exceeds the input");
                }
                let mut out = Tensor::zeros(&[h - kh + 1, w - kw + 1]);
                ops::conv(
                    &self.data,
                    [h, w],
                    &kernel.data,
                    [kh, kw],
                    [0, 0],
                    &mut out.data,
                );
                Ok(out)
            }
            Pad::Same => {
                if kh % 2 == 0 || kw % 2 == 0 {
                    return Err("same padding needs odd kernel sides");
                }
                let mut out = Tensor::zeros(&[h, w]);
                ops::conv(
                    &self.data,
                    [h, w],
                    &kernel.data,
                    [kh, kw],
                    [kh / 2, kw / 2],
                    &mut out.data,
                );
                Ok(out)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_check_their_shapes() {
        assert!(Tensor::new(vec![1.0, 2.0], vec![2, 1]).is_ok());
        assert_eq!(
            Tensor::new(vec![1.0], vec![2]),
            Err("data does not fill the shape")
        );
        assert_eq!(Tensor::from_rows(&[]), Err("rows are empty"));
        assert_eq!(
            Tensor::from_rows(&[vec![1.0], vec![1.0, 2.0]]),
            Err("rows have uneven lengths")
        );
        assert_eq!(Tensor::full(&[2, 2], 3.0).data, vec![3.0; 4]);
    }

    #[test]
    fn elementwise_ops_match_hand_values() {
        let a = Tensor::from_rows(&[vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let b = Tensor::from_rows(&[vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        assert_eq!(a.add(&b).unwrap().data, vec![6.0, 8.0, 10.0, 12.0]);
        assert_eq!(a.sub(&b).unwrap().data, vec![-4.0; 4]);
        assert_eq!(a.mul(&b).unwrap().data, vec![5.0, 12.0, 21.0, 32.0]);
        assert_eq!(a.scale(2.0).data, vec![2.0, 4.0, 6.0, 8.0]);
        let c = Tensor::zeros(&[3]);
        assert_eq!(a.add(&c), Err("shapes differ"));
    }

    #[test]
    fn matmul_matches_hand_values_and_checks_shapes() {
        let a = Tensor::from_rows(&[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let b = Tensor::from_rows(&[vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]).unwrap();
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape, vec![2, 2]);
        assert_eq!(c.data, vec![58.0, 64.0, 139.0, 154.0]);
        assert_eq!(a.matmul(&a), Err("inner sizes differ"));
        assert_eq!(
            a.matmul(&Tensor::zeros(&[3])),
            Err("matmul needs two 2d tensors")
        );
    }

    #[test]
    fn transpose_laws_hold() {
        let a = Tensor::from_rows(&[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let t = a.transpose().unwrap();
        assert_eq!(t.shape, vec![3, 2]);
        assert_eq!(t.at(&[2, 1]), a.at(&[1, 2]));
        assert_eq!(t.transpose().unwrap(), a);
        let b = Tensor::from_rows(&[vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]).unwrap();
        let left = a.matmul(&b).unwrap().transpose().unwrap();
        let right = b
            .transpose()
            .unwrap()
            .matmul(&a.transpose().unwrap())
            .unwrap();
        assert_eq!(left, right);
        assert_eq!(
            Tensor::zeros(&[2]).transpose(),
            Err("transpose needs a 2d tensor")
        );
    }

    #[test]
    fn reshape_keeps_data_and_checks_size() {
        let a = Tensor::from_rows(&[vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let flat = a.reshape(&[4]).unwrap();
        assert_eq!(flat.data, a.data);
        assert_eq!(flat.reshape(&[2, 2]).unwrap(), a);
        assert_eq!(a.reshape(&[3]), Err("reshape changes the size"));
    }

    #[test]
    fn reductions_match_hand_values() {
        let a = Tensor::from_rows(&[vec![1.0, -2.0], vec![7.0, 4.0]]).unwrap();
        assert_eq!(a.sum(), 10.0);
        assert_eq!(a.mean(), 2.5);
        assert_eq!(a.argmax(), 2);
        assert_eq!(Tensor::zeros(&[0]).mean(), 0.0);
        assert_eq!(Tensor::zeros(&[0]).argmax(), 0);
    }

    #[test]
    fn conv2d_matches_hand_values() {
        let x = Tensor::from_rows(&[
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ])
        .unwrap();
        let k = Tensor::from_rows(&[vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
        let valid = x.conv2d(&k, Pad::Valid).unwrap();
        assert_eq!(valid.shape, vec![2, 2]);
        assert_eq!(valid.data, vec![6.0, 8.0, 12.0, 14.0]);
        let ones = Tensor::full(&[3, 3], 1.0);
        let same = x.conv2d(&ones, Pad::Same).unwrap();
        assert_eq!(same.shape, vec![3, 3]);
        assert_eq!(same.at(&[1, 1]), 45.0);
        assert_eq!(same.at(&[0, 0]), 12.0);
        assert_eq!(
            x.conv2d(&k, Pad::Same),
            Err("same padding needs odd kernel sides")
        );
        assert_eq!(k.conv2d(&ones, Pad::Valid), Err("kernel exceeds the input"));
    }

    #[test]
    fn strides_walk_row_major() {
        let t = Tensor::zeros(&[2, 3, 4]);
        assert_eq!(t.strides(), vec![12, 4, 1]);
        assert_eq!(Tensor::zeros(&[5]).strides(), vec![1]);
    }
}
