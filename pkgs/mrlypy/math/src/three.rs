use mrlycore::MrlyError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn to_py_err(e: MrlyError) -> PyErr {
    PyValueError::new_err(e.to_string())
}

#[pyclass]
pub struct Cell3d {
    inner: mrlymath::three::Cell3d,
}

#[pymethods]
impl Cell3d {
    #[getter]
    fn width(&self) -> usize {
        self.inner.width()
    }

    #[getter]
    fn height(&self) -> usize {
        self.inner.height()
    }

    #[getter]
    fn depth(&self) -> usize {
        self.inner.depth()
    }

    fn volume(&self) -> usize {
        mrlymath::three::census::volume(&self.inner)
    }

    fn surface(&self) -> u128 {
        mrlymath::three::census::surface(&self.inner)
    }

    fn obj(&self) -> String {
        mrlymath::three::obj(&self.inner)
    }

    fn to_lists(&self) -> Vec<Vec<Vec<u8>>> {
        mrlymath::three::to_lists(&self.inner)
    }

    fn to_json(&self) -> String {
        mrlymath::three::to_json(&self.inner)
    }
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn carpet(number: usize, level: usize) -> PyResult<Cell3d> {
    Ok(Cell3d {
        inner: mrlymath::three::carpet(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn net(number: usize, level: usize) -> PyResult<Cell3d> {
    Ok(Cell3d {
        inner: mrlymath::three::net(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn xtree(number: usize, level: usize) -> PyResult<Cell3d> {
    Ok(Cell3d {
        inner: mrlymath::three::xtree(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn ztree(number: usize, level: usize) -> PyResult<Cell3d> {
    Ok(Cell3d {
        inner: mrlymath::three::ztree(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
fn from_lists(lists: Vec<Vec<Vec<u8>>>) -> PyResult<Cell3d> {
    Ok(Cell3d {
        inner: mrlymath::three::from_lists(&lists).map_err(to_py_err)?,
    })
}

#[pyfunction]
fn from_json(text: &str) -> PyResult<Cell3d> {
    Ok(Cell3d {
        inner: mrlymath::three::from_json(text).map_err(to_py_err)?,
    })
}

pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let three = PyModule::new_bound(py, "three")?;
    three.add_class::<Cell3d>()?;
    three.add_function(wrap_pyfunction!(carpet, &three)?)?;
    three.add_function(wrap_pyfunction!(net, &three)?)?;
    three.add_function(wrap_pyfunction!(xtree, &three)?)?;
    three.add_function(wrap_pyfunction!(ztree, &three)?)?;
    three.add_function(wrap_pyfunction!(from_lists, &three)?)?;
    three.add_function(wrap_pyfunction!(from_json, &three)?)?;
    parent.add_submodule(&three)?;
    Ok(())
}
