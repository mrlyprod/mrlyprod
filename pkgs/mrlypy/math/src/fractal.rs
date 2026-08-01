use pyo3::prelude::*;

fn grid<F: Fn(f64, f64) -> i64>(
    width: usize,
    height: usize,
    bounds: [f64; 4],
    point: F,
) -> Vec<Vec<i64>> {
    let [xmin, xmax, ymin, ymax] = bounds;
    let (vw, vh) = (xmax - xmin, ymax - ymin);
    (0..height)
        .map(|py| {
            let ci = ymax - (py as f64 + 0.5) / height as f64 * vh;
            (0..width)
                .map(|px| {
                    let cr = xmin + (px as f64 + 0.5) / width as f64 * vw;
                    point(cr, ci)
                })
                .collect()
        })
        .collect()
}

#[pyfunction]
#[pyo3(signature = (width, height, max_iter=100, bounds=None))]
fn mandelbrot(
    width: usize,
    height: usize,
    max_iter: i64,
    bounds: Option<[f64; 4]>,
) -> Vec<Vec<i64>> {
    let bounds = bounds.unwrap_or_else(|| mrlymath::fractal::MANDELBROT.reals());
    grid(width, height, bounds, |cr, ci| {
        mrlymath::fractal::mandelbrot(cr, ci, max_iter)
    })
}

#[pyfunction]
#[pyo3(signature = (width, height, cr, ci, max_iter=100, bounds=None))]
fn julia(
    width: usize,
    height: usize,
    cr: f64,
    ci: f64,
    max_iter: i64,
    bounds: Option<[f64; 4]>,
) -> Vec<Vec<i64>> {
    let bounds = bounds.unwrap_or_else(|| mrlymath::fractal::JULIA.reals());
    grid(width, height, bounds, |zr, zi| {
        mrlymath::fractal::julia(zr, zi, cr, ci, max_iter)
    })
}

pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let fractal = PyModule::new_bound(py, "fractal")?;
    fractal.add_function(wrap_pyfunction!(mandelbrot, &fractal)?)?;
    fractal.add_function(wrap_pyfunction!(julia, &fractal)?)?;
    parent.add_submodule(&fractal)?;
    Ok(())
}
