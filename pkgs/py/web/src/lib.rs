use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use pyo3::prelude::*;

fn loads(py: Python<'_>, text: String) -> PyResult<Py<PyAny>> {
    Ok(py.import("json")?.call_method1("loads", (text,))?.unbind())
}

fn dumps(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<String> {
    match obj.extract::<String>() {
        Ok(text) => Ok(text),
        Err(_) => py.import("json")?.call_method1("dumps", (obj,))?.extract(),
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

fn found(py: Python<'_>, value: Option<Json>) -> PyResult<Py<PyAny>> {
    match value {
        Some(value) => loads(py, value.to_string()),
        None => Ok(py.None()),
    }
}

/// The four-door handle on one booted world.
#[pyclass(unsendable)]
pub struct Handle {
    os: Os,
}

/// Boots the named loadout into a fresh world and returns its handle.
#[pyfunction]
fn boot(loadout: &str) -> Handle {
    Handle {
        os: mrlyweb::registry::boot(loadout),
    }
}

/// Lists the version, the apps and every verb on offer as Python data, pruned to the shape.
#[pyfunction]
#[pyo3(signature = (handle, shape=None))]
fn list(py: Python<'_>, handle: &Handle, shape: Option<&Bound<'_, PyAny>>) -> PyResult<Py<PyAny>> {
    let shape = shape_of(py, shape)?;
    loads(py, handle.os.list(shape.as_ref()).to_string())
}

/// Runs a call of verb, args and optional now, then returns the fresh whole envelope as Python data.
#[pyfunction]
fn call(py: Python<'_>, handle: &mut Handle, req: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
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

/// Reads the value at a slash path, from the whole envelope down to one drilled leaf, or None where nothing lives.
#[pyfunction]
#[pyo3(signature = (handle, path="", shape=None))]
fn read(
    py: Python<'_>,
    handle: &Handle,
    path: &str,
    shape: Option<&Bound<'_, PyAny>>,
) -> PyResult<Py<PyAny>> {
    let shape = shape_of(py, shape)?;
    found(py, handle.os.read(path, shape.as_ref()))
}

/// Mounts the handle and the four doors as the mrlyweb Python module.
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
