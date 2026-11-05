use crate::bang::Code;
use crate::cell::models::CellNd;
use crate::six::{Cell6d, Orientation, Projection};
pub use mrly_core::hand::py::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{Borrowed, IntoPyObjectExt};

pub struct PyCellNd<const N: usize>(pub CellNd<N>);

pub type PyCell2d = PyCellNd<2>;

pub type PyCell3d = PyCellNd<3>;

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

pub struct PyCell6d(pub Cell6d);

pub fn cell_6d_into_py<'py>(py: Python<'py>, cell: Cell6d) -> PyResult<Bound<'py, PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("cell", cell_into_py(py, cell.cell.cell)?)?;
    dict.set_item("projection", serde_into_py(py, &cell.projection)?)?;
    dict.set_item("orientation", serde_into_py(py, &cell.orientation)?)?;
    dict.set_item("start", cell.start)?;
    Ok(dict.into_any())
}

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
