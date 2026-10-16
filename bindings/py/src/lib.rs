mod doors;
pub mod hand;

use pyo3::prelude::*;

#[pymodule]
fn _mrly(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    doors::init(py, module)
}
