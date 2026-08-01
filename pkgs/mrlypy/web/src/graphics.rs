use mrlycore::cell::Cell;
use mrlycore::colors::Color;
use mrlycore::io;
use mrlycore::tensor::Tensor;
use mrlycore::MrlyError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn to_py_err(e: MrlyError) -> PyErr {
    PyValueError::new_err(e.to_string())
}

fn tensor_from_grid(grid: &[Vec<u8>]) -> PyResult<Tensor> {
    let height = grid.len();
    let width = grid.first().map_or(0, |row| row.len());
    if height == 0 || width == 0 {
        return Err(PyValueError::new_err("grid must be non-empty."));
    }
    if grid.iter().any(|row| row.len() != width) {
        return Err(PyValueError::new_err("grid rows must have equal length."));
    }
    let mut data = Vec::with_capacity(width * height);
    for row in grid {
        data.extend_from_slice(row);
    }
    Ok(Tensor::of(data, vec![height, width]))
}

fn grid_from_tensor(t: &Tensor) -> Vec<Vec<u8>> {
    let (height, width) = (t.shape[0], t.shape[1]);
    let bytes = t.bytes();
    (0..height)
        .map(|y| bytes[y * width..(y + 1) * width].to_vec())
        .collect()
}

fn colors_from_pixels(pixels: &[Vec<Vec<u8>>]) -> PyResult<(Vec<[u8; 4]>, usize, usize)> {
    let height = pixels.len();
    let width = pixels.first().map_or(0, |row| row.len());
    if height == 0 || width == 0 {
        return Err(PyValueError::new_err("pixels must be non-empty."));
    }
    let mut colors = Vec::with_capacity(width * height);
    for row in pixels {
        if row.len() != width {
            return Err(PyValueError::new_err("pixel rows must have equal length."));
        }
        for px in row {
            let rgba = match px.as_slice() {
                [r, g, b] => [*r, *g, *b, 255],
                [r, g, b, a] => [*r, *g, *b, *a],
                _ => {
                    return Err(PyValueError::new_err(
                        "each pixel must be [r, g, b] or [r, g, b, a].",
                    ))
                }
            };
            colors.push(rgba);
        }
    }
    Ok((colors, width, height))
}

fn pixels_from_colors(colors: &[[u8; 4]], width: usize, height: usize) -> Vec<Vec<Vec<u8>>> {
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let c = colors[y * width + x];
                    vec![c[0], c[1], c[2], c[3]]
                })
                .collect()
        })
        .collect()
}

fn cell_from_colors(colors: Vec<[u8; 4]>, width: usize, height: usize) -> Cell {
    let mut cell = Cell::new(Tensor::new(vec![height, width]));
    cell.colors = Some(colors);
    cell
}

fn color_from_entry(entry: &[u8]) -> PyResult<Color> {
    match entry {
        [r, g, b] => Ok(Color::rgb(*r, *g, *b)),
        [r, g, b, a] => Ok(Color::rgba(*r, *g, *b, *a)),
        _ => Err(PyValueError::new_err(
            "each palette entry must be [r, g, b] or [r, g, b, a].",
        )),
    }
}

#[pyfunction]
fn binarize(grid: Vec<Vec<u8>>, threshold: u8) -> PyResult<Vec<Vec<u8>>> {
    let t = tensor_from_grid(&grid)?;
    Ok(grid_from_tensor(&t.binarize(threshold)))
}

#[pyfunction]
fn blur(grid: Vec<Vec<u8>>, mask: Vec<Vec<u8>>, wrap: bool) -> PyResult<Vec<Vec<u8>>> {
    let t = tensor_from_grid(&grid)?;
    let m = tensor_from_grid(&mask)?;
    let out = t.blur(&m, wrap).map_err(to_py_err)?;
    Ok(grid_from_tensor(&out))
}

#[pyfunction]
fn perforate(grid: Vec<Vec<u8>>, mask: Vec<Vec<u8>>, value: u8) -> PyResult<Vec<Vec<u8>>> {
    let t = tensor_from_grid(&grid)?;
    let m = tensor_from_grid(&mask)?;
    let out = t.perforate(&m, value).map_err(to_py_err)?;
    Ok(grid_from_tensor(&out))
}

#[pyfunction]
fn get_type(pixels: Vec<Vec<Vec<u8>>>) -> PyResult<String> {
    let (colors, width, height) = colors_from_pixels(&pixels)?;
    let cell = cell_from_colors(colors, width, height);
    let kind = match io::analyze(&cell).kind {
        io::Kind::Binary => "binary",
        io::Kind::Grayscale => "grayscale",
        io::Kind::Color => "color",
    };
    Ok(kind.to_string())
}

#[pyfunction]
fn get_color(pixels: Vec<Vec<Vec<u8>>>) -> PyResult<(String, f64, f64, f64)> {
    let (colors, width, height) = colors_from_pixels(&pixels)?;
    let cell = cell_from_colors(colors, width, height);
    let a = io::analyze(&cell);
    Ok((
        a.dominant.to_hex(),
        a.mean_luminance,
        a.min_luminance,
        a.max_luminance,
    ))
}

#[pyfunction]
fn recolor(pixels: Vec<Vec<Vec<u8>>>, palette: Vec<Vec<u8>>) -> PyResult<Vec<Vec<Vec<u8>>>> {
    let (colors, width, height) = colors_from_pixels(&pixels)?;
    let cell = cell_from_colors(colors, width, height);
    let palette: Vec<Color> = palette
        .iter()
        .map(|entry| color_from_entry(entry))
        .collect::<PyResult<_>>()?;
    let out = io::recolor(&cell, &palette).map_err(to_py_err)?;
    Ok(pixels_from_colors(&out.colors.unwrap(), width, height))
}

pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let graphics = PyModule::new_bound(py, "graphics")?;
    graphics.add_function(wrap_pyfunction!(get_type, &graphics)?)?;
    graphics.add_function(wrap_pyfunction!(get_color, &graphics)?)?;
    graphics.add_function(wrap_pyfunction!(perforate, &graphics)?)?;
    graphics.add_function(wrap_pyfunction!(binarize, &graphics)?)?;
    graphics.add_function(wrap_pyfunction!(recolor, &graphics)?)?;
    graphics.add_function(wrap_pyfunction!(blur, &graphics)?)?;
    parent.add_submodule(&graphics)?;
    Ok(())
}
