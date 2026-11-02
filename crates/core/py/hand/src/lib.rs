use mrly_core::cell::Cell;
use mrly_core::colors::Color;
use mrly_core::error::{Error, Result};
use mrly_core::image::Image;
use mrly_core::rng::Rng;
use mrly_core::tensor::{Dtype, Tensor};
use numpy::ndarray::{ArrayD, IxDyn};
use numpy::{Element, IntoPyArray, PyReadonlyArrayDyn, PyReadwriteArrayDyn, PyUntypedArrayMethods};
use pyo3::exceptions::{PyException, PyOverflowError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use pyo3::{Borrowed, IntoPyObjectExt};
use pythonize::{depythonize, pythonize};
use serde::Serialize;
use serde::de::DeserializeOwned;

// ERRORS

pub fn py_error(error: Error) -> PyErr {
    match error {
        Error::Overflow(message) => PyOverflowError::new_err(message),
        other => PyValueError::new_err(other.to_string()),
    }
}

pub fn ok<T>(result: Result<T>) -> PyResult<T> {
    result.map_err(py_error)
}

pub fn bad(message: impl Into<String>) -> PyErr {
    PyValueError::new_err(message.into())
}

// TENSOR

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

pub fn tensor_into_py<'py>(py: Python<'py>, tensor: &Tensor) -> PyResult<Bound<'py, PyAny>> {
    let shape = &tensor.shape;
    match tensor.dtype() {
        Dtype::U8 => owned(py, shape, ok(tensor.bytes())?.to_vec()),
        Dtype::U16 => owned(py, shape, ok(tensor.u16s())?.to_vec()),
        Dtype::U32 => owned(py, shape, ok(tensor.u32s())?.to_vec()),
        Dtype::I32 => owned(py, shape, ok(tensor.i32s())?.to_vec()),
    }
}

fn copy_back<T: Element + Copy>(obj: &Bound<'_, PyAny>, data: &[T]) -> PyResult<()> {
    let mut view = obj
        .extract::<PyReadwriteArrayDyn<T>>()
        .map_err(|_| bad("a mutated tensor writes back into a writable array of its own dtype."))?;
    let slot = view
        .as_slice_mut()
        .map_err(|_| bad("a mutated tensor writes back into a C-contiguous array."))?;
    if slot.len() != data.len() {
        return Err(bad("a mutated tensor keeps its shape."));
    }
    slot.copy_from_slice(data);
    Ok(())
}

pub fn tensor_write_back(obj: &Bound<'_, PyAny>, tensor: &Tensor) -> PyResult<()> {
    match tensor.dtype() {
        Dtype::U8 => copy_back(obj, ok(tensor.bytes())?),
        Dtype::U16 => copy_back(obj, ok(tensor.u16s())?),
        Dtype::U32 => copy_back(obj, ok(tensor.u32s())?),
        Dtype::I32 => copy_back(obj, ok(tensor.i32s())?),
    }
}

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

pub struct PyCell(pub Cell);

pub fn field<'py>(obj: &Bound<'py, PyAny>, name: &str) -> Option<Bound<'py, PyAny>> {
    match obj.get_item(name) {
        Ok(value) if !value.is_none() => Some(value),
        _ => None,
    }
}

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

pub fn cell_write_back(obj: &Bound<'_, PyAny>, cell: Cell) -> PyResult<()> {
    let fresh = cell_into_py(obj.py(), cell)?;
    for key in ["types", "colors", "tags"] {
        obj.set_item(key, fresh.get_item(key)?)?;
    }
    Ok(())
}

pub fn ndim(obj: &Bound<'_, PyAny>) -> PyResult<usize> {
    if let Ok(dict) = obj.cast::<PyDict>() {
        return match dict.get_item("types")? {
            Some(types) => ndim(&types),
            None => Err(bad("a cell wants a \"types\" array.")),
        };
    }
    if let Ok(list) = obj.cast::<PyList>() {
        return match list.get_item(0) {
            Ok(first) => ndim(&first),
            Err(_) => Err(bad("a dispatch wants at least one cell.")),
        };
    }
    if let Ok(tuple) = obj.cast::<PyTuple>() {
        return match tuple.get_item(0) {
            Ok(first) => ndim(&first),
            Err(_) => Err(bad("a dispatch wants at least one cell.")),
        };
    }
    obj.getattr("ndim")
        .and_then(|rank| rank.extract::<usize>())
        .map_err(|_| bad("a dispatch wants a numpy array, a cell dict or a list of them."))
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

// IMAGE

pub struct PyImage(pub Image);

impl<'py> IntoPyObject<'py> for PyImage {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let Image {
            width,
            height,
            colors,
        } = self.0;
        let flat: Vec<u8> = colors.into_iter().flatten().collect();
        owned(py, &[height, width, 4], flat)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyImage {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyImage> {
        let view = obj
            .extract::<PyReadonlyArrayDyn<u8>>()
            .map_err(|_| bad("an image wants a uint8 array."))?;
        let (height, width) = match view.shape() {
            &[height, width, 4] => (height, width),
            _ => return Err(bad("an image wants an array of shape (h, w, 4).")),
        };
        let data = view
            .as_slice()
            .map_err(|_| bad("an image wants a C-contiguous array."))?;
        let colors = data
            .chunks_exact(4)
            .map(|rgba| [rgba[0], rgba[1], rgba[2], rgba[3]])
            .collect();
        Ok(PyImage(ok(Image::new(width, height, colors))?))
    }
}

// COLOR

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

// PIXELS

pub struct PyRgba(pub [u8; 4]);

impl<'py> IntoPyObject<'py> for PyRgba {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let [r, g, b, a] = self.0;
        (r, g, b, a).into_bound_py_any(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyRgba {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyRgba> {
        let rgba = obj
            .extract::<[u8; 4]>()
            .map_err(|_| bad("a pixel wants four channel bytes."))?;
        Ok(PyRgba(rgba))
    }
}

pub struct PyPixels(pub Vec<[u8; 4]>);

impl<'py> IntoPyObject<'py> for PyPixels {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let count = self.0.len();
        let flat: Vec<u8> = self.0.into_iter().flatten().collect();
        owned(py, &[count, 4], flat)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyPixels {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<PyPixels> {
        if let Ok(view) = obj.extract::<PyReadonlyArrayDyn<u8>>() {
            if view.shape().last() != Some(&4) {
                return Err(bad("pixels want a trailing axis of four channels."));
            }
            let data = view
                .as_slice()
                .map_err(|_| bad("pixels want a C-contiguous array."))?;
            return Ok(PyPixels(
                data.chunks_exact(4)
                    .map(|rgba| [rgba[0], rgba[1], rgba[2], rgba[3]])
                    .collect(),
            ));
        }
        let listed = obj
            .extract::<Vec<PyRgba>>()
            .map_err(|_| bad("pixels want an (n, 4) uint8 array or a list of four-tuples."))?;
        Ok(PyPixels(listed.into_iter().map(|p| p.0).collect()))
    }
}

// SERDE

pub struct PySerde<T>(pub T);

pub fn serde_into_py<'py, T: Serialize + ?Sized>(
    py: Python<'py>,
    value: &T,
) -> PyResult<Bound<'py, PyAny>> {
    Ok(pythonize(py, value)?)
}

pub fn serde_from_py<T: DeserializeOwned>(obj: &Bound<'_, PyAny>) -> PyResult<T> {
    let py = obj.py();
    depythonize(obj).map_err(|error| {
        let error = PyErr::from(error);
        if error.get_type(py).is(py.get_type::<PyException>())
            || error.is_instance_of::<PyOverflowError>(py)
        {
            bad(error.value(py).to_string())
        } else {
            error
        }
    })
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
#[pyclass(name = "Rng", module = "mrly.core")]
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
    /// Draws one item of the sequence, the same draw as Rust's choice.
    pub fn choice<'py>(&mut self, seq: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let indices: Vec<usize> = (0..seq.len()?).collect();
        let index = *ok(self.0.choice(&indices))?;
        seq.get_item(index)
    }
    /// Shuffles the list in place, the same permutation as Rust's shuffle.
    pub fn shuffle(&mut self, seq: &Bound<'_, PyList>) -> PyResult<()> {
        let mut order: Vec<usize> = (0..seq.len()).collect();
        self.0.shuffle(&mut order);
        let items: Vec<Bound<'_, PyAny>> = order
            .into_iter()
            .map(|index| seq.get_item(index))
            .collect::<PyResult<_>>()?;
        for (at, item) in items.into_iter().enumerate() {
            seq.set_item(at, item)?;
        }
        Ok(())
    }
    /// Returns the stream's state as text, the way a stream crosses into another unit.
    pub fn state(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(|error| bad(error.to_string()))
    }
    /// Reads a state back into the stream, the way a stream crosses back out of another unit.
    pub fn restore(&mut self, state: &str) -> PyResult<()> {
        self.0 = serde_json::from_str(state).map_err(|error| bad(error.to_string()))?;
        Ok(())
    }
}

pub fn rng_from_py(obj: &Bound<'_, PyAny>) -> PyResult<Rng> {
    let state: String = obj
        .call_method0("state")
        .and_then(|state| state.extract())
        .map_err(|_| bad("an Rng was wanted here."))?;
    serde_json::from_str(&state).map_err(|error| bad(error.to_string()))
}

pub fn rng_write_back(obj: &Bound<'_, PyAny>, rng: &Rng) -> PyResult<()> {
    let state = serde_json::to_string(rng).map_err(|error| bad(error.to_string()))?;
    obj.call_method1("restore", (state,))?;
    Ok(())
}
