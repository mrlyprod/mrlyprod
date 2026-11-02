mod doors;
pub use mrly_math_py_hand as hand;

use pyo3::prelude::*;

#[pymodule]
fn _math(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    doors::init(py, module)
}
