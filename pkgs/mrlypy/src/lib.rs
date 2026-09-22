mod gen;
pub mod hand;

use pyo3::prelude::*;

#[pymodule]
fn _mrlypy(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    gen::init(py, module)
}
