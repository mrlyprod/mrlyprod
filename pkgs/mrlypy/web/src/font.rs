use pyo3::prelude::*;
use std::collections::BTreeMap;

#[pyfunction]
fn glyphs() -> BTreeMap<String, Vec<String>> {
    mrlyfont::map()
        .into_iter()
        .map(|(c, rows)| (c.to_string(), rows))
        .collect()
}

#[pyfunction]
fn descenders() -> Vec<String> {
    mrlyfont::DESCENDERS.iter().map(|c| c.to_string()).collect()
}

#[pyfunction]
fn json() -> String {
    mrlyfont::to_json(&mrlyfont::all())
}

pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let font = PyModule::new_bound(py, "font")?;
    font.add_function(wrap_pyfunction!(glyphs, &font)?)?;
    font.add_function(wrap_pyfunction!(descenders, &font)?)?;
    font.add_function(wrap_pyfunction!(json, &font)?)?;
    parent.add_submodule(&font)?;
    Ok(())
}
