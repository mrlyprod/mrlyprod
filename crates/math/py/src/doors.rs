#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// The designs: codes, cells, cubes and hexagons, their counts, graphs and names, digit designs on the line, and wheels.
pub mod math {
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// The Truchet arcs of a flat design: every cell two quarter arcs, the loops and strands they join into, and the proved loop laws.
    pub mod arcs {
        use crate::hand::{PyCode, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Walks level `level` of `bang dim 2, base b, code c` in arcs into its curves: first the strands, from each boundary midpoint in turn, across edges before upright ones, then the loops, each from its first unwalked arc.
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

        /// Draws level `level` of `bang dim 2, base b, code c` in arcs: cell `(x, y)` of the `b x b` mask is filled when bit `b y + x` of the code is set, level `level` is its Kronecker power built by `crate::bang::factory::create`, and level 0 is one filled cell.
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

        /// Returns the proved loop law of `bang dim 2, base b, code c` at the level, or none where no law is proved: the carpet, base 3 code 495, has `(8^n - 1)/7 - 3^n + n + 1` loops; at base 2 codes 7 and 14 have `3^(n-1) - 2^n + 1`, codes 11 and 13 have `3^(n-1) - 2^(n-1)`, both from level 1 on, code 9 has `2^n - 1`, and the other eleven codes never loop.
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

        /// Draws a `side x side` grid of filled and deleted cells in arcs and counts its curves by union-find over the edge midpoints, a curve being a loop when no midpoint of it lies on the boundary.
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
            m.setattr("__doc__", "The Truchet arcs of a flat design: every cell two quarter arcs, the loops and strands they join into, and the proved loop laws.")?;
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

    /// The sparse arrays: sensor positions, their difference coarray and its weights, the essential sensors whose loss opens a hole, and the fractal array a generator grows with the essential count Theorem A proves.
    pub mod arrays {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns whether each sensor is essential, in the order given: whether removing it deletes a lag from the coarray, which happens exactly when some lag `t != 0` has all its pairs through it, the one pair of a lag of weight `1` or the two pairs `(s, s - t)` and `(s + t, s)` of a lag of weight `2`.
        #[pyfunction]
        #[pyo3(name = "essential", signature = (sensors))]
        pub fn essential<'py>(py: Python<'py>, sensors: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::essential(&sensors);
            (out).into_bound_py_any(py)
        }

        /// Returns the fractal array of the generator at the base and level, ascending: every `sum_(i < level) g_i base^i` with each digit `g_i` in the generator.
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

        /// Returns the holes of the coarray, ascending: the lags between `-span` and `span` that no pair of sensors makes.
        #[pyfunction]
        #[pyo3(name = "holes", signature = (sensors))]
        pub fn holes<'py>(py: Python<'py>, sensors: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::holes(&sensors);
            (out).into_bound_py_any(py)
        }

        /// Returns the essential count and the sensor count that Theorem A of the arrays note proves for the fractal array of the generator at base `2a + 1`, `(u^level, L^level)`, `u` the count of `paired` digits; `None` off the theorem: a level below 2, a generator without `0`, or a generator whose coarray has a hole.
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

        /// Returns the lags that removing the sensor deletes from the coarray, ascending: those every pair of which holds it; empty when the sensor is inessential or absent.
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

        /// Returns the paired sensors, ascending: each `g` with a partner `h` whose lag `g - h` has weight `1`, the set `U` of condition C1 of Cohen and Eldar; a paired sensor is essential.
        #[pyfunction]
        #[pyo3(name = "paired", signature = (sensors))]
        pub fn paired<'py>(py: Python<'py>, sensors: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::arrays::paired(&sensors);
            (out).into_bound_py_any(py)
        }

        /// Returns the coarray weights of the sensors, `w(t)` for every lag `t` from `-span` to `span` at index `t + span`, `span` the largest sensor less the least: the count of ordered pairs `(p, q)` of sensors with `p - q = t`.
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
            m.setattr("__doc__", "The sparse arrays: sensor positions, their difference coarray and its weights, the essential sensors whose loss opens a hole, and the fractal array a generator grows with the essential count Theorem A proves.")?;
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

    /// Ready-made tensors: zeros, ones, noise and carpets in two or three dimensions.
    pub mod atoms {
        use crate::hand::PyTensor;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Builds an n by n carpet, on where at most one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "carpet_2d", signature = (n))]
        pub fn carpet_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::carpet_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n carpet, on where at most one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "carpet_3d", signature = (n))]
        pub fn carpet_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::carpet_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a carpet of the given side at any rank, on where at most one coordinate is odd.
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

        /// Builds an n by n dust, on where both coordinates are even.
        #[pyfunction]
        #[pyo3(name = "dust_2d", signature = (n))]
        pub fn dust_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::dust_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n dust, on where all three coordinates are even.
        #[pyfunction]
        #[pyo3(name = "dust_3d", signature = (n))]
        pub fn dust_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::dust_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a dust of the given side at any rank, on where every coordinate is even.
        #[pyfunction]
        #[pyo3(name = "dust_nd", signature = (n, rank))]
        pub fn dust_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::dust_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n line, free on axis 1, on along the odd rows.
        #[pyfunction]
        #[pyo3(name = "hline_2d", signature = (n))]
        pub fn hline_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::hline_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tree, free on axis 1, on along the even rows.
        #[pyfunction]
        #[pyo3(name = "htree_2d", signature = (n))]
        pub fn htree_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::htree_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none.
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

        /// Builds an n by n net, on where at least one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "net_2d", signature = (n))]
        pub fn net_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::net_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n net, on where at least two coordinates are odd.
        #[pyfunction]
        #[pyo3(name = "net_3d", signature = (n))]
        pub fn net_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::net_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank.
        #[pyfunction]
        #[pyo3(name = "net_nd", signature = (n, rank))]
        pub fn net_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::net_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tensor where each cell turns on with probability density, drawn from the stream.
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

        /// Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream.
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

        /// Builds an n by n tensor of ones.
        #[pyfunction]
        #[pyo3(name = "ones_2d", signature = (n))]
        pub fn ones_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::ones_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tensor of ones.
        #[pyfunction]
        #[pyo3(name = "ones_3d", signature = (n))]
        pub fn ones_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::ones_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n point, on where both coordinates are odd.
        #[pyfunction]
        #[pyo3(name = "point_2d", signature = (n))]
        pub fn point_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::point_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n point, on where all three coordinates are odd.
        #[pyfunction]
        #[pyo3(name = "point_3d", signature = (n))]
        pub fn point_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::point_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a point of the given side at any rank, on where every coordinate is odd.
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

        /// Builds an n by n star, on where exactly one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "star_2d", signature = (n))]
        pub fn star_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::star_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n star, on where exactly one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "star_3d", signature = (n))]
        pub fn star_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::star_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a star of the given side at any rank, on where exactly one coordinate is odd.
        #[pyfunction]
        #[pyo3(name = "star_nd", signature = (n, rank))]
        pub fn star_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::star_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none.
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

        /// Builds an n by n line, free on axis 0, on along the odd columns.
        #[pyfunction]
        #[pyo3(name = "vline_2d", signature = (n))]
        pub fn vline_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::vline_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n void, on where both coordinates share one parity.
        #[pyfunction]
        #[pyo3(name = "void_2d", signature = (n))]
        pub fn void_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::void_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n void, on where all three coordinates share one parity.
        #[pyfunction]
        #[pyo3(name = "void_3d", signature = (n))]
        pub fn void_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::void_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds a void of the given side at any rank, on where every coordinate shares one parity.
        #[pyfunction]
        #[pyo3(name = "void_nd", signature = (n, rank))]
        pub fn void_nd<'py>(py: Python<'py>, n: usize, rank: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::void_nd(n, rank);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tree, free on axis 0, on along the even columns.
        #[pyfunction]
        #[pyo3(name = "vtree_2d", signature = (n))]
        pub fn vtree_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::vtree_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n line, free on axis 0, its rods running along x.
        #[pyfunction]
        #[pyo3(name = "xline_3d", signature = (n))]
        pub fn xline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::xline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tree, free on axis 0, its beams running along x.
        #[pyfunction]
        #[pyo3(name = "xtree_3d", signature = (n))]
        pub fn xtree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::xtree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n line, free on axis 1, its rods running along y.
        #[pyfunction]
        #[pyo3(name = "yline_3d", signature = (n))]
        pub fn yline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::yline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tree, free on axis 1, its beams running along y.
        #[pyfunction]
        #[pyo3(name = "ytree_3d", signature = (n))]
        pub fn ytree_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::ytree_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n tensor of zeros.
        #[pyfunction]
        #[pyo3(name = "zeros_2d", signature = (n))]
        pub fn zeros_2d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::zeros_2d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tensor of zeros.
        #[pyfunction]
        #[pyo3(name = "zeros_3d", signature = (n))]
        pub fn zeros_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::zeros_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n line, free on axis 2, its rods running along z.
        #[pyfunction]
        #[pyo3(name = "zline_3d", signature = (n))]
        pub fn zline_3d<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::atoms::zline_3d(n);
            (PyTensor(out)).into_bound_py_any(py)
        }

        /// Builds an n by n by n tree, free on axis 2, its beams running along z.
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
            m.setattr(
                "__doc__",
                "Ready-made tensors: zeros, ones, noise and carpets in two or three dimensions.",
            )?;
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

    /// The matrix ladder: the Dirichlet series of a memory design continued through its transfer matrix, its determinant cofactor and the residues on its pole combs.
    pub mod automaton {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from.
        #[pyclass(name = "Automaton", module = "mrly.math.automaton", from_py_object)]
        #[derive(Clone)]
        pub struct Automaton(pub mrly_math::automaton::Automaton);

        #[pymethods]
        impl Automaton {
            /// Builds the ladder of a rule, choosing the peel depth.
            #[new]
            #[pyo3(signature = (rule))]
            pub fn __new__(rule: PyRef<'_, crate::doors::math::memory::Rule>) -> PyResult<Self> {
                let out = mrly_math::automaton::Automaton::new(&rule.0);
                Ok(Self(ok(out)?))
            }
            /// Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of `crate::memory::perron`.
            #[pyo3(name = "abscissa", signature = ())]
            pub fn abscissa<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::abscissa(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the base `q = 2^D`.
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::base(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to.
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
            /// Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`.
            #[pyo3(name = "denominator", signature = ())]
            pub fn denominator<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::denominator(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of `crate::memory::transfer`, entry `(u', u)` counting the letters carrying `u` to `u'`.
            #[pyo3(name = "matrix", signature = ())]
            pub fn matrix<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::matrix(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds the ladder of a rule, choosing the peel depth.
            #[staticmethod]
            #[pyo3(name = "new", signature = (rule))]
            pub fn new_<'py>(
                py: Python<'py>,
                rule: PyRef<'_, crate::doors::math::memory::Rule>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::new(&rule.0);
                (crate::doors::math::automaton::Automaton(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the peel depth `P`.
            #[pyo3(name = "peel", signature = ())]
            pub fn peel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::peel(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the pole spacing `2 pi / log q`.
            #[pyo3(name = "period", signature = ())]
            pub fn period<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::period(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with.
            #[pyo3(name = "perron", signature = ())]
            pub fn perron<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::perron(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to.
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
            /// Returns the rule.
            #[pyo3(name = "rule", signature = ())]
            pub fn rule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::rule(&self.0);
                (crate::doors::math::memory::Rule(out)).into_bound_py_any(py)
            }
            /// Returns the state count `q^(k-1)`.
            #[pyo3(name = "states", signature = ())]
            pub fn states<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::automaton::Automaton::states(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds the ladder at an explicit peel depth, at least the rule width and at least two.
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
            /// Returns `zeta_W(s)` and the bound it is known to.
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

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.math.automaton")?;
            m.setattr("__doc__", "The matrix ladder: the Dirichlet series of a memory design continued through its transfer matrix, its determinant cofactor and the residues on its pole combs.\n\nA claim witness with no caller in the crate: it witnesses the [memory dial claim](https://github.com/mrlyprod/mrlyprod/blob/main/research/beneath/claims/memory-dial.md).")?;
            m.add_class::<Automaton>()?;
            m.add("ROUNDING", mrly_math::automaton::ROUNDING)?;
            let names: Vec<&str> = vec!["Automaton", "ROUNDING"];
            m.add("__all__", names)?;
            parent.add("automaton", &m)?;
            sys.set_item("mrly.math._math.math.automaton", &m)?;
            Ok(())
        }
    }

    /// The universe of design codes.
    pub mod bang {
        use crate::hand::{PyCode, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The base-q symmetry maps, the design counts raw and distinct, and the fill classes.
        pub mod baseq {
            use crate::hand::{PyCode, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Returns the distinct rotation and reflection maps of a base-q axis.
            #[pyfunction]
            #[pyo3(name = "axis_maps", signature = (base))]
            pub fn axis_maps<'py>(py: Python<'py>, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::axis_maps(base);
                (out).into_bound_py_any(py)
            }

            /// Returns the distinct one-dimensional design counts for bases 1 through max_base.
            #[pyfunction]
            #[pyo3(name = "bracelets", signature = (max_base))]
            pub fn bracelets<'py>(py: Python<'py>, max_base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::bracelets(max_base);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the least code of the design's orbit.
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

            /// Carries a code through one group element.
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

            /// Returns the fill-class counts for dimensions 1 through max_dimension.
            #[pyfunction]
            #[pyo3(name = "class_sequence", signature = (max_dimension))]
            pub fn class_sequence<'py>(
                py: Python<'py>,
                max_dimension: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::class_sequence(max_dimension);
                (out).into_bound_py_any(py)
            }

            /// Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension.
            #[pyfunction]
            #[pyo3(name = "classes", signature = (dimension))]
            pub fn classes<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::baseq::classes(dimension);
                (out).into_bound_py_any(py)
            }

            /// Counts base-q designs distinct under symmetry.
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

            /// Returns the collapsed fill count at an even side number.
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

            /// Returns the filled-cell count of a binary design at a side number, folded from its filled corners.
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

            /// Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`.
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

            /// Returns the symmetry group order counted from the enumerated axis maps.
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

            /// Returns every code a design reaches under the group.
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

            /// Returns the closed-form group order the axis-map count must match.
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

            /// Walks every code of a base and dimension and returns each orbit's least code with the orbit's size.
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

            /// Returns the distinct-design counts for dimensions 1 through max_dimension.
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

            /// Returns the raw design count before symmetry, two to the number of cells.
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

        /// The cached canonical codes and tile sources of a dimension.
        pub mod catalog {
            use crate::hand::PySerde;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// The named designs a source can point at: the four classics and their four antis.
            #[pyclass(
                name = "Design",
                module = "mrly.math.bang.catalog",
                skip_from_py_object
            )]
            pub struct Design;

            #[pymethods]
            impl Design {
                /// Returns every Design in canonical order.
                #[staticmethod]
                #[pyo3(name = "all", signature = ())]
                pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::bang::catalog::Design::all();
                    ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
                }
            }

            /// Returns the anti designs for a dimension.
            #[pyfunction]
            #[pyo3(name = "antis", signature = (dimension))]
            pub fn antis<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::catalog::antis(dimension);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Returns the classic designs for a dimension.
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
                m.setattr(
                    "__doc__",
                    "The cached canonical codes and tile sources of a dimension.",
                )?;
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

        /// The design code: the bitmask of filled corners, printed and parsed as one number.
        pub mod code {
            use crate::hand::PyCode;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Returns the bitmask the code carries.
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
                m.setattr("__doc__", "The design code: the bitmask of filled corners, printed and parsed as one number.")?;
                m.add_function(wrap_pyfunction!(get, &m)?)?;
                let names: Vec<&str> = vec!["get"];
                m.add("__all__", names)?;
                parent.add("code", &m)?;
                sys.set_item("mrly.math._math.math.bang.code", &m)?;
                Ok(())
            }
        }

        /// The packing of residue corners into codes and back.
        pub mod factory {
            use crate::hand::{PyCode, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Renders a coded design to a tensor at its side number, dimension, base and fractal level.
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

            /// Renders a design straight from its filled residue corners.
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

            /// Renders a design from its canonical JSON name.
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

            /// Returns every base-q residue corner of a dimension in row-major order.
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

            /// Returns the code count of a dimension and base, two to the number of corners.
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
                m.setattr(
                    "__doc__",
                    "The packing of residue corners into codes and back.",
                )?;
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

        /// The corners, codes and symmetries that name designs.
        pub mod universe {
            use crate::hand::PyCode;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Returns the algebraic normal form coefficients of a code, one per corner.
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

            /// Formats the algebraic normal form of a code as a sum of monomials.
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

            /// Applies a symmetry element to a corner.
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

            /// Returns the bit position a binary corner occupies in a code.
            #[pyfunction]
            #[pyo3(name = "corner_index", signature = (corner))]
            pub fn corner_index<'py>(
                py: Python<'py>,
                corner: Vec<u8>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::universe::corner_index(&corner);
                (out).into_bound_py_any(py)
            }

            /// Returns the algebraic degree of a code, or -1 for the zero design.
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

            /// Returns every code a design reaches under the full symmetry group.
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

            /// Returns every permutation of 0..n in sorted order.
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
                m.setattr(
                    "__doc__",
                    "The corners, codes and symmetries that name designs.",
                )?;
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

        /// The magic words: their products, their component counts and the schedules that spell them.
        pub mod word {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// The named infinite schedules over an ordered pair of letters.
            #[pyclass(name = "Schedule", module = "mrly.math.bang.word", skip_from_py_object)]
            pub struct Schedule;

            #[pymethods]
            impl Schedule {
                /// Returns every Schedule in canonical order.
                #[staticmethod]
                #[pyo3(name = "all", signature = ())]
                pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::bang::word::Schedule::all();
                    ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
                }
                /// Returns the letter frequencies the schedule tends to.
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
                /// Returns the letter the schedule takes at the place, zero or one.
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

            /// Counts the 4-connected components of a plane word without drawing it.
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

            /// Returns the constant-word component functional of a plane word's letter frequencies,
            /// in log two units.
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

            /// Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides.
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

            /// Returns the filled cells of a word, the product of its letter fills.
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

            /// Lists the filled cells of every letter, the product of which is the word's fill.
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

            /// Reads one plane letter: its fill, its runs, the rows and columns that wrap into a
            /// neighbouring copy, and its own components.
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

            /// Returns whether every letter renders at its own residue base, the native case where a
            /// periodic word folds to one residue rule at the product base.
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

            /// Returns the shortest whole period of the letter list, its own length when no shorter block repeats.
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

            /// Folds a plane word letter by letter and returns the counts at every prefix.
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

            /// Returns the prefix rates of a plane word in log two units, the component rate
            /// `(1/L) log2 comp` and the fill rate `(1/L) log2 fill` at every prefix length.
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

            /// Returns the side of a word, the product of its letter sides.
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

            /// Spells the first letters of a schedule over an ordered pair of letters.
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

            /// Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,
            /// then `magic(3,5)`, then `magic(3,5,7)`, and so on.
            #[pyfunction]
            #[pyo3(name = "staircase", signature = (depth))]
            pub fn staircase<'py>(py: Python<'py>, depth: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::word::staircase(depth);
                ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }

            /// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
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

        /// A single design with its place in the orbit structure.
        #[pyclass(name = "Design", module = "mrly.math.bang", from_py_object)]
        #[derive(Clone)]
        pub struct Design(pub mrly_math::bang::Design);

        #[pymethods]
        impl Design {
            /// The design's code.
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
            /// The design's dimension.
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
            /// Whether this code is the smallest in its orbit.
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
            /// The smallest code in the orbit.
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
            /// The number of codes in the orbit.
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
            /// Returns the design's algebraic normal form as a string.
            #[pyo3(name = "anf", signature = ())]
            pub fn anf<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::anf(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the design's algebraic degree, or -1 for the zero design.
            #[pyo3(name = "degree", signature = ())]
            pub fn degree<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::degree(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the design's name as a line of prose, `bang dim 2, code 7`.
            #[pyo3(name = "name", signature = ())]
            pub fn name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::name(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the design's filled corners in sorted order.
            #[pyo3(name = "rule", signature = ())]
            pub fn rule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Design::rule(&self.0);
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
        #[pyclass(name = "Universe", module = "mrly.math.bang", skip_from_py_object)]
        pub struct Universe(pub mrly_math::bang::Universe);

        #[pymethods]
        impl Universe {
            /// Enumerates every orbit of a dimension from 1 to 4.
            #[new]
            #[pyo3(signature = (dimension))]
            pub fn __new__(dimension: usize) -> PyResult<Self> {
                let out = mrly_math::bang::Universe::new(dimension);
                Ok(Self(ok(out)?))
            }
            /// The universe's dimension.
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
            /// The number of codes in the universe.
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
            /// Returns every design in code order.
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::all(&self.0);
                ((out)
                    .into_iter()
                    .map(crate::doors::math::bang::Design)
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }
            /// Returns the designs whose codes lead their orbits.
            #[pyo3(name = "canonical", signature = ())]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::canonical(&self.0);
                ((out)
                    .into_iter()
                    .map(crate::doors::math::bang::Design)
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }
            /// Returns the design at a code with its precomputed orbit facts.
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
            /// Returns the number of distinct orbits.
            #[pyo3(name = "distinct", signature = ())]
            pub fn distinct<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::distinct(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Enumerates every orbit of a dimension from 1 to 4.
            #[staticmethod]
            #[pyo3(name = "new", signature = (dimension))]
            pub fn new_<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::bang::Universe::new(dimension);
                (crate::doors::math::bang::Universe(ok(out)?)).into_bound_py_any(py)
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
        #[pyclass(name = "MagicLayer", module = "mrly.math.bang", skip_from_py_object)]
        pub struct MagicLayer;

        #[pymethods]
        impl MagicLayer {
            /// Pins a design to the side number it renders at.
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

        /// Builds the universe of a dimension.
        #[pyfunction]
        #[pyo3(name = "bang", signature = (dimension))]
        pub fn bang<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::bang(dimension);
            (crate::doors::math::bang::Universe(ok(out)?)).into_bound_py_any(py)
        }

        /// Unpacks a code into its filled residue corners.
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

        /// Returns the binary corners of a dimension in code order.
        #[pyfunction]
        #[pyo3(name = "corners", signature = (dimension))]
        pub fn corners<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::corners(dimension);
            (out).into_bound_py_any(py)
        }

        /// Packs filled residue corners back into their code.
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

        /// Returns the deepest level, never below one, whose grid of number to the dimension per level stays within the cell budget.
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

        /// Returns the code of the design filled wherever a corner's residue sum lands in the levels.
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

        /// Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost.
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

        /// Composes JSON-named layers in order.
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

        /// Rolls a code uniformly from the nonempty codes of a dimension and base that fill at least min_fills corners.
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

        /// Builds the tile sources a catalog names at a dimension.
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

        /// Returns the full symmetry group as axis permutations paired with flip patterns.
        #[pyfunction]
        #[pyo3(name = "symmetries", signature = (dimension))]
        pub fn symmetries<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::bang::symmetries(dimension);
            (out).into_bound_py_any(py)
        }

        /// Returns whether no two filled corners of a code sit at Hamming distance one.
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

        /// Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side.
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

        /// Returns the canonical design codes of a dimension, computed once and cached for the process.
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
            m.setattr("__doc__", "The universe of design codes.\n\nA code is a bitmask over the residue corners of a hypercube; this folder packs corners into\ncodes, folds codes into symmetry classes, counts the classes and spells the magic words.")?;
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

    /// The N-dimensional cell and the pipeline the fixed dimensions share.
    pub mod cell {
        use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyColor, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::exceptions::PyValueError;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The fill, void and exposure counts of an N-dimensional cell.
        pub mod census {
            use crate::hand::{PyCell2d, PyCell3d, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Counts the distinct unit edges the filled sites carry, the edge graph's branches.
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

            /// Counts the faces of filled sites open to emptiness or the border.
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

            /// Counts the filled sites of the cell.
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

            /// Counts the distinct corners the filled sites touch, the edge graph's nodes.
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

            /// Counts the empty sites of the cell.
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
                m.setattr(
                    "__doc__",
                    "The fill, void and exposure counts of an N-dimensional cell.",
                )?;
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

        /// The merges, magic folds, mosaics and perforations of N-dimensional cells.
        pub mod geometry {
            use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Merges same-shaped cells into one block laid out by the per-axis repetition counts.
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

            /// Writes the value into the cell wherever the tiled mask is nonzero.
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
                m.setattr(
                    "__doc__",
                    "The merges, magic folds, mosaics and perforations of N-dimensional cells.",
                )?;
                m.add_function(wrap_pyfunction!(merge_reps, &m)?)?;
                m.add_function(wrap_pyfunction!(perforate, &m)?)?;
                let names: Vec<&str> = vec!["merge_reps", "perforate"];
                m.add("__all__", names)?;
                parent.add("geometry", &m)?;
                sys.set_item("mrly.math._math.math.cell.geometry", &m)?;
                Ok(())
            }
        }

        /// The N-dimensional cell and its fixed-dimension aliases.
        pub mod models {
            use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyColor, PySerde, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::exceptions::PyValueError;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Inverts the cell.
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

            /// Maps each site to one at or above the threshold, zero below.
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

            /// Binarizes the cell at the threshold Otsu's method picks.
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

            /// Rounds each site to the mean of its masked neighborhood, wrapping on request.
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

            /// Returns the Kronecker product of the two cells.
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

            /// Returns the narrowest count dtype that fits the mask's popcount.
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

            /// Returns the size of axis 0, the cube's leading axis.
            #[pyfunction]
            #[pyo3(name = "depth", signature = (cell))]
            pub fn depth<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
                let cell = cell.0;
                let out = mrly_math::cell::models::CellNd::<3>::depth(&cell);
                (out).into_bound_py_any(py)
            }

            /// Returns the narrowest unsigned dtype that holds the peak value.
            #[pyfunction]
            #[pyo3(name = "dtype_for", signature = (peak))]
            pub fn dtype_for<'py>(py: Python<'py>, peak: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::cell::models::dtype_for(peak);
                (PySerde(out)).into_bound_py_any(py)
            }

            /// Deepens the cell into its level-fold fractal.
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

            /// Returns the size of the axis before the last.
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

            /// Swaps filled and empty sites.
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

            /// Tags each site with its ring distance from the center.
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

            /// Tags each site with its count of masked neighbors matching the target, wrapping on request.
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

            /// Builds a cell from an N-dimensional tensor of types.
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

            /// Turns the cell into one of the 24 cube orientations.
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

            /// Wraps the cell in count layers of the given value on every side.
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

            /// Colors each site by its type through the mapping in the given mode.
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

            /// Writes the value wherever the tiled mask is nonzero.
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

            /// Rotates the cell k quarter turns in the plane.
            ///
            /// Rotates the cell k quarter turns about the given pair of axes.
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

            /// Repeats the cell into a width-by-height array of copies.
            ///
            /// Repeats the cell into a width-by-height-by-depth array of copies.
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

            /// Returns the tensor of types.
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

            /// Returns the size of the last axis.
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
                m.setattr(
                    "__doc__",
                    "The N-dimensional cell and its fixed-dimension aliases.",
                )?;
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

        /// The JSON reading the cell serializers share.
        pub mod serializer {
            use crate::hand::{PyPixels, PySerde, PyTensor, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Reads a triply nested JSON array into layers of byte rows.
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

            /// Reads a nested JSON array into rows of bytes.
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

            /// Reads a nested JSON array into rows of four-channel colors.
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

            /// Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
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

            /// Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
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

            /// Parses JSON text into a value tree.
            #[pyfunction]
            #[pyo3(name = "parse", signature = (text))]
            pub fn parse<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::cell::serializer::parse(text);
                (PySerde(ok(out)?)).into_bound_py_any(py)
            }

            /// Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them.
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

            /// Returns the types field of the data.
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
                m.setattr("__doc__", "The JSON reading the cell serializers share.")?;
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

        /// Grows a seed pattern into a cell, deepened to its fractal past level one.
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

        /// Colors the cell through the given mapping and mode, defaulting to the standard palette by type.
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
            m.setattr("__doc__", "The N-dimensional cell and the pipeline the fixed dimensions share.\n\nA seed pattern grows into a fractal cell here, and `two`, `three` and `six` are that one\npipeline pinned to a dimension: census, geometry, graphs, painting, text and JSON.")?;
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

    /// The coprime points of a design: the Sierpinski gasket's pairs with no common divisor, counted row by row.
    pub mod coprime {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Counts the coprime points of the Sierpinski gasket at a level: the pairs `0 <= i, j < 2^level` with `i AND j = 0` and `gcd(i, j) = 1`, the sequence [A396934](https://oeis.org/A396934).
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
            m.setattr("__doc__", "The coprime points of a design: the Sierpinski gasket's pairs with no common divisor, counted row by row.")?;
            m.add_function(wrap_pyfunction!(gasket, &m)?)?;
            m.add("GASKET_LEVELS", mrly_math::coprime::GASKET_LEVELS)?;
            let names: Vec<&str> = vec!["gasket", "GASKET_LEVELS"];
            m.add("__all__", names)?;
            parent.add("coprime", &m)?;
            sys.set_item("mrly.math._math.math.coprime", &m)?;
            Ok(())
        }
    }

    /// The closed-form counts.
    pub mod counts {
        use crate::hand::{PyCode, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The diagonal profile of any tile's power, as a digit polynomial.
        pub mod diagonal {
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            pub fn init(
                py: Python<'_>,
                parent: &Bound<'_, PyModule>,
                sys: &Bound<'_, PyDict>,
            ) -> PyResult<()> {
                let m = PyModule::new(py, "mrly.math.counts.diagonal")?;
                m.setattr(
                    "__doc__",
                    "The diagonal profile of any tile's power, as a digit polynomial.",
                )?;
                m.add("WIDEST", mrly_math::counts::diagonal::WIDEST)?;
                let names: Vec<&str> = vec!["WIDEST"];
                m.add("__all__", names)?;
                parent.add("diagonal", &m)?;
                sys.set_item("mrly.math._math.math.counts.diagonal", &m)?;
                Ok(())
            }
        }

        /// The base-q slice carry automaton: its digit polynomial, its matrix, its ladder and its sign law.
        pub mod ladder {
            use crate::hand::ok;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// The widest dimension the exact carry arithmetic reaches at the base.
            #[pyfunction]
            #[pyo3(name = "cap", signature = (base))]
            pub fn cap<'py>(py: Python<'py>, base: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::cap(base);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out.
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

            /// The monic characteristic polynomial of a square integer matrix, highest power first.
            #[pyfunction]
            #[pyo3(name = "characteristic", signature = (rows))]
            pub fn characteristic<'py>(
                py: Python<'py>,
                rows: Vec<Vec<i128>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::characteristic(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The determinant of a square integer matrix, read off its characteristic polynomial.
            #[pyfunction]
            #[pyo3(name = "determinant", signature = (rows))]
            pub fn determinant<'py>(
                py: Python<'py>,
                rows: Vec<Vec<i128>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::determinant(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first.
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

            /// The reflection-even block of the carry matrix, of size `ceil(D/2)`.
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

            /// The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`.
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

            /// The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`.
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

            /// The Perron root of a nonnegative square integer matrix.
            #[pyfunction]
            #[pyo3(name = "perron", signature = (rows))]
            pub fn perron<'py>(
                py: Python<'py>,
                rows: Vec<Vec<i128>>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::ladder::perron(&rows);
                (ok(out)?).into_bound_py_any(py)
            }

            /// The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers.
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

            /// The Perron root over the modulus of the second eigenvalue, or none where the block is one wide.
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

            /// The trace of a square integer matrix.
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

        /// The closed-form triangle, node and edge counts of hex slices.
        pub mod six {
            use crate::hand::ok;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Returns the triangles of the full hexagon with side number to the level.
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

            /// Returns the boundary edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_boundary", signature = (number))]
            pub fn solid_slice_boundary<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_boundary(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the core edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_core_edges", signature = (number))]
            pub fn solid_slice_core_edges<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_core_edges(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the core node count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_core_nodes", signature = (number))]
            pub fn solid_slice_core_nodes<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_core_nodes(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the distinct triangle-edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_edges", signature = (number))]
            pub fn solid_slice_edges<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_edges(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the interior edge count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_interior", signature = (number))]
            pub fn solid_slice_interior<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_interior(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the triangle count of the solid slice, defined for odd number.
            #[pyfunction]
            #[pyo3(name = "solid_slice_triangles", signature = (number))]
            pub fn solid_slice_triangles<'py>(
                py: Python<'py>,
                number: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::six::solid_slice_triangles(number);
                (ok(out)?).into_bound_py_any(py)
            }

            /// Returns the vertex count of the solid slice, defined for odd number.
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
                m.setattr(
                    "__doc__",
                    "The closed-form triangle, node and edge counts of hex slices.",
                )?;
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

        /// A closed form of a design sequence.
        #[pyclass(name = "Closed", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Closed(pub mrly_math::counts::Closed);

        #[pymethods]
        impl Closed {
            /// Spells the closed form.
            #[pyo3(name = "text", signature = ())]
            pub fn text<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Closed::text(&self.0);
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

        /// The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions.
        #[pyclass(name = "Exposure", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Exposure(pub mrly_math::counts::Exposure);

        #[pymethods]
        impl Exposure {
            /// The filled cells of the tile.
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
            /// The exposed faces of the tile.
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
            /// Per axis, the adjacent filled pairs and the spanning positions.
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
            /// Returns the exposed faces of the level-fold Kronecker power, or none past a u128.
            #[pyo3(name = "at", signature = (level))]
            pub fn at<'py>(&self, py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Exposure::at(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Folds the counts from the filled residue corners at a side number, without rendering the tile.
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
            /// Reads the counts off a rendered tile.
            #[staticmethod]
            #[pyo3(name = "of_tile", signature = (tile))]
            pub fn of_tile<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
                let tile = tile.0;
                let out = mrly_math::counts::Exposure::of_tile(&tile);
                (crate::doors::math::counts::Exposure(out)).into_bound_py_any(py)
            }
            /// Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys.
            #[pyo3(name = "recurrence", signature = ())]
            pub fn recurrence<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Exposure::recurrence(&self.0);
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

        /// The placement counts the face recurrence runs on: for each pattern `P` of cells inside a `2^d` window and each lowest digit `w` of the tile, whether `w + P` fits the tile with wrap-around, and which pattern `P ^ W` the carried digits must then form one level up.
        #[pyclass(name = "Placements", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Placements(pub mrly_math::counts::Placements);

        #[pymethods]
        impl Placements {
            /// Returns the face counts of the level-fold Kronecker power, or none past a `u128`.
            #[pyo3(name = "at", signature = (level))]
            pub fn at<'py>(&self, py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Placements::at(&self.0, level);
                ((out).map(crate::doors::math::counts::Skeleton)).into_bound_py_any(py)
            }
            /// Reads the placement counts off a rendered tile of two to four dimensions.
            #[staticmethod]
            #[pyo3(name = "of_tile", signature = (tile))]
            pub fn of_tile<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
                let tile = tile.0;
                let out = mrly_math::counts::Placements::of_tile(&tile);
                (crate::doors::math::counts::Placements(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the `k`-face count obeys from `L = 0`, every face when `k` is none, or none past an `i128`.
            #[pyo3(name = "recurrence", signature = (k=None))]
            pub fn recurrence<'py>(
                &self,
                py: Python<'py>,
                k: Option<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Placements::recurrence(&self.0, k);
                (out).into_bound_py_any(py)
            }
            /// Returns the roots of the recurrence the `k`-face count obeys, every face when `k` is none, each with the multiplicity the recurrence needs: the spanning counts of the patterns the count reaches, a root repeated once per pattern of that count on one carry chain.
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

        /// The face counts of the union of closed unit cubes a design's level power fills: `counts[k]` is its distinct unit `k`-faces, the cells last, and `euler` their alternating sum.
        #[pyclass(name = "Skeleton", module = "mrly.math.counts", from_py_object)]
        #[derive(Clone)]
        pub struct Skeleton(pub mrly_math::counts::Skeleton);

        #[pymethods]
        impl Skeleton {
            /// The distinct unit `k`-faces for `k = 0..=dimension`.
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
            /// The Euler characteristic, `sum (-1)^k counts[k]`.
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
            /// The filled cells.
            #[pyo3(name = "cells", signature = ())]
            pub fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::cells(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The distinct unit edges.
            #[pyo3(name = "edges", signature = ())]
            pub fn edges<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::edges(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The distinct unit squares, the cells themselves in the plane.
            #[pyo3(name = "faces", signature = ())]
            pub fn faces<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::faces(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The distinct corners.
            #[pyo3(name = "vertices", signature = ())]
            pub fn vertices<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::counts::Skeleton::vertices(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        /// Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1.
        #[pyfunction]
        #[pyo3(name = "centered_hexagonal", signature = (m))]
        pub fn centered_hexagonal<'py>(py: Python<'py>, m: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::counts::centered_hexagonal(m);
            (out).into_bound_py_any(py)
        }

        /// Returns the filled triangle count of the code's cut section at the given level, without rendering it.
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

        /// Returns the empty triangle count of the code's cut section at the given level.
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

        /// Returns the code's fractal dimension, the log of its one-level fill over the log of number.
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

        /// Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term
        /// recurrence over four powers, or none past a u128.
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

        /// Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners.
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

        /// Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128.
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

        /// Returns the coefficients of the recurrence the tile's exposure obeys.
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

        /// Returns the filled cell count of the code's fractal at the given level, without rendering it.
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

        /// Sums each corner's position products into a base fill and raises it to the level.
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

        /// Expands the odd-side fill at side `2k - 1`, `sum over the corners of k^(zeros) (k - 1)^(ones)`, into coefficients by rising power of `k`.
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

        /// Returns the total cells of the grid, number to the dimension, to the level.
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

        /// Returns the fill ratio the code walks toward as the side number grows, reduced.
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

        /// Expands the whole grid at side `2k - 1`, `(2k - 1)^dimension`, into coefficients by rising power of `k`.
        #[pyfunction]
        #[pyo3(name = "odd_power", signature = (dimension))]
        pub fn odd_power<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::counts::odd_power(dimension);
            (out).into_bound_py_any(py)
        }

        /// Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled.
        #[pyfunction]
        #[pyo3(name = "pairs", signature = (tile))]
        pub fn pairs<'py>(py: Python<'py>, tile: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let tile = tile.0;
            let out = mrly_math::counts::pairs(&tile);
            (out).into_bound_py_any(py)
        }

        /// Counts the indices below number that equal residue modulo base.
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

        /// Returns the filled triangle count of the code's pro projection at the given level, without rendering it.
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

        /// Returns the empty triangle count of the code's pro projection at the given level.
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

        /// Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`.
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

        /// Returns the filled fraction of the grid, or 0.0 for an empty grid.
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

        /// Returns the exact filled fraction as a fraction of fill over grid, reduced.
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

        /// Returns the face counts of the code's design at the given level, from the tile's placement counts alone.
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

        /// Returns the face counts of the tile's level-fold Kronecker power, or none past a `u128`.
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

        /// Returns the coefficients of the recurrence the code's `k`-face count obeys along the level, every face and the Euler characteristic when `k` is none.
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

        /// Returns the exposed face count of the code's 3D fractal at the given level.
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

        /// Returns the empty cell count, grid minus fill.
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
            m.setattr("__doc__", "The closed-form counts.\n\nEvery number the censuses reach by walking a built cell, this folder reaches by formula:\nfills, grids, exposed surfaces, hex slices and the base-q carry ladder.")?;
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

    /// The digit designs on the integer line: their elements, their Mobius meter, its density echo and the ordinates its spectrum carries.
    pub mod design {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the digits a bitmask names inside the base, ascending.
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

        /// Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element.
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

        /// Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.
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

        /// Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last.
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

        /// Returns the running median of the power over a window of the given width, the window clamped at the ends.
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

        /// Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.
        #[pyfunction]
        #[pyo3(name = "meter", signature = (mu))]
        pub fn meter<'py>(py: Python<'py>, mu: Vec<i8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::design::meter(&mu);
            (out).into_bound_py_any(py)
        }

        /// Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty.
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

        /// Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first.
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

        /// Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.
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

        /// Reads the running meter at every point of the log grid and divides by x to the exponent.
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

        /// Returns the power over its local median floor, the score a peak is read against.
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

        /// Returns the count of elements the design holds at the depth, the length `elements` returns without building them.
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

        /// Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range.
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

        /// Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at.
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

    /// The unconditional dissection of a missing-digit set: the four regions of the frequency grid, the `l^1` masses of the digit transform, the chain certificate against `1/5`, the constant `kappa_F` and the set's own meter and prime count.
    pub mod dissection {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns `Q = floor(y^(3/5))`, the largest denominator the dissection admits, exact in integers.
        #[pyfunction]
        #[pyo3(name = "cap", signature = (y))]
        pub fn cap<'py>(py: Python<'py>, y: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::cap(y);
            (out).into_bound_py_any(py)
        }

        /// Returns the chain's certificate exponent at one missing digit, `alpha_1 = log_base(z base/(base - 1))`, the same at every missing digit.
        #[pyfunction]
        #[pyo3(name = "chain_exponent", signature = (base))]
        pub fn chain_exponent<'py>(py: Python<'py>, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_exponent(base);
            (out).into_bound_py_any(py)
        }

        /// Returns the chain's margin at the bar, the cubic cleared of denominators at `w = base^(1/5)(1 - 1/base)`, positive exactly when the root sits below `w` and `alpha_1 < 1/5`.
        #[pyfunction]
        #[pyo3(name = "chain_margin", signature = (base))]
        pub fn chain_margin<'py>(py: Python<'py>, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_margin(base);
            (out).into_bound_py_any(py)
        }

        /// Returns the root `z > 1` of the digit-uniform chain at one missing digit, `(z-1)^3 = (2/pi)(log base) z + gamma'(z-1) + (2/pi)(z-1)^2/(base z - 1)`, by bisection.
        #[pyfunction]
        #[pyo3(name = "chain_root", signature = (base))]
        pub fn chain_root<'py>(py: Python<'py>, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_root(base);
            (out).into_bound_py_any(py)
        }

        /// Returns the chain's wall, one past the last base up to `CAP_BASE` whose margin is not positive.
        #[pyfunction]
        #[pyo3(name = "chain_wall", signature = ())]
        pub fn chain_wall<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::chain_wall();
            (out).into_bound_py_any(py)
        }

        /// Returns whether the digit set keeps two consecutive digits, the hypothesis region C1 reads.
        #[pyfunction]
        #[pyo3(name = "consecutive", signature = (digits))]
        pub fn consecutive<'py>(py: Python<'py>, digits: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::consecutive(&digits);
            (out).into_bound_py_any(py)
        }

        /// Returns the last continued-fraction convergent `l/d` of `a/y` whose denominator is at most the cap, the Dirichlet fraction of the dissection.
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

        /// Returns `kappa_F = (base/phi(base)) #{f in F : gcd(f, base) = 1}/fill`, the main-term constant of the prime count, as a reduced fraction.
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

        /// Returns the unshifted masses `c_j = sum_(a < base^j) |hat F_j(a/base^j)|` for `j = 0..=level`, the `l^1` mass region A pays, `c_0 = 1`.
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

        /// Returns how the theorem reaches the set missing one digit: `proof` from the chain's wall, `certificate` at every base from `DIGIT_WALL` below it and at the `CERTIFIED` sets, `none` elsewhere.
        #[pyfunction]
        #[pyo3(name = "reach", signature = (base, missing))]
        pub fn reach<'py>(py: Python<'py>, base: u64, missing: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::dissection::reach(base, missing);
            ((out).to_string()).into_bound_py_any(py)
        }

        /// Returns the `l^1` exponent the top two masses read, `log_base(c_j/(fill c_(j-1)))`: a reading of the growth region A pays, never a certificate.
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

        /// Returns the region of every frequency `a/y`, `a < y = base^level`, at the cut `Z`, the fraction taken by `fraction` at `cap`.
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

        /// Tallies the set below `base^level` on a log grid of the given size, with the Mobius values and the primes sieved to the span.
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

        /// Returns `|hat F_level(a/base^level)|` at every `a < base^level`, built one digit at a time from `hat F_j(t) = hat F(t) hat F_(j-1)(base t)`.
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
            m.setattr("__doc__", "The unconditional dissection of a missing-digit set: the four regions of the frequency grid, the `l^1` masses of the digit transform, the chain certificate against `1/5`, the constant `kappa_F` and the set's own meter and prime count.")?;
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

    /// The chaos game of a design: the shift register that writes its digits backwards, the de Bruijn schedules that paint every cell once, the restricted games that draw the memory dial's sets, and the games on a graph that draw a mosaic or a slice.
    pub mod game {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the index of the level-`k` cell holding a point of `[0, 1)^D`: `floor(x b^k) + floor(y b^k) b^k + ...`, row major with coordinate `0` least significant, the dimension read off the point.
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

        /// Returns the count of level-`k` cells of the cube, `b^(D k)`.
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

        /// Returns the path counts from a state, `e_start M^L 1` for `L = 1 ..= levels`: the filled cells of a mosaic slot or of a slice at each level, and stops early on the level whose count overruns a `u64`.
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

        /// Returns a de Bruijn cycle of order `k` over `m` letters: `m^k` letters in which every word of length `k` occurs exactly once as a cyclic window, so played as the draw schedule of an `m`-digit design it paints every level-`k` cell once per lap, and from a cold start once in `m^k + k - 1` draws.
        #[pyfunction]
        #[pyo3(name = "de_bruijn", signature = (m, k))]
        pub fn de_bruijn<'py>(py: Python<'py>, m: u32, k: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::de_bruijn(m, k);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Reads the game of a mosaic: its substitution matrix, Perron root, dimension and edges.
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

        /// Returns the named games of the square, corner `c = x + 2 y`, cyclic order `0, 1, 3, 2` anticlockwise with `y` up: the seven one-step games and the width-3 game that forbids a neighbour after a repeat.
        #[pyfunction]
        #[pyo3(name = "named_games", signature = ())]
        pub fn named_games<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::named_games();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the Perron root of a square nonnegative integer matrix, the growth per level of the paths its graph counts.
        #[pyfunction]
        #[pyo3(name = "perron", signature = (matrix))]
        pub fn perron<'py>(py: Python<'py>, matrix: Vec<Vec<u64>>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::perron(&matrix);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the rule whose windows are the rule's windows read backwards.
        #[pyfunction]
        #[pyo3(name = "reverse", signature = (rule))]
        pub fn reverse<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::game::reverse(&rule.0);
            (crate::doors::math::memory::Rule(out)).into_bound_py_any(py)
        }

        /// Reads the game of a plane design, corners `c = x + b y`, cut by the line `x + y = p / q`: its carry states, edges, matrix, Perron root and dimension.
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

        /// Returns the letters a restricted game may draw next: the corners `c` for which some allowed window, read in time order, ends in the last `k - 1` draws of the history followed by `c`.
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

        /// Returns the cell of a word of corners read coarsest digit first, the cell the game lands in after playing the word's digits last letter first.
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
            m.setattr("__doc__", "The chaos game of a design: the shift register that writes its digits backwards, the de Bruijn schedules that paint every cell once, the restricted games that draw the memory dial's sets, and the games on a graph that draw a mosaic or a slice.\n\nA design at base `b` has one map per digit, `phi_d(x) = (x + d) / b`, and the game plays `x_n = phi_(d_n)(x_(n-1))` from a start in `[0, 1)^D`.\nEach draw becomes the new leading digit, so for `n >= k` the level-`k` cell of `x_n` is the word of the last `k` draws read newest first.\nA corner is the integer `c = sum_i d_i b^i`, coordinate `0` its least significant digit, the `c = x + b y` of the plane designs, and a cell index is row major, `y side + x` at `side = b^k`, as `crate::memory::cells` reads it.\nA restricted game is a rule of `crate::memory` on the draws in time order, the oldest draw the most significant digit of the window, and the set it draws is the dial's set of the reversed rule.\nA game on a graph keeps a state, draws an edge into it, plays the edge's digit and moves to the edge's source: the mosaic game on its slots, the slice game on its carry heights.")?;
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

    /// The spatial network.
    pub mod graph {
        use crate::hand::{PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A force-directed layout: every node repels every other, every branch pulls its ends together, and a cooling cap on the move per tick lets the lattice settle.
        #[pyclass(name = "Layout", module = "mrly.math.graph", from_py_object)]
        #[derive(Clone)]
        pub struct Layout(pub mrly_math::graph::Layout);

        #[pymethods]
        impl Layout {
            /// Starts a layout from flat positions, `dim` floats per node, and the branch pairs.
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
            /// Returns the mean net force per node in units of `k` after the last tick.
            #[pyo3(name = "energy", signature = ())]
            pub fn energy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::energy(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Starts a layout from a network's own positions and branches.
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
            /// Returns the ideal branch length `k`.
            #[pyo3(name = "ideal", signature = ())]
            pub fn ideal<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::ideal(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the mean distance a node moved in the last tick.
            #[pyo3(name = "moved", signature = ())]
            pub fn moved<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::moved(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Starts a layout from flat positions, `dim` floats per node, and the branch pairs.
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
            /// Returns the node count.
            #[pyo3(name = "nodes", signature = ())]
            pub fn nodes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::nodes(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the positions, `dim` floats per node.
            #[pyo3(name = "positions", signature = ())]
            pub fn positions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::positions(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Runs the ticks and returns the energy left: the mean net force per node in units of `k`.
            #[pyo3(name = "step", signature = (ticks))]
            pub fn step<'py>(
                &mut self,
                py: Python<'py>,
                ticks: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::step(&mut self.0, ticks);
                (out).into_bound_py_any(py)
            }
            /// Returns the cap on one node's move in the next tick.
            #[pyo3(name = "temperature", signature = ())]
            pub fn temperature<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::temperature(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the ticks stepped so far.
            #[pyo3(name = "ticks", signature = ())]
            pub fn ticks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Layout::ticks(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        /// A spatial graph of nodes and branches.
        #[pyclass(name = "Network", module = "mrly.math.graph", from_py_object)]
        #[derive(Clone)]
        pub struct Network(pub mrly_math::graph::Network);

        #[pymethods]
        impl Network {
            /// Builds an empty network of the given dimension.
            #[new]
            #[pyo3(signature = (dim))]
            pub fn __new__(dim: usize) -> PyResult<Self> {
                let out = mrly_math::graph::Network::new(dim);
                Ok(Self(out))
            }
            /// The dimension every position must match.
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
            /// The nodes in insertion order.
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
            /// The branches in insertion order.
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
            /// Appends a branch between two node indices.
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
            /// Appends a node at the position and returns its index.
            #[pyo3(name = "add_node", signature = (position))]
            pub fn add_node<'py>(
                &mut self,
                py: Python<'py>,
                position: Vec<f64>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::add_node(&mut self.0, position);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the undirected neighbor lists of every node.
            #[pyo3(name = "adjacency", signature = ())]
            pub fn adjacency<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::adjacency(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns each node's branch count, indexed like the node list.
            #[pyo3(name = "degree", signature = ())]
            pub fn degree<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::degree(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Builds an empty network of the given dimension.
            #[staticmethod]
            #[pyo3(name = "new", signature = (dim))]
            pub fn new_<'py>(py: Python<'py>, dim: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::graph::Network::new(dim);
                (crate::doors::math::graph::Network(out)).into_bound_py_any(py)
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
        pub fn census<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::census(&network.0);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts the connected components of the network.
        #[pyfunction]
        #[pyo3(name = "components", signature = (network))]
        pub fn components<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::components(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Extracts the network of filled sites joined to their axis neighbors.
        #[pyfunction]
        #[pyo3(name = "core_graph", signature = (grid))]
        pub fn core_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrly_math::graph::core_graph(&grid);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Extracts the network of corners and edges outlining every filled site.
        #[pyfunction]
        #[pyo3(name = "edge_graph", signature = (grid))]
        pub fn edge_graph<'py>(py: Python<'py>, grid: PyTensor) -> PyResult<Bound<'py, PyAny>> {
            let grid = grid.0;
            let out = mrly_math::graph::edge_graph(&grid);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample.
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

        /// Counts the nodes of degree three or more.
        #[pyfunction]
        #[pyo3(name = "junctions", signature = (network))]
        pub fn junctions<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::junctions(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Extracts the largest connected piece as a network of its own, branches re-indexed.
        #[pyfunction]
        #[pyo3(name = "largest_component", signature = (network))]
        pub fn largest_component<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::largest_component(&network.0);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Tags every node by its degree, indexed like the node list.
        #[pyfunction]
        #[pyo3(name = "roles", signature = (network))]
        pub fn roles<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::roles(&network.0);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Counts the nodes of degree one.
        #[pyfunction]
        #[pyo3(name = "tips", signature = (network))]
        pub fn tips<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::tips(&network.0);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Sums the straight-line lengths of every branch.
        #[pyfunction]
        #[pyo3(name = "total_length", signature = (network))]
        pub fn total_length<'py>(
            py: Python<'py>,
            network: PyRef<'_, crate::doors::math::graph::Network>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::graph::total_length(&network.0);
            (out).into_bound_py_any(py)
        }

        /// Extracts the core graph of the inverted grid, joining empty sites instead.
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
            m.setattr("__doc__", "The spatial network.\n\nA grid lifts into nodes and branches, as a core, an edge or a tunnel network, and the census\nreads its roles, tips, junctions and components back.")?;
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

    /// The peeled ladder: the Dirichlet series of a digit design continued to the whole plane, its Lyndon cofactor and the residues at its poles, each value carrying its bound.
    pub mod ladder {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A digit design: the base `q`, the digit set `F` its elements are written with, and the peel depth `P` its ladder starts at.
        #[pyclass(name = "Design", module = "mrly.math.ladder", from_py_object)]
        #[derive(Clone)]
        pub struct Design(pub mrly_math::ladder::Design);

        #[pymethods]
        impl Design {
            /// Builds a design on the base and the digit set, choosing the peel depth.
            #[new]
            #[pyo3(signature = (base, digits))]
            pub fn __new__(base: u64, digits: Vec<u64>) -> PyResult<Self> {
                let out = mrly_math::ladder::Design::new(base, &digits);
                Ok(Self(ok(out)?))
            }
            /// Returns the abscissa `alpha = log_q k`.
            #[pyo3(name = "abscissa", signature = ())]
            pub fn abscissa<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::abscissa(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the base.
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::base(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the digit set, ascending.
            #[pyo3(name = "digits", signature = ())]
            pub fn digits<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::digits(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Builds a design on the base and the digit set, choosing the peel depth.
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
            /// Returns the peel depth.
            #[pyo3(name = "peel", signature = ())]
            pub fn peel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::peel(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the pole spacing `2 pi / log q`.
            #[pyo3(name = "period", signature = ())]
            pub fn period<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::ladder::Design::period(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`.
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
            /// Builds a design at an explicit peel depth, at least two.
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

        /// Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to.
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

        /// Returns `zeta_F(s)` and the bound it is known to.
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
            m.setattr("__doc__", "The peeled ladder: the Dirichlet series of a digit design continued to the whole plane, its Lyndon cofactor and the residues at its poles, each value carrying its bound.\n\nA claim witness with no caller in the crate: it witnesses the [zeros of the design zeta claim](https://github.com/mrlyprod/mrlyprod/blob/main/research/zeta/claims/zeros-of-the-design-zeta.md).")?;
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

    /// The memory designs: a rule on `k` consecutive digits, its transfer matrix, the words it accepts and the Perron root that is their dimension.
    pub mod memory {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A rule on `k` consecutive digits of a design word.
        #[pyclass(name = "Rule", module = "mrly.math.memory", from_py_object)]
        #[derive(Clone)]
        pub struct Rule(pub mrly_math::memory::Rule);

        #[pymethods]
        impl Rule {
            /// Builds a rule from its dimension, its width and its code.
            #[new]
            #[pyo3(signature = (dimension, width, code))]
            pub fn __new__(dimension: usize, width: usize, code: u64) -> PyResult<Self> {
                let out = mrly_math::memory::Rule::new(dimension, width, code);
                Ok(Self(ok(out)?))
            }
            /// The dimension `D`, one to three.
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
            /// The window width `k`, at least one.
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
            /// The window code, bit `w` set when window `w` is allowed.
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
            /// Returns whether a word, coarsest digit first, is accepted.
            #[pyo3(name = "accepts", signature = (word))]
            pub fn accepts<'py>(
                &self,
                py: Python<'py>,
                word: Vec<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::accepts(&self.0, &word);
                (out).into_bound_py_any(py)
            }
            /// Returns whether the window is allowed, and false for any window out of range.
            #[pyo3(name = "allowed", signature = (window))]
            pub fn allowed<'py>(
                &self,
                py: Python<'py>,
                window: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::allowed(&self.0, window);
                (out).into_bound_py_any(py)
            }
            /// Returns the letters that stand in at least one allowed window.
            #[pyo3(name = "alphabet", signature = ())]
            pub fn alphabet<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::alphabet(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of rules of this shape, `2^(2^(k D))`.
            #[pyo3(name = "codes", signature = ())]
            pub fn codes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::codes(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the rule that allows every window.
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
            /// Returns the letter count `2^D`, the digit vectors of the cube's corners.
            #[pyo3(name = "letters", signature = ())]
            pub fn letters<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::letters(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a rule from its dimension, its width and its code.
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
            /// Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on.
            #[pyo3(name = "states", signature = ())]
            pub fn states<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::states(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the window count `2^(k D)`.
            #[pyo3(name = "windows", signature = ())]
            pub fn windows<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::memory::Rule::windows(&self.0);
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
        pub fn allowed_windows<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::allowed_windows(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first.
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

        /// Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`.
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

        /// Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count.
        #[pyfunction]
        #[pyo3(name = "exponent", signature = (rule))]
        pub fn exponent<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::exponent(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory.
        #[pyfunction]
        #[pyo3(name = "kappa", signature = (rule))]
        pub fn kappa<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::kappa(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the Perron root of the transfer matrix, the count's growth per level.
        #[pyfunction]
        #[pyo3(name = "perron", signature = (rule))]
        pub fn perron<'py>(
            py: Python<'py>,
            rule: PyRef<'_, crate::doors::math::memory::Rule>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::memory::perron(&rule.0);
            (out).into_bound_py_any(py)
        }

        /// Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed.
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
            m.setattr("__doc__", "The memory designs: a rule on `k` consecutive digits, its transfer matrix, the words it accepts and the Perron root that is their dimension.")?;
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

    /// The moire fields.
    pub mod moire {
        use crate::hand::{PyImage, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The exact correlations of flat carpet layers, and the prime detector they make.
        pub mod pairs {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid.
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

            /// Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated.
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

            /// Puts an odd scale of three or more on trial against every earlier odd scale.
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
                m.setattr("__doc__", "The exact correlations of flat carpet layers, and the prime detector they make.")?;
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

        /// The lattice coordinates and code-membership tests behind the layers.
        pub mod sample {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Returns the two lattice coordinates of each pixel centre along a row.
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

            /// Unpacks a code into its residue-corner truth table.
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

            /// Folds residues into a base-q index of the truth table.
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
                m.setattr(
                    "__doc__",
                    "The lattice coordinates and code-membership tests behind the layers.",
                )?;
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

        /// A square grid of f32 samples.
        #[pyclass(name = "Field", module = "mrly.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Field(pub mrly_math::moire::Field);

        #[pymethods]
        impl Field {
            /// Builds a zeroed field of the given side.
            #[new]
            #[pyo3(signature = (size))]
            pub fn __new__(size: usize) -> PyResult<Self> {
                let out = mrly_math::moire::Field::new(size);
                Ok(Self(out))
            }
            /// The samples in row-major order.
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
            /// The side length in samples.
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
            /// Returns the samples widened to f64.
            #[pyo3(name = "as_f64", signature = ())]
            pub fn as_f64<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::as_f64(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Wraps row-major samples of the given side.
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
            /// Returns the largest sample.
            #[pyo3(name = "max", signature = ())]
            pub fn max<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::max(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the mean sample, or zero for an empty field.
            #[pyo3(name = "mean", signature = ())]
            pub fn mean<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::mean(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the smallest sample.
            #[pyo3(name = "min", signature = ())]
            pub fn min<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::min(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a zeroed field of the given side.
            #[staticmethod]
            #[pyo3(name = "new", signature = (size))]
            pub fn new_<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::new(size);
                (crate::doors::math::moire::Field(out)).into_bound_py_any(py)
            }
            /// Returns the samples scaled into 0..1, symmetric about zero on request.
            #[pyo3(name = "normalized", signature = (symmetric))]
            pub fn normalized<'py>(
                &self,
                py: Python<'py>,
                symmetric: bool,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Field::normalized(&self.0, symmetric);
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
        #[pyclass(name = "Preset", module = "mrly.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Preset(pub mrly_math::moire::Preset);

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
            #[setter]
            #[pyo3(name = "spec")]
            pub fn set_spec(&mut self, value: PySerde<mrly_math::moire::Spec>) -> PyResult<()> {
                let value = value.0;
                self.0.spec = value;
                Ok(())
            }
            /// The side numbers stacked.
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
            /// The way the layers merge.
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
            /// The fractal depth of each layer.
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
            /// The lattice the layers are sampled on.
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
            /// The carpet stack: every base-three corner but the centre, summed over odd scales.
            #[staticmethod]
            #[pyo3(name = "carpet", signature = (limit))]
            pub fn carpet<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::carpet(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// Samples the preset into a square field of the given side.
            #[pyo3(name = "field", signature = (size))]
            pub fn field<'py>(&self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::field(&self.0, size);
                (crate::doors::math::moire::Field(ok(out)?)).into_bound_py_any(py)
            }
            /// The parity heatmap: odd scales of the low corner summed on the square lattice.
            #[staticmethod]
            #[pyo3(name = "heatmap", signature = (limit))]
            pub fn heatmap<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::heatmap(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// The hive: the parity heatmap sampled on the hexagonal lattice.
            #[staticmethod]
            #[pyo3(name = "hive", signature = (limit))]
            pub fn hive<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::hive(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// The parity weave: the same odd scales folded to their parity instead of summed.
            #[staticmethod]
            #[pyo3(name = "weave", signature = (limit))]
            pub fn weave<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Preset::weave(limit);
                (crate::doors::math::moire::Preset(out)).into_bound_py_any(py)
            }
            /// Returns the value as plain data.
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        /// A cubic grid of f32 samples, x-major.
        #[pyclass(name = "Volume", module = "mrly.math.moire", from_py_object)]
        #[derive(Clone)]
        pub struct Volume(pub mrly_math::moire::Volume);

        #[pymethods]
        impl Volume {
            /// Builds a zeroed volume of the side.
            #[new]
            #[pyo3(signature = (size))]
            pub fn __new__(size: usize) -> PyResult<Self> {
                let out = mrly_math::moire::Volume::new(size);
                Ok(Self(out))
            }
            /// The samples, x-major, then y, then z.
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
            /// The side in samples.
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
            /// Reads the sample at a voxel.
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
            /// Counts the samples at or above the level.
            #[pyo3(name = "count", signature = (level))]
            pub fn count<'py>(&self, py: Python<'py>, level: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::count(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Wraps x-major samples of the side.
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
            /// Returns the largest sample.
            #[pyo3(name = "max", signature = ())]
            pub fn max<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::max(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the smallest sample.
            #[pyo3(name = "min", signature = ())]
            pub fn min<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::min(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds a zeroed volume of the side.
            #[staticmethod]
            #[pyo3(name = "new", signature = (size))]
            pub fn new_<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::new(size);
                (crate::doors::math::moire::Volume(out)).into_bound_py_any(py)
            }
            /// Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube.
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
            /// Reads the voxel a point of the unit cube falls in, or zero outside it.
            #[pyo3(name = "sample", signature = (p))]
            pub fn sample<'py>(&self, py: Python<'py>, p: [f64; 3]) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::sample(&self.0, p);
                (out).into_bound_py_any(py)
            }
            /// Thresholds into a byte tensor: one where a sample reaches the level, zero below.
            #[pyo3(name = "solid", signature = (level))]
            pub fn solid<'py>(&self, py: Python<'py>, level: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::moire::Volume::solid(&self.0, level);
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
        #[pyclass(name = "Layer", module = "mrly.math.moire", skip_from_py_object)]
        pub struct Layer;

        #[pymethods]
        impl Layer {
            /// Builds a layer at level 1 on a 512-pixel square lattice.
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

        /// The identity of a design: its code, base and dimension.
        #[pyclass(name = "Spec", module = "mrly.math.moire", skip_from_py_object)]
        pub struct Spec;

        #[pymethods]
        impl Spec {
            /// Builds a spec from a code, base and dimension.
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

        /// Returns every preset stacked up to the given scale.
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

        /// Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal.
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

        /// Samples a design over the pixel grid into a boolean mask.
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

        /// Returns the preset the name picks.
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

        /// Quantizes a field into colored levels and renders them as an image, each sample a scale by scale block.
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

        /// Layers one design at several side numbers into a field under the chosen combine.
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

        /// Sums layers of several designs at one side number into a field.
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

        /// Layers one cube design at several side numbers into a volume under the chosen combine.
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
            m.setattr("__doc__", "The moire fields.\n\nOne design sampled at many scales and stacked makes an interference pattern; the layers, their\ncombination, the volume they cut and the image they render live here.")?;
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

    /// The design names: the bang, sequence and word kinds of the canonical JSON object.
    pub mod name {
        use crate::hand::{PyCode, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A design code pinned to its dimension, lattice and base, with one unit index per filled digit when it twists.
        #[pyclass(name = "Bang", module = "mrly.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Bang(pub mrly_math::name::Bang);

        #[pymethods]
        impl Bang {
            /// Pins a code to its dimension and base on the square lattice.
            #[new]
            #[pyo3(signature = (code, dim, base))]
            pub fn __new__(code: u128, dim: usize, base: usize) -> PyResult<Self> {
                let out = mrly_math::name::Bang::new(code, dim, base);
                Ok(Self(out))
            }
            /// The number of axes.
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
            /// The lattice, square unless said.
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
            /// The digits per axis, 2 unless said.
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
            /// The design as a number.
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
            /// One unit index per filled digit, absent when nothing turns.
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
            /// Returns the number of digits the code addresses.
            #[pyo3(name = "cells", signature = ())]
            pub fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Bang::cells(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Folds a decoded value to its canonical form, or an error for one outside the kind.
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    <mrly_math::name::Bang as mrly_core::name::Named>::checked(self.0.clone());
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a filename back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::from_file(text);
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a JSON object into its canonical value, or an error naming the broken key.
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::from_json(text);
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a path and query string back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::from_url(text);
                (crate::doors::math::name::Bang(ok(out)?)).into_bound_py_any(py)
            }
            /// Pins a code to its dimension and base on the square lattice.
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
            /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the first eight hex digits of the sha256 of the canonical JSON.
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the canonical JSON object.
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
            #[pyo3(name = "to_prose", signature = ())]
            pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_prose(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Bang as mrly_core::name::Named>::to_url(&self.0);
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
        #[pyclass(name = "Sequence", module = "mrly.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Sequence(pub mrly_math::name::Sequence);

        #[pymethods]
        impl Sequence {
            /// Pins a design's reading to its measure and axis.
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
            /// The number of axes.
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
            /// The digits per axis, 2 unless said.
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
            /// The design as a number.
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
            /// The reading taken.
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
            /// The index the reading runs along.
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
            /// Folds a decoded value to its canonical form, or an error for one outside the kind.
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    <mrly_math::name::Sequence as mrly_core::name::Named>::checked(self.0.clone());
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the design pinned to its dimension and base.
            #[pyo3(name = "design", signature = ())]
            pub fn design<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Sequence::design(&self.0);
                (crate::doors::math::name::Bang(out)).into_bound_py_any(py)
            }
            /// Reads a filename back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::from_file(text);
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a JSON object into its canonical value, or an error naming the broken key.
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::from_json(text);
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a path and query string back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::from_url(text);
                (crate::doors::math::name::Sequence(ok(out)?)).into_bound_py_any(py)
            }
            /// Pins a design's reading to its measure and axis.
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
            /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the first eight hex digits of the sha256 of the canonical JSON.
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the canonical JSON object.
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
            #[pyo3(name = "to_prose", signature = ())]
            pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_prose(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Sequence as mrly_core::name::Named>::to_url(&self.0);
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
        #[pyclass(name = "Word", module = "mrly.math.name", from_py_object)]
        #[derive(Clone)]
        pub struct Word(pub mrly_math::name::Word);

        #[pymethods]
        impl Word {
            /// Pins an ordered letter list at base 2.
            #[new]
            #[pyo3(signature = (dim, letters))]
            pub fn __new__(dim: usize, letters: Vec<(u128, usize)>) -> PyResult<Self> {
                let out = mrly_math::name::Word::new(dim, &letters);
                Ok(Self(ok(out)?))
            }
            /// The number of axes every letter shares.
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
            /// The codes of the letters in order.
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
            /// The side each letter renders at.
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
            /// The base of each letter, absent when every letter is base 2.
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
            /// Returns the base of every letter, 2 where the name says nothing.
            #[pyo3(name = "bases", signature = ())]
            pub fn bases<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Word::bases(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Folds a decoded value to its canonical form, or an error for one outside the kind.
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    <mrly_math::name::Word as mrly_core::name::Named>::checked(self.0.clone());
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a filename back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::from_file(text);
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a JSON object into its canonical value, or an error naming the broken key.
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::from_json(text);
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Reads a path and query string back into the value, or an error.
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::from_url(text);
                (crate::doors::math::name::Word(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns every letter as a design pinned to the word's dimension and its own base.
            #[pyo3(name = "letters", signature = ())]
            pub fn letters<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Word::letters(&self.0);
                ((out)
                    .into_iter()
                    .map(crate::doors::math::name::Bang)
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }
            /// Pins an ordered letter list at base 2.
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
            /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the first eight hex digits of the sha256 of the canonical JSON.
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the canonical JSON object.
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
            #[pyo3(name = "to_prose", signature = ())]
            pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_prose(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_math::name::Word as mrly_core::name::Named>::to_url(&self.0);
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
        #[pyclass(name = "Lattice", module = "mrly.math.name", skip_from_py_object)]
        pub struct Lattice;

        #[pymethods]
        impl Lattice {
            /// Returns the default Lattice.
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::name::Lattice::default();
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns whether this is the square lattice.
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
            /// Returns the number of unit directions a twist may pick from.
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

        /// Returns the word of the first named design a code draws in the plane or the cube, or None for a code no named design draws.
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

        /// Lists every classic then anti design of the plane or the cube as its decimal code and its word, empty for any other dimension.
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
            m.setattr("__doc__", "The design names: the bang, sequence and word kinds of the canonical JSON object.\n\nThe `Named` trait, the `kind!` marker and the text views live in `mrly_core::name`.")?;
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

    /// The Chladni figures of a design: the lowest modes of the plate it cuts, in a canonical basis, with their nodal domains, Courant bounds and rasters.
    pub mod nodal {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Rings the plate cut as the design `math::two::create(code, number, level, 0, 2)`: its cells joined to their four neighbours, free at the rim and at every hole, the combinatorial Laplacian `D - A`. Returns the lowest `count` modes, `count` clamped to the cells and to 64, each in a canonical basis: eigenvalues within `1e-8` form a cluster, split by the design's transpose, left-right and up-down mirrors in that order and then by the diagonal `x + sqrt(2) y`, each figure projected exactly onto its mirror parities and signed so its first largest cell is positive. A figure's domains are the 4-connected runs of one strict sign, cells within `1e-9` of its peak left out; its bound is the 1-based index of the last member of its cluster, the last cluster closed before the cut.
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

        /// Answers whether a 2D base-2 design cuts a plate the figures can ring: one 4-connected piece of 16 to 10,000 cells on a side of at most 128. The cells are counted before the design is built.
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
            m.setattr("__doc__", "The Chladni figures of a design: the lowest modes of the plate it cuts, in a canonical basis, with their nodal domains, Courant bounds and rasters.")?;
            m.add_function(wrap_pyfunction!(figures, &m)?)?;
            m.add_function(wrap_pyfunction!(holds, &m)?)?;
            let names: Vec<&str> = vec!["figures", "holds"];
            m.add("__all__", names)?;
            parent.add("nodal", &m)?;
            sys.set_item("mrly.math._math.math.nodal", &m)?;
            Ok(())
        }
    }

    /// The sequence press: the integers a design's digit rule keeps, weighed all at once.
    pub mod press {
        use crate::hand::{PyCode, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The tally press: one pass over the integers weighs every design of a universe at once.
        #[pyclass(name = "Press", module = "mrly.math.press", skip_from_py_object)]
        pub struct Press(pub mrly_math::press::Press);

        #[pymethods]
        impl Press {
            /// Builds an empty press over every design of the dimension and base.
            #[new]
            #[pyo3(signature = (dimension, base))]
            pub fn __new__(dimension: usize, base: usize) -> PyResult<Self> {
                let out = mrly_math::press::Press::new(dimension, base);
                Ok(Self(ok(out)?))
            }
            /// The design dimension of the universe.
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
            /// The numeral base of the universe.
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
            /// Adds a weighted number to its usage bucket.
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
            /// Builds an empty press over every design of the dimension and base.
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
            /// Returns the total weight the design at a code has collected.
            #[pyo3(name = "total", signature = (code))]
            pub fn total<'py>(&self, py: Python<'py>, code: PyCode) -> PyResult<Bound<'py, PyAny>> {
                let code = code.0;
                let out = mrly_math::press::Press::total(&self.0, code);
                (out).into_bound_py_any(py)
            }
            /// Returns every design's total in code order by one subset-sum transform.
            #[pyo3(name = "totals", signature = ())]
            pub fn totals<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::press::Press::totals(&self.0);
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
        pub fn containing<'py>(
            py: Python<'py>,
            number: u128,
            dimension: usize,
            base: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::press::containing(number, dimension, base);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Splits a number into its dimension coordinates, one base digit peeled per axis in parallel.
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

        /// Counts the members of a design below the limit.
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

        /// Returns the count of distinct digit vectors the number uses.
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

        /// Weaves dimension coordinates back into their single interleaved number.
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

        /// Returns the allowed digit table of one magic layer, one flag per cell of its tile.
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

        /// Returns whether every digit vector of the number lies in the design.
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

        /// Returns the first members of a design in ascending order.
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

        /// Returns the diagonal slice profile of one design pressed to a fractal level.
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

        /// Returns the corner-usage mask of a number, one bit per digit vector its expansion uses.
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

        /// Counts the members of a magic word from its layer fills, without enumeration.
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

        /// Returns whether the number lies in the magic word's composed design.
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

        /// Enumerates every member of the magic word in ascending order.
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

        /// Returns the diagonal slice profile of a magic word by the substitution product.
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

    /// The radix designs: a digit set inside the residues of a base in a ring, a unit twist per digit, and the points their words land on.
    pub mod radix {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The base of a radix design: a ring and an element of norm at least two, the scale every word is read against.
        #[pyclass(name = "Base", module = "mrly.math.radix", from_py_object)]
        #[derive(Clone)]
        pub struct Base(pub mrly_math::radix::Base);

        #[pymethods]
        impl Base {
            /// Fixes a base in a ring.
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
            /// Returns the index in the canonical residue system of the class of a point.
            #[pyo3(name = "class_", signature = (z))]
            pub fn class_<'py>(
                &self,
                py: Python<'py>,
                z: (i64, i64),
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::class(self.0, z);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns whether two points are congruent modulo the base.
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
            /// Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base.
            #[pyo3(name = "group", signature = ())]
            pub fn group<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::group(self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group.
            #[pyo3(name = "mirrored", signature = ())]
            pub fn mirrored<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::mirrored(self.0);
                (out).into_bound_py_any(py)
            }
            /// Fixes a base in a ring.
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
            /// Returns the norm `q` of the base: the count of residue classes and the square of the scale.
            #[pyo3(name = "norm", signature = ())]
            pub fn norm<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::norm(self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the base raised to a level.
            #[pyo3(name = "power", signature = (level))]
            pub fn power<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::power(self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`.
            #[pyo3(name = "residues", signature = ())]
            pub fn residues<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::residues(self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the ring.
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::ring(self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the base element.
            #[pyo3(name = "value", signature = ())]
            pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Base::value(self.0);
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
        #[pyclass(name = "Radix", module = "mrly.math.radix", from_py_object)]
        #[derive(Clone)]
        pub struct Radix(pub mrly_math::radix::Radix);

        #[pymethods]
        impl Radix {
            /// Builds a design from a base, a digit list and a unit twist per digit.
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
            /// Returns the base.
            #[pyo3(name = "base", signature = ())]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::base(&self.0);
                (crate::doors::math::radix::Base(out)).into_bound_py_any(py)
            }
            /// Returns whether every digit is the canonical representative of its class.
            #[pyo3(name = "canonical", signature = ())]
            pub fn canonical<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::canonical(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives.
            #[pyo3(name = "code", signature = ())]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::code(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Returns the digits.
            #[pyo3(name = "digits", signature = ())]
            pub fn digits<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::digits(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base.
            #[pyo3(name = "dimension", signature = ())]
            pub fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::dimension(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point.
            #[pyo3(name = "distinct", signature = (level))]
            pub fn distinct<'py>(
                &self,
                py: Python<'py>,
                level: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::distinct(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of words of a level, `|F|^L`.
            #[pyo3(name = "fill", signature = (level))]
            pub fn fill<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::fill(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`.
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
            /// Builds a design from a base, a digit list and a unit twist per digit.
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
            /// Returns the level-`L` points in the plane, the scaled words divided by `b^L`.
            #[pyo3(name = "plane", signature = (level))]
            pub fn plane<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::plane(&self.0, level);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring.
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::ring(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the digit count `|F|`.
            #[pyo3(name = "size", signature = ())]
            pub fn size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::size(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the twists.
            #[pyo3(name = "twists", signature = ())]
            pub fn twists<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::twists(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
            }
            /// Returns the design with the twists named by their index in the unit list, the units in turning order from one.
            #[pyo3(name = "with_twists", signature = (units))]
            pub fn with_twists<'py>(
                &self,
                py: Python<'py>,
                units: Vec<usize>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::with_twists(self.0.clone(), &units);
                (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the level-`L` points in exact ring coordinates scaled by `b^L`.
            #[pyo3(name = "words", signature = (level))]
            pub fn words<'py>(&self, py: Python<'py>, level: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::radix::Radix::words(&self.0, level);
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
            let out = mrly_math::radix::flowsnake();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`.
        #[pyfunction]
        #[pyo3(name = "gasket", signature = ())]
        pub fn gasket<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::gasket();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`.
        #[pyfunction]
        #[pyo3(name = "koch", signature = ())]
        pub fn koch<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::koch();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`.
        #[pyfunction]
        #[pyo3(name = "terdragon", signature = ())]
        pub fn terdragon<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::terdragon();
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`.
        #[pyfunction]
        #[pyo3(name = "tile", signature = (m, code))]
        pub fn tile<'py>(py: Python<'py>, m: u64, code: u128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::radix::tile(m, code);
            (crate::doors::math::radix::Radix(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`.
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
            m.setattr("__doc__", "The radix designs: a digit set inside the residues of a base in a ring, a unit twist per digit, and the points their words land on.")?;
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

    /// The nodes of a roulette: where the curves a wheel's pencils draw cross themselves and one another.
    pub mod roulette {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Every crossing of a traced roulette: the curves against themselves, the curves against one another, and how crowded the worst node is.
        #[pyclass(name = "Nodes", module = "mrly.math.roulette", from_py_object)]
        #[derive(Clone)]
        pub struct Nodes(pub mrly_math::roulette::Nodes);

        #[pymethods]
        impl Nodes {
            /// The curves counted, in the order the pencils came in.
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
            /// How often each curve crosses itself, curve by curve.
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
            /// How often each pair of curves crosses, the lower curve first, in lexicographic order.
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
            /// The most crossings one node carries: one at a plain double point, and `n(n - 1)/2` where `n` branches meet.
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
            /// The nodes more than one crossing clusters at.
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
            /// The distinct points the crossings sit at, one for every cluster.
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
            /// The branches through every node added up, which is the edge count of the picture as a plane graph, `n` at a node where `n` branches meet and `2` times `points` when no node is crowded.
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
            /// The segment pairs that meet without crossing: collinear or end to end.
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
            /// Returns the default Nodes.
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::default();
                (crate::doors::math::roulette::Nodes(out)).into_bound_py_any(py)
            }
            /// How often the curves `i` and `j` cross, either order, and zero when they are one curve.
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
            /// Every crossing of two curves.
            #[pyo3(name = "paired", signature = ())]
            pub fn paired<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::paired(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Every self crossing.
            #[pyo3(name = "selved", signature = ())]
            pub fn selved<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::selved(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Every crossing, self and pair together, which counts a node where `n` branches meet `n(n - 1)/2` times; `points` is the count of distinct nodes and the two agree exactly when `crowded` is zero.
            #[pyo3(name = "total", signature = ())]
            pub fn total<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::roulette::Nodes::total(&self.0);
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

        /// Counts the nodes of the roulette the pencils draw on the track: `mrly::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders.
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

        /// Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one.
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

        /// One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrly::math::spirograph::distinct` and `mrly::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve.
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
            m.setattr("__doc__", "The nodes of a roulette: where the curves a wheel's pencils draw cross themselves and one another.")?;
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

    /// The residue rules that mark a hypercube's cells.
    pub mod rules {
        use crate::hand::{PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list.
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

        /// Returns every axis but the free one.
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
            m.setattr(
                "__doc__",
                "The residue rules that mark a hypercube's cells.",
            )?;
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

    /// The exact crop machinery: rational shapes classified cell by cell, no floats.
    pub mod shape {
        use crate::hand::{PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// An exact rational number with a positive, reduced denominator.
        #[pyclass(name = "Frac", module = "mrly.math.shape", from_py_object)]
        #[derive(Clone)]
        pub struct Frac(pub mrly_math::shape::Frac);

        #[pymethods]
        impl Frac {
            /// Builds the reduced fraction num over den.
            #[new]
            #[pyo3(signature = (num, den))]
            pub fn __new__(num: i64, den: i64) -> PyResult<Self> {
                let out = mrly_math::shape::Frac::new(num, den);
                Ok(Self(ok(out)?))
            }
            /// The numerator, carrying the sign.
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
            /// The denominator, always positive.
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
            /// Returns the exact difference.
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
            /// Builds the reduced fraction num over den.
            #[staticmethod]
            #[pyo3(name = "new", signature = (num, den))]
            pub fn new_<'py>(py: Python<'py>, num: i64, den: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::shape::Frac::new(num, den);
                (crate::doors::math::shape::Frac(ok(out)?)).into_bound_py_any(py)
            }
            /// Returns the exact sum.
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
            /// Returns the exact product.
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
            /// Wraps an integer as a fraction over one.
            #[staticmethod]
            #[pyo3(name = "whole", signature = (num))]
            pub fn whole<'py>(py: Python<'py>, num: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::shape::Frac::whole(num);
                (crate::doors::math::shape::Frac(out)).into_bound_py_any(py)
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
        #[pyclass(name = "Region", module = "mrly.math.shape", skip_from_py_object)]
        pub struct Region;

        #[pymethods]
        impl Region {
            /// Swaps In and Out, keeping Cut.
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

        /// Tallies the design's cells and filled cells per region of the shape.
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

        /// Places one lattice cell relative to the shape, exactly, with no floats.
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

        /// Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti.
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

        /// Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order.
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

        /// Builds the whole crossing tree of one radius, pruned by the seats the design keeps.
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

        /// Centres a unit-box shape on the short side of a lattice of the extents and keeps it to that centred cube, so `regions`, `crop` and `census` read it on any lattice; a cube lattice keeps the shape as it is.
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

        /// Builds a named shape of the dimension, centered at one half on every axis.
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

        /// Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic.
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

        /// Returns the least radius, in den-ths up to one, at which the named shape covers the whole unit box, or none when it does not by one.
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

        /// Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes.
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

        /// Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side.
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

        /// Lists the named shapes of a dimension.
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
            m.setattr(
                "__doc__",
                "The exact crop machinery: rational shapes classified cell by cell, no floats.",
            )?;
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

    /// The hexagon world, the projection of `three`.
    pub mod six {
        use crate::hand::{
            PyCell2d, PyCell3d, PyCell6d, PyCellNd, PyCode, PyColor, PyImage, PyRgba, PySerde,
            PyTensor, ok,
        };
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The ghost star of the hexagonal cut stack: the arm ink law, the background and the cell-frame decay.
        pub mod star {
            use crate::hand::{PySerde, ok};
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// The exact reading of a cut layer: how many cells were inked out of how many were read.
            #[pyclass(name = "Share", module = "mrly.math.six.star", from_py_object)]
            #[derive(Clone)]
            pub struct Share(pub mrly_math::six::star::Share);

            #[pymethods]
            impl Share {
                /// The count of inked cells.
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
                /// The count of cells read.
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
                /// The share in lowest terms, numerator then denominator.
                #[pyo3(name = "reduced", signature = ())]
                pub fn reduced<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Share::reduced(self.0);
                    (out).into_bound_py_any(py)
                }
                /// The share as a real number.
                #[pyo3(name = "value", signature = ())]
                pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Share::value(self.0);
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
            #[pyclass(name = "Star", module = "mrly.math.six.star", skip_from_py_object)]
            pub struct Star(pub mrly_math::six::star::Star);

            #[pymethods]
            impl Star {
                /// Reads the star of a base-2 space code, the carpet being `23`.
                #[new]
                #[pyo3(signature = (code))]
                pub fn __new__(code: u128) -> PyResult<Self> {
                    let out = mrly_math::six::star::Star::new(code);
                    Ok(Self(ok(out)?))
                }
                /// The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`.
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
                /// The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`.
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
                /// The per-layer excess of the star band over the hexagon across the first `L` odd layers.
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
                /// The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against.
                #[pyo3(name = "hexagon", signature = (number))]
                pub fn hexagon<'py>(
                    &self,
                    py: Python<'py>,
                    number: usize,
                ) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Star::hexagon(&self.0, number);
                    (crate::doors::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
                }
                /// Reads the star of a base-2 space code, the carpet being `23`.
                #[staticmethod]
                #[pyo3(name = "new", signature = (code))]
                pub fn new_<'py>(py: Python<'py>, code: u128) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Star::new(code);
                    (crate::doors::math::six::star::Star(ok(out)?)).into_bound_py_any(py)
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
            #[pyclass(name = "Branch", module = "mrly.math.six.star", skip_from_py_object)]
            pub struct Branch;

            #[pymethods]
            impl Branch {
                /// The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`.
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
                /// The name of the branch.
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
                /// The branch of a layer count.
                #[staticmethod]
                #[pyo3(name = "of", signature = (layers))]
                pub fn of<'py>(py: Python<'py>, layers: usize) -> PyResult<Bound<'py, PyAny>> {
                    let out = mrly_math::six::star::Branch::of(layers);
                    (PySerde(out)).into_bound_py_any(py)
                }
                /// The exact `1/L^2` coefficient at even `L`, absent at odd `L`.
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

            /// The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`.
            #[pyfunction]
            #[pyo3(name = "arm_law", signature = (number))]
            pub fn arm_law<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::arm_law(number);
                (crate::doors::math::six::star::Share(ok(out)?)).into_bound_py_any(py)
            }

            /// The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`.
            #[pyfunction]
            #[pyo3(name = "chi8", signature = (number))]
            pub fn chi8<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::chi8(number);
                (out).into_bound_py_any(py)
            }

            /// The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`.
            #[pyfunction]
            #[pyo3(name = "constant", signature = ())]
            pub fn constant<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::six::star::constant();
                (out).into_bound_py_any(py)
            }

            /// The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`.
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

            /// The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`.
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
                m.setattr("__doc__", "The ghost star of the hexagonal cut stack: the arm ink law, the background and the cell-frame decay.")?;
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

        /// Swaps every fill triangle for a void and back.
        #[pyfunction]
        #[pyo3(name = "anti", signature = (cell))]
        pub fn anti<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::anti(cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// Maps each triangle to one at or above the threshold, zero below.
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

        /// Binarizes the triangles at the threshold Otsu's method picks.
        #[pyfunction]
        #[pyo3(name = "binarize_otsu", signature = (cell))]
        pub fn binarize_otsu<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::binarize_otsu(cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// Builds a hexagon of the given radius, fill inside and void outside.
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

        /// Rounds each triangle to the mean of its masked neighborhood, wrapping on request.
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

        /// Tallies a cell's triangles, corners and edges, counting the backdrop only on request.
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

        /// Counts the connected pieces of the fill, triangles joined across shared edges.
        #[pyfunction]
        #[pyo3(name = "components", signature = (cell))]
        pub fn components<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::components(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the hexagons in a disc of the given rings about one centre: 1, 7, 19, 37 and on.
        #[pyfunction]
        #[pyo3(name = "copies", signature = (rings))]
        pub fn copies<'py>(py: Python<'py>, rings: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::copies(rings);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Slices a cube through its center across the main diagonal into a hexagon.
        #[pyfunction]
        #[pyo3(name = "cut", signature = (cell))]
        pub fn cut<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::cut(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the coded 3d design and slices its central hexagon.
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

        /// Tessellates a hexagonal cell over the disc of the given rings about one centre, keeping its projection and orientation and taking the start parity of the uncropped sheet the disc sits in; no rings is the cell itself.
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

        /// The three corners of the east-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "east", signature = (x, y))]
        pub fn east<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::east(x, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request.
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

        /// Counts the filled triangles of the cell.
        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::fills(&cell);
            (out).into_bound_py_any(py)
        }

        /// Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh.
        #[pyfunction]
        #[pyo3(name = "fills_only", signature = (cell))]
        pub fn fills_only<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::fills_only(&cell);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood.
        #[pyfunction]
        #[pyo3(name = "framed", signature = (cell))]
        pub fn framed<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::framed(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Parses a cell from JSON, defaulting any missing projection metadata.
        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::from_json(text);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the triangle count of the fill's largest connected piece.
        #[pyfunction]
        #[pyo3(name = "giant", signature = (cell))]
        pub fn giant<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::giant(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the largest connected piece of the filled-triangle network as a network of its own.
        #[pyfunction]
        #[pyo3(name = "giant_network", signature = (cell))]
        pub fn giant_network<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::giant_network(&cell);
            (crate::doors::math::graph::Network(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the grid height in triangles.
        #[pyfunction]
        #[pyo3(name = "height", signature = (cell))]
        pub fn height<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::Cell6d::height(&cell);
            (out).into_bound_py_any(py)
        }

        /// Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh.
        #[pyfunction]
        #[pyo3(name = "holes", signature = (cell))]
        pub fn holes<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::holes(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Rasters a cell's triangles to an image at the given scale, stroked and padded when an outline is given.
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

        /// Returns whether the cell's three sides are equal.
        #[pyfunction]
        #[pyo3(name = "is_cube", signature = (cell))]
        pub fn is_cube<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::is_cube(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns whether the cell's width, height and parity frame a hexagon.
        #[pyfunction]
        #[pyo3(name = "is_hex", signature = (cell))]
        pub fn is_hex<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::is_hex(&cell);
            (out).into_bound_py_any(py)
        }

        /// Projects a cube into the isometric hexagon of top, left and right faces.
        #[pyfunction]
        #[pyo3(name = "iso", signature = (cell))]
        pub fn iso<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::iso(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the coded 3d design and projects it isometrically.
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

        /// Builds a cell from its four parts.
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

        /// The three corners of the north-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "north", signature = (x, y))]
        pub fn north<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::north(x, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the orientation a hexagon's width and height imply.
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

        /// Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along.
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

        /// Colors each triangle by its type through the custom or default mapping in the given or type mode.
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

        /// Writes the value wherever the tiled mask is nonzero.
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

        /// Projects a cube's three facing sides into a hexagon of fills and voids.
        #[pyfunction]
        #[pyo3(name = "pro", signature = (cell))]
        pub fn pro<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::pro(&cell);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the coded 3d design and projects its facing sides.
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

        /// Tessellates a hexagonal cell over the disc mask of the given radius.
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

        /// Crops the interlocking overhang off a disc tiled at the given radius and tile size.
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

        /// Builds the disc mask of cells within hex distance radius of the center.
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

        /// Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere.
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

        /// Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to an image.
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

        /// Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string.
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

        /// Counts the void regions the rim never reaches, the second route to the hole count.
        #[pyfunction]
        #[pyo3(name = "rim_holes", signature = (cell))]
        pub fn rim_holes<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::rim_holes(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure.
        #[pyfunction]
        #[pyo3(name = "skin", signature = (cell))]
        pub fn skin<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::skin(&cell);
            (PyCell6d(out)).into_bound_py_any(py)
        }

        /// Builds the network of filled triangles joined by shared edges.
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

        /// Builds the network of fill and void triangles joined by shared edges.
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

        /// Builds the corner-and-edge network of the triangles matching the value, or of every fill and void.
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

        /// Builds the network of void triangles joined by shared edges, the pore network of the slice.
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

        /// The three corners of the south-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "south", signature = (x, y))]
        pub fn south<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::south(x, y);
            (out).into_bound_py_any(py)
        }

        /// Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states.
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

        /// Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given.
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

        /// Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included.
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

        /// Tessellates a hexagonal cell over a full width-by-height mask.
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

        /// Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request.
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

        /// Crops one interlocking step off each side of a sheet tiled at the given size.
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

        /// Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped.
        #[pyfunction]
        #[pyo3(name = "tile_step", signature = (size))]
        pub fn tile_step<'py>(
            py: Python<'py>,
            size: (usize, usize),
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::tile_step(size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Serializes a cell and its projection metadata to JSON.
        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell6d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::six::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        /// Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override.
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

        /// The three corners of the west-pointing triangle at the grid column and row.
        #[pyfunction]
        #[pyo3(name = "west", signature = (x, y))]
        pub fn west<'py>(py: Python<'py>, x: i64, y: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::six::west(x, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the grid width in triangles.
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
            m.setattr("__doc__", "The hexagon world, the projection of `three`.\n\nA coded cube flattened to an iso, pro or cut hexagon, meshed into triangles, then counted,\ngraphed, rastered and drawn like any other cell.")?;
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

    /// The symmetric eigensolver and the Laplacian spectra it reads off a network.
    pub mod spectrum {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size.
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

        /// Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`.
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

        /// Returns the ascending Laplacian spectrum of a network, combinatorial or normalised.
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

        /// Counts the eigenvalues within the tolerance of a value.
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

        /// Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window.
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

        /// Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count.
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

        /// Builds the integrated density of states as points, each an eigenvalue and its rank fraction.
        #[pyfunction]
        #[pyo3(name = "spectral_points", signature = (eigenvalues))]
        pub fn spectral_points<'py>(
            py: Python<'py>,
            eigenvalues: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spectrum::spectral_points(&eigenvalues);
            (out).into_bound_py_any(py)
        }

        /// Returns the eigenvalues of a dense real symmetric matrix in ascending order.
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
            m.setattr(
                "__doc__",
                "The symmetric eigensolver and the Laplacian spectra it reads off a network.",
            )?;
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

    /// The turntable: the exact circle means of a raster about its centre, the profile they trace and the wheel it paints.
    pub mod spin {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The way radial copies merge: their mean, their sum, their union, their meet, their parity or what the first keeps that no other has.
        #[pyclass(name = "Blend", module = "mrly.math.spin", skip_from_py_object)]
        pub struct Blend;

        #[pymethods]
        impl Blend {
            /// Merges one site's copies into the blended value.
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
            /// Reads a blend by name: mean, sum, union, meet, parity or difference.
            #[staticmethod]
            #[pyo3(name = "named", signature = (name))]
            pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::spin::Blend::named(name);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
        }

        /// The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside.
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

        /// The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs.
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

        /// The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from.
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

        /// The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from.
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

        /// The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.
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

        /// The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle.
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

        /// Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points.
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

        /// The radius of the corner circle of a square raster of the side, the last radius a profile reads.
        #[pyfunction]
        #[pyo3(name = "reach", signature = (size))]
        pub fn reach<'py>(py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::reach(size);
            (out).into_bound_py_any(py)
        }

        /// The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero.
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

        /// The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does.
        #[pyfunction]
        #[pyo3(name = "turns", signature = (power))]
        pub fn turns<'py>(py: Python<'py>, power: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::spin::turns(&power);
            (out).into_bound_py_any(py)
        }

        /// The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius.
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

    /// The spirograph: a byte grid as a wheel with a pencil in every cell, rolled on a line, a circle or a polygon, and the curves it draws.
    pub mod spirograph {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The mass of a byte grid taken as a wheel: how many pencils of each kind it seats.
        #[pyclass(name = "Seats", module = "mrly.math.spirograph", skip_from_py_object)]
        pub struct Seats;

        #[pymethods]
        impl Seats {
            /// Returns the default Seats.
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::spirograph::Seats::default();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel.
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

        /// The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area.
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

        /// The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall.
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

        /// How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter.
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

        /// The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further.
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

        /// The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach.
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

        /// Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded.
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

        /// Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel.
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

        /// The wheel's centre after `s` of path length.
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

        /// One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter.
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

        /// Counts the pencils by kind.
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

        /// The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close.
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

        /// Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y.
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

        /// Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps.
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

        /// The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius.
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
            m.setattr("__doc__", "The spirograph: a byte grid as a wheel with a pencil in every cell, rolled on a line, a circle or a polygon, and the curves it draws.")?;
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

    /// The sumset of Erdos problem 125: the base-3 design `{0, 1}` plus the base-4 design `{0, 1}` as a bit array, its density below `x`, the gaps its level pairs leave and their additive energy.
    pub mod sumset {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A pair of levels: the base-3 level `A_k = A meet [0, 3^k)` against the base-4 level `B_m = B meet [0, 4^m)`, `k` read by `three()` and `m` by `four()`.
        #[pyclass(name = "Pair", module = "mrly.math.sumset", from_py_object)]
        #[derive(Clone)]
        pub struct Pair(pub mrly_math::sumset::Pair);

        #[pymethods]
        impl Pair {
            /// Names the pair `(k, m)`.
            #[new]
            #[pyo3(signature = (three, four))]
            pub fn __new__(three: u32, four: u32) -> PyResult<Self> {
                let out = mrly_math::sumset::Pair::new(three, four);
                Ok(Self(ok(out)?))
            }
            /// Whether the pair is clean, `3^k > d(k, m)` and `4^m > d(k, m)`, so that `S meet [0, d] = A_k + B_m`.
            #[pyo3(name = "clean", signature = ())]
            pub fn clean<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::clean(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Whether the pair is a gap copy, `2 4^m < 3^k + 5`: `A_k + B_m` is then two disjoint translates of `A_(k-1) + B_m` and its energy is twice theirs.
            #[pyo3(name = "copy", signature = ())]
            pub fn copy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::copy(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The additive energy `E(k, m) = sum_x r(x)^2`, `r(x)` the number of ways `x = a + b` with `a` in `A_k` and `b` in `B_m`.
            #[pyo3(name = "energy", signature = ())]
            pub fn energy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::energy(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The base-4 level `m`.
            #[pyo3(name = "four", signature = ())]
            pub fn four<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::four(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The first and the last integer of the open interval `(d(k, m), min(3^k, 4^m))`, which `S` misses, or `None` when it holds none.
            #[pyo3(name = "gap", signature = ())]
            pub fn gap<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::gap(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The largest element `d(k, m) = (3^k - 1)/2 + (4^m - 1)/3` of `A_k + B_m`.
            #[pyo3(name = "largest", signature = ())]
            pub fn largest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::largest(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Names the pair `(k, m)`.
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
            /// The energy ratio `Q(k, m) = E(k, m) (d + 1)/4^(k+m)` of the energy `Pair::energy` returns, the energy against its flat value, at least `1`; `card(A_k + B_m) >= (d + 1)/Q` by Cauchy-Schwarz.
            #[pyo3(name = "ratio", signature = (energy))]
            pub fn ratio<'py>(&self, py: Python<'py>, energy: u128) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::ratio(&self.0, energy);
                (out).into_bound_py_any(py)
            }
            /// The scaling `tau = 4^m/3^k`.
            #[pyo3(name = "scale", signature = ())]
            pub fn scale<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::scale(&self.0);
                (out).into_bound_py_any(py)
            }
            /// The base-3 level `k`.
            #[pyo3(name = "three", signature = ())]
            pub fn three<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Pair::three(&self.0);
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

        /// The sumset `S = A + B` of Erdos problem 125 up to `3^level`: `A` the integers whose base-3 digits are all `0` or `1`, `B` those whose base-4 digits are.
        #[pyclass(name = "Sumset", module = "mrly.math.sumset", skip_from_py_object)]
        pub struct Sumset(pub mrly_math::sumset::Sumset);

        #[pymethods]
        impl Sumset {
            /// Builds `S meet [0, 3^level]`.
            #[new]
            #[pyo3(signature = (level))]
            pub fn __new__(level: u32) -> PyResult<Self> {
                let out = mrly_math::sumset::Sumset::new(level);
                Ok(Self(ok(out)?))
            }
            /// Whether `x` is in `S`, or `None` past the top.
            #[pyo3(name = "contains", signature = (x))]
            pub fn contains<'py>(&self, py: Python<'py>, x: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::contains(&self.0, x);
                (out).into_bound_py_any(py)
            }
            /// Counts `card(S meet [1, x])`, or `None` past the top.
            #[pyo3(name = "count", signature = (x))]
            pub fn count<'py>(&self, py: Python<'py>, x: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::count(&self.0, x);
                (out).into_bound_py_any(py)
            }
            /// Reads the density `D(x) = card(S meet [1, x])/x`, or `None` at zero and past the top.
            #[pyo3(name = "density", signature = (x))]
            pub fn density<'py>(&self, py: Python<'py>, x: u64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::density(&self.0, x);
                (out).into_bound_py_any(py)
            }
            /// Reads the least and the greatest `D(x)` over each window `[edges[i], edges[i + 1])`, `None` for an empty window.
            #[pyo3(name = "extremes", signature = (edges))]
            pub fn extremes<'py>(
                &self,
                py: Python<'py>,
                edges: Vec<u64>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::extremes(&self.0, &edges);
                (ok(out)?).into_bound_py_any(py)
            }
            /// Reads the share of members in each of `cells` equal runs of the integers `[low, high)`, the strip of `S` a page draws.
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
            /// The level the array was built to.
            #[pyo3(name = "level", signature = ())]
            pub fn level<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::level(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Builds `S meet [0, 3^level]`.
            #[staticmethod]
            #[pyo3(name = "new", signature = (level))]
            pub fn new_<'py>(py: Python<'py>, level: u32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::new(level);
                (crate::doors::math::sumset::Sumset(ok(out)?)).into_bound_py_any(py)
            }
            /// The largest integer the array holds, `3^level`.
            #[pyo3(name = "top", signature = ())]
            pub fn top<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::sumset::Sumset::top(&self.0);
                (out).into_bound_py_any(py)
            }
        }

        /// Lists the pairs of the census: every `(k, m)` with `4^m` within a factor `3` of `3^k` and `d(k, m) <= 3^level`, by `d`.
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
            m.setattr("__doc__", "The sumset of Erdos problem 125: the base-3 design `{0, 1}` plus the base-4 design `{0, 1}` as a bit array, its density below `x`, the gaps its level pairs leave and their additive energy.")?;
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

    /// The cube pipeline.
    pub mod three {
        use crate::hand::{PyCell2d, PyCell3d, PyCellNd, PyCode, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::exceptions::PyValueError;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The Menger sponge's distance, its tube inside the plus and its Minkowski profile.
        pub mod sponge {
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// The volume `Deep(radius)` of the points of one arm's quarter beyond the tubes of both walls it touches, at every radius, to double precision.
            #[pyfunction]
            #[pyo3(name = "deep", signature = (radius))]
            pub fn deep<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::deep(radius);
                (out).into_bound_py_any(py)
            }

            /// The Minkowski dimension of the Menger sponge, `log(20)/log(3)`, the similarity dimension of its 20 maps of ratio `1/3`.
            #[pyfunction]
            #[pyo3(name = "dimension", signature = ())]
            pub fn dimension<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::dimension();
                (out).into_bound_py_any(py)
            }

            /// The Euclidean distance from a point of the unit cube to the Menger sponge, exact to the last binary place.
            #[pyfunction]
            #[pyo3(name = "distance", signature = (point))]
            pub fn distance<'py>(py: Python<'py>, point: [f64; 3]) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::distance(point);
                (out).into_bound_py_any(py)
            }

            /// The volume `T(radius)` of the points of the plus within `radius` of the sponge at every radius, `Deep` included, to double precision.
            #[pyfunction]
            #[pyo3(name = "exact", signature = (radius))]
            pub fn exact<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::exact(radius);
                (out).into_bound_py_any(py)
            }

            /// The periodic function `p` of Kombrink, Pearse and Winter at `radius`: the reading's limit profile, unchanged when the radius is multiplied by 3, or `None` on the phases `(1/6, sqrt(2)/6]` where `T` has no closed form.
            #[pyfunction]
            #[pyo3(name = "profile", signature = (radius))]
            pub fn profile<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::profile(radius);
                (out).into_bound_py_any(py)
            }

            /// The Minkowski reading `radius^(D-3)` times the volume inside the cube, the number whose limit as the radius shrinks would be the sponge's Minkowski content.
            #[pyfunction]
            #[pyo3(name = "reading", signature = (radius))]
            pub fn reading<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::reading(radius);
                (out).into_bound_py_any(py)
            }

            /// The volume `T(radius)` of the points of the plus of seven removed level-1 cubes within `radius` of the sponge, or `None` on `(1/6, sqrt(2)/6)`, where no closed form is known.
            #[pyfunction]
            #[pyo3(name = "tube", signature = (radius))]
            pub fn tube<'py>(py: Python<'py>, radius: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::sponge::tube(radius);
                (out).into_bound_py_any(py)
            }

            /// The volume of the points of the unit cube within `radius` of the sponge, `sum_k (20/27)^k T(3^k radius)`, or `None` when some `3^k radius` falls where `T` has no closed form.
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
                m.setattr("__doc__", "The Menger sponge's distance, its tube inside the plus and its Minkowski profile.")?;
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

        /// A three-component vector of f32.
        #[pyclass(name = "Vec3", module = "mrly.math.three", from_py_object)]
        #[derive(Clone)]
        pub struct Vec3(pub mrly_math::three::Vec3);

        #[pymethods]
        impl Vec3 {
            /// Builds a vector from its components.
            #[new]
            #[pyo3(signature = (x, y, z))]
            pub fn __new__(x: f32, y: f32, z: f32) -> PyResult<Self> {
                let out = mrly_math::three::Vec3::new(x, y, z);
                Ok(Self(out))
            }
            /// The x component.
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
            /// The y component.
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
            /// The z component.
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
            /// Returns the cross product, perpendicular to both vectors.
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
            /// Returns the dot product of the two vectors.
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
            /// Builds a vector from its components.
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
            /// Multiplies every component by the scalar.
            #[pyo3(name = "scale", signature = (s))]
            pub fn scale<'py>(&self, py: Python<'py>, s: f32) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_math::three::Vec3::scale(self.0, s);
                (crate::doors::math::three::Vec3(out)).into_bound_py_any(py)
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
        pub fn carpet<'py>(
            py: Python<'py>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::carpet(number, level);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading.
        #[pyfunction]
        #[pyo3(name = "census", signature = (cell))]
        pub fn census<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::census(&cell);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Extracts the network of filled sites joined to their axis neighbors.
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

        /// Builds the cube the universe code names, deepened to the given fractal level.
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

        /// Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples.
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

        /// Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner.
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

        /// Builds the dust cube, filled where every coordinate is even, at the given level.
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

        /// Extracts the network of corners and edges outlining every filled site.
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

        /// Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites.
        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell))]
        pub fn euler<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::euler(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it.
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

        /// Repeats every plane of a cube depth times along its leading axis, colors and tags with it.
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

        /// Returns the count of unit faces the filled sites touch, a face shared by two sites counted once.
        #[pyfunction]
        #[pyo3(name = "faces", signature = (cell))]
        pub fn faces<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::faces(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of filled sites.
        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::fills(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds a cube from its corner patterns, deepened to the given fractal level.
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

        /// Parses a cell from its JSON, colors and tags included.
        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::from_json(text);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a cube from one string of digits per row, grouped plane by plane.
        #[pyfunction]
        #[pyo3(name = "from_strings", signature = (data))]
        pub fn from_strings<'py>(
            py: Python<'py>,
            data: Vec<Vec<String>>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::from_strings(&data);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the count of faces buried between two filled sites, six per site less the exposed surface.
        #[pyfunction]
        #[pyo3(name = "hidden", signature = (cell))]
        pub fn hidden<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::hidden(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the cube filled wherever the residue sum lands in the levels, at the given level.
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

        /// Folds two or more cells into one by chained Kronecker combination.
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

        /// Tags every site with its Manhattan distance from the cube's center, the diamond shells.
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

        /// Merges the cells into one cube arranged width by height by depth.
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

        /// Builds a cell by placing at each mask site the cell its value indexes.
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

        /// Builds the cube the name picks, deepened to the given fractal level.
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

        /// Builds the net cube, filled where at least two coordinates are odd, at the given level.
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

        /// Builds a cube whose every site turns on with probability density, at the given level.
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

        /// Builds the solid cube at the given size and level.
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

        /// Returns the 24 rotation triples that reach each distinct cube orientation.
        #[pyfunction]
        #[pyo3(name = "orientations", signature = ())]
        pub fn orientations<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::orientations();
            (out).into_bound_py_any(py)
        }

        /// Builds the point cube, filled where every coordinate is odd, at the given level.
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

        /// Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`.
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

        /// Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`.
        #[pyfunction]
        #[pyo3(name = "project", signature = (point))]
        pub fn project<'py>(py: Python<'py>, point: [u32; 3]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::project(point);
            (out).into_bound_py_any(py)
        }

        /// Returns one outward quad per exposed face, scaled into the unit box.
        #[pyfunction]
        #[pyo3(name = "quads", signature = (cell))]
        pub fn quads<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::quads(&cell);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped.
        #[pyfunction]
        #[pyo3(name = "shadow", signature = (point))]
        pub fn shadow<'py>(py: Python<'py>, point: [u32; 3]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::shadow(point);
            (out).into_bound_py_any(py)
        }

        /// Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it.
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

        /// Orients a copy of the cell by each mask value and merges them in the mask's shape.
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

        /// Builds the star cube, filled where exactly one coordinate is odd, at the given level.
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

        /// Returns the first and last height a profile fills, or none when the design is empty.
        #[pyfunction]
        #[pyo3(name = "support", signature = (counts))]
        pub fn support<'py>(py: Python<'py>, counts: Vec<u128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::three::support(&counts);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of filled faces exposed to void or the outside.
        #[pyfunction]
        #[pyo3(name = "surface", signature = (cell))]
        pub fn surface<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::surface(&cell);
            (out).into_bound_py_any(py)
        }

        /// Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped.
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

        /// Serializes the cell's shape and types to JSON, with colors and tags when present.
        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        /// Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner.
        #[pyfunction]
        #[pyo3(name = "to_obj", signature = (cell))]
        pub fn to_obj<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::to_obj(&cell);
            (out).into_bound_py_any(py)
        }

        /// Unrolls the cube into one string of digits per row, grouped plane by plane.
        #[pyfunction]
        #[pyo3(name = "to_strings", signature = (cell))]
        pub fn to_strings<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::to_strings(&cell);
            (out).into_bound_py_any(py)
        }

        /// Extracts the network of empty sites joined to their axis neighbors.
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

        /// Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.
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

        /// Returns the count of empty sites.
        #[pyfunction]
        #[pyo3(name = "voids", signature = (cell))]
        pub fn voids<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::voids(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns the filled-site count, the cube's volume.
        #[pyfunction]
        #[pyo3(name = "volume", signature = (cell))]
        pub fn volume<'py>(py: Python<'py>, cell: PyCell3d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::three::volume(&cell);
            (out).into_bound_py_any(py)
        }

        /// Returns the cell's edge-graph segments, scaled into the unit box.
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

        /// Builds the cube of rods along the x axis at the given size and level.
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

        /// Builds the cube of beams along the x axis at the given size and level.
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

        /// Builds the cube of rods along the y axis at the given size and level.
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

        /// Builds the cube of beams along the y axis at the given size and level.
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

        /// Builds the all-void cube at the given size and level.
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

        /// Builds the cube of rods along the z axis at the given size and level.
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

        /// Builds the cube of beams along the z axis at the given size and level.
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
            m.setattr("__doc__", "The cube pipeline.\n\nThe shared cell pipeline pinned to three dimensions: coded and carpet cubes, their censuses,\ntheir diagonal slices, their exposed quads and their JSON.")?;
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

    /// The tourbillon: the odd parity carpets turned one angle a layer and stacked inside the inscribed disc.
    pub mod tourbillon {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle.
        #[pyfunction]
        #[pyo3(name = "eyes", signature = (qmax))]
        pub fn eyes<'py>(py: Python<'py>, qmax: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::tourbillon::eyes(qmax);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.
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

        /// The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle.
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

        /// The least whole number of increments that closes a quarter turn, none once the count passes the cap.
        #[pyfunction]
        #[pyo3(name = "period", signature = (increment))]
        pub fn period<'py>(py: Python<'py>, increment: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::tourbillon::period(increment);
            (out).into_bound_py_any(py)
        }

        /// The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one.
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

        /// Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site.
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

        /// Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites.
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
            m.setattr("__doc__", "The tourbillon: the odd parity carpets turned one angle a layer and stacked inside the inscribed disc.")?;
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

    /// The flat-cell pipeline.
    pub mod two {
        use crate::hand::{PyCell2d, PyCellNd, PyCode, PyColor, PyImage, PySerde, PyTensor, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The payload a cell's filled sites carry, framed in a sheet.
        pub mod payload {
            use crate::hand::PyTensor;
            use pyo3::IntoPyObjectExt;
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// The five by five mask a carried mosaic lays its four tiles out under.
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
                m.setattr(
                    "__doc__",
                    "The payload a cell's filled sites carry, framed in a sheet.",
                )?;
                m.add_function(wrap_pyfunction!(frame, &m)?)?;
                let names: Vec<&str> = vec!["frame"];
                m.add("__all__", names)?;
                parent.add("payload", &m)?;
                sys.set_item("mrly.math._math.math.two.payload", &m)?;
                Ok(())
            }
        }

        /// Returns the payload bytes the cell's filled sites can hold, its length header paid for.
        #[pyfunction]
        #[pyo3(name = "capacity", signature = (cell))]
        pub fn capacity<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::capacity(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level.
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

        /// Takes the cell's full census in one reading.
        #[pyfunction]
        #[pyo3(name = "census", signature = (cell))]
        pub fn census<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::census(&cell);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the design a universe code names, deepened to the level and rotated by quarter-turns.
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

        /// Builds the dust fractal, its seed on at every even-even site, deepened to the level.
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

        /// Writes the payload over the cell's filled sites, repeating it until every site is spoken for.
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

        /// Returns the Euler characteristic of the filled sites, vertices less edges plus faces.
        #[pyfunction]
        #[pyo3(name = "euler", signature = (cell))]
        pub fn euler<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::euler(&cell);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Reads the payload back, the plain cell naming the sites the carried one wrote over.
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

        /// Counts the filled sites of the cell.
        #[pyfunction]
        #[pyo3(name = "fills", signature = (cell))]
        pub fn fills<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::fills(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns.
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

        /// Restores a cell from its JSON string, colors and tags included.
        #[pyfunction]
        #[pyo3(name = "from_json", signature = (text))]
        pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::from_json(text);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds a cell from rows of digits, the inverse of the text rendering.
        #[pyfunction]
        #[pyo3(name = "from_strings", signature = (rows))]
        pub fn from_strings<'py>(
            py: Python<'py>,
            rows: Vec<String>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_math::two::from_strings(&rows);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        /// Builds the hline fractal, its seed striped along odd rows, deepened to the level.
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

        /// Builds the htree fractal, its seed striped along even rows, deepened to the level.
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

        /// Renders the cell to an image at the given pixel scale, stroked and padded when an outline is given.
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

        /// Builds the level-set design, filling every residue corner whose digits sum to a named level.
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

        /// Tiles the mask over the shape and crops it, the perforation pattern itself.
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

        /// Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept.
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

        /// Builds the design the name picks, deepened to the level and rotated by quarter-turns.
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

        /// Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level.
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

        /// Builds a random cell, each seed site drawn on with probability density, deepened to the level.
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

        /// Builds an all-filled cell of the given size and level.
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

        /// Counts the faces of filled sites open to emptiness or the border.
        #[pyfunction]
        #[pyo3(name = "perimeter", signature = (cell))]
        pub fn perimeter<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::perimeter(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the point fractal, its seed on at every odd-odd site, deepened to the level.
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

        /// Reads the payload back from a framed sheet, the plain fourth cell naming the sites.
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

        /// Builds the framed sheet of four same-sized cells, the fourth carrying the payload.
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

        /// Tiles quarter-turned copies of the cell as the 2d mask directs.
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

        /// Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level.
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

        /// Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given.
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

        /// Renders the cell as rows of glyphs, or of digits where no glyph is mapped.
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

        /// Lifts the flat cell into a cube one site deep, colors and tags with it.
        #[pyfunction]
        #[pyo3(name = "to_3d", signature = (cell))]
        pub fn to_3d<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::to_3d(&cell);
            (PyCellNd(out)).into_bound_py_any(py)
        }

        /// Serializes the cell to a JSON string of its types, with colors and tags when present.
        #[pyfunction]
        #[pyo3(name = "to_json", signature = (cell))]
        pub fn to_json<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::to_json(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the vline fractal, its seed striped along odd columns, deepened to the level.
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

        /// Builds the void fractal, its seed a checkerboard on even parity, deepened to the level.
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

        /// Counts the empty sites of the cell.
        #[pyfunction]
        #[pyo3(name = "voids", signature = (cell))]
        pub fn voids<'py>(py: Python<'py>, cell: PyCell2d) -> PyResult<Bound<'py, PyAny>> {
            let cell = cell.0;
            let out = mrly_math::two::voids(&cell);
            (out).into_bound_py_any(py)
        }

        /// Builds the vtree fractal, its seed striped along even columns, deepened to the level.
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

        /// Builds an all-empty cell of the given size and level.
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
            m.setattr("__doc__", "The flat-cell pipeline.\n\nThe shared cell pipeline pinned to two dimensions: coded and carpet cells, their censuses,\ntheir payloads, their text, SVG and pixel renderings and their JSON.")?;
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
        m.setattr("__doc__", "The designs: codes, cells, cubes and hexagons, their counts, graphs and names, digit designs on the line, and wheels.\n\nA small integer code picks the filled corners of a hypercube, that seed grows level by level into a fractal design, and the same code always unfolds into the same shape, so a design can be named, counted and drawn again from its number alone. Everything rests on the tensors and cells of `mrly_core` and the sequences of `mrly_num`.\n\n# The design pipeline\n\n- `atoms` fills a tensor with a carpet, a net, beams or noise.\n- `bang` enumerates the design codes, their symmetries and their counts.\n- `name` holds the bang, sequence and word kinds of the canonical name.\n- `rules` marks the cells of a hypercube whose coordinate residues satisfy a rule.\n- `cell` holds the N-dimensional cell and the pipeline the fixed dimensions share.\n- `two`, `three` and `six` run that pipeline for flat cells, cubes and hexagons.\n- `counts` gives the same fills, surfaces, hex slices and carry ladder in closed form.\n- `graph` lifts a grid into nodes and branches.\n- `spectrum` reads the Laplacian spectra off a network.\n- `nodal` rings the plate a design cuts: its lowest modes, nodal domains and Courant bounds.\n- `shape` crops a cell against a rational shape, cell by cell, with no floats.\n- `moire` layers one design at many scales into an interference field.\n- `press` weighs the integers a design's digit rule keeps.\n- `arcs` lays two quarter arcs in every cell and follows the loops they join into.\n- `coprime` counts the gasket's coprime points.\n\n# Line and Dirichlet designs\n\n- `design`: the digit designs on the line, their Mobius meter and the ordinates it carries.\n- `dissection`: the missing-digit sets cut by Dirichlet approximation, their four regions and the chain certificate.\n- `ladder`: the Dirichlet series of a digit design, continued to the plane, each value with its bound.\n- `memory`: a rule on consecutive digits, its transfer matrix, its words, its Perron root.\n- `automaton`: the Dirichlet series of a memory design, continued through its transfer matrix.\n- `game`: the chaos game of a design: its de Bruijn schedules, the restricted games that draw the memory dial's sets, and the mosaic and slice games on a graph.\n- `sumset`: the base-3 design plus the base-4 design of Erdos problem 125, its density and energy.\n- `radix`: a digit set inside the residues of a base in a ring, and where its words land.\n- `arrays`: a sparse sensor array, its difference coarray, its essential sensors and the fractal array a generator grows.\n\n# Wheels\n\n- `spin` spins a raster about its centre.\n- `tourbillon` stacks the turned parity carpets inside the inscribed disc.\n- `spirograph` rolls a byte grid as a wheel and draws the curves of its pencils.\n- `roulette` counts where those curves cross.\n\nThe doors are `crate::atoms::carpet_2d`, `crate::bang::bang`,\n`crate::two::carpet`, `crate::two::census()`, `crate::two::to_json`,\n`crate::counts::fill`, `crate::three::census::surface` and\n`crate::spectrum::laplacian_spectrum`.")?;
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
