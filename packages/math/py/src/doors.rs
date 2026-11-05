#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub mod math {
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    pub mod arcs {
        use crate::hand::{PyCode, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "curves", signature = (code, base, level))]
        pub fn curves<'py>(
            py: Python<'py>,
            code: PyCode,
            base: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::arcs::curves(code, base, level);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "draw", signature = (code, base, level))]
        pub fn draw<'py>(
            py: Python<'py>,
            code: PyCode,
            base: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::arcs::draw(code, base, level);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "law", signature = (code, base, level))]
        pub fn law<'py>(
            py: Python<'py>,
            code: PyCode,
            base: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::arcs::law(code, base, level);
            ((ok(out)?).map(PySerde)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "trace", signature = (side, on))]
        pub fn trace<'py>(
            py: Python<'py>,
            side: usize,
            on: Vec<bool>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arcs::trace(side, &on);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.arcs")?;
            m.add_function(wrap_pyfunction!(curves, &m)?)?;
            m.add_function(wrap_pyfunction!(draw, &m)?)?;
            m.add_function(wrap_pyfunction!(law, &m)?)?;
            m.add_function(wrap_pyfunction!(trace, &m)?)?;
            let names: Vec<&str> = vec!["curves", "draw", "law", "trace"];
            m.add("__all__", names)?;
            parent.add("arcs", &m)?;
            sys.set_item("mrly.math._math.math.arcs", &m)?;
            Ok(())
        }
    }

    pub mod arrays {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "essential", signature = (sensors))]
        pub fn essential<'py>(py: Python<'py>, sensors: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::essential(&sensors);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fractal", signature = (generator, base, level))]
        pub fn fractal<'py>(
            py: Python<'py>,
            generator: Vec<u64>,
            base: u64,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::fractal(&generator, base, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "holes", signature = (sensors))]
        pub fn holes<'py>(py: Python<'py>, sensors: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::holes(&sensors);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "law", signature = (generator, level))]
        pub fn law<'py>(
            py: Python<'py>,
            generator: Vec<u64>,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::law(&generator, level);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "lost", signature = (sensors, sensor))]
        pub fn lost<'py>(
            py: Python<'py>,
            sensors: Vec<u64>,
            sensor: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::lost(&sensors, sensor);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "paired", signature = (sensors))]
        pub fn paired<'py>(py: Python<'py>, sensors: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::paired(&sensors);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "weights", signature = (sensors))]
        pub fn weights<'py>(py: Python<'py>, sensors: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::weights(&sensors);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.arrays")?;
            m.add_function(wrap_pyfunction!(essential, &m)?)?;
            m.add_function(wrap_pyfunction!(fractal, &m)?)?;
            m.add_function(wrap_pyfunction!(holes, &m)?)?;
            m.add_function(wrap_pyfunction!(law, &m)?)?;
            m.add_function(wrap_pyfunction!(lost, &m)?)?;
            m.add_function(wrap_pyfunction!(paired, &m)?)?;
            m.add_function(wrap_pyfunction!(weights, &m)?)?;
            m.add("MOST", mrly_math::arrays::MOST)?;
            let names: Vec<&str> = vec![
                "essential",
                "fractal",
                "holes",
                "law",
                "lost",
                "paired",
                "weights",
                "MOST",
            ];
            m.add("__all__", names)?;
            parent.add("arrays", &m)?;
            sys.set_item("mrly.math._math.math.arrays", &m)?;
            Ok(())
        }
    }

    pub mod atoms {
        use crate::hand::PyTensor;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "carpet_2d", signature = (n))]
        pub fn carpet_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::carpet_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "carpet_3d", signature = (n))]
        pub fn carpet_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::carpet_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "carpet_nd", signature = (n, rank))]
        pub fn carpet_nd<'py>(
            py: Python<'py>,
            n: usize,
            rank: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::carpet_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "dust_2d", signature = (n))]
        pub fn dust_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::dust_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "dust_3d", signature = (n))]
        pub fn dust_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::dust_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "dust_nd", signature = (n, rank))]
        pub fn dust_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::dust_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "hline_2d", signature = (n))]
        pub fn hline_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::hline_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "htree_2d", signature = (n))]
        pub fn htree_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::htree_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "line_nd", signature = (n, rank, axis))]
        pub fn line_nd<'py>(
            py: Python<'py>,
            n: usize,
            rank: usize,
            axis: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::line_nd(n, rank, axis);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "net_2d", signature = (n))]
        pub fn net_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::net_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "net_3d", signature = (n))]
        pub fn net_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::net_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "net_nd", signature = (n, rank))]
        pub fn net_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::net_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "noise_2d", signature = (n, density, rng))]
        pub fn noise_2d<'py>(
            py: Python<'py>,
            n: usize,
            density: f64,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_math::atoms::noise_2d(n, density, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "noise_3d", signature = (n, density, rng))]
        pub fn noise_3d<'py>(
            py: Python<'py>,
            n: usize,
            density: f64,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_math::atoms::noise_3d(n, density, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ones_2d", signature = (n))]
        pub fn ones_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::ones_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ones_3d", signature = (n))]
        pub fn ones_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::ones_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "point_2d", signature = (n))]
        pub fn point_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::point_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "point_3d", signature = (n))]
        pub fn point_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::point_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "point_nd", signature = (n, rank))]
        pub fn point_nd<'py>(
            py: Python<'py>,
            n: usize,
            rank: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::point_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "star_2d", signature = (n))]
        pub fn star_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::star_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "star_3d", signature = (n))]
        pub fn star_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::star_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "star_nd", signature = (n, rank))]
        pub fn star_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::star_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tree_nd", signature = (n, rank, axis))]
        pub fn tree_nd<'py>(
            py: Python<'py>,
            n: usize,
            rank: usize,
            axis: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::tree_nd(n, rank, axis);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "vline_2d", signature = (n))]
        pub fn vline_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::vline_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "void_2d", signature = (n))]
        pub fn void_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::void_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "void_3d", signature = (n))]
        pub fn void_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::void_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "void_nd", signature = (n, rank))]
        pub fn void_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::void_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "vtree_2d", signature = (n))]
        pub fn vtree_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::vtree_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "xline_3d", signature = (n))]
        pub fn xline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::xline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "xtree_3d", signature = (n))]
        pub fn xtree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::xtree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "yline_3d", signature = (n))]
        pub fn yline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::yline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ytree_3d", signature = (n))]
        pub fn ytree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::ytree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zeros_2d", signature = (n))]
        pub fn zeros_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::zeros_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zeros_3d", signature = (n))]
        pub fn zeros_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::zeros_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zline_3d", signature = (n))]
        pub fn zline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::zline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ztree_3d", signature = (n))]
        pub fn ztree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::ztree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.atoms")?;
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
            let names: Vec<&str> = vec![
                "carpet_2d",
                "carpet_3d",
                "carpet_nd",
                "dust_2d",
                "dust_3d",
                "dust_nd",
                "hline_2d",
                "htree_2d",
                "line_nd",
                "net_2d",
                "net_3d",
                "net_nd",
                "noise_2d",
                "noise_3d",
                "ones_2d",
                "ones_3d",
                "point_2d",
                "point_3d",
                "point_nd",
                "star_2d",
                "star_3d",
                "star_nd",
                "tree_nd",
                "vline_2d",
                "void_2d",
                "void_3d",
                "void_nd",
                "vtree_2d",
                "xline_3d",
                "xtree_3d",
                "yline_3d",
                "ytree_3d",
                "zeros_2d",
                "zeros_3d",
                "zline_3d",
                "ztree_3d",
            ];
            m.add("__all__", names)?;
            parent.add("atoms", &m)?;
            sys.set_item("mrly.math._math.math.atoms", &m)?;
            Ok(())
        }
    }

    pub mod automaton {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Automaton", module = "mrly.math.automaton", from_py_object)]
        #[derive(Clone)]
        pub struct Automaton(pub mrly_math::automaton::Automaton);

        #[pymethods]
        impl Automaton {
            #[new]
            #[pyo3(signature = (rule))]
            pub fn __new__(rule: PyRef<'_, crate::doors::math::memory::Rule>) -> PyResult<Self> {
                let out = mrly_math::automaton::Automaton::new(&rule.0);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "abscissa", signature = ())]
            pub fn abscissa<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::abscissa(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::base(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "cofactor", signature = (s, tolerance))]
            pub fn cofactor<'py>(
                &self,
                py: Python<'py>,
                s: PySerde<mrly_num::zeta::Complex>,
                tolerance: f64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let s = s.0;
                let out = mrly_math::automaton::Automaton::cofactor(&self.0, s, tolerance);
                ({
                    let t = ok(out)?;
                    (PySerde(t.0), t.1)
                })
                .into_bound_py_any(py)
            }
            #[pyo3(name = "denominator", signature = ())]
            pub fn denominator<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::denominator(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "matrix", signature = ())]
            pub fn matrix<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::matrix(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (rule))]
            pub fn new_<'py>(
                py: Python<'py>,
                rule: PyRef<'_, crate::doors::math::memory::Rule>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::new(&rule.0);
                (crate::doors::math::automaton::Automaton(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "peel", signature = ())]
            pub fn peel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::peel(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "period", signature = ())]
            pub fn period<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::period(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "perron", signature = ())]
            pub fn perron<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::perron(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "residue", signature = (w0, tolerance))]
            pub fn residue<'py>(
                &self,
                py: Python<'py>,
                w0: PySerde<mrly_num::zeta::Complex>,
                tolerance: f64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let w0 = w0.0;
                let out = mrly_math::automaton::Automaton::residue(&self.0, w0, tolerance);
                ({
                    let t = ok(out)?;
                    (PySerde(t.0), t.1)
                })
                .into_bound_py_any(py)
            }
            #[pyo3(name = "rule", signature = ())]
            pub fn rule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::rule(&self.0);
                (crate::doors::math::memory::Rule(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "states", signature = ())]
            pub fn states<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::states(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "with_peel", signature = (rule, peel))]
            pub fn with_peel<'py>(
                py: Python<'py>,
                rule: PyRef<'_, crate::doors::math::memory::Rule>,
                peel: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::with_peel(&rule.0, peel);
                (crate::doors::math::automaton::Automaton(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "zeta", signature = (s, tolerance))]
            pub fn zeta<'py>(
                &self,
                py: Python<'py>,
                s: PySerde<mrly_num::zeta::Complex>,
                tolerance: f64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let s = s.0;
                let out = mrly_math::automaton::Automaton::zeta(&self.0, s, tolerance);
                ({
                    let t = ok(out)?;
                    (PySerde(t.0), t.1)
                })
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

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.automaton")?;
            m.add_class::<Automaton>()?;
            m.add("ROUNDING", mrly_math::automaton::ROUNDING)?;
            let names: Vec<&str> = vec!["Automaton", "ROUNDING"];
            m.add("__all__", names)?;
            parent.add("automaton", &m)?;
            sys.set_item("mrly.math._math.math.automaton", &m)?;
            Ok(())
        }
    }

    pub mod bang {
        use crate::hand::{PyCode, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        pub mod baseq {
            use crate::hand::{PyCode, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "axis_maps", signature = (base))]
            pub fn axis_maps<'py>(py: Python<'py>, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::axis_maps(base);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "bracelets", signature = (max_base))]
            pub fn bracelets<'py>(py: Python<'py>, max_base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::bracelets(max_base);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "canonical", signature = (group, code))]
            pub fn canonical<'py>(
                py: Python<'py>,
                group: Vec<Vec<usize>>,
                code: PyCode,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::baseq::canonical(&group, code);
                (PyCode(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "carry", signature = (element, code))]
            pub fn carry<'py>(
                py: Python<'py>,
                element: Vec<usize>,
                code: PyCode,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::baseq::carry(&element, code);
                (PyCode(out)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "class_sequence", signature = (max_dimension))]
            pub fn class_sequence<'py>(
                py: Python<'py>,
                max_dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::class_sequence(max_dimension);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "classes", signature = (dimension))]
            pub fn classes<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::classes(dimension);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "distinct_designs", signature = (base, dimension))]
            pub fn distinct_designs<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::distinct_designs(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "even_fill_is_balanced", signature = (number, dimension, popcount))]
            pub fn even_fill_is_balanced<'py>(
                py: Python<'py>,
                number: usize,
                dimension: usize,
                popcount: u128,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    mrly_math::bang::baseq::even_fill_is_balanced(number, dimension, popcount);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "fill_from_corners", signature = (filled, number, dimension))]
            pub fn fill_from_corners<'py>(
                py: Python<'py>,
                filled: Vec<Vec<u8>>,
                number: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::fill_from_corners(&filled, number, dimension);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "group", signature = (base, dimension))]
            pub fn group<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::group(base, dimension);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "group_order", signature = (base, dimension))]
            pub fn group_order<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::group_order(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "orbit", signature = (group, code))]
            pub fn orbit<'py>(
                py: Python<'py>,
                group: Vec<Vec<usize>>,
                code: PyCode,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::baseq::orbit(&group, code);
                ((out).into_iter().map(PyCode).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "predicted_group_order", signature = (base, dimension))]
            pub fn predicted_group_order<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::predicted_group_order(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "representatives", signature = (base, dimension))]
            pub fn representatives<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::representatives(base, dimension);
                ((ok(out)?)
                    .into_iter()
                    .map(|x| {
                        let t = x;
                        (PyCode(t.0), t.1)
                    })
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "sequence", signature = (base, max_dimension))]
            pub fn sequence<'py>(
                py: Python<'py>,
                base: usize,
                max_dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::sequence(base, max_dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "total_designs", signature = (base, dimension))]
            pub fn total_designs<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::total_designs(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.bang.baseq")?;
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
                m.add("WALK_LIMIT", mrly_math::bang::baseq::WALK_LIMIT)?;
                let names: Vec<&str> = vec![
                    "axis_maps",
                    "bracelets",
                    "canonical",
                    "carry",
                    "class_sequence",
                    "classes",
                    "distinct_designs",
                    "even_fill_is_balanced",
                    "fill_from_corners",
                    "group",
                    "group_order",
                    "orbit",
                    "predicted_group_order",
                    "representatives",
                    "sequence",
                    "total_designs",
                    "WALK_LIMIT",
                ];
                m.add("__all__", names)?;
                parent.add("baseq", &m)?;
                sys.set_item("mrly.math._math.math.bang.baseq", &m)?;
                Ok(())
            }
        }

        pub mod catalog {
            use crate::hand::PySerde;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyclass(
                name = "Design",
                module = "mrly.math.bang.catalog",
                skip_from_py_object
            )]
            pub struct Design;

            #[pymethods]
            impl Design {
                #[staticmethod]
                #[pyo3(name = "all", signature = ())]
                pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::bang::catalog::Design::all();
                    ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
                }
            }

            #[pyfunction]
            #[pyo3(name = "antis", signature = (dimension))]
            pub fn antis<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::catalog::antis(dimension);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "classics", signature = (dimension))]
            pub fn classics<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::catalog::classics(dimension);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.bang.catalog")?;
                m.add_class::<Design>()?;
                m.add_function(wrap_pyfunction!(antis, &m)?)?;
                m.add_function(wrap_pyfunction!(classics, &m)?)?;
                m.add(
                    "ANTIS_2D",
                    (mrly_math::bang::catalog::ANTIS_2D)
                        .into_iter()
                        .map(PySerde)
                        .collect::<Vec<_>>(),
                )?;
                m.add(
                    "ANTIS_3D",
                    (mrly_math::bang::catalog::ANTIS_3D)
                        .into_iter()
                        .map(PySerde)
                        .collect::<Vec<_>>(),
                )?;
                m.add(
                    "CLASSICS_2D",
                    (mrly_math::bang::catalog::CLASSICS_2D)
                        .into_iter()
                        .map(PySerde)
                        .collect::<Vec<_>>(),
                )?;
                m.add(
                    "CLASSICS_3D",
                    (mrly_math::bang::catalog::CLASSICS_3D)
                        .into_iter()
                        .map(PySerde)
                        .collect::<Vec<_>>(),
                )?;
                let names: Vec<&str> = vec![
                    "antis",
                    "classics",
                    "Design",
                    "ANTIS_2D",
                    "ANTIS_3D",
                    "CLASSICS_2D",
                    "CLASSICS_3D",
                ];
                m.add("__all__", names)?;
                parent.add("catalog", &m)?;
                sys.set_item("mrly.math._math.math.bang.catalog", &m)?;
                Ok(())
            }
        }

        pub mod code {
            use crate::hand::PyCode;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "get", signature = (code))]
            pub fn get<'py>(py: Python<'py>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::Code::get(code);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.bang.code")?;
                m.add_function(wrap_pyfunction!(get, &m)?)?;
                let names: Vec<&str> = vec!["get"];
                m.add("__all__", names)?;
                parent.add("code", &m)?;
                sys.set_item("mrly.math._math.math.bang.code", &m)?;
                Ok(())
            }
        }

        pub mod factory {
            use crate::hand::{PyCode, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "create", signature = (code, number, dimension, base, level))]
            pub fn create<'py>(
                py: Python<'py>,
                code: PyCode,
                number: usize,
                dimension: usize,
                base: usize,
                level: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::factory::create(code, number, dimension, base, level);
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "create_from_corners", signature = (filled, number, dimension, base, level))]
            pub fn create_from_corners<'py>(
                py: Python<'py>,
                filled: Vec<Vec<u8>>,
                number: usize,
                dimension: usize,
                base: usize,
                level: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::factory::create_from_corners(
                    &filled, number, dimension, base, level,
                );
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "create_named", signature = (spec, number, level))]
            pub fn create_named<'py>(
                py: Python<'py>,
                spec: &str,
                number: usize,
                level: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::factory::create_named(spec, number, level);
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "residue_corners", signature = (dimension, base))]
            pub fn residue_corners<'py>(
                py: Python<'py>,
                dimension: usize,
                base: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::factory::residue_corners(dimension, base);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "total_codes", signature = (dimension, base))]
            pub fn total_codes<'py>(
                py: Python<'py>,
                dimension: usize,
                base: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::factory::total_codes(dimension, base);
                (PyCode(ok(out)?)).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.bang.factory")?;
                m.add_function(wrap_pyfunction!(create, &m)?)?;
                m.add_function(wrap_pyfunction!(create_from_corners, &m)?)?;
                m.add_function(wrap_pyfunction!(create_named, &m)?)?;
                m.add_function(wrap_pyfunction!(residue_corners, &m)?)?;
                m.add_function(wrap_pyfunction!(total_codes, &m)?)?;
                let names: Vec<&str> = vec![
                    "create",
                    "create_from_corners",
                    "create_named",
                    "residue_corners",
                    "total_codes",
                ];
                m.add("__all__", names)?;
                parent.add("factory", &m)?;
                sys.set_item("mrly.math._math.math.bang.factory", &m)?;
                Ok(())
            }
        }

        pub mod universe {
            use crate::hand::PyCode;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "anf", signature = (code, dimension))]
            pub fn anf<'py>(
                py: Python<'py>,
                code: PyCode,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::universe::anf(code, dimension);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "anf_string", signature = (code, dimension))]
            pub fn anf_string<'py>(
                py: Python<'py>,
                code: PyCode,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::universe::anf_string(code, dimension);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "apply", signature = (element, corner))]
            pub fn apply<'py>(
                py: Python<'py>,
                element: (Vec<usize>, Vec<u8>),
                corner: Vec<u8>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::universe::apply(&element, &corner);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "corner_index", signature = (corner))]
            pub fn corner_index<'py>(
                py: Python<'py>,
                corner: Vec<u8>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::universe::corner_index(&corner);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "degree", signature = (code, dimension))]
            pub fn degree<'py>(
                py: Python<'py>,
                code: PyCode,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::universe::degree(code, dimension);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "orbit", signature = (code, dimension))]
            pub fn orbit<'py>(
                py: Python<'py>,
                code: PyCode,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::universe::orbit(code, dimension);
                ((out).into_iter().map(PyCode).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "permutations", signature = (n))]
            pub fn permutations<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::universe::permutations(n);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.bang.universe")?;
                m.add_function(wrap_pyfunction!(anf, &m)?)?;
                m.add_function(wrap_pyfunction!(anf_string, &m)?)?;
                m.add_function(wrap_pyfunction!(apply, &m)?)?;
                m.add_function(wrap_pyfunction!(corner_index, &m)?)?;
                m.add_function(wrap_pyfunction!(degree, &m)?)?;
                m.add_function(wrap_pyfunction!(orbit, &m)?)?;
                m.add_function(wrap_pyfunction!(permutations, &m)?)?;
                let names: Vec<&str> = vec![
                    "anf",
                    "anf_string",
                    "apply",
                    "corner_index",
                    "degree",
                    "orbit",
                    "permutations",
                ];
                m.add("__all__", names)?;
                parent.add("universe", &m)?;
                sys.set_item("mrly.math._math.math.bang.universe", &m)?;
                Ok(())
            }
        }

        pub mod word {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyclass(name = "Schedule", module = "mrly.math.bang.word", skip_from_py_object)]
            pub struct Schedule;

            #[pymethods]
            impl Schedule {
                #[staticmethod]
                #[pyo3(name = "all", signature = ())]
                pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::bang::word::Schedule::all();
                    ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
                }
                #[staticmethod]
                #[pyo3(name = "frequencies", signature = (schedule))]
                pub fn frequencies<'py>(
                    py: Python<'py>,
                    schedule: PySerde<mrly_math::bang::word::Schedule>,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let schedule = schedule.0;
                    let out = mrly_math::bang::word::Schedule::frequencies(schedule);
                    (out).into_bound_py_any(py)
                }
                #[staticmethod]
                #[pyo3(name = "place", signature = (schedule, index))]
                pub fn place<'py>(
                    py: Python<'py>,
                    schedule: PySerde<mrly_math::bang::word::Schedule>,
                    index: usize,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let schedule = schedule.0;
                    let out = mrly_math::bang::word::Schedule::place(schedule, index);
                    (out).into_bound_py_any(py)
                }
            }

            #[pyfunction]
            #[pyo3(name = "components", signature = (layers))]
            pub fn components<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::components(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "constant_functional", signature = (layers))]
            pub fn constant_functional<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::constant_functional(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "dimension", signature = (layers))]
            pub fn dimension<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::dimension(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "fill", signature = (layers))]
            pub fn fill<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::fill(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "fills", signature = (layers))]
            pub fn fills<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::fills(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "letter", signature = (layer))]
            pub fn letter<'py>(
                py: Python<'py>,
                layer: PySerde<mrly_math::bang::MagicLayer>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layer = layer.0;
                let out = mrly_math::bang::word::letter(&layer);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "native", signature = (layers))]
            pub fn native<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::native(&layers);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "period", signature = (layers))]
            pub fn period<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::period(&layers);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "prefixes", signature = (layers))]
            pub fn prefixes<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::prefixes(&layers);
                ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "rates", signature = (layers))]
            pub fn rates<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::rates(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "side", signature = (layers))]
            pub fn side<'py>(
                py: Python<'py>,
                layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
                let out = mrly_math::bang::word::side(&layers);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "spell", signature = (schedule, pair, length))]
            pub fn spell<'py>(
                py: Python<'py>,
                schedule: PySerde<mrly_math::bang::word::Schedule>,
                pair: (
                    PySerde<mrly_math::bang::MagicLayer>,
                    PySerde<mrly_math::bang::MagicLayer>,
                ),
                length: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let schedule = schedule.0;
                let pair = {
                    let t = pair;
                    (t.0.0, t.1.0)
                };
                let out = mrly_math::bang::word::spell(schedule, pair, length);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "staircase", signature = (depth))]
            pub fn staircase<'py>(py: Python<'py>, depth: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::word::staircase(depth);
                ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "thue_morse", signature = (index))]
            pub fn thue_morse<'py>(py: Python<'py>, index: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::word::thue_morse(index);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.bang.word")?;
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
                let names: Vec<&str> = vec![
                    "components",
                    "constant_functional",
                    "dimension",
                    "fill",
                    "fills",
                    "letter",
                    "native",
                    "period",
                    "prefixes",
                    "rates",
                    "side",
                    "spell",
                    "staircase",
                    "thue_morse",
                    "Schedule",
                ];
                m.add("__all__", names)?;
                parent.add("word", &m)?;
                sys.set_item("mrly.math._math.math.bang.word", &m)?;
                Ok(())
            }
        }

        #[pyclass(name = "Design", module = "mrly.math.bang", from_py_object)]
        #[derive(Clone)]
        pub struct Design(pub mrly_math::bang::Design);

        #[pymethods]
        impl Design {
            #[getter]
            #[pyo3(name = "i")]
            pub fn i<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.i;
                (PyCode(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "i")]
            pub fn set_i(&mut self, value: PyCode) -> PyResult<()> {
                let value = value.0;
                self.0.i = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dimension")]
            pub fn set_dimension(&mut self, value: usize) -> PyResult<()> {
                self.0.dimension = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "canonical")]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.canonical;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "canonical")]
            pub fn set_canonical(&mut self, value: bool) -> PyResult<()> {
                self.0.canonical = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "class_rep")]
            pub fn class_rep<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.class_rep;
                (PyCode(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "class_rep")]
            pub fn set_class_rep(&mut self, value: PyCode) -> PyResult<()> {
                let value = value.0;
                self.0.class_rep = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "orbit_size")]
            pub fn orbit_size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.orbit_size;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "orbit_size")]
            pub fn set_orbit_size(&mut self, value: usize) -> PyResult<()> {
                self.0.orbit_size = value;
                Ok(())
            }
            #[pyo3(name = "anf", signature = ())]
            pub fn anf<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::anf(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "degree", signature = ())]
            pub fn degree<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::degree(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "name", signature = ())]
            pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::name(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "rule", signature = ())]
            pub fn rule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::rule(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Universe", module = "mrly.math.bang", skip_from_py_object)]
        pub struct Universe(pub mrly_math::bang::Universe);

        #[pymethods]
        impl Universe {
            #[new]
            #[pyo3(signature = (dimension))]
            pub fn __new__(dimension: usize) -> PyResult<Self> {
                let out = mrly_math::bang::Universe::new(dimension);
                Ok(Self(ok(out)?))
            }
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dimension")]
            pub fn set_dimension(&mut self, value: usize) -> PyResult<()> {
                self.0.dimension = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "total")]
            pub fn total<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.total;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "total")]
            pub fn set_total(&mut self, value: usize) -> PyResult<()> {
                self.0.total = value;
                Ok(())
            }
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::all(&self.0);
                ((out)
                    .into_iter()
                    .map(crate::doors::math::bang::Design)
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }
            #[pyo3(name = "canonical", signature = ())]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::canonical(&self.0);
                ((out)
                    .into_iter()
                    .map(crate::doors::math::bang::Design)
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }
            #[pyo3(name = "design", signature = (code))]
            pub fn design<'py>(
                &self,
                py: Python<'py>,
                code: PyCode,
            ) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::bang::Universe::design(&self.0, code);
                (crate::doors::math::bang::Design(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "distinct", signature = ())]
            pub fn distinct<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::distinct(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (dimension))]
            pub fn new_<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::new(dimension);
                (crate::doors::math::bang::Universe(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "MagicLayer", module = "mrly.math.bang", skip_from_py_object)]
        pub struct MagicLayer;

        #[pymethods]
        impl MagicLayer {
            #[staticmethod]
            #[pyo3(name = "new", signature = (design, number))]
            pub fn new_<'py>(
                py: Python<'py>,
                design: crate::doors::math::name::Bang,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let design = design.0;
                let out = mrly_math::bang::MagicLayer::new(design, number);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "bang", signature = (dimension))]
        pub fn bang<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::bang(dimension);
            (crate::doors::math::bang::Universe(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "code_to_corners", signature = (code, dimension, base))]
        pub fn code_to_corners<'py>(
            py: Python<'py>,
            code: PyCode,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::bang::code_to_corners(code, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "corners", signature = (dimension))]
        pub fn corners<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::corners(dimension);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "corners_to_code", signature = (filled, dimension, base))]
        pub fn corners_to_code<'py>(
            py: Python<'py>,
            filled: Vec<Vec<u8>>,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::corners_to_code(&filled, dimension, base);
            (PyCode(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "level_cap", signature = (number, dimension, budget))]
        pub fn level_cap<'py>(
            py: Python<'py>,
            number: usize,
            dimension: usize,
            budget: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::level_cap(number, dimension, budget);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "levels_code", signature = (dimension, base, levels))]
        pub fn levels_code<'py>(
            py: Python<'py>,
            dimension: usize,
            base: usize,
            levels: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::levels_code(dimension, base, &levels);
            (PyCode(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "magic", signature = (layers))]
        pub fn magic<'py>(
            py: Python<'py>,
            layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::bang::magic(&layers);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "magic_named", signature = (layers))]
        pub fn magic_named<'py>(
            py: Python<'py>,
            layers: Vec<(String, usize)>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers
                .iter()
                .map(|y| (y.0.as_str(), y.1))
                .collect::<Vec<_>>();
            let out = mrly_math::bang::magic_named(&layers);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "random_code", signature = (dimension, base, rng, min_fills))]
        pub fn random_code<'py>(
            py: Python<'py>,
            dimension: usize,
            base: usize,
            rng: &Bound<'_, PyAny>,
            min_fills: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_math::bang::random_code(dimension, base, &mut rng_owned, min_fills);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (PyCode(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "sources", signature = (catalog, dimension))]
        pub fn sources<'py>(
            py: Python<'py>,
            catalog: PySerde<mrly_math::bang::catalog::Catalog>,
            dimension: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let catalog = catalog.0;
            let out = mrly_math::bang::sources(&catalog, dimension);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "symmetries", signature = (dimension))]
        pub fn symmetries<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::symmetries(dimension);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "total_exposure", signature = (code, dimension))]
        pub fn total_exposure<'py>(
            py: Python<'py>,
            code: PyCode,
            dimension: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::bang::total_exposure(code, dimension);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "touches_every_corner", signature = (code, dimension))]
        pub fn touches_every_corner<'py>(
            py: Python<'py>,
            code: PyCode,
            dimension: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::bang::touches_every_corner(code, dimension);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "universe_codes", signature = (dimension))]
        pub fn universe_codes<'py>(
            py: Python<'py>,
            dimension: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::universe_codes(dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.bang")?;
            m.add_class::<Design>()?;
            m.add_class::<Universe>()?;
            m.add_class::<MagicLayer>()?;
            m.add_function(wrap_pyfunction!(bang, &m)?)?;
            m.add_function(wrap_pyfunction!(code_to_corners, &m)?)?;
            m.add_function(wrap_pyfunction!(corners, &m)?)?;
            m.add_function(wrap_pyfunction!(corners_to_code, &m)?)?;
            m.add_function(wrap_pyfunction!(level_cap, &m)?)?;
            m.add_function(wrap_pyfunction!(levels_code, &m)?)?;
            m.add_function(wrap_pyfunction!(magic, &m)?)?;
            m.add_function(wrap_pyfunction!(magic_named, &m)?)?;
            m.add_function(wrap_pyfunction!(random_code, &m)?)?;
            m.add_function(wrap_pyfunction!(sources, &m)?)?;
            m.add_function(wrap_pyfunction!(symmetries, &m)?)?;
            m.add_function(wrap_pyfunction!(total_exposure, &m)?)?;
            m.add_function(wrap_pyfunction!(touches_every_corner, &m)?)?;
            m.add_function(wrap_pyfunction!(universe_codes, &m)?)?;
            let names: Vec<&str> = vec![
                "bang",
                "code_to_corners",
                "corners",
                "corners_to_code",
                "level_cap",
                "levels_code",
                "magic",
                "magic_named",
                "random_code",
                "sources",
                "symmetries",
                "total_exposure",
                "touches_every_corner",
                "universe_codes",
                "Design",
                "Universe",
                "MagicLayer",
            ];
            m.add("__all__", names)?;
            baseq::init(py, &m, sys)?;
            catalog::init(py, &m, sys)?;
            code::init(py, &m, sys)?;
            factory::init(py, &m, sys)?;
            universe::init(py, &m, sys)?;
            word::init(py, &m, sys)?;
            parent.add("bang", &m)?;
            sys.set_item("mrly.math._math.math.bang", &m)?;
            Ok(())
        }
    }

    pub mod cell {
        use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyColor, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::exceptions::PyValueError;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        pub mod census {
            use crate::hand::{PyCell2d, PyCell3d, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "edges", signature = (cell))]
            pub fn edges<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::edges::<2>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::edges::<3>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "edges wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "exposure", signature = (cell))]
            pub fn exposure<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::exposure::<2>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::exposure::<3>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "exposure wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "fills", signature = (cell))]
            pub fn fills<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::fills::<2>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::fills::<3>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "fills wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "vertices", signature = (cell))]
            pub fn vertices<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::vertices::<2>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::vertices::<3>(&cell);
                        (ok(out)?).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "vertices wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "voids", signature = (cell))]
            pub fn voids<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::voids::<2>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::census::voids::<3>(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "voids wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.cell.census")?;
                m.add_function(wrap_pyfunction!(edges, &m)?)?;
                m.add_function(wrap_pyfunction!(exposure, &m)?)?;
                m.add_function(wrap_pyfunction!(fills, &m)?)?;
                m.add_function(wrap_pyfunction!(vertices, &m)?)?;
                m.add_function(wrap_pyfunction!(voids, &m)?)?;
                let names: Vec<&str> = vec!["edges", "exposure", "fills", "vertices", "voids"];
                m.add("__all__", names)?;
                parent.add("census", &m)?;
                sys.set_item("mrly.math._math.math.cell.census", &m)?;
                Ok(())
            }
        }

        pub mod geometry {
            use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "merge_reps", signature = (cells, reps))]
            pub fn merge_reps<'py>(
                py: Python<'py>,
                cells: &Bound<'_, PyAny>,
                reps: Vec<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cells)?;
                match dim {
                    2 => {
                        let cells = cells.extract::<Vec<PyCell2d>>()?;
                        let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                        let out = mrly_math::cell::geometry::merge_reps::<2>(&cells, &reps);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cells = cells.extract::<Vec<PyCell3d>>()?;
                        let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                        let out = mrly_math::cell::geometry::merge_reps::<3>(&cells, &reps);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "merge_reps wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "perforate", signature = (mask, cell, value))]
            pub fn perforate<'py>(
                py: Python<'py>,
                mask: PyTensor,
                cell: &Bound<'_, PyAny>,
                value: u8,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let mask = mask.0;
                        let cell = cell.0;
                        let out = mrly_math::cell::geometry::perforate::<2>(&mask, &cell, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let mask = mask.0;
                        let cell = cell.0;
                        let out = mrly_math::cell::geometry::perforate::<3>(&mask, &cell, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "perforate wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.cell.geometry")?;
                m.add_function(wrap_pyfunction!(merge_reps, &m)?)?;
                m.add_function(wrap_pyfunction!(perforate, &m)?)?;
                let names: Vec<&str> = vec!["merge_reps", "perforate"];
                m.add("__all__", names)?;
                parent.add("geometry", &m)?;
                sys.set_item("mrly.math._math.math.cell.geometry", &m)?;
                Ok(())
            }
        }

        pub mod models {
            use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyColor, PySerde, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "anti", signature = (cell))]
            pub fn anti<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::anti(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::anti(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "anti wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "binarize", signature = (cell, threshold))]
            pub fn binarize<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                threshold: u8,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::binarize(cell, threshold);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::binarize(cell, threshold);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "binarize wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "binarize_otsu", signature = (cell))]
            pub fn binarize_otsu<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::binarize_otsu(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::binarize_otsu(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "binarize_otsu wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "blur", signature = (cell, mask, wrap))]
            pub fn blur<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                mask: PyTensor,
                wrap: bool,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrly_math::cell::models::CellNd::<2>::blur(cell, &mask, wrap);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrly_math::cell::models::CellNd::<3>::blur(cell, &mask, wrap);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "blur wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "combine", signature = (cell, other))]
            pub fn combine<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                other: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let other = other.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let other = other.0;
                        let out = mrly_math::cell::models::CellNd::<2>::combine(&cell, &other);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let other = other.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let other = other.0;
                        let out = mrly_math::cell::models::CellNd::<3>::combine(&cell, &other);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "combine wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "counting_dtype", signature = (mask))]
            pub fn counting_dtype<'py>(
                py: Python<'py>,
                mask: PyTensor,
            ) -> PyResult<Bound<'py, PyAny>> {
                let mask = mask.0;
                let out = mrly_math::cell::models::counting_dtype(&mask);
                (PySerde(out)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "depth", signature = (cell))]
            pub fn depth<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
                let cell = cell.0;
                let out = mrly_math::cell::models::CellNd::<3>::depth(&cell);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "dtype_for", signature = (peak))]
            pub fn dtype_for<'py>(py: Python<'py>, peak: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::cell::models::dtype_for(peak);
                (PySerde(out)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "fractal", signature = (cell, level))]
            pub fn fractal<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                level: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::fractal(cell, level);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::fractal(cell, level);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "fractal wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "height", signature = (cell))]
            pub fn height<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::height(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::height(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "height wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "invert", signature = (cell))]
            pub fn invert<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::invert(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::invert(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "invert wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "layers", signature = (cell))]
            pub fn layers<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::layers(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::layers(cell);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "layers wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "neighbors", signature = (cell, mask, target, wrap))]
            pub fn neighbors<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                mask: PyTensor,
                target: u8,
                wrap: bool,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrly_math::cell::models::CellNd::<2>::neighbors(
                            cell, &mask, target, wrap,
                        );
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out = mrly_math::cell::models::CellNd::<3>::neighbors(
                            cell, &mask, target, wrap,
                        );
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "neighbors wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "new", signature = (types))]
            pub fn new<'py>(
                py: Python<'py>,
                types: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(types)?;
                match dim {
                    2 => {
                        let types = types.extract::<PyTensor>()?;
                        let types = types.0;
                        let out = mrly_math::cell::models::CellNd::<2>::new(types);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let types = types.extract::<PyTensor>()?;
                        let types = types.0;
                        let out = mrly_math::cell::models::CellNd::<3>::new(types);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "new wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "orient", signature = (cell, index))]
            pub fn orient<'py>(
                py: Python<'py>,
                cell: PyCell3d,
                index: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let cell = cell.0;
                let out = mrly_math::cell::models::CellNd::<3>::orient(cell, index);
                (PyCellNd(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "pad", signature = (cell, count, value))]
            pub fn pad<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                count: usize,
                value: u8,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::pad(cell, count, value);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::pad(cell, count, value);
                        (PyCellNd(out)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "pad wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "paint", signature = (cell, mapping, mode, rng=None))]
            pub fn paint<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                mapping: std::collections::HashMap<u8, Vec<PyColor>>,
                mode: PySerde<mrly_core::Mode>,
                rng: Option<&Bound<'_, PyAny>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mapping = mapping
                            .into_iter()
                            .map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>()))
                            .collect();
                        let mode = mode.0;
                        let mut rng_owned = rng.map(crate::hand::rng_from_py).transpose()?;
                        let out = mrly_math::cell::models::CellNd::<2>::paint(
                            cell,
                            &mapping,
                            mode,
                            rng_owned.as_mut(),
                        );
                        if let (Some(rng), Some(stream)) = (rng, &rng_owned) {
                            crate::hand::rng_write_back(rng, stream)?;
                        }
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mapping = mapping
                            .into_iter()
                            .map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>()))
                            .collect();
                        let mode = mode.0;
                        let mut rng_owned = rng.map(crate::hand::rng_from_py).transpose()?;
                        let out = mrly_math::cell::models::CellNd::<3>::paint(
                            cell,
                            &mapping,
                            mode,
                            rng_owned.as_mut(),
                        );
                        if let (Some(rng), Some(stream)) = (rng, &rng_owned) {
                            crate::hand::rng_write_back(rng, stream)?;
                        }
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "paint wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "perforate", signature = (cell, mask, value))]
            pub fn perforate<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                mask: PyTensor,
                value: u8,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out =
                            mrly_math::cell::models::CellNd::<2>::perforate(cell, &mask, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let mask = mask.0;
                        let out =
                            mrly_math::cell::models::CellNd::<3>::perforate(cell, &mask, value);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "perforate wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "rotate", signature = (cell, k, axes=None))]
            pub fn rotate<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                k: usize,
                axes: Option<(usize, usize)>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::rotate(cell, k);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let axes =
                            axes.ok_or_else(|| PyValueError::new_err("a 3d rotate wants axes."))?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::rotate(cell, k, axes);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "rotate wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "tile", signature = (cell, width, height, depth=None))]
            pub fn tile<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
                width: usize,
                height: usize,
                depth: Option<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::tile(cell, width, height);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let depth =
                            depth.ok_or_else(|| PyValueError::new_err("a 3d tile wants depth."))?;
                        let cell = cell.0;
                        let out =
                            mrly_math::cell::models::CellNd::<3>::tile(cell, width, height, depth);
                        (PyCellNd(ok(out)?)).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "tile wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "types", signature = (cell))]
            pub fn types<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::types(&cell);
                        (PyTensor((out).clone())).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::types(&cell);
                        (PyTensor((out).clone())).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "types wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            #[pyfunction]
            #[pyo3(name = "width", signature = (cell))]
            pub fn width<'py>(
                py: Python<'py>,
                cell: &Bound<'_, PyAny>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let dim = crate::hand::ndim(cell)?;
                match dim {
                    2 => {
                        let cell = cell.extract::<PyCell2d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<2>::width(&cell);
                        (out).into_bound_py_any(py)
                    }
                    3 => {
                        let cell = cell.extract::<PyCell3d>()?;
                        let cell = cell.0;
                        let out = mrly_math::cell::models::CellNd::<3>::width(&cell);
                        (out).into_bound_py_any(py)
                    }
                    other => Err(PyValueError::new_err(format!(
                        "width wants a 2d or 3d argument, got {other}d."
                    ))),
                }
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.cell.models")?;
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
                let names: Vec<&str> = vec![
                    "anti",
                    "binarize",
                    "binarize_otsu",
                    "blur",
                    "combine",
                    "counting_dtype",
                    "depth",
                    "dtype_for",
                    "fractal",
                    "height",
                    "invert",
                    "layers",
                    "neighbors",
                    "new",
                    "orient",
                    "pad",
                    "paint",
                    "perforate",
                    "rotate",
                    "tile",
                    "types",
                    "width",
                ];
                m.add("__all__", names)?;
                parent.add("models", &m)?;
                sys.set_item("mrly.math._math.math.cell.models", &m)?;
                Ok(())
            }
        }

        pub mod serializer {
            use crate::hand::{PyPixels, PySerde, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "byte_cube", signature = (value))]
            pub fn byte_cube<'py>(
                py: Python<'py>,
                value: PySerde<serde_json::Value>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrly_math::cell::serializer::byte_cube(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "byte_grid", signature = (value))]
            pub fn byte_grid<'py>(
                py: Python<'py>,
                value: PySerde<serde_json::Value>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrly_math::cell::serializer::byte_grid(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "color_grid", signature = (value))]
            pub fn color_grid<'py>(
                py: Python<'py>,
                value: PySerde<serde_json::Value>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrly_math::cell::serializer::color_grid(&value);
                ((ok(out)?).into_iter().map(PyPixels).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "count_cube", signature = (value))]
            pub fn count_cube<'py>(
                py: Python<'py>,
                value: PySerde<serde_json::Value>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrly_math::cell::serializer::count_cube(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "count_grid", signature = (value))]
            pub fn count_grid<'py>(
                py: Python<'py>,
                value: PySerde<serde_json::Value>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let value = value.0;
                let out = mrly_math::cell::serializer::count_grid(&value);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "parse", signature = (text))]
            pub fn parse<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::cell::serializer::parse(text);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "tag_layer", signature = (counts, shape))]
            pub fn tag_layer<'py>(
                py: Python<'py>,
                counts: Vec<i64>,
                shape: Vec<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::cell::serializer::tag_layer(&counts, shape);
                (PyTensor(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "types_field", signature = (data))]
            pub fn types_field<'py>(
                py: Python<'py>,
                data: PySerde<serde_json::Value>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let data = data.0;
                let out = mrly_math::cell::serializer::types_field(&data);
                (PySerde((ok(out)?).clone())).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.cell.serializer")?;
                m.add_function(wrap_pyfunction!(byte_cube, &m)?)?;
                m.add_function(wrap_pyfunction!(byte_grid, &m)?)?;
                m.add_function(wrap_pyfunction!(color_grid, &m)?)?;
                m.add_function(wrap_pyfunction!(count_cube, &m)?)?;
                m.add_function(wrap_pyfunction!(count_grid, &m)?)?;
                m.add_function(wrap_pyfunction!(parse, &m)?)?;
                m.add_function(wrap_pyfunction!(tag_layer, &m)?)?;
                m.add_function(wrap_pyfunction!(types_field, &m)?)?;
                let names: Vec<&str> = vec![
                    "byte_cube",
                    "byte_grid",
                    "color_grid",
                    "count_cube",
                    "count_grid",
                    "parse",
                    "tag_layer",
                    "types_field",
                ];
                m.add("__all__", names)?;
                parent.add("serializer", &m)?;
                sys.set_item("mrly.math._math.math.cell.serializer", &m)?;
                Ok(())
            }
        }

        #[pyfunction]
        #[pyo3(name = "grow", signature = (pattern, level))]
        pub fn grow<'py>(
            py: Python<'py>,
            pattern: &Bound<'_, PyAny>,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(pattern)?;
            match dim {
                2 => {
                    let pattern = pattern.extract::<PyTensor>()?;
                    let pattern = pattern.0;
                    let out = mrly_math::cell::grow::<2>(pattern, level);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let pattern = pattern.extract::<PyTensor>()?;
                    let pattern = pattern.0;
                    let out = mrly_math::cell::grow::<3>(pattern, level);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!(
                    "grow wants a 2d or 3d argument, got {other}d."
                ))),
            }
        }

        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, custom=None, mode=None, rng=None))]
        pub fn paint<'py>(
            py: Python<'py>,
            cell: &Bound<'_, PyAny>,
            custom: Option<std::collections::HashMap<u8, Vec<PyColor>>>,
            mode: Option<PySerde<mrly_core::Mode>>,
            rng: Option<&Bound<'_, PyAny>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let custom = custom.map(|x| {
                        x.into_iter()
                            .map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>()))
                            .collect()
                    });
                    let mode = mode.map(|x| x.0);
                    let mut rng_owned = rng.map(crate::hand::rng_from_py).transpose()?;
                    let out = mrly_math::cell::paint::<2>(
                        cell,
                        custom.as_ref(),
                        mode,
                        rng_owned.as_mut(),
                    );
                    if let (Some(rng), Some(stream)) = (rng, &rng_owned) {
                        crate::hand::rng_write_back(rng, stream)?;
                    }
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let custom = custom.map(|x| {
                        x.into_iter()
                            .map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>()))
                            .collect()
                    });
                    let mode = mode.map(|x| x.0);
                    let mut rng_owned = rng.map(crate::hand::rng_from_py).transpose()?;
                    let out = mrly_math::cell::paint::<3>(
                        cell,
                        custom.as_ref(),
                        mode,
                        rng_owned.as_mut(),
                    );
                    if let (Some(rng), Some(stream)) = (rng, &rng_owned) {
                        crate::hand::rng_write_back(rng, stream)?;
                    }
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!(
                    "paint wants a 2d or 3d argument, got {other}d."
                ))),
            }
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.cell")?;
            m.add_function(wrap_pyfunction!(grow, &m)?)?;
            m.add_function(wrap_pyfunction!(paint, &m)?)?;
            let names: Vec<&str> = vec!["grow", "paint"];
            m.add("__all__", names)?;
            census::init(py, &m, sys)?;
            geometry::init(py, &m, sys)?;
            models::init(py, &m, sys)?;
            serializer::init(py, &m, sys)?;
            parent.add("cell", &m)?;
            sys.set_item("mrly.math._math.math.cell", &m)?;
            Ok(())
        }
    }

    pub mod coprime {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "gasket", signature = (level))]
        pub fn gasket<'py>(py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::coprime::gasket(level);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.coprime")?;
            m.add_function(wrap_pyfunction!(gasket, &m)?)?;
            m.add("GASKET_LEVELS", mrly_math::coprime::GASKET_LEVELS)?;
            let names: Vec<&str> = vec!["gasket", "GASKET_LEVELS"];
            m.add("__all__", names)?;
            parent.add("coprime", &m)?;
            sys.set_item("mrly.math._math.math.coprime", &m)?;
            Ok(())
        }
    }

    pub mod counts {
        use crate::hand::{PyCode, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        pub mod diagonal {
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.counts.diagonal")?;
                m.add("WIDEST", mrly_math::counts::diagonal::WIDEST)?;
                let names: Vec<&str> = vec!["WIDEST"];
                m.add("__all__", names)?;
                parent.add("diagonal", &m)?;
                sys.set_item("mrly.math._math.math.counts.diagonal", &m)?;
                Ok(())
            }
        }

        pub mod ladder {
            use crate::hand::ok;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "cap", signature = (base))]
            pub fn cap<'py>(py: Python<'py>, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::cap(base);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "carry_matrix", signature = (base, dimension))]
            pub fn carry_matrix<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::carry_matrix(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "characteristic", signature = (rows))]
            pub fn characteristic<'py>(
                py: Python<'py>,
                rows: Vec<Vec<i128>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::characteristic(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "determinant", signature = (rows))]
            pub fn determinant<'py>(
                py: Python<'py>,
                rows: Vec<Vec<i128>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::determinant(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "digit_polynomial", signature = (base, dimension))]
            pub fn digit_polynomial<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::digit_polynomial(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "even_block", signature = (base, dimension))]
            pub fn even_block<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::even_block(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "fill", signature = (base, dimension))]
            pub fn fill<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::fill(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "ladder", signature = (base, dimension, levels))]
            pub fn ladder<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
                levels: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::ladder(base, dimension, levels);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "perron", signature = (rows))]
            pub fn perron<'py>(
                py: Python<'py>,
                rows: Vec<Vec<i128>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::perron(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "sign", signature = (base, dimension))]
            pub fn sign<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::sign(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "spectral_ratio", signature = (base, dimension))]
            pub fn spectral_ratio<'py>(
                py: Python<'py>,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::spectral_ratio(base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "trace", signature = (rows))]
            pub fn trace<'py>(
                py: Python<'py>,
                rows: Vec<Vec<i128>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::trace(&rows);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.counts.ladder")?;
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
                let names: Vec<&str> = vec![
                    "cap",
                    "carry_matrix",
                    "characteristic",
                    "determinant",
                    "digit_polynomial",
                    "even_block",
                    "fill",
                    "ladder",
                    "perron",
                    "sign",
                    "spectral_ratio",
                    "trace",
                ];
                m.add("__all__", names)?;
                parent.add("ladder", &m)?;
                sys.set_item("mrly.math._math.math.counts.ladder", &m)?;
                Ok(())
            }
        }

        pub mod six {
            use crate::hand::ok;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "grid_triangles", signature = (number, level))]
            pub fn grid_triangles<'py>(
                py: Python<'py>,
                number: usize,
                level: u32,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::grid_triangles(number, level);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "solid_slice_boundary", signature = (number))]
            pub fn solid_slice_boundary<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_boundary(number);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "solid_slice_core_edges", signature = (number))]
            pub fn solid_slice_core_edges<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_core_edges(number);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "solid_slice_core_nodes", signature = (number))]
            pub fn solid_slice_core_nodes<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_core_nodes(number);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "solid_slice_edges", signature = (number))]
            pub fn solid_slice_edges<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_edges(number);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "solid_slice_interior", signature = (number))]
            pub fn solid_slice_interior<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_interior(number);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "solid_slice_triangles", signature = (number))]
            pub fn solid_slice_triangles<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_triangles(number);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "solid_slice_vertices", signature = (number))]
            pub fn solid_slice_vertices<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_vertices(number);
                (ok(out)?).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.counts.six")?;
                m.add_function(wrap_pyfunction!(grid_triangles, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_boundary, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_core_edges, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_core_nodes, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_edges, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_interior, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_triangles, &m)?)?;
                m.add_function(wrap_pyfunction!(solid_slice_vertices, &m)?)?;
                let names: Vec<&str> = vec![
                    "grid_triangles",
                    "solid_slice_boundary",
                    "solid_slice_core_edges",
                    "solid_slice_core_nodes",
                    "solid_slice_edges",
                    "solid_slice_interior",
                    "solid_slice_triangles",
                    "solid_slice_vertices",
                ];
                m.add("__all__", names)?;
                parent.add("six", &m)?;
                sys.set_item("mrly.math._math.math.counts.six", &m)?;
                Ok(())
            }
        }

        #[pyclass(name = "Closed", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Closed(pub mrly_math::counts::Closed);

        #[pymethods]
        impl Closed {
            #[pyo3(name = "text", signature = ())]
            pub fn text<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Closed::text(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Exposure", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Exposure(pub mrly_math::counts::Exposure);

        #[pymethods]
        impl Exposure {
            #[getter]
            #[pyo3(name = "occupancy")]
            pub fn occupancy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.occupancy;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "occupancy")]
            pub fn set_occupancy(&mut self, value: u128) -> PyResult<()> {
                self.0.occupancy = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "exposed")]
            pub fn exposed<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.exposed;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "exposed")]
            pub fn set_exposed(&mut self, value: u128) -> PyResult<()> {
                self.0.exposed = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "axes")]
            pub fn axes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.axes.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "axes")]
            pub fn set_axes(&mut self, value: Vec<(u128, u128)>) -> PyResult<()> {
                self.0.axes = value;
                Ok(())
            }
            #[pyo3(name = "at", signature = (level))]
            pub fn at<'py>(&self, py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Exposure::at(&self.0, level);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_corners", signature = (filled, number, dimension, base))]
            pub fn from_corners<'py>(
                py: Python<'py>,
                filled: Vec<Vec<u8>>,
                number: usize,
                dimension: usize,
                base: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    mrly_math::counts::Exposure::from_corners(&filled, number, dimension, base);
                (crate::doors::math::counts::Exposure(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "of_tile", signature = (tile))]
            pub fn of_tile<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
                let tile = tile.0;
                let out = mrly_math::counts::Exposure::of_tile(&tile);
                (crate::doors::math::counts::Exposure(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "recurrence", signature = ())]
            pub fn recurrence<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Exposure::recurrence(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Placements", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Placements(pub mrly_math::counts::Placements);

        #[pymethods]
        impl Placements {
            #[pyo3(name = "at", signature = (level))]
            pub fn at<'py>(&self, py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Placements::at(&self.0, level);
                ((out).map(crate::doors::math::counts::Skeleton)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "of_tile", signature = (tile))]
            pub fn of_tile<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
                let tile = tile.0;
                let out = mrly_math::counts::Placements::of_tile(&tile);
                (crate::doors::math::counts::Placements(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "recurrence", signature = (k=None))]
            pub fn recurrence<'py>(
                &self,
                py: Python<'py>,
                k: Option<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Placements::recurrence(&self.0, k);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "roots", signature = (k=None))]
            pub fn roots<'py>(
                &self,
                py: Python<'py>,
                k: Option<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Placements::roots(&self.0, k);
                (out).into_bound_py_any(py)
            }
        }

        #[pyclass(name = "Skeleton", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Skeleton(pub mrly_math::counts::Skeleton);

        #[pymethods]
        impl Skeleton {
            #[getter]
            #[pyo3(name = "counts")]
            pub fn counts<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.counts.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "counts")]
            pub fn set_counts(&mut self, value: Vec<u128>) -> PyResult<()> {
                self.0.counts = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "euler")]
            pub fn euler<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.euler;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "euler")]
            pub fn set_euler(&mut self, value: i128) -> PyResult<()> {
                self.0.euler = value;
                Ok(())
            }
            #[pyo3(name = "cells", signature = ())]
            pub fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::cells(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "edges", signature = ())]
            pub fn edges<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::edges(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "faces", signature = ())]
            pub fn faces<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::faces(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "vertices", signature = ())]
            pub fn vertices<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::vertices(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "centered_hexagonal", signature = (m))]
        pub fn centered_hexagonal<'py>(py: Python<'py>, m: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::counts::centered_hexagonal(m);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "cut_fills", signature = (code, number, level))]
        pub fn cut_fills<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::cut_fills(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "cut_voids", signature = (code, number, level))]
        pub fn cut_voids<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::cut_voids(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "dimension", signature = (code, number, base_dimension, base))]
        pub fn dimension<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            base_dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::dimension(code, number, base_dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "edges_of_tile", signature = (tile, level))]
        pub fn edges_of_tile<'py>(
            py: Python<'py>,
            tile: PyTensor,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrly_math::counts::edges_of_tile(&tile, level);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "exposure", signature = (code, number, dimension, level, base))]
        pub fn exposure<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            dimension: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::exposure(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "exposure_of_tile", signature = (tile, level))]
        pub fn exposure_of_tile<'py>(
            py: Python<'py>,
            tile: PyTensor,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrly_math::counts::exposure_of_tile(&tile, level);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "exposure_recurrence", signature = (tile))]
        pub fn exposure_recurrence<'py>(
            py: Python<'py>,
            tile: PyTensor,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrly_math::counts::exposure_recurrence(&tile);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fill", signature = (code, number, dimension, level, base))]
        pub fn fill<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            dimension: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::fill(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fill_from_corners", signature = (filled, number, _dimension, level, base))]
        pub fn fill_from_corners<'py>(
            py: Python<'py>,
            filled: Vec<Vec<u8>>,
            number: usize,
            _dimension: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out =
                mrly_math::counts::fill_from_corners(&filled, number, _dimension, level, base);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fill_polynomial", signature = (corners, dimension))]
        pub fn fill_polynomial<'py>(
            py: Python<'py>,
            corners: Vec<Vec<u8>>,
            dimension: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::counts::fill_polynomial(&corners, dimension);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "grid", signature = (number, dimension, level))]
        pub fn grid<'py>(
            py: Python<'py>,
            number: usize,
            dimension: usize,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::counts::grid(number, dimension, level);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "limit", signature = (code, dimension, level, base))]
        pub fn limit<'py>(
            py: Python<'py>,
            code: PyCode,
            dimension: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::limit(code, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "odd_power", signature = (dimension))]
        pub fn odd_power<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::counts::odd_power(dimension);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pairs", signature = (tile))]
        pub fn pairs<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrly_math::counts::pairs(&tile);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "positions", signature = (residue, number, base))]
        pub fn positions<'py>(
            py: Python<'py>,
            residue: usize,
            number: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::counts::positions(residue, number, base);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pro_fills", signature = (code, number, level))]
        pub fn pro_fills<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::pro_fills(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pro_voids", signature = (code, number, level))]
        pub fn pro_voids<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::pro_voids(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "profile_of_tile", signature = (tile, level))]
        pub fn profile_of_tile<'py>(
            py: Python<'py>,
            tile: PyTensor,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrly_math::counts::profile_of_tile(&tile, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ratio", signature = (code, number, dimension, level, base))]
        pub fn ratio<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            dimension: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::ratio(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "rational", signature = (code, number, dimension, level, base))]
        pub fn rational<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            dimension: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::rational(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "skeleton", signature = (code, number, dimension, base, level))]
        pub fn skeleton<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            dimension: usize,
            base: usize,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::skeleton(code, number, dimension, base, level);
            (crate::doors::math::counts::Skeleton(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "skeleton_of_tile", signature = (tile, level))]
        pub fn skeleton_of_tile<'py>(
            py: Python<'py>,
            tile: PyTensor,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrly_math::counts::skeleton_of_tile(&tile, level);
            ((ok(out)?).map(crate::doors::math::counts::Skeleton)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "skeleton_recurrence", signature = (code, number, dimension, base, k=None))]
        pub fn skeleton_recurrence<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            dimension: usize,
            base: usize,
            k: Option<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::skeleton_recurrence(code, number, dimension, base, k);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "surface", signature = (code, number, level, base))]
        pub fn surface<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::surface(code, number, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "void", signature = (code, number, dimension, level, base))]
        pub fn void<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            dimension: usize,
            level: u32,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::counts::void(code, number, dimension, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.counts")?;
            m.add_class::<Closed>()?;
            m.add_class::<Exposure>()?;
            m.add_class::<Placements>()?;
            m.add_class::<Skeleton>()?;
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
            m.add_function(wrap_pyfunction!(fill_polynomial, &m)?)?;
            m.add_function(wrap_pyfunction!(grid, &m)?)?;
            m.add_function(wrap_pyfunction!(limit, &m)?)?;
            m.add_function(wrap_pyfunction!(odd_power, &m)?)?;
            m.add_function(wrap_pyfunction!(pairs, &m)?)?;
            m.add_function(wrap_pyfunction!(positions, &m)?)?;
            m.add_function(wrap_pyfunction!(pro_fills, &m)?)?;
            m.add_function(wrap_pyfunction!(pro_voids, &m)?)?;
            m.add_function(wrap_pyfunction!(profile_of_tile, &m)?)?;
            m.add_function(wrap_pyfunction!(ratio, &m)?)?;
            m.add_function(wrap_pyfunction!(rational, &m)?)?;
            m.add_function(wrap_pyfunction!(skeleton, &m)?)?;
            m.add_function(wrap_pyfunction!(skeleton_of_tile, &m)?)?;
            m.add_function(wrap_pyfunction!(skeleton_recurrence, &m)?)?;
            m.add_function(wrap_pyfunction!(surface, &m)?)?;
            m.add_function(wrap_pyfunction!(void, &m)?)?;
            let names: Vec<&str> = vec![
                "centered_hexagonal",
                "cut_fills",
                "cut_voids",
                "dimension",
                "edges_of_tile",
                "exposure",
                "exposure_of_tile",
                "exposure_recurrence",
                "fill",
                "fill_from_corners",
                "fill_polynomial",
                "grid",
                "limit",
                "odd_power",
                "pairs",
                "positions",
                "pro_fills",
                "pro_voids",
                "profile_of_tile",
                "ratio",
                "rational",
                "skeleton",
                "skeleton_of_tile",
                "skeleton_recurrence",
                "surface",
                "void",
                "Closed",
                "Exposure",
                "Placements",
                "Skeleton",
            ];
            m.add("__all__", names)?;
            diagonal::init(py, &m, sys)?;
            ladder::init(py, &m, sys)?;
            six::init(py, &m, sys)?;
            parent.add("counts", &m)?;
            sys.set_item("mrly.math._math.math.counts", &m)?;
            Ok(())
        }
    }

    pub mod design {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "digits_of", signature = (mask, base))]
        pub fn digits_of<'py>(
            py: Python<'py>,
            mask: u32,
            base: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::digits_of(mask, base);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "echo_series", signature = (values, log_x, exponent))]
        pub fn echo_series<'py>(
            py: Python<'py>,
            values: Vec<u64>,
            log_x: Vec<f64>,
            exponent: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::echo_series(&values, &log_x, exponent);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "elements", signature = (base, digits, depth))]
        pub fn elements<'py>(
            py: Python<'py>,
            base: u64,
            digits: Vec<u64>,
            depth: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::elements(base, &digits, depth);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "log_grid", signature = (values, samples))]
        pub fn log_grid<'py>(
            py: Python<'py>,
            values: Vec<u64>,
            samples: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::log_grid(&values, samples);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "median_floor", signature = (power, width))]
        pub fn median_floor<'py>(
            py: Python<'py>,
            power: Vec<f64>,
            width: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::median_floor(&power, width);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "meter", signature = (mu))]
        pub fn meter<'py>(py: Python<'py>, mu: Vec<i8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::meter(&mu);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "nearest", signature = (value, list))]
        pub fn nearest<'py>(
            py: Python<'py>,
            value: f64,
            list: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::nearest(value, &list);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "peaks", signature = (gamma, score, band, threshold))]
        pub fn peaks<'py>(
            py: Python<'py>,
            gamma: Vec<f64>,
            score: Vec<f64>,
            band: (f64, f64),
            threshold: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::peaks(&gamma, &score, band, threshold);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pole_lattice", signature = (base, top))]
        pub fn pole_lattice<'py>(
            py: Python<'py>,
            base: u64,
            top: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::pole_lattice(base, top);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "resample", signature = (values, running, exponent, log_x))]
        pub fn resample<'py>(
            py: Python<'py>,
            values: Vec<u64>,
            running: Vec<i64>,
            exponent: f64,
            log_x: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::resample(&values, &running, exponent, &log_x);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "score", signature = (power, width))]
        pub fn score<'py>(
            py: Python<'py>,
            power: Vec<f64>,
            width: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::score(&power, width);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "size", signature = (digits, depth))]
        pub fn size<'py>(
            py: Python<'py>,
            digits: Vec<u64>,
            depth: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::size(&digits, depth);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "spectrum", signature = (log_x, series))]
        pub fn spectrum<'py>(
            py: Python<'py>,
            log_x: Vec<f64>,
            series: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::spectrum(&log_x, &series);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "upper_rms", signature = (series))]
        pub fn upper_rms<'py>(py: Python<'py>, series: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::upper_rms(&series);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.design")?;
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
            m.add("ZETA_ORDINATES", mrly_math::design::ZETA_ORDINATES)?;
            let names: Vec<&str> = vec![
                "digits_of",
                "echo_series",
                "elements",
                "log_grid",
                "median_floor",
                "meter",
                "nearest",
                "peaks",
                "pole_lattice",
                "resample",
                "score",
                "size",
                "spectrum",
                "upper_rms",
                "ZETA_ORDINATES",
            ];
            m.add("__all__", names)?;
            parent.add("design", &m)?;
            sys.set_item("mrly.math._math.math.design", &m)?;
            Ok(())
        }
    }

    pub mod dissection {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "cap", signature = (y))]
        pub fn cap<'py>(py: Python<'py>, y: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::cap(y);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "chain_exponent", signature = (base))]
        pub fn chain_exponent<'py>(py: Python<'py>, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_exponent(base);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "chain_margin", signature = (base))]
        pub fn chain_margin<'py>(py: Python<'py>, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_margin(base);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "chain_root", signature = (base))]
        pub fn chain_root<'py>(py: Python<'py>, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_root(base);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "chain_wall", signature = ())]
        pub fn chain_wall<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_wall();
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "consecutive", signature = (digits))]
        pub fn consecutive<'py>(py: Python<'py>, digits: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::consecutive(&digits);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fraction", signature = (a, y, cap))]
        pub fn fraction<'py>(
            py: Python<'py>,
            a: u64,
            y: u64,
            cap: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::fraction(a, y, cap);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "kappa", signature = (base, digits))]
        pub fn kappa<'py>(
            py: Python<'py>,
            base: u64,
            digits: Vec<u64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::kappa(base, &digits);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "masses", signature = (base, digits, level))]
        pub fn masses<'py>(
            py: Python<'py>,
            base: u64,
            digits: Vec<u64>,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::masses(base, &digits, level);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "reach", signature = (base, missing))]
        pub fn reach<'py>(py: Python<'py>, base: u64, missing: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::reach(base, missing);
            ((out).to_string()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "reading", signature = (base, fill, masses))]
        pub fn reading<'py>(
            py: Python<'py>,
            base: u64,
            fill: usize,
            masses: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::reading(base, fill, &masses);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "regions", signature = (base, level, z))]
        pub fn regions<'py>(
            py: Python<'py>,
            base: u64,
            level: u32,
            z: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::regions(base, level, z);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tally", signature = (base, digits, level, samples))]
        pub fn tally<'py>(
            py: Python<'py>,
            base: u64,
            digits: Vec<u64>,
            level: usize,
            samples: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::tally(base, &digits, level, samples);
            (PySerde(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "weights", signature = (base, digits, level))]
        pub fn weights<'py>(
            py: Python<'py>,
            base: u64,
            digits: Vec<u64>,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::weights(base, &digits, level);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.dissection")?;
            m.add_function(wrap_pyfunction!(cap, &m)?)?;
            m.add_function(wrap_pyfunction!(chain_exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(chain_margin, &m)?)?;
            m.add_function(wrap_pyfunction!(chain_root, &m)?)?;
            m.add_function(wrap_pyfunction!(chain_wall, &m)?)?;
            m.add_function(wrap_pyfunction!(consecutive, &m)?)?;
            m.add_function(wrap_pyfunction!(fraction, &m)?)?;
            m.add_function(wrap_pyfunction!(kappa, &m)?)?;
            m.add_function(wrap_pyfunction!(masses, &m)?)?;
            m.add_function(wrap_pyfunction!(reach, &m)?)?;
            m.add_function(wrap_pyfunction!(reading, &m)?)?;
            m.add_function(wrap_pyfunction!(regions, &m)?)?;
            m.add_function(wrap_pyfunction!(tally, &m)?)?;
            m.add_function(wrap_pyfunction!(weights, &m)?)?;
            m.add("BAR_A", mrly_math::dissection::BAR_A)?;
            m.add("BAR_B", mrly_math::dissection::BAR_B)?;
            m.add("CAP_BASE", mrly_math::dissection::CAP_BASE)?;
            m.add("CERTIFIED", mrly_math::dissection::CERTIFIED)?;
            m.add("DIGIT_WALL", mrly_math::dissection::DIGIT_WALL)?;
            m.add("FIRST_BELOW", mrly_math::dissection::FIRST_BELOW)?;
            m.add("GAMMA", mrly_math::dissection::GAMMA)?;
            m.add("WINDOW_WALL", mrly_math::dissection::WINDOW_WALL)?;
            let names: Vec<&str> = vec![
                "cap",
                "chain_exponent",
                "chain_margin",
                "chain_root",
                "chain_wall",
                "consecutive",
                "fraction",
                "kappa",
                "masses",
                "reach",
                "reading",
                "regions",
                "tally",
                "weights",
                "BAR_A",
                "BAR_B",
                "CAP_BASE",
                "CERTIFIED",
                "DIGIT_WALL",
                "FIRST_BELOW",
                "GAMMA",
                "WINDOW_WALL",
            ];
            m.add("__all__", names)?;
            parent.add("dissection", &m)?;
            sys.set_item("mrly.math._math.math.dissection", &m)?;
            Ok(())
        }
    }

    pub mod game {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "cell", signature = (point, base, level))]
        pub fn cell<'py>(
            py: Python<'py>,
            point: Vec<f64>,
            base: u8,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::cell(&point, base, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "cells", signature = (base, dimension, level))]
        pub fn cells<'py>(
            py: Python<'py>,
            base: u8,
            dimension: u8,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::cells(base, dimension, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "counts", signature = (matrix, start, levels))]
        pub fn counts<'py>(
            py: Python<'py>,
            matrix: Vec<Vec<u64>>,
            start: usize,
            levels: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::counts(&matrix, start, levels);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "de_bruijn", signature = (m, k))]
        pub fn de_bruijn<'py>(py: Python<'py>, m: u32, k: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::de_bruijn(m, k);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mosaic_game", signature = (mosaic))]
        pub fn mosaic_game<'py>(
            py: Python<'py>,
            mosaic: PySerde<mrly_math::game::Mosaic>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mosaic = mosaic.0;
            let out = mrly_math::game::mosaic_game(&mosaic);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "named_games", signature = ())]
        pub fn named_games<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::named_games();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "perron", signature = (matrix))]
        pub fn perron<'py>(py: Python<'py>, matrix: Vec<Vec<u64>>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::perron(&matrix);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "reverse", signature = (rule))]
        pub fn reverse<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::reverse(&rule.0);
            (crate::doors::math::memory::Rule(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "slice_game", signature = (base, digits, p, q))]
        pub fn slice_game<'py>(
            py: Python<'py>,
            base: u8,
            digits: Vec<u8>,
            p: i64,
            q: i64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::slice_game(base, &digits, p, q);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "successors", signature = (rule, history))]
        pub fn successors<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
            history: Vec<u8>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::successors(&rule.0, &history);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "word_cell", signature = (word, base, dimension))]
        pub fn word_cell<'py>(
            py: Python<'py>,
            word: Vec<u8>,
            base: u8,
            dimension: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::word_cell(&word, base, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.game")?;
            m.add_function(wrap_pyfunction!(cell, &m)?)?;
            m.add_function(wrap_pyfunction!(cells, &m)?)?;
            m.add_function(wrap_pyfunction!(counts, &m)?)?;
            m.add_function(wrap_pyfunction!(de_bruijn, &m)?)?;
            m.add_function(wrap_pyfunction!(mosaic_game, &m)?)?;
            m.add_function(wrap_pyfunction!(named_games, &m)?)?;
            m.add_function(wrap_pyfunction!(perron, &m)?)?;
            m.add_function(wrap_pyfunction!(reverse, &m)?)?;
            m.add_function(wrap_pyfunction!(slice_game, &m)?)?;
            m.add_function(wrap_pyfunction!(successors, &m)?)?;
            m.add_function(wrap_pyfunction!(word_cell, &m)?)?;
            m.add("LAP", mrly_math::game::LAP)?;
            let names: Vec<&str> = vec![
                "cell",
                "cells",
                "counts",
                "de_bruijn",
                "mosaic_game",
                "named_games",
                "perron",
                "reverse",
                "slice_game",
                "successors",
                "word_cell",
                "LAP",
            ];
            m.add("__all__", names)?;
            parent.add("game", &m)?;
            sys.set_item("mrly.math._math.math.game", &m)?;
            Ok(())
        }
    }

    pub mod graph {
        use crate::hand::{PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Layout", module = "mrly.math.graph", from_py_object)]
        #[derive(Clone)]
        pub struct Layout(pub mrly_math::graph::Layout);

        #[pymethods]
        impl Layout {
            #[new]
            #[pyo3(signature = (positions, branches, dim, seed))]
            pub fn __new__(
                positions: Vec<f64>,
                branches: Vec<(usize, usize)>,
                dim: usize,
                seed: u64,
            ) -> PyResult<Self> {
                let out = mrly_math::graph::Layout::new(&positions, &branches, dim, seed);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "energy", signature = ())]
            pub fn energy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::energy(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_network", signature = (network, seed))]
            pub fn from_network<'py>(
                py: Python<'py>,
                network: PyRef<'_, crate::doors::math::graph::Network>,
                seed: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::from_network(&network.0, seed);
                (crate::doors::math::graph::Layout(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "ideal", signature = ())]
            pub fn ideal<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::ideal(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "moved", signature = ())]
            pub fn moved<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::moved(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (positions, branches, dim, seed))]
            pub fn new_<'py>(
                py: Python<'py>,
                positions: Vec<f64>,
                branches: Vec<(usize, usize)>,
                dim: usize,
                seed: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::new(&positions, &branches, dim, seed);
                (crate::doors::math::graph::Layout(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "nodes", signature = ())]
            pub fn nodes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::nodes(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "positions", signature = ())]
            pub fn positions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::positions(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            #[pyo3(name = "step", signature = (ticks))]
            pub fn step<'py>(
                &mut self,
                py: Python<'py>,
                ticks: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::step(&mut self.0, ticks);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "temperature", signature = ())]
            pub fn temperature<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::temperature(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "ticks", signature = ())]
            pub fn ticks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::ticks(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        #[pyclass(name = "Network", module = "mrly.math.graph", from_py_object)]
        #[derive(Clone)]
        pub struct Network(pub mrly_math::graph::Network);

        #[pymethods]
        impl Network {
            #[new]
            #[pyo3(signature = (dim))]
            pub fn __new__(dim: usize) -> PyResult<Self> {
                let out = mrly_math::graph::Network::new(dim);
                Ok(Self(out))
            }
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dim")]
            pub fn set_dim(&mut self, value: usize) -> PyResult<()> {
                self.0.dim = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "nodes")]
            pub fn nodes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.nodes.clone();
                ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "nodes")]
            pub fn set_nodes(
                &mut self,
                value: Vec<PySerde<mrly_math::graph::Node>>,
            ) -> PyResult<()> {
                let value = value.into_iter().map(|x| x.0).collect::<Vec<_>>();
                self.0.nodes = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "branches")]
            pub fn branches<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.branches.clone();
                ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "branches")]
            pub fn set_branches(
                &mut self,
                value: Vec<PySerde<mrly_math::graph::Branch>>,
            ) -> PyResult<()> {
                let value = value.into_iter().map(|x| x.0).collect::<Vec<_>>();
                self.0.branches = value;
                Ok(())
            }
            #[pyo3(name = "add_branch", signature = (parent, child, radius))]
            pub fn add_branch<'py>(
                &mut self,
                py: Python<'py>,
                parent: usize,
                child: usize,
                radius: f64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::add_branch(&mut self.0, parent, child, radius);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "add_node", signature = (position))]
            pub fn add_node<'py>(
                &mut self,
                py: Python<'py>,
                position: Vec<f64>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::add_node(&mut self.0, position);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "adjacency", signature = ())]
            pub fn adjacency<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::adjacency(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "degree", signature = ())]
            pub fn degree<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::degree(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (dim))]
            pub fn new_<'py>(py: Python<'py>, dim: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::new(dim);
                (crate::doors::math::graph::Network(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyfunction]
        #[pyo3(name = "census", signature = (network))]
        pub fn census<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::census(&network.0);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "components", signature = (network))]
        pub fn components<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::components(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "core_graph", signature = (grid))]
        pub fn core_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrly_math::graph::core_graph(&grid);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "edge_graph", signature = (grid))]
        pub fn edge_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrly_math::graph::edge_graph(&grid);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fractal_dimension", signature = (network, samples))]
        pub fn fractal_dimension<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
            samples: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::fractal_dimension(&network.0, samples);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "junctions", signature = (network))]
        pub fn junctions<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::junctions(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "largest_component", signature = (network))]
        pub fn largest_component<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::largest_component(&network.0);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "roles", signature = (network))]
        pub fn roles<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::roles(&network.0);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tips", signature = (network))]
        pub fn tips<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::tips(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "total_length", signature = (network))]
        pub fn total_length<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::total_length(&network.0);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tunnel_graph", signature = (grid))]
        pub fn tunnel_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrly_math::graph::tunnel_graph(&grid);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.graph")?;
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
            let names: Vec<&str> = vec![
                "census",
                "components",
                "core_graph",
                "edge_graph",
                "fractal_dimension",
                "junctions",
                "largest_component",
                "roles",
                "tips",
                "total_length",
                "tunnel_graph",
                "Layout",
                "Network",
            ];
            m.add("__all__", names)?;
            parent.add("graph", &m)?;
            sys.set_item("mrly.math._math.math.graph", &m)?;
            Ok(())
        }
    }

    pub mod ladder {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Design", module = "mrly.math.ladder", from_py_object)]
        #[derive(Clone)]
        pub struct Design(pub mrly_math::ladder::Design);

        #[pymethods]
        impl Design {
            #[new]
            #[pyo3(signature = (base, digits))]
            pub fn __new__(base: u64, digits: Vec<u64>) -> PyResult<Self> {
                let out = mrly_math::ladder::Design::new(base, &digits);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "abscissa", signature = ())]
            pub fn abscissa<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::abscissa(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::base(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "digits", signature = ())]
            pub fn digits<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::digits(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (base, digits))]
            pub fn new_<'py>(
                py: Python<'py>,
                base: u64,
                digits: Vec<u64>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::new(base, &digits);
                (crate::doors::math::ladder::Design(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "peel", signature = ())]
            pub fn peel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::peel(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "period", signature = ())]
            pub fn period<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::period(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "pole", signature = (m, j))]
            pub fn pole<'py>(
                &self,
                py: Python<'py>,
                m: usize,
                j: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::pole(&self.0, m, j);
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "with_peel", signature = (base, digits, peel))]
            pub fn with_peel<'py>(
                py: Python<'py>,
                base: u64,
                digits: Vec<u64>,
                peel: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::with_peel(base, &digits, peel);
                (crate::doors::math::ladder::Design(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyfunction]
        #[pyo3(name = "cofactor", signature = (design, s, tolerance))]
        pub fn cofactor<'py>(
            py: Python<'py>,
            design: PyRef<'_, crate::doors::math::ladder::Design>,
            s: PySerde<mrly_num::zeta::Complex>,
            tolerance: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let s = s.0;
            let out = mrly_math::ladder::cofactor(&design.0, s, tolerance);
            ({
                let t = ok(out)?;
                (PySerde(t.0), t.1)
            })
            .into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "residue", signature = (design, m, j, tolerance))]
        pub fn residue<'py>(
            py: Python<'py>,
            design: PyRef<'_, crate::doors::math::ladder::Design>,
            m: usize,
            j: i64,
            tolerance: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::ladder::residue(&design.0, m, j, tolerance);
            ({
                let t = ok(out)?;
                (PySerde(t.0), t.1)
            })
            .into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zeta", signature = (design, s, tolerance))]
        pub fn zeta<'py>(
            py: Python<'py>,
            design: PyRef<'_, crate::doors::math::ladder::Design>,
            s: PySerde<mrly_num::zeta::Complex>,
            tolerance: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let s = s.0;
            let out = mrly_math::ladder::zeta(&design.0, s, tolerance);
            ({
                let t = ok(out)?;
                (PySerde(t.0), t.1)
            })
            .into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.ladder")?;
            m.add_class::<Design>()?;
            m.add_function(wrap_pyfunction!(cofactor, &m)?)?;
            m.add_function(wrap_pyfunction!(residue, &m)?)?;
            m.add_function(wrap_pyfunction!(zeta, &m)?)?;
            m.add("ROUNDING", mrly_math::ladder::ROUNDING)?;
            let names: Vec<&str> = vec!["cofactor", "residue", "zeta", "Design", "ROUNDING"];
            m.add("__all__", names)?;
            parent.add("ladder", &m)?;
            sys.set_item("mrly.math._math.math.ladder", &m)?;
            Ok(())
        }
    }

    pub mod memory {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Rule", module = "mrly.math.memory", from_py_object)]
        #[derive(Clone)]
        pub struct Rule(pub mrly_math::memory::Rule);

        #[pymethods]
        impl Rule {
            #[new]
            #[pyo3(signature = (dimension, width, code))]
            pub fn __new__(dimension: usize, width: usize, code: u64) -> PyResult<Self> {
                let out = mrly_math::memory::Rule::new(dimension, width, code);
                Ok(Self(ok(out)?))
            }
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dimension")]
            pub fn set_dimension(&mut self, value: usize) -> PyResult<()> {
                self.0.dimension = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "width")]
            pub fn width<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.width;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "width")]
            pub fn set_width(&mut self, value: usize) -> PyResult<()> {
                self.0.width = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "code")]
            pub fn set_code(&mut self, value: u64) -> PyResult<()> {
                self.0.code = value;
                Ok(())
            }
            #[pyo3(name = "accepts", signature = (word))]
            pub fn accepts<'py>(
                &self,
                py: Python<'py>,
                word: Vec<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::accepts(&self.0, &word);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "allowed", signature = (window))]
            pub fn allowed<'py>(
                &self,
                py: Python<'py>,
                window: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::allowed(&self.0, window);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "alphabet", signature = ())]
            pub fn alphabet<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::alphabet(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "codes", signature = ())]
            pub fn codes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::codes(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "full", signature = (dimension, width))]
            pub fn full<'py>(
                py: Python<'py>,
                dimension: usize,
                width: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::full(dimension, width);
                (crate::doors::math::memory::Rule(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "letters", signature = ())]
            pub fn letters<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::letters(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (dimension, width, code))]
            pub fn new_<'py>(
                py: Python<'py>,
                dimension: usize,
                width: usize,
                code: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::new(dimension, width, code);
                (crate::doors::math::memory::Rule(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "states", signature = ())]
            pub fn states<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::states(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "windows", signature = ())]
            pub fn windows<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::windows(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyfunction]
        #[pyo3(name = "allowed_windows", signature = (rule))]
        pub fn allowed_windows<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::allowed_windows(&rule.0);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "cells", signature = (rule, level))]
        pub fn cells<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::cells(&rule.0, level);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "counts", signature = (rule, levels))]
        pub fn counts<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
            levels: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::counts(&rule.0, levels);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "exponent", signature = (rule))]
        pub fn exponent<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::exponent(&rule.0);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "kappa", signature = (rule))]
        pub fn kappa<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::kappa(&rule.0);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "perron", signature = (rule))]
        pub fn perron<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::perron(&rule.0);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "transfer", signature = (rule))]
        pub fn transfer<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::transfer(&rule.0);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.memory")?;
            m.add_class::<Rule>()?;
            m.add_function(wrap_pyfunction!(allowed_windows, &m)?)?;
            m.add_function(wrap_pyfunction!(cells, &m)?)?;
            m.add_function(wrap_pyfunction!(counts, &m)?)?;
            m.add_function(wrap_pyfunction!(exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(kappa, &m)?)?;
            m.add_function(wrap_pyfunction!(perron, &m)?)?;
            m.add_function(wrap_pyfunction!(transfer, &m)?)?;
            m.add("SPAN", mrly_math::memory::SPAN)?;
            m.add("SWEEPS", mrly_math::memory::SWEEPS)?;
            m.add("TOLERANCE", mrly_math::memory::TOLERANCE)?;
            let names: Vec<&str> = vec![
                "allowed_windows",
                "cells",
                "counts",
                "exponent",
                "kappa",
                "perron",
                "transfer",
                "Rule",
                "SPAN",
                "SWEEPS",
                "TOLERANCE",
            ];
            m.add("__all__", names)?;
            parent.add("memory", &m)?;
            sys.set_item("mrly.math._math.math.memory", &m)?;
            Ok(())
        }
    }

    pub mod moire {
        use crate::hand::{PyImage, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        pub mod pairs {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "correlation", signature = (m, n))]
            pub fn correlation<'py>(
                py: Python<'py>,
                m: usize,
                n: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::pairs::correlation(m, n);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "sampled", signature = (m, n))]
            pub fn sampled<'py>(
                py: Python<'py>,
                m: usize,
                n: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::pairs::sampled(m, n);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "witness", signature = (scale))]
            pub fn witness<'py>(py: Python<'py>, scale: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::pairs::witness(scale);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.moire.pairs")?;
                m.add_function(wrap_pyfunction!(correlation, &m)?)?;
                m.add_function(wrap_pyfunction!(sampled, &m)?)?;
                m.add_function(wrap_pyfunction!(witness, &m)?)?;
                let names: Vec<&str> = vec!["correlation", "sampled", "witness"];
                m.add("__all__", names)?;
                parent.add("pairs", &m)?;
                sys.set_item("mrly.math._math.math.moire.pairs", &m)?;
                Ok(())
            }
        }

        pub mod sample {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "axes", signature = (size, lattice, row))]
            pub fn axes<'py>(
                py: Python<'py>,
                size: usize,
                lattice: PySerde<mrly_math::moire::Lattice>,
                row: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_math::moire::sample::axes(size, lattice, row);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "membership", signature = (code, base, dimension))]
            pub fn membership<'py>(
                py: Python<'py>,
                code: u128,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::sample::membership(code, base, dimension);
                (ok(out)?).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "pack", signature = (residues, base))]
            pub fn pack<'py>(
                py: Python<'py>,
                residues: Vec<usize>,
                base: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::sample::pack(&residues, base);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.moire.sample")?;
                m.add_function(wrap_pyfunction!(axes, &m)?)?;
                m.add_function(wrap_pyfunction!(membership, &m)?)?;
                m.add_function(wrap_pyfunction!(pack, &m)?)?;
                let names: Vec<&str> = vec!["axes", "membership", "pack"];
                m.add("__all__", names)?;
                parent.add("sample", &m)?;
                sys.set_item("mrly.math._math.math.moire.sample", &m)?;
                Ok(())
            }
        }

        #[pyclass(name = "Field", module = "mrly.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Field(pub mrly_math::moire::Field);

        #[pymethods]
        impl Field {
            #[new]
            #[pyo3(signature = (size))]
            pub fn __new__(size: usize) -> PyResult<Self> {
                let out = mrly_math::moire::Field::new(size);
                Ok(Self(out))
            }
            #[getter]
            #[pyo3(name = "data")]
            pub fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.data.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "data")]
            pub fn set_data(&mut self, value: Vec<f32>) -> PyResult<()> {
                self.0.data = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "size")]
            pub fn size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.size;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "size")]
            pub fn set_size(&mut self, value: usize) -> PyResult<()> {
                self.0.size = value;
                Ok(())
            }
            #[pyo3(name = "as_f64", signature = ())]
            pub fn as_f64<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::as_f64(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_data", signature = (data, size))]
            pub fn from_data<'py>(
                py: Python<'py>,
                data: Vec<f32>,
                size: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::from_data(data, size);
                (crate::doors::math::moire::Field(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "max", signature = ())]
            pub fn max<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::max(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "mean", signature = ())]
            pub fn mean<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::mean(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "min", signature = ())]
            pub fn min<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::min(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (size))]
            pub fn new_<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::new(size);
                (crate::doors::math::moire::Field(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "normalized", signature = (symmetric))]
            pub fn normalized<'py>(
                &self,
                py: Python<'py>,
                symmetric: bool,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::normalized(&self.0, symmetric);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Preset", module = "mrly.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Preset(pub mrly_math::moire::Preset);

        #[pymethods]
        impl Preset {
            #[getter]
            #[pyo3(name = "name")]
            pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.name;
                ((value).to_string()).into_bound_py_any(py)
            }
            #[getter]
            #[pyo3(name = "spec")]
            pub fn spec<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.spec;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "spec")]
            pub fn set_spec(&mut self, value: PySerde<mrly_math::moire::Spec>) -> PyResult<()> {
                let value = value.0;
                self.0.spec = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "numbers")]
            pub fn numbers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.numbers.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "numbers")]
            pub fn set_numbers(&mut self, value: Vec<usize>) -> PyResult<()> {
                self.0.numbers = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "combine")]
            pub fn combine<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.combine;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "combine")]
            pub fn set_combine(
                &mut self,
                value: PySerde<mrly_math::moire::Combine>,
            ) -> PyResult<()> {
                let value = value.0;
                self.0.combine = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "level")]
            pub fn level<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.level;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "level")]
            pub fn set_level(&mut self, value: usize) -> PyResult<()> {
                self.0.level = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "lattice")]
            pub fn lattice<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.lattice;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "lattice")]
            pub fn set_lattice(
                &mut self,
                value: PySerde<mrly_math::moire::Lattice>,
            ) -> PyResult<()> {
                let value = value.0;
                self.0.lattice = value;
                Ok(())
            }
            #[staticmethod]
            #[pyo3(name = "carpet", signature = (limit))]
            pub fn carpet<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::carpet(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "field", signature = (size))]
            pub fn field<'py>(&self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::field(&self.0, size);
                (crate::doors::math::moire::Field(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "heatmap", signature = (limit))]
            pub fn heatmap<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::heatmap(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "hive", signature = (limit))]
            pub fn hive<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::hive(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "weave", signature = (limit))]
            pub fn weave<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::weave(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Volume", module = "mrly.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Volume(pub mrly_math::moire::Volume);

        #[pymethods]
        impl Volume {
            #[new]
            #[pyo3(signature = (size))]
            pub fn __new__(size: usize) -> PyResult<Self> {
                let out = mrly_math::moire::Volume::new(size);
                Ok(Self(out))
            }
            #[getter]
            #[pyo3(name = "data")]
            pub fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.data.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "data")]
            pub fn set_data(&mut self, value: Vec<f32>) -> PyResult<()> {
                self.0.data = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "size")]
            pub fn size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.size;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "size")]
            pub fn set_size(&mut self, value: usize) -> PyResult<()> {
                self.0.size = value;
                Ok(())
            }
            #[pyo3(name = "at", signature = (x, y, z))]
            pub fn at<'py>(
                &self,
                py: Python<'py>,
                x: usize,
                y: usize,
                z: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::at(&self.0, x, y, z);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "count", signature = (level))]
            pub fn count<'py>(&self, py: Python<'py>, level: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::count(&self.0, level);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_data", signature = (data, size))]
            pub fn from_data<'py>(
                py: Python<'py>,
                data: Vec<f32>,
                size: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::from_data(data, size);
                (crate::doors::math::moire::Volume(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "max", signature = ())]
            pub fn max<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::max(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "min", signature = ())]
            pub fn min<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::min(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (size))]
            pub fn new_<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::new(size);
                (crate::doors::math::moire::Volume(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "plane", signature = (frame, out))]
            pub fn plane<'py>(
                &self,
                py: Python<'py>,
                frame: PySerde<mrly_math::moire::Frame>,
                out: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let frame = frame.0;
                let out = mrly_math::moire::Volume::plane(&self.0, &frame, out);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "sample", signature = (p))]
            pub fn sample<'py>(&self, py: Python<'py>, p: [f64; 3]) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::sample(&self.0, p);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "solid", signature = (level))]
            pub fn solid<'py>(&self, py: Python<'py>, level: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::solid(&self.0, level);
                (PyTensor(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Layer", module = "mrly.math.moire", skip_from_py_object)]
        pub struct Layer;

        #[pymethods]
        impl Layer {
            #[staticmethod]
            #[pyo3(name = "new", signature = (spec, number))]
            pub fn new_<'py>(
                py: Python<'py>,
                spec: PySerde<mrly_math::moire::Spec>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let spec = spec.0;
                let out = mrly_math::moire::Layer::new(spec, number);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyclass(name = "Spec", module = "mrly.math.moire", skip_from_py_object)]
        pub struct Spec;

        #[pymethods]
        impl Spec {
            #[staticmethod]
            #[pyo3(name = "new", signature = (code, base, dimension))]
            pub fn new_<'py>(
                py: Python<'py>,
                code: u128,
                base: usize,
                dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Spec::new(code, base, dimension);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "all", signature = (limit))]
        pub fn all<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::moire::all(limit);
            ((out)
                .into_iter()
                .map(crate::doors::math::moire::Preset)
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "frame", signature = (normal, offset))]
        pub fn frame<'py>(
            py: Python<'py>,
            normal: [f64; 3],
            offset: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::moire::frame(normal, offset);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "layer", signature = (params))]
        pub fn layer<'py>(
            py: Python<'py>,
            params: PySerde<mrly_math::moire::Layer>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let params = params.0;
            let out = mrly_math::moire::layer(&params);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "named", signature = (name, limit))]
        pub fn named<'py>(
            py: Python<'py>,
            name: &str,
            limit: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::moire::named(name, limit);
            (crate::doors::math::moire::Preset(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "render", signature = (field, colorizer, levels, symmetric, invert, scale))]
        pub fn render<'py>(
            py: Python<'py>,
            field: PyRef<'_, crate::doors::math::moire::Field>,
            colorizer: PySerde<mrly_core::Colorizer>,
            levels: usize,
            symmetric: bool,
            invert: bool,
            scale: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let colorizer = colorizer.0;
            let out =
                mrly_math::moire::render(&field.0, &colorizer, levels, symmetric, invert, scale);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "stack", signature = (spec, numbers, combine, level, lattice, size, slices))]
        pub fn stack<'py>(
            py: Python<'py>,
            spec: PySerde<mrly_math::moire::Spec>,
            numbers: Vec<usize>,
            combine: PySerde<mrly_math::moire::Combine>,
            level: usize,
            lattice: PySerde<mrly_math::moire::Lattice>,
            size: usize,
            slices: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let spec = spec.0;
            let combine = combine.0;
            let lattice = lattice.0;
            let out =
                mrly_math::moire::stack(spec, &numbers, combine, level, lattice, size, &slices);
            (crate::doors::math::moire::Field(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "stack_codes", signature = (specs, number, level, lattice, size, slices))]
        pub fn stack_codes<'py>(
            py: Python<'py>,
            specs: Vec<PySerde<mrly_math::moire::Spec>>,
            number: usize,
            level: usize,
            lattice: PySerde<mrly_math::moire::Lattice>,
            size: usize,
            slices: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let specs = specs.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let lattice = lattice.0;
            let out = mrly_math::moire::stack_codes(&specs, number, level, lattice, size, &slices);
            (crate::doors::math::moire::Field(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "volume", signature = (spec, numbers, combine, level, size))]
        pub fn volume<'py>(
            py: Python<'py>,
            spec: PySerde<mrly_math::moire::Spec>,
            numbers: Vec<usize>,
            combine: PySerde<mrly_math::moire::Combine>,
            level: usize,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let spec = spec.0;
            let combine = combine.0;
            let out = mrly_math::moire::volume(spec, &numbers, combine, level, size);
            (crate::doors::math::moire::Volume(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.moire")?;
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
            let names: Vec<&str> = vec![
                "all",
                "frame",
                "layer",
                "named",
                "render",
                "stack",
                "stack_codes",
                "volume",
                "Field",
                "Preset",
                "Volume",
                "Layer",
                "Spec",
            ];
            m.add("__all__", names)?;
            pairs::init(py, &m, sys)?;
            sample::init(py, &m, sys)?;
            parent.add("moire", &m)?;
            sys.set_item("mrly.math._math.math.moire", &m)?;
            Ok(())
        }
    }

    pub mod name {
        use crate::hand::{PyCode, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Bang", module = "mrly.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Bang(pub mrly_math::name::Bang);

        #[pymethods]
        impl Bang {
            #[new]
            #[pyo3(signature = (code, dim, base))]
            pub fn __new__(code: u128, dim: usize, base: usize) -> PyResult<Self> {
                let out = mrly_math::name::Bang::new(code, dim, base);
                Ok(Self(out))
            }
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dim")]
            pub fn set_dim(&mut self, value: usize) -> PyResult<()> {
                self.0.dim = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "lattice")]
            pub fn lattice<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.lattice;
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "lattice")]
            pub fn set_lattice(
                &mut self,
                value: PySerde<mrly_math::name::Lattice>,
            ) -> PyResult<()> {
                let value = value.0;
                self.0.lattice = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "base")]
            pub fn set_base(&mut self, value: usize) -> PyResult<()> {
                self.0.base = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "code")]
            pub fn set_code(&mut self, value: u128) -> PyResult<()> {
                self.0.code = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "twist")]
            pub fn twist<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.twist.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "twist")]
            pub fn set_twist(&mut self, value: Option<Vec<usize>>) -> PyResult<()> {
                self.0.twist = value;
                Ok(())
            }
            #[pyo3(name = "cells", signature = ())]
            pub fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Bang::cells(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    <mrly_math::name::Bang as mrly_core::name::Named>::checked(self.0.clone());
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::from_file(text);
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::from_json(text);
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::from_url(text);
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (code, dim, base))]
            pub fn new_<'py>(
                py: Python<'py>,
                code: u128,
                dim: usize,
                base: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Bang::new(code, dim, base);
                (crate::doors::math::name::Bang(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_prose", signature = ())]
            pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_prose(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_url(&self.0);
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

        #[pyclass(name = "Sequence", module = "mrly.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Sequence(pub mrly_math::name::Sequence);

        #[pymethods]
        impl Sequence {
            #[new]
            #[pyo3(signature = (code, dim, base, measure, axis))]
            pub fn __new__(
                code: u128,
                dim: usize,
                base: usize,
                measure: &str,
                axis: &str,
            ) -> PyResult<Self> {
                let out = mrly_math::name::Sequence::new(code, dim, base, measure, axis);
                Ok(Self(out))
            }
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dim")]
            pub fn set_dim(&mut self, value: usize) -> PyResult<()> {
                self.0.dim = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "base")]
            pub fn set_base(&mut self, value: usize) -> PyResult<()> {
                self.0.base = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "code")]
            pub fn set_code(&mut self, value: u128) -> PyResult<()> {
                self.0.code = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "measure")]
            pub fn measure<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.measure.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "measure")]
            pub fn set_measure(&mut self, value: String) -> PyResult<()> {
                self.0.measure = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "axis")]
            pub fn axis<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.axis.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "axis")]
            pub fn set_axis(&mut self, value: String) -> PyResult<()> {
                self.0.axis = value;
                Ok(())
            }
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    <mrly_math::name::Sequence as mrly_core::name::Named>::checked(self.0.clone());
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "design", signature = ())]
            pub fn design<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Sequence::design(&self.0);
                (crate::doors::math::name::Bang(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::from_file(text);
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::from_json(text);
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::from_url(text);
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (code, dim, base, measure, axis))]
            pub fn new_<'py>(
                py: Python<'py>,
                code: u128,
                dim: usize,
                base: usize,
                measure: &str,
                axis: &str,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Sequence::new(code, dim, base, measure, axis);
                (crate::doors::math::name::Sequence(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_prose", signature = ())]
            pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_prose(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_url(&self.0);
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

        #[pyclass(name = "Word", module = "mrly.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Word(pub mrly_math::name::Word);

        #[pymethods]
        impl Word {
            #[new]
            #[pyo3(signature = (dim, letters))]
            pub fn __new__(dim: usize, letters: Vec<(u128, usize)>) -> PyResult<Self> {
                let out = mrly_math::name::Word::new(dim, &letters);
                Ok(Self(ok(out)?))
            }
            #[getter]
            #[pyo3(name = "dim")]
            pub fn dim<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dim;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dim")]
            pub fn set_dim(&mut self, value: usize) -> PyResult<()> {
                self.0.dim = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "magic")]
            pub fn magic<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.magic.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "magic")]
            pub fn set_magic(&mut self, value: Vec<u128>) -> PyResult<()> {
                self.0.magic = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "side")]
            pub fn side<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.side.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "side")]
            pub fn set_side(&mut self, value: Vec<usize>) -> PyResult<()> {
                self.0.side = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "base")]
            pub fn set_base(&mut self, value: Option<Vec<usize>>) -> PyResult<()> {
                self.0.base = value;
                Ok(())
            }
            #[pyo3(name = "bases", signature = ())]
            pub fn bases<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Word::bases(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    <mrly_math::name::Word as mrly_core::name::Named>::checked(self.0.clone());
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::from_file(text);
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::from_json(text);
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::from_url(text);
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "letters", signature = ())]
            pub fn letters<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Word::letters(&self.0);
                ((out)
                    .into_iter()
                    .map(crate::doors::math::name::Bang)
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (dim, letters))]
            pub fn new_<'py>(
                py: Python<'py>,
                dim: usize,
                letters: Vec<(u128, usize)>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Word::new(dim, &letters);
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_prose", signature = ())]
            pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_prose(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_url(&self.0);
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

        #[pyclass(name = "Lattice", module = "mrly.math.name", skip_from_py_object)]
        pub struct Lattice;

        #[pymethods]
        impl Lattice {
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Lattice::default();
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "is_square", signature = (lattice))]
            pub fn is_square<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_math::name::Lattice>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_math::name::Lattice::is_square(&lattice);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "units", signature = (lattice))]
            pub fn units<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_math::name::Lattice>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_math::name::Lattice::units(lattice);
                (out).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "of", signature = (code, dim, base))]
        pub fn of<'py>(
            py: Python<'py>,
            code: PyCode,
            dim: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::name::of(code, dim, base);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "words", signature = (dim, base))]
        pub fn words<'py>(py: Python<'py>, dim: usize, base: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::name::words(dim, base);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.name")?;
            m.add_class::<Bang>()?;
            m.add_class::<Sequence>()?;
            m.add_class::<Word>()?;
            m.add_class::<Lattice>()?;
            m.add_function(wrap_pyfunction!(of, &m)?)?;
            m.add_function(wrap_pyfunction!(words, &m)?)?;
            let names: Vec<&str> = vec!["of", "words", "Bang", "Sequence", "Word", "Lattice"];
            m.add("__all__", names)?;
            parent.add("name", &m)?;
            sys.set_item("mrly.math._math.math.name", &m)?;
            Ok(())
        }
    }

    pub mod nodal {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "figures", signature = (code, number, level, count))]
        pub fn figures<'py>(
            py: Python<'py>,
            code: u64,
            number: usize,
            level: usize,
            count: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::nodal::figures(code, number, level, count);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "holds", signature = (code, number, level))]
        pub fn holds<'py>(
            py: Python<'py>,
            code: u64,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::nodal::holds(code, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.nodal")?;
            m.add_function(wrap_pyfunction!(figures, &m)?)?;
            m.add_function(wrap_pyfunction!(holds, &m)?)?;
            let names: Vec<&str> = vec!["figures", "holds"];
            m.add("__all__", names)?;
            parent.add("nodal", &m)?;
            sys.set_item("mrly.math._math.math.nodal", &m)?;
            Ok(())
        }
    }

    pub mod press {
        use crate::hand::{PyCode, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Press", module = "mrly.math.press", skip_from_py_object)]
        pub struct Press(pub mrly_math::press::Press);

        #[pymethods]
        impl Press {
            #[new]
            #[pyo3(signature = (dimension, base))]
            pub fn __new__(dimension: usize, base: usize) -> PyResult<Self> {
                let out = mrly_math::press::Press::new(dimension, base);
                Ok(Self(ok(out)?))
            }
            #[getter]
            #[pyo3(name = "dimension")]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.dimension;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "dimension")]
            pub fn set_dimension(&mut self, value: usize) -> PyResult<()> {
                self.0.dimension = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "base")]
            pub fn set_base(&mut self, value: usize) -> PyResult<()> {
                self.0.base = value;
                Ok(())
            }
            #[pyo3(name = "add", signature = (number, weight))]
            pub fn add<'py>(
                &mut self,
                py: Python<'py>,
                number: u128,
                weight: i128,
            ) -> PyResult<Bound<'py, PyAny>> {
                mrly_math::press::Press::add(&mut self.0, number, weight);
                ().into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (dimension, base))]
            pub fn new_<'py>(
                py: Python<'py>,
                dimension: usize,
                base: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::press::Press::new(dimension, base);
                (crate::doors::math::press::Press(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "total", signature = (code))]
            pub fn total<'py>(&self, py: Python<'py>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::press::Press::total(&self.0, code);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "totals", signature = ())]
            pub fn totals<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::press::Press::totals(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyfunction]
        #[pyo3(name = "containing", signature = (number, dimension, base))]
        pub fn containing<'py>(
            py: Python<'py>,
            number: u128,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::press::containing(number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "coordinates", signature = (number, dimension, base))]
        pub fn coordinates<'py>(
            py: Python<'py>,
            number: u128,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::press::coordinates(number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "count_below", signature = (code, dimension, base, limit))]
        pub fn count_below<'py>(
            py: Python<'py>,
            code: PyCode,
            dimension: usize,
            base: usize,
            limit: u128,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::press::count_below(code, dimension, base, limit);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "distinct", signature = (number, dimension, base))]
        pub fn distinct<'py>(
            py: Python<'py>,
            number: u128,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::press::distinct(number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "interleave", signature = (coords, base))]
        pub fn interleave<'py>(
            py: Python<'py>,
            coords: Vec<u128>,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::press::interleave(&coords, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "layer_table", signature = (layer))]
        pub fn layer_table<'py>(
            py: Python<'py>,
            layer: PySerde<mrly_math::bang::MagicLayer>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let layer = layer.0;
            let out = mrly_math::press::layer_table(&layer);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "member", signature = (code, number, dimension, base))]
        pub fn member<'py>(
            py: Python<'py>,
            code: PyCode,
            number: u128,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::press::member(code, number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "members", signature = (code, dimension, base, count))]
        pub fn members<'py>(
            py: Python<'py>,
            code: PyCode,
            dimension: usize,
            base: usize,
            count: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::press::members(code, dimension, base, count);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "profile", signature = (code, dimension, base, level))]
        pub fn profile<'py>(
            py: Python<'py>,
            code: PyCode,
            dimension: usize,
            base: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::press::profile(code, dimension, base, level);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "usage", signature = (number, dimension, base))]
        pub fn usage<'py>(
            py: Python<'py>,
            number: u128,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::press::usage(number, dimension, base);
            (PyCode(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "word_count", signature = (layers))]
        pub fn word_count<'py>(
            py: Python<'py>,
            layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::press::word_count(&layers);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "word_member", signature = (layers, number))]
        pub fn word_member<'py>(
            py: Python<'py>,
            layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
            number: u128,
        ) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::press::word_member(&layers, number);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "word_members", signature = (layers))]
        pub fn word_members<'py>(
            py: Python<'py>,
            layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::press::word_members(&layers);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "word_profile", signature = (layers))]
        pub fn word_profile<'py>(
            py: Python<'py>,
            layers: Vec<PySerde<mrly_math::bang::MagicLayer>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let layers = layers.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::press::word_profile(&layers);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.press")?;
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
            m.add("CORNERS", mrly_math::press::CORNERS)?;
            let names: Vec<&str> = vec![
                "containing",
                "coordinates",
                "count_below",
                "distinct",
                "interleave",
                "layer_table",
                "member",
                "members",
                "profile",
                "usage",
                "word_count",
                "word_member",
                "word_members",
                "word_profile",
                "Press",
                "CORNERS",
            ];
            m.add("__all__", names)?;
            parent.add("press", &m)?;
            sys.set_item("mrly.math._math.math.press", &m)?;
            Ok(())
        }
    }

    pub mod radix {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Base", module = "mrly.math.radix", from_py_object)]
        #[derive(Clone)]
        pub struct Base(pub mrly_math::radix::Base);

        #[pymethods]
        impl Base {
            #[new]
            #[pyo3(signature = (ring, value))]
            pub fn __new__(
                ring: PySerde<mrly_num::gauss::Ring>,
                value: (i64, i64),
            ) -> PyResult<Self> {
                let ring = ring.0;
                let out = mrly_math::radix::Base::new(ring, value);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "class_", signature = (z))]
            pub fn class_<'py>(
                &self,
                py: Python<'py>,
                z: (i64, i64),
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::class(self.0, z);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "congruent", signature = (z, w))]
            pub fn congruent<'py>(
                &self,
                py: Python<'py>,
                z: (i64, i64),
                w: (i64, i64),
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::congruent(self.0, z, w);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "group", signature = ())]
            pub fn group<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::group(self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "mirrored", signature = ())]
            pub fn mirrored<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::mirrored(self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (ring, value))]
            pub fn new_<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                value: (i64, i64),
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_math::radix::Base::new(ring, value);
                (crate::doors::math::radix::Base(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "norm", signature = ())]
            pub fn norm<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::norm(self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "power", signature = (level))]
            pub fn power<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::power(self.0, level);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "residues", signature = ())]
            pub fn residues<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::residues(self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::ring(self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "value", signature = ())]
            pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::value(self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Radix", module = "mrly.math.radix", from_py_object)]
        #[derive(Clone)]
        pub struct Radix(pub mrly_math::radix::Radix);

        #[pymethods]
        impl Radix {
            #[new]
            #[pyo3(signature = (base, digits, twists))]
            pub fn __new__(
                base: crate::doors::math::radix::Base,
                digits: Vec<(i64, i64)>,
                twists: Vec<(i64, i64)>,
            ) -> PyResult<Self> {
                let base = base.0;
                let out = mrly_math::radix::Radix::new(base, digits, twists);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::base(&self.0);
                (crate::doors::math::radix::Base(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "canonical", signature = ())]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::canonical(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "code", signature = ())]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::code(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "digits", signature = ())]
            pub fn digits<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::digits(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            #[pyo3(name = "dimension", signature = ())]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::dimension(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "distinct", signature = (level))]
            pub fn distinct<'py>(
                &self,
                py: Python<'py>,
                level: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::distinct(&self.0, level);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "fill", signature = (level))]
            pub fn fill<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::fill(&self.0, level);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_code", signature = (base, code))]
            pub fn from_code<'py>(
                py: Python<'py>,
                base: crate::doors::math::radix::Base,
                code: u128,
            ) -> PyResult<Bound<'py, PyAny>> {
                let base = base.0;
                let out = mrly_math::radix::Radix::from_code(base, code);
                (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (base, digits, twists))]
            pub fn new_<'py>(
                py: Python<'py>,
                base: crate::doors::math::radix::Base,
                digits: Vec<(i64, i64)>,
                twists: Vec<(i64, i64)>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let base = base.0;
                let out = mrly_math::radix::Radix::new(base, digits, twists);
                (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "plane", signature = (level))]
            pub fn plane<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::plane(&self.0, level);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::ring(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "size", signature = ())]
            pub fn size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::size(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "twists", signature = ())]
            pub fn twists<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::twists(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            #[pyo3(name = "with_twists", signature = (units))]
            pub fn with_twists<'py>(
                &self,
                py: Python<'py>,
                units: Vec<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::with_twists(self.0.clone(), &units);
                (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "words", signature = (level))]
            pub fn words<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::words(&self.0, level);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyfunction]
        #[pyo3(name = "flowsnake", signature = ())]
        pub fn flowsnake<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::flowsnake();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "gasket", signature = ())]
        pub fn gasket<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::gasket();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "koch", signature = ())]
        pub fn koch<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::koch();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "terdragon", signature = ())]
        pub fn terdragon<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::terdragon();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tile", signature = (m, code))]
        pub fn tile<'py>(py: Python<'py>, m: u64, code: u128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::tile(m, code);
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "twindragon", signature = ())]
        pub fn twindragon<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::twindragon();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.radix")?;
            m.add_class::<Base>()?;
            m.add_class::<Radix>()?;
            m.add_function(wrap_pyfunction!(flowsnake, &m)?)?;
            m.add_function(wrap_pyfunction!(gasket, &m)?)?;
            m.add_function(wrap_pyfunction!(koch, &m)?)?;
            m.add_function(wrap_pyfunction!(terdragon, &m)?)?;
            m.add_function(wrap_pyfunction!(tile, &m)?)?;
            m.add_function(wrap_pyfunction!(twindragon, &m)?)?;
            let names: Vec<&str> = vec![
                "flowsnake",
                "gasket",
                "koch",
                "terdragon",
                "tile",
                "twindragon",
                "Base",
                "Radix",
            ];
            m.add("__all__", names)?;
            parent.add("radix", &m)?;
            sys.set_item("mrly.math._math.math.radix", &m)?;
            Ok(())
        }
    }

    pub mod roulette {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Nodes", module = "mrly.math.roulette", from_py_object)]
        #[derive(Clone)]
        pub struct Nodes(pub mrly_math::roulette::Nodes);

        #[pymethods]
        impl Nodes {
            #[getter]
            #[pyo3(name = "curves")]
            pub fn curves<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.curves;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "curves")]
            pub fn set_curves(&mut self, value: usize) -> PyResult<()> {
                self.0.curves = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "selves")]
            pub fn selves<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.selves.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "selves")]
            pub fn set_selves(&mut self, value: Vec<usize>) -> PyResult<()> {
                self.0.selves = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "pairs")]
            pub fn pairs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.pairs.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "pairs")]
            pub fn set_pairs(&mut self, value: Vec<usize>) -> PyResult<()> {
                self.0.pairs = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "most")]
            pub fn most<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.most;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "most")]
            pub fn set_most(&mut self, value: usize) -> PyResult<()> {
                self.0.most = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "crowded")]
            pub fn crowded<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.crowded;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "crowded")]
            pub fn set_crowded(&mut self, value: usize) -> PyResult<()> {
                self.0.crowded = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "points")]
            pub fn points<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.points;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "points")]
            pub fn set_points(&mut self, value: usize) -> PyResult<()> {
                self.0.points = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "branches")]
            pub fn branches<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.branches;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "branches")]
            pub fn set_branches(&mut self, value: usize) -> PyResult<()> {
                self.0.branches = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "touches")]
            pub fn touches<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.touches;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "touches")]
            pub fn set_touches(&mut self, value: usize) -> PyResult<()> {
                self.0.touches = value;
                Ok(())
            }
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::default();
                (crate::doors::math::roulette::Nodes(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "pair", signature = (i, j))]
            pub fn pair<'py>(
                &self,
                py: Python<'py>,
                i: usize,
                j: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::pair(&self.0, i, j);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "paired", signature = ())]
            pub fn paired<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::paired(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "selved", signature = ())]
            pub fn selved<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::selved(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "total", signature = ())]
            pub fn total<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::total(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyfunction]
        #[pyo3(name = "nodes", signature = (track, pencils, samples, tol))]
        pub fn nodes<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
            samples: usize,
            tol: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::roulette::nodes(&track, &pencils, samples, tol);
            (crate::doors::math::roulette::Nodes(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "side", signature = (a, b, c))]
        pub fn side<'py>(
            py: Python<'py>,
            a: [f64; 2],
            b: [f64; 2],
            c: [f64; 2],
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::roulette::side(a, b, c);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "spread", signature = (track, pencils, exact))]
        pub fn spread<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
            exact: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::roulette::spread(&track, &pencils, exact);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.roulette")?;
            m.add_class::<Nodes>()?;
            m.add_function(wrap_pyfunction!(nodes, &m)?)?;
            m.add_function(wrap_pyfunction!(side, &m)?)?;
            m.add_function(wrap_pyfunction!(spread, &m)?)?;
            let names: Vec<&str> = vec!["nodes", "side", "spread", "Nodes"];
            m.add("__all__", names)?;
            parent.add("roulette", &m)?;
            sys.set_item("mrly.math._math.math.roulette", &m)?;
            Ok(())
        }
    }

    pub mod rules {
        use crate::hand::{PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "render", signature = (filled, number, dimension, base))]
        pub fn render<'py>(
            py: Python<'py>,
            filled: Vec<Vec<u8>>,
            number: usize,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::rules::render(&filled, number, dimension, base);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tree_axes", signature = (dimension, free_axis))]
        pub fn tree_axes<'py>(
            py: Python<'py>,
            dimension: usize,
            free_axis: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::rules::tree_axes(dimension, free_axis);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.rules")?;
            m.add_function(wrap_pyfunction!(render, &m)?)?;
            m.add_function(wrap_pyfunction!(tree_axes, &m)?)?;
            m.add("BASE", mrly_math::rules::BASE)?;
            let names: Vec<&str> = vec!["render", "tree_axes", "BASE"];
            m.add("__all__", names)?;
            parent.add("rules", &m)?;
            sys.set_item("mrly.math._math.math.rules", &m)?;
            Ok(())
        }
    }

    pub mod shape {
        use crate::hand::{PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Frac", module = "mrly.math.shape", from_py_object)]
        #[derive(Clone)]
        pub struct Frac(pub mrly_math::shape::Frac);

        #[pymethods]
        impl Frac {
            #[new]
            #[pyo3(signature = (num, den))]
            pub fn __new__(num: i64, den: i64) -> PyResult<Self> {
                let out = mrly_math::shape::Frac::new(num, den);
                Ok(Self(ok(out)?))
            }
            #[getter]
            #[pyo3(name = "num")]
            pub fn num<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.num;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "num")]
            pub fn set_num(&mut self, value: i64) -> PyResult<()> {
                self.0.num = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "den")]
            pub fn den<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.den;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "den")]
            pub fn set_den(&mut self, value: i64) -> PyResult<()> {
                self.0.den = value;
                Ok(())
            }
            #[pyo3(name = "minus", signature = (other))]
            pub fn minus<'py>(
                &self,
                py: Python<'py>,
                other: crate::doors::math::shape::Frac,
            ) -> PyResult<Bound<'py, PyAny>> {
                let other = other.0;
                let out = mrly_math::shape::Frac::minus(self.0, other);
                (crate::doors::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (num, den))]
            pub fn new_<'py>(py: Python<'py>, num: i64, den: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::shape::Frac::new(num, den);
                (crate::doors::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "plus", signature = (other))]
            pub fn plus<'py>(
                &self,
                py: Python<'py>,
                other: crate::doors::math::shape::Frac,
            ) -> PyResult<Bound<'py, PyAny>> {
                let other = other.0;
                let out = mrly_math::shape::Frac::plus(self.0, other);
                (crate::doors::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "times", signature = (other))]
            pub fn times<'py>(
                &self,
                py: Python<'py>,
                other: crate::doors::math::shape::Frac,
            ) -> PyResult<Bound<'py, PyAny>> {
                let other = other.0;
                let out = mrly_math::shape::Frac::times(self.0, other);
                (crate::doors::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "whole", signature = (num))]
            pub fn whole<'py>(py: Python<'py>, num: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::shape::Frac::whole(num);
                (crate::doors::math::shape::Frac(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Region", module = "mrly.math.shape", skip_from_py_object)]
        pub struct Region;

        #[pymethods]
        impl Region {
            #[staticmethod]
            #[pyo3(name = "flip", signature = (region))]
            pub fn flip<'py>(
                py: Python<'py>,
                region: PySerde<mrly_math::shape::Region>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let region = region.0;
                let out = mrly_math::shape::Region::flip(region);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "census", signature = (shape, types))]
        pub fn census<'py>(
            py: Python<'py>,
            shape: PySerde<mrly_math::shape::Shape>,
            types: PyTensor,
        ) -> PyResult<Bound<'py, PyAny>> {
            let shape = shape.0;
            let types = types.0;
            let out = mrly_math::shape::census(&shape, &types);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "classify", signature = (shape, side, index))]
        pub fn classify<'py>(
            py: Python<'py>,
            shape: PySerde<mrly_math::shape::Shape>,
            side: usize,
            index: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let shape = shape.0;
            let out = mrly_math::shape::classify(&shape, side, &index);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "crop", signature = (types, shape, keep_cut))]
        pub fn crop<'py>(
            py: Python<'py>,
            types: PyTensor,
            shape: PySerde<mrly_math::shape::Shape>,
            keep_cut: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let shape = shape.0;
            let out = mrly_math::shape::crop(&types, &shape, keep_cut);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "crossing_shell", signature = (radius, number, level))]
        pub fn crossing_shell<'py>(
            py: Python<'py>,
            radius: u64,
            number: u64,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::shape::crossing_shell(radius, number, level);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "crossing_tree", signature = (radius, number, keep))]
        pub fn crossing_tree<'py>(
            py: Python<'py>,
            radius: u64,
            number: u64,
            keep: Vec<bool>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::shape::crossing_tree(radius, number, &keep);
            (PySerde(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fit", signature = (shape, extents))]
        pub fn fit<'py>(
            py: Python<'py>,
            shape: PySerde<mrly_math::shape::Shape>,
            extents: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let shape = shape.0;
            let out = mrly_math::shape::fit(&shape, &extents);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "named", signature = (name, dimension, radius))]
        pub fn named<'py>(
            py: Python<'py>,
            name: &str,
            dimension: usize,
            radius: crate::doors::math::shape::Frac,
        ) -> PyResult<Bound<'py, PyAny>> {
            let radius = radius.0;
            let out = mrly_math::shape::named(name, dimension, radius);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "radial_census", signature = (types, centre, r_max))]
        pub fn radial_census<'py>(
            py: Python<'py>,
            types: PyTensor,
            centre: Vec<i64>,
            r_max: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let out = mrly_math::shape::radial_census(&types, &centre, r_max);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "reach", signature = (name, dimension, den))]
        pub fn reach<'py>(
            py: Python<'py>,
            name: &str,
            dimension: usize,
            den: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::shape::reach(name, dimension, den);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "refine", signature = (types, shape, base, extra, keep_cut))]
        pub fn refine<'py>(
            py: Python<'py>,
            types: PyTensor,
            shape: PySerde<mrly_math::shape::Shape>,
            base: usize,
            extra: usize,
            keep_cut: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let types = types.0;
            let shape = shape.0;
            let out = mrly_math::shape::refine(&types, &shape, base, extra, keep_cut);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "regions", signature = (shape, dims))]
        pub fn regions<'py>(
            py: Python<'py>,
            shape: PySerde<mrly_math::shape::Shape>,
            dims: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let shape = shape.0;
            let out = mrly_math::shape::regions(&shape, &dims);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "shapes", signature = (dimension))]
        pub fn shapes<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::shape::shapes(dimension);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.shape")?;
            m.add_class::<Frac>()?;
            m.add_class::<Region>()?;
            m.add_function(wrap_pyfunction!(census, &m)?)?;
            m.add_function(wrap_pyfunction!(classify, &m)?)?;
            m.add_function(wrap_pyfunction!(crop, &m)?)?;
            m.add_function(wrap_pyfunction!(crossing_shell, &m)?)?;
            m.add_function(wrap_pyfunction!(crossing_tree, &m)?)?;
            m.add_function(wrap_pyfunction!(fit, &m)?)?;
            m.add_function(wrap_pyfunction!(named, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_census, &m)?)?;
            m.add_function(wrap_pyfunction!(reach, &m)?)?;
            m.add_function(wrap_pyfunction!(refine, &m)?)?;
            m.add_function(wrap_pyfunction!(regions, &m)?)?;
            m.add_function(wrap_pyfunction!(shapes, &m)?)?;
            m.add("REFINE_LIMIT", mrly_math::shape::REFINE_LIMIT)?;
            let names: Vec<&str> = vec![
                "census",
                "classify",
                "crop",
                "crossing_shell",
                "crossing_tree",
                "fit",
                "named",
                "radial_census",
                "reach",
                "refine",
                "regions",
                "shapes",
                "Frac",
                "Region",
                "REFINE_LIMIT",
            ];
            m.add("__all__", names)?;
            parent.add("shape", &m)?;
            sys.set_item("mrly.math._math.math.shape", &m)?;
            Ok(())
        }
    }

    pub mod six {
        use crate::hand::{
            PyCell2d, PyCell3d, PyCell6d, PyCellNd, PyCode, PyColor, PyImage, PyRgba, PySerde,
            PyTensor, ok,
        };
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        pub mod star {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyclass(name = "Share", module = "mrly.math.six.star", from_py_object)]
            #[derive(Clone)]
            pub struct Share(pub mrly_math::six::star::Share);

            #[pymethods]
            impl Share {
                #[getter]
                #[pyo3(name = "inked")]
                pub fn inked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let value = self.0.inked;
                    (value).into_bound_py_any(py)
                }
                #[setter]
                #[pyo3(name = "inked")]
                pub fn set_inked(&mut self, value: i64) -> PyResult<()> {
                    self.0.inked = value;
                    Ok(())
                }
                #[getter]
                #[pyo3(name = "cells")]
                pub fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let value = self.0.cells;
                    (value).into_bound_py_any(py)
                }
                #[setter]
                #[pyo3(name = "cells")]
                pub fn set_cells(&mut self, value: i64) -> PyResult<()> {
                    self.0.cells = value;
                    Ok(())
                }
                #[pyo3(name = "reduced", signature = ())]
                pub fn reduced<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Share::reduced(self.0);
                    (out).into_bound_py_any(py)
                }
                #[pyo3(name = "value", signature = ())]
                pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Share::value(self.0);
                    (out).into_bound_py_any(py)
                }
                #[staticmethod]
                pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                    Ok(Self(crate::hand::serde_from_py(data)?))
                }
                pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    crate::hand::serde_into_py(py, &self.0)
                }
            }

            #[pyclass(name = "Star", module = "mrly.math.six.star", skip_from_py_object)]
            pub struct Star(pub mrly_math::six::star::Star);

            #[pymethods]
            impl Star {
                #[new]
                #[pyo3(signature = (code))]
                pub fn __new__(code: u128) -> PyResult<Self> {
                    let out = mrly_math::six::star::Star::new(code);
                    Ok(Self(ok(out)?))
                }
                #[pyo3(name = "arm", signature = (number, half))]
                pub fn arm<'py>(
                    &self,
                    py: Python<'py>,
                    number: usize,
                    half: usize,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Star::arm(&self.0, number, half);
                    (crate::doors::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
                }
                #[pyo3(name = "cell", signature = (number, x, z))]
                pub fn cell<'py>(
                    &self,
                    py: Python<'py>,
                    number: usize,
                    x: i64,
                    z: i64,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Star::cell(&self.0, number, x, z);
                    (out).into_bound_py_any(py)
                }
                #[pyo3(name = "excesses", signature = (layers, half))]
                pub fn excesses<'py>(
                    &self,
                    py: Python<'py>,
                    layers: usize,
                    half: usize,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Star::excesses(&self.0, layers, half);
                    (ok(out)?).into_bound_py_any(py)
                }
                #[pyo3(name = "hexagon", signature = (number))]
                pub fn hexagon<'py>(
                    &self,
                    py: Python<'py>,
                    number: usize,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Star::hexagon(&self.0, number);
                    (crate::doors::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
                }
                #[staticmethod]
                #[pyo3(name = "new", signature = (code))]
                pub fn new_<'py>(py: Python<'py>, code: u128) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Star::new(code);
                    (crate::doors::math::six::star::Star(ok(out)?)).into_bound_py_any(py)
                }
                #[staticmethod]
                pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                    Ok(Self(crate::hand::serde_from_py(data)?))
                }
                pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    crate::hand::serde_into_py(py, &self.0)
                }
            }

            #[pyclass(name = "Branch", module = "mrly.math.six.star", skip_from_py_object)]
            pub struct Branch;

            #[pymethods]
            impl Branch {
                #[staticmethod]
                #[pyo3(name = "constant", signature = (branch))]
                pub fn constant<'py>(
                    py: Python<'py>,
                    branch: PySerde<mrly_math::six::star::Branch>,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let branch = branch.0;
                    let out = mrly_math::six::star::Branch::constant(branch);
                    (out).into_bound_py_any(py)
                }
                #[staticmethod]
                #[pyo3(name = "name", signature = (branch))]
                pub fn name<'py>(
                    py: Python<'py>,
                    branch: PySerde<mrly_math::six::star::Branch>,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let branch = branch.0;
                    let out = mrly_math::six::star::Branch::name(branch);
                    (out).into_bound_py_any(py)
                }
                #[staticmethod]
                #[pyo3(name = "of", signature = (layers))]
                pub fn of<'py>(py: Python<'py>, layers: usize) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Branch::of(layers);
                    (PySerde(out)).into_bound_py_any(py)
                }
                #[staticmethod]
                #[pyo3(name = "residual", signature = (branch))]
                pub fn residual<'py>(
                    py: Python<'py>,
                    branch: PySerde<mrly_math::six::star::Branch>,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let branch = branch.0;
                    let out = mrly_math::six::star::Branch::residual(branch);
                    (out).into_bound_py_any(py)
                }
            }

            #[pyfunction]
            #[pyo3(name = "arm_law", signature = (number))]
            pub fn arm_law<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::arm_law(number);
                (crate::doors::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "chi8", signature = (number))]
            pub fn chi8<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::chi8(number);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "constant", signature = ())]
            pub fn constant<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::constant();
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "decay", signature = (excesses, layers))]
            pub fn decay<'py>(
                py: Python<'py>,
                excesses: Vec<f64>,
                layers: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::decay(&excesses, layers);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "width_law", signature = (half))]
            pub fn width_law<'py>(py: Python<'py>, half: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::width_law(half);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.six.star")?;
                m.add_class::<Share>()?;
                m.add_class::<Star>()?;
                m.add_class::<Branch>()?;
                m.add_function(wrap_pyfunction!(arm_law, &m)?)?;
                m.add_function(wrap_pyfunction!(chi8, &m)?)?;
                m.add_function(wrap_pyfunction!(constant, &m)?)?;
                m.add_function(wrap_pyfunction!(decay, &m)?)?;
                m.add_function(wrap_pyfunction!(width_law, &m)?)?;
                let names: Vec<&str> = vec![
                    "arm_law",
                    "chi8",
                    "constant",
                    "decay",
                    "width_law",
                    "Share",
                    "Star",
                    "Branch",
                ];
                m.add("__all__", names)?;
                parent.add("star", &m)?;
                sys.set_item("mrly.math._math.math.six.star", &m)?;
                Ok(())
            }
        }

        #[pyfunction]
        #[pyo3(name = "anti", signature = (cell))]
        pub fn anti<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::anti(cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "binarize", signature = (cell, threshold))]
        pub fn binarize<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            threshold: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::binarize(cell, threshold);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "binarize_otsu", signature = (cell))]
        pub fn binarize_otsu<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::binarize_otsu(cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "blank", signature = (radius, orient, fill, void))]
        pub fn blank<'py>(
            py: Python<'py>,
            radius: usize,
            orient: PySerde<mrly_math::six::Orientation>,
            fill: u8,
            void: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let orient = orient.0;
            let out = mrly_math::six::blank(radius, orient, fill, void);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "blur", signature = (cell, mask, wrap))]
        pub fn blur<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            mask: PyTensor,
            wrap: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrly_math::six::Cell6d::blur(cell, &mask, wrap);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "census", signature = (cell, include_grid))]
        pub fn census<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            include_grid: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::census(&cell, include_grid);
            (PySerde(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "components", signature = (cell))]
        pub fn components<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::components(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "copies", signature = (rings))]
        pub fn copies<'py>(py: Python<'py>, rings: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::copies(rings);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "cut", signature = (cell))]
        pub fn cut<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::cut(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "cut_design", signature = (code, number, level, base))]
        pub fn cut_design<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::six::cut_design(code, number, level, base);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "disc", signature = (cell, rings))]
        pub fn disc<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            rings: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::disc(&cell, rings);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "east", signature = (x, y))]
        pub fn east<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::east(x, y);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell, include_grid))]
        pub fn euler<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            include_grid: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::euler(&cell, include_grid);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::fills(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fills_only", signature = (cell))]
        pub fn fills_only<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::fills_only(&cell);
            (PySerde(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "framed", signature = (cell))]
        pub fn framed<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::framed(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::from_json(text);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "giant", signature = (cell))]
        pub fn giant<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::giant(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "giant_network", signature = (cell))]
        pub fn giant_network<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::giant_network(&cell);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "height", signature = (cell))]
        pub fn height<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::height(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "holes", signature = (cell))]
        pub fn holes<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::holes(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "image", signature = (cell, scale, outline, width))]
        pub fn image<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            scale: usize,
            outline: Option<PyColor>,
            width: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let out = mrly_math::six::image(&cell, scale, outline, width);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "is_cube", signature = (cell))]
        pub fn is_cube<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::is_cube(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "is_hex", signature = (cell))]
        pub fn is_hex<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::is_hex(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "iso", signature = (cell))]
        pub fn iso<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::iso(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "iso_design", signature = (code, number, level, base))]
        pub fn iso_design<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::six::iso_design(code, number, level, base);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "new", signature = (cell, projection, orientation, start))]
        pub fn new<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            projection: PySerde<mrly_math::six::Projection>,
            orientation: PySerde<mrly_math::six::Orientation>,
            start: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let projection = projection.0;
            let orientation = orientation.0;
            let out = mrly_math::six::Cell6d::new(cell, projection, orientation, start);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "north", signature = (x, y))]
        pub fn north<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::north(x, y);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "orientation", signature = (width, height))]
        pub fn orientation<'py>(
            py: Python<'py>,
            width: usize,
            height: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::orientation(width, height);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pad", signature = (cell, k, value))]
        pub fn pad<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            k: usize,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::pad(&cell, k, value);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "paint", signature = (cell, custom=None, mode=None, rng=None))]
        pub fn paint<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            custom: Option<std::collections::HashMap<u8, Vec<PyColor>>>,
            mode: Option<PySerde<mrly_core::Mode>>,
            rng: Option<&Bound<'_, PyAny>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let custom = custom.map(|x| {
                x.into_iter()
                    .map(|(k, v)| (k, v.into_iter().map(|x| x.0).collect::<Vec<_>>()))
                    .collect()
            });
            let mode = mode.map(|x| x.0);
            let mut rng_owned = rng.map(crate::hand::rng_from_py).transpose()?;
            let out = mrly_math::six::paint(cell, custom.as_ref(), mode, rng_owned.as_mut());
            if let (Some(rng), Some(stream)) = (rng, &rng_owned) {
                crate::hand::rng_write_back(rng, stream)?;
            }
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "perforate", signature = (cell, mask, value))]
        pub fn perforate<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            mask: PyTensor,
            value: u8,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrly_math::six::Cell6d::perforate(cell, &mask, value);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pro", signature = (cell))]
        pub fn pro<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::pro(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pro_design", signature = (code, number, level, base))]
        pub fn pro_design<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::six::pro_design(code, number, level, base);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "radial", signature = (cell, radius))]
        pub fn radial<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            radius: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::radial(&cell, radius);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "radial_crop", signature = (cell, radius, size))]
        pub fn radial_crop<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            radius: usize,
            size: (usize, usize),
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::radial_crop(&cell, radius, size);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "radial_mask", signature = (radius, orient))]
        pub fn radial_mask<'py>(
            py: Python<'py>,
            radius: usize,
            orient: PySerde<mrly_math::six::Orientation>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let orient = orient.0;
            let out = mrly_math::six::radial_mask(radius, orient);
            (PyTensor(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "raster", signature = (cell, size))]
        pub fn raster<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::raster(&cell, size);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "rect_image", signature = (cell, scale, start=None))]
        pub fn rect_image<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            scale: usize,
            start: Option<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::rect_image(&cell, scale, start);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "rect_svg", signature = (cell, scale, start=None))]
        pub fn rect_svg<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            scale: usize,
            start: Option<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::rect_svg(&cell, scale, start);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "rim_holes", signature = (cell))]
        pub fn rim_holes<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::rim_holes(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "skin", signature = (cell))]
        pub fn skin<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::skin(&cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "slice_core_graph", signature = (cell))]
        pub fn slice_core_graph<'py>(
            py: Python<'py>,
            cell: PyCell6d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::slice_core_graph(&cell);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "slice_dual_graph", signature = (cell))]
        pub fn slice_dual_graph<'py>(
            py: Python<'py>,
            cell: PyCell6d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::slice_dual_graph(&cell);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "slice_edge_graph", signature = (cell, value=None))]
        pub fn slice_edge_graph<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            value: Option<u8>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::slice_edge_graph(&cell, value);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "slice_tunnel_graph", signature = (cell))]
        pub fn slice_tunnel_graph<'py>(
            py: Python<'py>,
            cell: PyCell6d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::slice_tunnel_graph(&cell);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "south", signature = (x, y))]
        pub fn south<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::south(x, y);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "spectral_exponent", signature = (cell, window))]
        pub fn spectral_exponent<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            window: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::spectral_exponent(&cell, window);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "svg", signature = (cell, scale, outline, width, start=None))]
        pub fn svg<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            scale: usize,
            outline: Option<PyColor>,
            width: usize,
            start: Option<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let out = mrly_math::six::svg(&cell, scale, outline, width, start);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tessellate", signature = (cell, mask))]
        pub fn tessellate<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            mask: PyTensor,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let mask = mask.0;
            let out = mrly_math::six::tessellate(&cell, &mask);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tile", signature = (cell, width, height))]
        pub fn tile<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            width: usize,
            height: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::tile(&cell, width, height);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tile_cell", signature = (cell, width, height, crop))]
        pub fn tile_cell<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            width: usize,
            height: usize,
            crop: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::tile_cell(&cell, width, height, crop);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tile_crop", signature = (cell, size))]
        pub fn tile_crop<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            size: (usize, usize),
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::tile_crop(&cell, size);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tile_step", signature = (size))]
        pub fn tile_step<'py>(
            py: Python<'py>,
            size: (usize, usize),
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::tile_step(size);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "triangles", signature = (cell, start=None))]
        pub fn triangles<'py>(
            py: Python<'py>,
            cell: PyCell6d,
            start: Option<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::triangles(&cell, start);
            ((ok(out)?)
                .into_iter()
                .map(|x| {
                    let t = x;
                    (t.0, PyRgba(t.1))
                })
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "west", signature = (x, y))]
        pub fn west<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::west(x, y);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "width", signature = (cell))]
        pub fn width<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::width(&cell);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.six")?;
            m.add_function(wrap_pyfunction!(anti, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize, &m)?)?;
            m.add_function(wrap_pyfunction!(binarize_otsu, &m)?)?;
            m.add_function(wrap_pyfunction!(blank, &m)?)?;
            m.add_function(wrap_pyfunction!(blur, &m)?)?;
            m.add_function(wrap_pyfunction!(census, &m)?)?;
            m.add_function(wrap_pyfunction!(components, &m)?)?;
            m.add_function(wrap_pyfunction!(copies, &m)?)?;
            m.add_function(wrap_pyfunction!(cut, &m)?)?;
            m.add_function(wrap_pyfunction!(cut_design, &m)?)?;
            m.add_function(wrap_pyfunction!(disc, &m)?)?;
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
            m.add_function(wrap_pyfunction!(image, &m)?)?;
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
            m.add_function(wrap_pyfunction!(pro, &m)?)?;
            m.add_function(wrap_pyfunction!(pro_design, &m)?)?;
            m.add_function(wrap_pyfunction!(radial, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_crop, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_mask, &m)?)?;
            m.add_function(wrap_pyfunction!(raster, &m)?)?;
            m.add_function(wrap_pyfunction!(rect_image, &m)?)?;
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
            m.add("FILL", mrly_math::six::FILL)?;
            m.add("GRID", mrly_math::six::GRID)?;
            m.add("LEFT", mrly_math::six::LEFT)?;
            m.add("RIGHT", mrly_math::six::RIGHT)?;
            m.add("UP", mrly_math::six::UP)?;
            m.add("VOID", mrly_math::six::VOID)?;
            let names: Vec<&str> = vec![
                "anti",
                "binarize",
                "binarize_otsu",
                "blank",
                "blur",
                "census",
                "components",
                "copies",
                "cut",
                "cut_design",
                "disc",
                "east",
                "euler",
                "fills",
                "fills_only",
                "framed",
                "from_json",
                "giant",
                "giant_network",
                "height",
                "holes",
                "image",
                "is_cube",
                "is_hex",
                "iso",
                "iso_design",
                "new",
                "north",
                "orientation",
                "pad",
                "paint",
                "perforate",
                "pro",
                "pro_design",
                "radial",
                "radial_crop",
                "radial_mask",
                "raster",
                "rect_image",
                "rect_svg",
                "rim_holes",
                "skin",
                "slice_core_graph",
                "slice_dual_graph",
                "slice_edge_graph",
                "slice_tunnel_graph",
                "south",
                "spectral_exponent",
                "svg",
                "tessellate",
                "tile",
                "tile_cell",
                "tile_crop",
                "tile_step",
                "to_json",
                "triangles",
                "west",
                "width",
                "FILL",
                "GRID",
                "LEFT",
                "RIGHT",
                "UP",
                "VOID",
            ];
            m.add("__all__", names)?;
            star::init(py, &m, sys)?;
            parent.add("six", &m)?;
            sys.set_item("mrly.math._math.math.six", &m)?;
            Ok(())
        }
    }

    pub mod spectrum {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "clusters", signature = (eigenvalues, tolerance))]
        pub fn clusters<'py>(
            py: Python<'py>,
            eigenvalues: Vec<f64>,
            tolerance: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::clusters(&eigenvalues, tolerance);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "laplacian", signature = (network, normalised))]
        pub fn laplacian<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
            normalised: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::laplacian(&network.0, normalised);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "laplacian_spectrum", signature = (network, normalised))]
        pub fn laplacian_spectrum<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
            normalised: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::laplacian_spectrum(&network.0, normalised);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "multiplicity", signature = (eigenvalues, value, tolerance))]
        pub fn multiplicity<'py>(
            py: Python<'py>,
            eigenvalues: Vec<f64>,
            value: f64,
            tolerance: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::multiplicity(&eigenvalues, value, tolerance);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "spectral_exponent", signature = (eigenvalues, window))]
        pub fn spectral_exponent<'py>(
            py: Python<'py>,
            eigenvalues: Vec<f64>,
            window: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::spectral_exponent(&eigenvalues, window);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "spectral_fit", signature = (eigenvalues, window))]
        pub fn spectral_fit<'py>(
            py: Python<'py>,
            eigenvalues: Vec<f64>,
            window: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::spectral_fit(&eigenvalues, window);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "spectral_points", signature = (eigenvalues))]
        pub fn spectral_points<'py>(
            py: Python<'py>,
            eigenvalues: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::spectral_points(&eigenvalues);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "symmetric_eigenvalues", signature = (matrix))]
        pub fn symmetric_eigenvalues<'py>(
            py: Python<'py>,
            matrix: Vec<Vec<f64>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::symmetric_eigenvalues(&matrix);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.spectrum")?;
            m.add_function(wrap_pyfunction!(clusters, &m)?)?;
            m.add_function(wrap_pyfunction!(laplacian, &m)?)?;
            m.add_function(wrap_pyfunction!(laplacian_spectrum, &m)?)?;
            m.add_function(wrap_pyfunction!(multiplicity, &m)?)?;
            m.add_function(wrap_pyfunction!(spectral_exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(spectral_fit, &m)?)?;
            m.add_function(wrap_pyfunction!(spectral_points, &m)?)?;
            m.add_function(wrap_pyfunction!(symmetric_eigenvalues, &m)?)?;
            let names: Vec<&str> = vec![
                "clusters",
                "laplacian",
                "laplacian_spectrum",
                "multiplicity",
                "spectral_exponent",
                "spectral_fit",
                "spectral_points",
                "symmetric_eigenvalues",
            ];
            m.add("__all__", names)?;
            parent.add("spectrum", &m)?;
            sys.set_item("mrly.math._math.math.spectrum", &m)?;
            Ok(())
        }
    }

    pub mod spin {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Blend", module = "mrly.math.spin", skip_from_py_object)]
        pub struct Blend;

        #[pymethods]
        impl Blend {
            #[staticmethod]
            #[pyo3(name = "fold", signature = (blend, values))]
            pub fn fold<'py>(
                py: Python<'py>,
                blend: PySerde<mrly_math::spin::Blend>,
                values: Vec<f32>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let blend = blend.0;
                let out = mrly_math::spin::Blend::fold(blend, &values);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "named", signature = (name))]
            pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::spin::Blend::named(name);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "arcs", signature = (data, size, radius))]
        pub fn arcs<'py>(
            py: Python<'py>,
            data: Vec<f32>,
            size: usize,
            radius: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::arcs(&data, size, radius);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "harmonics", signature = (data, size, rings, orders))]
        pub fn harmonics<'py>(
            py: Python<'py>,
            data: Vec<f32>,
            size: usize,
            rings: usize,
            orders: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::harmonics(&data, size, rings, orders);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mass", signature = (profile, size))]
        pub fn mass<'py>(
            py: Python<'py>,
            profile: Vec<f32>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::mass(&profile, size);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mass_within", signature = (profile, size, radius))]
        pub fn mass_within<'py>(
            py: Python<'py>,
            profile: Vec<f32>,
            size: usize,
            radius: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::mass_within(&profile, size, radius);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "petals", signature = (copies, order))]
        pub fn petals<'py>(
            py: Python<'py>,
            copies: usize,
            order: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::petals(copies, order);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "profile", signature = (data, size, steps))]
        pub fn profile<'py>(
            py: Python<'py>,
            data: Vec<f32>,
            size: usize,
            steps: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::profile(&data, size, steps);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "radial", signature = (data, size, out, copies, step, blend, samples))]
        pub fn radial<'py>(
            py: Python<'py>,
            data: Vec<f32>,
            size: usize,
            out: usize,
            copies: usize,
            step: f64,
            blend: PySerde<mrly_math::spin::Blend>,
            samples: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let blend = blend.0;
            let out = mrly_math::spin::radial(&data, size, out, copies, step, blend, samples);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "reach", signature = (size))]
        pub fn reach<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::reach(size);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ring", signature = (data, size, radius))]
        pub fn ring<'py>(
            py: Python<'py>,
            data: Vec<f32>,
            size: usize,
            radius: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::ring(&data, size, radius);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "turns", signature = (power))]
        pub fn turns<'py>(py: Python<'py>, power: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::turns(&power);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "wheel", signature = (profile, size))]
        pub fn wheel<'py>(
            py: Python<'py>,
            profile: Vec<f32>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::wheel(&profile, size);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.spin")?;
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
            let names: Vec<&str> = vec![
                "arcs",
                "harmonics",
                "mass",
                "mass_within",
                "petals",
                "profile",
                "radial",
                "reach",
                "ring",
                "turns",
                "wheel",
                "Blend",
            ];
            m.add("__all__", names)?;
            parent.add("spin", &m)?;
            sys.set_item("mrly.math._math.math.spin", &m)?;
            Ok(())
        }
    }

    pub mod spirograph {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Seats", module = "mrly.math.spirograph", skip_from_py_object)]
        pub struct Seats;

        #[pymethods]
        impl Seats {
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::spirograph::Seats::default();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "cell", signature = (width, height, reach))]
        pub fn cell<'py>(
            py: Python<'py>,
            width: usize,
            height: usize,
            reach: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spirograph::cell(width, height, reach);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "cover", signature = (track, pencils, exact, samples, side))]
        pub fn cover<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
            exact: bool,
            samples: usize,
            side: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::cover(&track, &pencils, exact, samples, side);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "disc", signature = (track, pencils))]
        pub fn disc<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::disc(&track, &pencils);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "distinct", signature = (track, pencils, exact))]
        pub fn distinct<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
            exact: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::distinct(&track, &pencils, exact);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "frame", signature = (track, pencils))]
        pub fn frame<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::frame(&track, &pencils);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "nodes", signature = (track, pencils, exact))]
        pub fn nodes<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
            exact: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::nodes(&track, &pencils, exact);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pencils", signature = (types, width, height, mode, reach, jitter, seed))]
        pub fn pencils<'py>(
            py: Python<'py>,
            types: Vec<u8>,
            width: usize,
            height: usize,
            mode: &str,
            reach: f64,
            jitter: f64,
            seed: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out =
                mrly_math::spirograph::pencils(&types, width, height, mode, reach, jitter, seed);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "point", signature = (track, pencil, s))]
        pub fn point<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencil: PySerde<mrly_math::spirograph::Pencil>,
            s: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencil = pencil.0;
            let out = mrly_math::spirograph::point(&track, &pencil, s);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pose", signature = (track, s))]
        pub fn pose<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            s: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let out = mrly_math::spirograph::pose(&track, s);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "representatives", signature = (track, pencils, exact))]
        pub fn representatives<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
            exact: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::representatives(&track, &pencils, exact);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "seats", signature = (pencils))]
        pub fn seats<'py>(
            py: Python<'py>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::seats(&pencils);
            (PySerde(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "signed_area", signature = (track, pencil))]
        pub fn signed_area<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencil: PySerde<mrly_math::spirograph::Pencil>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencil = pencil.0;
            let out = mrly_math::spirograph::signed_area(&track, &pencil);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "trace", signature = (track, pencils, samples))]
        pub fn trace<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            pencils: Vec<PySerde<mrly_math::spirograph::Pencil>>,
            samples: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let pencils = pencils.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::spirograph::trace(&track, &pencils, samples);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "track", signature = (kind, ring, wheel, sides, laps))]
        pub fn track<'py>(
            py: Python<'py>,
            kind: &str,
            ring: usize,
            wheel: usize,
            sides: usize,
            laps: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spirograph::track(kind, ring, wheel, sides, laps);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "turn", signature = (track, s))]
        pub fn turn<'py>(
            py: Python<'py>,
            track: PySerde<mrly_math::spirograph::Track>,
            s: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let track = track.0;
            let out = mrly_math::spirograph::turn(&track, s);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.spirograph")?;
            m.add_class::<Seats>()?;
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
            m.add("LAPS_CAP", mrly_math::spirograph::LAPS_CAP)?;
            m.add("PENCIL_CAP", mrly_math::spirograph::PENCIL_CAP)?;
            m.add("POINT_CAP", mrly_math::spirograph::POINT_CAP)?;
            m.add("RADIUS_CAP", mrly_math::spirograph::RADIUS_CAP)?;
            m.add("RASTER_CAP", mrly_math::spirograph::RASTER_CAP)?;
            m.add("SIDES", mrly_math::spirograph::SIDES)?;
            let names: Vec<&str> = vec![
                "cell",
                "cover",
                "disc",
                "distinct",
                "frame",
                "nodes",
                "pencils",
                "point",
                "pose",
                "representatives",
                "seats",
                "signed_area",
                "trace",
                "track",
                "turn",
                "Seats",
                "LAPS_CAP",
                "PENCIL_CAP",
                "POINT_CAP",
                "RADIUS_CAP",
                "RASTER_CAP",
                "SIDES",
            ];
            m.add("__all__", names)?;
            parent.add("spirograph", &m)?;
            sys.set_item("mrly.math._math.math.spirograph", &m)?;
            Ok(())
        }
    }

    pub mod sumset {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Pair", module = "mrly.math.sumset", from_py_object)]
        #[derive(Clone)]
        pub struct Pair(pub mrly_math::sumset::Pair);

        #[pymethods]
        impl Pair {
            #[new]
            #[pyo3(signature = (three, four))]
            pub fn __new__(three: u32, four: u32) -> PyResult<Self> {
                let out = mrly_math::sumset::Pair::new(three, four);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "clean", signature = ())]
            pub fn clean<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::clean(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "copy", signature = ())]
            pub fn copy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::copy(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "energy", signature = ())]
            pub fn energy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::energy(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "four", signature = ())]
            pub fn four<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::four(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "gap", signature = ())]
            pub fn gap<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::gap(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "largest", signature = ())]
            pub fn largest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::largest(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (three, four))]
            pub fn new_<'py>(
                py: Python<'py>,
                three: u32,
                four: u32,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::new(three, four);
                (crate::doors::math::sumset::Pair(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "ratio", signature = (energy))]
            pub fn ratio<'py>(&self, py: Python<'py>, energy: u128) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::ratio(&self.0, energy);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "scale", signature = ())]
            pub fn scale<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::scale(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "three", signature = ())]
            pub fn three<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::three(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Sumset", module = "mrly.math.sumset", skip_from_py_object)]
        pub struct Sumset(pub mrly_math::sumset::Sumset);

        #[pymethods]
        impl Sumset {
            #[new]
            #[pyo3(signature = (level))]
            pub fn __new__(level: u32) -> PyResult<Self> {
                let out = mrly_math::sumset::Sumset::new(level);
                Ok(Self(ok(out)?))
            }
            #[pyo3(name = "contains", signature = (x))]
            pub fn contains<'py>(&self, py: Python<'py>, x: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::contains(&self.0, x);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "count", signature = (x))]
            pub fn count<'py>(&self, py: Python<'py>, x: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::count(&self.0, x);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "density", signature = (x))]
            pub fn density<'py>(&self, py: Python<'py>, x: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::density(&self.0, x);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "extremes", signature = (edges))]
            pub fn extremes<'py>(
                &self,
                py: Python<'py>,
                edges: Vec<u64>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::extremes(&self.0, &edges);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "fills", signature = (low, high, cells))]
            pub fn fills<'py>(
                &self,
                py: Python<'py>,
                low: u64,
                high: u64,
                cells: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::fills(&self.0, low, high, cells);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "level", signature = ())]
            pub fn level<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::level(&self.0);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (level))]
            pub fn new_<'py>(py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::new(level);
                (crate::doors::math::sumset::Sumset(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "top", signature = ())]
            pub fn top<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::top(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "pairs", signature = (level))]
        pub fn pairs<'py>(py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::sumset::pairs(level);
            ((ok(out)?)
                .into_iter()
                .map(crate::doors::math::sumset::Pair)
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.sumset")?;
            m.add_class::<Pair>()?;
            m.add_class::<Sumset>()?;
            m.add_function(wrap_pyfunction!(pairs, &m)?)?;
            m.add("DEEPEST", mrly_math::sumset::DEEPEST)?;
            m.add("WIDEST", mrly_math::sumset::WIDEST)?;
            let names: Vec<&str> = vec!["pairs", "Pair", "Sumset", "DEEPEST", "WIDEST"];
            m.add("__all__", names)?;
            parent.add("sumset", &m)?;
            sys.set_item("mrly.math._math.math.sumset", &m)?;
            Ok(())
        }
    }

    pub mod three {
        use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyCode, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::exceptions::PyValueError;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        pub mod sponge {
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "deep", signature = (radius))]
            pub fn deep<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::deep(radius);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "dimension", signature = ())]
            pub fn dimension<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::dimension();
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "distance", signature = (point))]
            pub fn distance<'py>(py: Python<'py>, point: [f64; 3]) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::distance(point);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "exact", signature = (radius))]
            pub fn exact<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::exact(radius);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "profile", signature = (radius))]
            pub fn profile<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::profile(radius);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "reading", signature = (radius))]
            pub fn reading<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::reading(radius);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "tube", signature = (radius))]
            pub fn tube<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::tube(radius);
                (out).into_bound_py_any(py)
            }

            #[pyfunction]
            #[pyo3(name = "volume", signature = (radius))]
            pub fn volume<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::volume(radius);
                (out).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.three.sponge")?;
                m.add_function(wrap_pyfunction!(deep, &m)?)?;
                m.add_function(wrap_pyfunction!(dimension, &m)?)?;
                m.add_function(wrap_pyfunction!(distance, &m)?)?;
                m.add_function(wrap_pyfunction!(exact, &m)?)?;
                m.add_function(wrap_pyfunction!(profile, &m)?)?;
                m.add_function(wrap_pyfunction!(reading, &m)?)?;
                m.add_function(wrap_pyfunction!(tube, &m)?)?;
                m.add_function(wrap_pyfunction!(volume, &m)?)?;
                m.add("COVER", mrly_math::three::sponge::COVER)?;
                m.add("EDGE", mrly_math::three::sponge::EDGE)?;
                let names: Vec<&str> = vec![
                    "deep",
                    "dimension",
                    "distance",
                    "exact",
                    "profile",
                    "reading",
                    "tube",
                    "volume",
                    "COVER",
                    "EDGE",
                ];
                m.add("__all__", names)?;
                parent.add("sponge", &m)?;
                sys.set_item("mrly.math._math.math.three.sponge", &m)?;
                Ok(())
            }
        }

        #[pyclass(name = "Vec3", module = "mrly.math.three", from_py_object)]
        #[derive(Clone)]
        pub struct Vec3(pub mrly_math::three::Vec3);

        #[pymethods]
        impl Vec3 {
            #[new]
            #[pyo3(signature = (x, y, z))]
            pub fn __new__(x: f32, y: f32, z: f32) -> PyResult<Self> {
                let out = mrly_math::three::Vec3::new(x, y, z);
                Ok(Self(out))
            }
            #[getter]
            #[pyo3(name = "x")]
            pub fn x<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.x;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "x")]
            pub fn set_x(&mut self, value: f32) -> PyResult<()> {
                self.0.x = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "y")]
            pub fn y<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.y;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "y")]
            pub fn set_y(&mut self, value: f32) -> PyResult<()> {
                self.0.y = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "z")]
            pub fn z<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.z;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "z")]
            pub fn set_z(&mut self, value: f32) -> PyResult<()> {
                self.0.z = value;
                Ok(())
            }
            #[pyo3(name = "cross", signature = (o))]
            pub fn cross<'py>(
                &self,
                py: Python<'py>,
                o: crate::doors::math::three::Vec3,
            ) -> PyResult<Bound<'py, PyAny>> {
                let o = o.0;
                let out = mrly_math::three::Vec3::cross(self.0, o);
                (crate::doors::math::three::Vec3(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "dot", signature = (o))]
            pub fn dot<'py>(
                &self,
                py: Python<'py>,
                o: crate::doors::math::three::Vec3,
            ) -> PyResult<Bound<'py, PyAny>> {
                let o = o.0;
                let out = mrly_math::three::Vec3::dot(self.0, o);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (x, y, z))]
            pub fn new_<'py>(
                py: Python<'py>,
                x: f32,
                y: f32,
                z: f32,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::Vec3::new(x, y, z);
                (crate::doors::math::three::Vec3(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "scale", signature = (s))]
            pub fn scale<'py>(&self, py: Python<'py>, s: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::Vec3::scale(self.0, s);
                (crate::doors::math::three::Vec3(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyfunction]
        #[pyo3(name = "carpet", signature = (number, level))]
        pub fn carpet<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::carpet(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "census", signature = (cell))]
        pub fn census<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::census(&cell);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "core_graph", signature = (cell))]
        pub fn core_graph<'py>(
            py: Python<'py>,
            cell: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let out = mrly_math::three::core_graph::<2>(&cell);
                    (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let out = mrly_math::three::core_graph::<3>(&cell);
                    (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!(
                    "core_graph wants a 2d or 3d argument, got {other}d."
                ))),
            }
        }

        #[pyfunction]
        #[pyo3(name = "create", signature = (code, number, level, base))]
        pub fn create<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::three::create(code, number, level, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "diagonal_slice", signature = (code, number, level, base, height))]
        pub fn diagonal_slice<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            base: usize,
            height: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::three::diagonal_slice(code, number, level, base, height);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "diagonal_svg", signature = (code, number, level, base, heights, scale))]
        pub fn diagonal_svg<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            base: usize,
            heights: Vec<usize>,
            scale: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::three::diagonal_svg(code, number, level, base, &heights, scale);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "dust", signature = (number, level))]
        pub fn dust<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::dust(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "edge_graph", signature = (cell))]
        pub fn edge_graph<'py>(
            py: Python<'py>,
            cell: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let out = mrly_math::three::edge_graph::<2>(&cell);
                    (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let out = mrly_math::three::edge_graph::<3>(&cell);
                    (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!(
                    "edge_graph wants a 2d or 3d argument, got {other}d."
                ))),
            }
        }

        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell))]
        pub fn euler<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::euler(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "extrude", signature = (cell, axis, depth))]
        pub fn extrude<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            axis: usize,
            depth: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::extrude(&cell, axis, depth);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "extrude_cube", signature = (cell, depth))]
        pub fn extrude_cube<'py>(
            py: Python<'py>,
            cell: PyCell3d,
            depth: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::extrude_cube(&cell, depth);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "faces", signature = (cell))]
        pub fn faces<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::faces(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::fills(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "from_corners", signature = (corners, number, level, base))]
        pub fn from_corners<'py>(
            py: Python<'py>,
            corners: Vec<Vec<u8>>,
            number: usize,
            level: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::from_corners(&corners, number, level, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::from_json(text);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "from_strings", signature = (data))]
        pub fn from_strings<'py>(
            py: Python<'py>,
            data: Vec<Vec<String>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::from_strings(&data);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "hidden", signature = (cell))]
        pub fn hidden<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::hidden(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "level_set", signature = (number, levels, level, base))]
        pub fn level_set<'py>(
            py: Python<'py>,
            number: usize,
            levels: Vec<usize>,
            level: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::level_set(number, &levels, level, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "magic", signature = (cells))]
        pub fn magic<'py>(
            py: Python<'py>,
            cells: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cells)?;
            match dim {
                2 => {
                    let cells = cells.extract::<Vec<PyCell2d>>()?;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrly_math::three::magic::<2>(&cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cells = cells.extract::<Vec<PyCell3d>>()?;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrly_math::three::magic::<3>(&cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!(
                    "magic wants a 2d or 3d argument, got {other}d."
                ))),
            }
        }

        #[pyfunction]
        #[pyo3(name = "manhattan_layers", signature = (cell))]
        pub fn manhattan_layers<'py>(
            py: Python<'py>,
            cell: PyCell3d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::manhattan_layers(cell);
            (PyCellNd(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "merge", signature = (cells, width, height, depth))]
        pub fn merge<'py>(
            py: Python<'py>,
            cells: Vec<PyCell3d>,
            width: usize,
            height: usize,
            depth: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::three::merge(&cells, width, height, depth);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mosaic", signature = (mask, cells))]
        pub fn mosaic<'py>(
            py: Python<'py>,
            mask: PyTensor,
            cells: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cells)?;
            match dim {
                2 => {
                    let cells = cells.extract::<Vec<PyCell2d>>()?;
                    let mask = mask.0;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrly_math::three::mosaic::<2>(&mask, &cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cells = cells.extract::<Vec<PyCell3d>>()?;
                    let mask = mask.0;
                    let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
                    let out = mrly_math::three::mosaic::<3>(&mask, &cells);
                    (PyCellNd(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!(
                    "mosaic wants a 2d or 3d argument, got {other}d."
                ))),
            }
        }

        #[pyfunction]
        #[pyo3(name = "named", signature = (design, number, level))]
        pub fn named<'py>(
            py: Python<'py>,
            design: PySerde<mrly_math::bang::catalog::Design>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrly_math::three::named(design, number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "net", signature = (number, level))]
        pub fn net<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::net(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "noise", signature = (number, level, density, rng))]
        pub fn noise<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
            density: f64,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_math::three::noise(number, level, density, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ones", signature = (number, level))]
        pub fn ones<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::ones(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "orientations", signature = ())]
        pub fn orientations<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::orientations();
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "point", signature = (number, level))]
        pub fn point<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::point(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "profile", signature = (code, number, level, base))]
        pub fn profile<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::three::profile(code, number, level, base);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "project", signature = (point))]
        pub fn project<'py>(py: Python<'py>, point: [u32; 3]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::project(point);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "quads", signature = (cell))]
        pub fn quads<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::quads(&cell);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "shadow", signature = (point))]
        pub fn shadow<'py>(py: Python<'py>, point: [u32; 3]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::shadow(point);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "slice", signature = (cell, axis, index))]
        pub fn slice<'py>(
            py: Python<'py>,
            cell: PyCell3d,
            axis: usize,
            index: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::slice(&cell, axis, index);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "special", signature = (mask, cell))]
        pub fn special<'py>(
            py: Python<'py>,
            mask: PyTensor,
            cell: PyCell3d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let cell = cell.0;
            let out = mrly_math::three::special(&mask, &cell);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "star", signature = (number, level))]
        pub fn star<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::star(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "support", signature = (counts))]
        pub fn support<'py>(py: Python<'py>, counts: Vec<u128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::support(&counts);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "surface", signature = (cell))]
        pub fn surface<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::surface(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "text", signature = (cell, glyphs=None))]
        pub fn text<'py>(
            py: Python<'py>,
            cell: PyCell3d,
            glyphs: Option<std::collections::HashMap<u8, String>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let glyphs = glyphs.map(|x| x.into_iter().collect());
            let out = mrly_math::three::text(&cell, glyphs.as_ref());
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "to_obj", signature = (cell))]
        pub fn to_obj<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::to_obj(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "to_strings", signature = (cell))]
        pub fn to_strings<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::to_strings(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "tunnel_graph", signature = (cell))]
        pub fn tunnel_graph<'py>(
            py: Python<'py>,
            cell: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let dim = crate::hand::ndim(cell)?;
            match dim {
                2 => {
                    let cell = cell.extract::<PyCell2d>()?;
                    let cell = cell.0;
                    let out = mrly_math::three::tunnel_graph::<2>(&cell);
                    (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                3 => {
                    let cell = cell.extract::<PyCell3d>()?;
                    let cell = cell.0;
                    let out = mrly_math::three::tunnel_graph::<3>(&cell);
                    (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
                }
                other => Err(PyValueError::new_err(format!(
                    "tunnel_graph wants a 2d or 3d argument, got {other}d."
                ))),
            }
        }

        #[pyfunction]
        #[pyo3(name = "void", signature = (number, level))]
        pub fn void<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::void(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "voids", signature = (cell))]
        pub fn voids<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::voids(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "volume", signature = (cell))]
        pub fn volume<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::volume(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "wires", signature = (cell))]
        pub fn wires<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::wires(&cell);
            ((out)
                .into_iter()
                .map(|x| {
                    (x).into_iter()
                        .map(crate::doors::math::three::Vec3)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "xline", signature = (number, level))]
        pub fn xline<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::xline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "xtree", signature = (number, level))]
        pub fn xtree<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::xtree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "yline", signature = (number, level))]
        pub fn yline<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::yline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ytree", signature = (number, level))]
        pub fn ytree<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::ytree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zeros", signature = (number, level))]
        pub fn zeros<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::zeros(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zline", signature = (number, level))]
        pub fn zline<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::zline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ztree", signature = (number, level))]
        pub fn ztree<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::ztree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.three")?;
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
            let names: Vec<&str> = vec![
                "carpet",
                "census",
                "core_graph",
                "create",
                "diagonal_slice",
                "diagonal_svg",
                "dust",
                "edge_graph",
                "euler",
                "extrude",
                "extrude_cube",
                "faces",
                "fills",
                "from_corners",
                "from_json",
                "from_strings",
                "hidden",
                "level_set",
                "magic",
                "manhattan_layers",
                "merge",
                "mosaic",
                "named",
                "net",
                "noise",
                "ones",
                "orientations",
                "point",
                "profile",
                "project",
                "quads",
                "shadow",
                "slice",
                "special",
                "star",
                "support",
                "surface",
                "text",
                "to_json",
                "to_obj",
                "to_strings",
                "tunnel_graph",
                "void",
                "voids",
                "volume",
                "wires",
                "xline",
                "xtree",
                "yline",
                "ytree",
                "zeros",
                "zline",
                "ztree",
                "Vec3",
            ];
            m.add("__all__", names)?;
            sponge::init(py, &m, sys)?;
            parent.add("three", &m)?;
            sys.set_item("mrly.math._math.math.three", &m)?;
            Ok(())
        }
    }

    pub mod tourbillon {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "eyes", signature = (qmax))]
        pub fn eyes<'py>(py: Python<'py>, qmax: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::tourbillon::eyes(qmax);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "field", signature = (top, size, schedule, increment, set, weights, mode, blend, seed))]
        pub fn field<'py>(
            py: Python<'py>,
            top: usize,
            size: usize,
            schedule: &str,
            increment: f64,
            set: &str,
            weights: &str,
            mode: &str,
            blend: &str,
            seed: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::tourbillon::field(
                top, size, schedule, increment, set, weights, mode, blend, seed,
            );
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "layers", signature = (top, schedule, increment, set, weights, seed))]
        pub fn layers<'py>(
            py: Python<'py>,
            top: usize,
            schedule: &str,
            increment: f64,
            set: &str,
            weights: &str,
            seed: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::tourbillon::layers(top, schedule, increment, set, weights, seed);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "period", signature = (increment))]
        pub fn period<'py>(py: Python<'py>, increment: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::tourbillon::period(increment);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "sharing", signature = (list))]
        pub fn sharing<'py>(
            py: Python<'py>,
            list: Vec<PySerde<mrly_math::tourbillon::Layer>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let list = list.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::tourbillon::sharing(&list);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "stack", signature = (list, size, mode, blend))]
        pub fn stack<'py>(
            py: Python<'py>,
            list: Vec<PySerde<mrly_math::tourbillon::Layer>>,
            size: usize,
            mode: &str,
            blend: PySerde<mrly_math::spin::Blend>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let list = list.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let blend = blend.0;
            let out = mrly_math::tourbillon::stack(&list, size, mode, blend);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "stats", signature = (field, size, top, schedule, increment, set, weights, blend, seed))]
        pub fn stats<'py>(
            py: Python<'py>,
            field: Vec<f32>,
            size: usize,
            top: usize,
            schedule: &str,
            increment: f64,
            set: &str,
            weights: &str,
            blend: &str,
            seed: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::tourbillon::stats(
                &field, size, top, schedule, increment, set, weights, blend, seed,
            );
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.tourbillon")?;
            m.add_function(wrap_pyfunction!(eyes, &m)?)?;
            m.add_function(wrap_pyfunction!(field, &m)?)?;
            m.add_function(wrap_pyfunction!(layers, &m)?)?;
            m.add_function(wrap_pyfunction!(period, &m)?)?;
            m.add_function(wrap_pyfunction!(sharing, &m)?)?;
            m.add_function(wrap_pyfunction!(stack, &m)?)?;
            m.add_function(wrap_pyfunction!(stats, &m)?)?;
            let names: Vec<&str> = vec![
                "eyes", "field", "layers", "period", "sharing", "stack", "stats",
            ];
            m.add("__all__", names)?;
            parent.add("tourbillon", &m)?;
            sys.set_item("mrly.math._math.math.tourbillon", &m)?;
            Ok(())
        }
    }

    pub mod two {
        use crate::hand::{PyCell2d, PyCellNd, PyCode, PyColor, PyImage, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        pub mod payload {
            use crate::hand::PyTensor;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            #[pyfunction]
            #[pyo3(name = "frame", signature = ())]
            pub fn frame<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::two::payload::frame();
                (PyTensor(out)).into_bound_py_any(py)
            }

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.two.payload")?;
                m.add_function(wrap_pyfunction!(frame, &m)?)?;
                let names: Vec<&str> = vec!["frame"];
                m.add("__all__", names)?;
                parent.add("payload", &m)?;
                sys.set_item("mrly.math._math.math.two.payload", &m)?;
                Ok(())
            }
        }

        #[pyfunction]
        #[pyo3(name = "capacity", signature = (cell))]
        pub fn capacity<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::capacity(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "carpet", signature = (number, level))]
        pub fn carpet<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::carpet(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "census", signature = (cell))]
        pub fn census<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::census(&cell);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "create", signature = (code, number, level, rotation, base))]
        pub fn create<'py>(
            py: Python<'py>,
            code: PyCode,
            number: usize,
            level: usize,
            rotation: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let code = code.0;
            let out = mrly_math::two::create(code, number, level, rotation, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "dust", signature = (number, level))]
        pub fn dust<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::dust(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "embed", signature = (cell, payload))]
        pub fn embed<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            payload: Vec<u8>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::embed(&cell, &payload);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell))]
        pub fn euler<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::euler(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "extract", signature = (carrier, carried))]
        pub fn extract<'py>(
            py: Python<'py>,
            carrier: PyCell2d,
            carried: PyCell2d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let carrier = carrier.0;
            let carried = carried.0;
            let out = mrly_math::two::extract(&carrier, &carried);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::fills(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "from_corners", signature = (corners, number, level, rotation, base))]
        pub fn from_corners<'py>(
            py: Python<'py>,
            corners: Vec<Vec<u8>>,
            number: usize,
            level: usize,
            rotation: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::from_corners(&corners, number, level, rotation, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::from_json(text);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "from_strings", signature = (rows))]
        pub fn from_strings<'py>(
            py: Python<'py>,
            rows: Vec<String>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::from_strings(&rows);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "hline", signature = (number, level))]
        pub fn hline<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::hline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "htree", signature = (number, level))]
        pub fn htree<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::htree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "image", signature = (cell, scale, outline, width, shape))]
        pub fn image<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            scale: usize,
            outline: Option<PyColor>,
            width: usize,
            shape: PySerde<mrly_math::two::Shape>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let shape = shape.0;
            let out = mrly_math::two::image(&cell, scale, outline, width, shape);
            (PyImage(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "level_set", signature = (number, levels, level, rotation, base))]
        pub fn level_set<'py>(
            py: Python<'py>,
            number: usize,
            levels: Vec<usize>,
            level: usize,
            rotation: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::level_set(number, &levels, level, rotation, base);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mask", signature = (mask, shape))]
        pub fn mask<'py>(
            py: Python<'py>,
            mask: PyTensor,
            shape: Vec<usize>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let out = mrly_math::two::mask(&mask, &shape);
            (PyTensor(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "merge", signature = (cells, width, height))]
        pub fn merge<'py>(
            py: Python<'py>,
            cells: Vec<PyCell2d>,
            width: usize,
            height: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_math::two::merge(&cells, width, height);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "named", signature = (design, number, level, rotation))]
        pub fn named<'py>(
            py: Python<'py>,
            design: PySerde<mrly_math::bang::catalog::Design>,
            number: usize,
            level: usize,
            rotation: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrly_math::two::named(design, number, level, rotation);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "net", signature = (number, level))]
        pub fn net<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::net(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "noise", signature = (number, level, density, rng))]
        pub fn noise<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
            density: f64,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_math::two::noise(number, level, density, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "ones", signature = (number, level))]
        pub fn ones<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::ones(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "perimeter", signature = (cell))]
        pub fn perimeter<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::perimeter(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "point", signature = (number, level))]
        pub fn point<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::point(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "read", signature = (sheet, carrier))]
        pub fn read<'py>(
            py: Python<'py>,
            sheet: PyCell2d,
            carrier: PyCell2d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let sheet = sheet.0;
            let carrier = carrier.0;
            let out = mrly_math::two::read(&sheet, &carrier);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "sheet", signature = (cells, payload))]
        pub fn sheet<'py>(
            py: Python<'py>,
            cells: [PyCell2d; 4],
            payload: Vec<u8>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cells = cells.map(|x| x.0);
            let out = mrly_math::two::sheet(&cells, &payload);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "special", signature = (mask, cell))]
        pub fn special<'py>(
            py: Python<'py>,
            mask: PyTensor,
            cell: PyCell2d,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mask = mask.0;
            let cell = cell.0;
            let out = mrly_math::two::special(&mask, &cell);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "star", signature = (number, level))]
        pub fn star<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::star(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "svg", signature = (cell, scale, outline, width, shape))]
        pub fn svg<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            scale: usize,
            outline: Option<PyColor>,
            width: usize,
            shape: PySerde<mrly_math::two::Shape>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let outline = outline.map(|x| x.0);
            let shape = shape.0;
            let out = mrly_math::two::svg(&cell, scale, outline, width, shape);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "text", signature = (cell, glyphs=None))]
        pub fn text<'py>(
            py: Python<'py>,
            cell: PyCell2d,
            glyphs: Option<std::collections::HashMap<u8, String>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let glyphs = glyphs.map(|x| x.into_iter().collect());
            let out = mrly_math::two::text(&cell, glyphs.as_ref());
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "to_3d", signature = (cell))]
        pub fn to_3d<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::to_3d(&cell);
            (PyCellNd(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "vline", signature = (number, level))]
        pub fn vline<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::vline(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "void", signature = (number, level))]
        pub fn void<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::void(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "voids", signature = (cell))]
        pub fn voids<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::voids(&cell);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "vtree", signature = (number, level))]
        pub fn vtree<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::vtree(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zeros", signature = (number, level))]
        pub fn zeros<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::zeros(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.two")?;
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
            m.add_function(wrap_pyfunction!(image, &m)?)?;
            m.add_function(wrap_pyfunction!(level_set, &m)?)?;
            m.add_function(wrap_pyfunction!(mask, &m)?)?;
            m.add_function(wrap_pyfunction!(merge, &m)?)?;
            m.add_function(wrap_pyfunction!(named, &m)?)?;
            m.add_function(wrap_pyfunction!(net, &m)?)?;
            m.add_function(wrap_pyfunction!(noise, &m)?)?;
            m.add_function(wrap_pyfunction!(ones, &m)?)?;
            m.add_function(wrap_pyfunction!(perimeter, &m)?)?;
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
            let names: Vec<&str> = vec![
                "capacity",
                "carpet",
                "census",
                "create",
                "dust",
                "embed",
                "euler",
                "extract",
                "fills",
                "from_corners",
                "from_json",
                "from_strings",
                "hline",
                "htree",
                "image",
                "level_set",
                "mask",
                "merge",
                "named",
                "net",
                "noise",
                "ones",
                "perimeter",
                "point",
                "read",
                "sheet",
                "special",
                "star",
                "svg",
                "text",
                "to_3d",
                "to_json",
                "vline",
                "void",
                "voids",
                "vtree",
                "zeros",
            ];
            m.add("__all__", names)?;
            payload::init(py, &m, sys)?;
            parent.add("two", &m)?;
            sys.set_item("mrly.math._math.math.two", &m)?;
            Ok(())
        }
    }

    pub fn init(
        py: Python<'_>,
        parent: &Bound<'_, PyModule>,
        sys: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let m = PyModule::new(py, "mrly.math")?;
        let names: Vec<&str> = vec![];
        m.add("__all__", names)?;
        arcs::init(py, &m, sys)?;
        arrays::init(py, &m, sys)?;
        atoms::init(py, &m, sys)?;
        automaton::init(py, &m, sys)?;
        bang::init(py, &m, sys)?;
        cell::init(py, &m, sys)?;
        coprime::init(py, &m, sys)?;
        counts::init(py, &m, sys)?;
        design::init(py, &m, sys)?;
        dissection::init(py, &m, sys)?;
        game::init(py, &m, sys)?;
        graph::init(py, &m, sys)?;
        ladder::init(py, &m, sys)?;
        memory::init(py, &m, sys)?;
        moire::init(py, &m, sys)?;
        name::init(py, &m, sys)?;
        nodal::init(py, &m, sys)?;
        press::init(py, &m, sys)?;
        radix::init(py, &m, sys)?;
        roulette::init(py, &m, sys)?;
        rules::init(py, &m, sys)?;
        shape::init(py, &m, sys)?;
        six::init(py, &m, sys)?;
        spectrum::init(py, &m, sys)?;
        spin::init(py, &m, sys)?;
        spirograph::init(py, &m, sys)?;
        sumset::init(py, &m, sys)?;
        three::init(py, &m, sys)?;
        tourbillon::init(py, &m, sys)?;
        two::init(py, &m, sys)?;
        parent.add("math", &m)?;
        sys.set_item("mrly.math._math.math", &m)?;
        Ok(())
    }
}

pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = py
        .import("sys")?
        .getattr("modules")?
        .cast_into::<PyDict>()?;
    math::init(py, root, &sys)?;
    Ok(())
}
