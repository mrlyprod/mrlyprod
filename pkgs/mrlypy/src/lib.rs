mod graphics;

use mrlyos::kernel::{Call, Iden, Os};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde_json::{json, Value};

fn build() -> Os {
    let mut os = Os::new(Iden::new("guest"));
    for app in mrlynet::registry::catalogue() {
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

fn target(handle: &Handle, app: Option<&str>) -> PyResult<String> {
    match app {
        Some(a) => Ok(a.to_string()),
        None => handle
            .os
            .frame()
            .route
            .map(|r| r.app)
            .ok_or_else(|| PyValueError::new_err("no current app")),
    }
}

#[pyfunction]
#[pyo3(signature = (handle, app=None))]
fn peek(py: Python<'_>, handle: &Handle, app: Option<&str>) -> PyResult<PyObject> {
    let app = target(handle, app)?;
    let view = handle
        .os
        .peek(&app)
        .ok_or_else(|| PyValueError::new_err("no such app"))?;
    loads(py, view.to_json().to_string())
}

#[pyfunction]
#[pyo3(signature = (handle, app=None))]
fn capture(
    py: Python<'_>,
    handle: &Handle,
    app: Option<&str>,
) -> PyResult<(usize, usize, PyObject)> {
    let app = target(handle, app)?;
    let (w, h, buf) =
        mrlynet::face::canvas_rgba(&handle.os, &app).map_err(PyValueError::new_err)?;
    Ok((w, h, PyBytes::new_bound(py, &buf).into()))
}

#[pyfunction]
#[pyo3(signature = (handle, app=None))]
fn capture_png(py: Python<'_>, handle: &Handle, app: Option<&str>) -> PyResult<PyObject> {
    let app = target(handle, app)?;
    let bytes = handle.os.snapshot(&app).map_err(PyValueError::new_err)?;
    Ok(PyBytes::new_bound(py, &bytes).into())
}

#[pyfunction]
#[pyo3(signature = (handle, app=None))]
fn face(py: Python<'_>, handle: &Handle, app: Option<&str>) -> PyResult<(usize, usize, PyObject)> {
    let app = target(handle, app)?;
    let (w, h, buf) = mrlynet::face::face_rgba(&handle.os, &app).map_err(PyValueError::new_err)?;
    Ok((w, h, PyBytes::new_bound(py, &buf).into()))
}

#[pyfunction]
#[pyo3(signature = (handle, app=None))]
fn face_png(py: Python<'_>, handle: &Handle, app: Option<&str>) -> PyResult<PyObject> {
    let app = target(handle, app)?;
    let bytes = mrlynet::face::face_png(&handle.os, &app).map_err(PyValueError::new_err)?;
    Ok(PyBytes::new_bound(py, &bytes).into())
}

#[pymodule]
fn mrlypy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Handle>()?;
    m.add_function(wrap_pyfunction!(boot, m)?)?;
    m.add_function(wrap_pyfunction!(act, m)?)?;
    m.add_function(wrap_pyfunction!(frame, m)?)?;
    m.add_function(wrap_pyfunction!(describe, m)?)?;
    m.add_function(wrap_pyfunction!(peek, m)?)?;
    m.add_function(wrap_pyfunction!(capture, m)?)?;
    m.add_function(wrap_pyfunction!(capture_png, m)?)?;
    m.add_function(wrap_pyfunction!(face, m)?)?;
    m.add_function(wrap_pyfunction!(face_png, m)?)?;
    graphics::register(m.py(), m)?;
    Ok(())
}
