#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// The engine: a rule over a grid, stepped, recorded, measured and rendered.
pub mod life {
    use crate::hand::{PyCell2d, PyCellNd, PyCode, PyImage, PySerde, PyTensor, ok};
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// The elementary automata: their stepping, their space-time diagrams and the card of one rule.
    pub mod elementary {
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell.
        #[pyfunction]
        #[pyo3(name = "output", signature = (rule, l, c, r))]
        pub fn output<'py>(
            py: Python<'py>,
            rule: u8,
            l: u8,
            c: u8,
            r: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::elementary::output(rule, l, c, r);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.life.elementary")?;
            m.setattr("__doc__", "The elementary automata: their stepping, their space-time diagrams and the card of one rule.")?;
            m.add_function(wrap_pyfunction!(output, &m)?)?;
            let names: Vec<&str> = vec!["output"];
            m.add("__all__", names)?;
            parent.add("elementary", &m)?;
            sys.set_item("mrly.life._life.life.elementary", &m)?;
            Ok(())
        }
    }

    /// The recipe grammar and its two-mode run: one name regenerates a run bit for bit.
    pub mod recipe {
        use crate::hand::{PyCell, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A recipe's run, stepped one generation at a time until its stop.
        #[pyclass(name = "Run", module = "mrly.life.recipe", from_py_object)]
        #[derive(Clone)]
        pub struct Run(pub mrly_life::recipe::Run);

        #[pymethods]
        impl Run {
            /// Sets a recipe up at generation zero: the tiled seed, the mask's kernel, the rule drawn against its budget.
            #[new]
            #[pyo3(signature = (recipe))]
            pub fn __new__(recipe: PySerde<mrly_life::recipe::Recipe>) -> PyResult<Self> {
                let recipe = recipe.0;
                let out = mrly_life::recipe::Run::new(&recipe);
                Ok(Self(ok(out)?))
            }
            /// Returns the background the frame sits on, 1 once a flipping rule has turned it.
            #[pyo3(name = "background", signature = ())]
            pub fn background<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::background(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the birth counts drawn against the budget.
            #[pyo3(name = "birth", signature = ())]
            pub fn birth<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::birth(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the mask's budget, the most neighbours a cell can count.
            #[pyo3(name = "budget", signature = ())]
            pub fn budget<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::budget(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the canvas side: an island's cap, at least its seed board, or a wallpaper's torus.
            #[pyo3(name = "cap", signature = ())]
            pub fn cap<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::cap(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns whether the rule flips its background, birth holding zero.
            #[pyo3(name = "flips", signature = ())]
            pub fn flips<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::flips(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the live frame: the cropped island pattern, or the whole torus.
            #[pyo3(name = "frame", signature = ())]
            pub fn frame<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::frame(&self.0);
                (PyCell(out)).into_bound_py_any(py)
            }
            /// Returns the generation reached.
            #[pyo3(name = "generation", signature = ())]
            pub fn generation<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::generation(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the mask as a square tensor of bytes, its centre popped.
            #[pyo3(name = "mask", signature = ())]
            pub fn mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::mask(&self.0);
                (PyTensor(out)).into_bound_py_any(py)
            }
            /// Sets a recipe up at generation zero: the tiled seed, the mask's kernel, the rule drawn against its budget.
            #[staticmethod]
            #[pyo3(name = "new", signature = (recipe))]
            pub fn new_<'py>(
                py: Python<'py>,
                recipe: PySerde<mrly_life::recipe::Recipe>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let recipe = recipe.0;
                let out = mrly_life::recipe::Run::new(&recipe);
                (crate::doors::life::recipe::Run(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the row and column of the frame's top-left corner, the seed's top-left corner being the origin.
            #[pyo3(name = "origin", signature = ())]
            pub fn origin<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::origin(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the live cells of the frame.
            #[pyo3(name = "population", signature = ())]
            pub fn population<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::population(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the mask's reach, half its side.
            #[pyo3(name = "reach", signature = ())]
            pub fn reach<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::reach(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the side of the seed board, the seed's side to its level times its tiling.
            #[pyo3(name = "side", signature = ())]
            pub fn side<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::side(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Advances one generation and returns the stop the moment the run ends; after a stop it returns the same stop and changes nothing.
            #[pyo3(name = "step", signature = ())]
            pub fn step<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::step(&mut self.0);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
            /// Returns the stop once the run has ended.
            #[pyo3(name = "stop", signature = ())]
            pub fn stop<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::stop(&self.0);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
            /// Returns the survive counts drawn against the budget.
            #[pyo3(name = "survive", signature = ())]
            pub fn survive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::survive(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        /// The mode of a run.
        #[pyclass(name = "Mode", module = "mrly.life.recipe", skip_from_py_object)]
        pub struct Mode;

        #[pymethods]
        impl Mode {
            /// Returns every Mode in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Mode::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// Lists what every knob of a recipe may offer, in grammar order.
        #[pyfunction]
        #[pyo3(name = "grammar", signature = ())]
        pub fn grammar<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::recipe::grammar();
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Writes a recipe as its name, the one string that regenerates it.
        #[pyfunction]
        #[pyo3(name = "name", signature = (recipe))]
        pub fn name<'py>(
            py: Python<'py>,
            recipe: PySerde<mrly_life::recipe::Recipe>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let recipe = recipe.0;
            let out = mrly_life::recipe::name(&recipe);
            (out).into_bound_py_any(py)
        }

        /// Reads a recipe out of its name.
        #[pyfunction]
        #[pyo3(name = "parse", signature = (name))]
        pub fn parse<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::recipe::parse(name);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.life.recipe")?;
            m.setattr("__doc__", "The recipe grammar and its two-mode run: one name regenerates a run bit for bit.\n\n- A name reads `seed.side.level x<tiling> <mode> mask:<mask> birth:<side> survive:<side> x:<limit> cap:<cap>`; a wallpaper drops the cap and tiles once.\n- The seed is a plane class `code1 code3 code6 code7 code15` or a named design at base 2; the mask is a design popped at its centre, or `copy` and `copyinv` of the tiled seed.\n- A rule side names a source with the `_zeros` and `_ones` rolls, or lists its counts; the counts are drawn against the mask's budget.\n- Island: the seed alone on the plane; the canvas grows with the pattern, and the run stops when a live cell comes within the mask's reach of the cap wall.\n- Wallpaper: the seed is the torus, stepped on its own side.\n- A zero in birth runs the rule on a flipping background; the frame is the pattern against its background.\n- A repeated frame stops the run as still, loop or mover with its period and displacement; a mover tells the generation by which it reaches the wall.\n- Counts come from the mask's Kronecker factors, one dilated pass per factor.")?;
            m.add_class::<Run>()?;
            m.add_class::<Mode>()?;
            m.add_function(wrap_pyfunction!(grammar, &m)?)?;
            m.add_function(wrap_pyfunction!(name, &m)?)?;
            m.add_function(wrap_pyfunction!(parse, &m)?)?;
            let names: Vec<&str> = vec!["grammar", "name", "parse", "Run", "Mode"];
            m.add("__all__", names)?;
            parent.add("recipe", &m)?;
            sys.set_item("mrly.life._life.life.recipe", &m)?;
            Ok(())
        }
    }

    /// The frames and the cumulative-visit heatmap of grids, as images.
    pub mod render {
        use crate::hand::{PyCell2d, PyImage, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Renders one grid to a white-on-black image at a pixel scale.
        #[pyfunction]
        #[pyo3(name = "frame", signature = (grid, scale))]
        pub fn frame<'py>(
            py: Python<'py>,
            grid: PyCell2d,
            scale: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrly_life::render::frame(&grid, scale);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.life.render")?;
            m.setattr(
                "__doc__",
                "The frames and the cumulative-visit heatmap of grids, as images.",
            )?;
            m.add_function(wrap_pyfunction!(frame, &m)?)?;
            let names: Vec<&str> = vec!["frame"];
            m.add("__all__", names)?;
            parent.add("render", &m)?;
            sys.set_item("mrly.life._life.life.render", &m)?;
            Ok(())
        }
    }

    /// The named sources of neighbor-count values, and the counts they lay down.
    pub mod source {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Generates the sequence's values up to the limit.
        #[pyfunction]
        #[pyo3(name = "sequence", signature = (seq, limit))]
        pub fn sequence<'py>(
            py: Python<'py>,
            seq: crate::doors::life::Source,
            limit: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let seq = seq.0;
            let out = mrly_life::source::sequence(seq, limit);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.life.source")?;
            m.setattr(
                "__doc__",
                "The named sources of neighbor-count values, and the counts they lay down.",
            )?;
            m.add_function(wrap_pyfunction!(sequence, &m)?)?;
            let names: Vec<&str> = vec!["sequence"];
            m.add("__all__", names)?;
            parent.add("source", &m)?;
            sys.set_item("mrly.life._life.life.source", &m)?;
            Ok(())
        }
    }

    /// The rulebook of a life run.
    #[pyclass(name = "Config", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Config(pub mrly_life::Config);

    #[pymethods]
    impl Config {
        /// Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.
        #[new]
        #[pyo3(signature = (mask, birth, survive))]
        pub fn __new__(
            mask: PyCell2d,
            birth: crate::doors::life::Counts,
            survive: crate::doors::life::Counts,
        ) -> PyResult<Self> {
            let mask = mask.0;
            let birth = birth.0;
            let survive = survive.0;
            let out = mrly_life::Config::new(mask, birth, survive);
            Ok(Self(out))
        }
        /// The neighborhood mask.
        #[getter]
        #[pyo3(name = "mask")]
        pub fn mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.mask.clone();
            (PyCellNd(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "mask")]
        pub fn set_mask(&mut self, value: PyCell2d) -> PyResult<()> {
            let value = value.0;
            self.0.mask = value;
            Ok(())
        }
        /// The neighbor counts that create a cell.
        #[getter]
        #[pyo3(name = "birth")]
        pub fn birth<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.birth.clone();
            (crate::doors::life::Counts(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "birth")]
        pub fn set_birth(&mut self, value: crate::doors::life::Counts) -> PyResult<()> {
            let value = value.0;
            self.0.birth = value;
            Ok(())
        }
        /// The neighbor counts that keep a cell.
        #[getter]
        #[pyo3(name = "survive")]
        pub fn survive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.survive.clone();
            (crate::doors::life::Counts(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "survive")]
        pub fn set_survive(&mut self, value: crate::doors::life::Counts) -> PyResult<()> {
            let value = value.0;
            self.0.survive = value;
            Ok(())
        }
        /// The edge policy.
        #[getter]
        #[pyo3(name = "boundary")]
        pub fn boundary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.boundary;
            (PySerde(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "boundary")]
        pub fn set_boundary(&mut self, value: PySerde<mrly_life::Boundary>) -> PyResult<()> {
            let value = value.0;
            self.0.boundary = value;
            Ok(())
        }
        /// The generation cap.
        #[getter]
        #[pyo3(name = "max_generations")]
        pub fn max_generations<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.max_generations;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "max_generations")]
        pub fn set_max_generations(&mut self, value: usize) -> PyResult<()> {
            self.0.max_generations = value;
            Ok(())
        }
        /// The tiling factor applied to the seed.
        #[getter]
        #[pyo3(name = "grid_size")]
        pub fn grid_size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.grid_size;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "grid_size")]
        pub fn set_grid_size(&mut self, value: usize) -> PyResult<()> {
            self.0.grid_size = value;
            Ok(())
        }
        /// The dead border added around the seed.
        #[getter]
        #[pyo3(name = "padding")]
        pub fn padding<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.padding;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "padding")]
        pub fn set_padding(&mut self, value: usize) -> PyResult<()> {
            self.0.padding = value;
            Ok(())
        }
        /// Returns the largest neighbor count the mask can reach.
        #[pyo3(name = "budget", signature = ())]
        pub fn budget<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Config::budget(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Resolves the birth and survive counts against the mask's budget.
        #[pyo3(name = "counts", signature = ())]
        pub fn counts<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Config::counts(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.
        #[staticmethod]
        #[pyo3(name = "new", signature = (mask, birth, survive))]
        pub fn new_<'py>(
            py: Python<'py>,
            mask: PyCell2d,
            birth: crate::doors::life::Counts,
            survive: crate::doors::life::Counts,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let birth = birth.0;
            let survive = survive.0;
            let out = mrly_life::Config::new(mask, birth, survive);
            (crate::doors::life::Config(out)).into_bound_py_any(py)
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
    #[pyclass(name = "Counts", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Counts(pub mrly_life::Counts);

    #[pymethods]
    impl Counts {
        /// Builds the counts a sequence lays down, keeping zeros and ones on request.
        #[staticmethod]
        #[pyo3(name = "drawn", signature = (seq, zeros, ones))]
        pub fn drawn<'py>(
            py: Python<'py>,
            seq: crate::doors::life::Source,
            zeros: bool,
            ones: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let seq = seq.0;
            let out = mrly_life::Counts::drawn(seq, zeros, ones);
            (crate::doors::life::Counts(out)).into_bound_py_any(py)
        }
        /// Spells the counts outright.
        #[staticmethod]
        #[pyo3(name = "list", signature = (counts))]
        pub fn list<'py>(py: Python<'py>, counts: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Counts::list(counts);
            (crate::doors::life::Counts(out)).into_bound_py_any(py)
        }
        /// Returns the counts, a drawn side resolved against the mask's neighbor budget.
        #[pyo3(name = "values", signature = (budget))]
        pub fn values<'py>(&self, py: Python<'py>, budget: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Counts::values(&self.0, budget);
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
    #[pyclass(name = "Life", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Life(pub mrly_life::Life);

    #[pymethods]
    impl Life {
        /// Every generation in order.
        #[getter]
        #[pyo3(name = "grids")]
        pub fn grids<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.grids.clone();
            ((value).into_iter().map(PyCellNd).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "grids")]
        pub fn set_grids(&mut self, value: Vec<PyCell2d>) -> PyResult<()> {
            let value = value.into_iter().map(|x| x.0).collect::<Vec<_>>();
            self.0.grids = value;
            Ok(())
        }
        /// The run's ending.
        #[getter]
        #[pyo3(name = "fate")]
        pub fn fate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.fate;
            (PySerde(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "fate")]
        pub fn set_fate(&mut self, value: PySerde<mrly_life::Fate>) -> PyResult<()> {
            let value = value.0;
            self.0.fate = value;
            Ok(())
        }
        /// The number of recorded generations.
        #[getter]
        #[pyo3(name = "count")]
        pub fn count<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.count;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "count")]
        pub fn set_count(&mut self, value: usize) -> PyResult<()> {
            self.0.count = value;
            Ok(())
        }
        /// The cycle length when the fate is a loop, else zero.
        #[getter]
        #[pyo3(name = "loop_length")]
        pub fn loop_length<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.loop_length;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "loop_length")]
        pub fn set_loop_length(&mut self, value: usize) -> PyResult<()> {
            self.0.loop_length = value;
            Ok(())
        }
        /// Returns the final grid, or None when the run is empty.
        #[pyo3(name = "last", signature = ())]
        pub fn last<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Life::last(&self.0);
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
    #[pyclass(name = "Rule", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Rule(pub mrly_life::Rule);

    #[pymethods]
    impl Rule {
        /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
        #[new]
        #[pyo3(signature = (birth, survive, wrap))]
        pub fn __new__(
            birth: crate::doors::life::Counts,
            survive: crate::doors::life::Counts,
            wrap: bool,
        ) -> PyResult<Self> {
            let birth = birth.0;
            let survive = survive.0;
            let out = mrly_life::Rule::new(birth, survive, wrap);
            Ok(Self(out))
        }
        /// The neighbor counts that create a cell, listed or drawn from a sequence.
        #[getter]
        #[pyo3(name = "birth")]
        pub fn birth<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.birth.clone();
            (crate::doors::life::Counts(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "birth")]
        pub fn set_birth(&mut self, value: crate::doors::life::Counts) -> PyResult<()> {
            let value = value.0;
            self.0.birth = value;
            Ok(())
        }
        /// The neighbor counts that keep a cell, listed or drawn from a sequence.
        #[getter]
        #[pyo3(name = "survive")]
        pub fn survive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.survive.clone();
            (crate::doors::life::Counts(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "survive")]
        pub fn set_survive(&mut self, value: crate::doors::life::Counts) -> PyResult<()> {
            let value = value.0;
            self.0.survive = value;
            Ok(())
        }
        /// Whether the edge wraps, false unless said.
        #[getter]
        #[pyo3(name = "wrap")]
        pub fn wrap<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.wrap;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "wrap")]
        pub fn set_wrap(&mut self, value: bool) -> PyResult<()> {
            self.0.wrap = value;
            Ok(())
        }
        /// Returns the edge policy the rule runs under.
        #[pyo3(name = "boundary", signature = ())]
        pub fn boundary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Rule::boundary(&self.0);
            (PySerde(out)).into_bound_py_any(py)
        }
        /// Folds a decoded value to its canonical form, or an error for one outside the kind.
        #[pyo3(name = "checked", signature = ())]
        pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::checked(self.0.clone());
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Builds a life config running this rule over a neighborhood mask.
        #[pyo3(name = "config", signature = (mask))]
        pub fn config<'py>(&self, py: Python<'py>, mask: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let out = mrly_life::Rule::config(&self.0, mask);
            (crate::doors::life::Config(out)).into_bound_py_any(py)
        }
        /// Reads a filename back into the value, or an error.
        #[staticmethod]
        #[pyo3(name = "from_file", signature = (text))]
        pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::from_file(text);
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Reads a JSON object into its canonical value, or an error naming the broken key.
        #[staticmethod]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::from_json(text);
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Reads a path and query string back into the value, or an error.
        #[staticmethod]
        #[pyo3(name = "from_url", signature = (text))]
        pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::from_url(text);
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
        #[staticmethod]
        #[pyo3(name = "new", signature = (birth, survive, wrap))]
        pub fn new_<'py>(
            py: Python<'py>,
            birth: crate::doors::life::Counts,
            survive: crate::doors::life::Counts,
            wrap: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let birth = birth.0;
            let survive = survive.0;
            let out = mrly_life::Rule::new(birth, survive, wrap);
            (crate::doors::life::Rule(out)).into_bound_py_any(py)
        }
        /// Reads the rule out of a life config.
        #[staticmethod]
        #[pyo3(name = "of", signature = (config))]
        pub fn of<'py>(
            py: Python<'py>,
            config: PyRef<'_, crate::doors::life::Config>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Rule::of(&config.0);
            (crate::doors::life::Rule(out)).into_bound_py_any(py)
        }
        /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
        #[pyo3(name = "to_file", signature = ())]
        pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_file(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Prints the first eight hex digits of the sha256 of the canonical JSON.
        #[pyo3(name = "to_id", signature = ())]
        pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_id(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Prints the canonical JSON object.
        #[pyo3(name = "to_json", signature = ())]
        pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_json(&self.0);
            (out).into_bound_py_any(py)
        }
        /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
        #[pyo3(name = "to_prose", signature = ())]
        pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_prose(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
        #[pyo3(name = "to_url", signature = ())]
        pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_url(&self.0);
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
    #[pyclass(name = "Source", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Source(pub mrly_life::Source);

    #[pymethods]
    impl Source {
        /// Returns every fixed sequence, the seeded and coded families excluded.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::all();
            ((out)
                .into_iter()
                .map(crate::doors::life::Source)
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }
        /// Returns the seventeen mrly design families: the grid, the four classics and their antis.
        #[staticmethod]
        #[pyo3(name = "designs", signature = ())]
        pub fn designs<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::designs();
            ((out)
                .into_iter()
                .map(crate::doors::life::Source)
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }
        /// Returns whether the sequence is a seeded random draw.
        #[pyo3(name = "is_random", signature = ())]
        pub fn is_random<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::is_random(self.0);
            (out).into_bound_py_any(py)
        }
        /// Returns the sequence's parseable name, the one string that regenerates it.
        #[pyo3(name = "name", signature = ())]
        pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::name(self.0);
            (out).into_bound_py_any(py)
        }
        /// Returns the six number sequences, the random one listed under seed zero.
        #[staticmethod]
        #[pyo3(name = "numbers", signature = ())]
        pub fn numbers<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::numbers();
            ((out)
                .into_iter()
                .map(crate::doors::life::Source)
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }
        /// Returns the sequence's OEIS id, or None off the encyclopedia.
        #[pyo3(name = "oeis", signature = ())]
        pub fn oeis<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::oeis(self.0);
            (out).into_bound_py_any(py)
        }
        /// Parses a sequence name back to its source.
        #[staticmethod]
        #[pyo3(name = "parse", signature = (name))]
        pub fn parse<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::parse(name);
            (crate::doors::life::Source(ok(out)?)).into_bound_py_any(py)
        }
        /// Reads a canonical name off the front of the text, returning the tail left over.
        #[staticmethod]
        #[pyo3(name = "read", signature = (text))]
        pub fn read<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::read(text);
            ((out).map(|x| {
                let t = x;
                (crate::doors::life::Source(t.0), (t.1).to_string())
            }))
            .into_bound_py_any(py)
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
    #[pyclass(name = "Boundary", module = "mrly.life", skip_from_py_object)]
    pub struct Boundary;

    #[pymethods]
    impl Boundary {
        /// Returns every Boundary in canonical order.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Boundary::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        /// Returns whether the edges wrap.
        #[staticmethod]
        #[pyo3(name = "wrap", signature = (boundary))]
        pub fn wrap<'py>(
            py: Python<'py>,
            boundary: PySerde<mrly_life::Boundary>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let boundary = boundary.0;
            let out = mrly_life::Boundary::wrap(boundary);
            (out).into_bound_py_any(py)
        }
    }

    /// The ending of a life run.
    #[pyclass(name = "Fate", module = "mrly.life", skip_from_py_object)]
    pub struct Fate;

    #[pymethods]
    impl Fate {
        /// Returns every Fate in canonical order.
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Fate::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
    }

    /// Returns whether a rule is affine, its algebraic degree at most one.
    #[pyfunction]
    #[pyo3(name = "affine", signature = (rule))]
    pub fn affine<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::affine(rule);
        (out).into_bound_py_any(py)
    }

    /// Runs a seed under a config until it fixes, loops or times out, recording every generation.
    #[pyfunction]
    #[pyo3(name = "animate", signature = (seed, config))]
    pub fn animate<'py>(
        py: Python<'py>,
        seed: PyCell2d,
        config: PyRef<'_, crate::doors::life::Config>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let seed = seed.0;
        let out = mrly_life::animate(&seed, &config.0);
        (crate::doors::life::Life(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns the mean fraction of sites changed between consecutive grids.
    #[pyfunction]
    #[pyo3(name = "churn", signature = (grids))]
    pub fn churn<'py>(py: Python<'py>, grids: Vec<PyCell2d>) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrly_life::churn(&grids);
        (out).into_bound_py_any(py)
    }

    /// Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`.
    #[pyfunction]
    #[pyo3(name = "corner_bits", signature = (rule))]
    pub fn corner_bits<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::corner_bits(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the sequence up to max_neighbors, keeping zeros and ones only on request.
    #[pyfunction]
    #[pyo3(name = "counts", signature = (seq, max_neighbors, include_zeros, include_ones))]
    pub fn counts<'py>(
        py: Python<'py>,
        seq: crate::doors::life::Source,
        max_neighbors: usize,
        include_zeros: bool,
        include_ones: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let seq = seq.0;
        let out = mrly_life::counts(seq, max_neighbors, include_zeros, include_ones);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Crops a frame sequence to the centred square bounding every cell ever alive.
    #[pyfunction]
    #[pyo3(name = "crop", signature = (grids))]
    pub fn crop<'py>(py: Python<'py>, grids: Vec<PyCell2d>) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrly_life::crop(&grids);
        ((ok(out)?).into_iter().map(PyCellNd).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order.
    #[pyfunction]
    #[pyo3(name = "cube_orbit", signature = (rule))]
    pub fn cube_orbit<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::cube_orbit(rule);
        (out).into_bound_py_any(py)
    }

    /// Builds the base-2 design mask a code names at an odd side grown to the given Kronecker
    /// level, its centre popped.
    #[pyfunction]
    #[pyo3(name = "design_mask", signature = (dimension, code, number, level))]
    pub fn design_mask<'py>(
        py: Python<'py>,
        dimension: usize,
        code: PyCode,
        number: usize,
        level: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let code = code.0;
        let out = mrly_life::design_mask(dimension, code, number, level);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns the grid's binary Shannon entropy in millibits.
    #[pyfunction]
    #[pyo3(name = "entropy", signature = (grid))]
    pub fn entropy<'py>(py: Python<'py>, grid: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
        let grid = grid.0;
        let out = mrly_life::entropy(&grid);
        (out).into_bound_py_any(py)
    }

    /// Renders grids to white-on-black images at a pixel scale.
    #[pyfunction]
    #[pyo3(name = "frames", signature = (grids, scale))]
    pub fn frames<'py>(
        py: Python<'py>,
        grids: Vec<PyCell2d>,
        scale: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrly_life::frames(&grids, scale);
        ((ok(out)?).into_iter().map(PyImage).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the base-2 plane design a rule's single seed draws, or None when it draws none.
    #[pyfunction]
    #[pyo3(name = "gasket", signature = (rule))]
    pub fn gasket<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::gasket(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`.
    #[pyfunction]
    #[pyo3(name = "genus", signature = (rule))]
    pub fn genus<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::genus(rule);
        (out).into_bound_py_any(py)
    }

    /// Renders a whole run's cumulative-visit heatmap frames with the heat ramp.
    #[pyfunction]
    #[pyo3(name = "heatmap", signature = (grids, scale))]
    pub fn heatmap<'py>(
        py: Python<'py>,
        grids: Vec<PyCell2d>,
        scale: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrly_life::heatmap(&grids, scale);
        ((ok(out)?).into_iter().map(PyImage).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation.
    #[pyfunction]
    #[pyo3(name = "history", signature = (row, rule, steps, wrap))]
    pub fn history<'py>(
        py: Python<'py>,
        row: Vec<u8>,
        rule: u8,
        steps: usize,
        wrap: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::history(&row, rule, steps, wrap);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns Langton's lambda, the popcount over eight.
    #[pyfunction]
    #[pyo3(name = "lambda_", signature = (rule))]
    pub fn lambda_<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::lambda(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension.
    #[pyfunction]
    #[pyo3(name = "lattice_index", signature = (mask))]
    pub fn lattice_index<'py>(py: Python<'py>, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
        let mask = mask.0;
        let out = mrly_life::lattice_index(&mask);
        (out).into_bound_py_any(py)
    }

    /// Returns the offsets a mask's filled sites take from its centre, the centre itself dropped.
    #[pyfunction]
    #[pyo3(name = "mask_offsets", signature = (mask))]
    pub fn mask_offsets<'py>(py: Python<'py>, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
        let mask = mask.0;
        let out = mrly_life::mask_offsets(&mask);
        (out).into_bound_py_any(py)
    }

    /// Builds the 3 by 3 Moore mask, every site on but the center.
    #[pyfunction]
    #[pyo3(name = "moore", signature = ())]
    pub fn moore<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::moore();
        (PyCellNd(ok(out)?)).into_bound_py_any(py)
    }

    /// Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary.
    #[pyfunction]
    #[pyo3(name = "next_grid", signature = (cell, birth, survive, mask, boundary))]
    pub fn next_grid<'py>(
        py: Python<'py>,
        cell: PyCell2d,
        birth: Vec<usize>,
        survive: Vec<usize>,
        mask: PyTensor,
        boundary: PySerde<mrly_life::Boundary>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let cell = cell.0;
        let mask = mask.0;
        let boundary = boundary.0;
        let out = mrly_life::next_grid(&cell, &birth, &survive, &mask, boundary);
        (PyCellNd(ok(out)?)).into_bound_py_any(py)
    }

    /// Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order.
    #[pyfunction]
    #[pyo3(name = "npn_class", signature = (rule))]
    pub fn npn_class<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::npn_class(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone.
    #[pyfunction]
    #[pyo3(name = "outer_totalistic", signature = (rule))]
    pub fn outer_totalistic<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::outer_totalistic(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the count of neighbourhoods a rule sends to one.
    #[pyfunction]
    #[pyo3(name = "popcount", signature = (rule))]
    pub fn popcount<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::popcount(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core.
    #[pyfunction]
    #[pyo3(name = "reversible", signature = (rule))]
    pub fn reversible<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::reversible(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the GF(2) algebraic degree of a rule, minus one for the zero rule.
    #[pyfunction]
    #[pyo3(name = "rule_degree", signature = (rule))]
    pub fn rule_degree<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::rule_degree(rule);
        (out).into_bound_py_any(py)
    }

    /// Returns the design name a rule carries, `bang dim 3, code <rule>`.
    #[pyfunction]
    #[pyo3(name = "rule_name", signature = (rule))]
    pub fn rule_name<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::rule_name(rule);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window.
    #[pyfunction]
    #[pyo3(name = "single_seed", signature = (rule, steps))]
    pub fn single_seed<'py>(
        py: Python<'py>,
        rule: u8,
        steps: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::single_seed(rule, steps);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    /// Advances one row one generation, a constant-0 boundary unless the edges wrap.
    #[pyfunction]
    #[pyo3(name = "step", signature = (row, rule, wrap))]
    pub fn step<'py>(
        py: Python<'py>,
        row: Vec<u8>,
        rule: u8,
        wrap: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::step(&row, rule, wrap);
        (ok(out)?).into_bound_py_any(py)
    }

    /// Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set.
    #[pyfunction]
    #[pyo3(name = "surjective", signature = (rule))]
    pub fn surjective<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::surjective(rule);
        (out).into_bound_py_any(py)
    }

    /// Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there.
    #[pyfunction]
    #[pyo3(name = "tessellate", signature = (grids, min_canvas))]
    pub fn tessellate<'py>(
        py: Python<'py>,
        grids: Vec<PyCell2d>,
        min_canvas: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrly_life::tessellate(&grids, min_canvas);
        ((ok(out)?).into_iter().map(PyCellNd).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    /// Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order.
    #[pyfunction]
    #[pyo3(name = "wolfram_class", signature = (rule))]
    pub fn wolfram_class<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::wolfram_class(rule);
        (out).into_bound_py_any(py)
    }

    pub fn init(
        py: Python<'_>,
        parent: &Bound<'_, PyModule>,
        sys: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let m = PyModule::new(py, "mrly.life")?;
        m.setattr("__doc__", "The engine: a rule over a grid, stepped, recorded, measured and rendered.\n\n- `step`: one generation of a grid under birth and survive counts.\n- `animate`: a seed run until it fixes, loops or times out.\n- `models`: the run config and the recorded life.\n- `recipe`: the recipe grammar and its two-mode run, one name regenerating a run bit for bit.\n- `metrics`: the entropy and churn readings of a run.\n- `crop`: the centred cropping and tiling of a run's frames.\n- `mask`: the design masks a rule reads and the lattice they generate.\n- `source`: the named sources of neighbor counts, and the counts they lay down.\n- `elementary`: the one-line automata and the card of one rule.\n- `render`: the frames and the visit heatmap of a run, as images.\n- `rule`: the canonical name of a rule.\n\nThe doors: `next_grid`, `animate`,\n`entropy`, `churn`,\n`counts`, `frames`,\n`heatmap`, `history` and\n`Rule`.")?;
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
        let names: Vec<&str> = vec![
            "affine",
            "animate",
            "churn",
            "corner_bits",
            "counts",
            "crop",
            "cube_orbit",
            "design_mask",
            "entropy",
            "frames",
            "gasket",
            "genus",
            "heatmap",
            "history",
            "lambda_",
            "lattice_index",
            "mask_offsets",
            "moore",
            "next_grid",
            "npn_class",
            "outer_totalistic",
            "popcount",
            "reversible",
            "rule_degree",
            "rule_name",
            "single_seed",
            "step",
            "surjective",
            "tessellate",
            "wolfram_class",
            "Config",
            "Counts",
            "Life",
            "Rule",
            "Source",
            "Boundary",
            "Fate",
        ];
        m.add("__all__", names)?;
        elementary::init(py, &m, sys)?;
        recipe::init(py, &m, sys)?;
        render::init(py, &m, sys)?;
        source::init(py, &m, sys)?;
        parent.add("life", &m)?;
        sys.set_item("mrly.life._life.life", &m)?;
        Ok(())
    }
}

pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = py
        .import("sys")?
        .getattr("modules")?
        .cast_into::<PyDict>()?;
    life::init(py, root, &sys)?;
    Ok(())
}
