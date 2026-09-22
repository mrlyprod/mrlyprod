use mrlyrs::core::cell::Cell;
use mrlyrs::core::colors::Color;
use mrlyrs::core::error::{Error, Result};
use mrlyrs::core::rng::Rng;
use mrlyrs::core::tensor::{Dtype, Tensor};
use mrlyrs::math::bang::Code;
use mrlyrs::math::cell::models::CellNd;
use mrlyrs::math::six::{Cell6d, Orientation, Projection};
use numpy::ndarray::{ArrayD, IxDyn};
use numpy::{Element, IntoPyArray, PyReadonlyArrayDyn, PyUntypedArrayMethods};
use pyo3::exceptions::{PyOverflowError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{Borrowed, IntoPyObjectExt};
use pythonize::{depythonize, pythonize};
use serde::de::DeserializeOwned;
use serde::Serialize;

// ERRORS

/// Maps one crate error onto its Python class, overflow apart and everything else a ValueError.
pub fn py_error(error: Error) -> PyErr {
    match error {
        Error::Overflow(message) => PyOverflowError::new_err(message),
        other => PyValueError::new_err(other.to_string()),
    }
}

/// Unwraps a crate result into a Python result.
pub fn ok<T>(result: Result<T>) -> PyResult<T> {
    result.map_err(py_error)
}

fn bad(message: impl Into<String>) -> PyErr {
    PyValueError::new_err(message.into())
}

// TENSOR

/// A tensor crossing as a numpy array of its own dtype.
pub struct PyTensor(pub Tensor);

fn owned<'py, T: Element>(
    py: Python<'py>,
    shape: &[usize],
    data: Vec<T>,
) -> PyResult<Bound<'py, PyAny>> {
    let array =
        ArrayD::from_shape_vec(IxDyn(shape), data).map_err(|error| bad(error.to_string()))?;
    Ok(array.into_pyarray(py).into_any())
}

fn read<T: Element + Copy>(obj: &Bound<'_, PyAny>) -> Option<PyResult<(Vec<T>, Vec<usize>)>> {
    let view = obj.extract::<PyReadonlyArrayDyn<T>>().ok()?;
    let shape = view.shape().to_vec();
    Some(match view.as_slice() {
        Ok(data) => Ok((data.to_vec(), shape)),
        Err(_) => Err(bad("an array crossing into Rust must be C-contiguous.")),
    })
}

/// Builds a numpy array of the tensor's shape and dtype, the buffer moved into numpy.
pub fn tensor_into_py<'py>(py: Python<'py>, tensor: &Tensor) -> PyResult<Bound<'py, PyAny>> {
    let shape = &tensor.shape;
    match tensor.dtype() {
        Dtype::U8 => owned(py, shape, ok(tensor.bytes())?.to_vec()),
        Dtype::U16 => owned(py, shape, ok(tensor.u16s())?.to_vec()),
        Dtype::U32 => owned(py, shape, ok(tensor.u32s())?.to_vec()),
        Dtype::I32 => owned(py, shape, ok(tensor.i32s())?.to_vec()),
    }
}

/// Reads a C-contiguous uint8, uint16, uint32 or int32 array into a tensor.
pub fn tensor_from_py(obj: &Bound<'_, PyAny>) -> PyResult<Tensor> {
    if let Some(result) = read::<u8>(obj) {
        let (data, shape) = result?;
        return ok(Tensor::u8(data, shape));
    }
    if let Some(result) = read::<u16>(obj) {
        let (data, shape) = result?;
        return ok(Tensor::u16(data, shape));
    }
    if let Some(result) = read::<u32>(obj) {
        let (data, shape) = result?;
        return ok(Tensor::u32(data, shape));
    }
    if let Some(result) = read::<i32>(obj) {
        let (data, shape) = result?;
        return ok(Tensor::i32(data, shape));
    }
    Err(bad(
        "a tensor wants a uint8, uint16, uint32 or int32 numpy array.",
    ))
}

impl<'py> IntoPyObject<'py> for PyTensor {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        tensor_into_py(py, &self.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyTensor {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyTensor> {
        Ok(PyTensor(tensor_from_py(&obj)?))
    }
}

// CELL

/// A cell crossing as the dict of its types, colors and tags.
pub struct PyCell(pub Cell);

fn field<'py>(obj: &Bound<'py, PyAny>, name: &str) -> Option<Bound<'py, PyAny>> {
    match obj.get_item(name) {
        Ok(value) if !value.is_none() => Some(value),
        _ => None,
    }
}

/// Builds the `{types, colors, tags}` dict of a cell, each array moved into numpy.
pub fn cell_into_py<'py>(py: Python<'py>, cell: Cell) -> PyResult<Bound<'py, PyAny>> {
    let Cell {
        types,
        colors,
        tags,
    } = cell;
    let dict = PyDict::new(py);
    dict.set_item("types", tensor_into_py(py, &types)?)?;
    match colors {
        Some(colors) => {
            let mut shape = types.shape.clone();
            shape.push(4);
            let flat: Vec<u8> = colors.into_iter().flatten().collect();
            dict.set_item("colors", owned(py, &shape, flat)?)?;
        }
        None => dict.set_item("colors", py.None())?,
    }
    match tags {
        Some(tags) => dict.set_item("tags", tensor_into_py(py, &tags)?)?,
        None => dict.set_item("tags", py.None())?,
    }
    Ok(dict.into_any())
}

/// Reads a `{types, colors, tags}` dict into a cell.
pub fn cell_from_py(obj: &Bound<'_, PyAny>) -> PyResult<Cell> {
    let types = match field(obj, "types") {
        Some(value) => tensor_from_py(&value)?,
        None => return Err(bad("a cell wants a \"types\" array.")),
    };
    let colors = match field(obj, "colors") {
        Some(value) => {
            let view = value
                .extract::<PyReadonlyArrayDyn<u8>>()
                .map_err(|_| bad("cell colors want a uint8 array."))?;
            if view.shape().last() != Some(&4) {
                return Err(bad("cell colors want a trailing axis of four channels."));
            }
            let data = view
                .as_slice()
                .map_err(|_| bad("cell colors want a C-contiguous array."))?;
            if data.len() != types.size() * 4 {
                return Err(bad("cell colors want one rgba per cell."));
            }
            Some(
                data.chunks_exact(4)
                    .map(|rgba| [rgba[0], rgba[1], rgba[2], rgba[3]])
                    .collect(),
            )
        }
        None => None,
    };
    let tags = match field(obj, "tags") {
        Some(value) => Some(tensor_from_py(&value)?),
        None => None,
    };
    Ok(Cell {
        types,
        colors,
        tags,
    })
}

impl<'py> IntoPyObject<'py> for PyCell {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        cell_into_py(py, self.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyCell {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyCell> {
        Ok(PyCell(cell_from_py(&obj)?))
    }
}

// CELLND

/// An N-dimensional cell crossing as the same dict, N read off the types array.
pub struct PyCellNd<const N: usize>(pub CellNd<N>);

/// The two-dimensional crossing.
pub type PyCell2d = PyCellNd<2>;

/// The three-dimensional crossing.
pub type PyCell3d = PyCellNd<3>;

/// Reads the dict into an N-dimensional cell, erring when the types array is not N-dimensional.
pub fn cell_nd_from_py<const N: usize>(obj: &Bound<'_, PyAny>) -> PyResult<CellNd<N>> {
    let cell = cell_from_py(obj)?;
    let rank = cell.types.shape.len();
    if rank != N {
        return Err(bad(format!(
            "a {N}d cell wants a {N}d types array, got {rank}d."
        )));
    }
    Ok(CellNd { cell })
}

impl<'py, const N: usize> IntoPyObject<'py> for PyCellNd<N> {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        cell_into_py(py, self.0.cell)
    }
}

impl<'py, const N: usize> FromPyObject<'_, 'py> for PyCellNd<N> {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyCellNd<N>> {
        Ok(PyCellNd(cell_nd_from_py(&obj)?))
    }
}

// CELL6D

/// A hexagonal cell crossing as the dict of its four fields.
pub struct PyCell6d(pub Cell6d);

/// Builds the `{cell, projection, orientation, start}` dict of a hexagonal cell.
pub fn cell_6d_into_py<'py>(py: Python<'py>, cell: Cell6d) -> PyResult<Bound<'py, PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("cell", cell_into_py(py, cell.cell.cell)?)?;
    dict.set_item("projection", serde_into_py(py, &cell.projection)?)?;
    dict.set_item("orientation", serde_into_py(py, &cell.orientation)?)?;
    dict.set_item("start", cell.start)?;
    Ok(dict.into_any())
}

/// Reads a `{cell, projection, orientation, start}` dict into a hexagonal cell.
pub fn cell_6d_from_py(obj: &Bound<'_, PyAny>) -> PyResult<Cell6d> {
    let inner = match field(obj, "cell") {
        Some(value) => cell_nd_from_py::<2>(&value)?,
        None => return Err(bad("a 6d cell wants a \"cell\" dict.")),
    };
    let projection: Projection = match field(obj, "projection") {
        Some(value) => serde_from_py(&value)?,
        None => return Err(bad("a 6d cell wants a \"projection\".")),
    };
    let orientation: Orientation = match field(obj, "orientation") {
        Some(value) => serde_from_py(&value)?,
        None => return Err(bad("a 6d cell wants an \"orientation\".")),
    };
    let start: u8 = match field(obj, "start") {
        Some(value) => value.extract()?,
        None => return Err(bad("a 6d cell wants a \"start\".")),
    };
    Ok(Cell6d::new(inner, projection, orientation, start))
}

impl<'py> IntoPyObject<'py> for PyCell6d {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        cell_6d_into_py(py, self.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyCell6d {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyCell6d> {
        Ok(PyCell6d(cell_6d_from_py(&obj)?))
    }
}

// COLOR

/// A color crossing as the four-tuple of its channels.
pub struct PyColor(pub Color);

impl<'py> IntoPyObject<'py> for PyColor {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let color = self.0;
        (color.r, color.g, color.b, color.a).into_bound_py_any(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyColor {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyColor> {
        let (r, g, b, a) = obj
            .extract::<(u8, u8, u8, u8)>()
            .map_err(|_| bad("a color wants four channel bytes."))?;
        Ok(PyColor(Color::rgba(r, g, b, a)))
    }
}

// CODE

/// A design code crossing as a Python int.
pub struct PyCode(pub Code);

impl<'py> IntoPyObject<'py> for PyCode {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.0.get().into_bound_py_any(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyCode {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyCode> {
        let bits = obj
            .extract::<u128>()
            .map_err(|_| bad("a code wants a non-negative integer below 2^128."))?;
        Ok(PyCode(Code::from(bits)))
    }
}

// SERDE

/// Every other serde type crossing as plain Python data.
pub struct PySerde<T>(pub T);

/// Turns any serde value into plain Python data.
pub fn serde_into_py<'py, T: Serialize + ?Sized>(
    py: Python<'py>,
    value: &T,
) -> PyResult<Bound<'py, PyAny>> {
    Ok(pythonize(py, value)?)
}

/// Reads plain Python data back into any serde type.
pub fn serde_from_py<T: DeserializeOwned>(obj: &Bound<'_, PyAny>) -> PyResult<T> {
    Ok(depythonize(obj)?)
}

impl<'py, T: Serialize> IntoPyObject<'py> for PySerde<T> {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        serde_into_py(py, &self.0)
    }
}

impl<'py, T: DeserializeOwned> FromPyObject<'_, 'py> for PySerde<T> {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PySerde<T>> {
        Ok(PySerde(serde_from_py(&obj)?))
    }
}

// RNG

/// The seeded random stream, one class, passed wherever Rust takes a mutable stream.
#[pyclass(name = "Rng", module = "mrlypy")]
pub struct PyRng(pub Rng);

#[pymethods]
impl PyRng {
    /// Opens the stream of the seed.
    #[new]
    pub fn new(seed: u64) -> PyRng {
        PyRng(Rng::new(seed))
    }
    /// Draws a float at or above zero and below one.
    pub fn unit(&mut self) -> f64 {
        self.0.unit()
    }
    /// Draws an integer below n, or zero when n is zero.
    pub fn below(&mut self, n: usize) -> usize {
        self.0.below(n)
    }
    /// Draws an integer between lo and hi inclusive.
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        self.0.range(lo, hi)
    }
    /// Draws a fair coin flip.
    pub fn boolean(&mut self) -> bool {
        self.0.boolean()
    }
    /// Returns true with probability p.
    pub fn chance(&mut self, p: f64) -> bool {
        self.0.chance(p)
    }
    /// Draws amount distinct indices below length.
    pub fn sample_indices(&mut self, length: usize, amount: usize) -> Vec<usize> {
        self.0.sample_indices(length, amount)
    }
}
