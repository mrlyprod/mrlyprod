#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub mod life {
    use crate::hand::{PyCell2d, PyCellNd, PyCode, PyImage, PySerde, PyTensor, ok};
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    pub mod elementary {
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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
            m.add_function(wrap_pyfunction!(output, &m)?)?;
            let names: Vec<&str> = vec!["output"];
            m.add("__all__", names)?;
            parent.add("elementary", &m)?;
            sys.set_item("mrly.life._life.life.elementary", &m)?;
            Ok(())
        }
    }

    pub mod recipe {
        use crate::hand::{PyCell, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Run", module = "mrly.life.recipe", from_py_object)]
        #[derive(Clone)]
        pub struct Run(pub mrly_life::recipe::Run);

        #[pymethods]
        impl Run {
            #[new]
            #[pyo3(signature = (recipe))]
            pub fn __new__(recipe: PySerde<mrly_life::recipe::Recipe>) -> PyResult<Self> {
                let recipe = recipe.0;
                let out = mrly_life::recipe::Run::new(&recipe);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "background", signature = ())]
            pub fn background<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::background(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "birth", signature = ())]
            pub fn birth<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::birth(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "budget", signature = ())]
            pub fn budget<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::budget(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "cap", signature = ())]
            pub fn cap<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::cap(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "flips", signature = ())]
            pub fn flips<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::flips(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "frame", signature = ())]
            pub fn frame<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::frame(&self.0);
                (PyCell(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "generation", signature = ())]
            pub fn generation<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::generation(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "mask", signature = ())]
            pub fn mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::mask(&self.0);
                (PyTensor(out)).into_bound_py_any(py)
            }
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
            #[pyo3(name = "origin", signature = ())]
            pub fn origin<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::origin(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "population", signature = ())]
            pub fn population<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::population(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "reach", signature = ())]
            pub fn reach<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::reach(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "side", signature = ())]
            pub fn side<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::side(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "step", signature = ())]
            pub fn step<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::step(&mut self.0);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
            #[pyo3(name = "stop", signature = ())]
            pub fn stop<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::stop(&self.0);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
            #[pyo3(name = "survive", signature = ())]
            pub fn survive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Run::survive(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        #[pyclass(name = "Mode", module = "mrly.life.recipe", skip_from_py_object)]
        pub struct Mode;

        #[pymethods]
        impl Mode {
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_life::recipe::Mode::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "grammar", signature = ())]
        pub fn grammar<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::recipe::grammar();
            (PySerde(out)).into_bound_py_any(py)
        }

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

    pub mod render {
        use crate::hand::{PyCell2d, PyImage, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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
            m.add_function(wrap_pyfunction!(frame, &m)?)?;
            let names: Vec<&str> = vec!["frame"];
            m.add("__all__", names)?;
            parent.add("render", &m)?;
            sys.set_item("mrly.life._life.life.render", &m)?;
            Ok(())
        }
    }

    pub mod source {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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
            m.add_function(wrap_pyfunction!(sequence, &m)?)?;
            let names: Vec<&str> = vec!["sequence"];
            m.add("__all__", names)?;
            parent.add("source", &m)?;
            sys.set_item("mrly.life._life.life.source", &m)?;
            Ok(())
        }
    }

    #[pyclass(name = "Config", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Config(pub mrly_life::Config);

    #[pymethods]
    impl Config {
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
        #[pyo3(name = "budget", signature = ())]
        pub fn budget<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Config::budget(&self.0);
            (out).into_bound_py_any(py)
        }
        #[pyo3(name = "counts", signature = ())]
        pub fn counts<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Config::counts(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
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
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    #[pyclass(name = "Counts", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Counts(pub mrly_life::Counts);

    #[pymethods]
    impl Counts {
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
        #[staticmethod]
        #[pyo3(name = "list", signature = (counts))]
        pub fn list<'py>(py: Python<'py>, counts: Vec<usize>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Counts::list(counts);
            (crate::doors::life::Counts(out)).into_bound_py_any(py)
        }
        #[pyo3(name = "values", signature = (budget))]
        pub fn values<'py>(&self, py: Python<'py>, budget: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Counts::values(&self.0, budget);
            (ok(out)?).into_bound_py_any(py)
        }
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    #[pyclass(name = "Life", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Life(pub mrly_life::Life);

    #[pymethods]
    impl Life {
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
        #[pyo3(name = "last", signature = ())]
        pub fn last<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Life::last(&self.0);
            ((out).map(|x| PyCellNd((x).clone()))).into_bound_py_any(py)
        }
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    #[pyclass(name = "Rule", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Rule(pub mrly_life::Rule);

    #[pymethods]
    impl Rule {
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
        #[pyo3(name = "boundary", signature = ())]
        pub fn boundary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Rule::boundary(&self.0);
            (PySerde(out)).into_bound_py_any(py)
        }
        #[pyo3(name = "checked", signature = ())]
        pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::checked(self.0.clone());
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        #[pyo3(name = "config", signature = (mask))]
        pub fn config<'py>(&self, py: Python<'py>, mask: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let out = mrly_life::Rule::config(&self.0, mask);
            (crate::doors::life::Config(out)).into_bound_py_any(py)
        }
        #[staticmethod]
        #[pyo3(name = "from_file", signature = (text))]
        pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::from_file(text);
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        #[staticmethod]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::from_json(text);
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
        #[staticmethod]
        #[pyo3(name = "from_url", signature = (text))]
        pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::from_url(text);
            (crate::doors::life::Rule(ok(out)?)).into_bound_py_any(py)
        }
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
        #[staticmethod]
        #[pyo3(name = "of", signature = (config))]
        pub fn of<'py>(
            py: Python<'py>,
            config: PyRef<'_, crate::doors::life::Config>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Rule::of(&config.0);
            (crate::doors::life::Rule(out)).into_bound_py_any(py)
        }
        #[pyo3(name = "to_file", signature = ())]
        pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_file(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        #[pyo3(name = "to_id", signature = ())]
        pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_id(&self.0);
            (out).into_bound_py_any(py)
        }
        #[pyo3(name = "to_json", signature = ())]
        pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_json(&self.0);
            (out).into_bound_py_any(py)
        }
        #[pyo3(name = "to_prose", signature = ())]
        pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_prose(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        #[pyo3(name = "to_url", signature = ())]
        pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = <mrly_life::Rule as mrly_core::name::Named>::to_url(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    #[pyclass(name = "Source", module = "mrly.life", from_py_object)]
    #[derive(Clone)]
    pub struct Source(pub mrly_life::Source);

    #[pymethods]
    impl Source {
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
        #[pyo3(name = "is_random", signature = ())]
        pub fn is_random<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::is_random(self.0);
            (out).into_bound_py_any(py)
        }
        #[pyo3(name = "name", signature = ())]
        pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::name(self.0);
            (out).into_bound_py_any(py)
        }
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
        #[pyo3(name = "oeis", signature = ())]
        pub fn oeis<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::oeis(self.0);
            (out).into_bound_py_any(py)
        }
        #[staticmethod]
        #[pyo3(name = "parse", signature = (name))]
        pub fn parse<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Source::parse(name);
            (crate::doors::life::Source(ok(out)?)).into_bound_py_any(py)
        }
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
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    #[pyclass(name = "Boundary", module = "mrly.life", skip_from_py_object)]
    pub struct Boundary;

    #[pymethods]
    impl Boundary {
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Boundary::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
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

    #[pyclass(name = "Fate", module = "mrly.life", skip_from_py_object)]
    pub struct Fate;

    #[pymethods]
    impl Fate {
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_life::Fate::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
    }

    #[pyfunction]
    #[pyo3(name = "affine", signature = (rule))]
    pub fn affine<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::affine(rule);
        (out).into_bound_py_any(py)
    }

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

    #[pyfunction]
    #[pyo3(name = "churn", signature = (grids))]
    pub fn churn<'py>(py: Python<'py>, grids: Vec<PyCell2d>) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrly_life::churn(&grids);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "corner_bits", signature = (rule))]
    pub fn corner_bits<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::corner_bits(rule);
        (out).into_bound_py_any(py)
    }

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

    #[pyfunction]
    #[pyo3(name = "crop", signature = (grids))]
    pub fn crop<'py>(py: Python<'py>, grids: Vec<PyCell2d>) -> PyResult<Bound<'py, PyAny>> {
        let grids = grids.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let out = mrly_life::crop(&grids);
        ((ok(out)?).into_iter().map(PyCellNd).collect::<Vec<_>>()).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "cube_orbit", signature = (rule))]
    pub fn cube_orbit<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::cube_orbit(rule);
        (out).into_bound_py_any(py)
    }

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

    #[pyfunction]
    #[pyo3(name = "entropy", signature = (grid))]
    pub fn entropy<'py>(py: Python<'py>, grid: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
        let grid = grid.0;
        let out = mrly_life::entropy(&grid);
        (out).into_bound_py_any(py)
    }

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

    #[pyfunction]
    #[pyo3(name = "gasket", signature = (rule))]
    pub fn gasket<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::gasket(rule);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "genus", signature = (rule))]
    pub fn genus<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::genus(rule);
        (out).into_bound_py_any(py)
    }

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

    #[pyfunction]
    #[pyo3(name = "lambda_", signature = (rule))]
    pub fn lambda_<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::lambda(rule);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "lattice_index", signature = (mask))]
    pub fn lattice_index<'py>(py: Python<'py>, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
        let mask = mask.0;
        let out = mrly_life::lattice_index(&mask);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "mask_offsets", signature = (mask))]
    pub fn mask_offsets<'py>(py: Python<'py>, mask: PyTensor) -> PyResult<Bound<'py, PyAny>> {
        let mask = mask.0;
        let out = mrly_life::mask_offsets(&mask);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "moore", signature = ())]
    pub fn moore<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::moore();
        (PyCellNd(ok(out)?)).into_bound_py_any(py)
    }

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

    #[pyfunction]
    #[pyo3(name = "npn_class", signature = (rule))]
    pub fn npn_class<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::npn_class(rule);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "outer_totalistic", signature = (rule))]
    pub fn outer_totalistic<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::outer_totalistic(rule);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "popcount", signature = (rule))]
    pub fn popcount<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::popcount(rule);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "reversible", signature = (rule))]
    pub fn reversible<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::reversible(rule);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "rule_degree", signature = (rule))]
    pub fn rule_degree<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::rule_degree(rule);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "rule_name", signature = (rule))]
    pub fn rule_name<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::rule_name(rule);
        (ok(out)?).into_bound_py_any(py)
    }

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

    #[pyfunction]
    #[pyo3(name = "surjective", signature = (rule))]
    pub fn surjective<'py>(py: Python<'py>, rule: u8) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_life::surjective(rule);
        (out).into_bound_py_any(py)
    }

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
