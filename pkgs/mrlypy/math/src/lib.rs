mod fractal;
mod three;
mod two;

use pyo3::prelude::*;

#[pyfunction]
fn seed(n: u64) {
    mrlycore::state::seed(n);
}

#[pymodule]
#[pyo3(name = "mrlymath")]
fn math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(seed, m)?)?;
    two::register(m.py(), m)?;
    three::register(m.py(), m)?;
    fractal::register(m.py(), m)?;
    Ok(())
}
