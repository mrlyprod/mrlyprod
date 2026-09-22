#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// The substrate: tensors, cells, colors, images, codecs, resampling and seeded chance.
/// The substrate: the road from a grid of bytes to pixels, with nothing mrly on it.
pub mod core {
    use crate::hand::{ok, PyColor, PyPixels, PyRng, PySerde};
    use pyo3::prelude::*;
    use pyo3::types::PyDict;
    use pyo3::IntoPyObjectExt;

    /// The cell grid: type bytes with optional per-cell colors and tags.
    pub mod cell {
        use crate::hand::{ok, PyCell, PyColor, PyRgba, PyRng, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Flips every type to one minus itself, same as invert.
        #[pyfunction]
        #[pyo3(name = "anti", signature = (cell))]
        pub fn anti<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::anti(cell);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Maps every type to one at or above the threshold and zero below, dropping colors.
        #[pyfunction]
        #[pyo3(name = "binarize", signature = (cell, threshold))]
        pub fn binarize<'py>(py: Python<'py>, cell: PyCell, threshold: u8) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::binarize(cell, threshold);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Binarizes the types at Otsu's threshold, dropping colors.
        #[pyfunction]
        #[pyo3(name = "binarize_otsu", signature = (cell))]
        pub fn binarize_otsu<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::binarize_otsu(cell);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Replaces every type with the rounded mean of its masked neighborhood, dropping colors.
        #[pyfunction]
        #[pyo3(name = "blur", signature = (cell, mask, wrap))]
        pub fn blur<'py>(py: Python<'py>, cell: PyCell, mask: PyTensor, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrlyrs::core::Cell::blur(cell, &mask, wrap);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the painted color at a flat index, or transparent while unpainted or past the end.
        #[pyfunction]
        #[pyo3(name = "color_at", signature = (cell, flat))]
        pub fn color_at<'py>(py: Python<'py>, cell: PyCell, flat: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::color_at(&cell, flat);
            (PyRgba(out)).into_bound_py_any(py)
        }

        /// Builds the Kronecker product of the two cells' types.
        #[pyfunction]
        #[pyo3(name = "combine", signature = (cell, other))]
        pub fn combine<'py>(py: Python<'py>, cell: PyCell, other: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let other = other.0;
            let out = mrlyrs::core::Cell::combine(&cell, &other);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags.
        #[pyfunction]
        #[pyo3(name = "fractal", signature = (cell, level))]
        pub fn fractal<'py>(py: Python<'py>, cell: PyCell, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::fractal(cell, level);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Flips every type to one minus itself.
        #[pyfunction]
        #[pyo3(name = "invert", signature = (cell))]
        pub fn invert<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::invert(cell);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Tags every cell with its concentric shell distance from the center.
        #[pyfunction]
        #[pyo3(name = "layers", signature = (cell, dtype))]
        pub fn layers<'py>(py: Python<'py>, cell: PyCell, dtype: PySerde<mrlyrs::core::Dtype>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let dtype = dtype.0;
            let out = mrlyrs::core::Cell::layers(cell, dtype);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Folds at least two cells into one by chained Kronecker products.
        #[pyfunction]
        #[pyo3(name = "magic", signature = (cells))]
        pub fn magic<'py>(py: Python<'py>, cells: Vec<PyCell>) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::cell::magic(&cells);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the default mapping of the first six types to white, black, alpha, red, green, and blue.
        #[pyfunction]
        #[pyo3(name = "mapping", signature = ())]
        pub fn mapping<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::cell::mapping();
            ((out).into_iter().map(|(k, v)| (k, (v).into_iter().map(PyColor).collect::<Vec<_>>())).collect::<std::collections::HashMap<_, _>>()).into_bound_py_any(py)
        }

        /// Stitches same-shaped cells into one grid of reps blocks per axis.
        #[pyfunction]
        #[pyo3(name = "merge", signature = (cells, reps))]
        pub fn merge<'py>(py: Python<'py>, cells: Vec<PyCell>, reps: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::cell::merge(&cells, &reps);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the 3-wide Moore mask of the dimension, every site on but the center.
        #[pyfunction]
        #[pyo3(name = "moore", signature = (dimension))]
        pub fn moore<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::cell::moore(dimension);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Lays the cell each mask entry indexes into that entry's place and merges the lot.
        #[pyfunction]
        #[pyo3(name = "mosaic", signature = (mask, cells))]
        pub fn mosaic<'py>(py: Python<'py>, mask: PyTensor, cells: Vec<PyCell>) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::cell::mosaic(&mask, &cells);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Tags every cell with its count of target-valued neighbors under the mask.
        #[pyfunction]
        #[pyo3(name = "neighbors", signature = (cell, mask, target, wrap, dtype))]
        pub fn neighbors<'py>(py: Python<'py>, cell: PyCell, mask: PyTensor, target: u8, wrap: bool, dtype: PySerde<mrlyrs::core::Dtype>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let dtype = dtype.0;
            let out = mrlyrs::core::Cell::neighbors(cell, &mask, target, wrap, dtype);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Wraps a tensor of types in a bare cell, colorless and tagless.
        #[pyfunction]
        #[pyo3(name = "new", signature = (types))]
        pub fn new<'py>(py: Python<'py>, types: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let out = mrlyrs::core::Cell::new(types);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Wraps the cell in count layers of value on every side, dropping colors.
        #[pyfunction]
        #[pyo3(name = "pad", signature = (cell, count, value))]
        pub fn pad<'py>(py: Python<'py>, cell: PyCell, count: usize, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::pad(cell, count, value);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Colors every mapped cell, picking within each type's palette by the mode.
        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, mapping, mode, rng=None))]
        pub fn paint<'py>(py: Python<'py>, cell: PyCell, mapping: std::collections::HashMap<u8, Vec<PyColor>>, mode: PySerde<mrlyrs::core::Mode>, rng: Option<&mut PyRng>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mapping = mapping.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>())).collect();
            let mode = mode.0;
            let out = mrlyrs::core::Cell::paint(cell, &mapping, mode, rng.map(|r| &mut r.0));
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Stamps value wherever the tiled mask is on, dropping colors.
        #[pyfunction]
        #[pyo3(name = "perforate", signature = (cell, mask, value))]
        pub fn perforate<'py>(py: Python<'py>, cell: PyCell, mask: PyTensor, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrlyrs::core::Cell::perforate(cell, &mask, value);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map.
        #[pyfunction]
        #[pyo3(name = "remap", signature = (cell, map, shape))]
        pub fn remap<'py>(py: Python<'py>, cell: PyCell, map: Vec<usize>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::cell::remap(&cell, &map, &shape);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the flat rgba bytes of the cells, four to a cell, the stored color where there is one and opaque black everywhere else.
        #[pyfunction]
        #[pyo3(name = "rgba", signature = (cell))]
        pub fn rgba<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::rgba(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the flat source index of every destination cell after k quarter turns in the plane of the axes.
        #[pyfunction]
        #[pyo3(name = "rot90_map", signature = (shape, k, axes))]
        pub fn rot90_map<'py>(py: Python<'py>, shape: Vec<usize>, k: usize, axes: (usize, usize)) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::cell::rot90_map(&shape, k, axes);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along.
        #[pyfunction]
        #[pyo3(name = "rotate", signature = (cell, k, axes))]
        pub fn rotate<'py>(py: Python<'py>, cell: PyCell, k: usize, axes: (usize, usize)) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::rotate(cell, k, axes);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the shape of the type tensor.
        #[pyfunction]
        #[pyo3(name = "shape", signature = (cell))]
        pub fn shape<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::shape(&cell);
            ((out).to_vec()).into_bound_py_any(py)
        }

        /// Returns the number of cells.
        #[pyfunction]
        #[pyo3(name = "size", signature = (cell))]
        pub fn size<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::size(&cell);
            (out).into_bound_py_any(py)
        }

        /// Repeats the cell reps times along each axis, carrying colors and tags along.
        #[pyfunction]
        #[pyo3(name = "tile", signature = (cell, reps))]
        pub fn tile<'py>(py: Python<'py>, cell: PyCell, reps: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::core::Cell::tile(cell, &reps);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the flat source index of every destination cell after tiling reps copies per axis.
        #[pyfunction]
        #[pyo3(name = "tile_map", signature = (shape, reps))]
        pub fn tile_map<'py>(py: Python<'py>, shape: Vec<usize>, reps: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::cell::tile_map(&shape, &reps);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.cell")?;
            m.setattr("__doc__", "The cell grid: type bytes with optional per-cell colors and tags.")?;
            m.add_function(wrap_pyfunction!(anti, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize_otsu, &m)?)?;
            m.add_function(wrap_pyfunction!(blur, &m)?)?;
            m.add_function(wrap_pyfunction!(color_at, &m)?)?;
            m.add_function(wrap_pyfunction!(combine, &m)?)?;
            m.add_function(wrap_pyfunction!(fractal, &m)?)?;
            m.add_function(wrap_pyfunction!(invert, &m)?)?;
            m.add_function(wrap_pyfunction!(layers, &m)?)?;
            m.add_function(wrap_pyfunction!(magic, &m)?)?;
            m.add_function(wrap_pyfunction!(mapping, &m)?)?;
            m.add_function(wrap_pyfunction!(merge, &m)?)?;
            m.add_function(wrap_pyfunction!(moore, &m)?)?;
            m.add_function(wrap_pyfunction!(mosaic, &m)?)?;
            m.add_function(wrap_pyfunction!(neighbors, &m)?)?;
            m.add_function(wrap_pyfunction!(new, &m)?)?;
            m.add_function(wrap_pyfunction!(pad, &m)?)?;
            m.add_function(wrap_pyfunction!(paint, &m)?)?;
            m.add_function(wrap_pyfunction!(perforate, &m)?)?;
            m.add_function(wrap_pyfunction!(remap, &m)?)?;
            m.add_function(wrap_pyfunction!(rgba, &m)?)?;
            m.add_function(wrap_pyfunction!(rot90_map, &m)?)?;
            m.add_function(wrap_pyfunction!(rotate, &m)?)?;
            m.add_function(wrap_pyfunction!(shape, &m)?)?;
            m.add_function(wrap_pyfunction!(size, &m)?)?;
            m.add_function(wrap_pyfunction!(tile, &m)?)?;
            m.add_function(wrap_pyfunction!(tile_map, &m)?)?;
            let names: Vec<&str> = vec!["anti", "binarize", "binarize_otsu", "blur", "color_at", "combine", "fractal", "invert", "layers", "magic", "mapping", "merge", "moore", "mosaic", "neighbors", "new", "pad", "paint", "perforate", "remap", "rgba", "rot90_map", "rotate", "shape", "size", "tile", "tile_map"];
            m.add("__all__", names)?;
            parent.add("cell", &m)?;
            sys.set_item("mrlypy._mrlypy.core.cell", &m)?;
            Ok(())
        }
    }

    /// The png and gif codecs, rented from the png and gif crates.
    /// A png is written paletted whenever 256 colors or fewer fit, rgba otherwise; a gif is always paletted and always loops.
    pub mod codec {
        use crate::hand::{ok, PyPixels};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block.
        #[pyfunction]
        #[pyo3(name = "gif", signature = (frames, palette, width, height, scale, delay))]
        pub fn gif<'py>(py: Python<'py>, frames: Vec<Vec<u8>>, palette: PyPixels, width: usize, height: usize, scale: usize, delay: usize) -> PyResult<Bound<'py, PyAny>> {
            let frames = frames.iter().map(|y| y.as_slice()).collect::<Vec<_>>();
            let palette = palette.0;
            let out = mrlyrs::core::codec::gif(&frames, &palette, width, height, scale, delay);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Encodes rgba colors as a png, drawing each source pixel as a scale by scale block.
        #[pyfunction]
        #[pyo3(name = "png", signature = (colors, width, height, scale))]
        pub fn png<'py>(py: Python<'py>, colors: PyPixels, width: usize, height: usize, scale: usize) -> PyResult<Bound<'py, PyAny>> {
            let colors = colors.0;
            let out = mrlyrs::core::codec::png(&colors, width, height, scale);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.codec")?;
            m.setattr("__doc__", "The png and gif codecs, rented from the png and gif crates.\nA png is written paletted whenever 256 colors or fewer fit, rgba otherwise; a gif is always paletted and always loops.")?;
            m.add_function(wrap_pyfunction!(gif, &m)?)?;
            m.add_function(wrap_pyfunction!(png, &m)?)?;
            let names: Vec<&str> = vec!["gif", "png"];
            m.add("__all__", names)?;
            parent.add("codec", &m)?;
            sys.set_item("mrlypy._mrlypy.core.codec", &m)?;
            Ok(())
        }
    }

    /// The rgba color, its themes and the fifteen-color palette utils/colors.py stamps.
    pub mod colors {
        use crate::hand::{ok, PyColor, PyPixels, PyRgba, PyRng, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// One theme: the surfaces of a dark or a light ground and the thirteen inks, the same on both.
        #[pyclass(name = "Theme", module = "mrlypy.core.colors", from_py_object)]
        #[derive(Clone)]
        pub struct Theme(pub mrlyrs::core::colors::Theme);

        #[pymethods]
        impl Theme {
            /// The ground every figure is painted on.
            #[getter]
            #[pyo3(name = "ground")]
            pub fn ground<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.ground;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The page background, one step off the ground.
            #[getter]
            #[pyo3(name = "bg")]
            pub fn bg<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.bg;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The raised panel.
            #[getter]
            #[pyo3(name = "panel")]
            pub fn panel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.panel;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The sunken well.
            #[getter]
            #[pyo3(name = "deep")]
            pub fn deep<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.deep;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The hairline between things.
            #[getter]
            #[pyo3(name = "line")]
            pub fn line<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.line;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The foreground, the strongest tone.
            #[getter]
            #[pyo3(name = "fg")]
            pub fn fg<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.fg;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The dimmed foreground, for anything secondary.
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The interactive accent.
            #[getter]
            #[pyo3(name = "accent")]
            pub fn accent<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.accent;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The tone written on the accent.
            #[getter]
            #[pyo3(name = "on_accent")]
            pub fn on_accent<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.on_accent;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The red ink.
            #[getter]
            #[pyo3(name = "red")]
            pub fn red<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.red;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The orange ink.
            #[getter]
            #[pyo3(name = "orange")]
            pub fn orange<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.orange;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The yellow ink.
            #[getter]
            #[pyo3(name = "yellow")]
            pub fn yellow<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.yellow;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The green ink.
            #[getter]
            #[pyo3(name = "green")]
            pub fn green<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.green;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The mint ink.
            #[getter]
            #[pyo3(name = "mint")]
            pub fn mint<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.mint;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The teal ink.
            #[getter]
            #[pyo3(name = "teal")]
            pub fn teal<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.teal;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The cyan ink.
            #[getter]
            #[pyo3(name = "cyan")]
            pub fn cyan<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.cyan;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The blue ink.
            #[getter]
            #[pyo3(name = "blue")]
            pub fn blue<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.blue;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The indigo ink.
            #[getter]
            #[pyo3(name = "indigo")]
            pub fn indigo<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.indigo;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The purple ink.
            #[getter]
            #[pyo3(name = "purple")]
            pub fn purple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.purple;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The pink ink.
            #[getter]
            #[pyo3(name = "pink")]
            pub fn pink<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.pink;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The brown ink.
            #[getter]
            #[pyo3(name = "brown")]
            pub fn brown<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.brown;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The gray ink.
            #[getter]
            #[pyo3(name = "gray")]
            pub fn gray<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.gray;
                (PyColor(value)).into_bound_py_any(py)
            }
            /// The thirteen inks in name order.
            #[pyo3(name = "hues", signature = ())]
            pub fn hues<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::core::colors::Theme::hues(&self.0);
                ((out).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo.
            #[pyo3(name = "inks", signature = ())]
            pub fn inks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::core::colors::Theme::inks(&self.0);
                ((out).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Returns the color with its alpha set to level.
        #[pyfunction]
        #[pyo3(name = "alpha", signature = (color, level))]
        pub fn alpha<'py>(py: Python<'py>, color: PyColor, level: u8) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrlyrs::core::Color::alpha(&color, level);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Returns the ground rgba of the dark or the light theme.
        #[pyfunction]
        #[pyo3(name = "board", signature = (dark))]
        pub fn board<'py>(py: Python<'py>, dark: bool) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::colors::board(dark);
            (PyRgba(out)).into_bound_py_any(py)
        }

        /// Formats the color as a css rgb or rgba call.
        #[pyfunction]
        #[pyo3(name = "css", signature = (color))]
        pub fn css<'py>(py: Python<'py>, color: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrlyrs::core::Color::css(&color);
            (out).into_bound_py_any(py)
        }

        /// Parses a #RRGGBB or #RRGGBBAA code, hash optional.
        #[pyfunction]
        #[pyo3(name = "from_hex", signature = (hex))]
        pub fn from_hex<'py>(py: Python<'py>, hex: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Color::from_hex(hex);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a gradient of steps colors sweeping evenly through the given stops.
        #[pyfunction]
        #[pyo3(name = "gradient", signature = (colors, steps))]
        pub fn gradient<'py>(py: Python<'py>, colors: Vec<PyColor>, steps: usize) -> PyResult<Bound<'py, PyAny>> {
            let colors = colors.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::colors::gradient(&colors, steps);
            ((ok(out)?).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the foreground rgba of the dark or the light theme.
        #[pyfunction]
        #[pyo3(name = "ink", signature = (dark))]
        pub fn ink<'py>(py: Python<'py>, dark: bool) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::colors::ink(dark);
            (PyRgba(out)).into_bound_py_any(py)
        }

        /// Returns the color with every channel flipped and the alpha kept.
        #[pyfunction]
        #[pyo3(name = "invert", signature = (color))]
        pub fn invert<'py>(py: Python<'py>, color: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrlyrs::core::Color::invert(&color);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Returns the color scaled toward black below level 50 and toward white above.
        #[pyfunction]
        #[pyo3(name = "lightness", signature = (color, level))]
        pub fn lightness<'py>(py: Python<'py>, color: PyColor, level: u8) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrlyrs::core::Color::lightness(&color, level);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Reads rgba pixels as a type grid, one wherever the rgb mean falls below the level.
        #[pyfunction]
        #[pyo3(name = "luma_types", signature = (pixels, width, height, level))]
        pub fn luma_types<'py>(py: Python<'py>, pixels: PyPixels, width: usize, height: usize, level: u8) -> PyResult<Bound<'py, PyAny>> {
            let pixels = pixels.0;
            let out = mrlyrs::core::colors::luma_types(&pixels, width, height, level);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Blends two colors linearly by ratio.
        #[pyfunction]
        #[pyo3(name = "mix", signature = (color_1, color_2, ratio))]
        pub fn mix<'py>(py: Python<'py>, color_1: PyColor, color_2: PyColor, ratio: f64) -> PyResult<Bound<'py, PyAny>> {
            let color_1 = color_1.0;
            let color_2 = color_2.0;
            let out = mrlyrs::core::colors::mix(color_1, color_2, ratio);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the palette color a name spells.
        #[pyfunction]
        #[pyo3(name = "named", signature = (name))]
        pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::colors::named(name);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a color from the stream, opaque unless alpha is asked for.
        #[pyfunction]
        #[pyo3(name = "random", signature = (alpha, rng))]
        pub fn random<'py>(py: Python<'py>, alpha: bool, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Color::random(alpha, &mut rng.0);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Builds an opaque color.
        #[pyfunction]
        #[pyo3(name = "rgb", signature = (r, g, b))]
        pub fn rgb<'py>(py: Python<'py>, r: u8, g: u8, b: u8) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Color::rgb(r, g, b);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Builds a color with an explicit alpha.
        #[pyfunction]
        #[pyo3(name = "rgba", signature = (r, g, b, a))]
        pub fn rgba<'py>(py: Python<'py>, r: u8, g: u8, b: u8, a: u8) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Color::rgba(r, g, b, a);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Returns a hue one shade lighter, itself, and one shade darker.
        #[pyfunction]
        #[pyo3(name = "shades", signature = (hue))]
        pub fn shades<'py>(py: Python<'py>, hue: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let hue = hue.0;
            let out = mrlyrs::core::colors::shades(hue);
            ((out).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Snaps every pixel to the palette color nearest it in squared rgba distance, first on a tie.
        #[pyfunction]
        #[pyo3(name = "snap", signature = (pixels, palette))]
        pub fn snap<'py>(py: Python<'py>, pixels: PyPixels, palette: Vec<PyColor>) -> PyResult<Bound<'py, PyAny>> {
            let pixels = pixels.0;
            let palette = palette.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::colors::snap(&pixels, &palette);
            (PyPixels(out)).into_bound_py_any(py)
        }

        /// Formats the color as lowercase hex, appending the alpha pair only when not opaque.
        #[pyfunction]
        #[pyo3(name = "to_hex", signature = (color))]
        pub fn to_hex<'py>(py: Python<'py>, color: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrlyrs::core::Color::to_hex(&color);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.colors")?;
            m.setattr("__doc__", "The rgba color, its themes and the fifteen-color palette utils/colors.py stamps.")?;
            m.add_class::<Theme>()?;
            m.add_function(wrap_pyfunction!(alpha, &m)?)?;
            m.add_function(wrap_pyfunction!(board, &m)?)?;
            m.add_function(wrap_pyfunction!(css, &m)?)?;
            m.add_function(wrap_pyfunction!(from_hex, &m)?)?;
            m.add_function(wrap_pyfunction!(gradient, &m)?)?;
            m.add_function(wrap_pyfunction!(ink, &m)?)?;
            m.add_function(wrap_pyfunction!(invert, &m)?)?;
            m.add_function(wrap_pyfunction!(lightness, &m)?)?;
            m.add_function(wrap_pyfunction!(luma_types, &m)?)?;
            m.add_function(wrap_pyfunction!(mix, &m)?)?;
            m.add_function(wrap_pyfunction!(named, &m)?)?;
            m.add_function(wrap_pyfunction!(random, &m)?)?;
            m.add_function(wrap_pyfunction!(rgb, &m)?)?;
            m.add_function(wrap_pyfunction!(rgba, &m)?)?;
            m.add_function(wrap_pyfunction!(shades, &m)?)?;
            m.add_function(wrap_pyfunction!(snap, &m)?)?;
            m.add_function(wrap_pyfunction!(to_hex, &m)?)?;
            m.add("ALPHA", PyColor(mrlyrs::core::colors::ALPHA))?;
            m.add("BLACK", PyColor(mrlyrs::core::colors::BLACK))?;
            m.add("BLUE", PyColor(mrlyrs::core::colors::BLUE))?;
            m.add("BROWN", PyColor(mrlyrs::core::colors::BROWN))?;
            m.add("CYAN", PyColor(mrlyrs::core::colors::CYAN))?;
            m.add("DARK", crate::gen::core::colors::Theme(mrlyrs::core::colors::DARK))?;
            m.add("GRAY", PyColor(mrlyrs::core::colors::GRAY))?;
            m.add("GREEN", PyColor(mrlyrs::core::colors::GREEN))?;
            m.add("INDIGO", PyColor(mrlyrs::core::colors::INDIGO))?;
            m.add("LIGHT", crate::gen::core::colors::Theme(mrlyrs::core::colors::LIGHT))?;
            m.add("MINT", PyColor(mrlyrs::core::colors::MINT))?;
            m.add("NAMES", (mrlyrs::core::colors::NAMES).into_iter().map(|x| (x).to_string()).collect::<Vec<_>>())?;
            m.add("ORANGE", PyColor(mrlyrs::core::colors::ORANGE))?;
            m.add("PALETTE", (mrlyrs::core::colors::PALETTE).into_iter().map(PyColor).collect::<Vec<_>>())?;
            m.add("PINK", PyColor(mrlyrs::core::colors::PINK))?;
            m.add("PURPLE", PyColor(mrlyrs::core::colors::PURPLE))?;
            m.add("RED", PyColor(mrlyrs::core::colors::RED))?;
            m.add("TEAL", PyColor(mrlyrs::core::colors::TEAL))?;
            m.add("WHITE", PyColor(mrlyrs::core::colors::WHITE))?;
            m.add("YELLOW", PyColor(mrlyrs::core::colors::YELLOW))?;
            let names: Vec<&str> = vec!["alpha", "board", "css", "from_hex", "gradient", "ink", "invert", "lightness", "luma_types", "mix", "named", "random", "rgb", "rgba", "shades", "snap", "to_hex", "Theme", "ALPHA", "BLACK", "BLUE", "BROWN", "CYAN", "DARK", "GRAY", "GREEN", "INDIGO", "LIGHT", "MINT", "NAMES", "ORANGE", "PALETTE", "PINK", "PURPLE", "RED", "TEAL", "WHITE", "YELLOW"];
            m.add("__all__", names)?;
            parent.add("colors", &m)?;
            sys.set_item("mrlypy._mrlypy.core.colors", &m)?;
            Ok(())
        }
    }

    /// The one error type, its Result and the json parser over the rented value.
    pub mod error {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Parses JSON text into a value.
        #[pyfunction]
        #[pyo3(name = "parse", signature = (text))]
        pub fn parse<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::error::parse(text);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.error")?;
            m.setattr("__doc__", "The one error type, its Result and the json parser over the rented value.")?;
            m.add_function(wrap_pyfunction!(parse, &m)?)?;
            let names: Vec<&str> = vec!["parse"];
            m.add("__all__", names)?;
            parent.add("error", &m)?;
            sys.set_item("mrlypy._mrlypy.core.error", &m)?;
            Ok(())
        }
    }

    /// The paletted image and its rows.
    pub mod image {
        use crate::hand::{PyPixels};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window.
        #[pyfunction]
        #[pyo3(name = "blur", signature = (pixels, width, height, radius))]
        pub fn blur<'py>(py: Python<'py>, pixels: PyPixels, width: usize, height: usize, radius: usize) -> PyResult<Bound<'py, PyAny>> {
            let pixels = pixels.0;
            let out = mrlyrs::core::image::blur(&pixels, width, height, radius);
            (PyPixels(out)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.image")?;
            m.setattr("__doc__", "The paletted image and its rows.")?;
            m.add_function(wrap_pyfunction!(blur, &m)?)?;
            let names: Vec<&str> = vec!["blur"];
            m.add("__all__", names)?;
            parent.add("image", &m)?;
            sys.set_item("mrlypy._mrlypy.core.image", &m)?;
            Ok(())
        }
    }

    /// The editions that distribute a palette over a cell.
    pub mod paint {
        use crate::hand::{ok, PyColor, PyRng, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A complete coloring recipe for one cell.
        #[pyclass(name = "Paint", module = "mrlypy.core.paint", from_py_object)]
        #[derive(Clone)]
        pub struct Paint(pub mrlyrs::core::paint::Paint);

        #[pymethods]
        impl Paint {
            /// Builds a black-primary, fill-target, multicolor paint for an edition.
            #[new]
            #[pyo3(signature = (edition))]
            pub fn __new__(edition: PySerde<mrlyrs::core::paint::Edition>) -> PyResult<Self> {
                let edition = edition.0;
                let out = mrlyrs::core::paint::Paint::new(edition);
                Ok(Self(out))
            }
            /// The coloring edition.
            #[getter]
            #[pyo3(name = "edition")]
            pub fn edition<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.edition;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The secondary color scheme.
            #[getter]
            #[pyo3(name = "scheme")]
            pub fn scheme<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.scheme;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The side the primary ink lands on.
            #[getter]
            #[pyo3(name = "target")]
            pub fn target<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.target;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The primary ink.
            #[getter]
            #[pyo3(name = "primary")]
            pub fn primary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.primary;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The secondary inks.
            #[getter]
            #[pyo3(name = "secondary")]
            pub fn secondary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.secondary.clone();
                ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// The shade indices of a multitone ramp.
            #[getter]
            #[pyo3(name = "shades")]
            pub fn shades<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.shades.clone();
                (value).into_bound_py_any(py)
            }
            /// Returns true for the Simple edition.
            #[pyo3(name = "is_simple", signature = ())]
            pub fn is_simple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::core::paint::Paint::is_simple(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a black-primary, fill-target, multicolor paint for an edition.
            #[staticmethod]
            #[pyo3(name = "new", signature = (edition))]
            pub fn new_<'py>(py: Python<'py>, edition: PySerde<mrlyrs::core::paint::Edition>) -> PyResult<Bound<'py, PyAny>> {
                let edition = edition.0;
                let out = mrlyrs::core::paint::Paint::new(edition);
                (crate::gen::core::paint::Paint(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// The seven ways a paint distributes its colors over a cell.
        #[pyclass(name = "Edition", module = "mrlypy.core.paint", skip_from_py_object)]
        pub struct Edition;

        #[pymethods]
        impl Edition {
            /// Returns every Edition in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::core::paint::Edition::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the cell-painting mode this edition renders with, or None for Random, which scatters.
            #[staticmethod]
            #[pyo3(name = "mode", signature = (edition))]
            pub fn mode<'py>(py: Python<'py>, edition: PySerde<mrlyrs::core::paint::Edition>) -> PyResult<Bound<'py, PyAny>> {
                let edition = edition.0;
                let out = mrlyrs::core::paint::Edition::mode(edition);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
        }

        /// The fifteen named inks a paint draws from.
        #[pyclass(name = "Ink", module = "mrlypy.core.paint", skip_from_py_object)]
        pub struct Ink;

        #[pymethods]
        impl Ink {
            /// Returns every Ink in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::core::paint::Ink::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the ink's color.
            #[staticmethod]
            #[pyo3(name = "color", signature = (ink))]
            pub fn color<'py>(py: Python<'py>, ink: PySerde<mrlyrs::core::paint::Ink>) -> PyResult<Bound<'py, PyAny>> {
                let ink = ink.0;
                let out = mrlyrs::core::paint::Ink::color(ink);
                (PyColor(out)).into_bound_py_any(py)
            }
        }

        /// The two ways secondary colors are drawn.
        #[pyclass(name = "Scheme", module = "mrlypy.core.paint", skip_from_py_object)]
        pub struct Scheme;

        #[pymethods]
        impl Scheme {
            /// Returns every Scheme in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::core::paint::Scheme::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// The side of the figure the primary ink lands on.
        #[pyclass(name = "Target", module = "mrlypy.core.paint", skip_from_py_object)]
        pub struct Target;

        #[pymethods]
        impl Target {
            /// Returns every Target in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::core::paint::Target::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream.
        #[pyfunction]
        #[pyo3(name = "apply", signature = (paint, cell, rng))]
        pub fn apply<'py>(py: Python<'py>, paint: PyRef<'_, crate::gen::core::paint::Paint>, cell: &Bound<'_, PyAny>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let out = mrlyrs::core::paint::apply(&paint.0, &mut cell_owned, &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        /// Replays a stored paint onto a cell, tagging first and applying it from the stream.
        #[pyfunction]
        #[pyo3(name = "coat", signature = (cell, paint, mask, rng))]
        pub fn coat<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, paint: PyRef<'_, crate::gen::core::paint::Paint>, mask: Option<PyTensor>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let mask = mask.map(|x| x.0);
            let out = mrlyrs::core::paint::coat(&mut cell_owned, &paint.0, mask.as_ref(), &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        /// Draws a random paint under the config, applies it to the cell, and returns the recipe.
        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, config, mask, rng))]
        pub fn paint<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, config: PySerde<mrlyrs::core::paint::Config>, mask: Option<PyTensor>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let config = config.0;
            let mask = mask.map(|x| x.0);
            let out = mrlyrs::core::paint::paint(&mut cell_owned, &config, mask.as_ref(), &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (crate::gen::core::paint::Paint(ok(out)?)).into_bound_py_any(py)
        }

        /// Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count.
        #[pyfunction]
        #[pyo3(name = "prime", signature = (paint, cell, mask, rng))]
        pub fn prime<'py>(py: Python<'py>, paint: crate::gen::core::paint::Paint, cell: &Bound<'_, PyAny>, mask: Option<PyTensor>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let paint = paint.0;
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let mask = mask.map(|x| x.0);
            let out = mrlyrs::core::paint::prime(paint, &mut cell_owned, mask.as_ref(), &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (crate::gen::core::paint::Paint(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a random edition from the allowed list, or from all seven.
        #[pyfunction]
        #[pyo3(name = "random_edition", signature = (editions, rng))]
        pub fn random_edition<'py>(py: Python<'py>, editions: Option<Vec<PySerde<mrlyrs::core::paint::Edition>>>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let editions = editions.map(|x| x.into_iter().map(|x| x.0).collect::<Vec<_>>());
            let out = mrlyrs::core::paint::random_edition(editions.as_deref(), &mut rng.0);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Redraws the paint's secondary inks and shades under its scheme.
        #[pyfunction]
        #[pyo3(name = "reroll", signature = (paint, rng))]
        pub fn reroll<'py>(py: Python<'py>, paint: crate::gen::core::paint::Paint, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let paint = paint.0;
            let out = mrlyrs::core::paint::reroll(paint, &mut rng.0);
            (crate::gen::core::paint::Paint(out)).into_bound_py_any(py)
        }

        /// Draws the paint's scheme, target and primary under the config, then rerolls the rest.
        #[pyfunction]
        #[pyo3(name = "setup", signature = (paint, config, rng))]
        pub fn setup<'py>(py: Python<'py>, paint: crate::gen::core::paint::Paint, config: PySerde<mrlyrs::core::paint::Config>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let paint = paint.0;
            let config = config.0;
            let out = mrlyrs::core::paint::setup(paint, &config, &mut rng.0);
            (crate::gen::core::paint::Paint(out)).into_bound_py_any(py)
        }

        /// Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side.
        #[pyfunction]
        #[pyo3(name = "tag", signature = (cell, edition, target, mask=None))]
        pub fn tag<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, edition: PySerde<mrlyrs::core::paint::Edition>, target: PySerde<mrlyrs::core::paint::Target>, mask: Option<PyTensor>) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let edition = edition.0;
            let target = target.0;
            let mask = mask.map(|x| x.0);
            let out = mrlyrs::core::paint::tag(&mut cell_owned, edition, target, mask.as_ref());
            crate::hand::cell_write_back(cell, cell_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.paint")?;
            m.setattr("__doc__", "The editions that distribute a palette over a cell.")?;
            m.add_class::<Paint>()?;
            m.add_class::<Edition>()?;
            m.add_class::<Ink>()?;
            m.add_class::<Scheme>()?;
            m.add_class::<Target>()?;
            m.add_function(wrap_pyfunction!(apply, &m)?)?;
            m.add_function(wrap_pyfunction!(coat, &m)?)?;
            m.add_function(wrap_pyfunction!(paint, &m)?)?;
            m.add_function(wrap_pyfunction!(prime, &m)?)?;
            m.add_function(wrap_pyfunction!(random_edition, &m)?)?;
            m.add_function(wrap_pyfunction!(reroll, &m)?)?;
            m.add_function(wrap_pyfunction!(setup, &m)?)?;
            m.add_function(wrap_pyfunction!(tag, &m)?)?;
            let names: Vec<&str> = vec!["apply", "coat", "paint", "prime", "random_edition", "reroll", "setup", "tag", "Paint", "Edition", "Ink", "Scheme", "Target"];
            m.add("__all__", names)?;
            parent.add("paint", &m)?;
            sys.set_item("mrlypy._mrlypy.core.paint", &m)?;
            Ok(())
        }
    }

    /// The colorizers that turn counter values into colors.
    pub mod ramp {
        use crate::hand::{PyColor, PyPixels, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up.
        #[pyfunction]
        #[pyo3(name = "color", signature = (colorizer, value, max))]
        pub fn color<'py>(py: Python<'py>, colorizer: PySerde<mrlyrs::core::Colorizer>, value: usize, max: usize) -> PyResult<Bound<'py, PyAny>> {
            let colorizer = colorizer.0;
            let out = mrlyrs::core::ramp::color(&colorizer, value, max);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Maps a slice of values to rgba pixels against the range maximum.
        #[pyfunction]
        #[pyo3(name = "colors", signature = (colorizer, values, max))]
        pub fn colors<'py>(py: Python<'py>, colorizer: PySerde<mrlyrs::core::Colorizer>, values: Vec<usize>, max: usize) -> PyResult<Bound<'py, PyAny>> {
            let colorizer = colorizer.0;
            let out = mrlyrs::core::ramp::colors(&colorizer, &values, max);
            (PyPixels(out)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.ramp")?;
            m.setattr("__doc__", "The colorizers that turn counter values into colors.")?;
            m.add_function(wrap_pyfunction!(color, &m)?)?;
            m.add_function(wrap_pyfunction!(colors, &m)?)?;
            let names: Vec<&str> = vec!["color", "colors"];
            m.add("__all__", names)?;
            parent.add("ramp", &m)?;
            sys.set_item("mrlypy._mrlypy.core.ramp", &m)?;
            Ok(())
        }
    }

    /// The seeded random stream.
    pub mod rng {
        use crate::hand::{PyRng};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Draws an integer below n, or zero when n is zero.
        #[pyfunction]
        #[pyo3(name = "below", signature = (rng, n))]
        pub fn below<'py>(py: Python<'py>, rng: &mut PyRng, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Rng::below(&mut rng.0, n);
            (out).into_bound_py_any(py)
        }

        /// Draws a fair coin flip.
        #[pyfunction]
        #[pyo3(name = "boolean", signature = (rng))]
        pub fn boolean<'py>(py: Python<'py>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Rng::boolean(&mut rng.0);
            (out).into_bound_py_any(py)
        }

        /// Returns true with probability p.
        #[pyfunction]
        #[pyo3(name = "chance", signature = (rng, p))]
        pub fn chance<'py>(py: Python<'py>, rng: &mut PyRng, p: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Rng::chance(&mut rng.0, p);
            (out).into_bound_py_any(py)
        }

        /// Builds the stream from a seed.
        #[pyfunction]
        #[pyo3(name = "new", signature = (seed))]
        pub fn new<'py>(py: Python<'py>, seed: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Rng::new(seed);
            (PyRng(out)).into_bound_py_any(py)
        }

        /// Draws an integer between lo and hi inclusive, or lo when hi is not above lo.
        #[pyfunction]
        #[pyo3(name = "range", signature = (rng, lo, hi))]
        pub fn range<'py>(py: Python<'py>, rng: &mut PyRng, lo: i64, hi: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Rng::range(&mut rng.0, lo, hi);
            (out).into_bound_py_any(py)
        }

        /// Draws amount distinct indices below length, or every index when amount is larger.
        #[pyfunction]
        #[pyo3(name = "sample_indices", signature = (rng, length, amount))]
        pub fn sample_indices<'py>(py: Python<'py>, rng: &mut PyRng, length: usize, amount: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Rng::sample_indices(&mut rng.0, length, amount);
            (out).into_bound_py_any(py)
        }

        /// Draws a float at or above zero and below one.
        #[pyfunction]
        #[pyo3(name = "unit", signature = (rng))]
        pub fn unit<'py>(py: Python<'py>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Rng::unit(&mut rng.0);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.rng")?;
            m.setattr("__doc__", "The seeded random stream.")?;
            m.add_function(wrap_pyfunction!(below, &m)?)?;
            m.add_function(wrap_pyfunction!(boolean, &m)?)?;
            m.add_function(wrap_pyfunction!(chance, &m)?)?;
            m.add_function(wrap_pyfunction!(new, &m)?)?;
            m.add_function(wrap_pyfunction!(range, &m)?)?;
            m.add_function(wrap_pyfunction!(sample_indices, &m)?)?;
            m.add_function(wrap_pyfunction!(unit, &m)?)?;
            let names: Vec<&str> = vec!["below", "boolean", "chance", "new", "range", "sample_indices", "unit"];
            m.add("__all__", names)?;
            parent.add("rng", &m)?;
            sys.set_item("mrlypy._mrlypy.core.rng", &m)?;
            Ok(())
        }
    }

    /// The tensor and its dtypes.
    pub mod tensor {
        use crate::hand::{ok, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the element at a flat index, which must be below the size like a slice index.
        #[pyfunction]
        #[pyo3(name = "at", signature = (tensor, flat))]
        pub fn at<'py>(py: Python<'py>, tensor: PyTensor, flat: usize) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::at(&tensor, flat);
            (out).into_bound_py_any(py)
        }

        /// Maps every element to one at or above the threshold, zero below.
        #[pyfunction]
        #[pyo3(name = "binarize", signature = (tensor, threshold))]
        pub fn binarize<'py>(py: Python<'py>, tensor: PyTensor, threshold: u8) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::binarize(&tensor, threshold);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Binarizes at one above the Otsu threshold.
        #[pyfunction]
        #[pyo3(name = "binarize_otsu", signature = (tensor))]
        pub fn binarize_otsu<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::binarize_otsu(&tensor);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Averages every position over its masked neighborhood, rounded.
        #[pyfunction]
        #[pyo3(name = "blur", signature = (tensor, mask, wrap))]
        pub fn blur<'py>(py: Python<'py>, tensor: PyTensor, mask: PyTensor, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let mask = mask.0;
            let out = mrlyrs::core::Tensor::blur(&tensor, &mask, wrap);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as bytes.
        #[pyfunction]
        #[pyo3(name = "bytes", signature = (tensor))]
        pub fn bytes<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::bytes(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Counts the cells holding one value.
        #[pyfunction]
        #[pyo3(name = "count", signature = (tensor, value))]
        pub fn count<'py>(py: Python<'py>, tensor: PyTensor, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::count(&tensor, value);
            (out).into_bound_py_any(py)
        }

        /// Returns the element width.
        #[pyfunction]
        #[pyo3(name = "dtype", signature = (tensor))]
        pub fn dtype<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::dtype(&tensor);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d.
        #[pyfunction]
        #[pyo3(name = "exposed", signature = (tensor))]
        pub fn exposed<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::exposed(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Builds a tensor of the shape and width filled with one value.
        #[pyfunction]
        #[pyo3(name = "filled", signature = (shape, value, dtype))]
        pub fn filled<'py>(py: Python<'py>, shape: Vec<usize>, value: i64, dtype: PySerde<mrlyrs::core::Dtype>) -> PyResult<Bound<'py, PyAny>> {
            let dtype = dtype.0;
            let out = mrlyrs::core::Tensor::filled(shape, value, dtype);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Reverses the tensor along one axis.
        #[pyfunction]
        #[pyo3(name = "flip", signature = (tensor, axis))]
        pub fn flip<'py>(py: Python<'py>, tensor: PyTensor, axis: usize) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::flip(&tensor, axis);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Folds the tensor into its level-fold Kronecker power.
        #[pyfunction]
        #[pyo3(name = "fractal", signature = (tensor, level))]
        pub fn fractal<'py>(py: Python<'py>, tensor: PyTensor, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::fractal(&tensor, level);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a u8 tensor filled with one value.
        #[pyfunction]
        #[pyo3(name = "full", signature = (shape, value))]
        pub fn full<'py>(py: Python<'py>, shape: Vec<usize>, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Tensor::full(shape, value);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Returns the byte at a multi-index.
        #[pyfunction]
        #[pyo3(name = "get", signature = (tensor, multi))]
        pub fn get<'py>(py: Python<'py>, tensor: PyTensor, multi: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::get(&tensor, &multi);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Wraps an i32 vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "i32", signature = (data, shape))]
        pub fn i32_<'py>(py: Python<'py>, data: Vec<i32>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Tensor::i32(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as i32s.
        #[pyfunction]
        #[pyo3(name = "i32s", signature = (tensor))]
        pub fn i32s<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::i32s(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Folds a multi-index into its flat index.
        #[pyfunction]
        #[pyo3(name = "index", signature = (tensor, multi))]
        pub fn index<'py>(py: Python<'py>, tensor: PyTensor, multi: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::index(&tensor, &multi);
            (out).into_bound_py_any(py)
        }

        /// Flips every element between zero and one.
        #[pyfunction]
        #[pyo3(name = "invert", signature = (tensor))]
        pub fn invert<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::invert(&tensor);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds the Kronecker product of the two tensors.
        #[pyfunction]
        #[pyo3(name = "kron", signature = (tensor, other))]
        pub fn kron<'py>(py: Python<'py>, tensor: PyTensor, other: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let other = other.0;
            let out = mrlyrs::core::Tensor::kron(&tensor, &other);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Numbers every position by its concentric ring out from the center.
        #[pyfunction]
        #[pyo3(name = "layers", signature = (tensor, dtype))]
        pub fn layers<'py>(py: Python<'py>, tensor: PyTensor, dtype: PySerde<mrlyrs::core::Dtype>) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let dtype = dtype.0;
            let out = mrlyrs::core::Tensor::layers(&tensor, dtype);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Counts each position's masked neighbors holding the target bit.
        #[pyfunction]
        #[pyo3(name = "neighbors", signature = (tensor, mask, target, wrap, dtype))]
        pub fn neighbors<'py>(py: Python<'py>, tensor: PyTensor, mask: PyTensor, target: u8, wrap: bool, dtype: PySerde<mrlyrs::core::Dtype>) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let mask = mask.0;
            let dtype = dtype.0;
            let out = mrlyrs::core::Tensor::neighbors(&tensor, &mask, target, wrap, dtype);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a zeroed u8 tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "new", signature = (shape))]
        pub fn new<'py>(py: Python<'py>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Tensor::new(shape);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Wraps a byte vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "of", signature = (data, shape))]
        pub fn of<'py>(py: Python<'py>, data: Vec<u8>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Tensor::of(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Otsu threshold splitting the histogram at greatest variance.
        #[pyfunction]
        #[pyo3(name = "otsu_threshold", signature = (tensor))]
        pub fn otsu_threshold<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::otsu_threshold(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Wraps the tensor in a count-thick border of one value.
        #[pyfunction]
        #[pyo3(name = "pad", signature = (tensor, count, value))]
        pub fn pad<'py>(py: Python<'py>, tensor: PyTensor, count: usize, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::pad(&tensor, count, value);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Stamps the value wherever the tiled mask is nonzero.
        #[pyfunction]
        #[pyo3(name = "perforate", signature = (tensor, mask, value))]
        pub fn perforate<'py>(py: Python<'py>, tensor: PyTensor, mask: PyTensor, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let mask = mask.0;
            let out = mrlyrs::core::Tensor::perforate(&tensor, &mask, value);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Writes the element at a flat index, which must be below the size like a slice index.
        #[pyfunction]
        #[pyo3(name = "put", signature = (tensor, flat, value))]
        pub fn put<'py>(py: Python<'py>, tensor: &Bound<'_, PyAny>, flat: usize, value: i64) -> PyResult<Bound<'py, PyAny>> {
            let mut tensor_owned = crate::hand::tensor_from_py(tensor)?;
            mrlyrs::core::Tensor::put(&mut tensor_owned, flat, value);
            crate::hand::tensor_write_back(tensor, &tensor_owned)?;
            ().into_bound_py_any(py)
        }

        /// Rotates the tensor k quarter turns in the plane of two axes.
        #[pyfunction]
        #[pyo3(name = "rot90", signature = (tensor, k, axes))]
        pub fn rot90<'py>(py: Python<'py>, tensor: PyTensor, k: usize, axes: (usize, usize)) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::rot90(&tensor, k, axes);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Writes the byte at a multi-index.
        #[pyfunction]
        #[pyo3(name = "set", signature = (tensor, multi, value))]
        pub fn set<'py>(py: Python<'py>, tensor: &Bound<'_, PyAny>, multi: Vec<usize>, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let mut tensor_owned = crate::hand::tensor_from_py(tensor)?;
            let out = mrlyrs::core::Tensor::set(&mut tensor_owned, &multi, value);
            crate::hand::tensor_write_back(tensor, &tensor_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the number of elements.
        #[pyfunction]
        #[pyo3(name = "size", signature = (tensor))]
        pub fn size<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::size(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Drops one axis by fixing it at an index.
        #[pyfunction]
        #[pyo3(name = "slice", signature = (tensor, axis, index))]
        pub fn slice<'py>(py: Python<'py>, tensor: PyTensor, axis: usize, index: usize) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::slice(&tensor, axis, index);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the sum of all elements.
        #[pyfunction]
        #[pyo3(name = "sum", signature = (tensor))]
        pub fn sum<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::sum(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Repeats the tensor the given number of times along each axis.
        #[pyfunction]
        #[pyo3(name = "tile", signature = (tensor, reps))]
        pub fn tile<'py>(py: Python<'py>, tensor: PyTensor, reps: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::tile(&tensor, &reps);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Swaps two axes.
        #[pyfunction]
        #[pyo3(name = "transpose", signature = (tensor, a, b))]
        pub fn transpose<'py>(py: Python<'py>, tensor: PyTensor, a: usize, b: usize) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::transpose(&tensor, a, b);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a zeroed tensor of the shape and width.
        #[pyfunction]
        #[pyo3(name = "typed", signature = (shape, dtype))]
        pub fn typed<'py>(py: Python<'py>, shape: Vec<usize>, dtype: PySerde<mrlyrs::core::Dtype>) -> PyResult<Bound<'py, PyAny>> {
            let dtype = dtype.0;
            let out = mrlyrs::core::Tensor::typed(shape, dtype);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Wraps a u16 vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "u16", signature = (data, shape))]
        pub fn u16_<'py>(py: Python<'py>, data: Vec<u16>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Tensor::u16(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as u16s.
        #[pyfunction]
        #[pyo3(name = "u16s", signature = (tensor))]
        pub fn u16s<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::u16s(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Wraps a u32 vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "u32", signature = (data, shape))]
        pub fn u32_<'py>(py: Python<'py>, data: Vec<u32>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Tensor::u32(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as u32s.
        #[pyfunction]
        #[pyo3(name = "u32s", signature = (tensor))]
        pub fn u32s<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrlyrs::core::Tensor::u32s(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Wraps a u8 vector as a tensor of the shape, the same door as [`Tensor::of`].
        #[pyfunction]
        #[pyo3(name = "u8", signature = (data, shape))]
        pub fn u8_<'py>(py: Python<'py>, data: Vec<u8>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Tensor::u8(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.core.tensor")?;
            m.setattr("__doc__", "The tensor and its dtypes.")?;
            m.add_function(wrap_pyfunction!(at, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize_otsu, &m)?)?;
            m.add_function(wrap_pyfunction!(blur, &m)?)?;
            m.add_function(wrap_pyfunction!(bytes, &m)?)?;
            m.add_function(wrap_pyfunction!(count, &m)?)?;
            m.add_function(wrap_pyfunction!(dtype, &m)?)?;
            m.add_function(wrap_pyfunction!(exposed, &m)?)?;
            m.add_function(wrap_pyfunction!(filled, &m)?)?;
            m.add_function(wrap_pyfunction!(flip, &m)?)?;
            m.add_function(wrap_pyfunction!(fractal, &m)?)?;
            m.add_function(wrap_pyfunction!(full, &m)?)?;
            m.add_function(wrap_pyfunction!(get, &m)?)?;
            m.add_function(wrap_pyfunction!(i32_, &m)?)?;
            m.add_function(wrap_pyfunction!(i32s, &m)?)?;
            m.add_function(wrap_pyfunction!(index, &m)?)?;
            m.add_function(wrap_pyfunction!(invert, &m)?)?;
            m.add_function(wrap_pyfunction!(kron, &m)?)?;
            m.add_function(wrap_pyfunction!(layers, &m)?)?;
            m.add_function(wrap_pyfunction!(neighbors, &m)?)?;
            m.add_function(wrap_pyfunction!(new, &m)?)?;
            m.add_function(wrap_pyfunction!(of, &m)?)?;
            m.add_function(wrap_pyfunction!(otsu_threshold, &m)?)?;
            m.add_function(wrap_pyfunction!(pad, &m)?)?;
            m.add_function(wrap_pyfunction!(perforate, &m)?)?;
            m.add_function(wrap_pyfunction!(put, &m)?)?;
            m.add_function(wrap_pyfunction!(rot90, &m)?)?;
            m.add_function(wrap_pyfunction!(set, &m)?)?;
            m.add_function(wrap_pyfunction!(size, &m)?)?;
            m.add_function(wrap_pyfunction!(slice, &m)?)?;
            m.add_function(wrap_pyfunction!(sum, &m)?)?;
            m.add_function(wrap_pyfunction!(tile, &m)?)?;
            m.add_function(wrap_pyfunction!(transpose, &m)?)?;
            m.add_function(wrap_pyfunction!(typed, &m)?)?;
            m.add_function(wrap_pyfunction!(u16_, &m)?)?;
            m.add_function(wrap_pyfunction!(u16s, &m)?)?;
            m.add_function(wrap_pyfunction!(u32_, &m)?)?;
            m.add_function(wrap_pyfunction!(u32s, &m)?)?;
            m.add_function(wrap_pyfunction!(u8_, &m)?)?;
            let names: Vec<&str> = vec!["at", "binarize", "binarize_otsu", "blur", "bytes", "count", "dtype", "exposed", "filled", "flip", "fractal", "full", "get", "i32", "i32s", "index", "invert", "kron", "layers", "neighbors", "new", "of", "otsu_threshold", "pad", "perforate", "put", "rot90", "set", "size", "slice", "sum", "tile", "transpose", "typed", "u16", "u16s", "u32", "u32s", "u8"];
            m.add("__all__", names)?;
            parent.add("tensor", &m)?;
            sys.set_item("mrlypy._mrlypy.core.tensor", &m)?;
            Ok(())
        }
    }

    /// A paletted image: rows of palette indices and the palette they point into, hex strings in json.
    #[pyclass(name = "Image", module = "mrlypy.core", from_py_object)]
    #[derive(Clone)]
    pub struct Image(pub mrlyrs::core::Image);

    #[pymethods]
    impl Image {
        /// Builds an image from its four parts.
        #[new]
        #[pyo3(signature = (width, height, rows, palette))]
        pub fn __new__(width: usize, height: usize, rows: Vec<Vec<usize>>, palette: Vec<PyColor>) -> PyResult<Self> {
            let palette = palette.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::Image::new(width, height, rows, palette);
            Ok(Self(out))
        }
        /// The width in pixels.
        #[getter]
        #[pyo3(name = "width")]
        pub fn width<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.width;
            (value).into_bound_py_any(py)
        }
        /// The height in pixels.
        #[getter]
        #[pyo3(name = "height")]
        pub fn height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.height;
            (value).into_bound_py_any(py)
        }
        /// The palette index of every pixel, row by row.
        #[getter]
        #[pyo3(name = "rows")]
        pub fn rows<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.rows.clone();
            (value).into_bound_py_any(py)
        }
        /// The colors the rows index.
        #[getter]
        #[pyo3(name = "palette")]
        pub fn palette<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.palette.clone();
            ((value).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// Returns the flat rgba pixels, transparent wherever an index misses the palette.
        #[pyo3(name = "colors", signature = ())]
        pub fn colors<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Image::colors(&self.0);
            (PyPixels(out)).into_bound_py_any(py)
        }
        /// Builds a paletted image from raw rgba pixels, growing the palette as new colors appear.
        #[staticmethod]
        #[pyo3(name = "from_pixels", signature = (width, height, pixels))]
        pub fn from_pixels<'py>(py: Python<'py>, width: usize, height: usize, pixels: PyPixels) -> PyResult<Bound<'py, PyAny>> {
            let pixels = pixels.0;
            let out = mrlyrs::core::Image::from_pixels(width, height, &pixels);
            (crate::gen::core::Image(out)).into_bound_py_any(py)
        }
        /// Builds an image from its four parts.
        #[staticmethod]
        #[pyo3(name = "new", signature = (width, height, rows, palette))]
        pub fn new_<'py>(py: Python<'py>, width: usize, height: usize, rows: Vec<Vec<usize>>, palette: Vec<PyColor>) -> PyResult<Bound<'py, PyAny>> {
            let palette = palette.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::Image::new(width, height, rows, palette);
            (crate::gen::core::Image(out)).into_bound_py_any(py)
        }
        /// Encodes the image as a png at the given scale.
        #[pyo3(name = "png", signature = (scale))]
        pub fn png<'py>(&self, py: Python<'py>, scale: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Image::png(&self.0, scale);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Resamples the image to a new size, its palette rebuilt from the blended pixels.
        #[pyo3(name = "resample", signature = (width, height, filter))]
        pub fn resample<'py>(&self, py: Python<'py>, width: usize, height: usize, filter: PySerde<mrlyrs::core::Filter>) -> PyResult<Bound<'py, PyAny>> {
            let filter = filter.0;
            let out = mrlyrs::core::Image::resample(&self.0, width, height, filter);
            (crate::gen::core::Image(ok(out)?)).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// A rule that turns counter values into colors.
    #[pyclass(name = "Colorizer", module = "mrlypy.core", skip_from_py_object)]
    pub struct Colorizer;

    #[pymethods]
    impl Colorizer {
        /// Builds the blue-to-red diverging ramp around a white middle.
        #[staticmethod]
        #[pyo3(name = "diverge", signature = ())]
        pub fn diverge<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Colorizer::diverge();
            (PySerde(out)).into_bound_py_any(py)
        }
        /// Builds the black-through-ember fire ramp: black, dark red, orange, light yellow.
        #[staticmethod]
        #[pyo3(name = "fire", signature = ())]
        pub fn fire<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Colorizer::fire();
            (PySerde(out)).into_bound_py_any(py)
        }
        /// Builds a binned colorizer from a gradient through the given stops.
        #[staticmethod]
        #[pyo3(name = "gradient_bins", signature = (background, colors, shades))]
        pub fn gradient_bins<'py>(py: Python<'py>, background: PyColor, colors: Vec<PyColor>, shades: usize) -> PyResult<Bound<'py, PyAny>> {
            let background = background.0;
            let colors = colors.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::core::Colorizer::gradient_bins(background, &colors, shades);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }
        /// Builds the white-to-black heat ramp.
        #[staticmethod]
        #[pyo3(name = "heat", signature = ())]
        pub fn heat<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::core::Colorizer::heat();
            (PySerde(out)).into_bound_py_any(py)
        }
    }

    /// The element widths a tensor can hold.
    #[pyclass(name = "Dtype", module = "mrlypy.core", skip_from_py_object)]
    pub struct Dtype;

    #[pymethods]
    impl Dtype {
        /// Returns the largest value the width can hold.
        #[staticmethod]
        #[pyo3(name = "max", signature = (dtype))]
        pub fn max<'py>(py: Python<'py>, dtype: PySerde<mrlyrs::core::Dtype>) -> PyResult<Bound<'py, PyAny>> {
            let dtype = dtype.0;
            let out = mrlyrs::core::Dtype::max(dtype);
            (out).into_bound_py_any(py)
        }
    }

    /// Squashes rgba pixels to the hex aspect, returning the new width, height and pixels.
    #[pyfunction]
    #[pyo3(name = "hex_fit", signature = (pixels, width, height, vertical, filter))]
    pub fn hex_fit<'py>(py: Python<'py>, pixels: PyPixels, width: usize, height: usize, vertical: bool, filter: PySerde<mrlyrs::core::Filter>) -> PyResult<Bound<'py, PyAny>> {
        let pixels = pixels.0;
        let filter = filter.0;
        let out = mrlyrs::core::hex_fit(&pixels, width, height, vertical, filter);
        ({ let t = ok(out)?; (t.0, t.1, PyPixels(t.2)) }).into_bound_py_any(py)
    }

    /// Returns the size a hex rendering wears, the named axis squashed by the triangle ratio.
    #[pyfunction]
    #[pyo3(name = "hex_size", signature = (width, height, vertical))]
    pub fn hex_size<'py>(py: Python<'py>, width: usize, height: usize, vertical: bool) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::core::hex_size(width, height, vertical);
        (out).into_bound_py_any(py)
    }

    /// Resamples rgba pixels to a new size.
    #[pyfunction]
    #[pyo3(name = "resample", signature = (pixels, width, height, out_w, out_h, filter))]
    pub fn resample<'py>(py: Python<'py>, pixels: PyPixels, width: usize, height: usize, out_w: usize, out_h: usize, filter: PySerde<mrlyrs::core::Filter>) -> PyResult<Bound<'py, PyAny>> {
        let pixels = pixels.0;
        let filter = filter.0;
        let out = mrlyrs::core::resample(&pixels, width, height, out_w, out_h, filter);
        (PyPixels(ok(out)?)).into_bound_py_any(py)
    }

    /// Decodes a png to its width, height, and rgba colors.
    #[pyfunction]
    #[pyo3(name = "unpng", signature = (bytes))]
    pub fn unpng<'py>(py: Python<'py>, bytes: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::core::unpng(&bytes);
        ({ let t = ok(out)?; (t.0, t.1, PyPixels(t.2)) }).into_bound_py_any(py)
    }

    pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
        let m = PyModule::new(py, "mrlypy.core")?;
        m.setattr("__doc__", "The substrate: tensors, cells, colors, images, codecs, resampling and seeded chance.\nThe substrate: the road from a grid of bytes to pixels, with nothing mrly on it.\n\n- `tensor` makes and tallies the byte grids; `cell` dresses one in colors and tags.\n- `colors` holds the rgba color, the two themes and the house palette; `paint` spreads a palette over a cell.\n- `ramp` turns counter values into colors; `resample` rescales pixels and squashes them for hex.\n- `image` holds the paletted pixels; `codec` writes them as a png or a gif and reads a png back.\n- `rng` deals seeded chance: one xoshiro256++ stream, passed by hand, never global.\n- `error` holds the one error, its Result and the json parser; `named` names an enum.\n\nThe json value and map are serde_json's, kept under `preserve_order` so an object comes back in the order it was written and both bridges print the same text.\n\nThe doors: [`Tensor::of`](crate::core::tensor::Tensor::of), [`Tensor::rot90`](crate::core::tensor::Tensor::rot90), [`Color::from_hex`](crate::core::colors::Color::from_hex), [`Color::to_hex`](crate::core::colors::Color::to_hex), [`png`](crate::core::png()), [`unpng`](crate::core::unpng()), [`gif`](crate::core::gif()) and [`Colorizer::color`](crate::core::ramp::Colorizer::color).")?;
        m.add_class::<PyRng>()?;
        m.add_class::<Image>()?;
        m.add_class::<Colorizer>()?;
        m.add_class::<Dtype>()?;
        m.add_function(wrap_pyfunction!(hex_fit, &m)?)?;
        m.add_function(wrap_pyfunction!(hex_size, &m)?)?;
        m.add_function(wrap_pyfunction!(resample, &m)?)?;
        m.add_function(wrap_pyfunction!(unpng, &m)?)?;
        m.add("HEX_RATIO", mrlyrs::core::HEX_RATIO)?;
        m.add("PNG_MAGIC", mrlyrs::core::PNG_MAGIC)?;
        let names: Vec<&str> = vec!["hex_fit", "hex_size", "resample", "unpng", "Image", "Colorizer", "Dtype", "HEX_RATIO", "PNG_MAGIC", "Rng"];
        m.add("__all__", names)?;
        cell::init(py, &m, sys)?;
        codec::init(py, &m, sys)?;
        colors::init(py, &m, sys)?;
        error::init(py, &m, sys)?;
        image::init(py, &m, sys)?;
        paint::init(py, &m, sys)?;
        ramp::init(py, &m, sys)?;
        rng::init(py, &m, sys)?;
        tensor::init(py, &m, sys)?;
        parent.add("core", &m)?;
        sys.set_item("mrlypy._mrlypy.core", &m)?;
        Ok(())
    }
}

/// The alphabet: the stroked pixel glyphs, their rasters and their writing animations.
/// The alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke.
pub mod font {
    use crate::hand::{PySerde};
    use pyo3::prelude::*;
    use pyo3::types::PyDict;
    use pyo3::IntoPyObjectExt;

    /// The stroke orders that write each character.
    pub mod paths {
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the character's hand-penned strokes from the pen tables, or None outside the font.
        #[pyfunction]
        #[pyo3(name = "penned", signature = (c))]
        pub fn penned<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::font::paths::penned(c);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.font.paths")?;
            m.setattr("__doc__", "The stroke orders that write each character.")?;
            m.add_function(wrap_pyfunction!(penned, &m)?)?;
            let names: Vec<&str> = vec!["penned"];
            m.add("__all__", names)?;
            parent.add("paths", &m)?;
            sys.set_item("mrlypy._mrlypy.font.paths", &m)?;
            Ok(())
        }
    }

    /// The hand-penned stroke tables, one per glyph.
    pub mod pens {
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns every pen in font order: uppers, lowers, digits, extras, specials.
        #[pyfunction]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::font::pens::all();
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.font.pens")?;
            m.setattr("__doc__", "The hand-penned stroke tables, one per glyph.")?;
            m.add_function(wrap_pyfunction!(all, &m)?)?;
            let names: Vec<&str> = vec!["all"];
            m.add("__all__", names)?;
            parent.add("pens", &m)?;
            sys.set_item("mrlypy._mrlypy.font.pens", &m)?;
            Ok(())
        }
    }

    /// One character's pixel bitmap.
    #[pyclass(name = "Glyph", module = "mrlypy.font", from_py_object)]
    #[derive(Clone)]
    pub struct Glyph(pub mrlyrs::font::Glyph);

    #[pymethods]
    impl Glyph {
        /// Builds a glyph from its character and rows.
        #[new]
        #[pyo3(signature = (char_, rows))]
        pub fn __new__(char_: char, rows: Vec<String>) -> PyResult<Self> {
            let out = mrlyrs::font::Glyph::new(char_, rows);
            Ok(Self(out))
        }
        /// The character the glyph draws.
        #[getter]
        #[pyo3(name = "char")]
        pub fn char_<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.char;
            (value).into_bound_py_any(py)
        }
        /// The bitmap rows of '0' and '1' characters.
        #[getter]
        #[pyo3(name = "rows")]
        pub fn rows<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.rows.clone();
            (value).into_bound_py_any(py)
        }
        /// Returns the number of rows.
        #[pyo3(name = "height", signature = ())]
        pub fn height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::font::Glyph::height(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Builds a glyph from its character and rows.
        #[staticmethod]
        #[pyo3(name = "new", signature = (char_, rows))]
        pub fn new_<'py>(py: Python<'py>, char_: char, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::font::Glyph::new(char_, rows);
            (crate::gen::font::Glyph(out)).into_bound_py_any(py)
        }
        /// Returns the cell width of the first row, or 0 for an empty glyph.
        #[pyo3(name = "width", signature = ())]
        pub fn width<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::font::Glyph::width(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// Builds every glyph in font order: uppers, lowers, digits, extras, specials.
    #[pyfunction]
    #[pyo3(name = "all", signature = ())]
    pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::all();
        ((out).into_iter().map(crate::gen::font::Glyph).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster.
    #[pyfunction]
    #[pyo3(name = "animate", signature = (text, pad))]
    pub fn animate<'py>(py: Python<'py>, text: &str, pad: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::animate(text, pad);
        (PySerde(out)).into_bound_py_any(py)
    }

    /// Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any.
    #[pyfunction]
    #[pyo3(name = "cycle", signature = (write, merge, hold))]
    pub fn cycle<'py>(py: Python<'py>, write: PySerde<mrlyrs::font::Anim>, merge: Vec<Vec<usize>>, hold: usize) -> PyResult<Bound<'py, PyAny>> {
        let write = write.0;
        let out = mrlyrs::font::cycle(&write, &merge, hold);
        (PySerde(out)).into_bound_py_any(py)
    }

    /// Builds the ten digit glyphs.
    #[pyfunction]
    #[pyo3(name = "digits", signature = ())]
    pub fn digits<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::digits();
        ((out).into_iter().map(crate::gen::font::Glyph).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck.
    #[pyfunction]
    #[pyo3(name = "draft", signature = (rows))]
    pub fn draft<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::draft(&rows);
        (out).into_bound_py_any(py)
    }

    /// Builds the punctuation, symbol and arrow glyphs.
    #[pyfunction]
    #[pyo3(name = "extras", signature = ())]
    pub fn extras<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::extras();
        ((out).into_iter().map(crate::gen::font::Glyph).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank.
    #[pyfunction]
    #[pyo3(name = "floor", signature = (rows))]
    pub fn floor<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::floor(&rows);
        (out).into_bound_py_any(py)
    }

    /// Returns an owned copy of the character's glyph, or None outside the font.
    #[pyfunction]
    #[pyo3(name = "glyph", signature = (c))]
    pub fn glyph<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::glyph(c);
        ((out).map(crate::gen::font::Glyph)).into_bound_py_any(py)
    }

    /// Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form.
    #[pyfunction]
    #[pyo3(name = "lower", signature = (rows))]
    pub fn lower<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let rows = rows.iter().map(|y| y.as_str()).collect::<Vec<_>>();
        let out = mrlyrs::font::lower(&rows);
        (out).into_bound_py_any(py)
    }

    /// Builds the twenty-six lowercase glyphs by rounding the uppers' corners.
    #[pyfunction]
    #[pyo3(name = "lowers", signature = ())]
    pub fn lowers<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::lowers();
        ((out).into_iter().map(crate::gen::font::Glyph).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the whole font as a map from character to bitmap rows.
    #[pyfunction]
    #[pyo3(name = "map", signature = ())]
    pub fn map<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::map();
        (out).into_bound_py_any(py)
    }

    /// Folds the written text's glyphs, frame by frame, into one centered stack.
    #[pyfunction]
    #[pyo3(name = "merge", signature = (text, pad))]
    pub fn merge<'py>(py: Python<'py>, text: &str, pad: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::merge(text, pad);
        (out).into_bound_py_any(py)
    }

    /// Returns the character's Unicode name, or a U+ code point label for a character outside the font.
    #[pyfunction]
    #[pyo3(name = "name_of", signature = (c))]
    pub fn name_of<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::name_of(c);
        (out).into_bound_py_any(py)
    }

    /// Flattens the character's strokes into one cell-by-cell drawing order.
    #[pyfunction]
    #[pyo3(name = "path", signature = (c))]
    pub fn path<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::path(c);
        (out).into_bound_py_any(py)
    }

    /// Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart.
    #[pyfunction]
    #[pyo3(name = "raster", signature = (text))]
    pub fn raster<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::raster(text);
        (out).into_bound_py_any(py)
    }

    /// Builds the four seven-row glyphs: dollar, at, copyright and registered.
    #[pyfunction]
    #[pyo3(name = "specials", signature = ())]
    pub fn specials<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::specials();
        ((out).into_iter().map(crate::gen::font::Glyph).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font.
    #[pyfunction]
    #[pyo3(name = "strokes", signature = (c))]
    pub fn strokes<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::strokes(c);
        (out).into_bound_py_any(py)
    }

    /// Returns every character in the font, in font order.
    #[pyfunction]
    #[pyo3(name = "supported", signature = ())]
    pub fn supported<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::supported();
        (out).into_bound_py_any(py)
    }

    /// Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has.
    #[pyfunction]
    #[pyo3(name = "trim", signature = (rows))]
    pub fn trim<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::trim(&rows);
        (out).into_bound_py_any(py)
    }

    /// Builds the twenty-six uppercase glyphs.
    #[pyfunction]
    #[pyo3(name = "uppers", signature = ())]
    pub fn uppers<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::font::uppers();
        ((out).into_iter().map(crate::gen::font::Glyph).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
        let m = PyModule::new(py, "mrlypy.font")?;
        m.setattr("__doc__", "The alphabet: the stroked pixel glyphs, their rasters and their writing animations.\nThe alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke.\n\n- `bitmaps` holds the raw on/off rows; `glyph` builds them into glyphs and trims them.\n- `pens` holds every glyph's hand-penned strokes; `paths` reads them and drafts new ones.\n- `names` gives a character its Unicode name.\n- `raster` lays a text out as one 0/1 grid; `animate` writes it cell by cell and loops the cycle.\n\nThe font is the uppers, their corner-rounded lowers, the digits, the punctuation and arrows, and four specials; most glyphs are five rows of five, the ten that dip below the baseline are seven.\n\nThe doors: [`glyph`](crate::font::glyph()), [`supported`](crate::font::supported()), [`map`](crate::font::map()), [`raster`](crate::font::raster()), [`path`](crate::font::path()), [`floor`](crate::font::floor()), [`animate`](crate::font::animate()), [`merge`](crate::font::merge()), [`cycle`](crate::font::cycle()) and [`name_of`](crate::font::name_of()).\n\n`cargo run -p mrlyrs --example pen` prints the pen tables; `-- X` drafts one glyph and its stroke floor.")?;
        m.add_class::<Glyph>()?;
        m.add_function(wrap_pyfunction!(all, &m)?)?;
        m.add_function(wrap_pyfunction!(animate, &m)?)?;
        m.add_function(wrap_pyfunction!(cycle, &m)?)?;
        m.add_function(wrap_pyfunction!(digits, &m)?)?;
        m.add_function(wrap_pyfunction!(draft, &m)?)?;
        m.add_function(wrap_pyfunction!(extras, &m)?)?;
        m.add_function(wrap_pyfunction!(floor, &m)?)?;
        m.add_function(wrap_pyfunction!(glyph, &m)?)?;
        m.add_function(wrap_pyfunction!(lower, &m)?)?;
        m.add_function(wrap_pyfunction!(lowers, &m)?)?;
        m.add_function(wrap_pyfunction!(map, &m)?)?;
        m.add_function(wrap_pyfunction!(merge, &m)?)?;
        m.add_function(wrap_pyfunction!(name_of, &m)?)?;
        m.add_function(wrap_pyfunction!(path, &m)?)?;
        m.add_function(wrap_pyfunction!(raster, &m)?)?;
        m.add_function(wrap_pyfunction!(specials, &m)?)?;
        m.add_function(wrap_pyfunction!(strokes, &m)?)?;
        m.add_function(wrap_pyfunction!(supported, &m)?)?;
        m.add_function(wrap_pyfunction!(trim, &m)?)?;
        m.add_function(wrap_pyfunction!(uppers, &m)?)?;
        m.add("FPS", mrlyrs::font::FPS)?;
        m.add("HOLD", mrlyrs::font::HOLD)?;
        let names: Vec<&str> = vec!["all", "animate", "cycle", "digits", "draft", "extras", "floor", "glyph", "lower", "lowers", "map", "merge", "name_of", "path", "raster", "specials", "strokes", "supported", "trim", "uppers", "Glyph", "FPS", "HOLD"];
        m.add("__all__", names)?;
        paths::init(py, &m, sys)?;
        pens::init(py, &m, sys)?;
        parent.add("font", &m)?;
        sys.set_item("mrlypy._mrlypy.font", &m)?;
        Ok(())
    }
}

/// The generator: the whole pipeline from a recipe to a file.
/// The generator: the pipeline from a recipe to a file, for datasets, the automator, backgrounds.
pub mod gen_ {
    use crate::hand::{ok, PyRng, PySerde, PyTensor};
    use pyo3::prelude::*;
    use pyo3::types::PyDict;
    use pyo3::IntoPyObjectExt;

    /// The recipe to cell builders, one per dimension, and the random draws that feed them.
    pub mod build {
        use crate::hand::{ok, PyCell6d, PyCellNd, PyRng, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Builds the flat cell the tile describes.
        #[pyfunction]
        #[pyo3(name = "build_2d", signature = (tile))]
        pub fn build_2d<'py>(py: Python<'py>, tile: PyRef<'_, crate::gen::gen_::Tile>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::build::build_2d(&tile.0);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the cube the tile describes.
        #[pyfunction]
        #[pyo3(name = "build_3d", signature = (tile))]
        pub fn build_3d<'py>(py: Python<'py>, tile: PyRef<'_, crate::gen::gen_::Tile>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::build::build_3d(&tile.0);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the tile's cube and flattens it through its projection.
        #[pyfunction]
        #[pyo3(name = "build_6d", signature = (hex))]
        pub fn build_6d<'py>(py: Python<'py>, hex: PySerde<mrlyrs::gen::build::HexTile>) -> PyResult<Bound<'py, PyAny>> {
            let hex = hex.0;
            let out = mrlyrs::gen::build::build_6d(&hex);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a random flat tile from the stream, rotations from the four quarter-turns.
        #[pyfunction]
        #[pyo3(name = "create_2d", signature = (config, rng))]
        pub fn create_2d<'py>(py: Python<'py>, config: &Bound<'_, PyAny>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let config = crate::hand::serde_from_py(config)?;
            let out = mrlyrs::gen::build::create_2d(&config, &mut rng.0);
            (crate::gen::gen_::Tile(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a cube tile from the config with cube orientations drawn from the stream.
        #[pyfunction]
        #[pyo3(name = "create_3d", signature = (config, rng))]
        pub fn create_3d<'py>(py: Python<'py>, config: &Bound<'_, PyAny>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let config = crate::hand::serde_from_py(config)?;
            let out = mrlyrs::gen::build::create_3d(&config, &mut rng.0);
            (crate::gen::gen_::Tile(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a cube tile from the config under a projection drawn from the stream.
        #[pyfunction]
        #[pyo3(name = "create_6d", signature = (config, rng))]
        pub fn create_6d<'py>(py: Python<'py>, config: &Bound<'_, PyAny>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let config = crate::hand::serde_from_py(config)?;
            let out = mrlyrs::gen::build::create_6d(&config, &mut rng.0);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a random flat tile up to the given size under the default config.
        #[pyfunction]
        #[pyo3(name = "random_tile_2d", signature = (max_size, rng))]
        pub fn random_tile_2d<'py>(py: Python<'py>, max_size: usize, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::build::random_tile_2d(max_size, &mut rng.0);
            (crate::gen::gen_::Tile(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a random cube tile up to the given size.
        #[pyfunction]
        #[pyo3(name = "random_tile_3d", signature = (max_size, rng))]
        pub fn random_tile_3d<'py>(py: Python<'py>, max_size: usize, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::build::random_tile_3d(max_size, &mut rng.0);
            (crate::gen::gen_::Tile(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a random cube tile up to the given size under a random projection.
        #[pyfunction]
        #[pyo3(name = "random_tile_6d", signature = (max_size, rng))]
        pub fn random_tile_6d<'py>(py: Python<'py>, max_size: usize, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::build::random_tile_6d(max_size, &mut rng.0);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.gen.build")?;
            m.setattr("__doc__", "The recipe to cell builders, one per dimension, and the random draws that feed them.")?;
            m.add_function(wrap_pyfunction!(build_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(build_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(build_6d, &m)?)?;
            m.add_function(wrap_pyfunction!(create_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(create_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(create_6d, &m)?)?;
            m.add_function(wrap_pyfunction!(random_tile_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(random_tile_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(random_tile_6d, &m)?)?;
            let names: Vec<&str> = vec!["build_2d", "build_3d", "build_6d", "create_2d", "create_3d", "create_6d", "random_tile_2d", "random_tile_3d", "random_tile_6d"];
            m.add("__all__", names)?;
            parent.add("build", &m)?;
            sys.set_item("mrlypy._mrlypy.gen.build", &m)?;
            Ok(())
        }
    }

    /// The tile name: a full tile recipe folded to its one canonical object.
    pub mod name {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A tile recipe folded to its one canonical object.
        #[pyclass(name = "Tile", module = "mrlypy.gen.name", from_py_object)]
        #[derive(Clone)]
        pub struct Tile(pub mrlyrs::gen::name::Tile);

        #[pymethods]
        impl Tile {
            /// The one design of a flat or fractal tile.
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            /// The mask code of a special tile.
            #[getter]
            #[pyo3(name = "special")]
            pub fn special<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.special;
                (value).into_bound_py_any(py)
            }
            /// The letters of a magic tile, first letter outermost.
            #[getter]
            #[pyo3(name = "magic")]
            pub fn magic<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.magic.clone();
                (value).into_bound_py_any(py)
            }
            /// The three codes of a mosaic tile.
            #[getter]
            #[pyo3(name = "mosaic")]
            pub fn mosaic<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.mosaic.clone();
                (value).into_bound_py_any(py)
            }
            /// The side of the mask of a special or mosaic tile.
            #[getter]
            #[pyo3(name = "factor")]
            pub fn factor<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.factor;
                (value).into_bound_py_any(py)
            }
            /// The side each slot renders at, one per letter for a magic tile.
            #[getter]
            #[pyo3(name = "side")]
            pub fn side<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.side.clone();
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The power a fractal tile is raised to, absent at one.
            #[getter]
            #[pyo3(name = "level")]
            pub fn level<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.level;
                (value).into_bound_py_any(py)
            }
            /// The quarter turns of each slot, absent when nothing turns.
            #[getter]
            #[pyo3(name = "turn")]
            pub fn turn<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.turn.clone();
                (PySerde(value)).into_bound_py_any(py)
            }
            /// Whether a special tile flips its mask.
            #[getter]
            #[pyo3(name = "flip")]
            pub fn flip<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.flip;
                (value).into_bound_py_any(py)
            }
            /// Whether the finished tile inverts.
            #[getter]
            #[pyo3(name = "invert")]
            pub fn invert<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.invert;
                (value).into_bound_py_any(py)
            }
            /// Folds a decoded value to its canonical form, or an error for one outside the kind.
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::checked(self.0.clone());
                (crate::gen::gen_::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a filename back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_file(text);
                (crate::gen::gen_::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a JSON object into its canonical value, or an error naming the broken key.
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_json(text);
                (crate::gen::gen_::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a path and query string back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_url(text);
                (crate::gen::gen_::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            /// Folds a recipe to its name.
            #[staticmethod]
            #[pyo3(name = "of", signature = (recipe))]
            pub fn of<'py>(py: Python<'py>, recipe: PyRef<'_, crate::gen::gen_::Tile>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::gen::name::Tile::of(&recipe.0);
                (crate::gen::gen_::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            /// Builds the recipe the name folds, resized and checked.
            #[pyo3(name = "recipe", signature = ())]
            pub fn recipe<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::gen::name::Tile::recipe(&self.0);
                (crate::gen::gen_::Tile(ok(out)?)).into_bound_py_any(py)
            }
            /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the first eight hex digits of the sha256 of the canonical JSON.
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the canonical JSON object.
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
            #[pyo3(name = "to_mrly", signature = ())]
            pub fn to_mrly<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_mrly(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_url(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.gen.name")?;
            m.setattr("__doc__", "The tile name: a full tile recipe folded to its one canonical object.")?;
            m.add_class::<Tile>()?;
            let names: Vec<&str> = vec!["Tile"];
            m.add("__all__", names)?;
            parent.add("name", &m)?;
            sys.set_item("mrlypy._mrlypy.gen.name", &m)?;
            Ok(())
        }
    }

    /// The tile recipe: its families, groups, parities, catalog and size lists.
    pub mod recipe {
        use crate::hand::{PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The named designs a source can point at: the four classics and their four antis.
        #[pyclass(name = "Design", module = "mrlypy.gen.recipe", skip_from_py_object)]
        pub struct Design;

        #[pymethods]
        impl Design {
            /// Returns every Design in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::gen::recipe::Design::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// Returns the classic designs for a dimension.
        #[pyfunction]
        #[pyo3(name = "classics", signature = (dimension))]
        pub fn classics<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::recipe::classics(dimension);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns every flat size in the range that passes the parity filter.
        #[pyfunction]
        #[pyo3(name = "generals", signature = (min_size, max_size, parity))]
        pub fn generals<'py>(py: Python<'py>, min_size: usize, max_size: usize, parity: PySerde<mrlyrs::gen::Parity>) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrlyrs::gen::recipe::generals(min_size, max_size, parity);
            (out).into_bound_py_any(py)
        }

        /// Returns every factor list of depth two and beyond whose product lands in the size range.
        #[pyfunction]
        #[pyo3(name = "nestings", signature = (min_size, max_size, parity))]
        pub fn nestings<'py>(py: Python<'py>, min_size: usize, max_size: usize, parity: PySerde<mrlyrs::gen::Parity>) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrlyrs::gen::recipe::nestings(min_size, max_size, parity);
            (out).into_bound_py_any(py)
        }

        /// Returns every factor and level whose power lands in the size range.
        #[pyfunction]
        #[pyo3(name = "powers", signature = (min_size, max_size, parity))]
        pub fn powers<'py>(py: Python<'py>, min_size: usize, max_size: usize, parity: PySerde<mrlyrs::gen::Parity>) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrlyrs::gen::recipe::powers(min_size, max_size, parity);
            (out).into_bound_py_any(py)
        }

        /// Returns every count-long factor list whose product lands in the size range.
        #[pyfunction]
        #[pyo3(name = "products", signature = (min_size, max_size, count, parity))]
        pub fn products<'py>(py: Python<'py>, min_size: usize, max_size: usize, count: usize, parity: PySerde<mrlyrs::gen::Parity>) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrlyrs::gen::recipe::products(min_size, max_size, count, parity);
            (out).into_bound_py_any(py)
        }

        /// Returns the side a factor raised to a level makes, or None when no usize holds it.
        #[pyfunction]
        #[pyo3(name = "size", signature = (number, level))]
        pub fn size<'py>(py: Python<'py>, number: i64, level: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::recipe::size(number, level);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.gen.recipe")?;
            m.setattr("__doc__", "The tile recipe: its families, groups, parities, catalog and size lists.")?;
            m.add_class::<Design>()?;
            m.add_function(wrap_pyfunction!(classics, &m)?)?;
            m.add_function(wrap_pyfunction!(generals, &m)?)?;
            m.add_function(wrap_pyfunction!(nestings, &m)?)?;
            m.add_function(wrap_pyfunction!(powers, &m)?)?;
            m.add_function(wrap_pyfunction!(products, &m)?)?;
            m.add_function(wrap_pyfunction!(size, &m)?)?;
            m.add("CLASSICS_2D", (mrlyrs::gen::recipe::CLASSICS_2D).into_iter().map(PySerde).collect::<Vec<_>>())?;
            m.add("CLASSICS_3D", (mrlyrs::gen::recipe::CLASSICS_3D).into_iter().map(PySerde).collect::<Vec<_>>())?;
            m.add("MAX_LEVEL", mrlyrs::gen::recipe::MAX_LEVEL)?;
            m.add("MAX_SIDE", mrlyrs::gen::recipe::MAX_SIDE)?;
            m.add("MAX_SLOTS", mrlyrs::gen::recipe::MAX_SLOTS)?;
            m.add("MIN_SIDE", mrlyrs::gen::recipe::MIN_SIDE)?;
            let names: Vec<&str> = vec!["classics", "generals", "nestings", "powers", "products", "size", "Design", "CLASSICS_2D", "CLASSICS_3D", "MAX_LEVEL", "MAX_SIDE", "MAX_SLOTS", "MIN_SIDE"];
            m.add("__all__", names)?;
            parent.add("recipe", &m)?;
            sys.set_item("mrlypy._mrlypy.gen.recipe", &m)?;
            Ok(())
        }
    }

    /// The seeded artwork run from tile recipe to rendered files.
    pub mod variation {
        use crate::hand::{ok, PyCellNd, PyRng, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// One rendering of an artwork, sized in tile repetitions.
        #[pyclass(name = "File", module = "mrlypy.gen.variation", from_py_object)]
        #[derive(Clone)]
        pub struct File(pub mrlyrs::gen::variation::File);

        #[pymethods]
        impl File {
            /// Builds a file of the given repetition counts with no PNG bytes.
            #[new]
            #[pyo3(signature = (width, height))]
            pub fn __new__(width: usize, height: usize) -> PyResult<Self> {
                let out = mrlyrs::gen::variation::File::new(width, height);
                Ok(Self(out))
            }
            /// The count of tile repetitions across.
            #[getter]
            #[pyo3(name = "width")]
            pub fn width<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.width;
                (value).into_bound_py_any(py)
            }
            /// The count of tile repetitions down.
            #[getter]
            #[pyo3(name = "height")]
            pub fn height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.height;
                (value).into_bound_py_any(py)
            }
            /// The encoded PNG bytes, empty until rendered and left out of the json.
            #[getter]
            #[pyo3(name = "png")]
            pub fn png<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.png.clone();
                (value).into_bound_py_any(py)
            }
            /// Builds a file of the given repetition counts with no PNG bytes.
            #[staticmethod]
            #[pyo3(name = "new", signature = (width, height))]
            pub fn new_<'py>(py: Python<'py>, width: usize, height: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::gen::variation::File::new(width, height);
                (crate::gen::gen_::variation::File(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// One seeded artwork, from tile recipe to rendered files.
        #[pyclass(name = "Variation", module = "mrlypy.gen.variation", from_py_object)]
        #[derive(Clone)]
        pub struct Variation(pub mrlyrs::gen::variation::Variation);

        #[pymethods]
        impl Variation {
            /// The random hex identifier.
            #[getter]
            #[pyo3(name = "key")]
            pub fn key<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.key.clone();
                (value).into_bound_py_any(py)
            }
            /// The seed the variation is drawn under.
            #[getter]
            #[pyo3(name = "seed")]
            pub fn seed<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.seed;
                (value).into_bound_py_any(py)
            }
            /// The paint edition.
            #[getter]
            #[pyo3(name = "edition")]
            pub fn edition<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.edition;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The primary inks, when the config fixes them.
            #[getter]
            #[pyo3(name = "primaries")]
            pub fn primaries<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.primaries.clone();
                ((value).map(|x| (x).into_iter().map(PySerde).collect::<Vec<_>>())).into_bound_py_any(py)
            }
            /// The tile recipe.
            #[getter]
            #[pyo3(name = "tile")]
            pub fn tile<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.tile.clone();
                (crate::gen::gen_::Tile(value)).into_bound_py_any(py)
            }
            /// The mask tile, present only under the Neighbors edition.
            #[getter]
            #[pyo3(name = "mask")]
            pub fn mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.mask.clone();
                ((value).map(crate::gen::gen_::Tile)).into_bound_py_any(py)
            }
            /// The paint, set by generate.
            #[getter]
            #[pyo3(name = "paint")]
            pub fn paint<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.paint.clone();
                ((value).map(crate::gen::core::paint::Paint)).into_bound_py_any(py)
            }
            /// The built base cell, set by generate and left out of the json.
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base.clone();
                ((value).map(PyCellNd)).into_bound_py_any(py)
            }
            /// The renderings, filled by render.
            #[getter]
            #[pyo3(name = "files")]
            pub fn files<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.files.clone();
                ((value).into_iter().map(crate::gen::gen_::variation::File).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns whether the edition paints the whole tiled canvas.
            #[pyo3(name = "is_cover", signature = ())]
            pub fn is_cover<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::gen::variation::Variation::is_cover(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns whether the edition paints the base cell before tiling.
            #[pyo3(name = "is_prime", signature = ())]
            pub fn is_prime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::gen::variation::Variation::is_prime(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Draws a variation's seed from the stream, then the variation itself on that seed, with a
        /// mask when the edition is Neighbors.
        #[pyfunction]
        #[pyo3(name = "create", signature = (config, rng))]
        pub fn create<'py>(py: Python<'py>, config: PySerde<mrlyrs::gen::variation::Config>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let config = config.0;
            let out = mrlyrs::gen::variation::create(&config, &mut rng.0);
            (crate::gen::gen_::variation::Variation(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the variation's base cell and draws its paint from the stream, painting the base
        /// under a prime edition.
        #[pyfunction]
        #[pyo3(name = "generate", signature = (variation, config, rng))]
        pub fn generate<'py>(py: Python<'py>, variation: crate::gen::gen_::variation::Variation, config: PySerde<mrlyrs::gen::variation::Config>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let variation = variation.0;
            let config = config.0;
            let out = mrlyrs::gen::variation::generate(variation, &config, &mut rng.0);
            (crate::gen::gen_::variation::Variation(ok(out)?)).into_bound_py_any(py)
        }

        /// Renders every file of the variation to PNG at the given scale, scattering a Random edition
        /// from the stream.
        #[pyfunction]
        #[pyo3(name = "render", signature = (variation, scale, rng))]
        pub fn render<'py>(py: Python<'py>, variation: crate::gen::gen_::variation::Variation, scale: usize, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let variation = variation.0;
            let out = mrlyrs::gen::variation::render(variation, scale, &mut rng.0);
            (crate::gen::gen_::variation::Variation(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.gen.variation")?;
            m.setattr("__doc__", "The seeded artwork run from tile recipe to rendered files.")?;
            m.add_class::<File>()?;
            m.add_class::<Variation>()?;
            m.add_function(wrap_pyfunction!(create, &m)?)?;
            m.add_function(wrap_pyfunction!(generate, &m)?)?;
            m.add_function(wrap_pyfunction!(render, &m)?)?;
            let names: Vec<&str> = vec!["create", "generate", "render", "File", "Variation"];
            m.add("__all__", names)?;
            parent.add("variation", &m)?;
            sys.set_item("mrlypy._mrlypy.gen.variation", &m)?;
            Ok(())
        }
    }

    /// A complete recipe for one tile.
    #[pyclass(name = "Tile", module = "mrlypy.gen", from_py_object)]
    #[derive(Clone)]
    pub struct Tile(pub mrlyrs::gen::Tile);

    #[pymethods]
    impl Tile {
        /// Builds an empty tile in a group.
        #[new]
        #[pyo3(signature = (group))]
        pub fn __new__(group: PySerde<mrlyrs::gen::Group>) -> PyResult<Self> {
            let group = group.0;
            let out = mrlyrs::gen::Tile::new(group);
            Ok(Self(out))
        }
        /// The construction family.
        #[getter]
        #[pyo3(name = "group")]
        pub fn group<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.group;
            (PySerde(value)).into_bound_py_any(py)
        }
        /// The base factor of the construction.
        #[getter]
        #[pyo3(name = "factor")]
        pub fn factor<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.factor;
            (value).into_bound_py_any(py)
        }
        /// The origin of each layer.
        #[getter]
        #[pyo3(name = "sources")]
        pub fn sources<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.sources.clone();
            ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// The grid size of each source.
        #[getter]
        #[pyo3(name = "numbers")]
        pub fn numbers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.numbers.clone();
            (value).into_bound_py_any(py)
        }
        /// The fractal level of each source.
        #[getter]
        #[pyo3(name = "levels")]
        pub fn levels<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.levels.clone();
            (value).into_bound_py_any(py)
        }
        /// The quarter-turn rotation of each source.
        #[getter]
        #[pyo3(name = "rotations")]
        pub fn rotations<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.rotations.clone();
            (value).into_bound_py_any(py)
        }
        /// Whether the finished tile inverts.
        #[getter]
        #[pyo3(name = "invert")]
        pub fn invert<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.invert;
            (value).into_bound_py_any(py)
        }
        /// Whether the finished tile flips.
        #[getter]
        #[pyo3(name = "flip")]
        pub fn flip<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.flip;
            (value).into_bound_py_any(py)
        }
        /// The tile's width in cells.
        #[getter]
        #[pyo3(name = "width")]
        pub fn width<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.width;
            (value).into_bound_py_any(py)
        }
        /// The tile's height in cells.
        #[getter]
        #[pyo3(name = "height")]
        pub fn height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.height;
            (value).into_bound_py_any(py)
        }
        /// Checks that the slots, numbers and sizes agree.
        #[pyo3(name = "check", signature = ())]
        pub fn check<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::Tile::check(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Returns whether the recipe is a magic tile of one repeated source at one repeated number,
        /// the shape a fractal tile of the same factor and level already draws.
        #[pyo3(name = "degenerate", signature = ())]
        pub fn degenerate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::Tile::degenerate(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Returns the larger of width and height.
        #[pyo3(name = "max_size", signature = ())]
        pub fn max_size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::Tile::max_size(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Builds an empty tile in a group.
        #[staticmethod]
        #[pyo3(name = "new", signature = (group))]
        pub fn new_<'py>(py: Python<'py>, group: PySerde<mrlyrs::gen::Group>) -> PyResult<Bound<'py, PyAny>> {
            let group = group.0;
            let out = mrlyrs::gen::Tile::new(group);
            (crate::gen::gen_::Tile(out)).into_bound_py_any(py)
        }
        /// Recomputes the factor and side length the group and numbers imply, zero when they overflow.
        #[pyo3(name = "resize", signature = ())]
        pub fn resize<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            mrlyrs::gen::Tile::resize(&mut self.0);
            ().into_bound_py_any(py)
        }
        /// Sets the tile's width and height.
        #[pyo3(name = "size", signature = (width, height))]
        pub fn size<'py>(&self, py: Python<'py>, width: usize, height: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::Tile::size(self.0.clone(), width, height);
            (crate::gen::gen_::Tile(out)).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// The five construction families a tile can belong to.
    #[pyclass(name = "Group", module = "mrlypy.gen", skip_from_py_object)]
    pub struct Group;

    #[pymethods]
    impl Group {
        /// Returns every Group in canonical order.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::Group::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
    }

    /// The parity filter over candidate sizes.
    #[pyclass(name = "Parity", module = "mrlypy.gen", skip_from_py_object)]
    pub struct Parity;

    #[pymethods]
    impl Parity {
        /// Returns every Parity in canonical order.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::gen::Parity::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// Returns true when the number passes the filter.
        #[staticmethod]
        #[pyo3(name = "keep", signature = (parity, n))]
        pub fn keep<'py>(py: Python<'py>, parity: PySerde<mrlyrs::gen::Parity>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrlyrs::gen::Parity::keep(parity, n);
            (out).into_bound_py_any(py)
        }
    }

    /// Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe
    /// constraints and paint, repeated `width` across and `height` down at one pixel per cell.
    #[pyfunction]
    #[pyo3(name = "background", signature = (seed, width, height))]
    pub fn background<'py>(py: Python<'py>, seed: u64, width: usize, height: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::gen::background(seed, width, height);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Returns the plane's bang code of a classic design, or None for one outside the plane.
    #[pyfunction]
    #[pyo3(name = "classic_code", signature = (design))]
    pub fn classic_code<'py>(py: Python<'py>, design: PySerde<mrlyrs::gen::recipe::Design>) -> PyResult<Bound<'py, PyAny>> {
        let design = design.0;
        let out = mrlyrs::gen::classic_code(design);
        (out).into_bound_py_any(py)
    }

    /// Returns the bang code of a named design in a dimension, or None where it has no design.
    #[pyfunction]
    #[pyo3(name = "classic_code_nd", signature = (design, dimension))]
    pub fn classic_code_nd<'py>(py: Python<'py>, design: PySerde<mrlyrs::gen::recipe::Design>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
        let design = design.0;
        let out = mrlyrs::gen::classic_code_nd(design, dimension);
        (out).into_bound_py_any(py)
    }

    /// Draws a hex key of the given length from the stream.
    #[pyfunction]
    #[pyo3(name = "hex_key", signature = (length, rng))]
    pub fn hex_key<'py>(py: Python<'py>, length: usize, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::gen::hex_key(length, &mut rng.0);
        (out).into_bound_py_any(py)
    }

    /// Draws one named design from the stream.
    #[pyfunction]
    #[pyo3(name = "random_design", signature = (rng))]
    pub fn random_design<'py>(py: Python<'py>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::gen::random_design(&mut rng.0);
        (PySerde(out)).into_bound_py_any(py)
    }

    /// Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3.
    #[pyfunction]
    #[pyo3(name = "random_rotation", signature = (design, rng))]
    pub fn random_rotation<'py>(py: Python<'py>, design: PySerde<mrlyrs::gen::recipe::Design>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
        let design = design.0;
        let out = mrlyrs::gen::random_rotation(design, &mut rng.0);
        (out).into_bound_py_any(py)
    }

    /// Builds the mask of a mosaic tile: the two trees of the side, two where they cross.
    #[pyfunction]
    #[pyo3(name = "tree_mask", signature = (n))]
    pub fn tree_mask<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::gen::tree_mask(n);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
        let m = PyModule::new(py, "mrlypy.gen")?;
        m.setattr("__doc__", "The generator: the whole pipeline from a recipe to a file.\nThe generator: the pipeline from a recipe to a file, for datasets, the automator, backgrounds.\n\n- `recipe`: the tile recipe, its families, groups, parities, catalog and size lists.\n- `draw`: one recipe drawn at random from a seeded stream under size constraints.\n- `build`: the recipe to cell builders, one per dimension.\n- `variation`: the seeded artwork, from recipe through paint to rendered files.\n- `name`: the canonical name of a recipe, and the recipe back out of it.\n\nThe doors: [`Tile::new`](crate::gen::recipe::Tile::new),\n[`Tile::size`](crate::gen::recipe::Tile::size),\n[`Tile::check`](crate::gen::recipe::Tile::check),\n[`draw::create`](crate::gen::draw::create), [`random_design`](crate::gen::random_design),\n[`random_rotation`](crate::gen::random_rotation), [`build_2d`](crate::gen::build::build_2d),\n[`tree_mask`](crate::gen::tree_mask), [`variation::create`](crate::gen::variation::create),\n[`hex_key`](crate::gen::hex_key), [`classic_code_nd`](crate::gen::classic_code_nd),\n[`Tile::of`](crate::gen::name::Tile::of) and [`background`](crate::gen::background).")?;
        m.add_class::<Tile>()?;
        m.add_class::<Group>()?;
        m.add_class::<Parity>()?;
        m.add_function(wrap_pyfunction!(background, &m)?)?;
        m.add_function(wrap_pyfunction!(classic_code, &m)?)?;
        m.add_function(wrap_pyfunction!(classic_code_nd, &m)?)?;
        m.add_function(wrap_pyfunction!(hex_key, &m)?)?;
        m.add_function(wrap_pyfunction!(random_design, &m)?)?;
        m.add_function(wrap_pyfunction!(random_rotation, &m)?)?;
        m.add_function(wrap_pyfunction!(tree_mask, &m)?)?;
        let names: Vec<&str> = vec!["background", "classic_code", "classic_code_nd", "hex_key", "random_design", "random_rotation", "tree_mask", "Tile", "Group", "Parity"];
        m.add("__all__", names)?;
        build::init(py, &m, sys)?;
        name::init(py, &m, sys)?;
        recipe::init(py, &m, sys)?;
        variation::init(py, &m, sys)?;
        parent.add("gen", &m)?;
        sys.set_item("mrlypy._mrlypy.gen", &m)?;
        Ok(())
    }
}

/// The engine: a rule over a grid, stepped, recorded, measured and rendered.
/// The engine: a rule over a grid, stepped, recorded, measured and rendered.
pub mod life {
    use crate::hand::{ok, PyCell2d, PyCellNd, PyCode, PySerde, PyTensor};
    use pyo3::prelude::*;
    use pyo3::types::PyDict;
    use pyo3::IntoPyObjectExt;

    /// The elementary automata: their stepping, their space-time diagrams and the card of one rule.
    pub mod elementary {
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell.
        #[pyfunction]
        #[pyo3(name = "output", signature = (rule, l, c, r))]
        pub fn output<'py>(py: Python<'py>, rule: u8, l: u8, c: u8, r: u8) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::elementary::output(rule, l, c, r);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.life.elementary")?;
            m.setattr("__doc__", "The elementary automata: their stepping, their space-time diagrams and the card of one rule.")?;
            m.add_function(wrap_pyfunction!(output, &m)?)?;
            let names: Vec<&str> = vec!["output"];
            m.add("__all__", names)?;
            parent.add("elementary", &m)?;
            sys.set_item("mrlypy._mrlypy.life.elementary", &m)?;
            Ok(())
        }
    }

    /// The PNG frames, the cumulative-visit heatmap and the gif movie of grids.
    pub mod render {
        use crate::hand::{ok, PyCell2d};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Renders one grid to white-on-black PNG bytes at a pixel scale.
        #[pyfunction]
        #[pyo3(name = "frame", signature = (grid, scale))]
        pub fn frame<'py>(py: Python<'py>, grid: PyCell2d, scale: usize) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrlyrs::life::render::frame(&grid, scale);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.life.render")?;
            m.setattr("__doc__", "The PNG frames, the cumulative-visit heatmap and the gif movie of grids.")?;
            m.add_function(wrap_pyfunction!(frame, &m)?)?;
            let names: Vec<&str> = vec!["frame"];
            m.add("__all__", names)?;
            parent.add("render", &m)?;
            sys.set_item("mrlypy._mrlypy.life.render", &m)?;
            Ok(())
        }
    }

    /// The named sources of neighbor-count values, and the counts they lay down.
    pub mod source {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Generates the sequence's values up to the limit.
        #[pyfunction]
        #[pyo3(name = "sequence", signature = (seq, limit))]
        pub fn sequence<'py>(py: Python<'py>, seq: crate::gen::life::Source, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let seq = seq.0;
            let out = mrlyrs::life::source::sequence(seq, limit);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.life.source")?;
            m.setattr("__doc__", "The named sources of neighbor-count values, and the counts they lay down.")?;
            m.add_function(wrap_pyfunction!(sequence, &m)?)?;
            let names: Vec<&str> = vec!["sequence"];
            m.add("__all__", names)?;
            parent.add("source", &m)?;
            sys.set_item("mrlypy._mrlypy.life.source", &m)?;
            Ok(())
        }
    }

    /// The rulebook of a life run.
    #[pyclass(name = "Config", module = "mrlypy.life", from_py_object)]
    #[derive(Clone)]
    pub struct Config(pub mrlyrs::life::Config);

    #[pymethods]
    impl Config {
        /// Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.
        #[new]
        #[pyo3(signature = (mask, birth, survive))]
        pub fn __new__(mask: PyCell2d, birth: crate::gen::life::Counts, survive: crate::gen::life::Counts) -> PyResult<Self> {
            let mask = mask.0;
            let birth = birth.0;
            let survive = survive.0;
            let out = mrlyrs::life::Config::new(mask, birth, survive);
            Ok(Self(out))
        }
        /// The neighborhood mask.
        #[getter]
        #[pyo3(name = "mask")]
        pub fn mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.mask.clone();
            (PyCellNd(value)).into_bound_py_any(py)
        }
        /// The neighbor counts that create a cell.
        #[getter]
        #[pyo3(name = "birth")]
        pub fn birth<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.birth.clone();
            (crate::gen::life::Counts(value)).into_bound_py_any(py)
        }
        /// The neighbor counts that keep a cell.
        #[getter]
        #[pyo3(name = "survive")]
        pub fn survive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.survive.clone();
            (crate::gen::life::Counts(value)).into_bound_py_any(py)
        }
        /// The edge policy.
        #[getter]
        #[pyo3(name = "boundary")]
        pub fn boundary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.boundary;
            (PySerde(value)).into_bound_py_any(py)
        }
        /// The generation cap.
        #[getter]
        #[pyo3(name = "max_generations")]
        pub fn max_generations<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.max_generations;
            (value).into_bound_py_any(py)
        }
        /// The tiling factor applied to the seed.
        #[getter]
        #[pyo3(name = "grid_size")]
        pub fn grid_size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.grid_size;
            (value).into_bound_py_any(py)
        }
        /// The dead border added around the seed.
        #[getter]
        #[pyo3(name = "padding")]
        pub fn padding<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.padding;
            (value).into_bound_py_any(py)
        }
        /// Returns the largest neighbor count the mask can reach.
        #[pyo3(name = "budget", signature = ())]
        pub fn budget<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Config::budget(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Resolves the birth and survive counts against the mask's budget.
        #[pyo3(name = "counts", signature = ())]
        pub fn counts<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Config::counts(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.
        #[staticmethod]
        #[pyo3(name = "new", signature = (mask, birth, survive))]
        pub fn new_<'py>(py: Python<'py>, mask: PyCell2d, birth: crate::gen::life::Counts, survive: crate::gen::life::Counts) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let birth = birth.0;
            let survive = survive.0;
            let out = mrlyrs::life::Config::new(mask, birth, survive);
            (crate::gen::life::Config(out)).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// The neighbor counts one side of a rule fires on.
    #[pyclass(name = "Counts", module = "mrlypy.life", from_py_object)]
    #[derive(Clone)]
    pub struct Counts(pub mrlyrs::life::Counts);

    #[pymethods]
    impl Counts {
        /// Builds the counts a sequence lays down, keeping zeros and ones on request.
        #[staticmethod]
        #[pyo3(name = "drawn", signature = (seq, zeros, ones))]
        pub fn drawn<'py>(py: Python<'py>, seq: crate::gen::life::Source, zeros: bool, ones: bool) -> PyResult<Bound<'py, PyAny>> {
            let seq = seq.0;
            let out = mrlyrs::life::Counts::drawn(seq, zeros, ones);
            (crate::gen::life::Counts(out)).into_bound_py_any(py)
        }
        /// Spells the counts outright.
        #[staticmethod]
        #[pyo3(name = "list", signature = (counts))]
        pub fn list<'py>(py: Python<'py>, counts: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Counts::list(counts);
            (crate::gen::life::Counts(out)).into_bound_py_any(py)
        }
        /// Returns the counts, a drawn side resolved against the mask's neighbor budget.
        #[pyo3(name = "values", signature = (budget))]
        pub fn values<'py>(&self, py: Python<'py>, budget: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Counts::values(&self.0, budget);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// The recorded run of one seed.
    #[pyclass(name = "Life", module = "mrlypy.life", from_py_object)]
    #[derive(Clone)]
    pub struct Life(pub mrlyrs::life::Life);

    #[pymethods]
    impl Life {
        /// Every generation in order.
        #[getter]
        #[pyo3(name = "grids")]
        pub fn grids<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.grids.clone();
            ((value).into_iter().map(PyCellNd).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// The run's ending.
        #[getter]
        #[pyo3(name = "fate")]
        pub fn fate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.fate;
            (PySerde(value)).into_bound_py_any(py)
        }
        /// The number of recorded generations.
        #[getter]
        #[pyo3(name = "count")]
        pub fn count<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.count;
            (value).into_bound_py_any(py)
        }
        /// The cycle length when the fate is a loop, else zero.
        #[getter]
        #[pyo3(name = "loop_length")]
        pub fn loop_length<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.loop_length;
            (value).into_bound_py_any(py)
        }
        /// Returns the final grid, or None when the run is empty.
        #[pyo3(name = "last", signature = ())]
        pub fn last<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Life::last(&self.0);
            ((out).map(|x| PyCellNd((x).clone()))).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// A life rule: the birth and survival counts and whether the edge wraps.
    #[pyclass(name = "Rule", module = "mrlypy.life", from_py_object)]
    #[derive(Clone)]
    pub struct Rule(pub mrlyrs::life::Rule);

    #[pymethods]
    impl Rule {
        /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
        #[new]
        #[pyo3(signature = (birth, survive, wrap))]
        pub fn __new__(birth: crate::gen::life::Counts, survive: crate::gen::life::Counts, wrap: bool) -> PyResult<Self> {
            let birth = birth.0;
            let survive = survive.0;
            let out = mrlyrs::life::Rule::new(birth, survive, wrap);
            Ok(Self(out))
        }
        /// The neighbor counts that create a cell, listed or drawn from a sequence.
        #[getter]
        #[pyo3(name = "birth")]
        pub fn birth<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.birth.clone();
            (crate::gen::life::Counts(value)).into_bound_py_any(py)
        }
        /// The neighbor counts that keep a cell, listed or drawn from a sequence.
        #[getter]
        #[pyo3(name = "survive")]
        pub fn survive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.survive.clone();
            (crate::gen::life::Counts(value)).into_bound_py_any(py)
        }
        /// Whether the edge wraps, false unless said.
        #[getter]
        #[pyo3(name = "wrap")]
        pub fn wrap<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.wrap;
            (value).into_bound_py_any(py)
        }
        /// Returns the edge policy the rule runs under.
        #[pyo3(name = "boundary", signature = ())]
        pub fn boundary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Rule::boundary(&self.0);
            (PySerde(out)).into_bound_py_any(py)
        }
        /// Folds a decoded value to its canonical form, or an error for one outside the kind.
        #[pyo3(name = "checked", signature = ())]
        pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::checked(self.0.clone());
            (crate::gen::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Builds a life config running this rule over a neighborhood mask.
        #[pyo3(name = "config", signature = (mask))]
        pub fn config<'py>(&self, py: Python<'py>, mask: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let out = mrlyrs::life::Rule::config(&self.0, mask);
            (crate::gen::life::Config(out)).into_bound_py_any(py)
        }
        /// Reads a filename back into the value, or an error.
        #[staticmethod]
        #[pyo3(name = "from_file", signature = (text))]
        pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_file(text);
            (crate::gen::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Reads a JSON object into its canonical value, or an error naming the broken key.
        #[staticmethod]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_json(text);
            (crate::gen::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Reads a path and query string back into the value, or an error.
        #[staticmethod]
        #[pyo3(name = "from_url", signature = (text))]
        pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_url(text);
            (crate::gen::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
        #[staticmethod]
        #[pyo3(name = "new", signature = (birth, survive, wrap))]
        pub fn new_<'py>(py: Python<'py>, birth: crate::gen::life::Counts, survive: crate::gen::life::Counts, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
            let birth = birth.0;
            let survive = survive.0;
            let out = mrlyrs::life::Rule::new(birth, survive, wrap);
            (crate::gen::life::Rule(out)).into_bound_py_any(py)
        }
        /// Reads the rule out of a life config.
        #[staticmethod]
        #[pyo3(name = "of", signature = (config))]
        pub fn of<'py>(py: Python<'py>, config: PyRef<'_, crate::gen::life::Config>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Rule::of(&config.0);
            (crate::gen::life::Rule(out)).into_bound_py_any(py)
        }
        /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
        #[pyo3(name = "to_file", signature = ())]
        pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_file(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Prints the first eight hex digits of the sha256 of the canonical JSON.
        #[pyo3(name = "to_id", signature = ())]
        pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_id(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Prints the canonical JSON object.
        #[pyo3(name = "to_json", signature = ())]
        pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_json(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
        #[pyo3(name = "to_mrly", signature = ())]
        pub fn to_mrly<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_mrly(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
        #[pyo3(name = "to_url", signature = ())]
        pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_url(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// A named source of neighbor-count values.
    #[pyclass(name = "Source", module = "mrlypy.life", from_py_object)]
    #[derive(Clone)]
    pub struct Source(pub mrlyrs::life::Source);

    #[pymethods]
    impl Source {
        /// Returns every fixed sequence, the seeded and coded families excluded.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::all();
            ((out).into_iter().map(crate::gen::life::Source).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// Returns the seventeen mrly design families: the grid, the four classics and their antis.
        #[staticmethod]
        #[pyo3(name = "designs", signature = ())]
        pub fn designs<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::designs();
            ((out).into_iter().map(crate::gen::life::Source).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// Returns whether the sequence is a seeded random draw.
        #[pyo3(name = "is_random", signature = ())]
        pub fn is_random<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::is_random(self.0);
            (out).into_bound_py_any(py)
        }
        /// Returns the sequence's parseable name, the one string that regenerates it.
        #[pyo3(name = "name", signature = ())]
        pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::name(self.0);
            (out).into_bound_py_any(py)
        }
        /// Returns the six number sequences, the random one listed under seed zero.
        #[staticmethod]
        #[pyo3(name = "numbers", signature = ())]
        pub fn numbers<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::numbers();
            ((out).into_iter().map(crate::gen::life::Source).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// Returns the sequence's OEIS id, or None off the encyclopedia.
        #[pyo3(name = "oeis", signature = ())]
        pub fn oeis<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::oeis(self.0);
            (out).into_bound_py_any(py)
        }
        /// Parses a sequence name back to its source.
        #[staticmethod]
        #[pyo3(name = "parse", signature = (name))]
        pub fn parse<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::parse(name);
            (crate::gen::life::Source(ok(out)?)).into_bound_py_any(py)
        }
        /// Reads a canonical name off the front of the text, returning the tail left over.
        #[staticmethod]
        #[pyo3(name = "read", signature = (text))]
        pub fn read<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Source::read(text);
            ((out).map(|x| { let t = x; (crate::gen::life::Source(t.0), (t.1).to_string()) })).into_bound_py_any(py)
        }
        /// Reads plain data into the class.
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        /// Returns the value as plain data.
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    /// The edge policy of a life grid.
    #[pyclass(name = "Boundary", module = "mrlypy.life", skip_from_py_object)]
    pub struct Boundary;

    #[pymethods]
    impl Boundary {
        /// Returns every Boundary in canonical order.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Boundary::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// Returns whether the edges wrap.
        #[staticmethod]
        #[pyo3(name = "wrap", signature = (boundary))]
        pub fn wrap<'py>(py: Python<'py>, boundary: PySerde<mrlyrs::life::Boundary>) -> PyResult<Bound<'py, PyAny>> {
            let boundary = boundary.0;
            let out = mrlyrs::life::Boundary::wrap(boundary);
            (out).into_bound_py_any(py)
        }
    }

    /// The ending of a life run.
    #[pyclass(name = "Fate", module = "mrlypy.life", skip_from_py_object)]
    pub struct Fate;

    #[pymethods]
    impl Fate {
        /// Returns every Fate in canonical order.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::life::Fate::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
    }

    /// Returns whether a rule is affine, its algebraic degree at most one.
    #[pyfunction]
    #[pyo3(name = "affine", signature = (rule))]
    pub fn affine<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::affine(rule);
        (out).into_bound_py_any(py)
    }

    /// Runs a seed under a config until it fixes, loops or times out, recording every generation.
    #[pyfunction]
    #[pyo3(name = "animate", signature = (seed, config))]
    pub fn animate<'py>(py: Python<'py>, seed: PyCell2d, config: PyRef<'_, crate::gen::life::Config>) -> PyResult<Bound<'py, PyAny>> {
        let seed = seed.0;
        let out = mrlyrs::life::animate(&seed, &config.0);
        (crate::gen::life::Life(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns the mean fraction of sites changed between consecutive grids.
    #[pyfunction]
    #[pyo3(name = "churn", signature = (grids))]
    pub fn churn<'py>(py: Python<'py>, grids: Vec<PyCell2d>) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrlyrs::life::churn(&grids);
        (out).into_bound_py_any(py)
    }

    /// Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`.
    #[pyfunction]
    #[pyo3(name = "corner_bits", signature = (rule))]
    pub fn corner_bits<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::corner_bits(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the sequence up to max_neighbors, keeping zeros and ones only on request.
    #[pyfunction]
    #[pyo3(name = "counts", signature = (seq, max_neighbors, include_zeros, include_ones))]
    pub fn counts<'py>(py: Python<'py>, seq: crate::gen::life::Source, max_neighbors: usize, include_zeros: bool, include_ones: bool) -> PyResult<Bound<'py, PyAny>> {
        let seq = seq.0;
        let out = mrlyrs::life::counts(seq, max_neighbors, include_zeros, include_ones);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Crops a frame sequence to the centred square bounding every cell ever alive.
    #[pyfunction]
    #[pyo3(name = "crop", signature = (grids))]
    pub fn crop<'py>(py: Python<'py>, grids: Vec<PyCell2d>) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrlyrs::life::crop(&grids);
        ((ok(out)?).into_iter().map(PyCellNd).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order.
    #[pyfunction]
    #[pyo3(name = "cube_orbit", signature = (rule))]
    pub fn cube_orbit<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::cube_orbit(rule);
        (out).into_bound_py_any(py)
    }

    /// Builds the base-2 design mask a code names at an odd side grown to the given Kronecker
    /// level, its centre popped.
    #[pyfunction]
    #[pyo3(name = "design_mask", signature = (dimension, code, number, level))]
    pub fn design_mask<'py>(py: Python<'py>, dimension: usize, code: PyCode, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
        let code = code.0;
        let out = mrlyrs::life::design_mask(dimension, code, number, level);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns the grid's binary Shannon entropy in millibits.
    #[pyfunction]
    #[pyo3(name = "entropy", signature = (grid))]
    pub fn entropy<'py>(py: Python<'py>, grid: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
        let grid = grid.0;
        let out = mrlyrs::life::entropy(&grid);
        (out).into_bound_py_any(py)
    }

    /// Renders grids to white-on-black PNG bytes at a pixel scale.
    #[pyfunction]
    #[pyo3(name = "frames", signature = (grids, scale))]
    pub fn frames<'py>(py: Python<'py>, grids: Vec<PyCell2d>, scale: usize) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrlyrs::life::frames(&grids, scale);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Returns the base-2 plane design a rule's single seed draws, or None when it draws none.
    #[pyfunction]
    #[pyo3(name = "gasket", signature = (rule))]
    pub fn gasket<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::gasket(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`.
    #[pyfunction]
    #[pyo3(name = "genus", signature = (rule))]
    pub fn genus<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::genus(rule);
        (out).into_bound_py_any(py)
    }

    /// Renders a whole run's cumulative-visit heatmap frames with the heat ramp.
    #[pyfunction]
    #[pyo3(name = "heatmap", signature = (grids, scale))]
    pub fn heatmap<'py>(py: Python<'py>, grids: Vec<PyCell2d>, scale: usize) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrlyrs::life::heatmap(&grids, scale);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation.
    #[pyfunction]
    #[pyo3(name = "history", signature = (row, rule, steps, wrap))]
    pub fn history<'py>(py: Python<'py>, row: Vec<u8>, rule: u8, steps: usize, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::history(&row, rule, steps, wrap);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns Langton's lambda, the popcount over eight.
    #[pyfunction]
    #[pyo3(name = "lambda_", signature = (rule))]
    pub fn lambda_<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::lambda(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension.
    #[pyfunction]
    #[pyo3(name = "lattice_index", signature = (mask))]
    pub fn lattice_index<'py>(py: Python<'py>, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
        let mask = mask.0;
        let out = mrlyrs::life::lattice_index(&mask);
        (out).into_bound_py_any(py)
    }

    /// Returns the offsets a mask's filled sites take from its centre, the centre itself dropped.
    #[pyfunction]
    #[pyo3(name = "mask_offsets", signature = (mask))]
    pub fn mask_offsets<'py>(py: Python<'py>, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
        let mask = mask.0;
        let out = mrlyrs::life::mask_offsets(&mask);
        (out).into_bound_py_any(py)
    }

    /// Builds the 3 by 3 Moore mask, every site on but the center.
    #[pyfunction]
    #[pyo3(name = "moore", signature = ())]
    pub fn moore<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::moore();
        (PyCellNd(ok(out)?)).into_bound_py_any(py)
    }

    /// Renders grids into one looping black-on-white gif, the delay in hundredths of a second.
    #[pyfunction]
    #[pyo3(name = "movie", signature = (grids, scale, delay))]
    pub fn movie<'py>(py: Python<'py>, grids: Vec<PyCell2d>, scale: usize, delay: usize) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrlyrs::life::movie(&grids, scale, delay);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary.
    #[pyfunction]
    #[pyo3(name = "next_grid", signature = (cell, birth, survive, mask, boundary))]
    pub fn next_grid<'py>(py: Python<'py>, cell: PyCell2d, birth: Vec<usize>, survive: Vec<usize>, mask: PyTensor, boundary: PySerde<mrlyrs::life::Boundary>) -> PyResult<Bound<'py, PyAny>> {
        let cell = cell.0;
        let mask = mask.0;
        let boundary = boundary.0;
        let out = mrlyrs::life::next_grid(&cell, &birth, &survive, &mask, boundary);
        (PyCellNd(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order.
    #[pyfunction]
    #[pyo3(name = "npn_class", signature = (rule))]
    pub fn npn_class<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::npn_class(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone.
    #[pyfunction]
    #[pyo3(name = "outer_totalistic", signature = (rule))]
    pub fn outer_totalistic<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::outer_totalistic(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the count of neighbourhoods a rule sends to one.
    #[pyfunction]
    #[pyo3(name = "popcount", signature = (rule))]
    pub fn popcount<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::popcount(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core.
    #[pyfunction]
    #[pyo3(name = "reversible", signature = (rule))]
    pub fn reversible<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::reversible(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the GF(2) algebraic degree of a rule, minus one for the zero rule.
    #[pyfunction]
    #[pyo3(name = "rule_degree", signature = (rule))]
    pub fn rule_degree<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::rule_degree(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the design name a rule carries, `bang dim 3, code <rule>`.
    #[pyfunction]
    #[pyo3(name = "rule_name", signature = (rule))]
    pub fn rule_name<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::rule_name(rule);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window.
    #[pyfunction]
    #[pyo3(name = "single_seed", signature = (rule, steps))]
    pub fn single_seed<'py>(py: Python<'py>, rule: u8, steps: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::single_seed(rule, steps);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    /// Advances one row one generation, a constant-0 boundary unless the edges wrap.
    #[pyfunction]
    #[pyo3(name = "step", signature = (row, rule, wrap))]
    pub fn step<'py>(py: Python<'py>, row: Vec<u8>, rule: u8, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::step(&row, rule, wrap);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set.
    #[pyfunction]
    #[pyo3(name = "surjective", signature = (rule))]
    pub fn surjective<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::surjective(rule);
        (out).into_bound_py_any(py)
    }

    /// Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there.
    #[pyfunction]
    #[pyo3(name = "tessellate", signature = (grids, min_canvas))]
    pub fn tessellate<'py>(py: Python<'py>, grids: Vec<PyCell2d>, min_canvas: usize) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrlyrs::life::tessellate(&grids, min_canvas);
        ((ok(out)?).into_iter().map(PyCellNd).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order.
    #[pyfunction]
    #[pyo3(name = "wolfram_class", signature = (rule))]
    pub fn wolfram_class<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrlyrs::life::wolfram_class(rule);
        (out).into_bound_py_any(py)
    }

    pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
        let m = PyModule::new(py, "mrlypy.life")?;
        m.setattr("__doc__", "The engine: a rule over a grid, stepped, recorded, measured and rendered.\nThe engine: a rule over a grid, stepped, recorded, measured and rendered.\n\n- `step`: one generation of a grid under birth and survive counts.\n- `animate`: a seed run until it fixes, loops or times out.\n- `models`: the run config and the recorded life.\n- `metrics`: the entropy and churn readings of a run.\n- `crop`: the centred cropping and tiling of a run's frames.\n- `mask`: the design masks a rule reads and the lattice they generate.\n- `source`: the named sources of neighbor counts, and the counts they lay down.\n- `elementary`: the one-line automata and the card of one rule.\n- `render`: the PNG frames, the visit heatmap and the gif movie.\n- `rule`: the canonical name of a rule.\n\nThe doors: [`next_grid`](crate::life::next_grid), [`animate`](crate::life::animate()),\n[`entropy`](crate::life::entropy), [`churn`](crate::life::churn),\n[`counts`](crate::life::counts), [`frames`](crate::life::frames),\n[`heatmap`](crate::life::heatmap), [`history`](crate::life::history) and\n[`Rule`](crate::life::Rule).")?;
        m.add_class::<Config>()?;
        m.add_class::<Counts>()?;
        m.add_class::<Life>()?;
        m.add_class::<Rule>()?;
        m.add_class::<Source>()?;
        m.add_class::<Boundary>()?;
        m.add_class::<Fate>()?;
        m.add_function(wrap_pyfunction!(affine, &m)?)?;
        m.add_function(wrap_pyfunction!(animate, &m)?)?;
        m.add_function(wrap_pyfunction!(churn, &m)?)?;
        m.add_function(wrap_pyfunction!(corner_bits, &m)?)?;
        m.add_function(wrap_pyfunction!(counts, &m)?)?;
        m.add_function(wrap_pyfunction!(crop, &m)?)?;
        m.add_function(wrap_pyfunction!(cube_orbit, &m)?)?;
        m.add_function(wrap_pyfunction!(design_mask, &m)?)?;
        m.add_function(wrap_pyfunction!(entropy, &m)?)?;
        m.add_function(wrap_pyfunction!(frames, &m)?)?;
        m.add_function(wrap_pyfunction!(gasket, &m)?)?;
        m.add_function(wrap_pyfunction!(genus, &m)?)?;
        m.add_function(wrap_pyfunction!(heatmap, &m)?)?;
        m.add_function(wrap_pyfunction!(history, &m)?)?;
        m.add_function(wrap_pyfunction!(lambda_, &m)?)?;
        m.add_function(wrap_pyfunction!(lattice_index, &m)?)?;
        m.add_function(wrap_pyfunction!(mask_offsets, &m)?)?;
        m.add_function(wrap_pyfunction!(moore, &m)?)?;
        m.add_function(wrap_pyfunction!(movie, &m)?)?;
        m.add_function(wrap_pyfunction!(next_grid, &m)?)?;
        m.add_function(wrap_pyfunction!(npn_class, &m)?)?;
        m.add_function(wrap_pyfunction!(outer_totalistic, &m)?)?;
        m.add_function(wrap_pyfunction!(popcount, &m)?)?;
        m.add_function(wrap_pyfunction!(reversible, &m)?)?;
        m.add_function(wrap_pyfunction!(rule_degree, &m)?)?;
        m.add_function(wrap_pyfunction!(rule_name, &m)?)?;
        m.add_function(wrap_pyfunction!(single_seed, &m)?)?;
        m.add_function(wrap_pyfunction!(step, &m)?)?;
        m.add_function(wrap_pyfunction!(surjective, &m)?)?;
        m.add_function(wrap_pyfunction!(tessellate, &m)?)?;
        m.add_function(wrap_pyfunction!(wolfram_class, &m)?)?;
        let names: Vec<&str> = vec!["affine", "animate", "churn", "corner_bits", "counts", "crop", "cube_orbit", "design_mask", "entropy", "frames", "gasket", "genus", "heatmap", "history", "lambda_", "lattice_index", "mask_offsets", "moore", "movie", "next_grid", "npn_class", "outer_totalistic", "popcount", "reversible", "rule_degree", "rule_name", "single_seed", "step", "surjective", "tessellate", "wolfram_class", "Config", "Counts", "Life", "Rule", "Source", "Boundary", "Fate"];
        m.add("__all__", names)?;
        elementary::init(py, &m, sys)?;
        render::init(py, &m, sys)?;
        source::init(py, &m, sys)?;
        parent.add("life", &m)?;
        sys.set_item("mrlypy._mrlypy.life", &m)?;
        Ok(())
    }
}

/// The designs in space: codes, cells, cubes, hexagons, their counts, graphs and names.
/// The designs in space.
pub mod math {
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// Ready-made tensors: zeros, ones, noise and carpets in two or three dimensions.
    pub mod atoms {
        use crate::hand::{PyRng, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Builds an n by n carpet, on where at most one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "carpet_2d", signature = (n))]
        pub fn carpet_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::carpet_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n carpet, on where at most one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "carpet_3d", signature = (n))]
        pub fn carpet_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::carpet_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a carpet of the given side at any rank, on where at most one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "carpet_nd", signature = (n, rank))]
        pub fn carpet_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::carpet_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n dust, on where both coordinates are even.
        #[pyfunction]
        #[pyo3(name = "dust_2d", signature = (n))]
        pub fn dust_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::dust_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n dust, on where all three coordinates are even.
        #[pyfunction]
        #[pyo3(name = "dust_3d", signature = (n))]
        pub fn dust_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::dust_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a dust of the given side at any rank, on where every coordinate is even.
        #[pyfunction]
        #[pyo3(name = "dust_nd", signature = (n, rank))]
        pub fn dust_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::dust_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n line, free on axis 1, on along the odd rows.
        #[pyfunction]
        #[pyo3(name = "hline_2d", signature = (n))]
        pub fn hline_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::hline_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tree, free on axis 1, on along the even rows.
        #[pyfunction]
        #[pyo3(name = "htree_2d", signature = (n))]
        pub fn htree_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::htree_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none.
        #[pyfunction]
        #[pyo3(name = "line_nd", signature = (n, rank, axis))]
        pub fn line_nd<'py>(py: Python<'py>, n: usize, rank: usize, axis: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::line_nd(n, rank, axis);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n net, on where at least one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "net_2d", signature = (n))]
        pub fn net_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::net_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n net, on where at least two coordinates are odd.
        #[pyfunction]
        #[pyo3(name = "net_3d", signature = (n))]
        pub fn net_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::net_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank.
        #[pyfunction]
        #[pyo3(name = "net_nd", signature = (n, rank))]
        pub fn net_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::net_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tensor where each cell turns on with probability density, drawn from the stream.
        #[pyfunction]
        #[pyo3(name = "noise_2d", signature = (n, density, rng))]
        pub fn noise_2d<'py>(py: Python<'py>, n: usize, density: f64, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::noise_2d(n, density, &mut rng.0);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream.
        #[pyfunction]
        #[pyo3(name = "noise_3d", signature = (n, density, rng))]
        pub fn noise_3d<'py>(py: Python<'py>, n: usize, density: f64, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::noise_3d(n, density, &mut rng.0);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tensor of ones.
        #[pyfunction]
        #[pyo3(name = "ones_2d", signature = (n))]
        pub fn ones_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::ones_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tensor of ones.
        #[pyfunction]
        #[pyo3(name = "ones_3d", signature = (n))]
        pub fn ones_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::ones_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n point, on where both coordinates are odd.
        #[pyfunction]
        #[pyo3(name = "point_2d", signature = (n))]
        pub fn point_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::point_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n point, on where all three coordinates are odd.
        #[pyfunction]
        #[pyo3(name = "point_3d", signature = (n))]
        pub fn point_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::point_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a point of the given side at any rank, on where every coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "point_nd", signature = (n, rank))]
        pub fn point_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::point_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n star, on where exactly one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "star_2d", signature = (n))]
        pub fn star_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::star_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n star, on where exactly one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "star_3d", signature = (n))]
        pub fn star_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::star_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a star of the given side at any rank, on where exactly one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "star_nd", signature = (n, rank))]
        pub fn star_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::star_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none.
        #[pyfunction]
        #[pyo3(name = "tree_nd", signature = (n, rank, axis))]
        pub fn tree_nd<'py>(py: Python<'py>, n: usize, rank: usize, axis: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::tree_nd(n, rank, axis);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n line, free on axis 0, on along the odd columns.
        #[pyfunction]
        #[pyo3(name = "vline_2d", signature = (n))]
        pub fn vline_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::vline_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n void, on where both coordinates share one parity.
        #[pyfunction]
        #[pyo3(name = "void_2d", signature = (n))]
        pub fn void_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::void_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n void, on where all three coordinates share one parity.
        #[pyfunction]
        #[pyo3(name = "void_3d", signature = (n))]
        pub fn void_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::void_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a void of the given side at any rank, on where every coordinate shares one parity.
        #[pyfunction]
        #[pyo3(name = "void_nd", signature = (n, rank))]
        pub fn void_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::void_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tree, free on axis 0, on along the even columns.
        #[pyfunction]
        #[pyo3(name = "vtree_2d", signature = (n))]
        pub fn vtree_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::vtree_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n line, free on axis 0, its rods running along x.
        #[pyfunction]
        #[pyo3(name = "xline_3d", signature = (n))]
        pub fn xline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::xline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tree, free on axis 0, its beams running along x.
        #[pyfunction]
        #[pyo3(name = "xtree_3d", signature = (n))]
        pub fn xtree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::xtree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n line, free on axis 1, its rods running along y.
        #[pyfunction]
        #[pyo3(name = "yline_3d", signature = (n))]
        pub fn yline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::yline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tree, free on axis 1, its beams running along y.
        #[pyfunction]
        #[pyo3(name = "ytree_3d", signature = (n))]
        pub fn ytree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::ytree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tensor of zeros.
        #[pyfunction]
        #[pyo3(name = "zeros_2d", signature = (n))]
        pub fn zeros_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::zeros_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tensor of zeros.
        #[pyfunction]
        #[pyo3(name = "zeros_3d", signature = (n))]
        pub fn zeros_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::zeros_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n line, free on axis 2, its rods running along z.
        #[pyfunction]
        #[pyo3(name = "zline_3d", signature = (n))]
        pub fn zline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::zline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tree, free on axis 2, its beams running along z.
        #[pyfunction]
        #[pyo3(name = "ztree_3d", signature = (n))]
        pub fn ztree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::atoms::ztree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.atoms")?;
            m.setattr("__doc__", "Ready-made tensors: zeros, ones, noise and carpets in two or three dimensions.")?;
            m.add_function(wrap_pyfunction!(carpet_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(carpet_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(carpet_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(dust_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(dust_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(dust_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(hline_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(htree_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(line_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(net_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(net_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(net_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(noise_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(noise_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(ones_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(ones_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(point_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(point_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(point_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(star_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(star_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(star_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(tree_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(vline_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(void_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(void_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(void_nd, &m)?)?;
            m.add_function(wrap_pyfunction!(vtree_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(xline_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(xtree_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(yline_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(ytree_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(zeros_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(zeros_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(zline_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(ztree_3d, &m)?)?;
            let names: Vec<&str> = vec!["carpet_2d", "carpet_3d", "carpet_nd", "dust_2d", "dust_3d", "dust_nd", "hline_2d", "htree_2d", "line_nd", "net_2d", "net_3d", "net_nd", "noise_2d", "noise_3d", "ones_2d", "ones_3d", "point_2d", "point_3d", "point_nd", "star_2d", "star_3d", "star_nd", "tree_nd", "vline_2d", "void_2d", "void_3d", "void_nd", "vtree_2d", "xline_3d", "xtree_3d", "yline_3d", "ytree_3d", "zeros_2d", "zeros_3d", "zline_3d", "ztree_3d"];
            m.add("__all__", names)?;
            parent.add("atoms", &m)?;
            sys.set_item("mrlypy._mrlypy.math.atoms", &m)?;
            Ok(())
        }
    }

    /// The universe of design codes: corners, symmetries and their counts.
    /// The universe of design codes.
    pub mod bang {
        use crate::hand::{ok, PyCode, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The base-q symmetry maps, the design counts raw and distinct, and the fill classes.
        pub mod baseq {
            use crate::hand::{ok, PyCode};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Returns the distinct rotation and reflection maps of a base-q axis.
            #[pyfunction]
            #[pyo3(name = "axis_maps", signature = (base))]
            pub fn axis_maps<'py>(py: Python<'py>, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::axis_maps(base);
                (out).into_bound_py_any(py)
            }

            /// Returns the distinct one-dimensional design counts for bases 1 through max_base.
            #[pyfunction]
            #[pyo3(name = "bracelets", signature = (max_base))]
            pub fn bracelets<'py>(py: Python<'py>, max_base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::bracelets(max_base);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the least code of the design's orbit.
            #[pyfunction]
            #[pyo3(name = "canonical", signature = (group, code))]
            pub fn canonical<'py>(py: Python<'py>, group: Vec<Vec<usize>>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::baseq::canonical(&group, code);
                (PyCode(ok(out)?)).into_bound_py_any(py)
            }

            /// Carries a code through one group element.
            #[pyfunction]
            #[pyo3(name = "carry", signature = (element, code))]
            pub fn carry<'py>(py: Python<'py>, element: Vec<usize>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::baseq::carry(&element, code);
                (PyCode(out)).into_bound_py_any(py)
            }

            /// Returns the fill-class counts for dimensions 1 through max_dimension.
            #[pyfunction]
            #[pyo3(name = "class_sequence", signature = (max_dimension))]
            pub fn class_sequence<'py>(py: Python<'py>, max_dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::class_sequence(max_dimension);
                (out).into_bound_py_any(py)
            }

            /// Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension.
            #[pyfunction]
            #[pyo3(name = "classes", signature = (dimension))]
            pub fn classes<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::classes(dimension);
                (out).into_bound_py_any(py)
            }

            /// Counts base-q designs distinct under symmetry.
            #[pyfunction]
            #[pyo3(name = "distinct_designs", signature = (base, dimension))]
            pub fn distinct_designs<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::distinct_designs(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the collapsed fill count at an even side number.
            #[pyfunction]
            #[pyo3(name = "even_fill_is_balanced", signature = (number, dimension, popcount))]
            pub fn even_fill_is_balanced<'py>(py: Python<'py>, number: usize, dimension: usize, popcount: u128) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::even_fill_is_balanced(number, dimension, popcount);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the filled-cell count of a binary design at a side number, folded from its filled corners.
            #[pyfunction]
            #[pyo3(name = "fill_from_corners", signature = (filled, number, dimension))]
            pub fn fill_from_corners<'py>(py: Python<'py>, filled: Vec<Vec<u8>>, number: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::fill_from_corners(&filled, number, dimension);
                (out).into_bound_py_any(py)
            }

            /// Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`.
            #[pyfunction]
            #[pyo3(name = "group", signature = (base, dimension))]
            pub fn group<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::group(base, dimension);
                (out).into_bound_py_any(py)
            }

            /// Returns the symmetry group order counted from the enumerated axis maps.
            #[pyfunction]
            #[pyo3(name = "group_order", signature = (base, dimension))]
            pub fn group_order<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::group_order(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns every code a design reaches under the group.
            #[pyfunction]
            #[pyo3(name = "orbit", signature = (group, code))]
            pub fn orbit<'py>(py: Python<'py>, group: Vec<Vec<usize>>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::baseq::orbit(&group, code);
                ((out).into_iter().map(PyCode).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Returns the closed-form group order the axis-map count must match.
            #[pyfunction]
            #[pyo3(name = "predicted_group_order", signature = (base, dimension))]
            pub fn predicted_group_order<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::predicted_group_order(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Walks every code of a base and dimension and returns each orbit's least code with the orbit's size.
            #[pyfunction]
            #[pyo3(name = "representatives", signature = (base, dimension))]
            pub fn representatives<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::representatives(base, dimension);
                ((ok(out)?).into_iter().map(|x| { let t = x; (PyCode(t.0), t.1) }).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Returns the distinct-design counts for dimensions 1 through max_dimension.
            #[pyfunction]
            #[pyo3(name = "sequence", signature = (base, max_dimension))]
            pub fn sequence<'py>(py: Python<'py>, base: usize, max_dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::sequence(base, max_dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the raw design count before symmetry, two to the number of cells.
            #[pyfunction]
            #[pyo3(name = "total_designs", signature = (base, dimension))]
            pub fn total_designs<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::baseq::total_designs(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.bang.baseq")?;
                m.setattr("__doc__", "The base-q symmetry maps, the design counts raw and distinct, and the fill classes.")?;
                m.add_function(wrap_pyfunction!(axis_maps, &m)?)?;
                m.add_function(wrap_pyfunction!(bracelets, &m)?)?;
                m.add_function(wrap_pyfunction!(canonical, &m)?)?;
                m.add_function(wrap_pyfunction!(carry, &m)?)?;
                m.add_function(wrap_pyfunction!(class_sequence, &m)?)?;
                m.add_function(wrap_pyfunction!(classes, &m)?)?;
                m.add_function(wrap_pyfunction!(distinct_designs, &m)?)?;
                m.add_function(wrap_pyfunction!(even_fill_is_balanced, &m)?)?;
                m.add_function(wrap_pyfunction!(fill_from_corners, &m)?)?;
                m.add_function(wrap_pyfunction!(group, &m)?)?;
                m.add_function(wrap_pyfunction!(group_order, &m)?)?;
                m.add_function(wrap_pyfunction!(orbit, &m)?)?;
                m.add_function(wrap_pyfunction!(predicted_group_order, &m)?)?;
                m.add_function(wrap_pyfunction!(representatives, &m)?)?;
                m.add_function(wrap_pyfunction!(sequence, &m)?)?;
                m.add_function(wrap_pyfunction!(total_designs, &m)?)?;
                m.add("WALK_LIMIT", mrlyrs::math::bang::baseq::WALK_LIMIT)?;
                let names: Vec<&str> = vec!["axis_maps", "bracelets", "canonical", "carry", "class_sequence", "classes", "distinct_designs", "even_fill_is_balanced", "fill_from_corners", "group", "group_order", "orbit", "predicted_group_order", "representatives", "sequence", "total_designs", "WALK_LIMIT"];
                m.add("__all__", names)?;
                parent.add("baseq", &m)?;
                sys.set_item("mrlypy._mrlypy.math.bang.baseq", &m)?;
                Ok(())
            }
        }

        /// The cached canonical codes and tile sources of a dimension.
        pub mod catalog {
            use crate::hand::{PySerde};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Returns the anti designs for a dimension.
            #[pyfunction]
            #[pyo3(name = "antis", signature = (dimension))]
            pub fn antis<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::catalog::antis(dimension);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.bang.catalog")?;
                m.setattr("__doc__", "The cached canonical codes and tile sources of a dimension.")?;
                m.add_function(wrap_pyfunction!(antis, &m)?)?;
                m.add("ANTIS_2D", (mrlyrs::math::bang::catalog::ANTIS_2D).into_iter().map(PySerde).collect::<Vec<_>>())?;
                m.add("ANTIS_3D", (mrlyrs::math::bang::catalog::ANTIS_3D).into_iter().map(PySerde).collect::<Vec<_>>())?;
                let names: Vec<&str> = vec!["antis", "ANTIS_2D", "ANTIS_3D"];
                m.add("__all__", names)?;
                parent.add("catalog", &m)?;
                sys.set_item("mrlypy._mrlypy.math.bang.catalog", &m)?;
                Ok(())
            }
        }

        /// The design code: the bitmask of filled corners, printed and parsed as one number.
        pub mod code {
            use crate::hand::{PyCode};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Returns the bitmask the code carries.
            #[pyfunction]
            #[pyo3(name = "get", signature = (code))]
            pub fn get<'py>(py: Python<'py>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::Code::get(code);
                (out).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.bang.code")?;
                m.setattr("__doc__", "The design code: the bitmask of filled corners, printed and parsed as one number.")?;
                m.add_function(wrap_pyfunction!(get, &m)?)?;
                let names: Vec<&str> = vec!["get"];
                m.add("__all__", names)?;
                parent.add("code", &m)?;
                sys.set_item("mrlypy._mrlypy.math.bang.code", &m)?;
                Ok(())
            }
        }

        /// The packing of residue corners into codes and back.
        pub mod factory {
            use crate::hand::{ok, PyCode, PyTensor};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Renders a coded design to a tensor at its side number, dimension, base and fractal level.
            #[pyfunction]
            #[pyo3(name = "create", signature = (code, number, dimension, base, level))]
            pub fn create<'py>(py: Python<'py>, code: PyCode, number: usize, dimension: usize, base: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::factory::create(code, number, dimension, base, level);
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            /// Renders a design straight from its filled residue corners.
            #[pyfunction]
            #[pyo3(name = "create_from_corners", signature = (filled, number, dimension, base, level))]
            pub fn create_from_corners<'py>(py: Python<'py>, filled: Vec<Vec<u8>>, number: usize, dimension: usize, base: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::factory::create_from_corners(&filled, number, dimension, base, level);
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            /// Renders a design from its canonical JSON name.
            #[pyfunction]
            #[pyo3(name = "create_named", signature = (spec, number, level))]
            pub fn create_named<'py>(py: Python<'py>, spec: &str, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::factory::create_named(spec, number, level);
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            /// Returns every base-q residue corner of a dimension in row-major order.
            #[pyfunction]
            #[pyo3(name = "residue_corners", signature = (dimension, base))]
            pub fn residue_corners<'py>(py: Python<'py>, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::factory::residue_corners(dimension, base);
                (out).into_bound_py_any(py)
            }

            /// Returns the code count of a dimension and base, two to the number of corners.
            #[pyfunction]
            #[pyo3(name = "total_codes", signature = (dimension, base))]
            pub fn total_codes<'py>(py: Python<'py>, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::factory::total_codes(dimension, base);
                (PyCode(ok(out)?)).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.bang.factory")?;
                m.setattr("__doc__", "The packing of residue corners into codes and back.")?;
                m.add_function(wrap_pyfunction!(create, &m)?)?;
                m.add_function(wrap_pyfunction!(create_from_corners, &m)?)?;
                m.add_function(wrap_pyfunction!(create_named, &m)?)?;
                m.add_function(wrap_pyfunction!(residue_corners, &m)?)?;
                m.add_function(wrap_pyfunction!(total_codes, &m)?)?;
                let names: Vec<&str> = vec!["create", "create_from_corners", "create_named", "residue_corners", "total_codes"];
                m.add("__all__", names)?;
                parent.add("factory", &m)?;
                sys.set_item("mrlypy._mrlypy.math.bang.factory", &m)?;
                Ok(())
            }
        }

        /// The corners, codes and symmetries that name designs.
        pub mod universe {
            use crate::hand::{PyCode};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Returns the algebraic normal form coefficients of a code, one per corner.
            #[pyfunction]
            #[pyo3(name = "anf", signature = (code, dimension))]
            pub fn anf<'py>(py: Python<'py>, code: PyCode, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::universe::anf(code, dimension);
                (out).into_bound_py_any(py)
            }

            /// Formats the algebraic normal form of a code as a sum of monomials.
            #[pyfunction]
            #[pyo3(name = "anf_string", signature = (code, dimension))]
            pub fn anf_string<'py>(py: Python<'py>, code: PyCode, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::universe::anf_string(code, dimension);
                (out).into_bound_py_any(py)
            }

            /// Applies a symmetry element to a corner.
            #[pyfunction]
            #[pyo3(name = "apply", signature = (element, corner))]
            pub fn apply<'py>(py: Python<'py>, element: (Vec<usize>, Vec<u8>), corner: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::universe::apply(&element, &corner);
                (out).into_bound_py_any(py)
            }

            /// Returns the bit position a binary corner occupies in a code.
            #[pyfunction]
            #[pyo3(name = "corner_index", signature = (corner))]
            pub fn corner_index<'py>(py: Python<'py>, corner: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::universe::corner_index(&corner);
                (out).into_bound_py_any(py)
            }

            /// Returns the algebraic degree of a code, or -1 for the zero design.
            #[pyfunction]
            #[pyo3(name = "degree", signature = (code, dimension))]
            pub fn degree<'py>(py: Python<'py>, code: PyCode, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::universe::degree(code, dimension);
                (out).into_bound_py_any(py)
            }

            /// Returns every code a design reaches under the full symmetry group.
            #[pyfunction]
            #[pyo3(name = "orbit", signature = (code, dimension))]
            pub fn orbit<'py>(py: Python<'py>, code: PyCode, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::universe::orbit(code, dimension);
                ((out).into_iter().map(PyCode).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Returns every permutation of 0..n in sorted order.
            #[pyfunction]
            #[pyo3(name = "permutations", signature = (n))]
            pub fn permutations<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::universe::permutations(n);
                (out).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.bang.universe")?;
                m.setattr("__doc__", "The corners, codes and symmetries that name designs.")?;
                m.add_function(wrap_pyfunction!(anf, &m)?)?;
                m.add_function(wrap_pyfunction!(anf_string, &m)?)?;
                m.add_function(wrap_pyfunction!(apply, &m)?)?;
                m.add_function(wrap_pyfunction!(corner_index, &m)?)?;
                m.add_function(wrap_pyfunction!(degree, &m)?)?;
                m.add_function(wrap_pyfunction!(orbit, &m)?)?;
                m.add_function(wrap_pyfunction!(permutations, &m)?)?;
                let names: Vec<&str> = vec!["anf", "anf_string", "apply", "corner_index", "degree", "orbit", "permutations"];
                m.add("__all__", names)?;
                parent.add("universe", &m)?;
                sys.set_item("mrlypy._mrlypy.math.bang.universe", &m)?;
                Ok(())
            }
        }

        /// The magic words: their products, their component counts and the schedules that spell them.
        pub mod word {
            use crate::hand::{ok, PySerde};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// The named infinite schedules over an ordered pair of letters.
            #[pyclass(name = "Schedule", module = "mrlypy.math.bang.word", skip_from_py_object)]
            pub struct Schedule;

            #[pymethods]
            impl Schedule {
                /// Returns every Schedule in canonical order.
                #[staticmethod]
                #[pyo3(name = "all", signature = ())]
                pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::bang::word::Schedule::all();
                    ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
                }
                /// Returns the letter frequencies the schedule tends to.
                #[staticmethod]
                #[pyo3(name = "frequencies", signature = (schedule))]
                pub fn frequencies<'py>(py: Python<'py>, schedule: PySerde<mrlyrs::math::bang::word::Schedule>) -> PyResult<Bound<'py, PyAny>> {
                    let schedule = schedule.0;
                    let out = mrlyrs::math::bang::word::Schedule::frequencies(schedule);
                    (out).into_bound_py_any(py)
                }
                /// Returns the letter the schedule takes at the place, zero or one.
                #[staticmethod]
                #[pyo3(name = "place", signature = (schedule, index))]
                pub fn place<'py>(py: Python<'py>, schedule: PySerde<mrlyrs::math::bang::word::Schedule>, index: usize) -> PyResult<Bound<'py, PyAny>> {
                    let schedule = schedule.0;
                    let out = mrlyrs::math::bang::word::Schedule::place(schedule, index);
                    (out).into_bound_py_any(py)
                }
            }

            /// Counts the 4-connected components of a plane word without drawing it.
            #[pyfunction]
            #[pyo3(name = "components", signature = (layers))]
            pub fn components<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::components(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the constant-word component functional of a plane word's letter frequencies,
            /// in log two units.
            #[pyfunction]
            #[pyo3(name = "constant_functional", signature = (layers))]
            pub fn constant_functional<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::constant_functional(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides.
            #[pyfunction]
            #[pyo3(name = "dimension", signature = (layers))]
            pub fn dimension<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::dimension(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the filled cells of a word, the product of its letter fills.
            #[pyfunction]
            #[pyo3(name = "fill", signature = (layers))]
            pub fn fill<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::fill(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Lists the filled cells of every letter, the product of which is the word's fill.
            #[pyfunction]
            #[pyo3(name = "fills", signature = (layers))]
            pub fn fills<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::fills(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Reads one plane letter: its fill, its runs, the rows and columns that wrap into a
            /// neighbouring copy, and its own components.
            #[pyfunction]
            #[pyo3(name = "letter", signature = (layer))]
            pub fn letter<'py>(py: Python<'py>, layer: PySerde<mrlyrs::math::bang::MagicLayer>) -> PyResult<Bound<'py, PyAny>> {
                let layer = layer.0;
                let out = mrlyrs::math::bang::word::letter(&layer);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            /// Returns whether every letter renders at its own residue base, the native case where a
            /// periodic word folds to one residue rule at the product base.
            #[pyfunction]
            #[pyo3(name = "native", signature = (layers))]
            pub fn native<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::native(&layers);
                (out).into_bound_py_any(py)
            }

            /// Returns the shortest whole period of the letter list, its own length when no shorter block repeats.
            #[pyfunction]
            #[pyo3(name = "period", signature = (layers))]
            pub fn period<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::period(&layers);
                (out).into_bound_py_any(py)
            }

            /// Folds a plane word letter by letter and returns the counts at every prefix.
            #[pyfunction]
            #[pyo3(name = "prefixes", signature = (layers))]
            pub fn prefixes<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::prefixes(&layers);
                ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Returns the prefix rates of a plane word in log two units, the component rate
            /// `(1/L) log2 comp` and the fill rate `(1/L) log2 fill` at every prefix length.
            #[pyfunction]
            #[pyo3(name = "rates", signature = (layers))]
            pub fn rates<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::rates(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the side of a word, the product of its letter sides.
            #[pyfunction]
            #[pyo3(name = "side", signature = (layers))]
            pub fn side<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrlyrs::math::bang::word::side(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Spells the first letters of a schedule over an ordered pair of letters.
            #[pyfunction]
            #[pyo3(name = "spell", signature = (schedule, pair, length))]
            pub fn spell<'py>(py: Python<'py>, schedule: PySerde<mrlyrs::math::bang::word::Schedule>, pair: (PySerde<mrlyrs::math::bang::MagicLayer>, PySerde<mrlyrs::math::bang::MagicLayer>), length: usize) -> PyResult<Bound<'py, PyAny>> {
                let schedule = schedule.0;
                let pair = { let t = pair; (t.0.0, t.1.0) };
                let out = mrlyrs::math::bang::word::spell(schedule, pair, length);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,
            /// then `magic(3,5)`, then `magic(3,5,7)`, and so on.
            #[pyfunction]
            #[pyo3(name = "staircase", signature = (depth))]
            pub fn staircase<'py>(py: Python<'py>, depth: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::word::staircase(depth);
                ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
            #[pyfunction]
            #[pyo3(name = "thue_morse", signature = (index))]
            pub fn thue_morse<'py>(py: Python<'py>, index: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::word::thue_morse(index);
                (out).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.bang.word")?;
                m.setattr("__doc__", "The magic words: their products, their component counts and the schedules that spell them.")?;
                m.add_class::<Schedule>()?;
                m.add_function(wrap_pyfunction!(components, &m)?)?;
                m.add_function(wrap_pyfunction!(constant_functional, &m)?)?;
                m.add_function(wrap_pyfunction!(dimension, &m)?)?;
                m.add_function(wrap_pyfunction!(fill, &m)?)?;
                m.add_function(wrap_pyfunction!(fills, &m)?)?;
                m.add_function(wrap_pyfunction!(letter, &m)?)?;
                m.add_function(wrap_pyfunction!(native, &m)?)?;
                m.add_function(wrap_pyfunction!(period, &m)?)?;
                m.add_function(wrap_pyfunction!(prefixes, &m)?)?;
                m.add_function(wrap_pyfunction!(rates, &m)?)?;
                m.add_function(wrap_pyfunction!(side, &m)?)?;
                m.add_function(wrap_pyfunction!(spell, &m)?)?;
                m.add_function(wrap_pyfunction!(staircase, &m)?)?;
                m.add_function(wrap_pyfunction!(thue_morse, &m)?)?;
                let names: Vec<&str> = vec!["components", "constant_functional", "dimension", "fill", "fills", "letter", "native", "period", "prefixes", "rates", "side", "spell", "staircase", "thue_morse", "Schedule"];
                m.add("__all__", names)?;
                parent.add("word", &m)?;
                sys.set_item("mrlypy._mrlypy.math.bang.word", &m)?;
                Ok(())
            }
        }

        /// A single design with its place in the orbit structure.
        #[pyclass(name = "Design", module = "mrlypy.math.bang", from_py_object)]
        #[derive(Clone)]
        pub struct Design(pub mrlyrs::math::bang::Design);

        #[pymethods]
        impl Design {
            /// The design's code.
            #[getter]
            #[pyo3(name = "i")]
            pub fn i<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.i;
                (PyCode(value)).into_bound_py_any(py)
            }
            /// The design's dimension.
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            /// Whether this code is the smallest in its orbit.
            #[getter]
            #[pyo3(name = "canonical")]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.canonical;
                (value).into_bound_py_any(py)
            }
            /// The smallest code in the orbit.
            #[getter]
            #[pyo3(name = "class_rep")]
            pub fn class_rep<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.class_rep;
                (PyCode(value)).into_bound_py_any(py)
            }
            /// The number of codes in the orbit.
            #[getter]
            #[pyo3(name = "orbit_size")]
            pub fn orbit_size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.orbit_size;
                (value).into_bound_py_any(py)
            }
            /// Returns the design's algebraic normal form as a string.
            #[pyo3(name = "anf", signature = ())]
            pub fn anf<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Design::anf(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the design's algebraic degree, or -1 for the zero design.
            #[pyo3(name = "degree", signature = ())]
            pub fn degree<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Design::degree(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the design's name as a line of prose, `bang dim 2, code 7`.
            #[pyo3(name = "name", signature = ())]
            pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Design::name(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the design's filled corners in sorted order.
            #[pyo3(name = "rule", signature = ())]
            pub fn rule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Design::rule(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// The complete enumeration of one dimension's designs and orbits.
        #[pyclass(name = "Universe", module = "mrlypy.math.bang", skip_from_py_object)]
        pub struct Universe(pub mrlyrs::math::bang::Universe);

        #[pymethods]
        impl Universe {
            /// Enumerates every orbit of a dimension from 1 to 4.
            #[new]
            #[pyo3(signature = (dimension))]
            pub fn __new__(dimension: usize) -> PyResult<Self> {
                let out = mrlyrs::math::bang::Universe::new(dimension);
                Ok(Self(ok(out)?))
            }
            /// The universe's dimension.
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            /// The number of codes in the universe.
            #[getter]
            #[pyo3(name = "total")]
            pub fn total<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.total;
                (value).into_bound_py_any(py)
            }
            /// Returns every design in code order.
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Universe::all(&self.0);
                ((out).into_iter().map(crate::gen::math::bang::Design).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the designs whose codes lead their orbits.
            #[pyo3(name = "canonical", signature = ())]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Universe::canonical(&self.0);
                ((out).into_iter().map(crate::gen::math::bang::Design).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the design at a code with its precomputed orbit facts.
            #[pyo3(name = "design", signature = (code))]
            pub fn design<'py>(&self, py: Python<'py>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::bang::Universe::design(&self.0, code);
                (crate::gen::math::bang::Design(out)).into_bound_py_any(py)
            }
            /// Returns the number of distinct orbits.
            #[pyo3(name = "distinct", signature = ())]
            pub fn distinct<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Universe::distinct(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Enumerates every orbit of a dimension from 1 to 4.
            #[staticmethod]
            #[pyo3(name = "new", signature = (dimension))]
            pub fn new_<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::bang::Universe::new(dimension);
                (crate::gen::math::bang::Universe(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// One ordered layer of a magic composition: a coded design at its own side number.
        #[pyclass(name = "MagicLayer", module = "mrlypy.math.bang", skip_from_py_object)]
        pub struct MagicLayer;

        #[pymethods]
        impl MagicLayer {
            /// Pins a design to the side number it renders at.
            #[staticmethod]
            #[pyo3(name = "new", signature = (design, number))]
            pub fn new_<'py>(py: Python<'py>, design: crate::gen::math::name::Bang, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let design = design.0;
                let out = mrlyrs::math::bang::MagicLayer::new(design, number);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// Builds the universe of a dimension.
        #[pyfunction]
        #[pyo3(name = "bang", signature = (dimension))]
        pub fn bang<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::bang::bang(dimension);
            (crate::gen::math::bang::Universe(ok(out)?)).into_bound_py_any(py)
        }

        /// Unpacks a code into its filled residue corners.
        #[pyfunction]
        #[pyo3(name = "code_to_corners", signature = (code, dimension, base))]
        pub fn code_to_corners<'py>(py: Python<'py>, code: PyCode, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::bang::code_to_corners(code, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the binary corners of a dimension in code order.
        #[pyfunction]
        #[pyo3(name = "corners", signature = (dimension))]
        pub fn corners<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::bang::corners(dimension);
            (out).into_bound_py_any(py)
        }

        /// Packs filled residue corners back into their code.
        #[pyfunction]
        #[pyo3(name = "corners_to_code", signature = (filled, dimension, base))]
        pub fn corners_to_code<'py>(py: Python<'py>, filled: Vec<Vec<u8>>, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::bang::corners_to_code(&filled, dimension, base);
            (PyCode(out)).into_bound_py_any(py)
        }

        /// Returns the code of the design filled wherever a corner's residue sum lands in the levels.
        #[pyfunction]
        #[pyo3(name = "levels_code", signature = (dimension, base, levels))]
        pub fn levels_code<'py>(py: Python<'py>, dimension: usize, base: usize, levels: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::bang::levels_code(dimension, base, &levels);
            (PyCode(out)).into_bound_py_any(py)
        }

        /// Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost.
        #[pyfunction]
        #[pyo3(name = "magic", signature = (layers))]
        pub fn magic<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::bang::magic(&layers);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Composes JSON-named layers in order.
        #[pyfunction]
        #[pyo3(name = "magic_named", signature = (layers))]
        pub fn magic_named<'py>(py: Python<'py>, layers: Vec<(String, usize)>) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.iter().map(|y| (y.0.as_str(), y.1)).collect::<Vec<_>>();
            let out = mrlyrs::math::bang::magic_named(&layers);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the tile sources a catalog names at a dimension.
        #[pyfunction]
        #[pyo3(name = "sources", signature = (catalog, dimension))]
        pub fn sources<'py>(py: Python<'py>, catalog: PySerde<mrlyrs::gen::recipe::Catalog>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let catalog = catalog.0;
            let out = mrlyrs::math::bang::sources(&catalog, dimension);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the full symmetry group as axis permutations paired with flip patterns.
        #[pyfunction]
        #[pyo3(name = "symmetries", signature = (dimension))]
        pub fn symmetries<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::bang::symmetries(dimension);
            (out).into_bound_py_any(py)
        }

        /// Returns whether no two filled corners of a code sit at Hamming distance one.
        #[pyfunction]
        #[pyo3(name = "total_exposure", signature = (code, dimension))]
        pub fn total_exposure<'py>(py: Python<'py>, code: PyCode, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::bang::total_exposure(code, dimension);
            (out).into_bound_py_any(py)
        }

        /// Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side.
        #[pyfunction]
        #[pyo3(name = "touches_every_corner", signature = (code, dimension))]
        pub fn touches_every_corner<'py>(py: Python<'py>, code: PyCode, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::bang::touches_every_corner(code, dimension);
            (out).into_bound_py_any(py)
        }

        /// Returns the canonical design codes of a dimension, computed once and cached for the process.
        #[pyfunction]
        #[pyo3(name = "universe_codes", signature = (dimension))]
        pub fn universe_codes<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::bang::universe_codes(dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.bang")?;
            m.setattr("__doc__", "The universe of design codes: corners, symmetries and their counts.\nThe universe of design codes.\n\nA code is a bitmask over the residue corners of a hypercube; this folder packs corners into\ncodes, folds codes into symmetry classes, counts the classes and spells the magic words.")?;
            m.add_class::<Design>()?;
            m.add_class::<Universe>()?;
            m.add_class::<MagicLayer>()?;
            m.add_function(wrap_pyfunction!(bang, &m)?)?;
            m.add_function(wrap_pyfunction!(code_to_corners, &m)?)?;
            m.add_function(wrap_pyfunction!(corners, &m)?)?;
            m.add_function(wrap_pyfunction!(corners_to_code, &m)?)?;
            m.add_function(wrap_pyfunction!(levels_code, &m)?)?;
            m.add_function(wrap_pyfunction!(magic, &m)?)?;
            m.add_function(wrap_pyfunction!(magic_named, &m)?)?;
            m.add_function(wrap_pyfunction!(sources, &m)?)?;
            m.add_function(wrap_pyfunction!(symmetries, &m)?)?;
            m.add_function(wrap_pyfunction!(total_exposure, &m)?)?;
            m.add_function(wrap_pyfunction!(touches_every_corner, &m)?)?;
            m.add_function(wrap_pyfunction!(universe_codes, &m)?)?;
            let names: Vec<&str> = vec!["bang", "code_to_corners", "corners", "corners_to_code", "levels_code", "magic", "magic_named", "sources", "symmetries", "total_exposure", "touches_every_corner", "universe_codes", "Design", "Universe", "MagicLayer"];
            m.add("__all__", names)?;
            baseq::init(py, &m, sys)?;
            catalog::init(py, &m, sys)?;
            code::init(py, &m, sys)?;
            factory::init(py, &m, sys)?;
            universe::init(py, &m, sys)?;
            word::init(py, &m, sys)?;
            parent.add("bang", &m)?;
            sys.set_item("mrlypy._mrlypy.math.bang", &m)?;
            Ok(())
        }
    }

    /// The dimension-generic cell and the pipeline the fixed dimensions share.
    /// The N-dimensional cell and the pipeline the fixed dimensions share.
    pub mod cell {
        use crate::hand::{ok, PyCell2d, PyCell3d, PyCellNd, PyColor, PyRng, PySerde, PyTensor};
        use pyo3::exceptions::PyValueError;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The fill, void and exposure counts of an N-dimensional cell.
        pub mod census {
            use crate::hand::{ok, PyCell2d, PyCell3d};
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Counts the distinct unit edges the filled sites carry, the edge graph's branches.
            #[pyfunction]
            #[pyo3(name = "edges", signature = (cell))]
            pub fn edges<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::edges::<2>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::edges::<3>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("edges wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Counts the faces of filled sites open to emptiness or the border.
            #[pyfunction]
            #[pyo3(name = "exposure", signature = (cell))]
            pub fn exposure<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::exposure::<2>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::exposure::<3>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("exposure wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Counts the filled sites of the cell.
            #[pyfunction]
            #[pyo3(name = "fills", signature = (cell))]
            pub fn fills<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::fills::<2>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::fills::<3>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("fills wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Counts the distinct corners the filled sites touch, the edge graph's nodes.
            #[pyfunction]
            #[pyo3(name = "vertices", signature = (cell))]
            pub fn vertices<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::vertices::<2>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::vertices::<3>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("vertices wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Counts the empty sites of the cell.
            #[pyfunction]
            #[pyo3(name = "voids", signature = (cell))]
            pub fn voids<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::voids::<2>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::census::voids::<3>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("voids wants a 2d or 3d argument, got {other}d."))),
                }
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.cell.census")?;
                m.setattr("__doc__", "The fill, void and exposure counts of an N-dimensional cell.")?;
                m.add_function(wrap_pyfunction!(edges, &m)?)?;
                m.add_function(wrap_pyfunction!(exposure, &m)?)?;
                m.add_function(wrap_pyfunction!(fills, &m)?)?;
                m.add_function(wrap_pyfunction!(vertices, &m)?)?;
                m.add_function(wrap_pyfunction!(voids, &m)?)?;
                let names: Vec<&str> = vec!["edges", "exposure", "fills", "vertices", "voids"];
                m.add("__all__", names)?;
                parent.add("census", &m)?;
                sys.set_item("mrlypy._mrlypy.math.cell.census", &m)?;
                Ok(())
            }
        }

        /// The merges, magic folds, mosaics and perforations of N-dimensional cells.
        pub mod geometry {
            use crate::hand::{ok, PyCell2d, PyCell3d, PyCellNd, PyTensor};
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Merges same-shaped cells into one block laid out by the per-axis repetition counts.
            #[pyfunction]
            #[pyo3(name = "merge_reps", signature = (cells, reps))]
            pub fn merge_reps<'py>(py: Python<'py>, cells: &Bound<'_, PyAny>, reps: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cells)?;
                match dim {
                    2 => {
                        let cells = cells.extract::<Vec<PyCell2d>>()?;
                        let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                        let out = mrlyrs::math::cell::geometry::merge_reps::<2>(&cells, &reps);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cells = cells.extract::<Vec<PyCell3d>>()?;
                        let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                        let out = mrlyrs::math::cell::geometry::merge_reps::<3>(&cells, &reps);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("merge_reps wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Writes the value into the cell wherever the tiled mask is nonzero.
            #[pyfunction]
            #[pyo3(name = "perforate", signature = (mask, cell, value))]
            pub fn perforate<'py>(py: Python<'py>, mask: PyTensor, cell: &Bound<'_, PyAny>, value: u8) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let mask = mask.0;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::geometry::perforate::<2>(&mask, &cell, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let mask = mask.0;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::geometry::perforate::<3>(&mask, &cell, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("perforate wants a 2d or 3d argument, got {other}d."))),
                }
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.cell.geometry")?;
                m.setattr("__doc__", "The merges, magic folds, mosaics and perforations of N-dimensional cells.")?;
                m.add_function(wrap_pyfunction!(merge_reps, &m)?)?;
                m.add_function(wrap_pyfunction!(perforate, &m)?)?;
                let names: Vec<&str> = vec!["merge_reps", "perforate"];
                m.add("__all__", names)?;
                parent.add("geometry", &m)?;
                sys.set_item("mrlypy._mrlypy.math.cell.geometry", &m)?;
                Ok(())
            }
        }

        /// The N-dimensional cell and its fixed-dimension aliases.
        pub mod models {
            use crate::hand::{ok, PyCell2d, PyCell3d, PyCellNd, PyColor, PyRng, PySerde, PyTensor};
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Inverts the cell.
            #[pyfunction]
            #[pyo3(name = "anti", signature = (cell))]
            pub fn anti<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::anti(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::anti(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("anti wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Maps each site to one at or above the threshold, zero below.
            #[pyfunction]
            #[pyo3(name = "binarize", signature = (cell, threshold))]
            pub fn binarize<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, threshold: u8) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::binarize(cell, threshold);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::binarize(cell, threshold);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("binarize wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Binarizes the cell at the threshold Otsu's method picks.
            #[pyfunction]
            #[pyo3(name = "binarize_otsu", signature = (cell))]
            pub fn binarize_otsu<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::binarize_otsu(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::binarize_otsu(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("binarize_otsu wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Rounds each site to the mean of its masked neighborhood, wrapping on request.
            #[pyfunction]
            #[pyo3(name = "blur", signature = (cell, mask, wrap))]
            pub fn blur<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, mask: PyTensor, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::blur(cell, &mask, wrap);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::blur(cell, &mask, wrap);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("blur wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Returns the Kronecker product of the two cells.
            #[pyfunction]
            #[pyo3(name = "combine", signature = (cell, other))]
            pub fn combine<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, other: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let other = other.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let other = other.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::combine(&cell, &other);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let other = other.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let other = other.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::combine(&cell, &other);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("combine wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Returns the narrowest count dtype that fits the mask's popcount.
            #[pyfunction]
            #[pyo3(name = "counting_dtype", signature = (mask))]
            pub fn counting_dtype<'py>(py: Python<'py>, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
                let mask = mask.0;
                let out = mrlyrs::math::cell::models::counting_dtype(&mask);
                (PySerde(out)).into_bound_py_any(py)
            }

            /// Returns the size of axis 0, the cube's leading axis.
            #[pyfunction]
            #[pyo3(name = "depth", signature = (cell))]
            pub fn depth<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
                let cell = cell.0;
                let out = mrlyrs::math::cell::models::CellNd::<3>::depth(&cell);
                (out).into_bound_py_any(py)
            }

            /// Returns the narrowest unsigned dtype that holds the peak value.
            #[pyfunction]
            #[pyo3(name = "dtype_for", signature = (peak))]
            pub fn dtype_for<'py>(py: Python<'py>, peak: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::cell::models::dtype_for(peak);
                (PySerde(out)).into_bound_py_any(py)
            }

            /// Deepens the cell into its level-fold fractal.
            #[pyfunction]
            #[pyo3(name = "fractal", signature = (cell, level))]
            pub fn fractal<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::fractal(cell, level);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::fractal(cell, level);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("fractal wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Returns the size of the axis before the last.
            #[pyfunction]
            #[pyo3(name = "height", signature = (cell))]
            pub fn height<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::height(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::height(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("height wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Swaps filled and empty sites.
            #[pyfunction]
            #[pyo3(name = "invert", signature = (cell))]
            pub fn invert<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::invert(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::invert(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("invert wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Tags each site with its ring distance from the center.
            #[pyfunction]
            #[pyo3(name = "layers", signature = (cell))]
            pub fn layers<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::layers(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::layers(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("layers wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Tags each site with its count of masked neighbors matching the target, wrapping on request.
            #[pyfunction]
            #[pyo3(name = "neighbors", signature = (cell, mask, target, wrap))]
            pub fn neighbors<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, mask: PyTensor, target: u8, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::neighbors(cell, &mask, target, wrap);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::neighbors(cell, &mask, target, wrap);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("neighbors wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Builds a cell from an N-dimensional tensor of types.
            #[pyfunction]
            #[pyo3(name = "new", signature = (types))]
            pub fn new<'py>(py: Python<'py>, types: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(types)?;
                match dim {
                    2 => {
                        let types = types.extract::<PyTensor>()?;
                        let types = types.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::new(types);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let types = types.extract::<PyTensor>()?;
                        let types = types.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::new(types);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("new wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Turns the cell into one of the 24 cube orientations.
            #[pyfunction]
            #[pyo3(name = "orient", signature = (cell, index))]
            pub fn orient<'py>(py: Python<'py>, cell: PyCell3d, index: usize) -> PyResult<Bound<'py, PyAny>> {
                let cell = cell.0;
                let out = mrlyrs::math::cell::models::CellNd::<3>::orient(cell, index);
                (PyCellNd(ok(out)?)).into_bound_py_any(py)
            }

            /// Wraps the cell in count layers of the given value on every side.
            #[pyfunction]
            #[pyo3(name = "pad", signature = (cell, count, value))]
            pub fn pad<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, count: usize, value: u8) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::pad(cell, count, value);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::pad(cell, count, value);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("pad wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Colors each site by its type through the mapping in the given mode.
            #[pyfunction]
            #[pyo3(name = "paint", signature = (cell, mapping, mode, rng=None))]
            pub fn paint<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, mapping: std::collections::HashMap<u8, Vec<PyColor>>, mode: PySerde<mrlyrs::core::Mode>, rng: Option<&mut PyRng>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mapping = mapping.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>())).collect();
                        let mode = mode.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::paint(cell, &mapping, mode, rng.map(|r| &mut r.0));
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mapping = mapping.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>())).collect();
                        let mode = mode.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::paint(cell, &mapping, mode, rng.map(|r| &mut r.0));
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("paint wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Writes the value wherever the tiled mask is nonzero.
            #[pyfunction]
            #[pyo3(name = "perforate", signature = (cell, mask, value))]
            pub fn perforate<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, mask: PyTensor, value: u8) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::perforate(cell, &mask, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::perforate(cell, &mask, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("perforate wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Rotates the cell k quarter turns in the plane.
            ///
            /// Rotates the cell k quarter turns about the given pair of axes.
            #[pyfunction]
            #[pyo3(name = "rotate", signature = (cell, k, axes=None))]
            pub fn rotate<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, k: usize, axes: Option<(usize, usize)>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::rotate(cell, k);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let axes = axes.ok_or_else(|| PyValueError::new_err("a 3d rotate wants axes."))?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::rotate(cell, k, axes);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("rotate wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Repeats the cell into a width-by-height array of copies.
            ///
            /// Repeats the cell into a width-by-height-by-depth array of copies.
            #[pyfunction]
            #[pyo3(name = "tile", signature = (cell, width, height, depth=None))]
            pub fn tile<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, width: usize, height: usize, depth: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::tile(cell, width, height);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let depth = depth.ok_or_else(|| PyValueError::new_err("a 3d tile wants depth."))?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::tile(cell, width, height, depth);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("tile wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Returns the tensor of types.
            #[pyfunction]
            #[pyo3(name = "types", signature = (cell))]
            pub fn types<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::types(&cell);
                        (PyTensor((out).clone())).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::types(&cell);
                        (PyTensor((out).clone())).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("types wants a 2d or 3d argument, got {other}d."))),
                }
            }

            /// Returns the size of the last axis.
            #[pyfunction]
            #[pyo3(name = "width", signature = (cell))]
            pub fn width<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<2>::width(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrlyrs::math::cell::models::CellNd::<3>::width(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!("width wants a 2d or 3d argument, got {other}d."))),
                }
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.cell.models")?;
                m.setattr("__doc__", "The N-dimensional cell and its fixed-dimension aliases.")?;
                m.add_function(wrap_pyfunction!(anti, &m)?)?;
                m.add_function(wrap_pyfunction!(binarize, &m)?)?;
                m.add_function(wrap_pyfunction!(binarize_otsu, &m)?)?;
                m.add_function(wrap_pyfunction!(blur, &m)?)?;
                m.add_function(wrap_pyfunction!(combine, &m)?)?;
                m.add_function(wrap_pyfunction!(counting_dtype, &m)?)?;
                m.add_function(wrap_pyfunction!(depth, &m)?)?;
                m.add_function(wrap_pyfunction!(dtype_for, &m)?)?;
                m.add_function(wrap_pyfunction!(fractal, &m)?)?;
                m.add_function(wrap_pyfunction!(height, &m)?)?;
                m.add_function(wrap_pyfunction!(invert, &m)?)?;
                m.add_function(wrap_pyfunction!(layers, &m)?)?;
                m.add_function(wrap_pyfunction!(neighbors, &m)?)?;
                m.add_function(wrap_pyfunction!(new, &m)?)?;
                m.add_function(wrap_pyfunction!(orient, &m)?)?;
                m.add_function(wrap_pyfunction!(pad, &m)?)?;
                m.add_function(wrap_pyfunction!(paint, &m)?)?;
                m.add_function(wrap_pyfunction!(perforate, &m)?)?;
                m.add_function(wrap_pyfunction!(rotate, &m)?)?;
                m.add_function(wrap_pyfunction!(tile, &m)?)?;
                m.add_function(wrap_pyfunction!(types, &m)?)?;
                m.add_function(wrap_pyfunction!(width, &m)?)?;
                let names: Vec<&str> = vec!["anti", "binarize", "binarize_otsu", "blur", "combine", "counting_dtype", "depth", "dtype_for", "fractal", "height", "invert", "layers", "neighbors", "new", "orient", "pad", "paint", "perforate", "rotate", "tile", "types", "width"];
                m.add("__all__", names)?;
                parent.add("models", &m)?;
                sys.set_item("mrlypy._mrlypy.math.cell.models", &m)?;
                Ok(())
            }
        }

        /// The JSON reading the cell serializers share.
        pub mod serializer {
            use crate::hand::{ok, PyPixels, PySerde, PyTensor};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Reads a triply nested JSON array into layers of byte rows.
            #[pyfunction]
            #[pyo3(name = "byte_cube", signature = (value))]
            pub fn byte_cube<'py>(py: Python<'py>, value: PySerde<serde_json::Value>) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrlyrs::math::cell::serializer::byte_cube(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Reads a nested JSON array into rows of bytes.
            #[pyfunction]
            #[pyo3(name = "byte_grid", signature = (value))]
            pub fn byte_grid<'py>(py: Python<'py>, value: PySerde<serde_json::Value>) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrlyrs::math::cell::serializer::byte_grid(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Reads a nested JSON array into rows of four-channel colors.
            #[pyfunction]
            #[pyo3(name = "color_grid", signature = (value))]
            pub fn color_grid<'py>(py: Python<'py>, value: PySerde<serde_json::Value>) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrlyrs::math::cell::serializer::color_grid(&value);
                ((ok(out)?).into_iter().map(PyPixels).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
            #[pyfunction]
            #[pyo3(name = "count_cube", signature = (value))]
            pub fn count_cube<'py>(py: Python<'py>, value: PySerde<serde_json::Value>) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrlyrs::math::cell::serializer::count_cube(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
            #[pyfunction]
            #[pyo3(name = "count_grid", signature = (value))]
            pub fn count_grid<'py>(py: Python<'py>, value: PySerde<serde_json::Value>) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrlyrs::math::cell::serializer::count_grid(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Parses JSON text into a value tree.
            #[pyfunction]
            #[pyo3(name = "parse", signature = (text))]
            pub fn parse<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::cell::serializer::parse(text);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            /// Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them.
            #[pyfunction]
            #[pyo3(name = "tag_layer", signature = (counts, shape))]
            pub fn tag_layer<'py>(py: Python<'py>, counts: Vec<i64>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::cell::serializer::tag_layer(&counts, shape);
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            /// Returns the types field of the data.
            #[pyfunction]
            #[pyo3(name = "types_field", signature = (data))]
            pub fn types_field<'py>(py: Python<'py>, data: PySerde<serde_json::Value>) -> PyResult<Bound<'py, PyAny>> {
                let data = data.0;
                let out = mrlyrs::math::cell::serializer::types_field(&data);
                (PySerde((ok(out)?).clone())).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.cell.serializer")?;
                m.setattr("__doc__", "The JSON reading the cell serializers share.")?;
                m.add_function(wrap_pyfunction!(byte_cube, &m)?)?;
                m.add_function(wrap_pyfunction!(byte_grid, &m)?)?;
                m.add_function(wrap_pyfunction!(color_grid, &m)?)?;
                m.add_function(wrap_pyfunction!(count_cube, &m)?)?;
                m.add_function(wrap_pyfunction!(count_grid, &m)?)?;
                m.add_function(wrap_pyfunction!(parse, &m)?)?;
                m.add_function(wrap_pyfunction!(tag_layer, &m)?)?;
                m.add_function(wrap_pyfunction!(types_field, &m)?)?;
                let names: Vec<&str> = vec!["byte_cube", "byte_grid", "color_grid", "count_cube", "count_grid", "parse", "tag_layer", "types_field"];
                m.add("__all__", names)?;
                parent.add("serializer", &m)?;
                sys.set_item("mrlypy._mrlypy.math.cell.serializer", &m)?;
                Ok(())
            }
        }

        /// Grows a seed pattern into a cell, deepened to its fractal past level one.
        #[pyfunction]
        #[pyo3(name = "grow", signature = (pattern, level))]
        pub fn grow<'py>(py: Python<'py>, pattern: &Bound<'_, PyAny>, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(pattern)?;
            match dim {
                2 => {
                    let pattern = pattern.extract::<PyTensor>()?;
                    let pattern = pattern.0;
                    let out = mrlyrs::math::cell::grow::<2>(pattern, level);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let pattern = pattern.extract::<PyTensor>()?;
                    let pattern = pattern.0;
                    let out = mrlyrs::math::cell::grow::<3>(pattern, level);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!("grow wants a 2d or 3d argument, got {other}d."))),
            }
        }

        /// Colors the cell through the given mapping and mode, defaulting to the standard palette by type.
        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, custom=None, mode=None, rng=None))]
        pub fn paint<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>, custom: Option<std::collections::HashMap<u8, Vec<PyColor>>>, mode: Option<PySerde<mrlyrs::core::Mode>>, rng: Option<&mut PyRng>) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let custom = custom.map(|x| x.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>())).collect());
                    let mode = mode.map(|x| x.0);
                    let out = mrlyrs::math::cell::paint::<2>(cell, custom.as_ref(), mode, rng.map(|r| &mut r.0));
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let custom = custom.map(|x| x.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>())).collect());
                    let mode = mode.map(|x| x.0);
                    let out = mrlyrs::math::cell::paint::<3>(cell, custom.as_ref(), mode, rng.map(|r| &mut r.0));
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!("paint wants a 2d or 3d argument, got {other}d."))),
            }
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.cell")?;
            m.setattr("__doc__", "The dimension-generic cell and the pipeline the fixed dimensions share.\nThe N-dimensional cell and the pipeline the fixed dimensions share.\n\nA seed pattern grows into a fractal cell here, and `two`, `three` and `six` are that one\npipeline pinned to a dimension: census, geometry, graphs, painting, text and JSON.")?;
            m.add_function(wrap_pyfunction!(grow, &m)?)?;
            m.add_function(wrap_pyfunction!(paint, &m)?)?;
            let names: Vec<&str> = vec!["grow", "paint"];
            m.add("__all__", names)?;
            census::init(py, &m, sys)?;
            geometry::init(py, &m, sys)?;
            models::init(py, &m, sys)?;
            serializer::init(py, &m, sys)?;
            parent.add("cell", &m)?;
            sys.set_item("mrlypy._mrlypy.math.cell", &m)?;
            Ok(())
        }
    }

    /// The closed-form counts: fills, surfaces, hex slices and the carry ladder, without rendering.
    /// The closed-form counts.
    pub mod counts {
        use crate::hand::{ok, PyCode, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The diagonal profile of any tile's power, as a digit polynomial.
        pub mod diagonal {
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.counts.diagonal")?;
                m.setattr("__doc__", "The diagonal profile of any tile's power, as a digit polynomial.")?;
                m.add("WIDEST", mrlyrs::math::counts::diagonal::WIDEST)?;
                let names: Vec<&str> = vec!["WIDEST"];
                m.add("__all__", names)?;
                parent.add("diagonal", &m)?;
                sys.set_item("mrlypy._mrlypy.math.counts.diagonal", &m)?;
                Ok(())
            }
        }

        /// The base-q slice carry automaton: its digit polynomial, its matrix, its ladder and its sign law.
        pub mod ladder {
            use crate::hand::{ok};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// The widest dimension the exact carry arithmetic reaches at the base.
            #[pyfunction]
            #[pyo3(name = "cap", signature = (base))]
            pub fn cap<'py>(py: Python<'py>, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::cap(base);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out.
            #[pyfunction]
            #[pyo3(name = "carry_matrix", signature = (base, dimension))]
            pub fn carry_matrix<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::carry_matrix(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The monic characteristic polynomial of a square integer matrix, highest power first.
            #[pyfunction]
            #[pyo3(name = "characteristic", signature = (rows))]
            pub fn characteristic<'py>(py: Python<'py>, rows: Vec<Vec<i128>>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::characteristic(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The determinant of a square integer matrix, read off its characteristic polynomial.
            #[pyfunction]
            #[pyo3(name = "determinant", signature = (rows))]
            pub fn determinant<'py>(py: Python<'py>, rows: Vec<Vec<i128>>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::determinant(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first.
            #[pyfunction]
            #[pyo3(name = "digit_polynomial", signature = (base, dimension))]
            pub fn digit_polynomial<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::digit_polynomial(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The reflection-even block of the carry matrix, of size `ceil(D/2)`.
            #[pyfunction]
            #[pyo3(name = "even_block", signature = (base, dimension))]
            pub fn even_block<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::even_block(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`.
            #[pyfunction]
            #[pyo3(name = "fill", signature = (base, dimension))]
            pub fn fill<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::fill(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`.
            #[pyfunction]
            #[pyo3(name = "ladder", signature = (base, dimension, levels))]
            pub fn ladder<'py>(py: Python<'py>, base: usize, dimension: usize, levels: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::ladder(base, dimension, levels);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The Perron root of a nonnegative square integer matrix.
            #[pyfunction]
            #[pyo3(name = "perron", signature = (rows))]
            pub fn perron<'py>(py: Python<'py>, rows: Vec<Vec<i128>>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::perron(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers.
            #[pyfunction]
            #[pyo3(name = "sign", signature = (base, dimension))]
            pub fn sign<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::sign(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The Perron root over the modulus of the second eigenvalue, or none where the block is one wide.
            #[pyfunction]
            #[pyo3(name = "spectral_ratio", signature = (base, dimension))]
            pub fn spectral_ratio<'py>(py: Python<'py>, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::spectral_ratio(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The trace of a square integer matrix.
            #[pyfunction]
            #[pyo3(name = "trace", signature = (rows))]
            pub fn trace<'py>(py: Python<'py>, rows: Vec<Vec<i128>>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::ladder::trace(&rows);
                (out).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.counts.ladder")?;
                m.setattr("__doc__", "The base-q slice carry automaton: its digit polynomial, its matrix, its ladder and its sign law.")?;
                m.add_function(wrap_pyfunction!(cap, &m)?)?;
                m.add_function(wrap_pyfunction!(carry_matrix, &m)?)?;
                m.add_function(wrap_pyfunction!(characteristic, &m)?)?;
                m.add_function(wrap_pyfunction!(determinant, &m)?)?;
                m.add_function(wrap_pyfunction!(digit_polynomial, &m)?)?;
                m.add_function(wrap_pyfunction!(even_block, &m)?)?;
                m.add_function(wrap_pyfunction!(fill, &m)?)?;
                m.add_function(wrap_pyfunction!(ladder, &m)?)?;
                m.add_function(wrap_pyfunction!(perron, &m)?)?;
                m.add_function(wrap_pyfunction!(sign, &m)?)?;
                m.add_function(wrap_pyfunction!(spectral_ratio, &m)?)?;
                m.add_function(wrap_pyfunction!(trace, &m)?)?;
                let names: Vec<&str> = vec!["cap", "carry_matrix", "characteristic", "determinant", "digit_polynomial", "even_block", "fill", "ladder", "perron", "sign", "spectral_ratio", "trace"];
                m.add("__all__", names)?;
                parent.add("ladder", &m)?;
                sys.set_item("mrlypy._mrlypy.math.counts.ladder", &m)?;
                Ok(())
            }
        }

        /// The closed-form triangle, node and edge counts of hex slices.
        pub mod six {
            use crate::hand::{ok};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Returns the triangles of the full hexagon with side number to the level.
            #[pyfunction]
            #[pyo3(name = "grid_triangles", signature = (number, level))]
            pub fn grid_triangles<'py>(py: Python<'py>, number: usize, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::grid_triangles(number, level);
                (out).into_bound_py_any(py)
            }

            /// Returns the boundary edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_boundary", signature = (number))]
            pub fn solid_slice_boundary<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::solid_slice_boundary(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the core edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_core_edges", signature = (number))]
            pub fn solid_slice_core_edges<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::solid_slice_core_edges(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the core node count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_core_nodes", signature = (number))]
            pub fn solid_slice_core_nodes<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::solid_slice_core_nodes(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the distinct triangle-edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_edges", signature = (number))]
            pub fn solid_slice_edges<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::solid_slice_edges(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the interior edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_interior", signature = (number))]
            pub fn solid_slice_interior<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::solid_slice_interior(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the triangle count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_triangles", signature = (number))]
            pub fn solid_slice_triangles<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::solid_slice_triangles(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the vertex count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_vertices", signature = (number))]
            pub fn solid_slice_vertices<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::six::solid_slice_vertices(number);
                (ok(out)?).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.counts.six")?;
                m.setattr("__doc__", "The closed-form triangle, node and edge counts of hex slices.")?;
                m.add_function(wrap_pyfunction!(grid_triangles, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_boundary, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_core_edges, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_core_nodes, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_edges, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_interior, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_triangles, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_vertices, &m)?)?;
                let names: Vec<&str> = vec!["grid_triangles", "solid_slice_boundary", "solid_slice_core_edges", "solid_slice_core_nodes", "solid_slice_edges", "solid_slice_interior", "solid_slice_triangles", "solid_slice_vertices"];
                m.add("__all__", names)?;
                parent.add("six", &m)?;
                sys.set_item("mrlypy._mrlypy.math.counts.six", &m)?;
                Ok(())
            }
        }

        /// The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions.
        #[pyclass(name = "Exposure", module = "mrlypy.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Exposure(pub mrlyrs::math::counts::Exposure);

        #[pymethods]
        impl Exposure {
            /// The filled cells of the tile.
            #[getter]
            #[pyo3(name = "occupancy")]
            pub fn occupancy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.occupancy;
                (value).into_bound_py_any(py)
            }
            /// The exposed faces of the tile.
            #[getter]
            #[pyo3(name = "exposed")]
            pub fn exposed<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.exposed;
                (value).into_bound_py_any(py)
            }
            /// Per axis, the adjacent filled pairs and the spanning positions.
            #[getter]
            #[pyo3(name = "axes")]
            pub fn axes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.axes.clone();
                (value).into_bound_py_any(py)
            }
            /// Returns the exposed faces of the level-fold Kronecker power, or none past a u128.
            #[pyo3(name = "at", signature = (level))]
            pub fn at<'py>(&self, py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::Exposure::at(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Folds the counts from the filled residue corners at a side number, without rendering the tile.
            #[staticmethod]
            #[pyo3(name = "from_corners", signature = (filled, number, dimension, base))]
            pub fn from_corners<'py>(py: Python<'py>, filled: Vec<Vec<u8>>, number: usize, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::Exposure::from_corners(&filled, number, dimension, base);
                (crate::gen::math::counts::Exposure(out)).into_bound_py_any(py)
            }
            /// Reads the counts off a rendered tile.
            #[staticmethod]
            #[pyo3(name = "of_tile", signature = (tile))]
            pub fn of_tile<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
                let tile = tile.0;
                let out = mrlyrs::math::counts::Exposure::of_tile(&tile);
                (crate::gen::math::counts::Exposure(out)).into_bound_py_any(py)
            }
            /// Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys.
            #[pyo3(name = "recurrence", signature = ())]
            pub fn recurrence<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::counts::Exposure::recurrence(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1.
        #[pyfunction]
        #[pyo3(name = "centered_hexagonal", signature = (m))]
        pub fn centered_hexagonal<'py>(py: Python<'py>, m: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::counts::centered_hexagonal(m);
            (out).into_bound_py_any(py)
        }

        /// Returns the filled triangle count of the code's cut section at the given level, without rendering it.
        #[pyfunction]
        #[pyo3(name = "cut_fills", signature = (code, number, level))]
        pub fn cut_fills<'py>(py: Python<'py>, code: PyCode, number: usize, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::cut_fills(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the empty triangle count of the code's cut section at the given level.
        #[pyfunction]
        #[pyo3(name = "cut_voids", signature = (code, number, level))]
        pub fn cut_voids<'py>(py: Python<'py>, code: PyCode, number: usize, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::cut_voids(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the code's fractal dimension, the log of its one-level fill over the log of number.
        #[pyfunction]
        #[pyo3(name = "dimension", signature = (code, number, base_dimension, base))]
        pub fn dimension<'py>(py: Python<'py>, code: PyCode, number: usize, base_dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::dimension(code, number, base_dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term
        /// recurrence over four powers, or none past a u128.
        #[pyfunction]
        #[pyo3(name = "edges_of_tile", signature = (tile, level))]
        pub fn edges_of_tile<'py>(py: Python<'py>, tile: PyTensor, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrlyrs::math::counts::edges_of_tile(&tile, level);
            (out).into_bound_py_any(py)
        }

        /// Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners.
        #[pyfunction]
        #[pyo3(name = "exposure", signature = (code, number, dimension, level, base))]
        pub fn exposure<'py>(py: Python<'py>, code: PyCode, number: usize, dimension: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::exposure(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128.
        #[pyfunction]
        #[pyo3(name = "exposure_of_tile", signature = (tile, level))]
        pub fn exposure_of_tile<'py>(py: Python<'py>, tile: PyTensor, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrlyrs::math::counts::exposure_of_tile(&tile, level);
            (out).into_bound_py_any(py)
        }

        /// Returns the coefficients of the recurrence the tile's exposure obeys.
        #[pyfunction]
        #[pyo3(name = "exposure_recurrence", signature = (tile))]
        pub fn exposure_recurrence<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrlyrs::math::counts::exposure_recurrence(&tile);
            (out).into_bound_py_any(py)
        }

        /// Returns the filled cell count of the code's fractal at the given level, without rendering it.
        #[pyfunction]
        #[pyo3(name = "fill", signature = (code, number, dimension, level, base))]
        pub fn fill<'py>(py: Python<'py>, code: PyCode, number: usize, dimension: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::fill(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Sums each corner's position products into a base fill and raises it to the level.
        #[pyfunction]
        #[pyo3(name = "fill_from_corners", signature = (filled, number, _dimension, level, base))]
        pub fn fill_from_corners<'py>(py: Python<'py>, filled: Vec<Vec<u8>>, number: usize, _dimension: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::counts::fill_from_corners(&filled, number, _dimension, level, base);
            (out).into_bound_py_any(py)
        }

        /// Returns the total cells of the grid, number to the dimension, to the level.
        #[pyfunction]
        #[pyo3(name = "grid", signature = (number, dimension, level))]
        pub fn grid<'py>(py: Python<'py>, number: usize, dimension: usize, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::counts::grid(number, dimension, level);
            (out).into_bound_py_any(py)
        }

        /// Returns the fill ratio the code walks toward as the side number grows, reduced.
        #[pyfunction]
        #[pyo3(name = "limit", signature = (code, dimension, level, base))]
        pub fn limit<'py>(py: Python<'py>, code: PyCode, dimension: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::limit(code, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled.
        #[pyfunction]
        #[pyo3(name = "pairs", signature = (tile))]
        pub fn pairs<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrlyrs::math::counts::pairs(&tile);
            (out).into_bound_py_any(py)
        }

        /// Counts the indices below number that equal residue modulo base.
        #[pyfunction]
        #[pyo3(name = "positions", signature = (residue, number, base))]
        pub fn positions<'py>(py: Python<'py>, residue: usize, number: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::counts::positions(residue, number, base);
            (out).into_bound_py_any(py)
        }

        /// Returns the filled triangle count of the code's pro projection at the given level, without rendering it.
        #[pyfunction]
        #[pyo3(name = "pro_fills", signature = (code, number, level))]
        pub fn pro_fills<'py>(py: Python<'py>, code: PyCode, number: usize, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::pro_fills(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the empty triangle count of the code's pro projection at the given level.
        #[pyfunction]
        #[pyo3(name = "pro_voids", signature = (code, number, level))]
        pub fn pro_voids<'py>(py: Python<'py>, code: PyCode, number: usize, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::pro_voids(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`.
        #[pyfunction]
        #[pyo3(name = "profile_of_tile", signature = (tile, level))]
        pub fn profile_of_tile<'py>(py: Python<'py>, tile: PyTensor, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrlyrs::math::counts::profile_of_tile(&tile, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the filled fraction of the grid, or 0.0 for an empty grid.
        #[pyfunction]
        #[pyo3(name = "ratio", signature = (code, number, dimension, level, base))]
        pub fn ratio<'py>(py: Python<'py>, code: PyCode, number: usize, dimension: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::ratio(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the exact filled fraction as a fraction of fill over grid, reduced.
        #[pyfunction]
        #[pyo3(name = "rational", signature = (code, number, dimension, level, base))]
        pub fn rational<'py>(py: Python<'py>, code: PyCode, number: usize, dimension: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::rational(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the exposed face count of the code's 3D fractal at the given level.
        #[pyfunction]
        #[pyo3(name = "surface", signature = (code, number, level, base))]
        pub fn surface<'py>(py: Python<'py>, code: PyCode, number: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::surface(code, number, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the empty cell count, grid minus fill.
        #[pyfunction]
        #[pyo3(name = "void", signature = (code, number, dimension, level, base))]
        pub fn void<'py>(py: Python<'py>, code: PyCode, number: usize, dimension: usize, level: u32, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::counts::void(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.counts")?;
            m.setattr("__doc__", "The closed-form counts: fills, surfaces, hex slices and the carry ladder, without rendering.\nThe closed-form counts.\n\nEvery number the censuses reach by walking a built cell, this folder reaches by formula:\nfills, grids, exposed surfaces, hex slices and the base-q carry ladder.")?;
            m.add_class::<Exposure>()?;
            m.add_function(wrap_pyfunction!(centered_hexagonal, &m)?)?;
            m.add_function(wrap_pyfunction!(cut_fills, &m)?)?;
            m.add_function(wrap_pyfunction!(cut_voids, &m)?)?;
            m.add_function(wrap_pyfunction!(dimension, &m)?)?;
            m.add_function(wrap_pyfunction!(edges_of_tile, &m)?)?;
            m.add_function(wrap_pyfunction!(exposure, &m)?)?;
            m.add_function(wrap_pyfunction!(exposure_of_tile, &m)?)?;
            m.add_function(wrap_pyfunction!(exposure_recurrence, &m)?)?;
            m.add_function(wrap_pyfunction!(fill, &m)?)?;
            m.add_function(wrap_pyfunction!(fill_from_corners, &m)?)?;
            m.add_function(wrap_pyfunction!(grid, &m)?)?;
            m.add_function(wrap_pyfunction!(limit, &m)?)?;
            m.add_function(wrap_pyfunction!(pairs, &m)?)?;
            m.add_function(wrap_pyfunction!(positions, &m)?)?;
            m.add_function(wrap_pyfunction!(pro_fills, &m)?)?;
            m.add_function(wrap_pyfunction!(pro_voids, &m)?)?;
            m.add_function(wrap_pyfunction!(profile_of_tile, &m)?)?;
            m.add_function(wrap_pyfunction!(ratio, &m)?)?;
            m.add_function(wrap_pyfunction!(rational, &m)?)?;
            m.add_function(wrap_pyfunction!(surface, &m)?)?;
            m.add_function(wrap_pyfunction!(void, &m)?)?;
            let names: Vec<&str> = vec!["centered_hexagonal", "cut_fills", "cut_voids", "dimension", "edges_of_tile", "exposure", "exposure_of_tile", "exposure_recurrence", "fill", "fill_from_corners", "grid", "limit", "pairs", "positions", "pro_fills", "pro_voids", "profile_of_tile", "ratio", "rational", "surface", "void", "Exposure"];
            m.add("__all__", names)?;
            diagonal::init(py, &m, sys)?;
            ladder::init(py, &m, sys)?;
            six::init(py, &m, sys)?;
            parent.add("counts", &m)?;
            sys.set_item("mrlypy._mrlypy.math.counts", &m)?;
            Ok(())
        }
    }

    /// The spatial network: its nodes, branches, extraction and census.
    /// The spatial network.
    pub mod graph {
        use crate::hand::{ok, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A force-directed layout: every node repels every other, every branch pulls its ends together, and a cooling cap on the move per tick lets the lattice settle.
        #[pyclass(name = "Layout", module = "mrlypy.math.graph", from_py_object)]
        #[derive(Clone)]
        pub struct Layout(pub mrlyrs::math::graph::Layout);

        #[pymethods]
        impl Layout {
            /// Starts a layout from flat positions, `dim` floats per node, and the branch pairs.
            #[new]
            #[pyo3(signature = (positions, branches, dim, seed))]
            pub fn __new__(positions: Vec<f64>, branches: Vec<(usize, usize)>, dim: usize, seed: u64) -> PyResult<Self> {
                let out = mrlyrs::math::graph::Layout::new(&positions, &branches, dim, seed);
                Ok(Self(ok(out)?))
            }
            /// Returns the mean net force per node in units of `k` after the last tick.
            #[pyo3(name = "energy", signature = ())]
            pub fn energy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::energy(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Starts a layout from a network's own positions and branches.
            #[staticmethod]
            #[pyo3(name = "from_network", signature = (network, seed))]
            pub fn from_network<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>, seed: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::from_network(&network.0, seed);
                (crate::gen::math::graph::Layout(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the ideal branch length `k`.
            #[pyo3(name = "ideal", signature = ())]
            pub fn ideal<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::ideal(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the mean distance a node moved in the last tick.
            #[pyo3(name = "moved", signature = ())]
            pub fn moved<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::moved(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Starts a layout from flat positions, `dim` floats per node, and the branch pairs.
            #[staticmethod]
            #[pyo3(name = "new", signature = (positions, branches, dim, seed))]
            pub fn new_<'py>(py: Python<'py>, positions: Vec<f64>, branches: Vec<(usize, usize)>, dim: usize, seed: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::new(&positions, &branches, dim, seed);
                (crate::gen::math::graph::Layout(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the node count.
            #[pyo3(name = "nodes", signature = ())]
            pub fn nodes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::nodes(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the positions, `dim` floats per node.
            #[pyo3(name = "positions", signature = ())]
            pub fn positions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::positions(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Runs the ticks and returns the energy left: the mean net force per node in units of `k`.
            #[pyo3(name = "step", signature = (ticks))]
            pub fn step<'py>(&mut self, py: Python<'py>, ticks: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::step(&mut self.0, ticks);
                (out).into_bound_py_any(py)
            }
            /// Returns the cap on one node's move in the next tick.
            #[pyo3(name = "temperature", signature = ())]
            pub fn temperature<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::temperature(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the ticks stepped so far.
            #[pyo3(name = "ticks", signature = ())]
            pub fn ticks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Layout::ticks(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        /// A spatial graph of nodes and branches.
        #[pyclass(name = "Network", module = "mrlypy.math.graph", from_py_object)]
        #[derive(Clone)]
        pub struct Network(pub mrlyrs::math::graph::Network);

        #[pymethods]
        impl Network {
            /// Builds an empty network of the given dimension.
            #[new]
            #[pyo3(signature = (dim))]
            pub fn __new__(dim: usize) -> PyResult<Self> {
                let out = mrlyrs::math::graph::Network::new(dim);
                Ok(Self(out))
            }
            /// The dimension every position must match.
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            /// The nodes in insertion order.
            #[getter]
            #[pyo3(name = "nodes")]
            pub fn nodes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.nodes.clone();
                ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// The branches in insertion order.
            #[getter]
            #[pyo3(name = "branches")]
            pub fn branches<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.branches.clone();
                ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Appends a branch between two node indices.
            #[pyo3(name = "add_branch", signature = (parent, child, radius))]
            pub fn add_branch<'py>(&mut self, py: Python<'py>, parent: usize, child: usize, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Network::add_branch(&mut self.0, parent, child, radius);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Appends a node at the position and returns its index.
            #[pyo3(name = "add_node", signature = (position))]
            pub fn add_node<'py>(&mut self, py: Python<'py>, position: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Network::add_node(&mut self.0, position);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the undirected neighbor lists of every node.
            #[pyo3(name = "adjacency", signature = ())]
            pub fn adjacency<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Network::adjacency(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns each node's branch count, indexed like the node list.
            #[pyo3(name = "degree", signature = ())]
            pub fn degree<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Network::degree(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Builds an empty network of the given dimension.
            #[staticmethod]
            #[pyo3(name = "new", signature = (dim))]
            pub fn new_<'py>(py: Python<'py>, dim: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::graph::Network::new(dim);
                (crate::gen::math::graph::Network(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Takes the full census of a network.
        #[pyfunction]
        #[pyo3(name = "census", signature = (network))]
        pub fn census<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::census(&network.0);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts the connected components of the network.
        #[pyfunction]
        #[pyo3(name = "components", signature = (network))]
        pub fn components<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::components(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Extracts the network of filled sites joined to their axis neighbors.
        #[pyfunction]
        #[pyo3(name = "core_graph", signature = (grid))]
        pub fn core_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrlyrs::math::graph::core_graph(&grid);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Extracts the network of corners and edges outlining every filled site.
        #[pyfunction]
        #[pyo3(name = "edge_graph", signature = (grid))]
        pub fn edge_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrlyrs::math::graph::edge_graph(&grid);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample.
        #[pyfunction]
        #[pyo3(name = "fractal_dimension", signature = (network, samples))]
        pub fn fractal_dimension<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>, samples: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::fractal_dimension(&network.0, samples);
            (out).into_bound_py_any(py)
        }

        /// Counts the nodes of degree three or more.
        #[pyfunction]
        #[pyo3(name = "junctions", signature = (network))]
        pub fn junctions<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::junctions(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Extracts the largest connected piece as a network of its own, branches re-indexed.
        #[pyfunction]
        #[pyo3(name = "largest_component", signature = (network))]
        pub fn largest_component<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::largest_component(&network.0);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Tags every node by its degree, indexed like the node list.
        #[pyfunction]
        #[pyo3(name = "roles", signature = (network))]
        pub fn roles<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::roles(&network.0);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Counts the nodes of degree one.
        #[pyfunction]
        #[pyo3(name = "tips", signature = (network))]
        pub fn tips<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::tips(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Sums the straight-line lengths of every branch.
        #[pyfunction]
        #[pyo3(name = "total_length", signature = (network))]
        pub fn total_length<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::graph::total_length(&network.0);
            (out).into_bound_py_any(py)
        }

        /// Extracts the core graph of the inverted grid, joining empty sites instead.
        #[pyfunction]
        #[pyo3(name = "tunnel_graph", signature = (grid))]
        pub fn tunnel_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrlyrs::math::graph::tunnel_graph(&grid);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.graph")?;
            m.setattr("__doc__", "The spatial network: its nodes, branches, extraction and census.\nThe spatial network.\n\nA grid lifts into nodes and branches, as a core, an edge or a tunnel network, and the census\nreads its roles, tips, junctions and components back.")?;
            m.add_class::<Layout>()?;
            m.add_class::<Network>()?;
            m.add_function(wrap_pyfunction!(census, &m)?)?;
            m.add_function(wrap_pyfunction!(components, &m)?)?;
            m.add_function(wrap_pyfunction!(core_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(edge_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(fractal_dimension, &m)?)?;
            m.add_function(wrap_pyfunction!(junctions, &m)?)?;
            m.add_function(wrap_pyfunction!(largest_component, &m)?)?;
            m.add_function(wrap_pyfunction!(roles, &m)?)?;
            m.add_function(wrap_pyfunction!(tips, &m)?)?;
            m.add_function(wrap_pyfunction!(total_length, &m)?)?;
            m.add_function(wrap_pyfunction!(tunnel_graph, &m)?)?;
            let names: Vec<&str> = vec!["census", "components", "core_graph", "edge_graph", "fractal_dimension", "junctions", "largest_component", "roles", "tips", "total_length", "tunnel_graph", "Layout", "Network"];
            m.add("__all__", names)?;
            parent.add("graph", &m)?;
            sys.set_item("mrlypy._mrlypy.math.graph", &m)?;
            Ok(())
        }
    }

    /// The moire fields layered from sampled designs.
    /// The moire fields.
    pub mod moire {
        use crate::hand::{ok, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The exact correlations of flat carpet layers, and the prime detector they make.
        pub mod pairs {
            use crate::hand::{ok, PySerde};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid.
            #[pyfunction]
            #[pyo3(name = "correlation", signature = (m, n))]
            pub fn correlation<'py>(py: Python<'py>, m: usize, n: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::pairs::correlation(m, n);
                (out).into_bound_py_any(py)
            }

            /// Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated.
            #[pyfunction]
            #[pyo3(name = "sampled", signature = (m, n))]
            pub fn sampled<'py>(py: Python<'py>, m: usize, n: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::pairs::sampled(m, n);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Puts an odd scale of three or more on trial against every earlier odd scale.
            #[pyfunction]
            #[pyo3(name = "witness", signature = (scale))]
            pub fn witness<'py>(py: Python<'py>, scale: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::pairs::witness(scale);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.moire.pairs")?;
                m.setattr("__doc__", "The exact correlations of flat carpet layers, and the prime detector they make.")?;
                m.add_function(wrap_pyfunction!(correlation, &m)?)?;
                m.add_function(wrap_pyfunction!(sampled, &m)?)?;
                m.add_function(wrap_pyfunction!(witness, &m)?)?;
                let names: Vec<&str> = vec!["correlation", "sampled", "witness"];
                m.add("__all__", names)?;
                parent.add("pairs", &m)?;
                sys.set_item("mrlypy._mrlypy.math.moire.pairs", &m)?;
                Ok(())
            }
        }

        /// The lattice coordinates and code-membership tests behind the layers.
        pub mod sample {
            use crate::hand::{ok, PySerde};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// Returns the two lattice coordinates of each pixel centre along a row.
            #[pyfunction]
            #[pyo3(name = "axes", signature = (size, lattice, row))]
            pub fn axes<'py>(py: Python<'py>, size: usize, lattice: PySerde<mrlyrs::math::moire::Lattice>, row: usize) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::math::moire::sample::axes(size, lattice, row);
                (out).into_bound_py_any(py)
            }

            /// Unpacks a code into its residue-corner truth table.
            #[pyfunction]
            #[pyo3(name = "membership", signature = (code, base, dimension))]
            pub fn membership<'py>(py: Python<'py>, code: u128, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::sample::membership(code, base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Folds residues into a base-q index of the truth table.
            #[pyfunction]
            #[pyo3(name = "pack", signature = (residues, base))]
            pub fn pack<'py>(py: Python<'py>, residues: Vec<usize>, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::sample::pack(&residues, base);
                (out).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.moire.sample")?;
                m.setattr("__doc__", "The lattice coordinates and code-membership tests behind the layers.")?;
                m.add_function(wrap_pyfunction!(axes, &m)?)?;
                m.add_function(wrap_pyfunction!(membership, &m)?)?;
                m.add_function(wrap_pyfunction!(pack, &m)?)?;
                let names: Vec<&str> = vec!["axes", "membership", "pack"];
                m.add("__all__", names)?;
                parent.add("sample", &m)?;
                sys.set_item("mrlypy._mrlypy.math.moire.sample", &m)?;
                Ok(())
            }
        }

        /// A square grid of f32 samples.
        #[pyclass(name = "Field", module = "mrlypy.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Field(pub mrlyrs::math::moire::Field);

        #[pymethods]
        impl Field {
            /// Builds a zeroed field of the given side.
            #[new]
            #[pyo3(signature = (size))]
            pub fn __new__(size: usize) -> PyResult<Self> {
                let out = mrlyrs::math::moire::Field::new(size);
                Ok(Self(out))
            }
            /// The samples in row-major order.
            #[getter]
            #[pyo3(name = "data")]
            pub fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.data.clone();
                (value).into_bound_py_any(py)
            }
            /// The side length in samples.
            #[getter]
            #[pyo3(name = "size")]
            pub fn size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.size;
                (value).into_bound_py_any(py)
            }
            /// Returns the samples widened to f64.
            #[pyo3(name = "as_f64", signature = ())]
            pub fn as_f64<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Field::as_f64(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Wraps row-major samples of the given side.
            #[staticmethod]
            #[pyo3(name = "from_data", signature = (data, size))]
            pub fn from_data<'py>(py: Python<'py>, data: Vec<f32>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Field::from_data(data, size);
                (crate::gen::math::moire::Field(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the largest sample.
            #[pyo3(name = "max", signature = ())]
            pub fn max<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Field::max(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the mean sample, or zero for an empty field.
            #[pyo3(name = "mean", signature = ())]
            pub fn mean<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Field::mean(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the smallest sample.
            #[pyo3(name = "min", signature = ())]
            pub fn min<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Field::min(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a zeroed field of the given side.
            #[staticmethod]
            #[pyo3(name = "new", signature = (size))]
            pub fn new_<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Field::new(size);
                (crate::gen::math::moire::Field(out)).into_bound_py_any(py)
            }
            /// Returns the samples scaled into 0..1, symmetric about zero on request.
            #[pyo3(name = "normalized", signature = (symmetric))]
            pub fn normalized<'py>(&self, py: Python<'py>, symmetric: bool) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Field::normalized(&self.0, symmetric);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// One named moire recipe: the design, the scales it stacks and the lattice it samples.
        #[pyclass(name = "Preset", module = "mrlypy.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Preset(pub mrlyrs::math::moire::Preset);

        #[pymethods]
        impl Preset {
            /// The name the recipe answers to.
            #[getter]
            #[pyo3(name = "name")]
            pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.name;
                ((value).to_string()).into_bound_py_any(py)
            }
            /// The design sampled at every scale.
            #[getter]
            #[pyo3(name = "spec")]
            pub fn spec<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.spec;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The side numbers stacked.
            #[getter]
            #[pyo3(name = "numbers")]
            pub fn numbers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.numbers.clone();
                (value).into_bound_py_any(py)
            }
            /// The way the layers merge.
            #[getter]
            #[pyo3(name = "combine")]
            pub fn combine<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.combine;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The fractal depth of each layer.
            #[getter]
            #[pyo3(name = "level")]
            pub fn level<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.level;
                (value).into_bound_py_any(py)
            }
            /// The lattice the layers are sampled on.
            #[getter]
            #[pyo3(name = "lattice")]
            pub fn lattice<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.lattice;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The carpet stack: every base-three corner but the centre, summed over odd scales.
            #[staticmethod]
            #[pyo3(name = "carpet", signature = (limit))]
            pub fn carpet<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Preset::carpet(limit);
                (crate::gen::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// Samples the preset into a square field of the given side.
            #[pyo3(name = "field", signature = (size))]
            pub fn field<'py>(&self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Preset::field(&self.0, size);
                (crate::gen::math::moire::Field(ok(out)?)).into_bound_py_any(py)
            }
            /// The parity heatmap: odd scales of the low corner summed on the square lattice.
            #[staticmethod]
            #[pyo3(name = "heatmap", signature = (limit))]
            pub fn heatmap<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Preset::heatmap(limit);
                (crate::gen::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// The hive: the parity heatmap sampled on the hexagonal lattice.
            #[staticmethod]
            #[pyo3(name = "hive", signature = (limit))]
            pub fn hive<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Preset::hive(limit);
                (crate::gen::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// The parity weave: the same odd scales folded to their parity instead of summed.
            #[staticmethod]
            #[pyo3(name = "weave", signature = (limit))]
            pub fn weave<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Preset::weave(limit);
                (crate::gen::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// A cubic grid of f32 samples, x-major.
        #[pyclass(name = "Volume", module = "mrlypy.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Volume(pub mrlyrs::math::moire::Volume);

        #[pymethods]
        impl Volume {
            /// Builds a zeroed volume of the side.
            #[new]
            #[pyo3(signature = (size))]
            pub fn __new__(size: usize) -> PyResult<Self> {
                let out = mrlyrs::math::moire::Volume::new(size);
                Ok(Self(out))
            }
            /// The samples, x-major, then y, then z.
            #[getter]
            #[pyo3(name = "data")]
            pub fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.data.clone();
                (value).into_bound_py_any(py)
            }
            /// The side in samples.
            #[getter]
            #[pyo3(name = "size")]
            pub fn size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.size;
                (value).into_bound_py_any(py)
            }
            /// Reads the sample at a voxel.
            #[pyo3(name = "at", signature = (x, y, z))]
            pub fn at<'py>(&self, py: Python<'py>, x: usize, y: usize, z: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::at(&self.0, x, y, z);
                (out).into_bound_py_any(py)
            }
            /// Counts the samples at or above the level.
            #[pyo3(name = "count", signature = (level))]
            pub fn count<'py>(&self, py: Python<'py>, level: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::count(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Wraps x-major samples of the side.
            #[staticmethod]
            #[pyo3(name = "from_data", signature = (data, size))]
            pub fn from_data<'py>(py: Python<'py>, data: Vec<f32>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::from_data(data, size);
                (crate::gen::math::moire::Volume(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the largest sample.
            #[pyo3(name = "max", signature = ())]
            pub fn max<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::max(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the smallest sample.
            #[pyo3(name = "min", signature = ())]
            pub fn min<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::min(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a zeroed volume of the side.
            #[staticmethod]
            #[pyo3(name = "new", signature = (size))]
            pub fn new_<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::new(size);
                (crate::gen::math::moire::Volume(out)).into_bound_py_any(py)
            }
            /// Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube.
            #[pyo3(name = "plane", signature = (frame, out))]
            pub fn plane<'py>(&self, py: Python<'py>, frame: PySerde<mrlyrs::math::moire::Frame>, out: usize) -> PyResult<Bound<'py, PyAny>> {
                let frame = frame.0;
                let out = mrlyrs::math::moire::Volume::plane(&self.0, &frame, out);
                (out).into_bound_py_any(py)
            }
            /// Reads the voxel a point of the unit cube falls in, or zero outside it.
            #[pyo3(name = "sample", signature = (p))]
            pub fn sample<'py>(&self, py: Python<'py>, p: [f64; 3]) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::sample(&self.0, p);
                (out).into_bound_py_any(py)
            }
            /// Thresholds into a byte tensor: one where a sample reaches the level, zero below.
            #[pyo3(name = "solid", signature = (level))]
            pub fn solid<'py>(&self, py: Python<'py>, level: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Volume::solid(&self.0, level);
                (PyTensor(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// The recipe for one moire layer.
        #[pyclass(name = "Layer", module = "mrlypy.math.moire", skip_from_py_object)]
        pub struct Layer;

        #[pymethods]
        impl Layer {
            /// Builds a layer at level 1 on a 512-pixel square lattice.
            #[staticmethod]
            #[pyo3(name = "new", signature = (spec, number))]
            pub fn new_<'py>(py: Python<'py>, spec: PySerde<mrlyrs::math::moire::Spec>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let spec = spec.0;
                let out = mrlyrs::math::moire::Layer::new(spec, number);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// The identity of a design: its code, base and dimension.
        #[pyclass(name = "Spec", module = "mrlypy.math.moire", skip_from_py_object)]
        pub struct Spec;

        #[pymethods]
        impl Spec {
            /// Builds a spec from a code, base and dimension.
            #[staticmethod]
            #[pyo3(name = "new", signature = (code, base, dimension))]
            pub fn new_<'py>(py: Python<'py>, code: u128, base: usize, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::moire::Spec::new(code, base, dimension);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// Returns every preset stacked up to the given scale.
        #[pyfunction]
        #[pyo3(name = "all", signature = (limit))]
        pub fn all<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::moire::all(limit);
            ((out).into_iter().map(crate::gen::math::moire::Preset).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal.
        #[pyfunction]
        #[pyo3(name = "frame", signature = (normal, offset))]
        pub fn frame<'py>(py: Python<'py>, normal: [f64; 3], offset: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::moire::frame(normal, offset);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Samples a design over the pixel grid into a boolean mask.
        #[pyfunction]
        #[pyo3(name = "layer", signature = (params))]
        pub fn layer<'py>(py: Python<'py>, params: PySerde<mrlyrs::math::moire::Layer>) -> PyResult<Bound<'py, PyAny>> {
            let params = params.0;
            let out = mrlyrs::math::moire::layer(&params);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the preset the name picks.
        #[pyfunction]
        #[pyo3(name = "named", signature = (name, limit))]
        pub fn named<'py>(py: Python<'py>, name: &str, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::moire::named(name, limit);
            (crate::gen::math::moire::Preset(ok(out)?)).into_bound_py_any(py)
        }

        /// Quantizes a field into colored levels and encodes PNG bytes.
        #[pyfunction]
        #[pyo3(name = "render", signature = (field, colorizer, levels, symmetric, invert, scale))]
        pub fn render<'py>(py: Python<'py>, field: PyRef<'_, crate::gen::math::moire::Field>, colorizer: PySerde<mrlyrs::core::Colorizer>, levels: usize, symmetric: bool, invert: bool, scale: usize) -> PyResult<Bound<'py, PyAny>> {
            let colorizer = colorizer.0;
            let out = mrlyrs::math::moire::render(&field.0, &colorizer, levels, symmetric, invert, scale);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Layers one design at several side numbers into a field under the chosen combine.
        #[pyfunction]
        #[pyo3(name = "stack", signature = (spec, numbers, combine, level, lattice, size, slices))]
        pub fn stack<'py>(py: Python<'py>, spec: PySerde<mrlyrs::math::moire::Spec>, numbers: Vec<usize>, combine: PySerde<mrlyrs::math::moire::Combine>, level: usize, lattice: PySerde<mrlyrs::math::moire::Lattice>, size: usize, slices: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let spec = spec.0;
            let combine = combine.0;
            let lattice = lattice.0;
            let out = mrlyrs::math::moire::stack(spec, &numbers, combine, level, lattice, size, &slices);
            (crate::gen::math::moire::Field(ok(out)?)).into_bound_py_any(py)
        }

        /// Sums layers of several designs at one side number into a field.
        #[pyfunction]
        #[pyo3(name = "stack_codes", signature = (specs, number, level, lattice, size, slices))]
        pub fn stack_codes<'py>(py: Python<'py>, specs: Vec<PySerde<mrlyrs::math::moire::Spec>>, number: usize, level: usize, lattice: PySerde<mrlyrs::math::moire::Lattice>, size: usize, slices: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let specs = specs.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let lattice = lattice.0;
            let out = mrlyrs::math::moire::stack_codes(&specs, number, level, lattice, size, &slices);
            (crate::gen::math::moire::Field(ok(out)?)).into_bound_py_any(py)
        }

        /// Layers one cube design at several side numbers into a volume under the chosen combine.
        #[pyfunction]
        #[pyo3(name = "volume", signature = (spec, numbers, combine, level, size))]
        pub fn volume<'py>(py: Python<'py>, spec: PySerde<mrlyrs::math::moire::Spec>, numbers: Vec<usize>, combine: PySerde<mrlyrs::math::moire::Combine>, level: usize, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let spec = spec.0;
            let combine = combine.0;
            let out = mrlyrs::math::moire::volume(spec, &numbers, combine, level, size);
            (crate::gen::math::moire::Volume(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.moire")?;
            m.setattr("__doc__", "The moire fields layered from sampled designs.\nThe moire fields.\n\nOne design sampled at many scales and stacked makes an interference pattern; the layers, their\ncombination, the volume they cut and the PNG they render live here.")?;
            m.add_class::<Field>()?;
            m.add_class::<Preset>()?;
            m.add_class::<Volume>()?;
            m.add_class::<Layer>()?;
            m.add_class::<Spec>()?;
            m.add_function(wrap_pyfunction!(all, &m)?)?;
            m.add_function(wrap_pyfunction!(frame, &m)?)?;
            m.add_function(wrap_pyfunction!(layer, &m)?)?;
            m.add_function(wrap_pyfunction!(named, &m)?)?;
            m.add_function(wrap_pyfunction!(render, &m)?)?;
            m.add_function(wrap_pyfunction!(stack, &m)?)?;
            m.add_function(wrap_pyfunction!(stack_codes, &m)?)?;
            m.add_function(wrap_pyfunction!(volume, &m)?)?;
            let names: Vec<&str> = vec!["all", "frame", "layer", "named", "render", "stack", "stack_codes", "volume", "Field", "Preset", "Volume", "Layer", "Spec"];
            m.add("__all__", names)?;
            pairs::init(py, &m, sys)?;
            sample::init(py, &m, sys)?;
            parent.add("moire", &m)?;
            sys.set_item("mrlypy._mrlypy.math.moire", &m)?;
            Ok(())
        }
    }

    /// The mrly names: one canonical JSON object for every mathematical thing.
    /// The mrly names.
    pub mod name {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A design code pinned to its dimension, lattice and base, with one unit index per filled digit when it twists.
        #[pyclass(name = "Bang", module = "mrlypy.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Bang(pub mrlyrs::math::name::Bang);

        #[pymethods]
        impl Bang {
            /// Pins a code to its dimension and base on the square lattice.
            #[new]
            #[pyo3(signature = (code, dim, base))]
            pub fn __new__(code: u128, dim: usize, base: usize) -> PyResult<Self> {
                let out = mrlyrs::math::name::Bang::new(code, dim, base);
                Ok(Self(out))
            }
            /// The number of axes.
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            /// The lattice, square unless said.
            #[getter]
            #[pyo3(name = "lattice")]
            pub fn lattice<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.lattice;
                (PySerde(value)).into_bound_py_any(py)
            }
            /// The digits per axis, 2 unless said.
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base;
                (value).into_bound_py_any(py)
            }
            /// The design as a number.
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            /// One unit index per filled digit, absent when nothing turns.
            #[getter]
            #[pyo3(name = "twist")]
            pub fn twist<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.twist.clone();
                (value).into_bound_py_any(py)
            }
            /// Returns the number of digits the code addresses.
            #[pyo3(name = "cells", signature = ())]
            pub fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::name::Bang::cells(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Folds a decoded value to its canonical form, or an error for one outside the kind.
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::checked(self.0.clone());
                (crate::gen::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a filename back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_file(text);
                (crate::gen::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a JSON object into its canonical value, or an error naming the broken key.
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_json(text);
                (crate::gen::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a path and query string back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_url(text);
                (crate::gen::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Pins a code to its dimension and base on the square lattice.
            #[staticmethod]
            #[pyo3(name = "new", signature = (code, dim, base))]
            pub fn new_<'py>(py: Python<'py>, code: u128, dim: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::name::Bang::new(code, dim, base);
                (crate::gen::math::name::Bang(out)).into_bound_py_any(py)
            }
            /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the first eight hex digits of the sha256 of the canonical JSON.
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the canonical JSON object.
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
            #[pyo3(name = "to_mrly", signature = ())]
            pub fn to_mrly<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_mrly(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_url(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// A design sequence's address: the design, the reading taken off it and the index it runs along.
        #[pyclass(name = "Sequence", module = "mrlypy.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Sequence(pub mrlyrs::math::name::Sequence);

        #[pymethods]
        impl Sequence {
            /// Pins a design's reading to its measure and axis.
            #[new]
            #[pyo3(signature = (code, dim, base, measure, axis))]
            pub fn __new__(code: u128, dim: usize, base: usize, measure: &str, axis: &str) -> PyResult<Self> {
                let out = mrlyrs::math::name::Sequence::new(code, dim, base, measure, axis);
                Ok(Self(out))
            }
            /// The number of axes.
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            /// The digits per axis, 2 unless said.
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base;
                (value).into_bound_py_any(py)
            }
            /// The design as a number.
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            /// The reading taken.
            #[getter]
            #[pyo3(name = "measure")]
            pub fn measure<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.measure.clone();
                (value).into_bound_py_any(py)
            }
            /// The index the reading runs along.
            #[getter]
            #[pyo3(name = "axis")]
            pub fn axis<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.axis.clone();
                (value).into_bound_py_any(py)
            }
            /// Folds a decoded value to its canonical form, or an error for one outside the kind.
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::checked(self.0.clone());
                (crate::gen::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the design pinned to its dimension and base.
            #[pyo3(name = "design", signature = ())]
            pub fn design<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::name::Sequence::design(&self.0);
                (crate::gen::math::name::Bang(out)).into_bound_py_any(py)
            }
            /// Reads a filename back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_file(text);
                (crate::gen::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a JSON object into its canonical value, or an error naming the broken key.
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_json(text);
                (crate::gen::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a path and query string back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_url(text);
                (crate::gen::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Pins a design's reading to its measure and axis.
            #[staticmethod]
            #[pyo3(name = "new", signature = (code, dim, base, measure, axis))]
            pub fn new_<'py>(py: Python<'py>, code: u128, dim: usize, base: usize, measure: &str, axis: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::name::Sequence::new(code, dim, base, measure, axis);
                (crate::gen::math::name::Sequence(out)).into_bound_py_any(py)
            }
            /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the first eight hex digits of the sha256 of the canonical JSON.
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the canonical JSON object.
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
            #[pyo3(name = "to_mrly", signature = ())]
            pub fn to_mrly<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_mrly(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_url(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// A magic word: an ordered list of design letters, first letter outermost, each at its own side.
        #[pyclass(name = "Word", module = "mrlypy.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Word(pub mrlyrs::math::name::Word);

        #[pymethods]
        impl Word {
            /// Pins an ordered letter list at base 2.
            #[new]
            #[pyo3(signature = (dim, letters))]
            pub fn __new__(dim: usize, letters: Vec<(u128, usize)>) -> PyResult<Self> {
                let out = mrlyrs::math::name::Word::new(dim, &letters);
                Ok(Self(ok(out)?))
            }
            /// The number of axes every letter shares.
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            /// The codes of the letters in order.
            #[getter]
            #[pyo3(name = "magic")]
            pub fn magic<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.magic.clone();
                (value).into_bound_py_any(py)
            }
            /// The side each letter renders at.
            #[getter]
            #[pyo3(name = "side")]
            pub fn side<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.side.clone();
                (value).into_bound_py_any(py)
            }
            /// The base of each letter, absent when every letter is base 2.
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base.clone();
                (value).into_bound_py_any(py)
            }
            /// Returns the base of every letter, 2 where the name says nothing.
            #[pyo3(name = "bases", signature = ())]
            pub fn bases<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::name::Word::bases(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Folds a decoded value to its canonical form, or an error for one outside the kind.
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::checked(self.0.clone());
                (crate::gen::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a filename back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_file(text);
                (crate::gen::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a JSON object into its canonical value, or an error naming the broken key.
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_json(text);
                (crate::gen::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a path and query string back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_url(text);
                (crate::gen::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns every letter as a design pinned to the word's dimension and its own base.
            #[pyo3(name = "letters", signature = ())]
            pub fn letters<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::name::Word::letters(&self.0);
                ((out).into_iter().map(crate::gen::math::name::Bang).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Pins an ordered letter list at base 2.
            #[staticmethod]
            #[pyo3(name = "new", signature = (dim, letters))]
            pub fn new_<'py>(py: Python<'py>, dim: usize, letters: Vec<(u128, usize)>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::name::Word::new(dim, &letters);
                (crate::gen::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the first eight hex digits of the sha256 of the canonical JSON.
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the canonical JSON object.
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
            #[pyo3(name = "to_mrly", signature = ())]
            pub fn to_mrly<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_mrly(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_url(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// The lattice the cells sit on.
        #[pyclass(name = "Lattice", module = "mrlypy.math.name", skip_from_py_object)]
        pub struct Lattice;

        #[pymethods]
        impl Lattice {
            /// Returns whether this is the square lattice.
            #[staticmethod]
            #[pyo3(name = "is_square", signature = (lattice))]
            pub fn is_square<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::math::name::Lattice>) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::math::name::Lattice::is_square(&lattice);
                (out).into_bound_py_any(py)
            }
            /// Returns the number of unit directions a twist may pick from.
            #[staticmethod]
            #[pyo3(name = "units", signature = (lattice))]
            pub fn units<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::math::name::Lattice>) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::math::name::Lattice::units(lattice);
                (out).into_bound_py_any(py)
            }
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.name")?;
            m.setattr("__doc__", "The mrly names: one canonical JSON object for every mathematical thing.\nThe mrly names.\n\nEvery mathematical thing prints one canonical JSON object, and the url, filename, prose and id\nviews are functions of that one string.")?;
            m.add_class::<Bang>()?;
            m.add_class::<Sequence>()?;
            m.add_class::<Word>()?;
            m.add_class::<Lattice>()?;
            let names: Vec<&str> = vec!["Bang", "Sequence", "Word", "Lattice"];
            m.add("__all__", names)?;
            parent.add("name", &m)?;
            sys.set_item("mrlypy._mrlypy.math.name", &m)?;
            Ok(())
        }
    }

    /// The sequence press: the integers a design's digit rule keeps, weighed all at once.
    pub mod press {
        use crate::hand::{ok, PyCode, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The tally press: one pass over the integers weighs every design of a universe at once.
        #[pyclass(name = "Press", module = "mrlypy.math.press", skip_from_py_object)]
        pub struct Press(pub mrlyrs::math::press::Press);

        #[pymethods]
        impl Press {
            /// Builds an empty press over every design of the dimension and base.
            #[new]
            #[pyo3(signature = (dimension, base))]
            pub fn __new__(dimension: usize, base: usize) -> PyResult<Self> {
                let out = mrlyrs::math::press::Press::new(dimension, base);
                Ok(Self(ok(out)?))
            }
            /// The design dimension of the universe.
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            /// The numeral base of the universe.
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base;
                (value).into_bound_py_any(py)
            }
            /// Adds a weighted number to its usage bucket.
            #[pyo3(name = "add", signature = (number, weight))]
            pub fn add<'py>(&mut self, py: Python<'py>, number: u128, weight: i128) -> PyResult<Bound<'py, PyAny>> {
                mrlyrs::math::press::Press::add(&mut self.0, number, weight);
                ().into_bound_py_any(py)
            }
            /// Builds an empty press over every design of the dimension and base.
            #[staticmethod]
            #[pyo3(name = "new", signature = (dimension, base))]
            pub fn new_<'py>(py: Python<'py>, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::press::Press::new(dimension, base);
                (crate::gen::math::press::Press(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the total weight the design at a code has collected.
            #[pyo3(name = "total", signature = (code))]
            pub fn total<'py>(&self, py: Python<'py>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrlyrs::math::press::Press::total(&self.0, code);
                (out).into_bound_py_any(py)
            }
            /// Returns every design's total in code order by one subset-sum transform.
            #[pyo3(name = "totals", signature = ())]
            pub fn totals<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::press::Press::totals(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Returns the number of designs of the dimension and base that contain the number.
        #[pyfunction]
        #[pyo3(name = "containing", signature = (number, dimension, base))]
        pub fn containing<'py>(py: Python<'py>, number: u128, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::press::containing(number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Splits a number into its dimension coordinates, one base digit peeled per axis in parallel.
        #[pyfunction]
        #[pyo3(name = "coordinates", signature = (number, dimension, base))]
        pub fn coordinates<'py>(py: Python<'py>, number: u128, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::press::coordinates(number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Counts the members of a design below the limit.
        #[pyfunction]
        #[pyo3(name = "count_below", signature = (code, dimension, base, limit))]
        pub fn count_below<'py>(py: Python<'py>, code: PyCode, dimension: usize, base: usize, limit: u128) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::press::count_below(code, dimension, base, limit);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the count of distinct digit vectors the number uses.
        #[pyfunction]
        #[pyo3(name = "distinct", signature = (number, dimension, base))]
        pub fn distinct<'py>(py: Python<'py>, number: u128, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::press::distinct(number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Weaves dimension coordinates back into their single interleaved number.
        #[pyfunction]
        #[pyo3(name = "interleave", signature = (coords, base))]
        pub fn interleave<'py>(py: Python<'py>, coords: Vec<u128>, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::press::interleave(&coords, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the allowed digit table of one magic layer, one flag per cell of its tile.
        #[pyfunction]
        #[pyo3(name = "layer_table", signature = (layer))]
        pub fn layer_table<'py>(py: Python<'py>, layer: PySerde<mrlyrs::math::bang::MagicLayer>) -> PyResult<Bound<'py, PyAny>> {
            let layer = layer.0;
            let out = mrlyrs::math::press::layer_table(&layer);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns whether every digit vector of the number lies in the design.
        #[pyfunction]
        #[pyo3(name = "member", signature = (code, number, dimension, base))]
        pub fn member<'py>(py: Python<'py>, code: PyCode, number: u128, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::press::member(code, number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the first members of a design in ascending order.
        #[pyfunction]
        #[pyo3(name = "members", signature = (code, dimension, base, count))]
        pub fn members<'py>(py: Python<'py>, code: PyCode, dimension: usize, base: usize, count: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::press::members(code, dimension, base, count);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the diagonal slice profile of one design pressed to a fractal level.
        #[pyfunction]
        #[pyo3(name = "profile", signature = (code, dimension, base, level))]
        pub fn profile<'py>(py: Python<'py>, code: PyCode, dimension: usize, base: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::press::profile(code, dimension, base, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the corner-usage mask of a number, one bit per digit vector its expansion uses.
        #[pyfunction]
        #[pyo3(name = "usage", signature = (number, dimension, base))]
        pub fn usage<'py>(py: Python<'py>, number: u128, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::press::usage(number, dimension, base);
            (PyCode(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts the members of a magic word from its layer fills, without enumeration.
        #[pyfunction]
        #[pyo3(name = "word_count", signature = (layers))]
        pub fn word_count<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::press::word_count(&layers);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns whether the number lies in the magic word's composed design.
        #[pyfunction]
        #[pyo3(name = "word_member", signature = (layers, number))]
        pub fn word_member<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>, number: u128) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::press::word_member(&layers, number);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Enumerates every member of the magic word in ascending order.
        #[pyfunction]
        #[pyo3(name = "word_members", signature = (layers))]
        pub fn word_members<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::press::word_members(&layers);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the diagonal slice profile of a magic word by the substitution product.
        #[pyfunction]
        #[pyo3(name = "word_profile", signature = (layers))]
        pub fn word_profile<'py>(py: Python<'py>, layers: Vec<PySerde<mrlyrs::math::bang::MagicLayer>>) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::press::word_profile(&layers);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.press")?;
            m.setattr("__doc__", "The sequence press: the integers a design's digit rule keeps, weighed all at once.")?;
            m.add_class::<Press>()?;
            m.add_function(wrap_pyfunction!(containing, &m)?)?;
            m.add_function(wrap_pyfunction!(coordinates, &m)?)?;
            m.add_function(wrap_pyfunction!(count_below, &m)?)?;
            m.add_function(wrap_pyfunction!(distinct, &m)?)?;
            m.add_function(wrap_pyfunction!(interleave, &m)?)?;
            m.add_function(wrap_pyfunction!(layer_table, &m)?)?;
            m.add_function(wrap_pyfunction!(member, &m)?)?;
            m.add_function(wrap_pyfunction!(members, &m)?)?;
            m.add_function(wrap_pyfunction!(profile, &m)?)?;
            m.add_function(wrap_pyfunction!(usage, &m)?)?;
            m.add_function(wrap_pyfunction!(word_count, &m)?)?;
            m.add_function(wrap_pyfunction!(word_member, &m)?)?;
            m.add_function(wrap_pyfunction!(word_members, &m)?)?;
            m.add_function(wrap_pyfunction!(word_profile, &m)?)?;
            m.add("CORNERS", mrlyrs::math::press::CORNERS)?;
            let names: Vec<&str> = vec!["containing", "coordinates", "count_below", "distinct", "interleave", "layer_table", "member", "members", "profile", "usage", "word_count", "word_member", "word_members", "word_profile", "Press", "CORNERS"];
            m.add("__all__", names)?;
            parent.add("press", &m)?;
            sys.set_item("mrlypy._mrlypy.math.press", &m)?;
            Ok(())
        }
    }

    /// The nodes of a roulette: where the curves a wheel's pencils draw cross themselves and one another.
    pub mod roulette {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Every crossing of a traced roulette: the curves against themselves, the curves against one another, and how crowded the worst node is.
        #[pyclass(name = "Nodes", module = "mrlypy.math.roulette", from_py_object)]
        #[derive(Clone)]
        pub struct Nodes(pub mrlyrs::math::roulette::Nodes);

        #[pymethods]
        impl Nodes {
            /// The curves counted, in the order the pencils came in.
            #[getter]
            #[pyo3(name = "curves")]
            pub fn curves<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.curves;
                (value).into_bound_py_any(py)
            }
            /// How often each curve crosses itself, curve by curve.
            #[getter]
            #[pyo3(name = "selves")]
            pub fn selves<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.selves.clone();
                (value).into_bound_py_any(py)
            }
            /// How often each pair of curves crosses, the lower curve first, in lexicographic order.
            #[getter]
            #[pyo3(name = "pairs")]
            pub fn pairs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.pairs.clone();
                (value).into_bound_py_any(py)
            }
            /// The most crossings one node carries: one at a plain double point, and `n(n - 1)/2` where `n` branches meet.
            #[getter]
            #[pyo3(name = "most")]
            pub fn most<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.most;
                (value).into_bound_py_any(py)
            }
            /// The nodes more than one crossing clusters at.
            #[getter]
            #[pyo3(name = "crowded")]
            pub fn crowded<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.crowded;
                (value).into_bound_py_any(py)
            }
            /// The distinct points the crossings sit at, one for every cluster.
            #[getter]
            #[pyo3(name = "points")]
            pub fn points<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.points;
                (value).into_bound_py_any(py)
            }
            /// The branches through every node added up, which is the edge count of the picture as a plane graph, `n` at a node where `n` branches meet and `2` times `points` when no node is crowded.
            #[getter]
            #[pyo3(name = "branches")]
            pub fn branches<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.branches;
                (value).into_bound_py_any(py)
            }
            /// The segment pairs that meet without crossing: collinear or end to end.
            #[getter]
            #[pyo3(name = "touches")]
            pub fn touches<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.touches;
                (value).into_bound_py_any(py)
            }
            /// How often the curves `i` and `j` cross, either order, and zero when they are one curve.
            #[pyo3(name = "pair", signature = (i, j))]
            pub fn pair<'py>(&self, py: Python<'py>, i: usize, j: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::roulette::Nodes::pair(&self.0, i, j);
                (out).into_bound_py_any(py)
            }
            /// Every crossing of two curves.
            #[pyo3(name = "paired", signature = ())]
            pub fn paired<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::roulette::Nodes::paired(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Every self crossing.
            #[pyo3(name = "selved", signature = ())]
            pub fn selved<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::roulette::Nodes::selved(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Every crossing, self and pair together, which counts a node where `n` branches meet `n(n - 1)/2` times; `points` is the count of distinct nodes and the two agree exactly when `crowded` is zero.
            #[pyo3(name = "total", signature = ())]
            pub fn total<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::roulette::Nodes::total(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Counts the nodes of the roulette the pencils draw on the track: `mrlyrs::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders.
        #[pyfunction]
        #[pyo3(name = "nodes", signature = (track, pencils, samples, tol))]
        pub fn nodes<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>, samples: usize, tol: f64) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::roulette::nodes(&track, &pencils, samples, tol);
            (crate::gen::math::roulette::Nodes(ok(out)?)).into_bound_py_any(py)
        }

        /// Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one.
        #[pyfunction]
        #[pyo3(name = "side", signature = (a, b, c))]
        pub fn side<'py>(py: Python<'py>, a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::roulette::side(a, b, c);
            (out).into_bound_py_any(py)
        }

        /// One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrlyrs::math::spirograph::distinct` and `mrlyrs::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve.
        #[pyfunction]
        #[pyo3(name = "spread", signature = (track, pencils, exact))]
        pub fn spread<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>, exact: bool) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::roulette::spread(&track, &pencils, exact);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.roulette")?;
            m.setattr("__doc__", "The nodes of a roulette: where the curves a wheel's pencils draw cross themselves and one another.")?;
            m.add_class::<Nodes>()?;
            m.add_function(wrap_pyfunction!(nodes, &m)?)?;
            m.add_function(wrap_pyfunction!(side, &m)?)?;
            m.add_function(wrap_pyfunction!(spread, &m)?)?;
            let names: Vec<&str> = vec!["nodes", "side", "spread", "Nodes"];
            m.add("__all__", names)?;
            parent.add("roulette", &m)?;
            sys.set_item("mrlypy._mrlypy.math.roulette", &m)?;
            Ok(())
        }
    }

    /// The residue rules that mark a hypercube's cells.
    pub mod rules {
        use crate::hand::{ok, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list.
        #[pyfunction]
        #[pyo3(name = "render", signature = (filled, number, dimension, base))]
        pub fn render<'py>(py: Python<'py>, filled: Vec<Vec<u8>>, number: usize, dimension: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::rules::render(&filled, number, dimension, base);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns every axis but the free one.
        #[pyfunction]
        #[pyo3(name = "tree_axes", signature = (dimension, free_axis))]
        pub fn tree_axes<'py>(py: Python<'py>, dimension: usize, free_axis: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::rules::tree_axes(dimension, free_axis);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.rules")?;
            m.setattr("__doc__", "The residue rules that mark a hypercube's cells.")?;
            m.add_function(wrap_pyfunction!(render, &m)?)?;
            m.add_function(wrap_pyfunction!(tree_axes, &m)?)?;
            m.add("BASE", mrlyrs::math::rules::BASE)?;
            let names: Vec<&str> = vec!["render", "tree_axes", "BASE"];
            m.add("__all__", names)?;
            parent.add("rules", &m)?;
            sys.set_item("mrlypy._mrlypy.math.rules", &m)?;
            Ok(())
        }
    }

    /// The exact crop machinery: rational shapes classified cell by cell, no floats.
    pub mod shape {
        use crate::hand::{ok, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// An exact rational number with a positive, reduced denominator.
        #[pyclass(name = "Frac", module = "mrlypy.math.shape", from_py_object)]
        #[derive(Clone)]
        pub struct Frac(pub mrlyrs::math::shape::Frac);

        #[pymethods]
        impl Frac {
            /// Builds the reduced fraction num over den.
            #[new]
            #[pyo3(signature = (num, den))]
            pub fn __new__(num: i64, den: i64) -> PyResult<Self> {
                let out = mrlyrs::math::shape::Frac::new(num, den);
                Ok(Self(ok(out)?))
            }
            /// The numerator, carrying the sign.
            #[getter]
            #[pyo3(name = "num")]
            pub fn num<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.num;
                (value).into_bound_py_any(py)
            }
            /// The denominator, always positive.
            #[getter]
            #[pyo3(name = "den")]
            pub fn den<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.den;
                (value).into_bound_py_any(py)
            }
            /// Returns the exact difference.
            #[pyo3(name = "minus", signature = (other))]
            pub fn minus<'py>(&self, py: Python<'py>, other: crate::gen::math::shape::Frac) -> PyResult<Bound<'py, PyAny>> {
                let other = other.0;
                let out = mrlyrs::math::shape::Frac::minus(self.0, other);
                (crate::gen::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            /// Builds the reduced fraction num over den.
            #[staticmethod]
            #[pyo3(name = "new", signature = (num, den))]
            pub fn new_<'py>(py: Python<'py>, num: i64, den: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::shape::Frac::new(num, den);
                (crate::gen::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the exact sum.
            #[pyo3(name = "plus", signature = (other))]
            pub fn plus<'py>(&self, py: Python<'py>, other: crate::gen::math::shape::Frac) -> PyResult<Bound<'py, PyAny>> {
                let other = other.0;
                let out = mrlyrs::math::shape::Frac::plus(self.0, other);
                (crate::gen::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the exact product.
            #[pyo3(name = "times", signature = (other))]
            pub fn times<'py>(&self, py: Python<'py>, other: crate::gen::math::shape::Frac) -> PyResult<Bound<'py, PyAny>> {
                let other = other.0;
                let out = mrlyrs::math::shape::Frac::times(self.0, other);
                (crate::gen::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            /// Wraps an integer as a fraction over one.
            #[staticmethod]
            #[pyo3(name = "whole", signature = (num))]
            pub fn whole<'py>(py: Python<'py>, num: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::shape::Frac::whole(num);
                (crate::gen::math::shape::Frac(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Where one lattice cell sits relative to a shape.
        #[pyclass(name = "Region", module = "mrlypy.math.shape", skip_from_py_object)]
        pub struct Region;

        #[pymethods]
        impl Region {
            /// Swaps In and Out, keeping Cut.
            #[staticmethod]
            #[pyo3(name = "flip", signature = (region))]
            pub fn flip<'py>(py: Python<'py>, region: PySerde<mrlyrs::math::shape::Region>) -> PyResult<Bound<'py, PyAny>> {
                let region = region.0;
                let out = mrlyrs::math::shape::Region::flip(region);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// Tallies the design's cells and filled cells per region of the shape.
        #[pyfunction]
        #[pyo3(name = "census", signature = (shape, types))]
        pub fn census<'py>(py: Python<'py>, shape: PySerde<mrlyrs::math::shape::Shape>, types: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let shape = shape.0;
            let types = types.0;
            let out = mrlyrs::math::shape::census(&shape, &types);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Places one lattice cell relative to the shape, exactly, with no floats.
        #[pyfunction]
        #[pyo3(name = "classify", signature = (shape, side, index))]
        pub fn classify<'py>(py: Python<'py>, shape: PySerde<mrlyrs::math::shape::Shape>, side: usize, index: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let shape = shape.0;
            let out = mrlyrs::math::shape::classify(&shape, side, &index);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti.
        #[pyfunction]
        #[pyo3(name = "crop", signature = (types, shape, keep_cut))]
        pub fn crop<'py>(py: Python<'py>, types: PyTensor, shape: PySerde<mrlyrs::math::shape::Shape>, keep_cut: bool) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let shape = shape.0;
            let out = mrlyrs::math::shape::crop(&types, &shape, keep_cut);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order.
        #[pyfunction]
        #[pyo3(name = "crossing_shell", signature = (radius, number, level))]
        pub fn crossing_shell<'py>(py: Python<'py>, radius: u64, number: u64, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::shape::crossing_shell(radius, number, level);
            (out).into_bound_py_any(py)
        }

        /// Builds the whole crossing tree of one radius, pruned by the seats the design keeps.
        #[pyfunction]
        #[pyo3(name = "crossing_tree", signature = (radius, number, keep))]
        pub fn crossing_tree<'py>(py: Python<'py>, radius: u64, number: u64, keep: Vec<bool>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::shape::crossing_tree(radius, number, &keep);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Builds a named shape of the dimension, centered at one half on every axis.
        #[pyfunction]
        #[pyo3(name = "named", signature = (name, dimension, radius))]
        pub fn named<'py>(py: Python<'py>, name: &str, dimension: usize, radius: crate::gen::math::shape::Frac) -> PyResult<Bound<'py, PyAny>> {
            let radius = radius.0;
            let out = mrlyrs::math::shape::named(name, dimension, radius);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic.
        #[pyfunction]
        #[pyo3(name = "radial_census", signature = (types, centre, r_max))]
        pub fn radial_census<'py>(py: Python<'py>, types: PyTensor, centre: Vec<i64>, r_max: u64) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let out = mrlyrs::math::shape::radial_census(&types, &centre, r_max);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes.
        #[pyfunction]
        #[pyo3(name = "refine", signature = (types, shape, base, extra, keep_cut))]
        pub fn refine<'py>(py: Python<'py>, types: PyTensor, shape: PySerde<mrlyrs::math::shape::Shape>, base: usize, extra: usize, keep_cut: bool) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let shape = shape.0;
            let out = mrlyrs::math::shape::refine(&types, &shape, base, extra, keep_cut);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side.
        #[pyfunction]
        #[pyo3(name = "regions", signature = (shape, dims))]
        pub fn regions<'py>(py: Python<'py>, shape: PySerde<mrlyrs::math::shape::Shape>, dims: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let shape = shape.0;
            let out = mrlyrs::math::shape::regions(&shape, &dims);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Lists the named shapes of a dimension.
        #[pyfunction]
        #[pyo3(name = "shapes", signature = (dimension))]
        pub fn shapes<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::shape::shapes(dimension);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.shape")?;
            m.setattr("__doc__", "The exact crop machinery: rational shapes classified cell by cell, no floats.")?;
            m.add_class::<Frac>()?;
            m.add_class::<Region>()?;
            m.add_function(wrap_pyfunction!(census, &m)?)?;
            m.add_function(wrap_pyfunction!(classify, &m)?)?;
            m.add_function(wrap_pyfunction!(crop, &m)?)?;
            m.add_function(wrap_pyfunction!(crossing_shell, &m)?)?;
            m.add_function(wrap_pyfunction!(crossing_tree, &m)?)?;
            m.add_function(wrap_pyfunction!(named, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_census, &m)?)?;
            m.add_function(wrap_pyfunction!(refine, &m)?)?;
            m.add_function(wrap_pyfunction!(regions, &m)?)?;
            m.add_function(wrap_pyfunction!(shapes, &m)?)?;
            m.add("REFINE_LIMIT", mrlyrs::math::shape::REFINE_LIMIT)?;
            let names: Vec<&str> = vec!["census", "classify", "crop", "crossing_shell", "crossing_tree", "named", "radial_census", "refine", "regions", "shapes", "Frac", "Region", "REFINE_LIMIT"];
            m.add("__all__", names)?;
            parent.add("shape", &m)?;
            sys.set_item("mrlypy._mrlypy.math.shape", &m)?;
            Ok(())
        }
    }

    /// The hexagon world: cubes flattened to triangle-meshed hexes.
    /// The hexagon world, the projection of `three`.
    pub mod six {
        use crate::hand::{ok, PyCell2d, PyCell3d, PyCell6d, PyCellNd, PyCode, PyColor, PyRgba, PyRng, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The ghost star of the hexagonal cut stack: the arm ink law, the background and the cell-frame decay.
        pub mod star {
            use crate::hand::{ok, PySerde};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// The exact reading of a cut layer: how many cells were inked out of how many were read.
            #[pyclass(name = "Share", module = "mrlypy.math.six.star", from_py_object)]
            #[derive(Clone)]
            pub struct Share(pub mrlyrs::math::six::star::Share);

            #[pymethods]
            impl Share {
                /// The count of inked cells.
                #[getter]
                #[pyo3(name = "inked")]
                pub fn inked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let value = self.0.inked;
                    (value).into_bound_py_any(py)
                }
                /// The count of cells read.
                #[getter]
                #[pyo3(name = "cells")]
                pub fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let value = self.0.cells;
                    (value).into_bound_py_any(py)
                }
                /// The share in lowest terms, numerator then denominator.
                #[pyo3(name = "reduced", signature = ())]
                pub fn reduced<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Share::reduced(self.0);
                    (out).into_bound_py_any(py)
                }
                /// The share as a real number.
                #[pyo3(name = "value", signature = ())]
                pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Share::value(self.0);
                    (out).into_bound_py_any(py)
                }
                /// Reads plain data into the class.
                #[staticmethod]
                pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                    Ok(Self(crate::hand::serde_from_py(data)?))
                }
                /// Returns the value as plain data.
                pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    crate::hand::serde_into_py(py, &self.0)
                }
            }

            /// The ghost star of a coded cube's hexagonal cut stack, read in the cell frame.
            #[pyclass(name = "Star", module = "mrlypy.math.six.star", skip_from_py_object)]
            pub struct Star(pub mrlyrs::math::six::star::Star);

            #[pymethods]
            impl Star {
                /// Reads the star of a base-2 space code, the carpet being `23`.
                #[new]
                #[pyo3(signature = (code))]
                pub fn __new__(code: u128) -> PyResult<Self> {
                    let out = mrlyrs::math::six::star::Star::new(code);
                    Ok(Self(ok(out)?))
                }
                /// The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`.
                #[pyo3(name = "arm", signature = (number, half))]
                pub fn arm<'py>(&self, py: Python<'py>, number: usize, half: usize) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Star::arm(&self.0, number, half);
                    (crate::gen::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
                }
                /// The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`.
                #[pyo3(name = "cell", signature = (number, x, z))]
                pub fn cell<'py>(&self, py: Python<'py>, number: usize, x: i64, z: i64) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Star::cell(&self.0, number, x, z);
                    (out).into_bound_py_any(py)
                }
                /// The per-layer excess of the star band over the hexagon across the first `L` odd layers.
                #[pyo3(name = "excesses", signature = (layers, half))]
                pub fn excesses<'py>(&self, py: Python<'py>, layers: usize, half: usize) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Star::excesses(&self.0, layers, half);
                    (ok(out)?).into_bound_py_any(py)
                }
                /// The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against.
                #[pyo3(name = "hexagon", signature = (number))]
                pub fn hexagon<'py>(&self, py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Star::hexagon(&self.0, number);
                    (crate::gen::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
                }
                /// Reads the star of a base-2 space code, the carpet being `23`.
                #[staticmethod]
                #[pyo3(name = "new", signature = (code))]
                pub fn new_<'py>(py: Python<'py>, code: u128) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Star::new(code);
                    (crate::gen::math::six::star::Star(ok(out)?)).into_bound_py_any(py)
                }
                /// Reads plain data into the class.
                #[staticmethod]
                pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                    Ok(Self(crate::hand::serde_from_py(data)?))
                }
                /// Returns the value as plain data.
                pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    crate::hand::serde_into_py(py, &self.0)
                }
            }

            /// The three classes of layer count the `1/L^2` term of the decay reads.
            #[pyclass(name = "Branch", module = "mrlypy.math.six.star", skip_from_py_object)]
            pub struct Branch;

            #[pymethods]
            impl Branch {
                /// The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`.
                #[staticmethod]
                #[pyo3(name = "constant", signature = (branch))]
                pub fn constant<'py>(py: Python<'py>, branch: PySerde<mrlyrs::math::six::star::Branch>) -> PyResult<Bound<'py, PyAny>> {
                    let branch = branch.0;
                    let out = mrlyrs::math::six::star::Branch::constant(branch);
                    (out).into_bound_py_any(py)
                }
                /// The name of the branch.
                #[staticmethod]
                #[pyo3(name = "name", signature = (branch))]
                pub fn name<'py>(py: Python<'py>, branch: PySerde<mrlyrs::math::six::star::Branch>) -> PyResult<Bound<'py, PyAny>> {
                    let branch = branch.0;
                    let out = mrlyrs::math::six::star::Branch::name(branch);
                    (out).into_bound_py_any(py)
                }
                /// The branch of a layer count.
                #[staticmethod]
                #[pyo3(name = "of", signature = (layers))]
                pub fn of<'py>(py: Python<'py>, layers: usize) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrlyrs::math::six::star::Branch::of(layers);
                    (PySerde(out)).into_bound_py_any(py)
                }
                /// The exact `1/L^2` coefficient at even `L`, absent at odd `L`.
                #[staticmethod]
                #[pyo3(name = "residual", signature = (branch))]
                pub fn residual<'py>(py: Python<'py>, branch: PySerde<mrlyrs::math::six::star::Branch>) -> PyResult<Bound<'py, PyAny>> {
                    let branch = branch.0;
                    let out = mrlyrs::math::six::star::Branch::residual(branch);
                    (out).into_bound_py_any(py)
                }
            }

            /// The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`.
            #[pyfunction]
            #[pyo3(name = "arm_law", signature = (number))]
            pub fn arm_law<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::six::star::arm_law(number);
                (crate::gen::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
            }

            /// The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`.
            #[pyfunction]
            #[pyo3(name = "chi8", signature = (number))]
            pub fn chi8<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::six::star::chi8(number);
                (out).into_bound_py_any(py)
            }

            /// The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`.
            #[pyfunction]
            #[pyo3(name = "constant", signature = ())]
            pub fn constant<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::six::star::constant();
                (out).into_bound_py_any(py)
            }

            /// The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`.
            #[pyfunction]
            #[pyo3(name = "decay", signature = (excesses, layers))]
            pub fn decay<'py>(py: Python<'py>, excesses: Vec<f64>, layers: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::six::star::decay(&excesses, layers);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            /// The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`.
            #[pyfunction]
            #[pyo3(name = "width_law", signature = (half))]
            pub fn width_law<'py>(py: Python<'py>, half: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::six::star::width_law(half);
                (out).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.six.star")?;
                m.setattr("__doc__", "The ghost star of the hexagonal cut stack: the arm ink law, the background and the cell-frame decay.")?;
                m.add_class::<Share>()?;
                m.add_class::<Star>()?;
                m.add_class::<Branch>()?;
                m.add_function(wrap_pyfunction!(arm_law, &m)?)?;
                m.add_function(wrap_pyfunction!(chi8, &m)?)?;
                m.add_function(wrap_pyfunction!(constant, &m)?)?;
                m.add_function(wrap_pyfunction!(decay, &m)?)?;
                m.add_function(wrap_pyfunction!(width_law, &m)?)?;
                let names: Vec<&str> = vec!["arm_law", "chi8", "constant", "decay", "width_law", "Share", "Star", "Branch"];
                m.add("__all__", names)?;
                parent.add("star", &m)?;
                sys.set_item("mrlypy._mrlypy.math.six.star", &m)?;
                Ok(())
            }
        }

        /// Swaps every fill triangle for a void and back.
        #[pyfunction]
        #[pyo3(name = "anti", signature = (cell))]
        pub fn anti<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::Cell6d::anti(cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// Maps each triangle to one at or above the threshold, zero below.
        #[pyfunction]
        #[pyo3(name = "binarize", signature = (cell, threshold))]
        pub fn binarize<'py>(py: Python<'py>, cell: PyCell6d, threshold: u8) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::Cell6d::binarize(cell, threshold);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// Binarizes the triangles at the threshold Otsu's method picks.
        #[pyfunction]
        #[pyo3(name = "binarize_otsu", signature = (cell))]
        pub fn binarize_otsu<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::Cell6d::binarize_otsu(cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// Builds a hexagon of the given radius, fill inside and void outside.
        #[pyfunction]
        #[pyo3(name = "blank", signature = (radius, orient, fill, void))]
        pub fn blank<'py>(py: Python<'py>, radius: usize, orient: PySerde<mrlyrs::math::six::Orientation>, fill: u8, void: u8) -> PyResult<Bound<'py, PyAny>> {
            let orient = orient.0;
            let out = mrlyrs::math::six::blank(radius, orient, fill, void);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Rounds each triangle to the mean of its masked neighborhood, wrapping on request.
        #[pyfunction]
        #[pyo3(name = "blur", signature = (cell, mask, wrap))]
        pub fn blur<'py>(py: Python<'py>, cell: PyCell6d, mask: PyTensor, wrap: bool) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrlyrs::math::six::Cell6d::blur(cell, &mask, wrap);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Tallies a cell's triangles, corners and edges, counting the backdrop only on request.
        #[pyfunction]
        #[pyo3(name = "census", signature = (cell, include_grid))]
        pub fn census<'py>(py: Python<'py>, cell: PyCell6d, include_grid: bool) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::census(&cell, include_grid);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Counts the connected pieces of the fill, triangles joined across shared edges.
        #[pyfunction]
        #[pyo3(name = "components", signature = (cell))]
        pub fn components<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::components(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Slices a cube through its center across the main diagonal into a hexagon.
        #[pyfunction]
        #[pyo3(name = "cut", signature = (cell))]
        pub fn cut<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::cut(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the coded 3d design and slices its central hexagon.
        #[pyfunction]
        #[pyo3(name = "cut_design", signature = (code, number, level, base))]
        pub fn cut_design<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::six::cut_design(code, number, level, base);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// The three corners of the east-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "east", signature = (x, y))]
        pub fn east<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::six::east(x, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request.
        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell, include_grid))]
        pub fn euler<'py>(py: Python<'py>, cell: PyCell6d, include_grid: bool) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::euler(&cell, include_grid);
            (out).into_bound_py_any(py)
        }

        /// Counts the filled triangles of the cell.
        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::fills(&cell);
            (out).into_bound_py_any(py)
        }

        /// Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh.
        #[pyfunction]
        #[pyo3(name = "fills_only", signature = (cell))]
        pub fn fills_only<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::fills_only(&cell);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood.
        #[pyfunction]
        #[pyo3(name = "framed", signature = (cell))]
        pub fn framed<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::framed(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Parses a cell from JSON, defaulting any missing projection metadata.
        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::six::from_json(text);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the triangle count of the fill's largest connected piece.
        #[pyfunction]
        #[pyo3(name = "giant", signature = (cell))]
        pub fn giant<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::giant(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the largest connected piece of the filled-triangle network as a network of its own.
        #[pyfunction]
        #[pyo3(name = "giant_network", signature = (cell))]
        pub fn giant_network<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::giant_network(&cell);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the grid height in triangles.
        #[pyfunction]
        #[pyo3(name = "height", signature = (cell))]
        pub fn height<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::Cell6d::height(&cell);
            (out).into_bound_py_any(py)
        }

        /// Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh.
        #[pyfunction]
        #[pyo3(name = "holes", signature = (cell))]
        pub fn holes<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::holes(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns whether the cell's three sides are equal.
        #[pyfunction]
        #[pyo3(name = "is_cube", signature = (cell))]
        pub fn is_cube<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::is_cube(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns whether the cell's width, height and parity frame a hexagon.
        #[pyfunction]
        #[pyo3(name = "is_hex", signature = (cell))]
        pub fn is_hex<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::is_hex(&cell);
            (out).into_bound_py_any(py)
        }

        /// Projects a cube into the isometric hexagon of top, left and right faces.
        #[pyfunction]
        #[pyo3(name = "iso", signature = (cell))]
        pub fn iso<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::iso(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the coded 3d design and projects it isometrically.
        #[pyfunction]
        #[pyo3(name = "iso_design", signature = (code, number, level, base))]
        pub fn iso_design<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::six::iso_design(code, number, level, base);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a cell from its four parts.
        #[pyfunction]
        #[pyo3(name = "new", signature = (cell, projection, orientation, start))]
        pub fn new<'py>(py: Python<'py>, cell: PyCell2d, projection: PySerde<mrlyrs::math::six::Projection>, orientation: PySerde<mrlyrs::math::six::Orientation>, start: u8) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let projection = projection.0;
            let orientation = orientation.0;
            let out = mrlyrs::math::six::Cell6d::new(cell, projection, orientation, start);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// The three corners of the north-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "north", signature = (x, y))]
        pub fn north<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::six::north(x, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the orientation a hexagon's width and height imply.
        #[pyfunction]
        #[pyo3(name = "orientation", signature = (width, height))]
        pub fn orientation<'py>(py: Python<'py>, width: usize, height: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::six::orientation(width, height);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along.
        #[pyfunction]
        #[pyo3(name = "pad", signature = (cell, k, value))]
        pub fn pad<'py>(py: Python<'py>, cell: PyCell6d, k: usize, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::pad(&cell, k, value);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Colors each triangle by its type through the custom or default mapping in the given or type mode.
        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, custom=None, mode=None, rng=None))]
        pub fn paint<'py>(py: Python<'py>, cell: PyCell6d, custom: Option<std::collections::HashMap<u8, Vec<PyColor>>>, mode: Option<PySerde<mrlyrs::core::Mode>>, rng: Option<&mut PyRng>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let custom = custom.map(|x| x.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>())).collect());
            let mode = mode.map(|x| x.0);
            let out = mrlyrs::math::six::paint(cell, custom.as_ref(), mode, rng.map(|r| &mut r.0));
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Writes the value wherever the tiled mask is nonzero.
        #[pyfunction]
        #[pyo3(name = "perforate", signature = (cell, mask, value))]
        pub fn perforate<'py>(py: Python<'py>, cell: PyCell6d, mask: PyTensor, value: u8) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrlyrs::math::six::Cell6d::perforate(cell, &mask, value);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Rasters a cell's triangles to PNG bytes at the given scale, stroked and padded when an outline is given.
        #[pyfunction]
        #[pyo3(name = "png", signature = (cell, scale, outline, width))]
        pub fn png<'py>(py: Python<'py>, cell: PyCell6d, scale: usize, outline: Option<PyColor>, width: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let out = mrlyrs::math::six::png(&cell, scale, outline, width);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Projects a cube's three facing sides into a hexagon of fills and voids.
        #[pyfunction]
        #[pyo3(name = "pro", signature = (cell))]
        pub fn pro<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::pro(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the coded 3d design and projects its facing sides.
        #[pyfunction]
        #[pyo3(name = "pro_design", signature = (code, number, level, base))]
        pub fn pro_design<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::six::pro_design(code, number, level, base);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Tessellates a hexagonal cell over the disc mask of the given radius.
        #[pyfunction]
        #[pyo3(name = "radial", signature = (cell, radius))]
        pub fn radial<'py>(py: Python<'py>, cell: PyCell6d, radius: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::radial(&cell, radius);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Crops the interlocking overhang off a disc tiled at the given radius and tile size.
        #[pyfunction]
        #[pyo3(name = "radial_crop", signature = (cell, radius, size))]
        pub fn radial_crop<'py>(py: Python<'py>, cell: PyCell2d, radius: usize, size: (usize, usize)) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::radial_crop(&cell, radius, size);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the disc mask of cells within hex distance radius of the center.
        #[pyfunction]
        #[pyo3(name = "radial_mask", signature = (radius, orient))]
        pub fn radial_mask<'py>(py: Python<'py>, radius: usize, orient: PySerde<mrlyrs::math::six::Orientation>) -> PyResult<Bound<'py, PyAny>> {
            let orient = orient.0;
            let out = mrlyrs::math::six::radial_mask(radius, orient);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere.
        #[pyfunction]
        #[pyo3(name = "raster", signature = (cell, size))]
        pub fn raster<'py>(py: Python<'py>, cell: PyCell6d, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::raster(&cell, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to PNG bytes.
        #[pyfunction]
        #[pyo3(name = "rect_png", signature = (cell, scale, start=None))]
        pub fn rect_png<'py>(py: Python<'py>, cell: PyCell6d, scale: usize, start: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::rect_png(&cell, scale, start);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string.
        #[pyfunction]
        #[pyo3(name = "rect_svg", signature = (cell, scale, start=None))]
        pub fn rect_svg<'py>(py: Python<'py>, cell: PyCell6d, scale: usize, start: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::rect_svg(&cell, scale, start);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Counts the void regions the rim never reaches, the second route to the hole count.
        #[pyfunction]
        #[pyo3(name = "rim_holes", signature = (cell))]
        pub fn rim_holes<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::rim_holes(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure.
        #[pyfunction]
        #[pyo3(name = "skin", signature = (cell))]
        pub fn skin<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::skin(&cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// Builds the network of filled triangles joined by shared edges.
        #[pyfunction]
        #[pyo3(name = "slice_core_graph", signature = (cell))]
        pub fn slice_core_graph<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::slice_core_graph(&cell);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the network of fill and void triangles joined by shared edges.
        #[pyfunction]
        #[pyo3(name = "slice_dual_graph", signature = (cell))]
        pub fn slice_dual_graph<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::slice_dual_graph(&cell);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the corner-and-edge network of the triangles matching the value, or of every fill and void.
        #[pyfunction]
        #[pyo3(name = "slice_edge_graph", signature = (cell, value=None))]
        pub fn slice_edge_graph<'py>(py: Python<'py>, cell: PyCell6d, value: Option<u8>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::slice_edge_graph(&cell, value);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the network of void triangles joined by shared edges, the pore network of the slice.
        #[pyfunction]
        #[pyo3(name = "slice_tunnel_graph", signature = (cell))]
        pub fn slice_tunnel_graph<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::slice_tunnel_graph(&cell);
            (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// The three corners of the south-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "south", signature = (x, y))]
        pub fn south<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::six::south(x, y);
            (out).into_bound_py_any(py)
        }

        /// Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states.
        #[pyfunction]
        #[pyo3(name = "spectral_exponent", signature = (cell, window))]
        pub fn spectral_exponent<'py>(py: Python<'py>, cell: PyCell6d, window: f64) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::spectral_exponent(&cell, window);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given.
        #[pyfunction]
        #[pyo3(name = "svg", signature = (cell, scale, outline, width, start=None))]
        pub fn svg<'py>(py: Python<'py>, cell: PyCell6d, scale: usize, outline: Option<PyColor>, width: usize, start: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let out = mrlyrs::math::six::svg(&cell, scale, outline, width, start);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included.
        #[pyfunction]
        #[pyo3(name = "tessellate", signature = (cell, mask))]
        pub fn tessellate<'py>(py: Python<'py>, cell: PyCell6d, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrlyrs::math::six::tessellate(&cell, &mask);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Tessellates a hexagonal cell over a full width-by-height mask.
        #[pyfunction]
        #[pyo3(name = "tile", signature = (cell, width, height))]
        pub fn tile<'py>(py: Python<'py>, cell: PyCell6d, width: usize, height: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::tile(&cell, width, height);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request.
        #[pyfunction]
        #[pyo3(name = "tile_cell", signature = (cell, width, height, crop))]
        pub fn tile_cell<'py>(py: Python<'py>, cell: PyCell6d, width: usize, height: usize, crop: bool) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::tile_cell(&cell, width, height, crop);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Crops one interlocking step off each side of a sheet tiled at the given size.
        #[pyfunction]
        #[pyo3(name = "tile_crop", signature = (cell, size))]
        pub fn tile_crop<'py>(py: Python<'py>, cell: PyCell2d, size: (usize, usize)) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::tile_crop(&cell, size);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped.
        #[pyfunction]
        #[pyo3(name = "tile_step", signature = (size))]
        pub fn tile_step<'py>(py: Python<'py>, size: (usize, usize)) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::six::tile_step(size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Serializes a cell and its projection metadata to JSON.
        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        /// Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override.
        #[pyfunction]
        #[pyo3(name = "triangles", signature = (cell, start=None))]
        pub fn triangles<'py>(py: Python<'py>, cell: PyCell6d, start: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::triangles(&cell, start);
            ((ok(out)?).into_iter().map(|x| { let t = x; (t.0, PyRgba(t.1)) }).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// The three corners of the west-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "west", signature = (x, y))]
        pub fn west<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::six::west(x, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the grid width in triangles.
        #[pyfunction]
        #[pyo3(name = "width", signature = (cell))]
        pub fn width<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::six::Cell6d::width(&cell);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.six")?;
            m.setattr("__doc__", "The hexagon world: cubes flattened to triangle-meshed hexes.\nThe hexagon world, the projection of `three`.\n\nA coded cube flattened to an iso, pro or cut hexagon, meshed into triangles, then counted,\ngraphed, rastered and drawn like any other cell.")?;
            m.add_function(wrap_pyfunction!(anti, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize_otsu, &m)?)?;
            m.add_function(wrap_pyfunction!(blank, &m)?)?;
            m.add_function(wrap_pyfunction!(blur, &m)?)?;
            m.add_function(wrap_pyfunction!(census, &m)?)?;
            m.add_function(wrap_pyfunction!(components, &m)?)?;
            m.add_function(wrap_pyfunction!(cut, &m)?)?;
            m.add_function(wrap_pyfunction!(cut_design, &m)?)?;
            m.add_function(wrap_pyfunction!(east, &m)?)?;
            m.add_function(wrap_pyfunction!(euler, &m)?)?;
            m.add_function(wrap_pyfunction!(fills, &m)?)?;
            m.add_function(wrap_pyfunction!(fills_only, &m)?)?;
            m.add_function(wrap_pyfunction!(framed, &m)?)?;
            m.add_function(wrap_pyfunction!(from_json, &m)?)?;
            m.add_function(wrap_pyfunction!(giant, &m)?)?;
            m.add_function(wrap_pyfunction!(giant_network, &m)?)?;
            m.add_function(wrap_pyfunction!(height, &m)?)?;
            m.add_function(wrap_pyfunction!(holes, &m)?)?;
            m.add_function(wrap_pyfunction!(is_cube, &m)?)?;
            m.add_function(wrap_pyfunction!(is_hex, &m)?)?;
            m.add_function(wrap_pyfunction!(iso, &m)?)?;
            m.add_function(wrap_pyfunction!(iso_design, &m)?)?;
            m.add_function(wrap_pyfunction!(new, &m)?)?;
            m.add_function(wrap_pyfunction!(north, &m)?)?;
            m.add_function(wrap_pyfunction!(orientation, &m)?)?;
            m.add_function(wrap_pyfunction!(pad, &m)?)?;
            m.add_function(wrap_pyfunction!(paint, &m)?)?;
            m.add_function(wrap_pyfunction!(perforate, &m)?)?;
            m.add_function(wrap_pyfunction!(png, &m)?)?;
            m.add_function(wrap_pyfunction!(pro, &m)?)?;
            m.add_function(wrap_pyfunction!(pro_design, &m)?)?;
            m.add_function(wrap_pyfunction!(radial, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_crop, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_mask, &m)?)?;
            m.add_function(wrap_pyfunction!(raster, &m)?)?;
            m.add_function(wrap_pyfunction!(rect_png, &m)?)?;
            m.add_function(wrap_pyfunction!(rect_svg, &m)?)?;
            m.add_function(wrap_pyfunction!(rim_holes, &m)?)?;
            m.add_function(wrap_pyfunction!(skin, &m)?)?;
            m.add_function(wrap_pyfunction!(slice_core_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(slice_dual_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(slice_edge_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(slice_tunnel_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(south, &m)?)?;
            m.add_function(wrap_pyfunction!(spectral_exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(svg, &m)?)?;
            m.add_function(wrap_pyfunction!(tessellate, &m)?)?;
            m.add_function(wrap_pyfunction!(tile, &m)?)?;
            m.add_function(wrap_pyfunction!(tile_cell, &m)?)?;
            m.add_function(wrap_pyfunction!(tile_crop, &m)?)?;
            m.add_function(wrap_pyfunction!(tile_step, &m)?)?;
            m.add_function(wrap_pyfunction!(to_json, &m)?)?;
            m.add_function(wrap_pyfunction!(triangles, &m)?)?;
            m.add_function(wrap_pyfunction!(west, &m)?)?;
            m.add_function(wrap_pyfunction!(width, &m)?)?;
            m.add("FILL", mrlyrs::math::six::FILL)?;
            m.add("GRID", mrlyrs::math::six::GRID)?;
            m.add("LEFT", mrlyrs::math::six::LEFT)?;
            m.add("RIGHT", mrlyrs::math::six::RIGHT)?;
            m.add("UP", mrlyrs::math::six::UP)?;
            m.add("VOID", mrlyrs::math::six::VOID)?;
            let names: Vec<&str> = vec!["anti", "binarize", "binarize_otsu", "blank", "blur", "census", "components", "cut", "cut_design", "east", "euler", "fills", "fills_only", "framed", "from_json", "giant", "giant_network", "height", "holes", "is_cube", "is_hex", "iso", "iso_design", "new", "north", "orientation", "pad", "paint", "perforate", "png", "pro", "pro_design", "radial", "radial_crop", "radial_mask", "raster", "rect_png", "rect_svg", "rim_holes", "skin", "slice_core_graph", "slice_dual_graph", "slice_edge_graph", "slice_tunnel_graph", "south", "spectral_exponent", "svg", "tessellate", "tile", "tile_cell", "tile_crop", "tile_step", "to_json", "triangles", "west", "width", "FILL", "GRID", "LEFT", "RIGHT", "UP", "VOID"];
            m.add("__all__", names)?;
            star::init(py, &m, sys)?;
            parent.add("six", &m)?;
            sys.set_item("mrlypy._mrlypy.math.six", &m)?;
            Ok(())
        }
    }

    /// The symmetric eigensolver and the Laplacian spectra it reads off a network.
    pub mod spectrum {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size.
        #[pyfunction]
        #[pyo3(name = "clusters", signature = (eigenvalues, tolerance))]
        pub fn clusters<'py>(py: Python<'py>, eigenvalues: Vec<f64>, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::clusters(&eigenvalues, tolerance);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`.
        #[pyfunction]
        #[pyo3(name = "laplacian", signature = (network, normalised))]
        pub fn laplacian<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>, normalised: bool) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::laplacian(&network.0, normalised);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the ascending Laplacian spectrum of a network, combinatorial or normalised.
        #[pyfunction]
        #[pyo3(name = "laplacian_spectrum", signature = (network, normalised))]
        pub fn laplacian_spectrum<'py>(py: Python<'py>, network: PyRef<'_, crate::gen::math::graph::Network>, normalised: bool) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::laplacian_spectrum(&network.0, normalised);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Counts the eigenvalues within the tolerance of a value.
        #[pyfunction]
        #[pyo3(name = "multiplicity", signature = (eigenvalues, value, tolerance))]
        pub fn multiplicity<'py>(py: Python<'py>, eigenvalues: Vec<f64>, value: f64, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::multiplicity(&eigenvalues, value, tolerance);
            (out).into_bound_py_any(py)
        }

        /// Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window.
        #[pyfunction]
        #[pyo3(name = "spectral_exponent", signature = (eigenvalues, window))]
        pub fn spectral_exponent<'py>(py: Python<'py>, eigenvalues: Vec<f64>, window: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::spectral_exponent(&eigenvalues, window);
            (out).into_bound_py_any(py)
        }

        /// Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count.
        #[pyfunction]
        #[pyo3(name = "spectral_fit", signature = (eigenvalues, window))]
        pub fn spectral_fit<'py>(py: Python<'py>, eigenvalues: Vec<f64>, window: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::spectral_fit(&eigenvalues, window);
            (out).into_bound_py_any(py)
        }

        /// Builds the integrated density of states as points, each an eigenvalue and its rank fraction.
        #[pyfunction]
        #[pyo3(name = "spectral_points", signature = (eigenvalues))]
        pub fn spectral_points<'py>(py: Python<'py>, eigenvalues: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::spectral_points(&eigenvalues);
            (out).into_bound_py_any(py)
        }

        /// Returns the eigenvalues of a dense real symmetric matrix in ascending order.
        #[pyfunction]
        #[pyo3(name = "symmetric_eigenvalues", signature = (matrix))]
        pub fn symmetric_eigenvalues<'py>(py: Python<'py>, matrix: Vec<Vec<f64>>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spectrum::symmetric_eigenvalues(&matrix);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.spectrum")?;
            m.setattr("__doc__", "The symmetric eigensolver and the Laplacian spectra it reads off a network.")?;
            m.add_function(wrap_pyfunction!(clusters, &m)?)?;
            m.add_function(wrap_pyfunction!(laplacian, &m)?)?;
            m.add_function(wrap_pyfunction!(laplacian_spectrum, &m)?)?;
            m.add_function(wrap_pyfunction!(multiplicity, &m)?)?;
            m.add_function(wrap_pyfunction!(spectral_exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(spectral_fit, &m)?)?;
            m.add_function(wrap_pyfunction!(spectral_points, &m)?)?;
            m.add_function(wrap_pyfunction!(symmetric_eigenvalues, &m)?)?;
            let names: Vec<&str> = vec!["clusters", "laplacian", "laplacian_spectrum", "multiplicity", "spectral_exponent", "spectral_fit", "spectral_points", "symmetric_eigenvalues"];
            m.add("__all__", names)?;
            parent.add("spectrum", &m)?;
            sys.set_item("mrlypy._mrlypy.math.spectrum", &m)?;
            Ok(())
        }
    }

    /// The turntable: the exact circle means of a raster about its centre, the profile they trace and the wheel it paints.
    pub mod spin {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The way radial copies merge: their mean, their sum, their union, their meet, their parity or what the first keeps that no other has.
        #[pyclass(name = "Blend", module = "mrlypy.math.spin", skip_from_py_object)]
        pub struct Blend;

        #[pymethods]
        impl Blend {
            /// Merges one site's copies into the blended value.
            #[staticmethod]
            #[pyo3(name = "fold", signature = (blend, values))]
            pub fn fold<'py>(py: Python<'py>, blend: PySerde<mrlyrs::math::spin::Blend>, values: Vec<f32>) -> PyResult<Bound<'py, PyAny>> {
                let blend = blend.0;
                let out = mrlyrs::math::spin::Blend::fold(blend, &values);
                (out).into_bound_py_any(py)
            }
            /// Reads a blend by name: mean, sum, union, meet, parity or difference.
            #[staticmethod]
            #[pyo3(name = "named", signature = (name))]
            pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::spin::Blend::named(name);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
        }

        /// The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside.
        #[pyfunction]
        #[pyo3(name = "arcs", signature = (data, size, radius))]
        pub fn arcs<'py>(py: Python<'py>, data: Vec<f32>, size: usize, radius: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::arcs(&data, size, radius);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs.
        #[pyfunction]
        #[pyo3(name = "harmonics", signature = (data, size, rings, orders))]
        pub fn harmonics<'py>(py: Python<'py>, data: Vec<f32>, size: usize, rings: usize, orders: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::harmonics(&data, size, rings, orders);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from.
        #[pyfunction]
        #[pyo3(name = "mass", signature = (profile, size))]
        pub fn mass<'py>(py: Python<'py>, profile: Vec<f32>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::mass(&profile, size);
            (out).into_bound_py_any(py)
        }

        /// The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from.
        #[pyfunction]
        #[pyo3(name = "mass_within", signature = (profile, size, radius))]
        pub fn mass_within<'py>(py: Python<'py>, profile: Vec<f32>, size: usize, radius: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::mass_within(&profile, size, radius);
            (out).into_bound_py_any(py)
        }

        /// The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.
        #[pyfunction]
        #[pyo3(name = "petals", signature = (copies, order))]
        pub fn petals<'py>(py: Python<'py>, copies: usize, order: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::petals(copies, order);
            (out).into_bound_py_any(py)
        }

        /// The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle.
        #[pyfunction]
        #[pyo3(name = "profile", signature = (data, size, steps))]
        pub fn profile<'py>(py: Python<'py>, data: Vec<f32>, size: usize, steps: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::profile(&data, size, steps);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points.
        #[pyfunction]
        #[pyo3(name = "radial", signature = (data, size, out, copies, step, blend, samples))]
        pub fn radial<'py>(py: Python<'py>, data: Vec<f32>, size: usize, out: usize, copies: usize, step: f64, blend: PySerde<mrlyrs::math::spin::Blend>, samples: usize) -> PyResult<Bound<'py, PyAny>> {
            let blend = blend.0;
            let out = mrlyrs::math::spin::radial(&data, size, out, copies, step, blend, samples);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The radius of the corner circle of a square raster of the side, the last radius a profile reads.
        #[pyfunction]
        #[pyo3(name = "reach", signature = (size))]
        pub fn reach<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::reach(size);
            (out).into_bound_py_any(py)
        }

        /// The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero.
        #[pyfunction]
        #[pyo3(name = "ring", signature = (data, size, radius))]
        pub fn ring<'py>(py: Python<'py>, data: Vec<f32>, size: usize, radius: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::ring(&data, size, radius);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does.
        #[pyfunction]
        #[pyo3(name = "turns", signature = (power))]
        pub fn turns<'py>(py: Python<'py>, power: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::turns(&power);
            (out).into_bound_py_any(py)
        }

        /// The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius.
        #[pyfunction]
        #[pyo3(name = "wheel", signature = (profile, size))]
        pub fn wheel<'py>(py: Python<'py>, profile: Vec<f32>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spin::wheel(&profile, size);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.spin")?;
            m.setattr("__doc__", "The turntable: the exact circle means of a raster about its centre, the profile they trace and the wheel it paints.")?;
            m.add_class::<Blend>()?;
            m.add_function(wrap_pyfunction!(arcs, &m)?)?;
            m.add_function(wrap_pyfunction!(harmonics, &m)?)?;
            m.add_function(wrap_pyfunction!(mass, &m)?)?;
            m.add_function(wrap_pyfunction!(mass_within, &m)?)?;
            m.add_function(wrap_pyfunction!(petals, &m)?)?;
            m.add_function(wrap_pyfunction!(profile, &m)?)?;
            m.add_function(wrap_pyfunction!(radial, &m)?)?;
            m.add_function(wrap_pyfunction!(reach, &m)?)?;
            m.add_function(wrap_pyfunction!(ring, &m)?)?;
            m.add_function(wrap_pyfunction!(turns, &m)?)?;
            m.add_function(wrap_pyfunction!(wheel, &m)?)?;
            let names: Vec<&str> = vec!["arcs", "harmonics", "mass", "mass_within", "petals", "profile", "radial", "reach", "ring", "turns", "wheel", "Blend"];
            m.add("__all__", names)?;
            parent.add("spin", &m)?;
            sys.set_item("mrlypy._mrlypy.math.spin", &m)?;
            Ok(())
        }
    }

    /// The spirograph: a byte grid as a wheel with a pencil in every cell, rolled on a line, a circle or a polygon, and the curves it draws.
    pub mod spirograph {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel.
        #[pyfunction]
        #[pyo3(name = "cell", signature = (width, height, reach))]
        pub fn cell<'py>(py: Python<'py>, width: usize, height: usize, reach: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spirograph::cell(width, height, reach);
            (out).into_bound_py_any(py)
        }

        /// The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area.
        #[pyfunction]
        #[pyo3(name = "cover", signature = (track, pencils, exact, samples, side))]
        pub fn cover<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>, exact: bool, samples: usize, side: usize) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::cover(&track, &pencils, exact, samples, side);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall.
        #[pyfunction]
        #[pyo3(name = "disc", signature = (track, pencils))]
        pub fn disc<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::disc(&track, &pencils);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter.
        #[pyfunction]
        #[pyo3(name = "distinct", signature = (track, pencils, exact))]
        pub fn distinct<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>, exact: bool) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::distinct(&track, &pencils, exact);
            (out).into_bound_py_any(py)
        }

        /// The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further.
        #[pyfunction]
        #[pyo3(name = "frame", signature = (track, pencils))]
        pub fn frame<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::frame(&track, &pencils);
            (out).into_bound_py_any(py)
        }

        /// The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach.
        #[pyfunction]
        #[pyo3(name = "nodes", signature = (track, pencils, exact))]
        pub fn nodes<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>, exact: bool) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::nodes(&track, &pencils, exact);
            (out).into_bound_py_any(py)
        }

        /// Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded.
        #[pyfunction]
        #[pyo3(name = "pencils", signature = (types, width, height, mode, reach, jitter, seed))]
        pub fn pencils<'py>(py: Python<'py>, types: Vec<u8>, width: usize, height: usize, mode: &str, reach: f64, jitter: f64, seed: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spirograph::pencils(&types, width, height, mode, reach, jitter, seed);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel.
        #[pyfunction]
        #[pyo3(name = "point", signature = (track, pencil, s))]
        pub fn point<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencil: PySerde<mrlyrs::math::spirograph::Pencil>, s: f64) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencil = pencil.0;
            let out = mrlyrs::math::spirograph::point(&track, &pencil, s);
            (out).into_bound_py_any(py)
        }

        /// The wheel's centre after `s` of path length.
        #[pyfunction]
        #[pyo3(name = "pose", signature = (track, s))]
        pub fn pose<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, s: f64) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let out = mrlyrs::math::spirograph::pose(&track, s);
            (out).into_bound_py_any(py)
        }

        /// One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter.
        #[pyfunction]
        #[pyo3(name = "representatives", signature = (track, pencils, exact))]
        pub fn representatives<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>, exact: bool) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::representatives(&track, &pencils, exact);
            (out).into_bound_py_any(py)
        }

        /// Counts the pencils by kind.
        #[pyfunction]
        #[pyo3(name = "seats", signature = (pencils))]
        pub fn seats<'py>(py: Python<'py>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>) -> PyResult<Bound<'py, PyAny>> {
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::seats(&pencils);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close.
        #[pyfunction]
        #[pyo3(name = "signed_area", signature = (track, pencil))]
        pub fn signed_area<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencil: PySerde<mrlyrs::math::spirograph::Pencil>) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencil = pencil.0;
            let out = mrlyrs::math::spirograph::signed_area(&track, &pencil);
            (out).into_bound_py_any(py)
        }

        /// Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y.
        #[pyfunction]
        #[pyo3(name = "trace", signature = (track, pencils, samples))]
        pub fn trace<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, pencils: Vec<PySerde<mrlyrs::math::spirograph::Pencil>>, samples: usize) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::spirograph::trace(&track, &pencils, samples);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps.
        #[pyfunction]
        #[pyo3(name = "track", signature = (kind, ring, wheel, sides, laps))]
        pub fn track<'py>(py: Python<'py>, kind: &str, ring: usize, wheel: usize, sides: usize, laps: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::spirograph::track(kind, ring, wheel, sides, laps);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius.
        #[pyfunction]
        #[pyo3(name = "turn", signature = (track, s))]
        pub fn turn<'py>(py: Python<'py>, track: PySerde<mrlyrs::math::spirograph::Track>, s: f64) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let out = mrlyrs::math::spirograph::turn(&track, s);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.spirograph")?;
            m.setattr("__doc__", "The spirograph: a byte grid as a wheel with a pencil in every cell, rolled on a line, a circle or a polygon, and the curves it draws.")?;
            m.add_function(wrap_pyfunction!(cell, &m)?)?;
            m.add_function(wrap_pyfunction!(cover, &m)?)?;
            m.add_function(wrap_pyfunction!(disc, &m)?)?;
            m.add_function(wrap_pyfunction!(distinct, &m)?)?;
            m.add_function(wrap_pyfunction!(frame, &m)?)?;
            m.add_function(wrap_pyfunction!(nodes, &m)?)?;
            m.add_function(wrap_pyfunction!(pencils, &m)?)?;
            m.add_function(wrap_pyfunction!(point, &m)?)?;
            m.add_function(wrap_pyfunction!(pose, &m)?)?;
            m.add_function(wrap_pyfunction!(representatives, &m)?)?;
            m.add_function(wrap_pyfunction!(seats, &m)?)?;
            m.add_function(wrap_pyfunction!(signed_area, &m)?)?;
            m.add_function(wrap_pyfunction!(trace, &m)?)?;
            m.add_function(wrap_pyfunction!(track, &m)?)?;
            m.add_function(wrap_pyfunction!(turn, &m)?)?;
            m.add("LAPS_CAP", mrlyrs::math::spirograph::LAPS_CAP)?;
            m.add("PENCIL_CAP", mrlyrs::math::spirograph::PENCIL_CAP)?;
            m.add("POINT_CAP", mrlyrs::math::spirograph::POINT_CAP)?;
            m.add("RADIUS_CAP", mrlyrs::math::spirograph::RADIUS_CAP)?;
            m.add("RASTER_CAP", mrlyrs::math::spirograph::RASTER_CAP)?;
            m.add("SIDES", mrlyrs::math::spirograph::SIDES)?;
            let names: Vec<&str> = vec!["cell", "cover", "disc", "distinct", "frame", "nodes", "pencils", "point", "pose", "representatives", "seats", "signed_area", "trace", "track", "turn", "LAPS_CAP", "PENCIL_CAP", "POINT_CAP", "RADIUS_CAP", "RASTER_CAP", "SIDES"];
            m.add("__all__", names)?;
            parent.add("spirograph", &m)?;
            sys.set_item("mrlypy._mrlypy.math.spirograph", &m)?;
            Ok(())
        }
    }

    /// The cube pipeline: designs, tiles, graphs and renderings in three dimensions.
    /// The cube pipeline.
    pub mod three {
        use crate::hand::{ok, PyCell2d, PyCell3d, PyCellNd, PyCode, PyRng, PySerde, PyTensor};
        use pyo3::exceptions::PyValueError;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A three-component vector of f32.
        #[pyclass(name = "Vec3", module = "mrlypy.math.three", from_py_object)]
        #[derive(Clone)]
        pub struct Vec3(pub mrlyrs::math::three::Vec3);

        #[pymethods]
        impl Vec3 {
            /// Builds a vector from its components.
            #[new]
            #[pyo3(signature = (x, y, z))]
            pub fn __new__(x: f32, y: f32, z: f32) -> PyResult<Self> {
                let out = mrlyrs::math::three::Vec3::new(x, y, z);
                Ok(Self(out))
            }
            /// The x component.
            #[getter]
            #[pyo3(name = "x")]
            pub fn x<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.x;
                (value).into_bound_py_any(py)
            }
            /// The y component.
            #[getter]
            #[pyo3(name = "y")]
            pub fn y<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.y;
                (value).into_bound_py_any(py)
            }
            /// The z component.
            #[getter]
            #[pyo3(name = "z")]
            pub fn z<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.z;
                (value).into_bound_py_any(py)
            }
            /// Returns the cross product, perpendicular to both vectors.
            #[pyo3(name = "cross", signature = (o))]
            pub fn cross<'py>(&self, py: Python<'py>, o: crate::gen::math::three::Vec3) -> PyResult<Bound<'py, PyAny>> {
                let o = o.0;
                let out = mrlyrs::math::three::Vec3::cross(self.0, o);
                (crate::gen::math::three::Vec3(out)).into_bound_py_any(py)
            }
            /// Returns the dot product of the two vectors.
            #[pyo3(name = "dot", signature = (o))]
            pub fn dot<'py>(&self, py: Python<'py>, o: crate::gen::math::three::Vec3) -> PyResult<Bound<'py, PyAny>> {
                let o = o.0;
                let out = mrlyrs::math::three::Vec3::dot(self.0, o);
                (out).into_bound_py_any(py)
            }
            /// Builds a vector from its components.
            #[staticmethod]
            #[pyo3(name = "new", signature = (x, y, z))]
            pub fn new_<'py>(py: Python<'py>, x: f32, y: f32, z: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::three::Vec3::new(x, y, z);
                (crate::gen::math::three::Vec3(out)).into_bound_py_any(py)
            }
            /// Multiplies every component by the scalar.
            #[pyo3(name = "scale", signature = (s))]
            pub fn scale<'py>(&self, py: Python<'py>, s: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::three::Vec3::scale(self.0, s);
                (crate::gen::math::three::Vec3(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Builds the Menger sponge, filled where at most one coordinate is odd, at the given level.
        #[pyfunction]
        #[pyo3(name = "carpet", signature = (number, level))]
        pub fn carpet<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::carpet(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading.
        #[pyfunction]
        #[pyo3(name = "census", signature = (cell))]
        pub fn census<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::census(&cell);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Extracts the network of filled sites joined to their axis neighbors.
        #[pyfunction]
        #[pyo3(name = "core_graph", signature = (cell))]
        pub fn core_graph<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let out = mrlyrs::math::three::core_graph::<2>(&cell);
                    (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let out = mrlyrs::math::three::core_graph::<3>(&cell);
                    (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!("core_graph wants a 2d or 3d argument, got {other}d."))),
            }
        }

        /// Builds the cube the universe code names, deepened to the given fractal level.
        #[pyfunction]
        #[pyo3(name = "create", signature = (code, number, level, base))]
        pub fn create<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::three::create(code, number, level, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples.
        #[pyfunction]
        #[pyo3(name = "diagonal_slice", signature = (code, number, level, base, height))]
        pub fn diagonal_slice<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, base: usize, height: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::three::diagonal_slice(code, number, level, base, height);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner.
        #[pyfunction]
        #[pyo3(name = "diagonal_svg", signature = (code, number, level, base, heights, scale))]
        pub fn diagonal_svg<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, base: usize, heights: Vec<usize>, scale: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::three::diagonal_svg(code, number, level, base, &heights, scale);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Builds the dust cube, filled where every coordinate is even, at the given level.
        #[pyfunction]
        #[pyo3(name = "dust", signature = (number, level))]
        pub fn dust<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::dust(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Extracts the network of corners and edges outlining every filled site.
        #[pyfunction]
        #[pyo3(name = "edge_graph", signature = (cell))]
        pub fn edge_graph<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let out = mrlyrs::math::three::edge_graph::<2>(&cell);
                    (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let out = mrlyrs::math::three::edge_graph::<3>(&cell);
                    (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!("edge_graph wants a 2d or 3d argument, got {other}d."))),
            }
        }

        /// Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites.
        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell))]
        pub fn euler<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::euler(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it.
        #[pyfunction]
        #[pyo3(name = "extrude", signature = (cell, axis, depth))]
        pub fn extrude<'py>(py: Python<'py>, cell: PyCell2d, axis: usize, depth: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::extrude(&cell, axis, depth);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Repeats every plane of a cube depth times along its leading axis, colors and tags with it.
        #[pyfunction]
        #[pyo3(name = "extrude_cube", signature = (cell, depth))]
        pub fn extrude_cube<'py>(py: Python<'py>, cell: PyCell3d, depth: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::extrude_cube(&cell, depth);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the count of unit faces the filled sites touch, a face shared by two sites counted once.
        #[pyfunction]
        #[pyo3(name = "faces", signature = (cell))]
        pub fn faces<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::faces(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of filled sites.
        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::fills(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds a cube from its corner patterns, deepened to the given fractal level.
        #[pyfunction]
        #[pyo3(name = "from_corners", signature = (corners, number, level, base))]
        pub fn from_corners<'py>(py: Python<'py>, corners: Vec<Vec<u8>>, number: usize, level: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::from_corners(&corners, number, level, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Parses a cell from its JSON, colors and tags included.
        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::from_json(text);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a cube from one string of digits per row, grouped plane by plane.
        #[pyfunction]
        #[pyo3(name = "from_strings", signature = (data))]
        pub fn from_strings<'py>(py: Python<'py>, data: Vec<Vec<String>>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::from_strings(&data);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the count of faces buried between two filled sites, six per site less the exposed surface.
        #[pyfunction]
        #[pyo3(name = "hidden", signature = (cell))]
        pub fn hidden<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::hidden(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the cube filled wherever the residue sum lands in the levels, at the given level.
        #[pyfunction]
        #[pyo3(name = "level_set", signature = (number, levels, level, base))]
        pub fn level_set<'py>(py: Python<'py>, number: usize, levels: Vec<usize>, level: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::level_set(number, &levels, level, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Folds two or more cells into one by chained Kronecker combination.
        #[pyfunction]
        #[pyo3(name = "magic", signature = (cells))]
        pub fn magic<'py>(py: Python<'py>, cells: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cells)?;
            match dim {
                2 => {
                    let cells = cells.extract::<Vec<PyCell2d>>()?;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrlyrs::math::three::magic::<2>(&cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cells = cells.extract::<Vec<PyCell3d>>()?;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrlyrs::math::three::magic::<3>(&cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!("magic wants a 2d or 3d argument, got {other}d."))),
            }
        }

        /// Tags every site with its Manhattan distance from the cube's center, the diamond shells.
        #[pyfunction]
        #[pyo3(name = "manhattan_layers", signature = (cell))]
        pub fn manhattan_layers<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::manhattan_layers(cell);
            (PyCellNd(out)).into_bound_py_any(py)
        }

        /// Merges the cells into one cube arranged width by height by depth.
        #[pyfunction]
        #[pyo3(name = "merge", signature = (cells, width, height, depth))]
        pub fn merge<'py>(py: Python<'py>, cells: Vec<PyCell3d>, width: usize, height: usize, depth: usize) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::three::merge(&cells, width, height, depth);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a cell by placing at each mask site the cell its value indexes.
        #[pyfunction]
        #[pyo3(name = "mosaic", signature = (mask, cells))]
        pub fn mosaic<'py>(py: Python<'py>, mask: PyTensor, cells: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cells)?;
            match dim {
                2 => {
                    let cells = cells.extract::<Vec<PyCell2d>>()?;
                    let mask = mask.0;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrlyrs::math::three::mosaic::<2>(&mask, &cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cells = cells.extract::<Vec<PyCell3d>>()?;
                    let mask = mask.0;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrlyrs::math::three::mosaic::<3>(&mask, &cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!("mosaic wants a 2d or 3d argument, got {other}d."))),
            }
        }

        /// Builds the cube the name picks, deepened to the given fractal level.
        #[pyfunction]
        #[pyo3(name = "named", signature = (design, number, level))]
        pub fn named<'py>(py: Python<'py>, design: PySerde<mrlyrs::gen::recipe::Design>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrlyrs::math::three::named(design, number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the net cube, filled where at least two coordinates are odd, at the given level.
        #[pyfunction]
        #[pyo3(name = "net", signature = (number, level))]
        pub fn net<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::net(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a cube whose every site turns on with probability density, at the given level.
        #[pyfunction]
        #[pyo3(name = "noise", signature = (number, level, density, rng))]
        pub fn noise<'py>(py: Python<'py>, number: usize, level: usize, density: f64, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::noise(number, level, density, &mut rng.0);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the solid cube at the given size and level.
        #[pyfunction]
        #[pyo3(name = "ones", signature = (number, level))]
        pub fn ones<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::ones(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the 24 rotation triples that reach each distinct cube orientation.
        #[pyfunction]
        #[pyo3(name = "orientations", signature = ())]
        pub fn orientations<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::orientations();
            (out).into_bound_py_any(py)
        }

        /// Builds the point cube, filled where every coordinate is odd, at the given level.
        #[pyfunction]
        #[pyo3(name = "point", signature = (number, level))]
        pub fn point<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::point(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`.
        #[pyfunction]
        #[pyo3(name = "profile", signature = (code, number, level, base))]
        pub fn profile<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::three::profile(code, number, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`.
        #[pyfunction]
        #[pyo3(name = "project", signature = (point))]
        pub fn project<'py>(py: Python<'py>, point: [u32; 3]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::project(point);
            (out).into_bound_py_any(py)
        }

        /// Returns one outward quad per exposed face, scaled into the unit box.
        #[pyfunction]
        #[pyo3(name = "quads", signature = (cell))]
        pub fn quads<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::quads(&cell);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped.
        #[pyfunction]
        #[pyo3(name = "shadow", signature = (point))]
        pub fn shadow<'py>(py: Python<'py>, point: [u32; 3]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::shadow(point);
            (out).into_bound_py_any(py)
        }

        /// Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it.
        #[pyfunction]
        #[pyo3(name = "slice", signature = (cell, axis, index))]
        pub fn slice<'py>(py: Python<'py>, cell: PyCell3d, axis: usize, index: usize) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::slice(&cell, axis, index);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Orients a copy of the cell by each mask value and merges them in the mask's shape.
        #[pyfunction]
        #[pyo3(name = "special", signature = (mask, cell))]
        pub fn special<'py>(py: Python<'py>, mask: PyTensor, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let cell = cell.0;
            let out = mrlyrs::math::three::special(&mask, &cell);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the star cube, filled where exactly one coordinate is odd, at the given level.
        #[pyfunction]
        #[pyo3(name = "star", signature = (number, level))]
        pub fn star<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::star(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the first and last height a profile fills, or none when the design is empty.
        #[pyfunction]
        #[pyo3(name = "support", signature = (counts))]
        pub fn support<'py>(py: Python<'py>, counts: Vec<u128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::support(&counts);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of filled faces exposed to void or the outside.
        #[pyfunction]
        #[pyo3(name = "surface", signature = (cell))]
        pub fn surface<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::surface(&cell);
            (out).into_bound_py_any(py)
        }

        /// Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped.
        #[pyfunction]
        #[pyo3(name = "text", signature = (cell, glyphs=None))]
        pub fn text<'py>(py: Python<'py>, cell: PyCell3d, glyphs: Option<std::collections::HashMap<u8, String>>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let glyphs = glyphs.map(|x| x.into_iter().collect());
            let out = mrlyrs::math::three::text(&cell, glyphs.as_ref());
            (out).into_bound_py_any(py)
        }

        /// Serializes the cell's shape and types to JSON, with colors and tags when present.
        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        /// Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner.
        #[pyfunction]
        #[pyo3(name = "to_obj", signature = (cell))]
        pub fn to_obj<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::to_obj(&cell);
            (out).into_bound_py_any(py)
        }

        /// Unrolls the cube into one string of digits per row, grouped plane by plane.
        #[pyfunction]
        #[pyo3(name = "to_strings", signature = (cell))]
        pub fn to_strings<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::to_strings(&cell);
            (out).into_bound_py_any(py)
        }

        /// Extracts the network of empty sites joined to their axis neighbors.
        #[pyfunction]
        #[pyo3(name = "tunnel_graph", signature = (cell))]
        pub fn tunnel_graph<'py>(py: Python<'py>, cell: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let out = mrlyrs::math::three::tunnel_graph::<2>(&cell);
                    (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let out = mrlyrs::math::three::tunnel_graph::<3>(&cell);
                    (crate::gen::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!("tunnel_graph wants a 2d or 3d argument, got {other}d."))),
            }
        }

        /// Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.
        #[pyfunction]
        #[pyo3(name = "void", signature = (number, level))]
        pub fn void<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::void(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the count of empty sites.
        #[pyfunction]
        #[pyo3(name = "voids", signature = (cell))]
        pub fn voids<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::voids(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns the filled-site count, the cube's volume.
        #[pyfunction]
        #[pyo3(name = "volume", signature = (cell))]
        pub fn volume<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::volume(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns the cell's edge-graph segments, scaled into the unit box.
        #[pyfunction]
        #[pyo3(name = "wires", signature = (cell))]
        pub fn wires<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::three::wires(&cell);
            ((out).into_iter().map(|x| (x).into_iter().map(crate::gen::math::three::Vec3).collect::<Vec<_>>()).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Builds the cube of rods along the x axis at the given size and level.
        #[pyfunction]
        #[pyo3(name = "xline", signature = (number, level))]
        pub fn xline<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::xline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the cube of beams along the x axis at the given size and level.
        #[pyfunction]
        #[pyo3(name = "xtree", signature = (number, level))]
        pub fn xtree<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::xtree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the cube of rods along the y axis at the given size and level.
        #[pyfunction]
        #[pyo3(name = "yline", signature = (number, level))]
        pub fn yline<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::yline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the cube of beams along the y axis at the given size and level.
        #[pyfunction]
        #[pyo3(name = "ytree", signature = (number, level))]
        pub fn ytree<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::ytree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the all-void cube at the given size and level.
        #[pyfunction]
        #[pyo3(name = "zeros", signature = (number, level))]
        pub fn zeros<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::zeros(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the cube of rods along the z axis at the given size and level.
        #[pyfunction]
        #[pyo3(name = "zline", signature = (number, level))]
        pub fn zline<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::zline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the cube of beams along the z axis at the given size and level.
        #[pyfunction]
        #[pyo3(name = "ztree", signature = (number, level))]
        pub fn ztree<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::three::ztree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.three")?;
            m.setattr("__doc__", "The cube pipeline: designs, tiles, graphs and renderings in three dimensions.\nThe cube pipeline.\n\nThe shared cell pipeline pinned to three dimensions: coded and carpet cubes, their censuses,\ntheir diagonal slices, their exposed quads and their JSON.")?;
            m.add_class::<Vec3>()?;
            m.add_function(wrap_pyfunction!(carpet, &m)?)?;
            m.add_function(wrap_pyfunction!(census, &m)?)?;
            m.add_function(wrap_pyfunction!(core_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(create, &m)?)?;
            m.add_function(wrap_pyfunction!(diagonal_slice, &m)?)?;
            m.add_function(wrap_pyfunction!(diagonal_svg, &m)?)?;
            m.add_function(wrap_pyfunction!(dust, &m)?)?;
            m.add_function(wrap_pyfunction!(edge_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(euler, &m)?)?;
            m.add_function(wrap_pyfunction!(extrude, &m)?)?;
            m.add_function(wrap_pyfunction!(extrude_cube, &m)?)?;
            m.add_function(wrap_pyfunction!(faces, &m)?)?;
            m.add_function(wrap_pyfunction!(fills, &m)?)?;
            m.add_function(wrap_pyfunction!(from_corners, &m)?)?;
            m.add_function(wrap_pyfunction!(from_json, &m)?)?;
            m.add_function(wrap_pyfunction!(from_strings, &m)?)?;
            m.add_function(wrap_pyfunction!(hidden, &m)?)?;
            m.add_function(wrap_pyfunction!(level_set, &m)?)?;
            m.add_function(wrap_pyfunction!(magic, &m)?)?;
            m.add_function(wrap_pyfunction!(manhattan_layers, &m)?)?;
            m.add_function(wrap_pyfunction!(merge, &m)?)?;
            m.add_function(wrap_pyfunction!(mosaic, &m)?)?;
            m.add_function(wrap_pyfunction!(named, &m)?)?;
            m.add_function(wrap_pyfunction!(net, &m)?)?;
            m.add_function(wrap_pyfunction!(noise, &m)?)?;
            m.add_function(wrap_pyfunction!(ones, &m)?)?;
            m.add_function(wrap_pyfunction!(orientations, &m)?)?;
            m.add_function(wrap_pyfunction!(point, &m)?)?;
            m.add_function(wrap_pyfunction!(profile, &m)?)?;
            m.add_function(wrap_pyfunction!(project, &m)?)?;
            m.add_function(wrap_pyfunction!(quads, &m)?)?;
            m.add_function(wrap_pyfunction!(shadow, &m)?)?;
            m.add_function(wrap_pyfunction!(slice, &m)?)?;
            m.add_function(wrap_pyfunction!(special, &m)?)?;
            m.add_function(wrap_pyfunction!(star, &m)?)?;
            m.add_function(wrap_pyfunction!(support, &m)?)?;
            m.add_function(wrap_pyfunction!(surface, &m)?)?;
            m.add_function(wrap_pyfunction!(text, &m)?)?;
            m.add_function(wrap_pyfunction!(to_json, &m)?)?;
            m.add_function(wrap_pyfunction!(to_obj, &m)?)?;
            m.add_function(wrap_pyfunction!(to_strings, &m)?)?;
            m.add_function(wrap_pyfunction!(tunnel_graph, &m)?)?;
            m.add_function(wrap_pyfunction!(void, &m)?)?;
            m.add_function(wrap_pyfunction!(voids, &m)?)?;
            m.add_function(wrap_pyfunction!(volume, &m)?)?;
            m.add_function(wrap_pyfunction!(wires, &m)?)?;
            m.add_function(wrap_pyfunction!(xline, &m)?)?;
            m.add_function(wrap_pyfunction!(xtree, &m)?)?;
            m.add_function(wrap_pyfunction!(yline, &m)?)?;
            m.add_function(wrap_pyfunction!(ytree, &m)?)?;
            m.add_function(wrap_pyfunction!(zeros, &m)?)?;
            m.add_function(wrap_pyfunction!(zline, &m)?)?;
            m.add_function(wrap_pyfunction!(ztree, &m)?)?;
            let names: Vec<&str> = vec!["carpet", "census", "core_graph", "create", "diagonal_slice", "diagonal_svg", "dust", "edge_graph", "euler", "extrude", "extrude_cube", "faces", "fills", "from_corners", "from_json", "from_strings", "hidden", "level_set", "magic", "manhattan_layers", "merge", "mosaic", "named", "net", "noise", "ones", "orientations", "point", "profile", "project", "quads", "shadow", "slice", "special", "star", "support", "surface", "text", "to_json", "to_obj", "to_strings", "tunnel_graph", "void", "voids", "volume", "wires", "xline", "xtree", "yline", "ytree", "zeros", "zline", "ztree", "Vec3"];
            m.add("__all__", names)?;
            parent.add("three", &m)?;
            sys.set_item("mrlypy._mrlypy.math.three", &m)?;
            Ok(())
        }
    }

    /// The tourbillon: the odd parity carpets turned one angle a layer and stacked inside the inscribed disc.
    pub mod tourbillon {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle.
        #[pyfunction]
        #[pyo3(name = "eyes", signature = (qmax))]
        pub fn eyes<'py>(py: Python<'py>, qmax: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::tourbillon::eyes(qmax);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.
        #[pyfunction]
        #[pyo3(name = "field", signature = (top, size, schedule, increment, set, weights, mode, blend, seed))]
        pub fn field<'py>(py: Python<'py>, top: usize, size: usize, schedule: &str, increment: f64, set: &str, weights: &str, mode: &str, blend: &str, seed: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::tourbillon::field(top, size, schedule, increment, set, weights, mode, blend, seed);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle.
        #[pyfunction]
        #[pyo3(name = "layers", signature = (top, schedule, increment, set, weights, seed))]
        pub fn layers<'py>(py: Python<'py>, top: usize, schedule: &str, increment: f64, set: &str, weights: &str, seed: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::tourbillon::layers(top, schedule, increment, set, weights, seed);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// The least whole number of increments that closes a quarter turn, none once the count passes the cap.
        #[pyfunction]
        #[pyo3(name = "period", signature = (increment))]
        pub fn period<'py>(py: Python<'py>, increment: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::tourbillon::period(increment);
            (out).into_bound_py_any(py)
        }

        /// The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one.
        #[pyfunction]
        #[pyo3(name = "sharing", signature = (list))]
        pub fn sharing<'py>(py: Python<'py>, list: Vec<PySerde<mrlyrs::math::tourbillon::Layer>>) -> PyResult<Bound<'py, PyAny>> {
            let list = list.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::tourbillon::sharing(&list);
            (out).into_bound_py_any(py)
        }

        /// Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site.
        #[pyfunction]
        #[pyo3(name = "stack", signature = (list, size, mode, blend))]
        pub fn stack<'py>(py: Python<'py>, list: Vec<PySerde<mrlyrs::math::tourbillon::Layer>>, size: usize, mode: &str, blend: PySerde<mrlyrs::math::spin::Blend>) -> PyResult<Bound<'py, PyAny>> {
            let list = list.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let blend = blend.0;
            let out = mrlyrs::math::tourbillon::stack(&list, size, mode, blend);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites.
        #[pyfunction]
        #[pyo3(name = "stats", signature = (field, size, top, schedule, increment, set, weights, blend, seed))]
        pub fn stats<'py>(py: Python<'py>, field: Vec<f32>, size: usize, top: usize, schedule: &str, increment: f64, set: &str, weights: &str, blend: &str, seed: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::tourbillon::stats(&field, size, top, schedule, increment, set, weights, blend, seed);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.tourbillon")?;
            m.setattr("__doc__", "The tourbillon: the odd parity carpets turned one angle a layer and stacked inside the inscribed disc.")?;
            m.add_function(wrap_pyfunction!(eyes, &m)?)?;
            m.add_function(wrap_pyfunction!(field, &m)?)?;
            m.add_function(wrap_pyfunction!(layers, &m)?)?;
            m.add_function(wrap_pyfunction!(period, &m)?)?;
            m.add_function(wrap_pyfunction!(sharing, &m)?)?;
            m.add_function(wrap_pyfunction!(stack, &m)?)?;
            m.add_function(wrap_pyfunction!(stats, &m)?)?;
            let names: Vec<&str> = vec!["eyes", "field", "layers", "period", "sharing", "stack", "stats"];
            m.add("__all__", names)?;
            parent.add("tourbillon", &m)?;
            sys.set_item("mrlypy._mrlypy.math.tourbillon", &m)?;
            Ok(())
        }
    }

    /// The flat-cell pipeline: designs, tiles, graphs and renderings in two dimensions.
    /// The flat-cell pipeline.
    pub mod two {
        use crate::hand::{ok, PyCell2d, PyCellNd, PyCode, PyColor, PyRng, PySerde, PyTensor};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The payload a cell's filled sites carry, framed in a sheet.
        pub mod payload {
            use crate::hand::{PyTensor};
            use pyo3::prelude::*;
            use pyo3::types::PyDict;
            use pyo3::IntoPyObjectExt;

            /// The five by five mask a carried mosaic lays its four tiles out under.
            #[pyfunction]
            #[pyo3(name = "frame", signature = ())]
            pub fn frame<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::math::two::payload::frame();
                (PyTensor(out)).into_bound_py_any(py)
            }

            pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
                let m = PyModule::new(py, "mrlypy.math.two.payload")?;
                m.setattr("__doc__", "The payload a cell's filled sites carry, framed in a sheet.")?;
                m.add_function(wrap_pyfunction!(frame, &m)?)?;
                let names: Vec<&str> = vec!["frame"];
                m.add("__all__", names)?;
                parent.add("payload", &m)?;
                sys.set_item("mrlypy._mrlypy.math.two.payload", &m)?;
                Ok(())
            }
        }

        /// Returns the payload bytes the cell's filled sites can hold, its length header paid for.
        #[pyfunction]
        #[pyo3(name = "capacity", signature = (cell))]
        pub fn capacity<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::capacity(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "carpet", signature = (number, level))]
        pub fn carpet<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::carpet(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Takes the cell's full census in one reading.
        #[pyfunction]
        #[pyo3(name = "census", signature = (cell))]
        pub fn census<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::census(&cell);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the design a universe code names, deepened to the level and rotated by quarter-turns.
        #[pyfunction]
        #[pyo3(name = "create", signature = (code, number, level, rotation, base))]
        pub fn create<'py>(py: Python<'py>, code: PyCode, number: usize, level: usize, rotation: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrlyrs::math::two::create(code, number, level, rotation, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the dust fractal, its seed on at every even-even site, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "dust", signature = (number, level))]
        pub fn dust<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::dust(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Writes the payload over the cell's filled sites, repeating it until every site is spoken for.
        #[pyfunction]
        #[pyo3(name = "embed", signature = (cell, payload))]
        pub fn embed<'py>(py: Python<'py>, cell: PyCell2d, payload: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::embed(&cell, &payload);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Euler characteristic of the filled sites, vertices less edges plus faces.
        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell))]
        pub fn euler<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::euler(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Reads the payload back, the plain cell naming the sites the carried one wrote over.
        #[pyfunction]
        #[pyo3(name = "extract", signature = (carrier, carried))]
        pub fn extract<'py>(py: Python<'py>, carrier: PyCell2d, carried: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let carrier = carrier.0;
            let carried = carried.0;
            let out = mrlyrs::math::two::extract(&carrier, &carried);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Counts the filled sites of the cell.
        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::fills(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns.
        #[pyfunction]
        #[pyo3(name = "from_corners", signature = (corners, number, level, rotation, base))]
        pub fn from_corners<'py>(py: Python<'py>, corners: Vec<Vec<u8>>, number: usize, level: usize, rotation: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::from_corners(&corners, number, level, rotation, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Restores a cell from its JSON string, colors and tags included.
        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::from_json(text);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a cell from rows of digits, the inverse of the text rendering.
        #[pyfunction]
        #[pyo3(name = "from_strings", signature = (rows))]
        pub fn from_strings<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::from_strings(&rows);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the hline fractal, its seed striped along odd rows, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "hline", signature = (number, level))]
        pub fn hline<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::hline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the htree fractal, its seed striped along even rows, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "htree", signature = (number, level))]
        pub fn htree<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::htree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the level-set design, filling every residue corner whose digits sum to a named level.
        #[pyfunction]
        #[pyo3(name = "level_set", signature = (number, levels, level, rotation, base))]
        pub fn level_set<'py>(py: Python<'py>, number: usize, levels: Vec<usize>, level: usize, rotation: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::level_set(number, &levels, level, rotation, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Tiles the mask over the shape and crops it, the perforation pattern itself.
        #[pyfunction]
        #[pyo3(name = "mask", signature = (mask, shape))]
        pub fn mask<'py>(py: Python<'py>, mask: PyTensor, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let out = mrlyrs::math::two::mask(&mask, &shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept.
        #[pyfunction]
        #[pyo3(name = "merge", signature = (cells, width, height))]
        pub fn merge<'py>(py: Python<'py>, cells: Vec<PyCell2d>, width: usize, height: usize) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::math::two::merge(&cells, width, height);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the design the name picks, deepened to the level and rotated by quarter-turns.
        #[pyfunction]
        #[pyo3(name = "named", signature = (design, number, level, rotation))]
        pub fn named<'py>(py: Python<'py>, design: PySerde<mrlyrs::gen::recipe::Design>, number: usize, level: usize, rotation: usize) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrlyrs::math::two::named(design, number, level, rotation);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "net", signature = (number, level))]
        pub fn net<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::net(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a random cell, each seed site drawn on with probability density, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "noise", signature = (number, level, density, rng))]
        pub fn noise<'py>(py: Python<'py>, number: usize, level: usize, density: f64, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::noise(number, level, density, &mut rng.0);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds an all-filled cell of the given size and level.
        #[pyfunction]
        #[pyo3(name = "ones", signature = (number, level))]
        pub fn ones<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::ones(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts the faces of filled sites open to emptiness or the border.
        #[pyfunction]
        #[pyo3(name = "perimeter", signature = (cell))]
        pub fn perimeter<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::perimeter(&cell);
            (out).into_bound_py_any(py)
        }

        /// Renders the cell to PNG bytes at the given pixel scale, stroked and padded when an outline is given.
        #[pyfunction]
        #[pyo3(name = "png", signature = (cell, scale, outline, width, shape))]
        pub fn png<'py>(py: Python<'py>, cell: PyCell2d, scale: usize, outline: Option<PyColor>, width: usize, shape: PySerde<mrlyrs::math::two::Shape>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let shape = shape.0;
            let out = mrlyrs::math::two::png(&cell, scale, outline, width, shape);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Builds the point fractal, its seed on at every odd-odd site, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "point", signature = (number, level))]
        pub fn point<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::point(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Reads the payload back from a framed sheet, the plain fourth cell naming the sites.
        #[pyfunction]
        #[pyo3(name = "read", signature = (sheet, carrier))]
        pub fn read<'py>(py: Python<'py>, sheet: PyCell2d, carrier: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let sheet = sheet.0;
            let carrier = carrier.0;
            let out = mrlyrs::math::two::read(&sheet, &carrier);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Builds the framed sheet of four same-sized cells, the fourth carrying the payload.
        #[pyfunction]
        #[pyo3(name = "sheet", signature = (cells, payload))]
        pub fn sheet<'py>(py: Python<'py>, cells: [PyCell2d; 4], payload: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.map(|x| x.0);
            let out = mrlyrs::math::two::sheet(&cells, &payload);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Tiles quarter-turned copies of the cell as the 2d mask directs.
        #[pyfunction]
        #[pyo3(name = "special", signature = (mask, cell))]
        pub fn special<'py>(py: Python<'py>, mask: PyTensor, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let cell = cell.0;
            let out = mrlyrs::math::two::special(&mask, &cell);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "star", signature = (number, level))]
        pub fn star<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::star(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given.
        #[pyfunction]
        #[pyo3(name = "svg", signature = (cell, scale, outline, width, shape))]
        pub fn svg<'py>(py: Python<'py>, cell: PyCell2d, scale: usize, outline: Option<PyColor>, width: usize, shape: PySerde<mrlyrs::math::two::Shape>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let shape = shape.0;
            let out = mrlyrs::math::two::svg(&cell, scale, outline, width, shape);
            (out).into_bound_py_any(py)
        }

        /// Renders the cell as rows of glyphs, or of digits where no glyph is mapped.
        #[pyfunction]
        #[pyo3(name = "text", signature = (cell, glyphs=None))]
        pub fn text<'py>(py: Python<'py>, cell: PyCell2d, glyphs: Option<std::collections::HashMap<u8, String>>) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let glyphs = glyphs.map(|x| x.into_iter().collect());
            let out = mrlyrs::math::two::text(&cell, glyphs.as_ref());
            (out).into_bound_py_any(py)
        }

        /// Lifts the flat cell into a cube one site deep, colors and tags with it.
        #[pyfunction]
        #[pyo3(name = "to_3d", signature = (cell))]
        pub fn to_3d<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::to_3d(&cell);
            (PyCellNd(out)).into_bound_py_any(py)
        }

        /// Serializes the cell to a JSON string of its types, with colors and tags when present.
        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the vline fractal, its seed striped along odd columns, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "vline", signature = (number, level))]
        pub fn vline<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::vline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the void fractal, its seed a checkerboard on even parity, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "void", signature = (number, level))]
        pub fn void<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::void(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts the empty sites of the cell.
        #[pyfunction]
        #[pyo3(name = "voids", signature = (cell))]
        pub fn voids<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrlyrs::math::two::voids(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the vtree fractal, its seed striped along even columns, deepened to the level.
        #[pyfunction]
        #[pyo3(name = "vtree", signature = (number, level))]
        pub fn vtree<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::vtree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds an all-empty cell of the given size and level.
        #[pyfunction]
        #[pyo3(name = "zeros", signature = (number, level))]
        pub fn zeros<'py>(py: Python<'py>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::math::two::zeros(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.math.two")?;
            m.setattr("__doc__", "The flat-cell pipeline: designs, tiles, graphs and renderings in two dimensions.\nThe flat-cell pipeline.\n\nThe shared cell pipeline pinned to two dimensions: coded and carpet cells, their censuses,\ntheir payloads, their text and PNG renderings and their JSON.")?;
            m.add_function(wrap_pyfunction!(capacity, &m)?)?;
            m.add_function(wrap_pyfunction!(carpet, &m)?)?;
            m.add_function(wrap_pyfunction!(census, &m)?)?;
            m.add_function(wrap_pyfunction!(create, &m)?)?;
            m.add_function(wrap_pyfunction!(dust, &m)?)?;
            m.add_function(wrap_pyfunction!(embed, &m)?)?;
            m.add_function(wrap_pyfunction!(euler, &m)?)?;
            m.add_function(wrap_pyfunction!(extract, &m)?)?;
            m.add_function(wrap_pyfunction!(fills, &m)?)?;
            m.add_function(wrap_pyfunction!(from_corners, &m)?)?;
            m.add_function(wrap_pyfunction!(from_json, &m)?)?;
            m.add_function(wrap_pyfunction!(from_strings, &m)?)?;
            m.add_function(wrap_pyfunction!(hline, &m)?)?;
            m.add_function(wrap_pyfunction!(htree, &m)?)?;
            m.add_function(wrap_pyfunction!(level_set, &m)?)?;
            m.add_function(wrap_pyfunction!(mask, &m)?)?;
            m.add_function(wrap_pyfunction!(merge, &m)?)?;
            m.add_function(wrap_pyfunction!(named, &m)?)?;
            m.add_function(wrap_pyfunction!(net, &m)?)?;
            m.add_function(wrap_pyfunction!(noise, &m)?)?;
            m.add_function(wrap_pyfunction!(ones, &m)?)?;
            m.add_function(wrap_pyfunction!(perimeter, &m)?)?;
            m.add_function(wrap_pyfunction!(png, &m)?)?;
            m.add_function(wrap_pyfunction!(point, &m)?)?;
            m.add_function(wrap_pyfunction!(read, &m)?)?;
            m.add_function(wrap_pyfunction!(sheet, &m)?)?;
            m.add_function(wrap_pyfunction!(special, &m)?)?;
            m.add_function(wrap_pyfunction!(star, &m)?)?;
            m.add_function(wrap_pyfunction!(svg, &m)?)?;
            m.add_function(wrap_pyfunction!(text, &m)?)?;
            m.add_function(wrap_pyfunction!(to_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(to_json, &m)?)?;
            m.add_function(wrap_pyfunction!(vline, &m)?)?;
            m.add_function(wrap_pyfunction!(void, &m)?)?;
            m.add_function(wrap_pyfunction!(voids, &m)?)?;
            m.add_function(wrap_pyfunction!(vtree, &m)?)?;
            m.add_function(wrap_pyfunction!(zeros, &m)?)?;
            let names: Vec<&str> = vec!["capacity", "carpet", "census", "create", "dust", "embed", "euler", "extract", "fills", "from_corners", "from_json", "from_strings", "hline", "htree", "level_set", "mask", "merge", "named", "net", "noise", "ones", "perimeter", "png", "point", "read", "sheet", "special", "star", "svg", "text", "to_3d", "to_json", "vline", "void", "voids", "vtree", "zeros"];
            m.add("__all__", names)?;
            payload::init(py, &m, sys)?;
            parent.add("two", &m)?;
            sys.set_item("mrlypy._mrlypy.math.two", &m)?;
            Ok(())
        }
    }

    pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
        let m = PyModule::new(py, "mrlypy.math")?;
        m.setattr("__doc__", "The designs in space: codes, cells, cubes, hexagons, their counts, graphs and names.\nThe designs in space.\n\nA small integer code picks the filled corners of a hypercube, that seed grows level by level\ninto a fractal design, and the same code always unfolds into the same shape, so a design can be\nnamed, counted and drawn again from its number alone. Half the module generates and half\nmeasures; everything rests on the tensors and cells of [`crate::core`] and the sequences of\n[`crate::num`].\n\n- `atoms` fills a tensor with a carpet, a net, beams or noise.\n- `bang` enumerates the design codes, their symmetries and their counts.\n- `cell` holds the N-dimensional cell and the pipeline the fixed dimensions share.\n- `two`, `three` and `six` run that pipeline for flat cells, cubes and hexagons.\n- `counts` gives the same fills, surfaces, hex slices and carry ladder in closed form.\n- `graph` lifts a grid into nodes and branches; `spectrum` reads the Laplacian spectra off it.\n- `shape` crops a cell against a rational shape, cell by cell, with no floats.\n- `rules` marks the cells of a hypercube whose coordinate residues satisfy a rule.\n- `moire` layers one design at many scales into an interference field.\n- `press` weighs the integers a design's digit rule keeps.\n- `spin` spins a raster about its centre; `tourbillon` stacks the turned parity carpets.\n- `spirograph` rolls a byte grid as a wheel; `roulette` counts where its curves cross.\n- `name` prints and parses the one canonical JSON object of every design, rule, tile and word.\n\nThe doors are [`crate::math::atoms::carpet_2d`], [`crate::math::bang::bang`],\n[`crate::math::two::carpet`], [`crate::math::two::census()`], [`crate::math::two::to_json`],\n[`crate::math::counts::fill`], [`crate::math::three::census::surface`] and\n[`crate::math::spectrum::laplacian_spectrum`].")?;
        let names: Vec<&str> = vec![];
        m.add("__all__", names)?;
        atoms::init(py, &m, sys)?;
        bang::init(py, &m, sys)?;
        cell::init(py, &m, sys)?;
        counts::init(py, &m, sys)?;
        graph::init(py, &m, sys)?;
        moire::init(py, &m, sys)?;
        name::init(py, &m, sys)?;
        press::init(py, &m, sys)?;
        roulette::init(py, &m, sys)?;
        rules::init(py, &m, sys)?;
        shape::init(py, &m, sys)?;
        six::init(py, &m, sys)?;
        spectrum::init(py, &m, sys)?;
        spin::init(py, &m, sys)?;
        spirograph::init(py, &m, sys)?;
        three::init(py, &m, sys)?;
        tourbillon::init(py, &m, sys)?;
        two::init(py, &m, sys)?;
        parent.add("math", &m)?;
        sys.set_item("mrlypy._mrlypy.math", &m)?;
        Ok(())
    }
}

/// The integers: primes, divisors, series, lattices, spectra and networks.
/// The instruments of number: primes, divisors, series, spectra, lattices and the designs the digits draw.
pub mod num {
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// The Apollonian gasket: a packing grown from its root quadruple in exact integers, the Ford circles it rests on the line, and the Farey stack they shadow.
    pub mod apollonian {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing.
        #[pyclass(name = "Circle", module = "mrlypy.num.apollonian", from_py_object)]
        #[derive(Clone)]
        pub struct Circle(pub mrlyrs::num::apollonian::Circle);

        #[pymethods]
        impl Circle {
            /// The curvature.
            #[getter]
            #[pyo3(name = "k")]
            pub fn k<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.k;
                (value).into_bound_py_any(py)
            }
            /// The curvature times the centre's abscissa.
            #[getter]
            #[pyo3(name = "x")]
            pub fn x<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.x;
                (value).into_bound_py_any(py)
            }
            /// The curvature times the centre's ordinate.
            #[getter]
            #[pyo3(name = "y")]
            pub fn y<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.y;
                (value).into_bound_py_any(py)
            }
            /// The centre, none on a line.
            #[pyo3(name = "centre", signature = ())]
            pub fn centre<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::apollonian::Circle::centre(self.0);
                (out).into_bound_py_any(py)
            }
            /// Whether the circle is a line.
            #[pyo3(name = "is_line", signature = ())]
            pub fn is_line<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::apollonian::Circle::is_line(self.0);
                (out).into_bound_py_any(py)
            }
            /// The radius, none on a line.
            #[pyo3(name = "radius", signature = ())]
            pub fn radius<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::apollonian::Circle::radius(self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.
        #[pyfunction]
        #[pyo3(name = "form", signature = (u, v))]
        pub fn form<'py>(py: Python<'py>, u: [i64; 4], v: [i64; 4]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::apollonian::form(u, v);
            (out).into_bound_py_any(py)
        }

        /// The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.
        #[pyfunction]
        #[pyo3(name = "frame", signature = (p))]
        pub fn frame<'py>(py: Python<'py>, p: PySerde<mrlyrs::num::apollonian::Packing>) -> PyResult<Bound<'py, PyAny>> {
            let p = p.0;
            let out = mrlyrs::num::apollonian::frame(&p);
            (out).into_bound_py_any(py)
        }

        /// Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.
        #[pyfunction]
        #[pyo3(name = "grow", signature = (name, cap))]
        pub fn grow<'py>(py: Python<'py>, name: &str, cap: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::apollonian::grow(name, cap);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.
        #[pyfunction]
        #[pyo3(name = "is_ford", signature = (c))]
        pub fn is_ford<'py>(py: Python<'py>, c: crate::gen::num::apollonian::Circle) -> PyResult<Bound<'py, PyAny>> {
            let c = c.0;
            let out = mrlyrs::num::apollonian::is_ford(c);
            (out).into_bound_py_any(py)
        }

        /// Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.
        #[pyfunction]
        #[pyo3(name = "on_line", signature = (c))]
        pub fn on_line<'py>(py: Python<'py>, c: crate::gen::num::apollonian::Circle) -> PyResult<Bound<'py, PyAny>> {
            let c = c.0;
            let out = mrlyrs::num::apollonian::on_line(c);
            (out).into_bound_py_any(py)
        }

        /// Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.
        #[pyfunction]
        #[pyo3(name = "reflect", signature = (q, at))]
        pub fn reflect<'py>(py: Python<'py>, q: [crate::gen::num::apollonian::Circle; 4], at: usize) -> PyResult<Bound<'py, PyAny>> {
            let q = q.map(|x| x.0);
            let out = mrlyrs::num::apollonian::reflect(&q, at);
            (crate::gen::num::apollonian::Circle(out)).into_bound_py_any(py)
        }

        /// The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.
        #[pyfunction]
        #[pyo3(name = "root", signature = (name))]
        pub fn root<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::apollonian::root(name);
            ((ok(out)?).into_iter().map(crate::gen::num::apollonian::Circle).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.
        #[pyfunction]
        #[pyo3(name = "shadow", signature = (p, order))]
        pub fn shadow<'py>(py: Python<'py>, p: PySerde<mrlyrs::num::apollonian::Packing>, order: usize) -> PyResult<Bound<'py, PyAny>> {
            let p = p.0;
            let out = mrlyrs::num::apollonian::shadow(&p, order);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.
        #[pyfunction]
        #[pyo3(name = "sound", signature = (q))]
        pub fn sound<'py>(py: Python<'py>, q: [crate::gen::num::apollonian::Circle; 4]) -> PyResult<Bound<'py, PyAny>> {
            let q = q.map(|x| x.0);
            let out = mrlyrs::num::apollonian::sound(&q);
            (out).into_bound_py_any(py)
        }

        /// The quadruple with the circle at the seat replaced by its reflection.
        #[pyfunction]
        #[pyo3(name = "swap", signature = (q, at))]
        pub fn swap<'py>(py: Python<'py>, q: [crate::gen::num::apollonian::Circle; 4], at: usize) -> PyResult<Bound<'py, PyAny>> {
            let q = q.map(|x| x.0);
            let out = mrlyrs::num::apollonian::swap(&q, at);
            ((out).into_iter().map(crate::gen::num::apollonian::Circle).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.
        #[pyfunction]
        #[pyo3(name = "touches", signature = (p))]
        pub fn touches<'py>(py: Python<'py>, p: PySerde<mrlyrs::num::apollonian::Packing>) -> PyResult<Bound<'py, PyAny>> {
            let p = p.0;
            let out = mrlyrs::num::apollonian::touches(&p);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.apollonian")?;
            m.setattr("__doc__", "The Apollonian gasket: a packing grown from its root quadruple in exact integers, the Ford circles it rests on the line, and the Farey stack they shadow.")?;
            m.add_class::<Circle>()?;
            m.add_function(wrap_pyfunction!(form, &m)?)?;
            m.add_function(wrap_pyfunction!(frame, &m)?)?;
            m.add_function(wrap_pyfunction!(grow, &m)?)?;
            m.add_function(wrap_pyfunction!(is_ford, &m)?)?;
            m.add_function(wrap_pyfunction!(on_line, &m)?)?;
            m.add_function(wrap_pyfunction!(reflect, &m)?)?;
            m.add_function(wrap_pyfunction!(root, &m)?)?;
            m.add_function(wrap_pyfunction!(shadow, &m)?)?;
            m.add_function(wrap_pyfunction!(sound, &m)?)?;
            m.add_function(wrap_pyfunction!(swap, &m)?)?;
            m.add_function(wrap_pyfunction!(touches, &m)?)?;
            m.add("CIRCLE_CAP", mrlyrs::num::apollonian::CIRCLE_CAP)?;
            m.add("CURVATURE_CAP", mrlyrs::num::apollonian::CURVATURE_CAP)?;
            m.add("ORDER_CAP", mrlyrs::num::apollonian::ORDER_CAP)?;
            m.add("ROOTS", (mrlyrs::num::apollonian::ROOTS).into_iter().map(|x| (x).to_string()).collect::<Vec<_>>())?;
            let names: Vec<&str> = vec!["form", "frame", "grow", "is_ford", "on_line", "reflect", "root", "shadow", "sound", "swap", "touches", "Circle", "CIRCLE_CAP", "CURVATURE_CAP", "ORDER_CAP", "ROOTS"];
            m.add("__all__", names)?;
            parent.add("apollonian", &m)?;
            sys.set_item("mrlypy._mrlypy.num.apollonian", &m)?;
            Ok(())
        }
    }

    /// The matrix ladder: the Dirichlet series of a memory design continued through its transfer matrix, its determinant cofactor and the residues on its pole combs.
    pub mod automaton {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from.
        #[pyclass(name = "Automaton", module = "mrlypy.num.automaton", from_py_object)]
        #[derive(Clone)]
        pub struct Automaton(pub mrlyrs::num::automaton::Automaton);

        #[pymethods]
        impl Automaton {
            /// Builds the ladder of a rule, choosing the peel depth.
            #[new]
            #[pyo3(signature = (rule))]
            pub fn __new__(rule: PyRef<'_, crate::gen::num::memory::Rule>) -> PyResult<Self> {
                let out = mrlyrs::num::automaton::Automaton::new(&rule.0);
                Ok(Self(ok(out)?))
            }
            /// Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of [`crate::num::memory::perron`].
            #[pyo3(name = "abscissa", signature = ())]
            pub fn abscissa<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::abscissa(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the base `q = 2^D`.
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::base(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to.
            #[pyo3(name = "cofactor", signature = (s, tolerance))]
            pub fn cofactor<'py>(&self, py: Python<'py>, s: crate::gen::num::zeta::Complex, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
                let s = s.0;
                let out = mrlyrs::num::automaton::Automaton::cofactor(&self.0, s, tolerance);
                ({ let t = ok(out)?; (crate::gen::num::zeta::Complex(t.0), t.1) }).into_bound_py_any(py)
            }
            /// Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`.
            #[pyo3(name = "denominator", signature = ())]
            pub fn denominator<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::denominator(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of [`crate::num::memory::transfer`], entry `(u', u)` counting the letters carrying `u` to `u'`.
            #[pyo3(name = "matrix", signature = ())]
            pub fn matrix<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::matrix(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds the ladder of a rule, choosing the peel depth.
            #[staticmethod]
            #[pyo3(name = "new", signature = (rule))]
            pub fn new_<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::new(&rule.0);
                (crate::gen::num::automaton::Automaton(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the peel depth `P`.
            #[pyo3(name = "peel", signature = ())]
            pub fn peel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::peel(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the pole spacing `2 pi / log q`.
            #[pyo3(name = "period", signature = ())]
            pub fn period<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::period(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with.
            #[pyo3(name = "perron", signature = ())]
            pub fn perron<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::perron(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to.
            #[pyo3(name = "residue", signature = (w0, tolerance))]
            pub fn residue<'py>(&self, py: Python<'py>, w0: crate::gen::num::zeta::Complex, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
                let w0 = w0.0;
                let out = mrlyrs::num::automaton::Automaton::residue(&self.0, w0, tolerance);
                ({ let t = ok(out)?; (crate::gen::num::zeta::Complex(t.0), t.1) }).into_bound_py_any(py)
            }
            /// Returns the rule.
            #[pyo3(name = "rule", signature = ())]
            pub fn rule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::rule(&self.0);
                (crate::gen::num::memory::Rule(out)).into_bound_py_any(py)
            }
            /// Returns the state count `q^(k-1)`.
            #[pyo3(name = "states", signature = ())]
            pub fn states<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::states(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds the ladder at an explicit peel depth, at least the rule width and at least two.
            #[staticmethod]
            #[pyo3(name = "with_peel", signature = (rule, peel))]
            pub fn with_peel<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>, peel: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::automaton::Automaton::with_peel(&rule.0, peel);
                (crate::gen::num::automaton::Automaton(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns `zeta_W(s)` and the bound it is known to.
            #[pyo3(name = "zeta", signature = (s, tolerance))]
            pub fn zeta<'py>(&self, py: Python<'py>, s: crate::gen::num::zeta::Complex, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
                let s = s.0;
                let out = mrlyrs::num::automaton::Automaton::zeta(&self.0, s, tolerance);
                ({ let t = ok(out)?; (crate::gen::num::zeta::Complex(t.0), t.1) }).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.automaton")?;
            m.setattr("__doc__", "The matrix ladder: the Dirichlet series of a memory design continued through its transfer matrix, its determinant cofactor and the residues on its pole combs.\n\nA claim witness with no caller in the crate: it witnesses `research/claims/memory-dial.md`.")?;
            m.add_class::<Automaton>()?;
            m.add("ROUNDING", mrlyrs::num::automaton::ROUNDING)?;
            let names: Vec<&str> = vec!["Automaton", "ROUNDING"];
            m.add("__all__", names)?;
            parent.add("automaton", &m)?;
            sys.set_item("mrlypy._mrlypy.num.automaton", &m)?;
            Ok(())
        }
    }

    /// The sequence blender: term ops, exact recurrences and growth rates.
    pub mod blend {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Adds two sequences term by term over their shared length.
        #[pyfunction]
        #[pyo3(name = "add", signature = (a, b))]
        pub fn add<'py>(py: Python<'py>, a: Vec<i128>, b: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::add(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Convolves two sequences, keeping the exact prefix their shared length affords.
        #[pyfunction]
        #[pyo3(name = "cauchy", signature = (a, b))]
        pub fn cauchy<'py>(py: Python<'py>, a: Vec<i128>, b: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::cauchy(&a, &b);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the monic characteristic polynomial of a recurrence, highest power first.
        #[pyfunction]
        #[pyo3(name = "characteristic", signature = (coefficients))]
        pub fn characteristic<'py>(py: Python<'py>, coefficients: Vec<(i128, i128)>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::characteristic(&coefficients);
            (out).into_bound_py_any(py)
        }

        /// Keeps every step-th term from the offset onward.
        #[pyfunction]
        #[pyo3(name = "decimate", signature = (a, step, offset))]
        pub fn decimate<'py>(py: Python<'py>, a: Vec<i128>, step: usize, offset: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::decimate(&a, step, offset);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the first differences of a sequence, one term shorter.
        #[pyfunction]
        #[pyo3(name = "delta", signature = (a))]
        pub fn delta<'py>(py: Python<'py>, a: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::delta(&a);
            (out).into_bound_py_any(py)
        }

        /// Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands.
        #[pyfunction]
        #[pyo3(name = "growth", signature = (coefficients))]
        pub fn growth<'py>(py: Python<'py>, coefficients: Vec<(i128, i128)>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::growth(&coefficients);
            (out).into_bound_py_any(py)
        }

        /// Multiplies two sequences term by term over their shared length.
        #[pyfunction]
        #[pyo3(name = "hadamard", signature = (a, b))]
        pub fn hadamard<'py>(py: Python<'py>, a: Vec<i128>, b: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::hadamard(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Finds the smallest linear constant-coefficient recurrence that fits every supplied term.
        #[pyfunction]
        #[pyo3(name = "recurrence", signature = (terms))]
        pub fn recurrence<'py>(py: Python<'py>, terms: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::recurrence(&terms);
            (out).into_bound_py_any(py)
        }

        /// Multiplies every term of a sequence by the factor.
        #[pyfunction]
        #[pyo3(name = "scale", signature = (a, factor))]
        pub fn scale<'py>(py: Python<'py>, a: Vec<i128>, factor: i128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::scale(&a, factor);
            (out).into_bound_py_any(py)
        }

        /// Drops the first terms of a sequence.
        #[pyfunction]
        #[pyo3(name = "shift", signature = (a, count))]
        pub fn shift<'py>(py: Python<'py>, a: Vec<i128>, count: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::shift(&a, count);
            (out).into_bound_py_any(py)
        }

        /// Returns the partial sums of a sequence.
        #[pyfunction]
        #[pyo3(name = "sigma", signature = (a))]
        pub fn sigma<'py>(py: Python<'py>, a: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::sigma(&a);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Subtracts the second sequence from the first over their shared length.
        #[pyfunction]
        #[pyo3(name = "sub", signature = (a, b))]
        pub fn sub<'py>(py: Python<'py>, a: Vec<i128>, b: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::blend::sub(&a, &b);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.blend")?;
            m.setattr("__doc__", "The sequence blender: term ops, exact recurrences and growth rates.")?;
            m.add_function(wrap_pyfunction!(add, &m)?)?;
            m.add_function(wrap_pyfunction!(cauchy, &m)?)?;
            m.add_function(wrap_pyfunction!(characteristic, &m)?)?;
            m.add_function(wrap_pyfunction!(decimate, &m)?)?;
            m.add_function(wrap_pyfunction!(delta, &m)?)?;
            m.add_function(wrap_pyfunction!(growth, &m)?)?;
            m.add_function(wrap_pyfunction!(hadamard, &m)?)?;
            m.add_function(wrap_pyfunction!(recurrence, &m)?)?;
            m.add_function(wrap_pyfunction!(scale, &m)?)?;
            m.add_function(wrap_pyfunction!(shift, &m)?)?;
            m.add_function(wrap_pyfunction!(sigma, &m)?)?;
            m.add_function(wrap_pyfunction!(sub, &m)?)?;
            let names: Vec<&str> = vec!["add", "cauchy", "characteristic", "decimate", "delta", "growth", "hadamard", "recurrence", "scale", "shift", "sigma", "sub"];
            m.add("__all__", names)?;
            parent.add("blend", &m)?;
            sys.set_item("mrlypy._mrlypy.num.blend", &m)?;
            Ok(())
        }
    }

    /// The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.
    pub mod boolean {
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Reports whether the packed function outputs one on exactly half of its inputs.
        #[pyfunction]
        #[pyo3(name = "is_balanced", signature = (code, n))]
        pub fn is_balanced<'py>(py: Python<'py>, code: u128, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::boolean::is_balanced(code, n);
            (out).into_bound_py_any(py)
        }

        /// Returns how far the packed function sits from every affine function, zero when it is one.
        #[pyfunction]
        #[pyo3(name = "nonlinearity", signature = (code, n))]
        pub fn nonlinearity<'py>(py: Python<'py>, code: u128, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::boolean::nonlinearity(code, n);
            (out).into_bound_py_any(py)
        }

        /// Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche.
        #[pyfunction]
        #[pyo3(name = "sac", signature = (code, n))]
        pub fn sac<'py>(py: Python<'py>, code: u128, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::boolean::sac(code, n);
            (out).into_bound_py_any(py)
        }

        /// Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code.
        #[pyfunction]
        #[pyo3(name = "walsh_spectrum", signature = (code, n))]
        pub fn walsh_spectrum<'py>(py: Python<'py>, code: u128, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::boolean::walsh_spectrum(code, n);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.boolean")?;
            m.setattr("__doc__", "The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.")?;
            m.add_function(wrap_pyfunction!(is_balanced, &m)?)?;
            m.add_function(wrap_pyfunction!(nonlinearity, &m)?)?;
            m.add_function(wrap_pyfunction!(sac, &m)?)?;
            m.add_function(wrap_pyfunction!(walsh_spectrum, &m)?)?;
            let names: Vec<&str> = vec!["is_balanced", "nonlinearity", "sac", "walsh_spectrum"];
            m.add("__all__", names)?;
            parent.add("boolean", &m)?;
            sys.set_item("mrlypy._mrlypy.num.boolean", &m)?;
            Ok(())
        }
    }

    /// The digit designs on the integer line: their elements, their Mobius meter, its density echo and the ordinates its spectrum carries.
    pub mod design {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the digits a bitmask names inside the base, ascending.
        #[pyfunction]
        #[pyo3(name = "digits_of", signature = (mask, base))]
        pub fn digits_of<'py>(py: Python<'py>, mask: u32, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::digits_of(mask, base);
            (out).into_bound_py_any(py)
        }

        /// Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element.
        #[pyfunction]
        #[pyo3(name = "echo_series", signature = (values, log_x, exponent))]
        pub fn echo_series<'py>(py: Python<'py>, values: Vec<u64>, log_x: Vec<f64>, exponent: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::echo_series(&values, &log_x, exponent);
            (out).into_bound_py_any(py)
        }

        /// Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.
        #[pyfunction]
        #[pyo3(name = "elements", signature = (base, digits, depth))]
        pub fn elements<'py>(py: Python<'py>, base: u64, digits: Vec<u64>, depth: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::elements(base, &digits, depth);
            (out).into_bound_py_any(py)
        }

        /// Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last.
        #[pyfunction]
        #[pyo3(name = "log_grid", signature = (values, samples))]
        pub fn log_grid<'py>(py: Python<'py>, values: Vec<u64>, samples: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::log_grid(&values, samples);
            (out).into_bound_py_any(py)
        }

        /// Returns the running median of the power over a window of the given width, the window clamped at the ends.
        #[pyfunction]
        #[pyo3(name = "median_floor", signature = (power, width))]
        pub fn median_floor<'py>(py: Python<'py>, power: Vec<f64>, width: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::median_floor(&power, width);
            (out).into_bound_py_any(py)
        }

        /// Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.
        #[pyfunction]
        #[pyo3(name = "meter", signature = (mu))]
        pub fn meter<'py>(py: Python<'py>, mu: Vec<i8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::meter(&mu);
            (out).into_bound_py_any(py)
        }

        /// Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty.
        #[pyfunction]
        #[pyo3(name = "nearest", signature = (value, list))]
        pub fn nearest<'py>(py: Python<'py>, value: f64, list: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::nearest(value, &list);
            (out).into_bound_py_any(py)
        }

        /// Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first.
        #[pyfunction]
        #[pyo3(name = "peaks", signature = (gamma, score, band, threshold))]
        pub fn peaks<'py>(py: Python<'py>, gamma: Vec<f64>, score: Vec<f64>, band: (f64, f64), threshold: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::peaks(&gamma, &score, band, threshold);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.
        #[pyfunction]
        #[pyo3(name = "pole_lattice", signature = (base, top))]
        pub fn pole_lattice<'py>(py: Python<'py>, base: u64, top: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::pole_lattice(base, top);
            (out).into_bound_py_any(py)
        }

        /// Reads the running meter at every point of the log grid and divides by x to the exponent.
        #[pyfunction]
        #[pyo3(name = "resample", signature = (values, running, exponent, log_x))]
        pub fn resample<'py>(py: Python<'py>, values: Vec<u64>, running: Vec<i64>, exponent: f64, log_x: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::resample(&values, &running, exponent, &log_x);
            (out).into_bound_py_any(py)
        }

        /// Returns the power over its local median floor, the score a peak is read against.
        #[pyfunction]
        #[pyo3(name = "score", signature = (power, width))]
        pub fn score<'py>(py: Python<'py>, power: Vec<f64>, width: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::score(&power, width);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of elements the design holds at the depth, the length [`elements`] returns without building them.
        #[pyfunction]
        #[pyo3(name = "size", signature = (digits, depth))]
        pub fn size<'py>(py: Python<'py>, digits: Vec<u64>, depth: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::size(&digits, depth);
            (out).into_bound_py_any(py)
        }

        /// Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range.
        #[pyfunction]
        #[pyo3(name = "spectrum", signature = (log_x, series))]
        pub fn spectrum<'py>(py: Python<'py>, log_x: Vec<f64>, series: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::spectrum(&log_x, &series);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at.
        #[pyfunction]
        #[pyo3(name = "upper_rms", signature = (series))]
        pub fn upper_rms<'py>(py: Python<'py>, series: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::design::upper_rms(&series);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.design")?;
            m.setattr("__doc__", "The digit designs on the integer line: their elements, their Mobius meter, its density echo and the ordinates its spectrum carries.")?;
            m.add_function(wrap_pyfunction!(digits_of, &m)?)?;
            m.add_function(wrap_pyfunction!(echo_series, &m)?)?;
            m.add_function(wrap_pyfunction!(elements, &m)?)?;
            m.add_function(wrap_pyfunction!(log_grid, &m)?)?;
            m.add_function(wrap_pyfunction!(median_floor, &m)?)?;
            m.add_function(wrap_pyfunction!(meter, &m)?)?;
            m.add_function(wrap_pyfunction!(nearest, &m)?)?;
            m.add_function(wrap_pyfunction!(peaks, &m)?)?;
            m.add_function(wrap_pyfunction!(pole_lattice, &m)?)?;
            m.add_function(wrap_pyfunction!(resample, &m)?)?;
            m.add_function(wrap_pyfunction!(score, &m)?)?;
            m.add_function(wrap_pyfunction!(size, &m)?)?;
            m.add_function(wrap_pyfunction!(spectrum, &m)?)?;
            m.add_function(wrap_pyfunction!(upper_rms, &m)?)?;
            m.add("ZETA_ORDINATES", mrlyrs::num::design::ZETA_ORDINATES)?;
            let names: Vec<&str> = vec!["digits_of", "echo_series", "elements", "log_grid", "median_floor", "meter", "nearest", "peaks", "pole_lattice", "resample", "score", "size", "spectrum", "upper_rms", "ZETA_ORDINATES"];
            m.add("__all__", names)?;
            parent.add("design", &m)?;
            sys.set_item("mrlypy._mrlypy.num.design", &m)?;
            Ok(())
        }
    }

    /// The divisor arithmetic: factorizations, divisors, totients, radicals, the Mobius values and the exact whole-number arithmetic under them.
    pub mod factor {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.
        #[pyfunction]
        #[pyo3(name = "aliquot", signature = (number))]
        pub fn aliquot<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::aliquot(number);
            (out).into_bound_py_any(py)
        }

        /// Returns whether two numbers share no divisor above one.
        #[pyfunction]
        #[pyo3(name = "coprime", signature = (a, b))]
        pub fn coprime<'py>(py: Python<'py>, a: usize, b: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::coprime(a, b);
            (out).into_bound_py_any(py)
        }

        /// Builds every divisor of a wide number from its factorization, ascending, empty for zero.
        #[pyfunction]
        #[pyo3(name = "divisors", signature = (number))]
        pub fn divisors<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::divisors(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the factorial of the number, the product of one through it, erring past thirty-four.
        #[pyfunction]
        #[pyo3(name = "factorial", signature = (number))]
        pub fn factorial<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::factorial(number);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.
        #[pyfunction]
        #[pyo3(name = "factorize", signature = (number))]
        pub fn factorize<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::factorize(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.
        #[pyfunction]
        #[pyo3(name = "factorize_wide", signature = (number))]
        pub fn factorize_wide<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::factorize_wide(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes.
        #[pyfunction]
        #[pyo3(name = "gcd", signature = (a, b))]
        pub fn gcd<'py>(py: Python<'py>, a: u128, b: u128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::gcd(a, b);
            (out).into_bound_py_any(py)
        }

        /// Returns the least common multiple of two numbers, zero when either side is zero.
        #[pyfunction]
        #[pyo3(name = "lcm", signature = (a, b))]
        pub fn lcm<'py>(py: Python<'py>, a: usize, b: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::lcm(a, b);
            (out).into_bound_py_any(py)
        }

        /// Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.
        #[pyfunction]
        #[pyo3(name = "mobius", signature = (number))]
        pub fn mobius<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::mobius(number);
            (out).into_bound_py_any(py)
        }

        /// Sieves the Mobius values of zero through the limit in one pass.
        #[pyfunction]
        #[pyo3(name = "mobius_sieve", signature = (limit))]
        pub fn mobius_sieve<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::mobius_sieve(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.
        #[pyfunction]
        #[pyo3(name = "radical", signature = (number))]
        pub fn radical<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::radical(number);
            (out).into_bound_py_any(py)
        }

        /// Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.
        #[pyfunction]
        #[pyo3(name = "reduce", signature = (numerator, denominator))]
        pub fn reduce<'py>(py: Python<'py>, numerator: u128, denominator: u128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::reduce(numerator, denominator);
            (out).into_bound_py_any(py)
        }

        /// Returns the sum of every divisor of the number raised to the power, so power zero counts them.
        #[pyfunction]
        #[pyo3(name = "sigma", signature = (number, power))]
        pub fn sigma<'py>(py: Python<'py>, number: usize, power: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::sigma(number, power);
            (out).into_bound_py_any(py)
        }

        /// Returns whether no prime squares into the number, true for one and false for zero.
        #[pyfunction]
        #[pyo3(name = "squarefree", signature = (number))]
        pub fn squarefree<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::squarefree(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the Euler totient of the number from its factorization, zero for zero and one for one.
        #[pyfunction]
        #[pyo3(name = "totient", signature = (number))]
        pub fn totient<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::totient(number);
            (out).into_bound_py_any(py)
        }

        /// Sieves the Euler totients of zero through n in one pass, the run beside the single value.
        #[pyfunction]
        #[pyo3(name = "totients", signature = (n))]
        pub fn totients<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::totients(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.
        #[pyfunction]
        #[pyo3(name = "twisted", signature = (number, rhythm))]
        pub fn twisted<'py>(py: Python<'py>, number: usize, rhythm: Vec<i8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::factor::twisted(number, &rhythm);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.factor")?;
            m.setattr("__doc__", "The divisor arithmetic: factorizations, divisors, totients, radicals, the Mobius values and the exact whole-number arithmetic under them.")?;
            m.add_function(wrap_pyfunction!(aliquot, &m)?)?;
            m.add_function(wrap_pyfunction!(coprime, &m)?)?;
            m.add_function(wrap_pyfunction!(divisors, &m)?)?;
            m.add_function(wrap_pyfunction!(factorial, &m)?)?;
            m.add_function(wrap_pyfunction!(factorize, &m)?)?;
            m.add_function(wrap_pyfunction!(factorize_wide, &m)?)?;
            m.add_function(wrap_pyfunction!(gcd, &m)?)?;
            m.add_function(wrap_pyfunction!(lcm, &m)?)?;
            m.add_function(wrap_pyfunction!(mobius, &m)?)?;
            m.add_function(wrap_pyfunction!(mobius_sieve, &m)?)?;
            m.add_function(wrap_pyfunction!(radical, &m)?)?;
            m.add_function(wrap_pyfunction!(reduce, &m)?)?;
            m.add_function(wrap_pyfunction!(sigma, &m)?)?;
            m.add_function(wrap_pyfunction!(squarefree, &m)?)?;
            m.add_function(wrap_pyfunction!(totient, &m)?)?;
            m.add_function(wrap_pyfunction!(totients, &m)?)?;
            m.add_function(wrap_pyfunction!(twisted, &m)?)?;
            let names: Vec<&str> = vec!["aliquot", "coprime", "divisors", "factorial", "factorize", "factorize_wide", "gcd", "lcm", "mobius", "mobius_sieve", "radical", "reduce", "sigma", "squarefree", "totient", "totients", "twisted"];
            m.add("__all__", names)?;
            parent.add("factor", &m)?;
            sys.set_item("mrlypy._mrlypy.num.factor", &m)?;
            Ok(())
        }
    }

    /// The fast Fourier transform in one and two dimensions.
    pub mod fft {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.
        #[pyfunction]
        #[pyo3(name = "convolve", signature = (field, kernel, size))]
        pub fn convolve<'py>(py: Python<'py>, field: Vec<f64>, kernel: Vec<f64>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::convolve(&field, &kernel, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared.
        #[pyfunction]
        #[pyo3(name = "convolve_with", signature = (field, kernel_re, kernel_im, size))]
        pub fn convolve_with<'py>(py: Python<'py>, field: Vec<f64>, kernel_re: Vec<f64>, kernel_im: Vec<f64>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::convolve_with(&field, &kernel_re, &kernel_im, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.
        #[pyfunction]
        #[pyo3(name = "embed_kernel", signature = (mask, side, size))]
        pub fn embed_kernel<'py>(py: Python<'py>, mask: Vec<u8>, side: usize, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::embed_kernel(&mask, side, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.
        #[pyfunction]
        #[pyo3(name = "log_spectrum", signature = (field, size))]
        pub fn log_spectrum<'py>(py: Python<'py>, field: Vec<f64>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::log_spectrum(&field, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.
        #[pyfunction]
        #[pyo3(name = "magnitude_spectrum", signature = (field, size))]
        pub fn magnitude_spectrum<'py>(py: Python<'py>, field: Vec<f64>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::magnitude_spectrum(&field, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
        #[pyfunction]
        #[pyo3(name = "peak_ring", signature = (profile))]
        pub fn peak_ring<'py>(py: Python<'py>, profile: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::peak_ring(&profile);
            (out).into_bound_py_any(py)
        }

        /// Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
        #[pyfunction]
        #[pyo3(name = "peak_wavelength", signature = (profile, size))]
        pub fn peak_wavelength<'py>(py: Python<'py>, profile: Vec<f64>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::peak_wavelength(&profile, size);
            (out).into_bound_py_any(py)
        }

        /// Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.
        #[pyfunction]
        #[pyo3(name = "radial_profile", signature = (spectrum, size))]
        pub fn radial_profile<'py>(py: Python<'py>, spectrum: Vec<f64>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::radial_profile(&spectrum, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Transforms a real size-square field forward by fft2, returning the real and imaginary parts.
        #[pyfunction]
        #[pyo3(name = "transform", signature = (field, size))]
        pub fn transform<'py>(py: Python<'py>, field: Vec<f64>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::fft::transform(&field, size);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.fft")?;
            m.setattr("__doc__", "The fast Fourier transform in one and two dimensions.")?;
            m.add_function(wrap_pyfunction!(convolve, &m)?)?;
            m.add_function(wrap_pyfunction!(convolve_with, &m)?)?;
            m.add_function(wrap_pyfunction!(embed_kernel, &m)?)?;
            m.add_function(wrap_pyfunction!(log_spectrum, &m)?)?;
            m.add_function(wrap_pyfunction!(magnitude_spectrum, &m)?)?;
            m.add_function(wrap_pyfunction!(peak_ring, &m)?)?;
            m.add_function(wrap_pyfunction!(peak_wavelength, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_profile, &m)?)?;
            m.add_function(wrap_pyfunction!(transform, &m)?)?;
            let names: Vec<&str> = vec!["convolve", "convolve_with", "embed_kernel", "log_spectrum", "magnitude_spectrum", "peak_ring", "peak_wavelength", "radial_profile", "transform"];
            m.add("__all__", names)?;
            parent.add("fft", &m)?;
            sys.set_item("mrlypy._mrlypy.num.fft", &m)?;
            Ok(())
        }
    }

    /// The primes of the plane: the Gaussian and the Eisenstein integers, their classes, windows and ring weights.
    pub mod gauss {
        use crate::hand::{PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The symmetric window of one ring: every point within a reach, with the norms sieved once.
        #[pyclass(name = "Window", module = "mrlypy.num.gauss", from_py_object)]
        #[derive(Clone)]
        pub struct Window(pub mrlyrs::num::gauss::Window);

        #[pymethods]
        impl Window {
            /// Opens the window of a ring out to a reach, sieving every norm inside it.
            #[new]
            #[pyo3(signature = (ring, radius))]
            pub fn __new__(ring: PySerde<mrlyrs::num::gauss::Ring>, radius: u64) -> PyResult<Self> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Window::new(ring, radius);
                Ok(Self(out))
            }
            /// Counts every class inside.
            #[pyo3(name = "census", signature = ())]
            pub fn census<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::gauss::Window::census(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime.
            #[pyo3(name = "class_", signature = (a, b))]
            pub fn class_<'py>(&self, py: Python<'py>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::gauss::Window::class(&self.0, a, b);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns whether a point lies inside.
            #[pyo3(name = "holds", signature = (a, b))]
            pub fn holds<'py>(&self, py: Python<'py>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::gauss::Window::holds(&self.0, a, b);
                (out).into_bound_py_any(py)
            }
            /// Opens the window of a ring out to a reach, sieving every norm inside it.
            #[staticmethod]
            #[pyo3(name = "new", signature = (ring, radius))]
            pub fn new_<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, radius: u64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Window::new(ring, radius);
                (crate::gen::num::gauss::Window(out)).into_bound_py_any(py)
            }
            /// Lists every point inside, row by row from the bottom left of the bounding square.
            #[pyo3(name = "points", signature = ())]
            pub fn points<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::gauss::Window::points(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the reach.
            #[pyo3(name = "radius", signature = ())]
            pub fn radius<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::gauss::Window::radius(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring.
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::gauss::Window::ring(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// What a point of the ring is.
        #[pyclass(name = "Class", module = "mrlypy.num.gauss", skip_from_py_object)]
        pub struct Class;

        #[pymethods]
        impl Class {
            /// Returns whether the class is prime.
            #[staticmethod]
            #[pyo3(name = "prime", signature = (class_))]
            pub fn prime<'py>(py: Python<'py>, class_: PySerde<mrlyrs::num::gauss::Class>) -> PyResult<Bound<'py, PyAny>> {
                let class_ = class_.0;
                let out = mrlyrs::num::gauss::Class::prime(class_);
                (out).into_bound_py_any(py)
            }
            /// Returns the class as a word.
            #[staticmethod]
            #[pyo3(name = "word", signature = (class_))]
            pub fn word<'py>(py: Python<'py>, class_: PySerde<mrlyrs::num::gauss::Class>) -> PyResult<Bound<'py, PyAny>> {
                let class_ = class_.0;
                let out = mrlyrs::num::gauss::Class::word(class_);
                (out).into_bound_py_any(py)
            }
        }

        /// The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice.
        #[pyclass(name = "Ring", module = "mrlypy.num.gauss", skip_from_py_object)]
        pub struct Ring;

        #[pymethods]
        impl Ring {
            /// Returns the unit multiples of a point, the point first, turning anticlockwise.
            #[staticmethod]
            #[pyo3(name = "associates", signature = (ring, a, b))]
            pub fn associates<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::associates(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin.
            #[staticmethod]
            #[pyo3(name = "canon", signature = (ring, a, b))]
            pub fn canon<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::canon(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the conjugate: the mirror image in the real axis.
            #[staticmethod]
            #[pyo3(name = "conjugate", signature = (ring, a, b))]
            pub fn conjugate<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::conjugate(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of points within the reach: the square or the hexagon.
            #[staticmethod]
            #[pyo3(name = "count", signature = (ring, radius))]
            pub fn count<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, radius: u64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::count(ring, radius);
                (out).into_bound_py_any(py)
            }
            /// Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`.
            #[staticmethod]
            #[pyo3(name = "div_rem", signature = (ring, z, w))]
            pub fn div_rem<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, z: (i64, i64), w: (i64, i64)) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::div_rem(ring, z, w);
                (out).into_bound_py_any(py)
            }
            /// Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.
            #[staticmethod]
            #[pyo3(name = "fate", signature = (ring, n))]
            pub fn fate<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, n: u64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::fate(ring, n);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins.
            #[staticmethod]
            #[pyo3(name = "gaussian_gcd", signature = (ring, z, w))]
            pub fn gaussian_gcd<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, z: (i64, i64), w: (i64, i64)) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::gaussian_gcd(ring, z, w);
                (out).into_bound_py_any(py)
            }
            /// Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3.
            #[staticmethod]
            #[pyo3(name = "inert", signature = (ring, p))]
            pub fn inert<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, p: u64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::inert(ring, p);
                (out).into_bound_py_any(py)
            }
            /// Returns the product of two points.
            #[staticmethod]
            #[pyo3(name = "mul", signature = (ring, arg1, arg2))]
            pub fn mul<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, arg1: (i64, i64), arg2: (i64, i64)) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::mul(ring, arg1, arg2);
                (out).into_bound_py_any(py)
            }
            /// Reads a ring from its name.
            #[staticmethod]
            #[pyo3(name = "named", signature = (name))]
            pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::gauss::Ring::named(name);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
            /// Returns the point nearest a place in the plane.
            #[staticmethod]
            #[pyo3(name = "nearest", signature = (ring, x, y))]
            pub fn nearest<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, x: f64, y: f64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::nearest(ring, x, y);
                (out).into_bound_py_any(py)
            }
            /// Returns the norm of a point: its squared length.
            #[staticmethod]
            #[pyo3(name = "norm", signature = (ring, a, b))]
            pub fn norm<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::norm(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the place of a point in the plane, x right and y up, one unit between neighbours.
            #[staticmethod]
            #[pyo3(name = "place", signature = (ring, a, b))]
            pub fn place<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::place(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the one rational prime that ramifies: 2 or 3.
            #[staticmethod]
            #[pyo3(name = "ramified", signature = (ring))]
            pub fn ramified<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::ramified(ring);
                (out).into_bound_py_any(py)
            }
            /// Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.
            #[staticmethod]
            #[pyo3(name = "reach", signature = (ring, a, b))]
            pub fn reach<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::reach(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.
            #[staticmethod]
            #[pyo3(name = "symmetry", signature = (ring))]
            pub fn symmetry<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::symmetry(ring);
                (out).into_bound_py_any(py)
            }
            /// Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.
            #[staticmethod]
            #[pyo3(name = "top", signature = (ring, radius))]
            pub fn top<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, radius: u64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::top(ring, radius);
                (out).into_bound_py_any(py)
            }
            /// Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.
            #[staticmethod]
            #[pyo3(name = "turn", signature = (ring, a, b))]
            pub fn turn<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::turn(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of units: 4 or 6.
            #[staticmethod]
            #[pyo3(name = "units", signature = (ring))]
            pub fn units<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::units(ring);
                (out).into_bound_py_any(py)
            }
            /// Returns the whole number an associate of the point lies on, when one lies on the positive real axis.
            #[staticmethod]
            #[pyo3(name = "whole", signature = (ring, a, b))]
            pub fn whole<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, a: i64, b: i64) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::gauss::Ring::whole(ring, a, b);
                (out).into_bound_py_any(py)
            }
        }

        /// Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates.
        #[pyfunction]
        #[pyo3(name = "classes", signature = (ring, bound))]
        pub fn classes<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, bound: u64) -> PyResult<Bound<'py, PyAny>> {
            let ring = ring.0;
            let out = mrlyrs::num::gauss::classes(ring, bound);
            (out).into_bound_py_any(py)
        }

        /// Returns the norm from one through the limit with the most points and that count, the earliest on a tie.
        #[pyfunction]
        #[pyo3(name = "peak", signature = (ring, limit))]
        pub fn peak<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let ring = ring.0;
            let out = mrlyrs::num::gauss::peak(ring, limit);
            (out).into_bound_py_any(py)
        }

        /// Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.
        #[pyfunction]
        #[pyo3(name = "shells", signature = (ring, limit))]
        pub fn shells<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let ring = ring.0;
            let out = mrlyrs::num::gauss::shells(ring, limit);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.gauss")?;
            m.setattr("__doc__", "The primes of the plane: the Gaussian and the Eisenstein integers, their classes, windows and ring weights.")?;
            m.add_class::<Window>()?;
            m.add_class::<Class>()?;
            m.add_class::<Ring>()?;
            m.add_function(wrap_pyfunction!(classes, &m)?)?;
            m.add_function(wrap_pyfunction!(peak, &m)?)?;
            m.add_function(wrap_pyfunction!(shells, &m)?)?;
            let names: Vec<&str> = vec!["classes", "peak", "shells", "Window", "Class", "Ring"];
            m.add("__all__", names)?;
            parent.add("gauss", &m)?;
            sys.set_item("mrlypy._mrlypy.num.gauss", &m)?;
            Ok(())
        }
    }

    /// The peeled ladder: the Dirichlet series of a digit design continued to the whole plane, its Lyndon cofactor and the residues at its poles, each value carrying its bound.
    pub mod ladder {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A digit design: the base `q`, the digit set `F` its elements are written with, and the peel depth `P` its ladder starts at.
        #[pyclass(name = "Design", module = "mrlypy.num.ladder", from_py_object)]
        #[derive(Clone)]
        pub struct Design(pub mrlyrs::num::ladder::Design);

        #[pymethods]
        impl Design {
            /// Builds a design on the base and the digit set, choosing the peel depth.
            #[new]
            #[pyo3(signature = (base, digits))]
            pub fn __new__(base: u64, digits: Vec<u64>) -> PyResult<Self> {
                let out = mrlyrs::num::ladder::Design::new(base, &digits);
                Ok(Self(ok(out)?))
            }
            /// Returns the abscissa `alpha = log_q k`.
            #[pyo3(name = "abscissa", signature = ())]
            pub fn abscissa<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::abscissa(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the base.
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::base(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the digit set, ascending.
            #[pyo3(name = "digits", signature = ())]
            pub fn digits<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::digits(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Builds a design on the base and the digit set, choosing the peel depth.
            #[staticmethod]
            #[pyo3(name = "new", signature = (base, digits))]
            pub fn new_<'py>(py: Python<'py>, base: u64, digits: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::new(base, &digits);
                (crate::gen::num::ladder::Design(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the peel depth.
            #[pyo3(name = "peel", signature = ())]
            pub fn peel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::peel(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the pole spacing `2 pi / log q`.
            #[pyo3(name = "period", signature = ())]
            pub fn period<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::period(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`.
            #[pyo3(name = "pole", signature = (m, j))]
            pub fn pole<'py>(&self, py: Python<'py>, m: usize, j: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::pole(&self.0, m, j);
                (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
            }
            /// Builds a design at an explicit peel depth, at least two.
            #[staticmethod]
            #[pyo3(name = "with_peel", signature = (base, digits, peel))]
            pub fn with_peel<'py>(py: Python<'py>, base: u64, digits: Vec<u64>, peel: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::ladder::Design::with_peel(base, &digits, peel);
                (crate::gen::num::ladder::Design(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to.
        #[pyfunction]
        #[pyo3(name = "cofactor", signature = (design, s, tolerance))]
        pub fn cofactor<'py>(py: Python<'py>, design: PyRef<'_, crate::gen::num::ladder::Design>, s: crate::gen::num::zeta::Complex, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
            let s = s.0;
            let out = mrlyrs::num::ladder::cofactor(&design.0, s, tolerance);
            ({ let t = ok(out)?; (crate::gen::num::zeta::Complex(t.0), t.1) }).into_bound_py_any(py)
        }

        /// Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to.
        #[pyfunction]
        #[pyo3(name = "residue", signature = (design, m, j, tolerance))]
        pub fn residue<'py>(py: Python<'py>, design: PyRef<'_, crate::gen::num::ladder::Design>, m: usize, j: i64, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::ladder::residue(&design.0, m, j, tolerance);
            ({ let t = ok(out)?; (crate::gen::num::zeta::Complex(t.0), t.1) }).into_bound_py_any(py)
        }

        /// Returns `zeta_F(s)` and the bound it is known to.
        #[pyfunction]
        #[pyo3(name = "zeta", signature = (design, s, tolerance))]
        pub fn zeta<'py>(py: Python<'py>, design: PyRef<'_, crate::gen::num::ladder::Design>, s: crate::gen::num::zeta::Complex, tolerance: f64) -> PyResult<Bound<'py, PyAny>> {
            let s = s.0;
            let out = mrlyrs::num::ladder::zeta(&design.0, s, tolerance);
            ({ let t = ok(out)?; (crate::gen::num::zeta::Complex(t.0), t.1) }).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.ladder")?;
            m.setattr("__doc__", "The peeled ladder: the Dirichlet series of a digit design continued to the whole plane, its Lyndon cofactor and the residues at its poles, each value carrying its bound.\n\nA claim witness with no caller in the crate: it witnesses `research/claims/zeros-of-the-design-zeta.md`.")?;
            m.add_class::<Design>()?;
            m.add_function(wrap_pyfunction!(cofactor, &m)?)?;
            m.add_function(wrap_pyfunction!(residue, &m)?)?;
            m.add_function(wrap_pyfunction!(zeta, &m)?)?;
            m.add("ROUNDING", mrlyrs::num::ladder::ROUNDING)?;
            let names: Vec<&str> = vec!["cofactor", "residue", "zeta", "Design", "ROUNDING"];
            m.add("__all__", names)?;
            parent.add("ladder", &m)?;
            sys.set_item("mrlypy._mrlypy.num.ladder", &m)?;
            Ok(())
        }
    }

    /// The visible lattice: coprime pairs, the constant a dimension recovers and the Farey nodes.
    pub mod lattice {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.
        #[pyfunction]
        #[pyo3(name = "coprime_pairs", signature = (n))]
        pub fn coprime_pairs<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::coprime_pairs(n);
            (out).into_bound_py_any(py)
        }

        /// Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending.
        #[pyfunction]
        #[pyo3(name = "farey", signature = (order))]
        pub fn farey<'py>(py: Python<'py>, order: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::farey(order);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.
        #[pyfunction]
        #[pyo3(name = "grid", signature = (n))]
        pub fn grid<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::grid(n);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.
        #[pyfunction]
        #[pyo3(name = "new_nodes", signature = (n))]
        pub fn new_nodes<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::new_nodes(n);
            (out).into_bound_py_any(py)
        }

        /// Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.
        #[pyfunction]
        #[pyo3(name = "pi_estimate", signature = (n))]
        pub fn pi_estimate<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::pi_estimate(n);
            (out).into_bound_py_any(py)
        }

        /// Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one.
        #[pyfunction]
        #[pyo3(name = "recovered", signature = (n, dimension))]
        pub fn recovered<'py>(py: Python<'py>, n: usize, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::recovered(n, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The density the visible count of a window in the dimension walks to, one over zeta of the dimension.
        #[pyfunction]
        #[pyo3(name = "visible_density", signature = (dimension))]
        pub fn visible_density<'py>(py: Python<'py>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::visible_density(dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve.
        #[pyfunction]
        #[pyo3(name = "zeta_factor", signature = (dimension))]
        pub fn zeta_factor<'py>(py: Python<'py>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::zeta_factor(dimension);
            (out).into_bound_py_any(py)
        }

        /// The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one.
        #[pyfunction]
        #[pyo3(name = "zeta_whole", signature = (s))]
        pub fn zeta_whole<'py>(py: Python<'py>, s: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::lattice::zeta_whole(s);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.lattice")?;
            m.setattr("__doc__", "The visible lattice: coprime pairs, the constant a dimension recovers and the Farey nodes.")?;
            m.add_function(wrap_pyfunction!(coprime_pairs, &m)?)?;
            m.add_function(wrap_pyfunction!(farey, &m)?)?;
            m.add_function(wrap_pyfunction!(grid, &m)?)?;
            m.add_function(wrap_pyfunction!(new_nodes, &m)?)?;
            m.add_function(wrap_pyfunction!(pi_estimate, &m)?)?;
            m.add_function(wrap_pyfunction!(recovered, &m)?)?;
            m.add_function(wrap_pyfunction!(visible_density, &m)?)?;
            m.add_function(wrap_pyfunction!(zeta_factor, &m)?)?;
            m.add_function(wrap_pyfunction!(zeta_whole, &m)?)?;
            let names: Vec<&str> = vec!["coprime_pairs", "farey", "grid", "new_nodes", "pi_estimate", "recovered", "visible_density", "zeta_factor", "zeta_whole"];
            m.add("__all__", names)?;
            parent.add("lattice", &m)?;
            sys.set_item("mrlypy._mrlypy.num.lattice", &m)?;
            Ok(())
        }
    }

    /// The memory designs: a rule on `k` consecutive digits, its transfer matrix, the words it accepts and the Perron root that is their dimension.
    pub mod memory {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A rule on `k` consecutive digits of a design word.
        #[pyclass(name = "Rule", module = "mrlypy.num.memory", from_py_object)]
        #[derive(Clone)]
        pub struct Rule(pub mrlyrs::num::memory::Rule);

        #[pymethods]
        impl Rule {
            /// Builds a rule from its dimension, its width and its code.
            #[new]
            #[pyo3(signature = (dimension, width, code))]
            pub fn __new__(dimension: usize, width: usize, code: u64) -> PyResult<Self> {
                let out = mrlyrs::num::memory::Rule::new(dimension, width, code);
                Ok(Self(ok(out)?))
            }
            /// The dimension `D`, one to three.
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            /// The window width `k`, at least one.
            #[getter]
            #[pyo3(name = "width")]
            pub fn width<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.width;
                (value).into_bound_py_any(py)
            }
            /// The window code, bit `w` set when window `w` is allowed.
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            /// Returns whether a word, coarsest digit first, is accepted.
            #[pyo3(name = "accepts", signature = (word))]
            pub fn accepts<'py>(&self, py: Python<'py>, word: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::accepts(&self.0, &word);
                (out).into_bound_py_any(py)
            }
            /// Returns whether the window is allowed, and false for any window out of range.
            #[pyo3(name = "allowed", signature = (window))]
            pub fn allowed<'py>(&self, py: Python<'py>, window: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::allowed(&self.0, window);
                (out).into_bound_py_any(py)
            }
            /// Returns the letters that stand in at least one allowed window.
            #[pyo3(name = "alphabet", signature = ())]
            pub fn alphabet<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::alphabet(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of rules of this shape, `2^(2^(k D))`.
            #[pyo3(name = "codes", signature = ())]
            pub fn codes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::codes(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the rule that allows every window.
            #[staticmethod]
            #[pyo3(name = "full", signature = (dimension, width))]
            pub fn full<'py>(py: Python<'py>, dimension: usize, width: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::full(dimension, width);
                (crate::gen::num::memory::Rule(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the letter count `2^D`, the digit vectors of the cube's corners.
            #[pyo3(name = "letters", signature = ())]
            pub fn letters<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::letters(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a rule from its dimension, its width and its code.
            #[staticmethod]
            #[pyo3(name = "new", signature = (dimension, width, code))]
            pub fn new_<'py>(py: Python<'py>, dimension: usize, width: usize, code: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::new(dimension, width, code);
                (crate::gen::num::memory::Rule(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on.
            #[pyo3(name = "states", signature = ())]
            pub fn states<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::states(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the window count `2^(k D)`.
            #[pyo3(name = "windows", signature = ())]
            pub fn windows<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::memory::Rule::windows(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Returns the count of allowed windows `card W`, the bits the code sets inside its window range.
        #[pyfunction]
        #[pyo3(name = "allowed_windows", signature = (rule))]
        pub fn allowed_windows<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::memory::allowed_windows(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first.
        #[pyfunction]
        #[pyo3(name = "cells", signature = (rule, level))]
        pub fn cells<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::memory::cells(&rule.0, level);
            (out).into_bound_py_any(py)
        }

        /// Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`.
        #[pyfunction]
        #[pyo3(name = "counts", signature = (rule, levels))]
        pub fn counts<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>, levels: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::memory::counts(&rule.0, levels);
            (out).into_bound_py_any(py)
        }

        /// Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count.
        #[pyfunction]
        #[pyo3(name = "exponent", signature = (rule))]
        pub fn exponent<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::memory::exponent(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory.
        #[pyfunction]
        #[pyo3(name = "kappa", signature = (rule))]
        pub fn kappa<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::memory::kappa(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the Perron root of the transfer matrix, the count's growth per level.
        #[pyfunction]
        #[pyo3(name = "perron", signature = (rule))]
        pub fn perron<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::memory::perron(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed.
        #[pyfunction]
        #[pyo3(name = "transfer", signature = (rule))]
        pub fn transfer<'py>(py: Python<'py>, rule: PyRef<'_, crate::gen::num::memory::Rule>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::memory::transfer(&rule.0);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.memory")?;
            m.setattr("__doc__", "The memory designs: a rule on `k` consecutive digits, its transfer matrix, the words it accepts and the Perron root that is their dimension.")?;
            m.add_class::<Rule>()?;
            m.add_function(wrap_pyfunction!(allowed_windows, &m)?)?;
            m.add_function(wrap_pyfunction!(cells, &m)?)?;
            m.add_function(wrap_pyfunction!(counts, &m)?)?;
            m.add_function(wrap_pyfunction!(exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(kappa, &m)?)?;
            m.add_function(wrap_pyfunction!(perron, &m)?)?;
            m.add_function(wrap_pyfunction!(transfer, &m)?)?;
            m.add("SPAN", mrlyrs::num::memory::SPAN)?;
            m.add("SWEEPS", mrlyrs::num::memory::SWEEPS)?;
            m.add("TOLERANCE", mrlyrs::num::memory::TOLERANCE)?;
            let names: Vec<&str> = vec!["allowed_windows", "cells", "counts", "exponent", "kappa", "perron", "transfer", "Rule", "SPAN", "SWEEPS", "TOLERANCE"];
            m.add("__all__", names)?;
            parent.add("memory", &m)?;
            sys.set_item("mrlypy._mrlypy.num.memory", &m)?;
            Ok(())
        }
    }

    /// The Thue-Morse world: the digit rule, the substitution, the plane lifts, the runs and the period-doubling word.
    pub mod morse {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The four ways the word lifts from a line to the plane, one sign at every site.
        #[pyclass(name = "Lift", module = "mrlypy.num.morse", skip_from_py_object)]
        pub struct Lift;

        #[pymethods]
        impl Lift {
            /// Returns every Lift in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::morse::Lift::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the sign at a site, zero for plus one and one for minus one.
            #[staticmethod]
            #[pyo3(name = "at", signature = (lift, i, j))]
            pub fn at<'py>(py: Python<'py>, lift: PySerde<mrlyrs::num::morse::Lift>, i: u64, j: u64) -> PyResult<Bound<'py, PyAny>> {
                let lift = lift.0;
                let out = mrlyrs::num::morse::Lift::at(lift, i, j);
                (out).into_bound_py_any(py)
            }
            /// Returns the lift's formula, written the way the page prints it.
            #[staticmethod]
            #[pyo3(name = "formula", signature = (lift))]
            pub fn formula<'py>(py: Python<'py>, lift: PySerde<mrlyrs::num::morse::Lift>) -> PyResult<Bound<'py, PyAny>> {
                let lift = lift.0;
                let out = mrlyrs::num::morse::Lift::formula(lift);
                (out).into_bound_py_any(py)
            }
        }

        /// Returns the run-boundary word, one wherever a letter differs from the next.
        #[pyfunction]
        #[pyo3(name = "boundary", signature = (word))]
        pub fn boundary<'py>(py: Python<'py>, word: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::boundary(&word);
            (out).into_bound_py_any(py)
        }

        /// Exclusive-ors two grids of the same length, site by site.
        #[pyfunction]
        #[pyo3(name = "difference", signature = (a, b))]
        pub fn difference<'py>(py: Python<'py>, a: Vec<u8>, b: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::difference(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Builds the first letters of the Thue-Morse word by the digit rule.
        #[pyfunction]
        #[pyo3(name = "digits", signature = (length))]
        pub fn digits<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::digits(length);
            (out).into_bound_py_any(py)
        }

        /// Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.
        #[pyfunction]
        #[pyo3(name = "doubling", signature = (length))]
        pub fn doubling<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::doubling(length);
            (out).into_bound_py_any(py)
        }

        /// Counts the sites where two grids of the same length differ.
        #[pyfunction]
        #[pyo3(name = "faults", signature = (a, b))]
        pub fn faults<'py>(py: Python<'py>, a: Vec<u8>, b: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::faults(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Tests a grid against the Kronecker power of its own corner tile.
        #[pyfunction]
        #[pyo3(name = "fold", signature = (grid, side, number))]
        pub fn fold<'py>(py: Python<'py>, grid: Vec<u8>, side: usize, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::fold(&grid, side, number);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
        #[pyfunction]
        #[pyo3(name = "letter", signature = (place))]
        pub fn letter<'py>(py: Python<'py>, place: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::letter(place);
            (out).into_bound_py_any(py)
        }

        /// Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.
        #[pyfunction]
        #[pyo3(name = "lift", signature = (kind, side))]
        pub fn lift<'py>(py: Python<'py>, kind: PySerde<mrlyrs::num::morse::Lift>, side: usize) -> PyResult<Bound<'py, PyAny>> {
            let kind = kind.0;
            let out = mrlyrs::num::morse::lift(kind, side);
            (out).into_bound_py_any(py)
        }

        /// Folds a tile of the side into its Kronecker power at the level, one bit per site.
        #[pyfunction]
        #[pyo3(name = "power", signature = (tile, number, level))]
        pub fn power<'py>(py: Python<'py>, tile: Vec<u8>, number: usize, level: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::power(&tile, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Repeats a tile until it fills a grid of the side.
        #[pyfunction]
        #[pyo3(name = "repeat", signature = (tile, number, side))]
        pub fn repeat<'py>(py: Python<'py>, tile: Vec<u8>, number: usize, side: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::repeat(&tile, number, side);
            (out).into_bound_py_any(py)
        }

        /// Returns the lengths of the maximal blocks of one repeated letter, in order.
        #[pyfunction]
        #[pyo3(name = "runs", signature = (word))]
        pub fn runs<'py>(py: Python<'py>, word: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::runs(&word);
            (out).into_bound_py_any(py)
        }

        /// Returns the substitution stage after the rounds, a word of length two to the rounds.
        #[pyfunction]
        #[pyo3(name = "stage", signature = (rounds))]
        pub fn stage<'py>(py: Python<'py>, rounds: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::stage(rounds);
            (out).into_bound_py_any(py)
        }

        /// Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.
        #[pyfunction]
        #[pyo3(name = "substitution", signature = (length))]
        pub fn substitution<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::substitution(length);
            (out).into_bound_py_any(py)
        }

        /// Blows a grid up by the scale, every site becoming a scale-by-scale block.
        #[pyfunction]
        #[pyo3(name = "upsample", signature = (grid, side, scale))]
        pub fn upsample<'py>(py: Python<'py>, grid: Vec<u8>, side: usize, scale: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::morse::upsample(&grid, side, scale);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.morse")?;
            m.setattr("__doc__", "The Thue-Morse world: the digit rule, the substitution, the plane lifts, the runs and the period-doubling word.")?;
            m.add_class::<Lift>()?;
            m.add_function(wrap_pyfunction!(boundary, &m)?)?;
            m.add_function(wrap_pyfunction!(difference, &m)?)?;
            m.add_function(wrap_pyfunction!(digits, &m)?)?;
            m.add_function(wrap_pyfunction!(doubling, &m)?)?;
            m.add_function(wrap_pyfunction!(faults, &m)?)?;
            m.add_function(wrap_pyfunction!(fold, &m)?)?;
            m.add_function(wrap_pyfunction!(letter, &m)?)?;
            m.add_function(wrap_pyfunction!(lift, &m)?)?;
            m.add_function(wrap_pyfunction!(power, &m)?)?;
            m.add_function(wrap_pyfunction!(repeat, &m)?)?;
            m.add_function(wrap_pyfunction!(runs, &m)?)?;
            m.add_function(wrap_pyfunction!(stage, &m)?)?;
            m.add_function(wrap_pyfunction!(substitution, &m)?)?;
            m.add_function(wrap_pyfunction!(upsample, &m)?)?;
            m.add("LIFTS", (mrlyrs::num::morse::LIFTS).into_iter().map(PySerde).collect::<Vec<_>>())?;
            let names: Vec<&str> = vec!["boundary", "difference", "digits", "doubling", "faults", "fold", "letter", "lift", "power", "repeat", "runs", "stage", "substitution", "upsample", "Lift", "LIFTS"];
            m.add("__all__", names)?;
            parent.add("morse", &m)?;
            sys.set_item("mrlypy._mrlypy.num.morse", &m)?;
            Ok(())
        }
    }

    /// The prime objects: the sieve and its readings, values, ranks, gaps, the counts they make and the shape readings of a number.
    pub mod prime {
        use crate::hand::{PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it.
        #[pyclass(name = "Sieve", module = "mrlypy.num.prime", from_py_object)]
        #[derive(Clone)]
        pub struct Sieve(pub mrlyrs::num::prime::Sieve);

        #[pymethods]
        impl Sieve {
            /// Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.
            #[new]
            #[pyo3(signature = (limit))]
            pub fn __new__(limit: usize) -> PyResult<Self> {
                let out = mrlyrs::num::prime::Sieve::new(limit);
                Ok(Self(out))
            }
            /// Returns the count of numbers marked prime so far.
            #[pyo3(name = "count", signature = ())]
            pub fn count<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::prime::Sieve::count(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns whether every number is settled.
            #[pyo3(name = "done", signature = ())]
            pub fn done<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::prime::Sieve::done(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Runs the sieve to the end.
            #[pyo3(name = "finish", signature = ())]
            pub fn finish<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                mrlyrs::num::prime::Sieve::finish(&mut self.0);
                ().into_bound_py_any(py)
            }
            /// Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.
            #[staticmethod]
            #[pyo3(name = "new", signature = (limit))]
            pub fn new_<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::prime::Sieve::new(limit);
                (crate::gen::num::prime::Sieve(out)).into_bound_py_any(py)
            }
            /// Returns the count of primes used so far.
            #[pyo3(name = "rank", signature = ())]
            pub fn rank<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::prime::Sieve::rank(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done.
            #[pyo3(name = "step", signature = ())]
            pub fn step<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::prime::Sieve::step(&mut self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of numbers the last step struck.
            #[pyo3(name = "struck", signature = ())]
            pub fn struck<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::prime::Sieve::struck(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it.
            #[pyo3(name = "types", signature = ())]
            pub fn types<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::prime::Sieve::types(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last.
        #[pyfunction]
        #[pyo3(name = "chart", signature = (top, bins))]
        pub fn chart<'py>(py: Python<'py>, top: usize, bins: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::chart(top, bins);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag.
        #[pyfunction]
        #[pyo3(name = "flags", signature = (limit))]
        pub fn flags<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::flags(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of unordered pairs of primes summing to the number, zero below four.
        #[pyfunction]
        #[pyo3(name = "goldbach", signature = (number))]
        pub fn goldbach<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::goldbach(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of prime pairs at every even number from four up to the top, one entry per even number.
        #[pyfunction]
        #[pyo3(name = "goldbach_record", signature = (top))]
        pub fn goldbach_record<'py>(py: Python<'py>, top: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::goldbach_record(top);
            (out).into_bound_py_any(py)
        }

        /// Returns whether the number is prime, by trial division on the six-step wheel.
        #[pyfunction]
        #[pyo3(name = "is_prime", signature = (number))]
        pub fn is_prime<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::is_prime(number);
            (out).into_bound_py_any(py)
        }

        /// Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.
        #[pyfunction]
        #[pyo3(name = "pile", signature = (number))]
        pub fn pile<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::pile(number);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Returns the count of primes at or below n.
        #[pyfunction]
        #[pyo3(name = "prime_count", signature = (n))]
        pub fn prime_count<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::prime_count(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the smallest prime at or above the number.
        #[pyfunction]
        #[pyo3(name = "prime_from", signature = (number))]
        pub fn prime_from<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::prime_from(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the primes up to the limit, the finished sieve read as a list.
        #[pyfunction]
        #[pyo3(name = "primes", signature = (limit))]
        pub fn primes<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::primes(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root.
        #[pyfunction]
        #[pyo3(name = "rectangles", signature = (number))]
        pub fn rectangles<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::rectangles(number);
            (out).into_bound_py_any(py)
        }

        /// Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.
        #[pyfunction]
        #[pyo3(name = "splits", signature = (number))]
        pub fn splits<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::splits(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the smallest pair of positive sides whose squares sum to the number, when one exists.
        #[pyfunction]
        #[pyo3(name = "squares", signature = (number))]
        pub fn squares<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::squares(number);
            (out).into_bound_py_any(py)
        }

        /// Returns one prime object for every prime up to and including the limit.
        #[pyfunction]
        #[pyo3(name = "study", signature = (limit))]
        pub fn study<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::prime::study(limit);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.prime")?;
            m.setattr("__doc__", "The prime objects: the sieve and its readings, values, ranks, gaps, the counts they make and the shape readings of a number.")?;
            m.add_class::<Sieve>()?;
            m.add_function(wrap_pyfunction!(chart, &m)?)?;
            m.add_function(wrap_pyfunction!(flags, &m)?)?;
            m.add_function(wrap_pyfunction!(goldbach, &m)?)?;
            m.add_function(wrap_pyfunction!(goldbach_record, &m)?)?;
            m.add_function(wrap_pyfunction!(is_prime, &m)?)?;
            m.add_function(wrap_pyfunction!(pile, &m)?)?;
            m.add_function(wrap_pyfunction!(prime_count, &m)?)?;
            m.add_function(wrap_pyfunction!(prime_from, &m)?)?;
            m.add_function(wrap_pyfunction!(primes, &m)?)?;
            m.add_function(wrap_pyfunction!(rectangles, &m)?)?;
            m.add_function(wrap_pyfunction!(splits, &m)?)?;
            m.add_function(wrap_pyfunction!(squares, &m)?)?;
            m.add_function(wrap_pyfunction!(study, &m)?)?;
            let names: Vec<&str> = vec!["chart", "flags", "goldbach", "goldbach_record", "is_prime", "pile", "prime_count", "prime_from", "primes", "rectangles", "splits", "squares", "study", "Sieve"];
            m.add("__all__", names)?;
            parent.add("prime", &m)?;
            sys.set_item("mrlypy._mrlypy.num.prime", &m)?;
            Ok(())
        }
    }

    /// The radix designs: a digit set inside the residues of a base in a ring, a unit twist per digit, and the points their words land on.
    pub mod radix {
        use crate::hand::{ok, PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// The base of a radix design: a ring and an element of norm at least two, the scale every word is read against.
        #[pyclass(name = "Base", module = "mrlypy.num.radix", from_py_object)]
        #[derive(Clone)]
        pub struct Base(pub mrlyrs::num::radix::Base);

        #[pymethods]
        impl Base {
            /// Fixes a base in a ring.
            #[new]
            #[pyo3(signature = (ring, value))]
            pub fn __new__(ring: PySerde<mrlyrs::num::gauss::Ring>, value: (i64, i64)) -> PyResult<Self> {
                let ring = ring.0;
                let out = mrlyrs::num::radix::Base::new(ring, value);
                Ok(Self(ok(out)?))
            }
            /// Returns the index in the canonical residue system of the class of a point.
            #[pyo3(name = "class_", signature = (z))]
            pub fn class_<'py>(&self, py: Python<'py>, z: (i64, i64)) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::class(self.0, z);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns whether two points are congruent modulo the base.
            #[pyo3(name = "congruent", signature = (z, w))]
            pub fn congruent<'py>(&self, py: Python<'py>, z: (i64, i64), w: (i64, i64)) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::congruent(self.0, z, w);
                (out).into_bound_py_any(py)
            }
            /// Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base.
            #[pyo3(name = "group", signature = ())]
            pub fn group<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::group(self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group.
            #[pyo3(name = "mirrored", signature = ())]
            pub fn mirrored<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::mirrored(self.0);
                (out).into_bound_py_any(py)
            }
            /// Fixes a base in a ring.
            #[staticmethod]
            #[pyo3(name = "new", signature = (ring, value))]
            pub fn new_<'py>(py: Python<'py>, ring: PySerde<mrlyrs::num::gauss::Ring>, value: (i64, i64)) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrlyrs::num::radix::Base::new(ring, value);
                (crate::gen::num::radix::Base(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the norm `q` of the base: the count of residue classes and the square of the scale.
            #[pyo3(name = "norm", signature = ())]
            pub fn norm<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::norm(self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the base raised to a level.
            #[pyo3(name = "power", signature = (level))]
            pub fn power<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::power(self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`.
            #[pyo3(name = "residues", signature = ())]
            pub fn residues<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::residues(self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the ring.
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::ring(self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the base element.
            #[pyo3(name = "value", signature = ())]
            pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Base::value(self.0);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// A radix design: a digit set inside one ring, placed by a base with a unit twist per digit.
        #[pyclass(name = "Radix", module = "mrlypy.num.radix", from_py_object)]
        #[derive(Clone)]
        pub struct Radix(pub mrlyrs::num::radix::Radix);

        #[pymethods]
        impl Radix {
            /// Builds a design from a base, a digit list and a unit twist per digit.
            #[new]
            #[pyo3(signature = (base, digits, twists))]
            pub fn __new__(base: crate::gen::num::radix::Base, digits: Vec<(i64, i64)>, twists: Vec<(i64, i64)>) -> PyResult<Self> {
                let base = base.0;
                let out = mrlyrs::num::radix::Radix::new(base, digits, twists);
                Ok(Self(ok(out)?))
            }
            /// Returns the base.
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::base(&self.0);
                (crate::gen::num::radix::Base(out)).into_bound_py_any(py)
            }
            /// Returns whether every digit is the canonical representative of its class.
            #[pyo3(name = "canonical", signature = ())]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::canonical(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives.
            #[pyo3(name = "code", signature = ())]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::code(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the digits.
            #[pyo3(name = "digits", signature = ())]
            pub fn digits<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::digits(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base.
            #[pyo3(name = "dimension", signature = ())]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::dimension(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point.
            #[pyo3(name = "distinct", signature = (level))]
            pub fn distinct<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::distinct(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of words of a level, `|F|^L`.
            #[pyo3(name = "fill", signature = (level))]
            pub fn fill<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::fill(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`.
            #[staticmethod]
            #[pyo3(name = "from_code", signature = (base, code))]
            pub fn from_code<'py>(py: Python<'py>, base: crate::gen::num::radix::Base, code: u128) -> PyResult<Bound<'py, PyAny>> {
                let base = base.0;
                let out = mrlyrs::num::radix::Radix::from_code(base, code);
                (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
            }
            /// Builds a design from a base, a digit list and a unit twist per digit.
            #[staticmethod]
            #[pyo3(name = "new", signature = (base, digits, twists))]
            pub fn new_<'py>(py: Python<'py>, base: crate::gen::num::radix::Base, digits: Vec<(i64, i64)>, twists: Vec<(i64, i64)>) -> PyResult<Bound<'py, PyAny>> {
                let base = base.0;
                let out = mrlyrs::num::radix::Radix::new(base, digits, twists);
                (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the level-`L` points in the plane, the scaled words divided by `b^L`.
            #[pyo3(name = "plane", signature = (level))]
            pub fn plane<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::plane(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring.
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::ring(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the digit count `|F|`.
            #[pyo3(name = "size", signature = ())]
            pub fn size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::size(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the twists.
            #[pyo3(name = "twists", signature = ())]
            pub fn twists<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::twists(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Returns the design with the twists named by their index in the unit list, the units in turning order from one.
            #[pyo3(name = "with_twists", signature = (units))]
            pub fn with_twists<'py>(&self, py: Python<'py>, units: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::with_twists(self.0.clone(), &units);
                (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the level-`L` points in exact ring coordinates scaled by `b^L`.
            #[pyo3(name = "words", signature = (level))]
            pub fn words<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::radix::Radix::words(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`.
        #[pyfunction]
        #[pyo3(name = "flowsnake", signature = ())]
        pub fn flowsnake<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::radix::flowsnake();
            (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`.
        #[pyfunction]
        #[pyo3(name = "gasket", signature = ())]
        pub fn gasket<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::radix::gasket();
            (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`.
        #[pyfunction]
        #[pyo3(name = "koch", signature = ())]
        pub fn koch<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::radix::koch();
            (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`.
        #[pyfunction]
        #[pyo3(name = "terdragon", signature = ())]
        pub fn terdragon<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::radix::terdragon();
            (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`.
        #[pyfunction]
        #[pyo3(name = "tile", signature = (m, code))]
        pub fn tile<'py>(py: Python<'py>, m: u64, code: u128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::radix::tile(m, code);
            (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`.
        #[pyfunction]
        #[pyo3(name = "twindragon", signature = ())]
        pub fn twindragon<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::radix::twindragon();
            (crate::gen::num::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.radix")?;
            m.setattr("__doc__", "The radix designs: a digit set inside the residues of a base in a ring, a unit twist per digit, and the points their words land on.")?;
            m.add_class::<Base>()?;
            m.add_class::<Radix>()?;
            m.add_function(wrap_pyfunction!(flowsnake, &m)?)?;
            m.add_function(wrap_pyfunction!(gasket, &m)?)?;
            m.add_function(wrap_pyfunction!(koch, &m)?)?;
            m.add_function(wrap_pyfunction!(terdragon, &m)?)?;
            m.add_function(wrap_pyfunction!(tile, &m)?)?;
            m.add_function(wrap_pyfunction!(twindragon, &m)?)?;
            let names: Vec<&str> = vec!["flowsnake", "gasket", "koch", "terdragon", "tile", "twindragon", "Base", "Radix"];
            m.add("__all__", names)?;
            parent.add("radix", &m)?;
            sys.set_item("mrlypy._mrlypy.num.radix", &m)?;
            Ok(())
        }
    }

    /// The infinite sums: the classic sequences, zeta and its Dirichlet cousins, the partials that walk to pi, e and gamma, the visible count and the Bernoulli fractions.
    pub mod series {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.
        #[pyfunction]
        #[pyo3(name = "basel", signature = (n))]
        pub fn basel<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::basel(n);
            (out).into_bound_py_any(py)
        }

        /// Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.
        #[pyfunction]
        #[pyo3(name = "bernoulli", signature = (count))]
        pub fn bernoulli<'py>(py: Python<'py>, count: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::bernoulli(count);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums.
        #[pyfunction]
        #[pyo3(name = "beta", signature = (s, terms))]
        pub fn beta<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::beta(s, terms);
            (out).into_bound_py_any(py)
        }

        /// Returns the powers of two up to the limit.
        #[pyfunction]
        #[pyo3(name = "binary", signature = (limit))]
        pub fn binary<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::binary(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the distinct Catalan numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "catalan", signature = (limit))]
        pub fn catalan<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::catalan(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the mod-three rhythm of the number: zero, one, minus one.
        #[pyfunction]
        #[pyo3(name = "chi3", signature = (number))]
        pub fn chi3<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::chi3(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the mod-four rhythm of the number: zero, one, zero, minus one.
        #[pyfunction]
        #[pyo3(name = "chi4", signature = (number))]
        pub fn chi4<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::chi4(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.
        #[pyfunction]
        #[pyo3(name = "chi8", signature = (number))]
        pub fn chi8<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::chi8(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the L-series partial sum with a periodic rhythm painted on the terms.
        #[pyfunction]
        #[pyo3(name = "dirichlet", signature = (s, rhythm, terms))]
        pub fn dirichlet<'py>(py: Python<'py>, s: f64, rhythm: Vec<i8>, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::dirichlet(s, &rhythm, terms);
            (out).into_bound_py_any(py)
        }

        /// Returns one plus one over n raised to the n, walking to the natural base.
        #[pyfunction]
        #[pyo3(name = "e_partial", signature = (n))]
        pub fn e_partial<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::e_partial(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.
        #[pyfunction]
        #[pyo3(name = "euler_gamma_partial", signature = (n))]
        pub fn euler_gamma_partial<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::euler_gamma_partial(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.
        #[pyfunction]
        #[pyo3(name = "euler_product", signature = (s, limit))]
        pub fn euler_product<'py>(py: Python<'py>, s: f64, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::euler_product(s, limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the even numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "evens", signature = (limit))]
        pub fn evens<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::evens(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the distinct Fibonacci numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "fibonacci", signature = (limit))]
        pub fn fibonacci<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::fibonacci(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the partial harmonic sum, the reciprocals of one through the term count.
        #[pyfunction]
        #[pyo3(name = "harmonic", signature = (terms))]
        pub fn harmonic<'py>(py: Python<'py>, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::harmonic(terms);
            (out).into_bound_py_any(py)
        }

        /// Returns the Dirichlet lambda value, one minus two to the minus s times zeta.
        #[pyfunction]
        #[pyo3(name = "lambda_", signature = (s, terms))]
        pub fn lambda_<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::lambda(s, terms);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.
        #[pyfunction]
        #[pyo3(name = "leibniz", signature = (n))]
        pub fn leibniz<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::leibniz(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.
        #[pyfunction]
        #[pyo3(name = "li", signature = (x))]
        pub fn li<'py>(py: Python<'py>, x: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::li(x);
            (out).into_bound_py_any(py)
        }

        /// Returns the Mertens function at n, the Mobius values of one through n summed.
        #[pyfunction]
        #[pyo3(name = "mertens", signature = (n))]
        pub fn mertens<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::mertens(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the odd numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "odds", signature = (limit))]
        pub fn odds<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::odds(limit);
            (out).into_bound_py_any(py)
        }

        /// Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.
        #[pyfunction]
        #[pyo3(name = "visible", signature = (limit, dimension))]
        pub fn visible<'py>(py: Python<'py>, limit: usize, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::visible(limit, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.
        #[pyfunction]
        #[pyo3(name = "wallis_half_pi", signature = (n))]
        pub fn wallis_half_pi<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::wallis_half_pi(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.
        #[pyfunction]
        #[pyo3(name = "wallis_quarter_pi", signature = (factors))]
        pub fn wallis_quarter_pi<'py>(py: Python<'py>, factors: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::wallis_quarter_pi(factors);
            (out).into_bound_py_any(py)
        }

        /// Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.
        #[pyfunction]
        #[pyo3(name = "zeta", signature = (s, terms))]
        pub fn zeta<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::series::zeta(s, terms);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.series")?;
            m.setattr("__doc__", "The infinite sums: the classic sequences, zeta and its Dirichlet cousins, the partials that walk to pi, e and gamma, the visible count and the Bernoulli fractions.")?;
            m.add_function(wrap_pyfunction!(basel, &m)?)?;
            m.add_function(wrap_pyfunction!(bernoulli, &m)?)?;
            m.add_function(wrap_pyfunction!(beta, &m)?)?;
            m.add_function(wrap_pyfunction!(binary, &m)?)?;
            m.add_function(wrap_pyfunction!(catalan, &m)?)?;
            m.add_function(wrap_pyfunction!(chi3, &m)?)?;
            m.add_function(wrap_pyfunction!(chi4, &m)?)?;
            m.add_function(wrap_pyfunction!(chi8, &m)?)?;
            m.add_function(wrap_pyfunction!(dirichlet, &m)?)?;
            m.add_function(wrap_pyfunction!(e_partial, &m)?)?;
            m.add_function(wrap_pyfunction!(euler_gamma_partial, &m)?)?;
            m.add_function(wrap_pyfunction!(euler_product, &m)?)?;
            m.add_function(wrap_pyfunction!(evens, &m)?)?;
            m.add_function(wrap_pyfunction!(fibonacci, &m)?)?;
            m.add_function(wrap_pyfunction!(harmonic, &m)?)?;
            m.add_function(wrap_pyfunction!(lambda_, &m)?)?;
            m.add_function(wrap_pyfunction!(leibniz, &m)?)?;
            m.add_function(wrap_pyfunction!(li, &m)?)?;
            m.add_function(wrap_pyfunction!(mertens, &m)?)?;
            m.add_function(wrap_pyfunction!(odds, &m)?)?;
            m.add_function(wrap_pyfunction!(visible, &m)?)?;
            m.add_function(wrap_pyfunction!(wallis_half_pi, &m)?)?;
            m.add_function(wrap_pyfunction!(wallis_quarter_pi, &m)?)?;
            m.add_function(wrap_pyfunction!(zeta, &m)?)?;
            m.add("APERY", mrlyrs::num::series::APERY)?;
            m.add("BASEL", mrlyrs::num::series::BASEL)?;
            m.add("CATALAN", mrlyrs::num::series::CATALAN)?;
            m.add("EULER", mrlyrs::num::series::EULER)?;
            m.add("VISIBLE", mrlyrs::num::series::VISIBLE)?;
            let names: Vec<&str> = vec!["basel", "bernoulli", "beta", "binary", "catalan", "chi3", "chi4", "chi8", "dirichlet", "e_partial", "euler_gamma_partial", "euler_product", "evens", "fibonacci", "harmonic", "lambda_", "leibniz", "li", "mertens", "odds", "visible", "wallis_half_pi", "wallis_quarter_pi", "zeta", "APERY", "BASEL", "CATALAN", "EULER", "VISIBLE"];
            m.add("__all__", names)?;
            parent.add("series", &m)?;
            sys.set_item("mrlypy._mrlypy.num.series", &m)?;
            Ok(())
        }
    }

    /// The punctured schedules: the Wallis sieve and its kin, their words, rasters, punctures and limits.
    pub mod sieve {
        use crate::hand::{ok};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.
        #[pyfunction]
        #[pyo3(name = "cells", signature = (word, dimension))]
        pub fn cells<'py>(py: Python<'py>, word: Vec<u64>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::cells(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
        #[pyfunction]
        #[pyo3(name = "exponent", signature = (word, dimension))]
        pub fn exponent<'py>(py: Python<'py>, word: Vec<u64>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::exponent(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
        #[pyfunction]
        #[pyo3(name = "flat_word", signature = (side, levels))]
        pub fn flat_word<'py>(py: Python<'py>, side: u64, levels: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::flat_word(side, levels);
            (out).into_bound_py_any(py)
        }

        /// Returns the punctures the word makes, one per surviving cell at every level.
        #[pyfunction]
        #[pyo3(name = "holes", signature = (word, dimension))]
        pub fn holes<'py>(py: Python<'py>, word: Vec<u64>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::holes(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.
        #[pyfunction]
        #[pyo3(name = "limit", signature = (word, dimension))]
        pub fn limit<'py>(py: Python<'py>, word: Vec<u64>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::limit(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
        #[pyfunction]
        #[pyo3(name = "odd_word", signature = (levels))]
        pub fn odd_word<'py>(py: Python<'py>, levels: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::odd_word(levels);
            (out).into_bound_py_any(py)
        }

        /// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
        #[pyfunction]
        #[pyo3(name = "punctures", signature = (word, dimension))]
        pub fn punctures<'py>(py: Python<'py>, word: Vec<u64>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::punctures(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
        #[pyfunction]
        #[pyo3(name = "raster", signature = (word))]
        pub fn raster<'py>(py: Python<'py>, word: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::raster(&word);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
        #[pyfunction]
        #[pyo3(name = "ratio", signature = (word, dimension))]
        pub fn ratio<'py>(py: Python<'py>, word: Vec<u64>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::ratio(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the side of the word, the product of its letters' sides.
        #[pyfunction]
        #[pyo3(name = "side", signature = (word))]
        pub fn side<'py>(py: Python<'py>, word: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::side(&word);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.
        #[pyfunction]
        #[pyo3(name = "solid_limit", signature = ())]
        pub fn solid_limit<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::sieve::solid_limit();
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.sieve")?;
            m.setattr("__doc__", "The punctured schedules: the Wallis sieve and its kin, their words, rasters, punctures and limits.")?;
            m.add_function(wrap_pyfunction!(cells, &m)?)?;
            m.add_function(wrap_pyfunction!(exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(flat_word, &m)?)?;
            m.add_function(wrap_pyfunction!(holes, &m)?)?;
            m.add_function(wrap_pyfunction!(limit, &m)?)?;
            m.add_function(wrap_pyfunction!(odd_word, &m)?)?;
            m.add_function(wrap_pyfunction!(punctures, &m)?)?;
            m.add_function(wrap_pyfunction!(raster, &m)?)?;
            m.add_function(wrap_pyfunction!(ratio, &m)?)?;
            m.add_function(wrap_pyfunction!(side, &m)?)?;
            m.add_function(wrap_pyfunction!(solid_limit, &m)?)?;
            m.add("PLANE_LIMIT", mrlyrs::num::sieve::PLANE_LIMIT)?;
            let names: Vec<&str> = vec!["cells", "exponent", "flat_word", "holes", "limit", "odd_word", "punctures", "raster", "ratio", "side", "solid_limit", "PLANE_LIMIT"];
            m.add("__all__", names)?;
            parent.add("sieve", &m)?;
            sys.set_item("mrlypy._mrlypy.num.sieve", &m)?;
            Ok(())
        }
    }

    /// The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.
    pub mod spiral {
        use crate::hand::{PySerde};
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// Which cells of the square winding grow into a tile.
        #[pyclass(name = "Growth", module = "mrlypy.num.spiral", skip_from_py_object)]
        pub struct Growth;

        #[pymethods]
        impl Growth {
            /// Returns every Growth in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::spiral::Growth::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right.
        #[pyclass(name = "Lattice", module = "mrlypy.num.spiral", skip_from_py_object)]
        pub struct Lattice;

        #[pymethods]
        impl Lattice {
            /// Returns every Lattice in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::spiral::Lattice::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.
            #[staticmethod]
            #[pyo3(name = "count", signature = (lattice, side))]
            pub fn count<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::num::spiral::Lattice>, side: usize) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::num::spiral::Lattice::count(lattice, side);
                (out).into_bound_py_any(py)
            }
            /// Returns the number at a cell, one at the origin.
            #[staticmethod]
            #[pyo3(name = "n", signature = (lattice, x, y))]
            pub fn n<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::num::spiral::Lattice>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::num::spiral::Lattice::n(lattice, x, y);
                (out).into_bound_py_any(py)
            }
            /// Returns the outermost ring of a sheet the odd side wide, half the side rounded down.
            #[staticmethod]
            #[pyo3(name = "radius", signature = (lattice, side))]
            pub fn radius<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::num::spiral::Lattice>, side: usize) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::num::spiral::Lattice::radius(lattice, side);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring a number sits on, zero for one.
            #[staticmethod]
            #[pyo3(name = "ring", signature = (lattice, n))]
            pub fn ring<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::num::spiral::Lattice>, n: u64) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::num::spiral::Lattice::ring(lattice, n);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.
            #[staticmethod]
            #[pyo3(name = "ring_of", signature = (lattice, x, y))]
            pub fn ring_of<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::num::spiral::Lattice>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::num::spiral::Lattice::ring_of(lattice, x, y);
                (out).into_bound_py_any(py)
            }
            /// Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.
            #[staticmethod]
            #[pyo3(name = "xy", signature = (lattice, n))]
            pub fn xy<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::num::spiral::Lattice>, n: u64) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrlyrs::num::spiral::Lattice::xy(lattice, n);
                (out).into_bound_py_any(py)
            }
        }

        /// What a cell is painted for.
        #[pyclass(name = "Mark", module = "mrlypy.num.spiral", skip_from_py_object)]
        pub struct Mark;

        #[pymethods]
        impl Mark {
            /// Returns every Mark in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::spiral::Mark::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak.
        #[pyfunction]
        #[pyo3(name = "diagonal", signature = (lattice, side, a, b, c))]
        pub fn diagonal<'py>(py: Python<'py>, lattice: PySerde<mrlyrs::num::spiral::Lattice>, side: usize, a: i64, b: i64, c: i64) -> PyResult<Bound<'py, PyAny>> {
            let lattice = lattice.0;
            let out = mrlyrs::num::spiral::diagonal(lattice, side, a, b, c);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself.
        #[pyfunction]
        #[pyo3(name = "level_of", signature = (n, base))]
        pub fn level_of<'py>(py: Python<'py>, n: u64, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::spiral::level_of(n, base);
            (out).into_bound_py_any(py)
        }

        /// Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.
        #[pyfunction]
        #[pyo3(name = "marks", signature = (mark, limit))]
        pub fn marks<'py>(py: Python<'py>, mark: PySerde<mrlyrs::num::spiral::Mark>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let mark = mark.0;
            let out = mrlyrs::num::spiral::marks(mark, limit);
            (out).into_bound_py_any(py)
        }

        /// Winds one to the top on the square spiral and lays a square tile on every cell, the snail.
        #[pyfunction]
        #[pyo3(name = "snail", signature = (base, top, growth))]
        pub fn snail<'py>(py: Python<'py>, base: u64, top: u64, growth: PySerde<mrlyrs::num::spiral::Growth>) -> PyResult<Bound<'py, PyAny>> {
            let growth = growth.0;
            let out = mrlyrs::num::spiral::snail(base, top, growth);
            (PySerde(out)).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.spiral")?;
            m.setattr("__doc__", "The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.")?;
            m.add_class::<Growth>()?;
            m.add_class::<Lattice>()?;
            m.add_class::<Mark>()?;
            m.add_function(wrap_pyfunction!(diagonal, &m)?)?;
            m.add_function(wrap_pyfunction!(level_of, &m)?)?;
            m.add_function(wrap_pyfunction!(marks, &m)?)?;
            m.add_function(wrap_pyfunction!(snail, &m)?)?;
            let names: Vec<&str> = vec!["diagonal", "level_of", "marks", "snail", "Growth", "Lattice", "Mark"];
            m.add("__all__", names)?;
            parent.add("spiral", &m)?;
            sys.set_item("mrlypy._mrlypy.num.spiral", &m)?;
            Ok(())
        }
    }

    /// The critical line: zeta at one half plus i t and off it, its zeros, the prime staircase they rebuild and the novelty meter their waves predict.
    pub mod zeta {
        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::IntoPyObjectExt;

        /// A complex number: a real and an imaginary part.
        #[pyclass(name = "Complex", module = "mrlypy.num.zeta", from_py_object)]
        #[derive(Clone)]
        pub struct Complex(pub mrlyrs::num::zeta::Complex);

        #[pymethods]
        impl Complex {
            /// Builds a complex number from its parts.
            #[new]
            #[pyo3(signature = (re, im))]
            pub fn __new__(re: f64, im: f64) -> PyResult<Self> {
                let out = mrlyrs::num::zeta::Complex::new(re, im);
                Ok(Self(out))
            }
            /// The real part.
            #[getter]
            #[pyo3(name = "re")]
            pub fn re<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.re;
                (value).into_bound_py_any(py)
            }
            /// The imaginary part.
            #[getter]
            #[pyo3(name = "im")]
            pub fn im<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.im;
                (value).into_bound_py_any(py)
            }
            /// Returns the modulus.
            #[pyo3(name = "abs", signature = ())]
            pub fn abs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Complex::abs(self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the principal argument.
            #[pyo3(name = "arg", signature = ())]
            pub fn arg<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Complex::arg(self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the exponential.
            #[pyo3(name = "exp", signature = ())]
            pub fn exp<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Complex::exp(self.0);
                (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
            }
            /// Returns the principal logarithm.
            #[pyo3(name = "ln", signature = ())]
            pub fn ln<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Complex::ln(self.0);
                (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
            }
            /// Builds a complex number from its parts.
            #[staticmethod]
            #[pyo3(name = "new", signature = (re, im))]
            pub fn new_<'py>(py: Python<'py>, re: f64, im: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Complex::new(re, im);
                (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
            }
            /// Returns a unit complex number at the given angle.
            #[staticmethod]
            #[pyo3(name = "turn", signature = (angle))]
            pub fn turn<'py>(py: Python<'py>, angle: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Complex::turn(angle);
                (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once.
        #[pyclass(name = "Line", module = "mrlypy.num.zeta", skip_from_py_object)]
        pub struct Line(pub mrlyrs::num::zeta::Line);

        #[pymethods]
        impl Line {
            /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
            #[new]
            #[pyo3(signature = ())]
            pub fn __new__() -> PyResult<Self> {
                let out = mrlyrs::num::zeta::Line::new();
                Ok(Self(out))
            }
            /// Counts the zeros on the line below t.
            #[pyo3(name = "count", signature = (t))]
            pub fn count<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::count(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns Z(t) from the Euler-Maclaurin value turned onto the real axis.
            #[pyo3(name = "exact", signature = (t))]
            pub fn exact<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::exact(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns the n-th Gram point, where theta is n pi, by Newton from the right.
            #[pyo3(name = "gram", signature = (n))]
            pub fn gram<'py>(&self, py: Python<'py>, n: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::gram(&self.0, n);
                (out).into_bound_py_any(py)
            }
            /// Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections.
            #[pyo3(name = "maclaurin", signature = (t))]
            pub fn maclaurin<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::maclaurin(&self.0, t);
                (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
            }
            /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
            #[staticmethod]
            #[pyo3(name = "new", signature = ())]
            pub fn new_<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::new();
                (crate::gen::num::zeta::Line(out)).into_bound_py_any(py)
            }
            /// Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump.
            #[pyo3(name = "novelty_coefficients", signature = (gammas))]
            pub fn novelty_coefficients<'py>(&self, py: Python<'py>, gammas: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::novelty_coefficients(&self.0, &gammas);
                ((out).into_iter().map(crate::gen::num::zeta::Complex).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s.
            #[pyo3(name = "pair", signature = (s))]
            pub fn pair<'py>(&self, py: Python<'py>, s: crate::gen::num::zeta::Complex) -> PyResult<Bound<'py, PyAny>> {
                let s = s.0;
                let out = mrlyrs::num::zeta::Line::pair(&self.0, s);
                ({ let t = out; (crate::gen::num::zeta::Complex(t.0), crate::gen::num::zeta::Complex(t.1)) }).into_bound_py_any(py)
            }
            /// Returns zeta on the line and Z(t) together, from the engine that serves the t.
            #[pyo3(name = "point", signature = (t))]
            pub fn point<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::point(&self.0, t);
                ({ let t = out; (crate::gen::num::zeta::Complex(t.0), t.1) }).into_bound_py_any(py)
            }
            /// Returns the largest gap between the two engines over the t range on a grid.
            #[pyo3(name = "seam", signature = (t0, t1, steps))]
            pub fn seam<'py>(&self, py: Python<'py>, t0: f64, t1: f64, steps: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::seam(&self.0, t0, t1, steps);
                (out).into_bound_py_any(py)
            }
            /// Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections.
            #[pyo3(name = "siegel", signature = (t))]
            pub fn siegel<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::siegel(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten.
            #[pyo3(name = "theta", signature = (t))]
            pub fn theta<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::theta(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above.
            #[pyo3(name = "z", signature = (t))]
            pub fn z<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::z(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth.
            #[pyo3(name = "zeros", signature = (count))]
            pub fn zeros<'py>(&self, py: Python<'py>, count: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrlyrs::num::zeta::Line::zeros(&self.0, count);
                (out).into_bound_py_any(py)
            }
            /// Reads plain data into the class.
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2.
        #[pyfunction]
        #[pyo3(name = "bump", signature = (u))]
        pub fn bump<'py>(py: Python<'py>, u: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::bump(u);
            (out).into_bound_py_any(py)
        }

        /// Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.
        #[pyfunction]
        #[pyo3(name = "corrections", signature = (p))]
        pub fn corrections<'py>(py: Python<'py>, p: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::corrections(p);
            (out).into_bound_py_any(py)
        }

        /// Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.
        #[pyfunction]
        #[pyo3(name = "kernel", signature = (p))]
        pub fn kernel<'py>(py: Python<'py>, p: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::kernel(p);
            (out).into_bound_py_any(py)
        }

        /// Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule.
        #[pyfunction]
        #[pyo3(name = "mellin", signature = (s))]
        pub fn mellin<'py>(py: Python<'py>, s: crate::gen::num::zeta::Complex) -> PyResult<Bound<'py, PyAny>> {
            let s = s.0;
            let out = mrlyrs::num::zeta::mellin(s);
            (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
        }

        /// Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two.
        #[pyfunction]
        #[pyo3(name = "novelty_main", signature = ())]
        pub fn novelty_main<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::novelty_main();
            (out).into_bound_py_any(py)
        }

        /// Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict.
        #[pyfunction]
        #[pyo3(name = "novelty_wave", signature = (gammas, coef, log_y))]
        pub fn novelty_wave<'py>(py: Python<'py>, gammas: Vec<f64>, coef: Vec<crate::gen::num::zeta::Complex>, log_y: f64) -> PyResult<Bound<'py, PyAny>> {
            let coef = coef.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrlyrs::num::zeta::novelty_wave(&gammas, &coef, log_y);
            (out).into_bound_py_any(py)
        }

        /// Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.
        #[pyfunction]
        #[pyo3(name = "psi_formula", signature = (x, gammas))]
        pub fn psi_formula<'py>(py: Python<'py>, x: f64, gammas: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::psi_formula(x, &gammas);
            (out).into_bound_py_any(py)
        }

        /// Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.
        #[pyfunction]
        #[pyo3(name = "psi_stair", signature = (x))]
        pub fn psi_stair<'py>(py: Python<'py>, x: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::psi_stair(x);
            (out).into_bound_py_any(py)
        }

        /// Returns a positive real base raised to a complex exponent.
        #[pyfunction]
        #[pyo3(name = "raise_", signature = (base, exponent))]
        pub fn raise_<'py>(py: Python<'py>, base: f64, exponent: crate::gen::num::zeta::Complex) -> PyResult<Bound<'py, PyAny>> {
            let exponent = exponent.0;
            let out = mrlyrs::num::zeta::raise(base, exponent);
            (crate::gen::num::zeta::Complex(out)).into_bound_py_any(py)
        }

        /// Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y.
        #[pyfunction]
        #[pyo3(name = "sharp_novelty", signature = (prefix, y))]
        pub fn sharp_novelty<'py>(py: Python<'py>, prefix: Vec<u64>, y: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::sharp_novelty(&prefix, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y.
        #[pyfunction]
        #[pyo3(name = "smoothed_novelty", signature = (phi, y, main))]
        pub fn smoothed_novelty<'py>(py: Python<'py>, phi: Vec<u64>, y: f64, main: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrlyrs::num::zeta::smoothed_novelty(&phi, y, main);
            (out).into_bound_py_any(py)
        }

        pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
            let m = PyModule::new(py, "mrlypy.num.zeta")?;
            m.setattr("__doc__", "The critical line: zeta at one half plus i t and off it, its zeros, the prime staircase they rebuild and the novelty meter their waves predict.")?;
            m.add_class::<Complex>()?;
            m.add_class::<Line>()?;
            m.add_function(wrap_pyfunction!(bump, &m)?)?;
            m.add_function(wrap_pyfunction!(corrections, &m)?)?;
            m.add_function(wrap_pyfunction!(kernel, &m)?)?;
            m.add_function(wrap_pyfunction!(mellin, &m)?)?;
            m.add_function(wrap_pyfunction!(novelty_main, &m)?)?;
            m.add_function(wrap_pyfunction!(novelty_wave, &m)?)?;
            m.add_function(wrap_pyfunction!(psi_formula, &m)?)?;
            m.add_function(wrap_pyfunction!(psi_stair, &m)?)?;
            m.add_function(wrap_pyfunction!(raise_, &m)?)?;
            m.add_function(wrap_pyfunction!(sharp_novelty, &m)?)?;
            m.add_function(wrap_pyfunction!(smoothed_novelty, &m)?)?;
            m.add("JOIN", mrlyrs::num::zeta::JOIN)?;
            let names: Vec<&str> = vec!["bump", "corrections", "kernel", "mellin", "novelty_main", "novelty_wave", "psi_formula", "psi_stair", "raise_", "sharp_novelty", "smoothed_novelty", "Complex", "Line", "JOIN"];
            m.add("__all__", names)?;
            parent.add("zeta", &m)?;
            sys.set_item("mrlypy._mrlypy.num.zeta", &m)?;
            Ok(())
        }
    }

    pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {
        let m = PyModule::new(py, "mrlypy.num")?;
        m.setattr("__doc__", "The integers: primes, divisors, series, lattices, spectra and networks.\nThe instruments of number: primes, divisors, series, spectra, lattices and the designs the digits draw.\n\nPlain numbers and byte grids go in; counts, fractions, rates and measurements come out.\nEvery answer is exact where the integers allow and a stated approximation where they do not.\n\n# Files\n\n- `apollonian`: an integral circle packing, the Ford circles on its line, the Farey stack beneath.\n- `automaton`: the Dirichlet series of a memory design, continued through its transfer matrix.\n- `blend`: term ops on sequences, the exact recurrence behind one, its growth rate.\n- `boolean`: a truth table's Walsh spectrum, nonlinearity, balance and avalanche.\n- `design`: the digit designs on the line, their Mobius meter and the ordinates it carries.\n- `factor`: factorizations, divisors, totients, radicals, Mobius values, gcd and lcm.\n- `fft`: the fast Fourier transform in one and two dimensions.\n- `gauss`: the Gaussian and the Eisenstein integers, their classes, windows and shells.\n- `ladder`: the Dirichlet series of a digit design, continued to the plane, each value with its bound.\n- `lattice`: coprime pairs, the Farey nodes of a window, the constant a dimension recovers.\n- `memory`: a rule on consecutive digits, its transfer matrix, its words, its Perron root.\n- `morse`: the Thue-Morse world: the digit rule, the substitution, the lifts and the runs.\n- `prime`: the sieve and its readings, ranks, gaps, counts and the shapes a number makes.\n- `radix`: a digit set inside the residues of a base in a ring, and where its words land.\n- `series`: the classic sequences, zeta and its cousins, the partials walking to pi, e and gamma.\n- `sieve`: the Wallis sieve and its kin as schedule words, with their rasters and limits.\n- `spiral`: the whole numbers wound on the square and the hexagonal lattice, marked and read.\n- `zeta`: zeta on the critical line, its zeros, the prime staircase they rebuild.\n\n# Doors\n\n- The divisor arithmetic: [`gcd`](crate::num::factor::gcd), [`factorial`](crate::num::factor::factorial), [`divisors`](crate::num::factor::divisors), [`mobius`](crate::num::factor::mobius).\n- The primality test: [`is_prime`](crate::num::prime::is_prime).\n- The zeta value above one: [`zeta`](crate::num::series::zeta).")?;
        let names: Vec<&str> = vec![];
        m.add("__all__", names)?;
        apollonian::init(py, &m, sys)?;
        automaton::init(py, &m, sys)?;
        blend::init(py, &m, sys)?;
        boolean::init(py, &m, sys)?;
        design::init(py, &m, sys)?;
        factor::init(py, &m, sys)?;
        fft::init(py, &m, sys)?;
        gauss::init(py, &m, sys)?;
        ladder::init(py, &m, sys)?;
        lattice::init(py, &m, sys)?;
        memory::init(py, &m, sys)?;
        morse::init(py, &m, sys)?;
        prime::init(py, &m, sys)?;
        radix::init(py, &m, sys)?;
        series::init(py, &m, sys)?;
        sieve::init(py, &m, sys)?;
        spiral::init(py, &m, sys)?;
        zeta::init(py, &m, sys)?;
        parent.add("num", &m)?;
        sys.set_item("mrlypy._mrlypy.num", &m)?;
        Ok(())
    }
}

pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = py.import("sys")?.getattr("modules")?.cast_into::<PyDict>()?;
    core::init(py, root, &sys)?;
    font::init(py, root, &sys)?;
    gen_::init(py, root, &sys)?;
    life::init(py, root, &sys)?;
    math::init(py, root, &sys)?;
    num::init(py, root, &sys)?;
    Ok(())
}
