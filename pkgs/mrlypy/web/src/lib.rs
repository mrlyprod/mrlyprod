mod font;
mod graphics;

use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Goose, Os};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

fn build() -> Os {
    mrlyweb::registry::boot()
}

fn loads(py: Python<'_>, text: String) -> PyResult<PyObject> {
    Ok(py
        .import_bound("json")?
        .call_method1("loads", (text,))?
        .unbind())
}

fn shape_of(py: Python<'_>, shape: Option<&Bound<'_, PyAny>>) -> PyResult<Option<Json>> {
    let Some(obj) = shape else { return Ok(None) };
    let text: String = match obj.extract::<String>() {
        Ok(s) => s,
        Err(_) => py
            .import_bound("json")?
            .call_method1("dumps", (obj,))?
            .extract()?,
    };
    if text.trim().is_empty() {
        return Ok(None);
    }
    Ok(mrlycore::json::parse(&text).ok())
}

#[pyclass(unsendable)]
pub struct Handle {
    os: Os,
    goose: Option<(u64, Goose)>,
}

#[pyfunction]
fn boot() -> Handle {
    Handle {
        os: build(),
        goose: None,
    }
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
    let parsed = mrlycore::json::parse(&text).unwrap_or(json!({}));
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
    loads(py, handle.os.frame(None).to_json().to_string())
}

#[pyfunction]
fn goose_step(handle: &mut Handle, seed: u64) -> Option<String> {
    if handle.goose.as_ref().map(|(s, _)| *s) != Some(seed) {
        handle.goose = Some((seed, Goose::new(seed)));
    }
    let (_, goose) = handle.goose.as_mut()?;
    goose
        .step(&mut handle.os)
        .map(|call| call.to_json().to_string())
}

#[pyfunction]
#[pyo3(signature = (handle, shape=None))]
fn frame(py: Python<'_>, handle: &Handle, shape: Option<&Bound<'_, PyAny>>) -> PyResult<PyObject> {
    let shape = shape_of(py, shape)?;
    loads(py, handle.os.frame(shape.as_ref()).to_json().to_string())
}

#[pyfunction]
#[pyo3(signature = (shape=None))]
fn describe(py: Python<'_>, shape: Option<&Bound<'_, PyAny>>) -> PyResult<PyObject> {
    let shape = shape_of(py, shape)?;
    loads(py, build().describe(shape.as_ref()).to_string())
}

fn target(handle: &Handle, app: Option<&str>) -> PyResult<String> {
    match app {
        Some(a) => Ok(a.to_string()),
        None => handle
            .os
            .frame(None)
            .route
            .map(|r| r.app)
            .ok_or_else(|| PyValueError::new_err("no current app")),
    }
}

#[pyfunction]
#[pyo3(signature = (handle, app=None, shape=None))]
fn peek(
    py: Python<'_>,
    handle: &Handle,
    app: Option<&str>,
    shape: Option<&Bound<'_, PyAny>>,
) -> PyResult<PyObject> {
    let app = target(handle, app)?;
    let shape = shape_of(py, shape)?;
    let view = handle
        .os
        .peek(&app, shape.as_ref())
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
    let view = handle
        .os
        .peek(&app, None)
        .ok_or_else(|| PyValueError::new_err("no such app"))?;
    let image = mrlycore::image::Image::from_json(&view.state["frame"])
        .map_err(|_| PyValueError::new_err("nothing to shoot here"))?;
    let mut buf = Vec::with_capacity(image.width * image.height * 4);
    for c in image.colors() {
        buf.extend_from_slice(&c);
    }
    Ok((
        image.width,
        image.height,
        PyBytes::new_bound(py, &buf).into(),
    ))
}

#[pyfunction]
#[pyo3(signature = (handle, app=None))]
fn capture_png(py: Python<'_>, handle: &Handle, app: Option<&str>) -> PyResult<PyObject> {
    let app = target(handle, app)?;
    let bytes = handle.os.snapshot(&app).map_err(PyValueError::new_err)?;
    Ok(PyBytes::new_bound(py, &bytes).into())
}

#[pymodule]
#[pyo3(name = "mrlyweb")]
fn web(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Handle>()?;
    m.add_function(wrap_pyfunction!(boot, m)?)?;
    m.add_function(wrap_pyfunction!(act, m)?)?;
    m.add_function(wrap_pyfunction!(goose_step, m)?)?;
    m.add_function(wrap_pyfunction!(frame, m)?)?;
    m.add_function(wrap_pyfunction!(describe, m)?)?;
    m.add_function(wrap_pyfunction!(peek, m)?)?;
    m.add_function(wrap_pyfunction!(capture, m)?)?;
    m.add_function(wrap_pyfunction!(capture_png, m)?)?;
    graphics::register(m.py(), m)?;
    font::register(m.py(), m)?;
    Ok(())
}
