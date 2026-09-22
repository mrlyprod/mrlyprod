pub mod hand;

use hand::{ok, PyCell2d, PyCell3d, PyCell6d, PyCellNd, PyCode, PyColor, PyRng, PySerde, PyTensor};
use mrlyrs::core::cell::Mode;
use mrlyrs::core::colors::Color;
use mrlyrs::{gen, math, num};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pymodule]
fn _mrlypy(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyRng>()?;
    module.add_function(wrap_pyfunction!(rot90, module)?)?;
    module.add_function(wrap_pyfunction!(carpet, module)?)?;
    module.add_function(wrap_pyfunction!(carpet_3d, module)?)?;
    module.add_function(wrap_pyfunction!(census, module)?)?;
    module.add_function(wrap_pyfunction!(paint, module)?)?;
    module.add_function(wrap_pyfunction!(noise, module)?)?;
    module.add_function(wrap_pyfunction!(cut_design, module)?)?;
    module.add_function(wrap_pyfunction!(six_census, module)?)?;
    module.add_function(wrap_pyfunction!(background, module)?)?;
    module.add_function(wrap_pyfunction!(gcd, module)?)?;
    module.add_function(wrap_pyfunction!(from_hex, module)?)?;
    module.add_function(wrap_pyfunction!(bernoulli, module)?)?;
    Ok(())
}

// PROOF

#[pyfunction]
fn rot90(tensor: PyTensor, k: usize, axes: (usize, usize)) -> PyResult<PyTensor> {
    Ok(PyTensor(ok(tensor.0.rot90(k, axes))?))
}

#[pyfunction]
fn carpet(number: usize, level: usize) -> PyResult<PyCell2d> {
    Ok(PyCellNd(ok(math::two::carpet(number, level))?))
}

#[pyfunction]
fn carpet_3d(number: usize, level: usize) -> PyResult<PyCell3d> {
    Ok(PyCellNd(ok(math::three::carpet(number, level))?))
}

#[pyfunction]
fn census(cell: PyCell2d) -> PyResult<PySerde<math::two::Census>> {
    Ok(PySerde(ok(math::two::census(&cell.0))?))
}

#[pyfunction]
#[pyo3(signature = (cell, custom = None, mode = None, rng = None))]
fn paint(
    cell: PyCell2d,
    custom: Option<PySerde<HashMap<u8, Vec<Color>>>>,
    mode: Option<PySerde<Mode>>,
    rng: Option<&mut PyRng>,
) -> PyResult<PyCell2d> {
    let custom = custom.map(|wrapped| wrapped.0);
    let mode = mode.map(|wrapped| wrapped.0);
    let rng = rng.map(|wrapped| &mut wrapped.0);
    Ok(PyCellNd(ok(math::two::paint(
        cell.0,
        custom.as_ref(),
        mode,
        rng,
    ))?))
}

#[pyfunction]
fn noise(number: usize, level: usize, density: f64, rng: &mut PyRng) -> PyResult<PyCell2d> {
    Ok(PyCellNd(ok(math::two::noise(
        number, level, density, &mut rng.0,
    ))?))
}

#[pyfunction]
fn cut_design(code: PyCode, number: usize, level: usize, base: usize) -> PyResult<PyCell6d> {
    Ok(PyCell6d(ok(math::six::cut_design(
        code.0, number, level, base,
    ))?))
}

#[pyfunction]
fn six_census(cell: PyCell6d, include_grid: bool) -> PySerde<math::six::Census> {
    PySerde(math::six::census(&cell.0, include_grid))
}

#[pyfunction]
fn background(seed: u64, width: usize, height: usize) -> PyResult<Vec<u8>> {
    ok(gen::background(seed, width, height))
}

#[pyfunction]
fn gcd(a: u128, b: u128) -> u128 {
    num::factor::gcd(a, b)
}

#[pyfunction]
fn from_hex(hex: &str) -> PyResult<PyColor> {
    Ok(PyColor(ok(Color::from_hex(hex))?))
}

#[pyfunction]
fn bernoulli(count: usize) -> PyResult<Vec<(i128, i128)>> {
    ok(num::series::bernoulli(count))
}
