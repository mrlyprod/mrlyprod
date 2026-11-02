use super::error::{Result, shape_error, value_error};
use serde::{Deserialize, Serialize};

mod filter;
mod shape;

/// The element widths a tensor can hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dtype {
    /// Unsigned 8-bit elements.
    U8,
    /// Unsigned 16-bit elements.
    U16,
    /// Unsigned 32-bit elements.
    U32,
    /// Signed 32-bit elements.
    I32,
}

impl Dtype {
    /// Returns the largest value the width can hold.
    pub fn max(self) -> i64 {
        match self {
            Dtype::U8 => u8::MAX as i64,
            Dtype::U16 => u16::MAX as i64,
            Dtype::U32 => u32::MAX as i64,
            Dtype::I32 => i32::MAX as i64,
        }
    }
}

/// The typed storage behind a tensor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Buf {
    /// Unsigned 8-bit storage.
    U8(Vec<u8>),
    /// Unsigned 16-bit storage.
    U16(Vec<u16>),
    /// Unsigned 32-bit storage.
    U32(Vec<u32>),
    /// Signed 32-bit storage.
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
    fn hits(&self, target: u8) -> Vec<u8> {
        fn flags<T: Copy + Into<i64>>(v: &[T], target: u8) -> Vec<u8> {
            v.iter()
                .map(|&x| u8::from(x.into() == i64::from(target)))
                .collect()
        }
        match self {
            Buf::U8(v) => flags(v, target),
            Buf::U16(v) => flags(v, target),
            Buf::U32(v) => flags(v, target),
            Buf::I32(v) => flags(v, target),
        }
    }
    fn counts(dtype: Dtype, counts: &[u32]) -> Buf {
        match dtype {
            Dtype::U8 => Buf::U8(counts.iter().map(|&c| c as u8).collect()),
            Dtype::U16 => Buf::U16(counts.iter().map(|&c| c as u16).collect()),
            Dtype::U32 => Buf::U32(counts.to_vec()),
            Dtype::I32 => Buf::I32(counts.iter().map(|&c| c as i32).collect()),
        }
    }
}

/// An n-dimensional grid of small integers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tensor {
    data: Buf,
    /// The extent of each axis.
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
    /// Builds a zeroed u8 tensor of the shape.
    pub fn new(shape: Vec<usize>) -> Self {
        Tensor::typed(shape, Dtype::U8)
    }
    /// Builds a u8 tensor filled with one value.
    pub fn full(shape: Vec<usize>, value: u8) -> Self {
        Tensor::filled(shape, value as i64, Dtype::U8)
    }
    /// Builds a zeroed tensor of the shape and width.
    pub fn typed(shape: Vec<usize>, dtype: Dtype) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: Buf::zeros(dtype, size),
            shape,
        }
    }
    /// Builds a tensor of the shape and width filled with one value.
    pub fn filled(shape: Vec<usize>, value: i64, dtype: Dtype) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: Buf::filled(dtype, size, value),
            shape,
        }
    }
    /// Wraps a byte vector as a tensor of the shape.
    ///
    /// # Errors
    ///
    /// Errs when the data length is not the shape's product.
    ///
    /// ```
    /// use mrly_core::tensor::Tensor;
    /// assert_eq!(Tensor::of(vec![1, 0, 0, 1], vec![2, 2]).unwrap().sum(), 2);
    /// assert!(Tensor::of(vec![1, 0, 0], vec![2, 2]).is_err());
    /// ```
    pub fn of(data: Vec<u8>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::U8(data), shape)
    }
    fn wrap(data: Buf, shape: Vec<usize>) -> Result<Tensor> {
        let size: usize = shape.iter().product();
        if data.len() != size {
            return shape_error(format!(
                "tensor data holds {} elements, shape {shape:?} wants {size}.",
                data.len()
            ));
        }
        Ok(Tensor { data, shape })
    }
    /// Wraps a u8 vector as a tensor of the shape, the same door as [`Tensor::of`].
    ///
    /// # Errors
    ///
    /// Errs when the data length is not the shape's product.
    pub fn u8(data: Vec<u8>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::U8(data), shape)
    }
    /// Wraps a u16 vector as a tensor of the shape.
    ///
    /// # Errors
    ///
    /// Errs when the data length is not the shape's product.
    ///
    /// ```
    /// use mrly_core::tensor::{Dtype, Tensor};
    /// let t = Tensor::u16(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    /// assert_eq!((t.dtype(), t.sum()), (Dtype::U16, 10));
    /// assert!(Tensor::u16(vec![1, 2, 3], vec![2, 2]).is_err());
    /// ```
    pub fn u16(data: Vec<u16>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::U16(data), shape)
    }
    /// Wraps a u32 vector as a tensor of the shape.
    ///
    /// # Errors
    ///
    /// Errs when the data length is not the shape's product.
    pub fn u32(data: Vec<u32>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::U32(data), shape)
    }
    /// Wraps an i32 vector as a tensor of the shape.
    ///
    /// # Errors
    ///
    /// Errs when the data length is not the shape's product.
    pub fn i32(data: Vec<i32>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::I32(data), shape)
    }
    /// Returns the element width.
    pub fn dtype(&self) -> Dtype {
        self.data.dtype()
    }
    /// Returns the number of elements.
    pub fn size(&self) -> usize {
        self.data.len()
    }
    /// Returns the elements as bytes.
    ///
    /// # Errors
    ///
    /// Errs when the tensor is wider than one byte.
    pub fn bytes(&self) -> Result<&[u8]> {
        match &self.data {
            Buf::U8(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u8.", other.dtype())),
        }
    }
    /// Returns the elements as mutable bytes.
    ///
    /// # Errors
    ///
    /// Errs when the tensor is wider than one byte.
    pub fn bytes_mut(&mut self) -> Result<&mut [u8]> {
        match &mut self.data {
            Buf::U8(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u8.", other.dtype())),
        }
    }
    /// Returns the elements as u16s.
    ///
    /// # Errors
    ///
    /// Errs when the tensor is not two bytes wide.
    pub fn u16s(&self) -> Result<&[u16]> {
        match &self.data {
            Buf::U16(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u16.", other.dtype())),
        }
    }
    /// Returns the elements as u32s.
    ///
    /// # Errors
    ///
    /// Errs when the tensor is not four unsigned bytes wide.
    pub fn u32s(&self) -> Result<&[u32]> {
        match &self.data {
            Buf::U32(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u32.", other.dtype())),
        }
    }
    /// Returns the elements as i32s.
    ///
    /// # Errors
    ///
    /// Errs when the tensor is not four signed bytes wide.
    pub fn i32s(&self) -> Result<&[i32]> {
        match &self.data {
            Buf::I32(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not i32.", other.dtype())),
        }
    }
    /// Returns the element at a flat index, which must be below the size like a slice index.
    pub fn at(&self, flat: usize) -> i64 {
        self.data.at(flat)
    }
    /// Writes the element at a flat index, which must be below the size like a slice index.
    pub fn put(&mut self, flat: usize, value: i64) {
        self.data.put(flat, value);
    }
    /// Returns the sum of all elements.
    pub fn sum(&self) -> u64 {
        (0..self.size()).map(|i| self.data.at(i) as u64).sum()
    }
    /// Folds a multi-index into its flat index.
    pub fn index(&self, multi: &[usize]) -> usize {
        let s = strides(&self.shape);
        multi.iter().zip(&s).map(|(m, st)| m * st).sum()
    }
    fn multi(&self, flat: usize) -> Vec<usize> {
        unravel(flat, &self.shape)
    }
    fn flat(&self, multi: &[usize]) -> Result<usize> {
        if multi.len() != self.shape.len() {
            return shape_error(format!(
                "index {multi:?} has {} axes, the tensor has {}.",
                multi.len(),
                self.shape.len()
            ));
        }
        if multi.iter().zip(&self.shape).any(|(&i, &n)| i >= n) {
            return shape_error(format!(
                "index {multi:?} is past the shape {:?}.",
                self.shape
            ));
        }
        Ok(self.index(multi))
    }
    fn axis(&self, axis: usize) -> Result<usize> {
        if axis >= self.shape.len() {
            return value_error(format!(
                "axis {axis} is past the tensor's rank {}.",
                self.shape.len()
            ));
        }
        Ok(axis)
    }
    /// Returns the byte at a multi-index.
    ///
    /// # Errors
    ///
    /// Errs when the index misses the shape or the tensor is wider than one byte.
    pub fn get(&self, multi: &[usize]) -> Result<u8> {
        let flat = self.flat(multi)?;
        Ok(self.bytes()?[flat])
    }
    /// Writes the byte at a multi-index.
    ///
    /// # Errors
    ///
    /// Errs when the index misses the shape or the tensor is wider than one byte.
    pub fn set(&mut self, multi: &[usize], value: u8) -> Result<()> {
        let flat = self.flat(multi)?;
        self.bytes_mut()?[flat] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn of(data: Vec<u8>, shape: Vec<usize>) -> Tensor {
        Tensor::of(data, shape).unwrap()
    }
    #[test]
    fn refuses_of() {
        assert!(Tensor::of(vec![1, 2, 3], vec![2, 2]).is_err());
        assert!(Tensor::of(vec![1, 2, 3, 4, 5], vec![2, 2]).is_err());
        assert!(Tensor::of(vec![1], vec![]).is_ok());
        assert!(Tensor::of(vec![], vec![0, 3]).is_ok());
    }
    #[test]
    fn refuses_get_and_set() {
        let mut a = of(vec![1, 2, 3, 4], vec![2, 2]);
        assert!(a.get(&[0]).is_err());
        assert!(a.get(&[0, 0, 0]).is_err());
        assert!(a.get(&[2, 0]).is_err());
        assert!(a.get(&[0, 2]).is_err());
        assert!(a.set(&[1], 9).is_err());
        assert!(a.set(&[0, 2], 9).is_err());
        let mut wide = Tensor::typed(vec![2, 2], Dtype::U16);
        assert!(wide.get(&[0, 0]).is_err());
        assert!(wide.set(&[0, 0], 9).is_err());
    }
    #[test]
    fn refuses_bytes_of_a_wider_tensor() {
        for dtype in [Dtype::U16, Dtype::U32, Dtype::I32] {
            let mut wide = Tensor::typed(vec![2, 2], dtype);
            assert!(wide.bytes().is_err());
            assert!(wide.bytes_mut().is_err());
        }
        assert!(Tensor::new(vec![2, 2]).bytes().is_ok());
    }
    #[test]
    fn typed_constructors_refuse_a_wrong_length() {
        assert!(Tensor::u8(vec![1, 2, 3], vec![2, 2]).is_err());
        assert!(Tensor::u16(vec![1, 2, 3], vec![2, 2]).is_err());
        assert!(Tensor::u32(vec![1, 2, 3], vec![2, 2]).is_err());
        assert!(Tensor::i32(vec![1, 2, 3], vec![2, 2]).is_err());
        assert_eq!(Tensor::u8(vec![1, 2, 3, 4], vec![2, 2]).unwrap().sum(), 10);
    }
    #[test]
    fn typed_slices_read_their_own_width() {
        assert_eq!(
            Tensor::u8(vec![1, 2], vec![2]).unwrap().bytes().unwrap(),
            [1, 2]
        );
        assert_eq!(
            Tensor::u16(vec![1, 2], vec![2]).unwrap().u16s().unwrap(),
            [1, 2]
        );
        assert_eq!(
            Tensor::u32(vec![1, 2], vec![2]).unwrap().u32s().unwrap(),
            [1, 2]
        );
        assert_eq!(
            Tensor::i32(vec![-1, 2], vec![2]).unwrap().i32s().unwrap(),
            [-1, 2]
        );
        assert!(Tensor::u8(vec![1, 2], vec![2]).unwrap().u16s().is_err());
    }
    #[test]
    fn serde_round_trip() {
        for tensor in [
            Tensor::u8(vec![1, 0, 0, 1], vec![2, 2]).unwrap(),
            Tensor::u16(vec![1, 300], vec![2]).unwrap(),
            Tensor::u32(vec![70000], vec![1]).unwrap(),
            Tensor::i32(vec![-7, 7], vec![2]).unwrap(),
        ] {
            let text = serde_json::to_string(&tensor).unwrap();
            assert_eq!(tensor, serde_json::from_str::<Tensor>(&text).unwrap());
        }
        let text = serde_json::to_string(&Tensor::u16(vec![1, 300], vec![2]).unwrap()).unwrap();
        assert_eq!(text, r#"{"data":{"U16":[1,300]},"shape":[2]}"#);
    }
}
