mod graphics;

use mrly::os::kernel::{Call, Iden, Os};
use pyo3::prelude::*;
use serde_json::{json, Value};

fn build() -> Os {
    let mut os = Os::new(Iden::new("guest"));
    for app in mrly::net::registry::catalogue() {
        os = os.install(app);
    }
    os
}

fn loads(py: Python<'_>, text: String) -> PyResult<PyObject> {
    Ok(py
        .import_bound("json")?
        .call_method1("loads", (text,))?
        .unbind())
}

#[pyclass(unsendable)]
pub struct Handle {
    os: Os,
}

#[pyfunction]
fn boot() -> Handle {
    Handle { os: build() }
}

#[pyfunction]
fn act(py: Python<'_>, handle: &mut Handle, req: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let text: String = match req.extract::<String>() {
        Ok(s) => s,
        Err(_) => py
            .import_bound("json")?
            .call_method1("dumps", (req,))?
            .extract()?,
    };
    let parsed: Value = serde_json::from_str(&text).unwrap_or(json!({}));
    let verb = parsed["verb"].as_str().unwrap_or("").to_string();
    let args = if parsed["args"].is_object() {
        parsed["args"].clone()
    } else {
        json!({})
    };
    let mut call = Call::new(&verb, args);
    if let Some(now) = parsed["now"].as_i64() {
        call = call.at(now);
    }
    handle.os.act(call);
    loads(py, handle.os.frame().to_json().to_string())
}

#[pyfunction]
fn frame(py: Python<'_>, handle: &Handle) -> PyResult<PyObject> {
    loads(py, handle.os.frame().to_json().to_string())
}

#[pyfunction]
fn describe(py: Python<'_>) -> PyResult<PyObject> {
    loads(py, build().describe().to_string())
}

#[pymodule]
fn mrlypy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Handle>()?;
    m.add_function(wrap_pyfunction!(boot, m)?)?;
    m.add_function(wrap_pyfunction!(act, m)?)?;
    m.add_function(wrap_pyfunction!(frame, m)?)?;
    m.add_function(wrap_pyfunction!(describe, m)?)?;
    graphics::register(m.py(), m)?;
    Ok(())
}
