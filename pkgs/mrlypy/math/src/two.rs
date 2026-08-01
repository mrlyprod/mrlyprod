use mrlycore::colors::Color;
use mrlycore::enums::Mode;
use mrlycore::MrlyError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::collections::HashMap;

fn to_py_err(e: MrlyError) -> PyErr {
    PyValueError::new_err(e.to_string())
}

fn color_from_entry(entry: &[u8]) -> PyResult<Color> {
    match entry {
        [r, g, b] => Ok(Color::rgb(*r, *g, *b)),
        [r, g, b, a] => Ok(Color::rgba(*r, *g, *b, *a)),
        _ => Err(PyValueError::new_err(
            "each palette entry must be [r, g, b] or [r, g, b, a].",
        )),
    }
}

fn mapping_of(
    mapping: Option<HashMap<u8, Vec<Vec<u8>>>>,
) -> PyResult<Option<HashMap<u8, Vec<Color>>>> {
    let Some(dict) = mapping else { return Ok(None) };
    let mut out = HashMap::new();
    for (key, entries) in dict {
        let colors: Vec<Color> = entries
            .iter()
            .map(|entry| color_from_entry(entry))
            .collect::<PyResult<_>>()?;
        out.insert(key, colors);
    }
    Ok(Some(out))
}

fn mode_of(mode: Option<&str>) -> PyResult<Option<Mode>> {
    let Some(name) = mode else { return Ok(None) };
    let mode = match name {
        "type" => Mode::Type,
        "tag" => Mode::Tag,
        "index" => Mode::Index,
        "enumerate" => Mode::Enumerate,
        "random" => Mode::Random,
        "row" => Mode::Row,
        "column" => Mode::Column,
        "depth" => Mode::Depth,
        _ => return Err(PyValueError::new_err(format!("unknown mode {name:?}."))),
    };
    Ok(Some(mode))
}

#[pyclass]
pub struct Cell2d {
    inner: mrlymath::two::Cell2d,
}

#[pymethods]
impl Cell2d {
    #[getter]
    fn width(&self) -> usize {
        self.inner.width()
    }

    #[getter]
    fn height(&self) -> usize {
        self.inner.height()
    }

    fn invert(&self) -> Cell2d {
        Cell2d {
            inner: self.inner.clone().invert(),
        }
    }

    fn fractal(&self, level: usize) -> PyResult<Cell2d> {
        Ok(Cell2d {
            inner: self.inner.clone().fractal(level).map_err(to_py_err)?,
        })
    }

    #[pyo3(signature = (mapping=None, mode=None))]
    fn paint(
        &self,
        mapping: Option<HashMap<u8, Vec<Vec<u8>>>>,
        mode: Option<&str>,
    ) -> PyResult<Cell2d> {
        let mapping = mapping_of(mapping)?;
        let mode = mode_of(mode)?;
        Ok(Cell2d {
            inner: mrlymath::two::paint(self.inner.clone(), mapping.as_ref(), mode),
        })
    }

    #[pyo3(signature = (scale=1))]
    fn png(&self, py: Python<'_>, scale: usize) -> PyResult<PyObject> {
        let bytes = mrlymath::two::png(&self.inner, scale).map_err(to_py_err)?;
        Ok(PyBytes::new_bound(py, &bytes).into())
    }

    fn to_lists(&self) -> Vec<Vec<u8>> {
        mrlymath::two::to_lists(&self.inner)
    }

    fn to_strings(&self) -> Vec<String> {
        mrlymath::two::to_strings(&self.inner)
    }

    fn to_json(&self) -> String {
        mrlymath::two::to_json(&self.inner)
    }
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn carpet(number: usize, level: usize) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::carpet(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn net(number: usize, level: usize) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::net(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn htree(number: usize, level: usize) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::htree(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn vtree(number: usize, level: usize) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::vtree(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1))]
fn void(number: usize, level: usize) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::void(number, level).map_err(to_py_err)?,
    })
}

#[pyfunction]
#[pyo3(signature = (number, level=1, density=0.5))]
fn noise(number: usize, level: usize, density: f64) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::noise(number, level, density).map_err(to_py_err)?,
    })
}

#[pyfunction]
fn from_lists(lists: Vec<Vec<u8>>) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::from_lists(&lists).map_err(to_py_err)?,
    })
}

#[pyfunction]
fn from_strings(rows: Vec<String>) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::from_strings(&rows).map_err(to_py_err)?,
    })
}

#[pyfunction]
fn from_json(text: &str) -> PyResult<Cell2d> {
    Ok(Cell2d {
        inner: mrlymath::two::from_json(text).map_err(to_py_err)?,
    })
}

pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let two = PyModule::new_bound(py, "two")?;
    two.add_class::<Cell2d>()?;
    two.add_function(wrap_pyfunction!(carpet, &two)?)?;
    two.add_function(wrap_pyfunction!(net, &two)?)?;
    two.add_function(wrap_pyfunction!(htree, &two)?)?;
    two.add_function(wrap_pyfunction!(vtree, &two)?)?;
    two.add_function(wrap_pyfunction!(void, &two)?)?;
    two.add_function(wrap_pyfunction!(noise, &two)?)?;
    two.add_function(wrap_pyfunction!(from_lists, &two)?)?;
    two.add_function(wrap_pyfunction!(from_strings, &two)?)?;
    two.add_function(wrap_pyfunction!(from_json, &two)?)?;
    parent.add_submodule(&two)?;
    Ok(())
}
