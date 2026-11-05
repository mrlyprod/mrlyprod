use super::error::{Result, shape_error, value_error};
use serde::{Deserialize, Serialize};

mod filter;
mod shape;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn u8(data: Vec<u8>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::U8(data), shape)
    }
    pub fn u16(data: Vec<u16>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::U16(data), shape)
    }
    pub fn u32(data: Vec<u32>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::U32(data), shape)
    }
    pub fn i32(data: Vec<i32>, shape: Vec<usize>) -> Result<Tensor> {
        Tensor::wrap(Buf::I32(data), shape)
    }
    pub fn dtype(&self) -> Dtype {
        self.data.dtype()
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn bytes(&self) -> Result<&[u8]> {
        match &self.data {
            Buf::U8(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u8.", other.dtype())),
        }
    }
    pub fn bytes_mut(&mut self) -> Result<&mut [u8]> {
        match &mut self.data {
            Buf::U8(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u8.", other.dtype())),
        }
    }
    pub fn u16s(&self) -> Result<&[u16]> {
        match &self.data {
            Buf::U16(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u16.", other.dtype())),
        }
    }
    pub fn u32s(&self) -> Result<&[u32]> {
        match &self.data {
            Buf::U32(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not u32.", other.dtype())),
        }
    }
    pub fn i32s(&self) -> Result<&[i32]> {
        match &self.data {
            Buf::I32(v) => Ok(v),
            other => shape_error(format!("tensor is {:?}, not i32.", other.dtype())),
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
    pub fn get(&self, multi: &[usize]) -> Result<u8> {
        let flat = self.flat(multi)?;
        Ok(self.bytes()?[flat])
    }
    pub fn set(&mut self, multi: &[usize], value: u8) -> Result<()> {
        let flat = self.flat(multi)?;
        self.bytes_mut()?[flat] = value;
        Ok(())
    }
}
