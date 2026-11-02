#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// The substrate: tensors, cells, colors, images, resampling, seeded chance and the canonical names.
pub mod core {
    use crate::hand::{PyColor, PyPixels, PyRng, PySerde, ok};
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// The cell grid: type bytes with optional per-cell colors and tags.
    pub mod cell {
        use crate::hand::{PyCell, PyColor, PyRgba, PyRng, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Flips every type to one minus itself, same as invert.
        #[pyfunction]
        #[pyo3(name = "anti", signature = (cell))]
        pub fn anti<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::anti(cell);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Maps every type to one at or above the threshold and zero below, dropping colors.
        #[pyfunction]
        #[pyo3(name = "binarize", signature = (cell, threshold))]
        pub fn binarize<'py>(
            py: Python<'py>,
            cell: PyCell,
            threshold: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::binarize(cell, threshold);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Binarizes the types at Otsu's threshold, dropping colors.
        #[pyfunction]
        #[pyo3(name = "binarize_otsu", signature = (cell))]
        pub fn binarize_otsu<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::binarize_otsu(cell);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Replaces every type with the rounded mean of its masked neighborhood, dropping colors.
        #[pyfunction]
        #[pyo3(name = "blur", signature = (cell, mask, wrap))]
        pub fn blur<'py>(
            py: Python<'py>,
            cell: PyCell,
            mask: PyTensor,
            wrap: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrly_core::Cell::blur(cell, &mask, wrap);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the painted color at a flat index, or transparent while unpainted or past the end.
        #[pyfunction]
        #[pyo3(name = "color_at", signature = (cell, flat))]
        pub fn color_at<'py>(
            py: Python<'py>,
            cell: PyCell,
            flat: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::color_at(&cell, flat);
            (PyRgba(out)).into_bound_py_any(py)
        }

        /// Builds the Kronecker product of the two cells' types.
        #[pyfunction]
        #[pyo3(name = "combine", signature = (cell, other))]
        pub fn combine<'py>(
            py: Python<'py>,
            cell: PyCell,
            other: PyCell,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let other = other.0;
            let out = mrly_core::Cell::combine(&cell, &other);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags.
        #[pyfunction]
        #[pyo3(name = "fractal", signature = (cell, level))]
        pub fn fractal<'py>(
            py: Python<'py>,
            cell: PyCell,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::fractal(cell, level);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Flips every type to one minus itself.
        #[pyfunction]
        #[pyo3(name = "invert", signature = (cell))]
        pub fn invert<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::invert(cell);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Tags every cell with its concentric shell distance from the center.
        #[pyfunction]
        #[pyo3(name = "layers", signature = (cell, dtype))]
        pub fn layers<'py>(
            py: Python<'py>,
            cell: PyCell,
            dtype: PySerde<mrly_core::Dtype>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let dtype = dtype.0;
            let out = mrly_core::Cell::layers(cell, dtype);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Folds at least two cells into one by chained Kronecker products.
        #[pyfunction]
        #[pyo3(name = "magic", signature = (cells))]
        pub fn magic<'py>(py: Python<'py>, cells: Vec<PyCell>) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_core::cell::magic(&cells);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the default mapping of the first six types to white, black, alpha, red, green, and blue.
        #[pyfunction]
        #[pyo3(name = "mapping", signature = ())]
        pub fn mapping<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::cell::mapping();
            ((out)
                .into_iter()
                .map(|(k, v)| (k, (v).into_iter().map(PyColor).collect::<Vec<_>>()))
                .collect::<std::collections::HashMap<_, _>>())
            .into_bound_py_any(py)
        }

        /// Stitches same-shaped cells into one grid of reps blocks per axis.
        #[pyfunction]
        #[pyo3(name = "merge", signature = (cells, reps))]
        pub fn merge<'py>(
            py: Python<'py>,
            cells: Vec<PyCell>,
            reps: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_core::cell::merge(&cells, &reps);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the 3-wide Moore mask of the dimension, every site on but the center.
        #[pyfunction]
        #[pyo3(name = "moore", signature = (dimension))]
        pub fn moore<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::cell::moore(dimension);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Lays the cell each mask entry indexes into that entry's place and merges the lot.
        #[pyfunction]
        #[pyo3(name = "mosaic", signature = (mask, cells))]
        pub fn mosaic<'py>(
            py: Python<'py>,
            mask: PyTensor,
            cells: Vec<PyCell>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_core::cell::mosaic(&mask, &cells);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Tags every cell with its count of target-valued neighbors under the mask.
        #[pyfunction]
        #[pyo3(name = "neighbors", signature = (cell, mask, target, wrap, dtype))]
        pub fn neighbors<'py>(
            py: Python<'py>,
            cell: PyCell,
            mask: PyTensor,
            target: u8,
            wrap: bool,
            dtype: PySerde<mrly_core::Dtype>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let dtype = dtype.0;
            let out = mrly_core::Cell::neighbors(cell, &mask, target, wrap, dtype);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Wraps a tensor of types in a bare cell, colorless and tagless.
        #[pyfunction]
        #[pyo3(name = "new", signature = (types))]
        pub fn new<'py>(py: Python<'py>, types: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let out = mrly_core::Cell::new(types);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Wraps the cell in count layers of value on every side, dropping colors.
        #[pyfunction]
        #[pyo3(name = "pad", signature = (cell, count, value))]
        pub fn pad<'py>(
            py: Python<'py>,
            cell: PyCell,
            count: usize,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::pad(cell, count, value);
            (PyCell(out)).into_bound_py_any(py)
        }

        /// Colors every mapped cell, picking within each type's palette by the mode.
        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, mapping, mode, rng=None))]
        pub fn paint<'py>(
            py: Python<'py>,
            cell: PyCell,
            mapping: std::collections::HashMap<u8, Vec<PyColor>>,
            mode: PySerde<mrly_core::Mode>,
            rng: Option<&mut PyRng>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mapping = mapping
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>()))
                .collect();
            let mode = mode.0;
            let out = mrly_core::Cell::paint(cell, &mapping, mode, rng.map(|r| &mut r.0));
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Stamps value wherever the tiled mask is on, dropping colors.
        #[pyfunction]
        #[pyo3(name = "perforate", signature = (cell, mask, value))]
        pub fn perforate<'py>(
            py: Python<'py>,
            cell: PyCell,
            mask: PyTensor,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrly_core::Cell::perforate(cell, &mask, value);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map.
        #[pyfunction]
        #[pyo3(name = "remap", signature = (cell, map, shape))]
        pub fn remap<'py>(
            py: Python<'py>,
            cell: PyCell,
            map: Vec<usize>,
            shape: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::cell::remap(&cell, &map, &shape);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the flat rgba bytes of the cells, four to a cell, the stored color where there is one and opaque black everywhere else.
        #[pyfunction]
        #[pyo3(name = "rgba", signature = (cell))]
        pub fn rgba<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::rgba(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the flat source index of every destination cell after k quarter turns in the plane of the axes.
        #[pyfunction]
        #[pyo3(name = "rot90_map", signature = (shape, k, axes))]
        pub fn rot90_map<'py>(
            py: Python<'py>,
            shape: Vec<usize>,
            k: usize,
            axes: (usize, usize),
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::cell::rot90_map(&shape, k, axes);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along.
        #[pyfunction]
        #[pyo3(name = "rotate", signature = (cell, k, axes))]
        pub fn rotate<'py>(
            py: Python<'py>,
            cell: PyCell,
            k: usize,
            axes: (usize, usize),
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::rotate(cell, k, axes);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the shape of the type tensor.
        #[pyfunction]
        #[pyo3(name = "shape", signature = (cell))]
        pub fn shape<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::shape(&cell);
            ((out).to_vec()).into_bound_py_any(py)
        }

        /// Returns the number of cells.
        #[pyfunction]
        #[pyo3(name = "size", signature = (cell))]
        pub fn size<'py>(py: Python<'py>, cell: PyCell) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::size(&cell);
            (out).into_bound_py_any(py)
        }

        /// Repeats the cell reps times along each axis, carrying colors and tags along.
        #[pyfunction]
        #[pyo3(name = "tile", signature = (cell, reps))]
        pub fn tile<'py>(
            py: Python<'py>,
            cell: PyCell,
            reps: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_core::Cell::tile(cell, &reps);
            (PyCell(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the flat source index of every destination cell after tiling reps copies per axis.
        #[pyfunction]
        #[pyo3(name = "tile_map", signature = (shape, reps))]
        pub fn tile_map<'py>(
            py: Python<'py>,
            shape: Vec<usize>,
            reps: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::cell::tile_map(&shape, &reps);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.cell")?;
            m.setattr(
                "__doc__",
                "The cell grid: type bytes with optional per-cell colors and tags.",
            )?;
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
            let names: Vec<&str> = vec![
                "anti",
                "binarize",
                "binarize_otsu",
                "blur",
                "color_at",
                "combine",
                "fractal",
                "invert",
                "layers",
                "magic",
                "mapping",
                "merge",
                "moore",
                "mosaic",
                "neighbors",
                "new",
                "pad",
                "paint",
                "perforate",
                "remap",
                "rgba",
                "rot90_map",
                "rotate",
                "shape",
                "size",
                "tile",
                "tile_map",
            ];
            m.add("__all__", names)?;
            parent.add("cell", &m)?;
            sys.set_item("mrly.core._core.core.cell", &m)?;
            Ok(())
        }
    }

    /// The rgba color, its themes and the fifteen-color palette.
    pub mod colors {
        use crate::hand::{PyColor, PyPixels, PyRgba, PyRng, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// One theme: the surfaces of a dark or a light ground and the thirteen inks, the same on both.
        #[pyclass(name = "Theme", module = "mrly.core.colors", from_py_object)]
        #[derive(Clone)]
        pub struct Theme(pub mrly_core::colors::Theme);

        #[pymethods]
        impl Theme {
            /// The ground every figure is painted on.
            #[getter]
            #[pyo3(name = "ground")]
            pub fn ground<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.ground;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "ground")]
            pub fn set_ground(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.ground = value;
                Ok(())
            }
            /// The page background, one step off the ground.
            #[getter]
            #[pyo3(name = "bg")]
            pub fn bg<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.bg;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "bg")]
            pub fn set_bg(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.bg = value;
                Ok(())
            }
            /// The raised panel.
            #[getter]
            #[pyo3(name = "panel")]
            pub fn panel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.panel;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "panel")]
            pub fn set_panel(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.panel = value;
                Ok(())
            }
            /// The sunken well.
            #[getter]
            #[pyo3(name = "deep")]
            pub fn deep<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.deep;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "deep")]
            pub fn set_deep(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.deep = value;
                Ok(())
            }
            /// The hairline between things.
            #[getter]
            #[pyo3(name = "line")]
            pub fn line<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.line;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "line")]
            pub fn set_line(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.line = value;
                Ok(())
            }
            /// The foreground, the strongest tone.
            #[getter]
            #[pyo3(name = "fg")]
            pub fn fg<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.fg;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "fg")]
            pub fn set_fg(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.fg = value;
                Ok(())
            }
            /// The dimmed foreground, for anything secondary.
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dim")]
            pub fn set_dim(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.dim = value;
                Ok(())
            }
            /// The interactive accent.
            #[getter]
            #[pyo3(name = "accent")]
            pub fn accent<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.accent;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "accent")]
            pub fn set_accent(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.accent = value;
                Ok(())
            }
            /// The tone written on the accent.
            #[getter]
            #[pyo3(name = "on_accent")]
            pub fn on_accent<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.on_accent;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "on_accent")]
            pub fn set_on_accent(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.on_accent = value;
                Ok(())
            }
            /// The red ink.
            #[getter]
            #[pyo3(name = "red")]
            pub fn red<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.red;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "red")]
            pub fn set_red(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.red = value;
                Ok(())
            }
            /// The orange ink.
            #[getter]
            #[pyo3(name = "orange")]
            pub fn orange<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.orange;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "orange")]
            pub fn set_orange(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.orange = value;
                Ok(())
            }
            /// The yellow ink.
            #[getter]
            #[pyo3(name = "yellow")]
            pub fn yellow<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.yellow;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "yellow")]
            pub fn set_yellow(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.yellow = value;
                Ok(())
            }
            /// The green ink.
            #[getter]
            #[pyo3(name = "green")]
            pub fn green<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.green;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "green")]
            pub fn set_green(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.green = value;
                Ok(())
            }
            /// The mint ink.
            #[getter]
            #[pyo3(name = "mint")]
            pub fn mint<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.mint;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "mint")]
            pub fn set_mint(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.mint = value;
                Ok(())
            }
            /// The teal ink.
            #[getter]
            #[pyo3(name = "teal")]
            pub fn teal<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.teal;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "teal")]
            pub fn set_teal(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.teal = value;
                Ok(())
            }
            /// The cyan ink.
            #[getter]
            #[pyo3(name = "cyan")]
            pub fn cyan<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.cyan;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "cyan")]
            pub fn set_cyan(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.cyan = value;
                Ok(())
            }
            /// The blue ink.
            #[getter]
            #[pyo3(name = "blue")]
            pub fn blue<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.blue;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "blue")]
            pub fn set_blue(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.blue = value;
                Ok(())
            }
            /// The indigo ink.
            #[getter]
            #[pyo3(name = "indigo")]
            pub fn indigo<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.indigo;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "indigo")]
            pub fn set_indigo(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.indigo = value;
                Ok(())
            }
            /// The purple ink.
            #[getter]
            #[pyo3(name = "purple")]
            pub fn purple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.purple;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "purple")]
            pub fn set_purple(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.purple = value;
                Ok(())
            }
            /// The pink ink.
            #[getter]
            #[pyo3(name = "pink")]
            pub fn pink<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.pink;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "pink")]
            pub fn set_pink(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.pink = value;
                Ok(())
            }
            /// The brown ink.
            #[getter]
            #[pyo3(name = "brown")]
            pub fn brown<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.brown;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "brown")]
            pub fn set_brown(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.brown = value;
                Ok(())
            }
            /// The gray ink.
            #[getter]
            #[pyo3(name = "gray")]
            pub fn gray<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.gray;
                (PyColor(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "gray")]
            pub fn set_gray(&mut self, value: PyColor) -> PyResult<()> {
                let value = value.0;
                self.0.gray = value;
                Ok(())
            }
            /// The thirteen inks in name order.
            #[pyo3(name = "hues", signature = ())]
            pub fn hues<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::colors::Theme::hues(&self.0);
                ((out).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo.
            #[pyo3(name = "inks", signature = ())]
            pub fn inks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::colors::Theme::inks(&self.0);
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
        pub fn alpha<'py>(
            py: Python<'py>,
            color: PyColor,
            level: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrly_core::Color::alpha(&color, level);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Returns the ground rgba of the dark or the light theme.
        #[pyfunction]
        #[pyo3(name = "board", signature = (dark))]
        pub fn board<'py>(py: Python<'py>, dark: bool) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::colors::board(dark);
            (PyRgba(out)).into_bound_py_any(py)
        }

        /// Formats the color as a css rgb or rgba call.
        #[pyfunction]
        #[pyo3(name = "css", signature = (color))]
        pub fn css<'py>(py: Python<'py>, color: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrly_core::Color::css(&color);
            (out).into_bound_py_any(py)
        }

        /// Parses a #RRGGBB or #RRGGBBAA code, hash optional.
        #[pyfunction]
        #[pyo3(name = "from_hex", signature = (hex))]
        pub fn from_hex<'py>(py: Python<'py>, hex: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Color::from_hex(hex);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a gradient of steps colors sweeping evenly through the given stops.
        #[pyfunction]
        #[pyo3(name = "gradient", signature = (colors, steps))]
        pub fn gradient<'py>(
            py: Python<'py>,
            colors: Vec<PyColor>,
            steps: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let colors = colors.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_core::colors::gradient(&colors, steps);
            ((ok(out)?).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the foreground rgba of the dark or the light theme.
        #[pyfunction]
        #[pyo3(name = "ink", signature = (dark))]
        pub fn ink<'py>(py: Python<'py>, dark: bool) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::colors::ink(dark);
            (PyRgba(out)).into_bound_py_any(py)
        }

        /// Returns the color with every channel flipped and the alpha kept.
        #[pyfunction]
        #[pyo3(name = "invert", signature = (color))]
        pub fn invert<'py>(py: Python<'py>, color: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrly_core::Color::invert(&color);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Returns the color scaled toward black below level 50 and toward white above.
        #[pyfunction]
        #[pyo3(name = "lightness", signature = (color, level))]
        pub fn lightness<'py>(
            py: Python<'py>,
            color: PyColor,
            level: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrly_core::Color::lightness(&color, level);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Reads rgba pixels as a type grid, one wherever the rgb mean falls below the level.
        #[pyfunction]
        #[pyo3(name = "luma_types", signature = (pixels, width, height, level))]
        pub fn luma_types<'py>(
            py: Python<'py>,
            pixels: PyPixels,
            width: usize,
            height: usize,
            level: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let pixels = pixels.0;
            let out = mrly_core::colors::luma_types(&pixels, width, height, level);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Blends two colors linearly by ratio.
        #[pyfunction]
        #[pyo3(name = "mix", signature = (color_1, color_2, ratio))]
        pub fn mix<'py>(
            py: Python<'py>,
            color_1: PyColor,
            color_2: PyColor,
            ratio: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let color_1 = color_1.0;
            let color_2 = color_2.0;
            let out = mrly_core::colors::mix(color_1, color_2, ratio);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the palette color a name spells.
        #[pyfunction]
        #[pyo3(name = "named", signature = (name))]
        pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::colors::named(name);
            (PyColor(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a color from the stream, opaque unless alpha is asked for.
        #[pyfunction]
        #[pyo3(name = "random", signature = (alpha, rng))]
        pub fn random<'py>(
            py: Python<'py>,
            alpha: bool,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Color::random(alpha, &mut rng.0);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Builds an opaque color.
        #[pyfunction]
        #[pyo3(name = "rgb", signature = (r, g, b))]
        pub fn rgb<'py>(py: Python<'py>, r: u8, g: u8, b: u8) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Color::rgb(r, g, b);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Builds a color with an explicit alpha.
        #[pyfunction]
        #[pyo3(name = "rgba", signature = (r, g, b, a))]
        pub fn rgba<'py>(
            py: Python<'py>,
            r: u8,
            g: u8,
            b: u8,
            a: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Color::rgba(r, g, b, a);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Returns a hue one shade lighter, itself, and one shade darker.
        #[pyfunction]
        #[pyo3(name = "shades", signature = (hue))]
        pub fn shades<'py>(py: Python<'py>, hue: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let hue = hue.0;
            let out = mrly_core::colors::shades(hue);
            ((out).into_iter().map(PyColor).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Snaps every pixel to the palette color nearest it in squared rgba distance, first on a tie.
        #[pyfunction]
        #[pyo3(name = "snap", signature = (pixels, palette))]
        pub fn snap<'py>(
            py: Python<'py>,
            pixels: PyPixels,
            palette: Vec<PyColor>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let pixels = pixels.0;
            let palette = palette.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_core::colors::snap(&pixels, &palette);
            (PyPixels(out)).into_bound_py_any(py)
        }

        /// Formats the color as lowercase hex, appending the alpha pair only when not opaque.
        #[pyfunction]
        #[pyo3(name = "to_hex", signature = (color))]
        pub fn to_hex<'py>(py: Python<'py>, color: PyColor) -> PyResult<Bound<'py, PyAny>> {
            let color = color.0;
            let out = mrly_core::Color::to_hex(&color);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.colors")?;
            m.setattr(
                "__doc__",
                "The rgba color, its themes and the fifteen-color palette.",
            )?;
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
            m.add("ALPHA", PyColor(mrly_core::colors::ALPHA))?;
            m.add("BLACK", PyColor(mrly_core::colors::BLACK))?;
            m.add("BLUE", PyColor(mrly_core::colors::BLUE))?;
            m.add("BROWN", PyColor(mrly_core::colors::BROWN))?;
            m.add("CYAN", PyColor(mrly_core::colors::CYAN))?;
            m.add(
                "DARK",
                crate::doors::core::colors::Theme(mrly_core::colors::DARK),
            )?;
            m.add("GRAY", PyColor(mrly_core::colors::GRAY))?;
            m.add("GREEN", PyColor(mrly_core::colors::GREEN))?;
            m.add("INDIGO", PyColor(mrly_core::colors::INDIGO))?;
            m.add(
                "LIGHT",
                crate::doors::core::colors::Theme(mrly_core::colors::LIGHT),
            )?;
            m.add("MINT", PyColor(mrly_core::colors::MINT))?;
            m.add(
                "NAMES",
                (mrly_core::colors::NAMES)
                    .into_iter()
                    .map(|x| (x).to_string())
                    .collect::<Vec<_>>(),
            )?;
            m.add("ORANGE", PyColor(mrly_core::colors::ORANGE))?;
            m.add(
                "PALETTE",
                (mrly_core::colors::PALETTE)
                    .into_iter()
                    .map(PyColor)
                    .collect::<Vec<_>>(),
            )?;
            m.add("PINK", PyColor(mrly_core::colors::PINK))?;
            m.add("PURPLE", PyColor(mrly_core::colors::PURPLE))?;
            m.add("RED", PyColor(mrly_core::colors::RED))?;
            m.add("TEAL", PyColor(mrly_core::colors::TEAL))?;
            m.add("WHITE", PyColor(mrly_core::colors::WHITE))?;
            m.add("YELLOW", PyColor(mrly_core::colors::YELLOW))?;
            let names: Vec<&str> = vec![
                "alpha",
                "board",
                "css",
                "from_hex",
                "gradient",
                "ink",
                "invert",
                "lightness",
                "luma_types",
                "mix",
                "named",
                "random",
                "rgb",
                "rgba",
                "shades",
                "snap",
                "to_hex",
                "Theme",
                "ALPHA",
                "BLACK",
                "BLUE",
                "BROWN",
                "CYAN",
                "DARK",
                "GRAY",
                "GREEN",
                "INDIGO",
                "LIGHT",
                "MINT",
                "NAMES",
                "ORANGE",
                "PALETTE",
                "PINK",
                "PURPLE",
                "RED",
                "TEAL",
                "WHITE",
                "YELLOW",
            ];
            m.add("__all__", names)?;
            parent.add("colors", &m)?;
            sys.set_item("mrly.core._core.core.colors", &m)?;
            Ok(())
        }
    }

    /// The one error type, its Result and the json parser over the rented value.
    pub mod error {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Parses JSON text into a value.
        #[pyfunction]
        #[pyo3(name = "parse", signature = (text))]
        pub fn parse<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::error::parse(text);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.error")?;
            m.setattr(
                "__doc__",
                "The one error type, its Result and the json parser over the rented value.",
            )?;
            m.add_function(wrap_pyfunction!(parse, &m)?)?;
            let names: Vec<&str> = vec!["parse"];
            m.add("__all__", names)?;
            parent.add("error", &m)?;
            sys.set_item("mrly.core._core.core.error", &m)?;
            Ok(())
        }
    }

    /// The image a renderer returns: its size and its rgba pixels.
    pub mod image {
        use crate::hand::{PyImage, PyPixels, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window.
        #[pyfunction]
        #[pyo3(name = "blur", signature = (pixels, width, height, radius))]
        pub fn blur<'py>(
            py: Python<'py>,
            pixels: PyPixels,
            width: usize,
            height: usize,
            radius: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let pixels = pixels.0;
            let out = mrly_core::image::blur(&pixels, width, height, radius);
            (PyPixels(out)).into_bound_py_any(py)
        }

        /// Builds an image from its width, its height and its colors.
        #[pyfunction]
        #[pyo3(name = "new", signature = (width, height, colors))]
        pub fn new<'py>(
            py: Python<'py>,
            width: usize,
            height: usize,
            colors: PyPixels,
        ) -> PyResult<Bound<'py, PyAny>> {
            let colors = colors.0;
            let out = mrly_core::Image::new(width, height, colors);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        /// Resamples the image to a new size.
        #[pyfunction]
        #[pyo3(name = "resample", signature = (image, width, height, filter))]
        pub fn resample<'py>(
            py: Python<'py>,
            image: PyImage,
            width: usize,
            height: usize,
            filter: PySerde<mrly_core::Filter>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let image = image.0;
            let filter = filter.0;
            let out = mrly_core::Image::resample(&image, width, height, filter);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws every pixel as a scale by scale block, growing both sides by scale.
        #[pyfunction]
        #[pyo3(name = "scale", signature = (image, scale))]
        pub fn scale<'py>(
            py: Python<'py>,
            image: PyImage,
            scale: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let image = image.0;
            let out = mrly_core::Image::scale(&image, scale);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.image")?;
            m.setattr(
                "__doc__",
                "The image a renderer returns: its size and its rgba pixels.",
            )?;
            m.add_function(wrap_pyfunction!(blur, &m)?)?;
            m.add_function(wrap_pyfunction!(new, &m)?)?;
            m.add_function(wrap_pyfunction!(resample, &m)?)?;
            m.add_function(wrap_pyfunction!(scale, &m)?)?;
            let names: Vec<&str> = vec!["blur", "new", "resample", "scale"];
            m.add("__all__", names)?;
            parent.add("image", &m)?;
            sys.set_item("mrly.core._core.core.image", &m)?;
            Ok(())
        }
    }

    /// The canonical names: one JSON object per thing, its views and its id.
    pub mod name {
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The sha256 digest the ids are cut from.
        pub mod sha {
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Returns the sha256 digest of `bytes` as 64 lowercase hex digits.
            #[pyfunction]
            #[pyo3(name = "hex", signature = (bytes))]
            pub fn hex<'py>(py: Python<'py>, bytes: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::name::sha::hex(&bytes);
                (out).into_bound_py_any(py)
            }

            /// Returns the 32-byte sha256 digest of `bytes`.
            #[pyfunction]
            #[pyo3(name = "sha256", signature = (bytes))]
            pub fn sha256<'py>(py: Python<'py>, bytes: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::name::sha::sha256(&bytes);
                (out).into_bound_py_any(py)
            }

            /// Returns the first eight hex digits of the sha256 digest of `bytes`, the id a name prints.
            #[pyfunction]
            #[pyo3(name = "short", signature = (bytes))]
            pub fn short<'py>(py: Python<'py>, bytes: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::name::sha::short(&bytes);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.core.name.sha")?;
                m.setattr("__doc__", "The sha256 digest the ids are cut from.")?;
                m.add_function(wrap_pyfunction!(hex, &m)?)?;
                m.add_function(wrap_pyfunction!(sha256, &m)?)?;
                m.add_function(wrap_pyfunction!(short, &m)?)?;
                let names: Vec<&str> = vec!["hex", "sha256", "short"];
                m.add("__all__", names)?;
                parent.add("sha", &m)?;
                sys.set_item("mrly.core._core.core.name.sha", &m)?;
                Ok(())
            }
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.name")?;
            m.setattr(
                "__doc__",
                "The canonical names: one JSON object per thing, its views and its id.",
            )?;
            let names: Vec<&str> = vec![];
            m.add("__all__", names)?;
            sha::init(py, &m, sys)?;
            parent.add("name", &m)?;
            sys.set_item("mrly.core._core.core.name", &m)?;
            Ok(())
        }
    }

    /// The editions that distribute a palette over a cell.
    pub mod paint {
        use crate::hand::{PyColor, PyRng, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A complete coloring recipe for one cell.
        #[pyclass(name = "Paint", module = "mrly.core.paint", from_py_object)]
        #[derive(Clone)]
        pub struct Paint(pub mrly_core::paint::Paint);

        #[pymethods]
        impl Paint {
            /// Builds a black-primary, fill-target, multicolor paint for an edition.
            #[new]
            #[pyo3(signature = (edition))]
            pub fn __new__(edition: PySerde<mrly_core::paint::Edition>) -> PyResult<Self> {
                let edition = edition.0;
                let out = mrly_core::paint::Paint::new(edition);
                Ok(Self(out))
            }
            /// The coloring edition.
            #[getter]
            #[pyo3(name = "edition")]
            pub fn edition<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.edition;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "edition")]
            pub fn set_edition(
                &mut self,
                value: PySerde<mrly_core::paint::Edition>,
            ) -> PyResult<()> {
                let value = value.0;
                self.0.edition = value;
                Ok(())
            }
            /// The secondary color scheme.
            #[getter]
            #[pyo3(name = "scheme")]
            pub fn scheme<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.scheme;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "scheme")]
            pub fn set_scheme(&mut self, value: PySerde<mrly_core::paint::Scheme>) -> PyResult<()> {
                let value = value.0;
                self.0.scheme = value;
                Ok(())
            }
            /// The side the primary ink lands on.
            #[getter]
            #[pyo3(name = "target")]
            pub fn target<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.target;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "target")]
            pub fn set_target(&mut self, value: PySerde<mrly_core::paint::Target>) -> PyResult<()> {
                let value = value.0;
                self.0.target = value;
                Ok(())
            }
            /// The primary ink.
            #[getter]
            #[pyo3(name = "primary")]
            pub fn primary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.primary;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "primary")]
            pub fn set_primary(&mut self, value: PySerde<mrly_core::paint::Ink>) -> PyResult<()> {
                let value = value.0;
                self.0.primary = value;
                Ok(())
            }
            /// The secondary inks.
            #[getter]
            #[pyo3(name = "secondary")]
            pub fn secondary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.secondary.clone();
                ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "secondary")]
            pub fn set_secondary(
                &mut self,
                value: Vec<PySerde<mrly_core::paint::Ink>>,
            ) -> PyResult<()> {
                let value = value.into_iter().map(|x| x.0).collect::<Vec<_>>();
                self.0.secondary = value;
                Ok(())
            }
            /// The shade indices of a multitone ramp.
            #[getter]
            #[pyo3(name = "shades")]
            pub fn shades<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.shades.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "shades")]
            pub fn set_shades(&mut self, value: Vec<usize>) -> PyResult<()> {
                self.0.shades = value;
                Ok(())
            }
            /// Returns true for the Simple edition.
            #[pyo3(name = "is_simple", signature = ())]
            pub fn is_simple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::paint::Paint::is_simple(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a black-primary, fill-target, multicolor paint for an edition.
            #[staticmethod]
            #[pyo3(name = "new", signature = (edition))]
            pub fn new_<'py>(
                py: Python<'py>,
                edition: PySerde<mrly_core::paint::Edition>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let edition = edition.0;
                let out = mrly_core::paint::Paint::new(edition);
                (crate::doors::core::paint::Paint(out)).into_bound_py_any(py)
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

        /// The constraints a caller may put on a random paint.
        #[pyclass(name = "Config", module = "mrly.core.paint", skip_from_py_object)]
        pub struct Config;

        #[pymethods]
        impl Config {
            /// Returns the default Config.
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::paint::Config::default();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// The seven ways a paint distributes its colors over a cell.
        #[pyclass(name = "Edition", module = "mrly.core.paint", skip_from_py_object)]
        pub struct Edition;

        #[pymethods]
        impl Edition {
            /// Returns every Edition in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::paint::Edition::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the cell-painting mode this edition renders with, or None for Random, which scatters.
            #[staticmethod]
            #[pyo3(name = "mode", signature = (edition))]
            pub fn mode<'py>(
                py: Python<'py>,
                edition: PySerde<mrly_core::paint::Edition>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let edition = edition.0;
                let out = mrly_core::paint::Edition::mode(edition);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
        }

        /// The fifteen named inks a paint draws from.
        #[pyclass(name = "Ink", module = "mrly.core.paint", skip_from_py_object)]
        pub struct Ink;

        #[pymethods]
        impl Ink {
            /// Returns every Ink in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::paint::Ink::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the ink's color.
            #[staticmethod]
            #[pyo3(name = "color", signature = (ink))]
            pub fn color<'py>(
                py: Python<'py>,
                ink: PySerde<mrly_core::paint::Ink>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ink = ink.0;
                let out = mrly_core::paint::Ink::color(ink);
                (PyColor(out)).into_bound_py_any(py)
            }
        }

        /// The two ways secondary colors are drawn.
        #[pyclass(name = "Scheme", module = "mrly.core.paint", skip_from_py_object)]
        pub struct Scheme;

        #[pymethods]
        impl Scheme {
            /// Returns every Scheme in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::paint::Scheme::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// The side of the figure the primary ink lands on.
        #[pyclass(name = "Target", module = "mrly.core.paint", skip_from_py_object)]
        pub struct Target;

        #[pymethods]
        impl Target {
            /// Returns every Target in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_core::paint::Target::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream.
        #[pyfunction]
        #[pyo3(name = "apply", signature = (paint, cell, rng))]
        pub fn apply<'py>(
            py: Python<'py>,
            paint: PyRef<'_, crate::doors::core::paint::Paint>,
            cell: &Bound<'_, PyAny>,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let out = mrly_core::paint::apply(&paint.0, &mut cell_owned, &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        /// Replays a stored paint onto a cell, tagging first and applying it from the stream.
        #[pyfunction]
        #[pyo3(name = "coat", signature = (cell, paint, mask, rng))]
        pub fn coat<'py>(
            py: Python<'py>,
            cell: &Bound<'_, PyAny>,
            paint: PyRef<'_, crate::doors::core::paint::Paint>,
            mask: Option<PyTensor>,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let mask = mask.map(|x| x.0);
            let out = mrly_core::paint::coat(&mut cell_owned, &paint.0, mask.as_ref(), &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        /// Draws a random paint under the config, applies it to the cell, and returns the recipe.
        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, config, mask, rng))]
        pub fn paint<'py>(
            py: Python<'py>,
            cell: &Bound<'_, PyAny>,
            config: PySerde<mrly_core::paint::Config>,
            mask: Option<PyTensor>,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let config = config.0;
            let mask = mask.map(|x| x.0);
            let out = mrly_core::paint::paint(&mut cell_owned, &config, mask.as_ref(), &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (crate::doors::core::paint::Paint(ok(out)?)).into_bound_py_any(py)
        }

        /// Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count.
        #[pyfunction]
        #[pyo3(name = "prime", signature = (paint, cell, mask, rng))]
        pub fn prime<'py>(
            py: Python<'py>,
            paint: crate::doors::core::paint::Paint,
            cell: &Bound<'_, PyAny>,
            mask: Option<PyTensor>,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let paint = paint.0;
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let mask = mask.map(|x| x.0);
            let out = mrly_core::paint::prime(paint, &mut cell_owned, mask.as_ref(), &mut rng.0);
            crate::hand::cell_write_back(cell, cell_owned)?;
            (crate::doors::core::paint::Paint(ok(out)?)).into_bound_py_any(py)
        }

        /// Draws a random edition from the allowed list, or from all seven.
        #[pyfunction]
        #[pyo3(name = "random_edition", signature = (editions, rng))]
        pub fn random_edition<'py>(
            py: Python<'py>,
            editions: Option<Vec<PySerde<mrly_core::paint::Edition>>>,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let editions = editions.map(|x| x.into_iter().map(|x| x.0).collect::<Vec<_>>());
            let out = mrly_core::paint::random_edition(editions.as_deref(), &mut rng.0);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Redraws the paint's secondary inks and shades under its scheme.
        #[pyfunction]
        #[pyo3(name = "reroll", signature = (paint, rng))]
        pub fn reroll<'py>(
            py: Python<'py>,
            paint: crate::doors::core::paint::Paint,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let paint = paint.0;
            let out = mrly_core::paint::reroll(paint, &mut rng.0);
            (crate::doors::core::paint::Paint(out)).into_bound_py_any(py)
        }

        /// Draws the paint's scheme, target and primary under the config, then rerolls the rest.
        #[pyfunction]
        #[pyo3(name = "setup", signature = (paint, config, rng))]
        pub fn setup<'py>(
            py: Python<'py>,
            paint: crate::doors::core::paint::Paint,
            config: PySerde<mrly_core::paint::Config>,
            rng: &mut PyRng,
        ) -> PyResult<Bound<'py, PyAny>> {
            let paint = paint.0;
            let config = config.0;
            let out = mrly_core::paint::setup(paint, &config, &mut rng.0);
            (crate::doors::core::paint::Paint(out)).into_bound_py_any(py)
        }

        /// Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side.
        #[pyfunction]
        #[pyo3(name = "tag", signature = (cell, edition, target, mask=None))]
        pub fn tag<'py>(
            py: Python<'py>,
            cell: &Bound<'_, PyAny>,
            edition: PySerde<mrly_core::paint::Edition>,
            target: PySerde<mrly_core::paint::Target>,
            mask: Option<PyTensor>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut cell_owned = crate::hand::cell_from_py(cell)?;
            let edition = edition.0;
            let target = target.0;
            let mask = mask.map(|x| x.0);
            let out = mrly_core::paint::tag(&mut cell_owned, edition, target, mask.as_ref());
            crate::hand::cell_write_back(cell, cell_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.paint")?;
            m.setattr(
                "__doc__",
                "The editions that distribute a palette over a cell.",
            )?;
            m.add_class::<Paint>()?;
            m.add_class::<Config>()?;
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
            let names: Vec<&str> = vec![
                "apply",
                "coat",
                "paint",
                "prime",
                "random_edition",
                "reroll",
                "setup",
                "tag",
                "Paint",
                "Config",
                "Edition",
                "Ink",
                "Scheme",
                "Target",
            ];
            m.add("__all__", names)?;
            parent.add("paint", &m)?;
            sys.set_item("mrly.core._core.core.paint", &m)?;
            Ok(())
        }
    }

    /// The colorizers that turn counter values into colors.
    pub mod ramp {
        use crate::hand::{PyColor, PyPixels, PySerde};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up.
        #[pyfunction]
        #[pyo3(name = "color", signature = (colorizer, value, max))]
        pub fn color<'py>(
            py: Python<'py>,
            colorizer: PySerde<mrly_core::Colorizer>,
            value: usize,
            max: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let colorizer = colorizer.0;
            let out = mrly_core::ramp::color(&colorizer, value, max);
            (PyColor(out)).into_bound_py_any(py)
        }

        /// Maps a slice of values to rgba pixels against the range maximum.
        #[pyfunction]
        #[pyo3(name = "colors", signature = (colorizer, values, max))]
        pub fn colors<'py>(
            py: Python<'py>,
            colorizer: PySerde<mrly_core::Colorizer>,
            values: Vec<usize>,
            max: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let colorizer = colorizer.0;
            let out = mrly_core::ramp::colors(&colorizer, &values, max);
            (PyPixels(out)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.ramp")?;
            m.setattr(
                "__doc__",
                "The colorizers that turn counter values into colors.",
            )?;
            m.add_function(wrap_pyfunction!(color, &m)?)?;
            m.add_function(wrap_pyfunction!(colors, &m)?)?;
            let names: Vec<&str> = vec!["color", "colors"];
            m.add("__all__", names)?;
            parent.add("ramp", &m)?;
            sys.set_item("mrly.core._core.core.ramp", &m)?;
            Ok(())
        }
    }

    /// The seeded random stream.
    pub mod rng {
        use crate::hand::PyRng;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Draws an integer below n, or zero when n is zero.
        #[pyfunction]
        #[pyo3(name = "below", signature = (rng, n))]
        pub fn below<'py>(
            py: Python<'py>,
            rng: &mut PyRng,
            n: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Rng::below(&mut rng.0, n);
            (out).into_bound_py_any(py)
        }

        /// Draws a fair coin flip.
        #[pyfunction]
        #[pyo3(name = "boolean", signature = (rng))]
        pub fn boolean<'py>(py: Python<'py>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Rng::boolean(&mut rng.0);
            (out).into_bound_py_any(py)
        }

        /// Returns true with probability p.
        #[pyfunction]
        #[pyo3(name = "chance", signature = (rng, p))]
        pub fn chance<'py>(
            py: Python<'py>,
            rng: &mut PyRng,
            p: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Rng::chance(&mut rng.0, p);
            (out).into_bound_py_any(py)
        }

        /// Builds the stream from a seed.
        #[pyfunction]
        #[pyo3(name = "new", signature = (seed))]
        pub fn new<'py>(py: Python<'py>, seed: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Rng::new(seed);
            (PyRng(out)).into_bound_py_any(py)
        }

        /// Draws an integer between lo and hi inclusive, or lo when hi is not above lo.
        #[pyfunction]
        #[pyo3(name = "range", signature = (rng, lo, hi))]
        pub fn range<'py>(
            py: Python<'py>,
            rng: &mut PyRng,
            lo: i64,
            hi: i64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Rng::range(&mut rng.0, lo, hi);
            (out).into_bound_py_any(py)
        }

        /// Draws amount distinct indices below length, or every index when amount is larger.
        #[pyfunction]
        #[pyo3(name = "sample_indices", signature = (rng, length, amount))]
        pub fn sample_indices<'py>(
            py: Python<'py>,
            rng: &mut PyRng,
            length: usize,
            amount: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Rng::sample_indices(&mut rng.0, length, amount);
            (out).into_bound_py_any(py)
        }

        /// Draws a float at or above zero and below one.
        #[pyfunction]
        #[pyo3(name = "unit", signature = (rng))]
        pub fn unit<'py>(py: Python<'py>, rng: &mut PyRng) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Rng::unit(&mut rng.0);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.rng")?;
            m.setattr("__doc__", "The seeded random stream.")?;
            m.add_function(wrap_pyfunction!(below, &m)?)?;
            m.add_function(wrap_pyfunction!(boolean, &m)?)?;
            m.add_function(wrap_pyfunction!(chance, &m)?)?;
            m.add_function(wrap_pyfunction!(new, &m)?)?;
            m.add_function(wrap_pyfunction!(range, &m)?)?;
            m.add_function(wrap_pyfunction!(sample_indices, &m)?)?;
            m.add_function(wrap_pyfunction!(unit, &m)?)?;
            let names: Vec<&str> = vec![
                "below",
                "boolean",
                "chance",
                "new",
                "range",
                "sample_indices",
                "unit",
            ];
            m.add("__all__", names)?;
            parent.add("rng", &m)?;
            sys.set_item("mrly.core._core.core.rng", &m)?;
            Ok(())
        }
    }

    /// The tensor and its dtypes.
    pub mod tensor {
        use crate::hand::{PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the element at a flat index, which must be below the size like a slice index.
        #[pyfunction]
        #[pyo3(name = "at", signature = (tensor, flat))]
        pub fn at<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            flat: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::at(&tensor, flat);
            (out).into_bound_py_any(py)
        }

        /// Maps every element to one at or above the threshold, zero below.
        #[pyfunction]
        #[pyo3(name = "binarize", signature = (tensor, threshold))]
        pub fn binarize<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            threshold: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::binarize(&tensor, threshold);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Binarizes at one above the Otsu threshold.
        #[pyfunction]
        #[pyo3(name = "binarize_otsu", signature = (tensor))]
        pub fn binarize_otsu<'py>(
            py: Python<'py>,
            tensor: PyTensor,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::binarize_otsu(&tensor);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Averages every position over its masked neighborhood, rounded.
        #[pyfunction]
        #[pyo3(name = "blur", signature = (tensor, mask, wrap))]
        pub fn blur<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            mask: PyTensor,
            wrap: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let mask = mask.0;
            let out = mrly_core::Tensor::blur(&tensor, &mask, wrap);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as bytes.
        #[pyfunction]
        #[pyo3(name = "bytes", signature = (tensor))]
        pub fn bytes<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::bytes(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Counts the cells holding one value.
        #[pyfunction]
        #[pyo3(name = "count", signature = (tensor, value))]
        pub fn count<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::count(&tensor, value);
            (out).into_bound_py_any(py)
        }

        /// Returns the element width.
        #[pyfunction]
        #[pyo3(name = "dtype", signature = (tensor))]
        pub fn dtype<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::dtype(&tensor);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d.
        #[pyfunction]
        #[pyo3(name = "exposed", signature = (tensor))]
        pub fn exposed<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::exposed(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Builds a tensor of the shape and width filled with one value.
        #[pyfunction]
        #[pyo3(name = "filled", signature = (shape, value, dtype))]
        pub fn filled<'py>(
            py: Python<'py>,
            shape: Vec<usize>,
            value: i64,
            dtype: PySerde<mrly_core::Dtype>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dtype = dtype.0;
            let out = mrly_core::Tensor::filled(shape, value, dtype);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Reverses the tensor along one axis.
        #[pyfunction]
        #[pyo3(name = "flip", signature = (tensor, axis))]
        pub fn flip<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            axis: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::flip(&tensor, axis);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Folds the tensor into its level-fold Kronecker power.
        #[pyfunction]
        #[pyo3(name = "fractal", signature = (tensor, level))]
        pub fn fractal<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::fractal(&tensor, level);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a u8 tensor filled with one value.
        #[pyfunction]
        #[pyo3(name = "full", signature = (shape, value))]
        pub fn full<'py>(
            py: Python<'py>,
            shape: Vec<usize>,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Tensor::full(shape, value);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Returns the byte at a multi-index.
        #[pyfunction]
        #[pyo3(name = "get", signature = (tensor, multi))]
        pub fn get<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            multi: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::get(&tensor, &multi);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Wraps an i32 vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "i32", signature = (data, shape))]
        pub fn i32_<'py>(
            py: Python<'py>,
            data: Vec<i32>,
            shape: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Tensor::i32(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as i32s.
        #[pyfunction]
        #[pyo3(name = "i32s", signature = (tensor))]
        pub fn i32s<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::i32s(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Folds a multi-index into its flat index.
        #[pyfunction]
        #[pyo3(name = "index", signature = (tensor, multi))]
        pub fn index<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            multi: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::index(&tensor, &multi);
            (out).into_bound_py_any(py)
        }

        /// Flips every element between zero and one.
        #[pyfunction]
        #[pyo3(name = "invert", signature = (tensor))]
        pub fn invert<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::invert(&tensor);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds the Kronecker product of the two tensors.
        #[pyfunction]
        #[pyo3(name = "kron", signature = (tensor, other))]
        pub fn kron<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            other: PyTensor,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let other = other.0;
            let out = mrly_core::Tensor::kron(&tensor, &other);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Numbers every position by its concentric ring out from the center.
        #[pyfunction]
        #[pyo3(name = "layers", signature = (tensor, dtype))]
        pub fn layers<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            dtype: PySerde<mrly_core::Dtype>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let dtype = dtype.0;
            let out = mrly_core::Tensor::layers(&tensor, dtype);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Counts each position's masked neighbors holding the target bit.
        #[pyfunction]
        #[pyo3(name = "neighbors", signature = (tensor, mask, target, wrap, dtype))]
        pub fn neighbors<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            mask: PyTensor,
            target: u8,
            wrap: bool,
            dtype: PySerde<mrly_core::Dtype>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let mask = mask.0;
            let dtype = dtype.0;
            let out = mrly_core::Tensor::neighbors(&tensor, &mask, target, wrap, dtype);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a zeroed u8 tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "new", signature = (shape))]
        pub fn new<'py>(py: Python<'py>, shape: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Tensor::new(shape);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Wraps a byte vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "of", signature = (data, shape))]
        pub fn of<'py>(
            py: Python<'py>,
            data: Vec<u8>,
            shape: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Tensor::of(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Otsu threshold splitting the histogram at greatest variance.
        #[pyfunction]
        #[pyo3(name = "otsu_threshold", signature = (tensor))]
        pub fn otsu_threshold<'py>(
            py: Python<'py>,
            tensor: PyTensor,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::otsu_threshold(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Wraps the tensor in a count-thick border of one value.
        #[pyfunction]
        #[pyo3(name = "pad", signature = (tensor, count, value))]
        pub fn pad<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            count: usize,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::pad(&tensor, count, value);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Stamps the value wherever the tiled mask is nonzero.
        #[pyfunction]
        #[pyo3(name = "perforate", signature = (tensor, mask, value))]
        pub fn perforate<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            mask: PyTensor,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let mask = mask.0;
            let out = mrly_core::Tensor::perforate(&tensor, &mask, value);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Writes the element at a flat index, which must be below the size like a slice index.
        #[pyfunction]
        #[pyo3(name = "put", signature = (tensor, flat, value))]
        pub fn put<'py>(
            py: Python<'py>,
            tensor: &Bound<'_, PyAny>,
            flat: usize,
            value: i64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut tensor_owned = crate::hand::tensor_from_py(tensor)?;
            mrly_core::Tensor::put(&mut tensor_owned, flat, value);
            crate::hand::tensor_write_back(tensor, &tensor_owned)?;
            ().into_bound_py_any(py)
        }

        /// Rotates the tensor k quarter turns in the plane of two axes.
        #[pyfunction]
        #[pyo3(name = "rot90", signature = (tensor, k, axes))]
        pub fn rot90<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            k: usize,
            axes: (usize, usize),
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::rot90(&tensor, k, axes);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Writes the byte at a multi-index.
        #[pyfunction]
        #[pyo3(name = "set", signature = (tensor, multi, value))]
        pub fn set<'py>(
            py: Python<'py>,
            tensor: &Bound<'_, PyAny>,
            multi: Vec<usize>,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut tensor_owned = crate::hand::tensor_from_py(tensor)?;
            let out = mrly_core::Tensor::set(&mut tensor_owned, &multi, value);
            crate::hand::tensor_write_back(tensor, &tensor_owned)?;
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the number of elements.
        #[pyfunction]
        #[pyo3(name = "size", signature = (tensor))]
        pub fn size<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::size(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Drops one axis by fixing it at an index.
        #[pyfunction]
        #[pyo3(name = "slice", signature = (tensor, axis, index))]
        pub fn slice<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            axis: usize,
            index: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::slice(&tensor, axis, index);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the sum of all elements.
        #[pyfunction]
        #[pyo3(name = "sum", signature = (tensor))]
        pub fn sum<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::sum(&tensor);
            (out).into_bound_py_any(py)
        }

        /// Repeats the tensor the given number of times along each axis.
        #[pyfunction]
        #[pyo3(name = "tile", signature = (tensor, reps))]
        pub fn tile<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            reps: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::tile(&tensor, &reps);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Swaps two axes.
        #[pyfunction]
        #[pyo3(name = "transpose", signature = (tensor, a, b))]
        pub fn transpose<'py>(
            py: Python<'py>,
            tensor: PyTensor,
            a: usize,
            b: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::transpose(&tensor, a, b);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a zeroed tensor of the shape and width.
        #[pyfunction]
        #[pyo3(name = "typed", signature = (shape, dtype))]
        pub fn typed<'py>(
            py: Python<'py>,
            shape: Vec<usize>,
            dtype: PySerde<mrly_core::Dtype>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dtype = dtype.0;
            let out = mrly_core::Tensor::typed(shape, dtype);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Wraps a u16 vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "u16", signature = (data, shape))]
        pub fn u16_<'py>(
            py: Python<'py>,
            data: Vec<u16>,
            shape: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Tensor::u16(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as u16s.
        #[pyfunction]
        #[pyo3(name = "u16s", signature = (tensor))]
        pub fn u16s<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::u16s(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Wraps a u32 vector as a tensor of the shape.
        #[pyfunction]
        #[pyo3(name = "u32", signature = (data, shape))]
        pub fn u32_<'py>(
            py: Python<'py>,
            data: Vec<u32>,
            shape: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Tensor::u32(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the elements as u32s.
        #[pyfunction]
        #[pyo3(name = "u32s", signature = (tensor))]
        pub fn u32s<'py>(py: Python<'py>, tensor: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tensor = tensor.0;
            let out = mrly_core::Tensor::u32s(&tensor);
            ((ok(out)?).to_vec()).into_bound_py_any(py)
        }

        /// Wraps a u8 vector as a tensor of the shape, the same door as `Tensor::of`.
        #[pyfunction]
        #[pyo3(name = "u8", signature = (data, shape))]
        pub fn u8_<'py>(
            py: Python<'py>,
            data: Vec<u8>,
            shape: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Tensor::u8(data, shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.core.tensor")?;
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
            let names: Vec<&str> = vec![
                "at",
                "binarize",
                "binarize_otsu",
                "blur",
                "bytes",
                "count",
                "dtype",
                "exposed",
                "filled",
                "flip",
                "fractal",
                "full",
                "get",
                "i32",
                "i32s",
                "index",
                "invert",
                "kron",
                "layers",
                "neighbors",
                "new",
                "of",
                "otsu_threshold",
                "pad",
                "perforate",
                "put",
                "rot90",
                "set",
                "size",
                "slice",
                "sum",
                "tile",
                "transpose",
                "typed",
                "u16",
                "u16s",
                "u32",
                "u32s",
                "u8",
            ];
            m.add("__all__", names)?;
            parent.add("tensor", &m)?;
            sys.set_item("mrly.core._core.core.tensor", &m)?;
            Ok(())
        }
    }

    /// A rule that turns counter values into colors.
    #[pyclass(name = "Colorizer", module = "mrly.core", skip_from_py_object)]
    pub struct Colorizer;

    #[pymethods]
    impl Colorizer {
        /// Returns the default Colorizer.
        #[staticmethod]
        #[pyo3(name = "default", signature = ())]
        pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Colorizer::default();
            (PySerde(out)).into_bound_py_any(py)
        }
        /// Builds the blue-to-red diverging ramp around a white middle.
        #[staticmethod]
        #[pyo3(name = "diverge", signature = ())]
        pub fn diverge<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Colorizer::diverge();
            (PySerde(out)).into_bound_py_any(py)
        }
        /// Builds the black-through-ember fire ramp: black, dark red, orange, light yellow.
        #[staticmethod]
        #[pyo3(name = "fire", signature = ())]
        pub fn fire<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Colorizer::fire();
            (PySerde(out)).into_bound_py_any(py)
        }
        /// Builds a binned colorizer from a gradient through the given stops.
        #[staticmethod]
        #[pyo3(name = "gradient_bins", signature = (background, colors, shades))]
        pub fn gradient_bins<'py>(
            py: Python<'py>,
            background: PyColor,
            colors: Vec<PyColor>,
            shades: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let background = background.0;
            let colors = colors.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_core::Colorizer::gradient_bins(background, &colors, shades);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }
        /// Builds the white-to-black heat ramp.
        #[staticmethod]
        #[pyo3(name = "heat", signature = ())]
        pub fn heat<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_core::Colorizer::heat();
            (PySerde(out)).into_bound_py_any(py)
        }
    }

    /// The element widths a tensor can hold.
    #[pyclass(name = "Dtype", module = "mrly.core", skip_from_py_object)]
    pub struct Dtype;

    #[pymethods]
    impl Dtype {
        /// Returns the largest value the width can hold.
        #[staticmethod]
        #[pyo3(name = "max", signature = (dtype))]
        pub fn max<'py>(
            py: Python<'py>,
            dtype: PySerde<mrly_core::Dtype>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dtype = dtype.0;
            let out = mrly_core::Dtype::max(dtype);
            (out).into_bound_py_any(py)
        }
    }

    /// Squashes rgba pixels to the hex aspect, returning the new width, height and pixels.
    #[pyfunction]
    #[pyo3(name = "hex_fit", signature = (pixels, width, height, vertical, filter))]
    pub fn hex_fit<'py>(
        py: Python<'py>,
        pixels: PyPixels,
        width: usize,
        height: usize,
        vertical: bool,
        filter: PySerde<mrly_core::Filter>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let pixels = pixels.0;
        let filter = filter.0;
        let out = mrly_core::hex_fit(&pixels, width, height, vertical, filter);
        ({
            let t = ok(out)?;
            (t.0, t.1, PyPixels(t.2))
        })
        .into_bound_py_any(py)
    }

    /// Returns the size a hex rendering wears, the named axis squashed by the triangle ratio.
    #[pyfunction]
    #[pyo3(name = "hex_size", signature = (width, height, vertical))]
    pub fn hex_size<'py>(
        py: Python<'py>,
        width: usize,
        height: usize,
        vertical: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_core::hex_size(width, height, vertical);
        (out).into_bound_py_any(py)
    }

    /// Resamples rgba pixels to a new size.
    #[pyfunction]
    #[pyo3(name = "resample", signature = (pixels, width, height, out_w, out_h, filter))]
    pub fn resample<'py>(
        py: Python<'py>,
        pixels: PyPixels,
        width: usize,
        height: usize,
        out_w: usize,
        out_h: usize,
        filter: PySerde<mrly_core::Filter>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let pixels = pixels.0;
        let filter = filter.0;
        let out = mrly_core::resample(&pixels, width, height, out_w, out_h, filter);
        (PyPixels(ok(out)?)).into_bound_py_any(py)
    }

    pub fn init(
        py: Python<'_>,
        parent: &Bound<'_, PyModule>,
        sys: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let m = PyModule::new(py, "mrly.core")?;
        m.setattr("__doc__", "The substrate: tensors, cells, colors, images, resampling, seeded chance and the canonical names.\n\nThe road from a grid of bytes to pixels, and the one JSON object every mrly thing is named by.\n\n- `tensor` makes and tallies the byte grids; `cell` dresses one in colors and tags.\n- `colors` holds the rgba color, the two themes and the house palette; `paint` spreads a palette over a cell.\n- `ramp` turns counter values into colors; `resample` rescales pixels and squashes them for hex.\n- `image` holds the rgba pixels a renderer returns; encoding them into a file is the caller's.\n- `rng` deals seeded chance: one xoshiro256++ stream, passed by hand, never global.\n- `error` holds the one error, its Result and the json parser.\n- `name` holds the `Named` trait, the `kind!` marker, the text views and the sha256 a name's id is cut from; `named_enum!` names an enum.\n\nThe json value and map are serde_json's, kept under `preserve_order` so an object comes back in the order it was written and both bridges print the same text.\n\nThe doors: `Tensor::of`, `Tensor::rot90`, `Color::from_hex`, `Color::to_hex`, `Image::scale` and `ramp::color`.")?;
        m.add_class::<PyRng>()?;
        m.add_class::<Colorizer>()?;
        m.add_class::<Dtype>()?;
        m.add_function(wrap_pyfunction!(hex_fit, &m)?)?;
        m.add_function(wrap_pyfunction!(hex_size, &m)?)?;
        m.add_function(wrap_pyfunction!(resample, &m)?)?;
        m.add("HEX_RATIO", mrly_core::HEX_RATIO)?;
        let names: Vec<&str> = vec![
            "hex_fit",
            "hex_size",
            "resample",
            "Colorizer",
            "Dtype",
            "HEX_RATIO",
            "Rng",
        ];
        m.add("__all__", names)?;
        cell::init(py, &m, sys)?;
        colors::init(py, &m, sys)?;
        error::init(py, &m, sys)?;
        image::init(py, &m, sys)?;
        name::init(py, &m, sys)?;
        paint::init(py, &m, sys)?;
        ramp::init(py, &m, sys)?;
        rng::init(py, &m, sys)?;
        tensor::init(py, &m, sys)?;
        parent.add("core", &m)?;
        sys.set_item("mrly.core._core.core", &m)?;
        Ok(())
    }
}

pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = py
        .import("sys")?
        .getattr("modules")?
        .cast_into::<PyDict>()?;
    core::init(py, root, &sys)?;
    Ok(())
}
