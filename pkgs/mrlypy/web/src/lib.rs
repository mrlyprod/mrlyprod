use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use pyo3::prelude::*;

fn loads(py: Python<'_>, text: String) -> PyResult<PyObject> {
    Ok(py
        .import_bound("json")?
        .call_method1("loads", (text,))?
        .unbind())
}

fn dumps(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<String> {
    match obj.extract::<String>() {
        Ok(text) => Ok(text),
        Err(_) => py
            .import_bound("json")?
            .call_method1("dumps", (obj,))?
            .extract(),
    }
}

fn shape_of(py: Python<'_>, shape: Option<&Bound<'_, PyAny>>) -> PyResult<Option<Json>> {
    let Some(obj) = shape else { return Ok(None) };
    let text = dumps(py, obj)?;
    if text.trim().is_empty() {
        return Ok(None);
    }
    Ok(mrlycore::json::parse(&text).ok())
}

fn found(py: Python<'_>, value: Option<Json>) -> PyResult<PyObject> {
    match value {
        Some(value) => loads(py, value.to_string()),
        None => Ok(py.None()),
    }
}

#[pyclass(unsendable)]
pub struct Handle {
    os: Os,
}

#[pyfunction]
fn boot(loadout: &str) -> Handle {
    Handle {
        os: mrlyweb::registry::boot(loadout),
    }
}

#[pyfunction]
#[pyo3(signature = (handle, shape=None))]
fn list(py: Python<'_>, handle: &Handle, shape: Option<&Bound<'_, PyAny>>) -> PyResult<PyObject> {
    let shape = shape_of(py, shape)?;
    loads(py, handle.os.list(shape.as_ref()).to_string())
}

#[pyfunction]
fn call(py: Python<'_>, handle: &mut Handle, req: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let text = dumps(py, req)?;
    let parsed = mrlycore::json::parse(&text).unwrap_or(json!({}));
    let verb = parsed["verb"].as_str().unwrap_or("").to_string();
    let args = if parsed["args"].is_object() {
        parsed["args"].clone()
    } else {
        json!({})
    };
    let mut made = Call::new(&verb, args);
    if let Some(now) = parsed["now"].as_i64() {
        made = made.at(now);
    }
    handle.os.call(made);
    found(py, handle.os.read("", None))
}

#[pyfunction]
#[pyo3(signature = (handle, path="", shape=None))]
fn read(
    py: Python<'_>,
    handle: &Handle,
    path: &str,
    shape: Option<&Bound<'_, PyAny>>,
) -> PyResult<PyObject> {
    let shape = shape_of(py, shape)?;
    found(py, handle.os.read(path, shape.as_ref()))
}

#[pymodule]
#[pyo3(name = "mrlyweb")]
fn web(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Handle>()?;
    m.add_function(wrap_pyfunction!(boot, m)?)?;
    m.add_function(wrap_pyfunction!(list, m)?)?;
    m.add_function(wrap_pyfunction!(call, m)?)?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
