mod doors;
pub use mrly_math::hand::py as hand;

use pyo3::prelude::*;

#[pymodule]
fn _tile(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    doors::init(py, module)
}
