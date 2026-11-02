#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// The alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke.
pub mod font {
    use crate::hand::PySerde;
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// The stroke orders that write each character.
    pub mod paths {
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the character's hand-penned strokes from the pen tables, or None outside the font.
        #[pyfunction]
        #[pyo3(name = "penned", signature = (c))]
        pub fn penned<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_font::paths::penned(c);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.font.paths")?;
            m.setattr("__doc__", "The stroke orders that write each character.")?;
            m.add_function(wrap_pyfunction!(penned, &m)?)?;
            let names: Vec<&str> = vec!["penned"];
            m.add("__all__", names)?;
            parent.add("paths", &m)?;
            sys.set_item("mrly.font._font.font.paths", &m)?;
            Ok(())
        }
    }

    /// The hand-penned stroke tables, one per glyph.
    pub mod pens {
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns every pen in font order: uppers, lowers, digits, extras, specials.
        #[pyfunction]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_font::pens::all();
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.font.pens")?;
            m.setattr("__doc__", "The hand-penned stroke tables, one per glyph.")?;
            m.add_function(wrap_pyfunction!(all, &m)?)?;
            let names: Vec<&str> = vec!["all"];
            m.add("__all__", names)?;
            parent.add("pens", &m)?;
            sys.set_item("mrly.font._font.font.pens", &m)?;
            Ok(())
        }
    }

    /// One character's pixel bitmap.
    #[pyclass(name = "Glyph", module = "mrly.font", from_py_object)]
    #[derive(Clone)]
    pub struct Glyph(pub mrly_font::Glyph);

    #[pymethods]
    impl Glyph {
        /// Builds a glyph from its character and rows.
        #[new]
        #[pyo3(signature = (char_, rows))]
        pub fn __new__(char_: char, rows: Vec<String>) -> PyResult<Self> {
            let out = mrly_font::Glyph::new(char_, rows);
            Ok(Self(out))
        }
        /// The character the glyph draws.
        #[getter]
        #[pyo3(name = "char")]
        pub fn char_<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.char;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "char")]
        pub fn set_char(&mut self, value: char) -> PyResult<()> {
            self.0.char = value;
            Ok(())
        }
        /// The bitmap rows of '0' and '1' characters.
        #[getter]
        #[pyo3(name = "rows")]
        pub fn rows<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.rows.clone();
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "rows")]
        pub fn set_rows(&mut self, value: Vec<String>) -> PyResult<()> {
            self.0.rows = value;
            Ok(())
        }
        /// Returns the number of rows.
        #[pyo3(name = "height", signature = ())]
        pub fn height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_font::Glyph::height(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Builds a glyph from its character and rows.
        #[staticmethod]
        #[pyo3(name = "new", signature = (char_, rows))]
        pub fn new_<'py>(
            py: Python<'py>,
            char_: char,
            rows: Vec<String>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_font::Glyph::new(char_, rows);
            (crate::doors::font::Glyph(out)).into_bound_py_any(py)
        }
        /// Returns the cell width of the first row, or 0 for an empty glyph.
        #[pyo3(name = "width", signature = ())]
        pub fn width<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_font::Glyph::width(&self.0);
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
        let out = mrly_font::all();
        ((out)
            .into_iter()
            .map(crate::doors::font::Glyph)
            .collect::<Vec<_>>())
        .into_bound_py_any(py)
    }

    /// Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster.
    #[pyfunction]
    #[pyo3(name = "animate", signature = (text, pad))]
    pub fn animate<'py>(py: Python<'py>, text: &str, pad: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::animate(text, pad);
        (PySerde(out)).into_bound_py_any(py)
    }

    /// Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any.
    #[pyfunction]
    #[pyo3(name = "cycle", signature = (write, merge, hold))]
    pub fn cycle<'py>(
        py: Python<'py>,
        write: PySerde<mrly_font::Anim>,
        merge: Vec<Vec<usize>>,
        hold: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let write = write.0;
        let out = mrly_font::cycle(&write, &merge, hold);
        (PySerde(out)).into_bound_py_any(py)
    }

    /// Builds the ten digit glyphs.
    #[pyfunction]
    #[pyo3(name = "digits", signature = ())]
    pub fn digits<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::digits();
        ((out)
            .into_iter()
            .map(crate::doors::font::Glyph)
            .collect::<Vec<_>>())
        .into_bound_py_any(py)
    }

    /// Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck.
    #[pyfunction]
    #[pyo3(name = "draft", signature = (rows))]
    pub fn draft<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::draft(&rows);
        (out).into_bound_py_any(py)
    }

    /// Builds the punctuation, symbol and arrow glyphs.
    #[pyfunction]
    #[pyo3(name = "extras", signature = ())]
    pub fn extras<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::extras();
        ((out)
            .into_iter()
            .map(crate::doors::font::Glyph)
            .collect::<Vec<_>>())
        .into_bound_py_any(py)
    }

    /// Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank.
    #[pyfunction]
    #[pyo3(name = "floor", signature = (rows))]
    pub fn floor<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::floor(&rows);
        (out).into_bound_py_any(py)
    }

    /// Returns an owned copy of the character's glyph, or None outside the font.
    #[pyfunction]
    #[pyo3(name = "glyph", signature = (c))]
    pub fn glyph<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::glyph(c);
        ((out).map(crate::doors::font::Glyph)).into_bound_py_any(py)
    }

    /// Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form.
    #[pyfunction]
    #[pyo3(name = "lower", signature = (rows))]
    pub fn lower<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let rows = rows.iter().map(|y| y.as_str()).collect::<Vec<_>>();
        let out = mrly_font::lower(&rows);
        (out).into_bound_py_any(py)
    }

    /// Builds the twenty-six lowercase glyphs by rounding the uppers' corners.
    #[pyfunction]
    #[pyo3(name = "lowers", signature = ())]
    pub fn lowers<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::lowers();
        ((out)
            .into_iter()
            .map(crate::doors::font::Glyph)
            .collect::<Vec<_>>())
        .into_bound_py_any(py)
    }

    /// Returns the whole font as a map from character to bitmap rows.
    #[pyfunction]
    #[pyo3(name = "map", signature = ())]
    pub fn map<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::map();
        (out).into_bound_py_any(py)
    }

    /// Folds the written text's glyphs, frame by frame, into one centered stack.
    #[pyfunction]
    #[pyo3(name = "merge", signature = (text, pad))]
    pub fn merge<'py>(py: Python<'py>, text: &str, pad: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::merge(text, pad);
        (out).into_bound_py_any(py)
    }

    /// Returns the character's Unicode name, or a U+ code point label for a character outside the font.
    #[pyfunction]
    #[pyo3(name = "name_of", signature = (c))]
    pub fn name_of<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::name_of(c);
        (out).into_bound_py_any(py)
    }

    /// Flattens the character's strokes into one cell-by-cell drawing order.
    #[pyfunction]
    #[pyo3(name = "path", signature = (c))]
    pub fn path<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::path(c);
        (out).into_bound_py_any(py)
    }

    /// Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart.
    #[pyfunction]
    #[pyo3(name = "raster", signature = (text))]
    pub fn raster<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::raster(text);
        (out).into_bound_py_any(py)
    }

    /// Builds the four seven-row glyphs: dollar, at, copyright and registered.
    #[pyfunction]
    #[pyo3(name = "specials", signature = ())]
    pub fn specials<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::specials();
        ((out)
            .into_iter()
            .map(crate::doors::font::Glyph)
            .collect::<Vec<_>>())
        .into_bound_py_any(py)
    }

    /// Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font.
    #[pyfunction]
    #[pyo3(name = "strokes", signature = (c))]
    pub fn strokes<'py>(py: Python<'py>, c: char) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::strokes(c);
        (out).into_bound_py_any(py)
    }

    /// Returns every character in the font, in font order.
    #[pyfunction]
    #[pyo3(name = "supported", signature = ())]
    pub fn supported<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::supported();
        (out).into_bound_py_any(py)
    }

    /// Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has.
    #[pyfunction]
    #[pyo3(name = "trim", signature = (rows))]
    pub fn trim<'py>(py: Python<'py>, rows: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::trim(&rows);
        (out).into_bound_py_any(py)
    }

    /// Builds the twenty-six uppercase glyphs.
    #[pyfunction]
    #[pyo3(name = "uppers", signature = ())]
    pub fn uppers<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_font::uppers();
        ((out)
            .into_iter()
            .map(crate::doors::font::Glyph)
            .collect::<Vec<_>>())
        .into_bound_py_any(py)
    }

    pub fn init(
        py: Python<'_>,
        parent: &Bound<'_, PyModule>,
        sys: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let m = PyModule::new(py, "mrly.font")?;
        m.setattr("__doc__", "The alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke.\n\n- `bitmaps` holds the raw on/off rows; `glyph` builds them into glyphs and trims them.\n- `pens` holds every glyph's hand-penned strokes; `paths` reads them and drafts new ones.\n- `names` gives a character its Unicode name.\n- `raster` lays a text out as one 0/1 grid; `animate` writes it cell by cell and loops the cycle.\n\nThe font is the uppers, their corner-rounded lowers, the digits, the punctuation and arrows, and four specials; most glyphs are five rows of five, the ten that dip below the baseline are seven.\n\nThe doors: `glyph`, `supported`, `map`, `raster`, `path`, `floor`, `animate`, `merge`, `cycle` and `name_of`.\n\n`cargo run -p mrly-font --example pen` prints the pen tables; `-- X` drafts one glyph and its stroke floor.")?;
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
        m.add("FPS", mrly_font::FPS)?;
        m.add("HOLD", mrly_font::HOLD)?;
        let names: Vec<&str> = vec![
            "all",
            "animate",
            "cycle",
            "digits",
            "draft",
            "extras",
            "floor",
            "glyph",
            "lower",
            "lowers",
            "map",
            "merge",
            "name_of",
            "path",
            "raster",
            "specials",
            "strokes",
            "supported",
            "trim",
            "uppers",
            "Glyph",
            "FPS",
            "HOLD",
        ];
        m.add("__all__", names)?;
        paths::init(py, &m, sys)?;
        pens::init(py, &m, sys)?;
        parent.add("font", &m)?;
        sys.set_item("mrly.font._font.font", &m)?;
        Ok(())
    }
}

pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = py
        .import("sys")?
        .getattr("modules")?
        .cast_into::<PyDict>()?;
    font::init(py, root, &sys)?;
    Ok(())
}
