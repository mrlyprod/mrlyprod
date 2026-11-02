#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// The integers: primes, divisors, series, sieves, lattices and spectra.
pub mod num {
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// The Apollonian gasket: a packing grown from its root quadruple in exact integers, the Ford circles it rests on the line, and the Farey stack they shadow.
    pub mod apollonian {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing.
        #[pyclass(name = "Circle", module = "mrly.num.apollonian", from_py_object)]
        #[derive(Clone)]
        pub struct Circle(pub mrly_num::apollonian::Circle);

        #[pymethods]
        impl Circle {
            /// The curvature.
            #[getter]
            #[pyo3(name = "k")]
            pub fn k<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.k;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "k")]
            pub fn set_k(&mut self, value: i64) -> PyResult<()> {
                self.0.k = value;
                Ok(())
            }
            /// The curvature times the centre's abscissa.
            #[getter]
            #[pyo3(name = "x")]
            pub fn x<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.x;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "x")]
            pub fn set_x(&mut self, value: i64) -> PyResult<()> {
                self.0.x = value;
                Ok(())
            }
            /// The curvature times the centre's ordinate.
            #[getter]
            #[pyo3(name = "y")]
            pub fn y<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.y;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "y")]
            pub fn set_y(&mut self, value: i64) -> PyResult<()> {
                self.0.y = value;
                Ok(())
            }
            /// The centre, none on a line.
            #[pyo3(name = "centre", signature = ())]
            pub fn centre<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::apollonian::Circle::centre(self.0);
                (out).into_bound_py_any(py)
            }
            /// Whether the circle is a line.
            #[pyo3(name = "is_line", signature = ())]
            pub fn is_line<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::apollonian::Circle::is_line(self.0);
                (out).into_bound_py_any(py)
            }
            /// The radius, none on a line.
            #[pyo3(name = "radius", signature = ())]
            pub fn radius<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::apollonian::Circle::radius(self.0);
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

        /// Grows the named packing and lays every circle of positive curvature as `x, y, r, k` in one flat run: the root's own circles first in root order, then the grown ones in curvature order.
        #[pyfunction]
        #[pyo3(name = "flat", signature = (name, cap))]
        pub fn flat<'py>(py: Python<'py>, name: &str, cap: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::apollonian::flat(name, cap);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.
        #[pyfunction]
        #[pyo3(name = "form", signature = (u, v))]
        pub fn form<'py>(py: Python<'py>, u: [i64; 4], v: [i64; 4]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::apollonian::form(u, v);
            (out).into_bound_py_any(py)
        }

        /// The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.
        #[pyfunction]
        #[pyo3(name = "frame", signature = (p))]
        pub fn frame<'py>(
            py: Python<'py>,
            p: PySerde<mrly_num::apollonian::Packing>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let p = p.0;
            let out = mrly_num::apollonian::frame(&p);
            (out).into_bound_py_any(py)
        }

        /// Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.
        #[pyfunction]
        #[pyo3(name = "grow", signature = (name, cap))]
        pub fn grow<'py>(py: Python<'py>, name: &str, cap: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::apollonian::grow(name, cap);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.
        #[pyfunction]
        #[pyo3(name = "is_ford", signature = (c))]
        pub fn is_ford<'py>(
            py: Python<'py>,
            c: crate::doors::num::apollonian::Circle,
        ) -> PyResult<Bound<'py, PyAny>> {
            let c = c.0;
            let out = mrly_num::apollonian::is_ford(c);
            (out).into_bound_py_any(py)
        }

        /// Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.
        #[pyfunction]
        #[pyo3(name = "on_line", signature = (c))]
        pub fn on_line<'py>(
            py: Python<'py>,
            c: crate::doors::num::apollonian::Circle,
        ) -> PyResult<Bound<'py, PyAny>> {
            let c = c.0;
            let out = mrly_num::apollonian::on_line(c);
            (out).into_bound_py_any(py)
        }

        /// Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.
        #[pyfunction]
        #[pyo3(name = "reflect", signature = (q, at))]
        pub fn reflect<'py>(
            py: Python<'py>,
            q: [crate::doors::num::apollonian::Circle; 4],
            at: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let q = q.map(|x| x.0);
            let out = mrly_num::apollonian::reflect(&q, at);
            (crate::doors::num::apollonian::Circle(out)).into_bound_py_any(py)
        }

        /// The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.
        #[pyfunction]
        #[pyo3(name = "root", signature = (name))]
        pub fn root<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::apollonian::root(name);
            ((ok(out)?)
                .into_iter()
                .map(crate::doors::num::apollonian::Circle)
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }

        /// Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.
        #[pyfunction]
        #[pyo3(name = "shadow", signature = (p, order))]
        pub fn shadow<'py>(
            py: Python<'py>,
            p: PySerde<mrly_num::apollonian::Packing>,
            order: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let p = p.0;
            let out = mrly_num::apollonian::shadow(&p, order);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.
        #[pyfunction]
        #[pyo3(name = "sound", signature = (q))]
        pub fn sound<'py>(
            py: Python<'py>,
            q: [crate::doors::num::apollonian::Circle; 4],
        ) -> PyResult<Bound<'py, PyAny>> {
            let q = q.map(|x| x.0);
            let out = mrly_num::apollonian::sound(&q);
            (out).into_bound_py_any(py)
        }

        /// The quadruple with the circle at the seat replaced by its reflection.
        #[pyfunction]
        #[pyo3(name = "swap", signature = (q, at))]
        pub fn swap<'py>(
            py: Python<'py>,
            q: [crate::doors::num::apollonian::Circle; 4],
            at: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let q = q.map(|x| x.0);
            let out = mrly_num::apollonian::swap(&q, at);
            ((out)
                .into_iter()
                .map(crate::doors::num::apollonian::Circle)
                .collect::<Vec<_>>())
            .into_bound_py_any(py)
        }

        /// The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.
        #[pyfunction]
        #[pyo3(name = "touches", signature = (p))]
        pub fn touches<'py>(
            py: Python<'py>,
            p: PySerde<mrly_num::apollonian::Packing>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let p = p.0;
            let out = mrly_num::apollonian::touches(&p);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.apollonian")?;
            m.setattr("__doc__", "The Apollonian gasket: a packing grown from its root quadruple in exact integers, the Ford circles it rests on the line, and the Farey stack they shadow.")?;
            m.add_class::<Circle>()?;
            m.add_function(wrap_pyfunction!(flat, &m)?)?;
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
            m.add("CIRCLE_CAP", mrly_num::apollonian::CIRCLE_CAP)?;
            m.add("CURVATURE_CAP", mrly_num::apollonian::CURVATURE_CAP)?;
            m.add("ORDER_CAP", mrly_num::apollonian::ORDER_CAP)?;
            m.add(
                "ROOTS",
                (mrly_num::apollonian::ROOTS)
                    .into_iter()
                    .map(|x| (x).to_string())
                    .collect::<Vec<_>>(),
            )?;
            m.add("STRIDE", mrly_num::apollonian::STRIDE)?;
            let names: Vec<&str> = vec![
                "flat",
                "form",
                "frame",
                "grow",
                "is_ford",
                "on_line",
                "reflect",
                "root",
                "shadow",
                "sound",
                "swap",
                "touches",
                "Circle",
                "CIRCLE_CAP",
                "CURVATURE_CAP",
                "ORDER_CAP",
                "ROOTS",
                "STRIDE",
            ];
            m.add("__all__", names)?;
            parent.add("apollonian", &m)?;
            sys.set_item("mrly.num._num.num.apollonian", &m)?;
            Ok(())
        }
    }

    /// The sequence blender: term ops, exact recurrences and growth rates.
    pub mod blend {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Adds two sequences term by term over their shared length.
        #[pyfunction]
        #[pyo3(name = "add", signature = (a, b))]
        pub fn add<'py>(
            py: Python<'py>,
            a: Vec<i128>,
            b: Vec<i128>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::add(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Convolves two sequences, keeping the exact prefix their shared length affords.
        #[pyfunction]
        #[pyo3(name = "cauchy", signature = (a, b))]
        pub fn cauchy<'py>(
            py: Python<'py>,
            a: Vec<i128>,
            b: Vec<i128>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::cauchy(&a, &b);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the monic characteristic polynomial of a recurrence, highest power first.
        #[pyfunction]
        #[pyo3(name = "characteristic", signature = (coefficients))]
        pub fn characteristic<'py>(
            py: Python<'py>,
            coefficients: Vec<(i128, i128)>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::characteristic(&coefficients);
            (out).into_bound_py_any(py)
        }

        /// Keeps every step-th term from the offset onward.
        #[pyfunction]
        #[pyo3(name = "decimate", signature = (a, step, offset))]
        pub fn decimate<'py>(
            py: Python<'py>,
            a: Vec<i128>,
            step: usize,
            offset: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::decimate(&a, step, offset);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the first differences of a sequence, one term shorter.
        #[pyfunction]
        #[pyo3(name = "delta", signature = (a))]
        pub fn delta<'py>(py: Python<'py>, a: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::delta(&a);
            (out).into_bound_py_any(py)
        }

        /// Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands.
        #[pyfunction]
        #[pyo3(name = "growth", signature = (coefficients))]
        pub fn growth<'py>(
            py: Python<'py>,
            coefficients: Vec<(i128, i128)>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::growth(&coefficients);
            (out).into_bound_py_any(py)
        }

        /// Multiplies two sequences term by term over their shared length.
        #[pyfunction]
        #[pyo3(name = "hadamard", signature = (a, b))]
        pub fn hadamard<'py>(
            py: Python<'py>,
            a: Vec<i128>,
            b: Vec<i128>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::hadamard(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Finds the smallest linear constant-coefficient recurrence that fits every supplied term.
        #[pyfunction]
        #[pyo3(name = "recurrence", signature = (terms))]
        pub fn recurrence<'py>(py: Python<'py>, terms: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::recurrence(&terms);
            (out).into_bound_py_any(py)
        }

        /// Multiplies every term of a sequence by the factor.
        #[pyfunction]
        #[pyo3(name = "scale", signature = (a, factor))]
        pub fn scale<'py>(
            py: Python<'py>,
            a: Vec<i128>,
            factor: i128,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::scale(&a, factor);
            (out).into_bound_py_any(py)
        }

        /// Drops the first terms of a sequence.
        #[pyfunction]
        #[pyo3(name = "shift", signature = (a, count))]
        pub fn shift<'py>(
            py: Python<'py>,
            a: Vec<i128>,
            count: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::shift(&a, count);
            (out).into_bound_py_any(py)
        }

        /// Returns the partial sums of a sequence.
        #[pyfunction]
        #[pyo3(name = "sigma", signature = (a))]
        pub fn sigma<'py>(py: Python<'py>, a: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::sigma(&a);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Subtracts the second sequence from the first over their shared length.
        #[pyfunction]
        #[pyo3(name = "sub", signature = (a, b))]
        pub fn sub<'py>(
            py: Python<'py>,
            a: Vec<i128>,
            b: Vec<i128>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::sub(&a, &b);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.blend")?;
            m.setattr(
                "__doc__",
                "The sequence blender: term ops, exact recurrences and growth rates.",
            )?;
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
            let names: Vec<&str> = vec![
                "add",
                "cauchy",
                "characteristic",
                "decimate",
                "delta",
                "growth",
                "hadamard",
                "recurrence",
                "scale",
                "shift",
                "sigma",
                "sub",
            ];
            m.add("__all__", names)?;
            parent.add("blend", &m)?;
            sys.set_item("mrly.num._num.num.blend", &m)?;
            Ok(())
        }
    }

    /// The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.
    pub mod boolean {
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Reports whether the packed function outputs one on exactly half of its inputs.
        #[pyfunction]
        #[pyo3(name = "is_balanced", signature = (code, n))]
        pub fn is_balanced<'py>(
            py: Python<'py>,
            code: u128,
            n: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::boolean::is_balanced(code, n);
            (out).into_bound_py_any(py)
        }

        /// Returns how far the packed function sits from every affine function, zero when it is one.
        #[pyfunction]
        #[pyo3(name = "nonlinearity", signature = (code, n))]
        pub fn nonlinearity<'py>(
            py: Python<'py>,
            code: u128,
            n: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::boolean::nonlinearity(code, n);
            (out).into_bound_py_any(py)
        }

        /// Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche.
        #[pyfunction]
        #[pyo3(name = "sac", signature = (code, n))]
        pub fn sac<'py>(py: Python<'py>, code: u128, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::boolean::sac(code, n);
            (out).into_bound_py_any(py)
        }

        /// Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code.
        #[pyfunction]
        #[pyo3(name = "walsh_spectrum", signature = (code, n))]
        pub fn walsh_spectrum<'py>(
            py: Python<'py>,
            code: u128,
            n: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::boolean::walsh_spectrum(code, n);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.boolean")?;
            m.setattr("__doc__", "The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.")?;
            m.add_function(wrap_pyfunction!(is_balanced, &m)?)?;
            m.add_function(wrap_pyfunction!(nonlinearity, &m)?)?;
            m.add_function(wrap_pyfunction!(sac, &m)?)?;
            m.add_function(wrap_pyfunction!(walsh_spectrum, &m)?)?;
            let names: Vec<&str> = vec!["is_balanced", "nonlinearity", "sac", "walsh_spectrum"];
            m.add("__all__", names)?;
            parent.add("boolean", &m)?;
            sys.set_item("mrly.num._num.num.boolean", &m)?;
            Ok(())
        }
    }

    /// The Chladni plate: the figures of a free square plate, their ladder by pitch, sand shaken onto a raster's still lines and those lines traced.
    pub mod chladni {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Lists every distinct plate figure with `0 <= n <= m <= cap` but `(0, 0)`, both signs when `n < m` and the sign 1 when `n == m`, by pitch, then `n`, then the sign 1 before -1.
        #[pyfunction]
        #[pyo3(name = "ladder", signature = (cap))]
        pub fn ladder<'py>(py: Python<'py>, cap: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::chladni::ladder(cap);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Traces the zero set of a plate raster by marching squares on the lattice of cell centres, its holes and its rim ghosted to the mean of their plate neighbours so the lines run to every free edge: segments `x0, y0, x1, y1` in `[0, 1]` back to back, pieces inside a NaN cell dropped, a saddle joined by the mean of its four corners.
        #[pyfunction]
        #[pyo3(name = "lines", signature = (field, side))]
        pub fn lines<'py>(
            py: Python<'py>,
            field: Vec<f64>,
            side: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::chladni::lines(&field, side);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Samples the Chladni figure `cos(n pi x) cos(m pi y) + sign cos(m pi x) cos(n pi y)` of a free square plate at the cell centres `(i + 1/2) / size` of a row-major `size` by `size` raster, `x` along a row, scaled so the largest magnitude is 1, or all zero when no centre rises past `1e-12`; when `n == m` it is the single product for either sign.
        #[pyfunction]
        #[pyo3(name = "plate", signature = (n, m, sign, size))]
        pub fn plate<'py>(
            py: Python<'py>,
            n: usize,
            m: usize,
            sign: i32,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::chladni::plate(n, m, sign, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Shakes sand on a plate raster: grains are `x, y` pairs in `[0, 1)` over the row-major `side` by `side` field, a NaN pair an empty slot that stays NaN, a NaN cell a hole. Each step moves a grain a random length `kick |f| + floor` in a random direction plus a drift `-0.3 f grad f / max|grad f|^2`, with `kick = 0.6 / max|grad f|` read off the raster by finite differences and a fixed kick on a flat field, the field read bilinearly with its holes and its rim ghosted to the mean of their plate neighbours. The rim reflects; a grain on a hole falls through and becomes NaN. Step `from + i` of grain `j` draws from a counter keyed by `(seed, from + i, j)`, so six steps are three and three.
        #[pyfunction]
        #[pyo3(name = "shake", signature = (field, side, grains, from_, steps, seed))]
        pub fn shake<'py>(
            py: Python<'py>,
            field: Vec<f64>,
            side: usize,
            grains: Vec<f32>,
            from_: u64,
            steps: usize,
            seed: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::chladni::shake(&field, side, &grains, from_, steps, seed);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.chladni")?;
            m.setattr("__doc__", "The Chladni plate: the figures of a free square plate, their ladder by pitch, sand shaken onto a raster's still lines and those lines traced.")?;
            m.add_function(wrap_pyfunction!(ladder, &m)?)?;
            m.add_function(wrap_pyfunction!(lines, &m)?)?;
            m.add_function(wrap_pyfunction!(plate, &m)?)?;
            m.add_function(wrap_pyfunction!(shake, &m)?)?;
            let names: Vec<&str> = vec!["ladder", "lines", "plate", "shake"];
            m.add("__all__", names)?;
            parent.add("chladni", &m)?;
            sys.set_item("mrly.num._num.num.chladni", &m)?;
            Ok(())
        }
    }

    /// The divisor arithmetic: factorizations, divisors, totients, radicals, the Mobius values and the exact whole-number arithmetic under them.
    pub mod factor {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.
        #[pyfunction]
        #[pyo3(name = "aliquot", signature = (number))]
        pub fn aliquot<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::aliquot(number);
            (out).into_bound_py_any(py)
        }

        /// Returns whether two numbers share no divisor above one.
        #[pyfunction]
        #[pyo3(name = "coprime", signature = (a, b))]
        pub fn coprime<'py>(py: Python<'py>, a: usize, b: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::coprime(a, b);
            (out).into_bound_py_any(py)
        }

        /// Builds every divisor of a wide number from its factorization, ascending, empty for zero.
        #[pyfunction]
        #[pyo3(name = "divisors", signature = (number))]
        pub fn divisors<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::divisors(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the factorial of the number, the product of one through it, erring past thirty-four.
        #[pyfunction]
        #[pyo3(name = "factorial", signature = (number))]
        pub fn factorial<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::factorial(number);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.
        #[pyfunction]
        #[pyo3(name = "factorize", signature = (number))]
        pub fn factorize<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::factorize(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.
        #[pyfunction]
        #[pyo3(name = "factorize_wide", signature = (number))]
        pub fn factorize_wide<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::factorize_wide(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes.
        #[pyfunction]
        #[pyo3(name = "gcd", signature = (a, b))]
        pub fn gcd<'py>(py: Python<'py>, a: u128, b: u128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::gcd(a, b);
            (out).into_bound_py_any(py)
        }

        /// Returns the least common multiple of two numbers, zero when either side is zero.
        #[pyfunction]
        #[pyo3(name = "lcm", signature = (a, b))]
        pub fn lcm<'py>(py: Python<'py>, a: usize, b: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::lcm(a, b);
            (out).into_bound_py_any(py)
        }

        /// Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.
        #[pyfunction]
        #[pyo3(name = "mobius", signature = (number))]
        pub fn mobius<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::mobius(number);
            (out).into_bound_py_any(py)
        }

        /// Sieves the Mobius values of zero through the limit in one pass.
        #[pyfunction]
        #[pyo3(name = "mobius_sieve", signature = (limit))]
        pub fn mobius_sieve<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::mobius_sieve(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.
        #[pyfunction]
        #[pyo3(name = "radical", signature = (number))]
        pub fn radical<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::radical(number);
            (out).into_bound_py_any(py)
        }

        /// Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.
        #[pyfunction]
        #[pyo3(name = "reduce", signature = (numerator, denominator))]
        pub fn reduce<'py>(
            py: Python<'py>,
            numerator: u128,
            denominator: u128,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::reduce(numerator, denominator);
            (out).into_bound_py_any(py)
        }

        /// Returns the sum of every divisor of the number raised to the power, so power zero counts them.
        #[pyfunction]
        #[pyo3(name = "sigma", signature = (number, power))]
        pub fn sigma<'py>(
            py: Python<'py>,
            number: usize,
            power: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::sigma(number, power);
            (out).into_bound_py_any(py)
        }

        /// Returns whether no prime squares into the number, true for one and false for zero.
        #[pyfunction]
        #[pyo3(name = "squarefree", signature = (number))]
        pub fn squarefree<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::squarefree(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the Euler totient of the number from its factorization, zero for zero and one for one.
        #[pyfunction]
        #[pyo3(name = "totient", signature = (number))]
        pub fn totient<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::totient(number);
            (out).into_bound_py_any(py)
        }

        /// Sieves the Euler totients of zero through n in one pass, the run beside the single value.
        #[pyfunction]
        #[pyo3(name = "totients", signature = (n))]
        pub fn totients<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::totients(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.
        #[pyfunction]
        #[pyo3(name = "twisted", signature = (number, rhythm))]
        pub fn twisted<'py>(
            py: Python<'py>,
            number: usize,
            rhythm: Vec<i8>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::twisted(number, &rhythm);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.factor")?;
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
            let names: Vec<&str> = vec![
                "aliquot",
                "coprime",
                "divisors",
                "factorial",
                "factorize",
                "factorize_wide",
                "gcd",
                "lcm",
                "mobius",
                "mobius_sieve",
                "radical",
                "reduce",
                "sigma",
                "squarefree",
                "totient",
                "totients",
                "twisted",
            ];
            m.add("__all__", names)?;
            parent.add("factor", &m)?;
            sys.set_item("mrly.num._num.num.factor", &m)?;
            Ok(())
        }
    }

    /// The fast Fourier transform in one and two dimensions.
    pub mod fft {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.
        #[pyfunction]
        #[pyo3(name = "convolve", signature = (field, kernel, size))]
        pub fn convolve<'py>(
            py: Python<'py>,
            field: Vec<f64>,
            kernel: Vec<f64>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::convolve(&field, &kernel, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared.
        #[pyfunction]
        #[pyo3(name = "convolve_with", signature = (field, kernel_re, kernel_im, size))]
        pub fn convolve_with<'py>(
            py: Python<'py>,
            field: Vec<f64>,
            kernel_re: Vec<f64>,
            kernel_im: Vec<f64>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::convolve_with(&field, &kernel_re, &kernel_im, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.
        #[pyfunction]
        #[pyo3(name = "embed_kernel", signature = (mask, side, size))]
        pub fn embed_kernel<'py>(
            py: Python<'py>,
            mask: Vec<u8>,
            side: usize,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::embed_kernel(&mask, side, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.
        #[pyfunction]
        #[pyo3(name = "log_spectrum", signature = (field, size))]
        pub fn log_spectrum<'py>(
            py: Python<'py>,
            field: Vec<f64>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::log_spectrum(&field, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.
        #[pyfunction]
        #[pyo3(name = "magnitude_spectrum", signature = (field, size))]
        pub fn magnitude_spectrum<'py>(
            py: Python<'py>,
            field: Vec<f64>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::magnitude_spectrum(&field, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
        #[pyfunction]
        #[pyo3(name = "peak_ring", signature = (profile))]
        pub fn peak_ring<'py>(py: Python<'py>, profile: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::peak_ring(&profile);
            (out).into_bound_py_any(py)
        }

        /// Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
        #[pyfunction]
        #[pyo3(name = "peak_wavelength", signature = (profile, size))]
        pub fn peak_wavelength<'py>(
            py: Python<'py>,
            profile: Vec<f64>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::peak_wavelength(&profile, size);
            (out).into_bound_py_any(py)
        }

        /// Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.
        #[pyfunction]
        #[pyo3(name = "radial_profile", signature = (spectrum, size))]
        pub fn radial_profile<'py>(
            py: Python<'py>,
            spectrum: Vec<f64>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::radial_profile(&spectrum, size);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Transforms a real size-square field forward by fft2, returning the real and imaginary parts.
        #[pyfunction]
        #[pyo3(name = "transform", signature = (field, size))]
        pub fn transform<'py>(
            py: Python<'py>,
            field: Vec<f64>,
            size: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::transform(&field, size);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.fft")?;
            m.setattr(
                "__doc__",
                "The fast Fourier transform in one and two dimensions.",
            )?;
            m.add_function(wrap_pyfunction!(convolve, &m)?)?;
            m.add_function(wrap_pyfunction!(convolve_with, &m)?)?;
            m.add_function(wrap_pyfunction!(embed_kernel, &m)?)?;
            m.add_function(wrap_pyfunction!(log_spectrum, &m)?)?;
            m.add_function(wrap_pyfunction!(magnitude_spectrum, &m)?)?;
            m.add_function(wrap_pyfunction!(peak_ring, &m)?)?;
            m.add_function(wrap_pyfunction!(peak_wavelength, &m)?)?;
            m.add_function(wrap_pyfunction!(radial_profile, &m)?)?;
            m.add_function(wrap_pyfunction!(transform, &m)?)?;
            let names: Vec<&str> = vec![
                "convolve",
                "convolve_with",
                "embed_kernel",
                "log_spectrum",
                "magnitude_spectrum",
                "peak_ring",
                "peak_wavelength",
                "radial_profile",
                "transform",
            ];
            m.add("__all__", names)?;
            parent.add("fft", &m)?;
            sys.set_item("mrly.num._num.num.fft", &m)?;
            Ok(())
        }
    }

    /// The primes of the plane: the Gaussian and the Eisenstein integers, their classes, windows and ring weights.
    pub mod gauss {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The symmetric window of one ring: every point within a reach, with the norms sieved once.
        #[pyclass(name = "Window", module = "mrly.num.gauss", from_py_object)]
        #[derive(Clone)]
        pub struct Window(pub mrly_num::gauss::Window);

        #[pymethods]
        impl Window {
            /// Opens the window of a ring out to a reach, sieving every norm inside it.
            #[new]
            #[pyo3(signature = (ring, radius))]
            pub fn __new__(ring: PySerde<mrly_num::gauss::Ring>, radius: u64) -> PyResult<Self> {
                let ring = ring.0;
                let out = mrly_num::gauss::Window::new(ring, radius);
                Ok(Self(out))
            }
            /// Counts every class inside.
            #[pyo3(name = "census", signature = ())]
            pub fn census<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::census(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime.
            #[pyo3(name = "class_", signature = (a, b))]
            pub fn class_<'py>(
                &self,
                py: Python<'py>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::class(&self.0, a, b);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns whether a point lies inside.
            #[pyo3(name = "holds", signature = (a, b))]
            pub fn holds<'py>(
                &self,
                py: Python<'py>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::holds(&self.0, a, b);
                (out).into_bound_py_any(py)
            }
            /// Opens the window of a ring out to a reach, sieving every norm inside it.
            #[staticmethod]
            #[pyo3(name = "new", signature = (ring, radius))]
            pub fn new_<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                radius: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Window::new(ring, radius);
                (crate::doors::num::gauss::Window(out)).into_bound_py_any(py)
            }
            /// Lists every point inside, row by row from the bottom left of the bounding square.
            #[pyo3(name = "points", signature = ())]
            pub fn points<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::points(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the reach.
            #[pyo3(name = "radius", signature = ())]
            pub fn radius<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::radius(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring.
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::ring(&self.0);
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
        #[pyclass(name = "Class", module = "mrly.num.gauss", skip_from_py_object)]
        pub struct Class;

        #[pymethods]
        impl Class {
            /// Returns whether the class is prime.
            #[staticmethod]
            #[pyo3(name = "prime", signature = (class_))]
            pub fn prime<'py>(
                py: Python<'py>,
                class_: PySerde<mrly_num::gauss::Class>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let class_ = class_.0;
                let out = mrly_num::gauss::Class::prime(class_);
                (out).into_bound_py_any(py)
            }
            /// Returns the class as a word.
            #[staticmethod]
            #[pyo3(name = "word", signature = (class_))]
            pub fn word<'py>(
                py: Python<'py>,
                class_: PySerde<mrly_num::gauss::Class>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let class_ = class_.0;
                let out = mrly_num::gauss::Class::word(class_);
                (out).into_bound_py_any(py)
            }
        }

        /// The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice.
        #[pyclass(name = "Ring", module = "mrly.num.gauss", skip_from_py_object)]
        pub struct Ring;

        #[pymethods]
        impl Ring {
            /// Returns the unit multiples of a point, the point first, turning anticlockwise.
            #[staticmethod]
            #[pyo3(name = "associates", signature = (ring, a, b))]
            pub fn associates<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::associates(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin.
            #[staticmethod]
            #[pyo3(name = "canon", signature = (ring, a, b))]
            pub fn canon<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::canon(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the conjugate: the mirror image in the real axis.
            #[staticmethod]
            #[pyo3(name = "conjugate", signature = (ring, a, b))]
            pub fn conjugate<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::conjugate(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of points within the reach: the square or the hexagon.
            #[staticmethod]
            #[pyo3(name = "count", signature = (ring, radius))]
            pub fn count<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                radius: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::count(ring, radius);
                (out).into_bound_py_any(py)
            }
            /// Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`.
            #[staticmethod]
            #[pyo3(name = "div_rem", signature = (ring, z, w))]
            pub fn div_rem<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                z: (i64, i64),
                w: (i64, i64),
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::div_rem(ring, z, w);
                (out).into_bound_py_any(py)
            }
            /// Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.
            #[staticmethod]
            #[pyo3(name = "fate", signature = (ring, n))]
            pub fn fate<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                n: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::fate(ring, n);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins.
            #[staticmethod]
            #[pyo3(name = "gaussian_gcd", signature = (ring, z, w))]
            pub fn gaussian_gcd<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                z: (i64, i64),
                w: (i64, i64),
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::gaussian_gcd(ring, z, w);
                (out).into_bound_py_any(py)
            }
            /// Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3.
            #[staticmethod]
            #[pyo3(name = "inert", signature = (ring, p))]
            pub fn inert<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                p: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::inert(ring, p);
                (out).into_bound_py_any(py)
            }
            /// Returns the product of two points.
            #[staticmethod]
            #[pyo3(name = "mul", signature = (ring, arg1, arg2))]
            pub fn mul<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                arg1: (i64, i64),
                arg2: (i64, i64),
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::mul(ring, arg1, arg2);
                (out).into_bound_py_any(py)
            }
            /// Reads a ring from its name.
            #[staticmethod]
            #[pyo3(name = "named", signature = (name))]
            pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Ring::named(name);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
            /// Returns the point nearest a place in the plane.
            #[staticmethod]
            #[pyo3(name = "nearest", signature = (ring, x, y))]
            pub fn nearest<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                x: f64,
                y: f64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::nearest(ring, x, y);
                (out).into_bound_py_any(py)
            }
            /// Returns the norm of a point: its squared length.
            #[staticmethod]
            #[pyo3(name = "norm", signature = (ring, a, b))]
            pub fn norm<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::norm(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the place of a point in the plane, x right and y up, one unit between neighbours.
            #[staticmethod]
            #[pyo3(name = "place", signature = (ring, a, b))]
            pub fn place<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::place(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the one rational prime that ramifies: 2 or 3.
            #[staticmethod]
            #[pyo3(name = "ramified", signature = (ring))]
            pub fn ramified<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::ramified(ring);
                (out).into_bound_py_any(py)
            }
            /// Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.
            #[staticmethod]
            #[pyo3(name = "reach", signature = (ring, a, b))]
            pub fn reach<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::reach(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.
            #[staticmethod]
            #[pyo3(name = "symmetry", signature = (ring))]
            pub fn symmetry<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::symmetry(ring);
                (out).into_bound_py_any(py)
            }
            /// Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.
            #[staticmethod]
            #[pyo3(name = "top", signature = (ring, radius))]
            pub fn top<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                radius: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::top(ring, radius);
                (out).into_bound_py_any(py)
            }
            /// Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.
            #[staticmethod]
            #[pyo3(name = "turn", signature = (ring, a, b))]
            pub fn turn<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::turn(ring, a, b);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of units: 4 or 6.
            #[staticmethod]
            #[pyo3(name = "units", signature = (ring))]
            pub fn units<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::units(ring);
                (out).into_bound_py_any(py)
            }
            /// Returns the whole number an associate of the point lies on, when one lies on the positive real axis.
            #[staticmethod]
            #[pyo3(name = "whole", signature = (ring, a, b))]
            pub fn whole<'py>(
                py: Python<'py>,
                ring: PySerde<mrly_num::gauss::Ring>,
                a: i64,
                b: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let ring = ring.0;
                let out = mrly_num::gauss::Ring::whole(ring, a, b);
                (out).into_bound_py_any(py)
            }
        }

        /// Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates.
        #[pyfunction]
        #[pyo3(name = "classes", signature = (ring, bound))]
        pub fn classes<'py>(
            py: Python<'py>,
            ring: PySerde<mrly_num::gauss::Ring>,
            bound: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let ring = ring.0;
            let out = mrly_num::gauss::classes(ring, bound);
            (out).into_bound_py_any(py)
        }

        /// Returns the norm from one through the limit with the most points and that count, the earliest on a tie.
        #[pyfunction]
        #[pyo3(name = "peak", signature = (ring, limit))]
        pub fn peak<'py>(
            py: Python<'py>,
            ring: PySerde<mrly_num::gauss::Ring>,
            limit: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let ring = ring.0;
            let out = mrly_num::gauss::peak(ring, limit);
            (out).into_bound_py_any(py)
        }

        /// Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.
        #[pyfunction]
        #[pyo3(name = "shells", signature = (ring, limit))]
        pub fn shells<'py>(
            py: Python<'py>,
            ring: PySerde<mrly_num::gauss::Ring>,
            limit: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let ring = ring.0;
            let out = mrly_num::gauss::shells(ring, limit);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.gauss")?;
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
            sys.set_item("mrly.num._num.num.gauss", &m)?;
            Ok(())
        }
    }

    /// The Kummer sets: the `k` with a prime `p` not dividing `C(a k, b k)`, read digit by digit from the units by the carry automaton of Kummer's theorem, counted below `p^level` and set against Legendre's valuation.
    pub mod kummer {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Builds the carry automaton of `K_(a,b)` at the prime, `a` the top and `b` the bottom.
        #[pyfunction]
        #[pyo3(name = "carry", signature = (prime, top, bottom))]
        pub fn carry<'py>(
            py: Python<'py>,
            prime: u64,
            top: u64,
            bottom: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::kummer::carry(prime, top, bottom);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the columns the automaton reads for `k`, units first: one per digit of `k`, then the zeros that spend the carries, ending at the first column that carries or once every digit is read and the state is `(0, 0)` again.
        #[pyfunction]
        #[pyo3(name = "columns", signature = (k, prime, top, bottom))]
        pub fn columns<'py>(
            py: Python<'py>,
            k: u64,
            prime: u64,
            top: u64,
            bottom: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::kummer::columns(k, prime, top, bottom);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the count of `k < p^level` in `K_(a,b)`: the words of length `level` the automaton allows, summed by the state they end in over the states that close.
        #[pyfunction]
        #[pyo3(name = "count", signature = (prime, top, bottom, level))]
        pub fn count<'py>(
            py: Python<'py>,
            prime: u64,
            top: u64,
            bottom: u64,
            level: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::kummer::count(prime, top, bottom, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns whether `k` lies in `K_(a,b)`, read by the carry automaton: whether the prime does not divide `C(a k, b k)`.
        #[pyfunction]
        #[pyo3(name = "member", signature = (k, prime, top, bottom))]
        pub fn member<'py>(
            py: Python<'py>,
            k: u64,
            prime: u64,
            top: u64,
            bottom: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::kummer::member(k, prime, top, bottom);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns `v_p(C(a k, b k))` by Legendre's digit sums, `(s_p(b k) + s_p((a - b) k) - s_p(a k))/(p - 1)` with `s_p` the digit sum in base `p`: the carries of `b k + (a - b) k`.
        #[pyfunction]
        #[pyo3(name = "valuation", signature = (k, prime, top, bottom))]
        pub fn valuation<'py>(
            py: Python<'py>,
            k: u64,
            prime: u64,
            top: u64,
            bottom: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::kummer::valuation(k, prime, top, bottom);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the least witness that `K_(a,b)` is no digit design, as the pair `(u, v p)` of elements: digits `u, v < p` with `u` and `v p` in the set and `u + v p` outside it, least `u + v p` first; a digit design holding `u` and `v p` holds `u + v p`. `None` when no two such digits exist, as on the half interval `K_(2,1)`.
        #[pyfunction]
        #[pyo3(name = "witness", signature = (prime, top, bottom))]
        pub fn witness<'py>(
            py: Python<'py>,
            prime: u64,
            top: u64,
            bottom: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::kummer::witness(prime, top, bottom);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.kummer")?;
            m.setattr("__doc__", "The Kummer sets: the `k` with a prime `p` not dividing `C(a k, b k)`, read digit by digit from the units by the carry automaton of Kummer's theorem, counted below `p^level` and set against Legendre's valuation.")?;
            m.add_function(wrap_pyfunction!(carry, &m)?)?;
            m.add_function(wrap_pyfunction!(columns, &m)?)?;
            m.add_function(wrap_pyfunction!(count, &m)?)?;
            m.add_function(wrap_pyfunction!(member, &m)?)?;
            m.add_function(wrap_pyfunction!(valuation, &m)?)?;
            m.add_function(wrap_pyfunction!(witness, &m)?)?;
            m.add("MOST", mrly_num::kummer::MOST)?;
            m.add("TABLE", mrly_num::kummer::TABLE)?;
            let names: Vec<&str> = vec![
                "carry",
                "columns",
                "count",
                "member",
                "valuation",
                "witness",
                "MOST",
                "TABLE",
            ];
            m.add("__all__", names)?;
            parent.add("kummer", &m)?;
            sys.set_item("mrly.num._num.num.kummer", &m)?;
            Ok(())
        }
    }

    /// The visible lattice: coprime pairs, the constant a dimension recovers and the Farey nodes.
    pub mod lattice {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.
        #[pyfunction]
        #[pyo3(name = "coprime_pairs", signature = (n))]
        pub fn coprime_pairs<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::coprime_pairs(n);
            (out).into_bound_py_any(py)
        }

        /// Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending.
        #[pyfunction]
        #[pyo3(name = "farey", signature = (order))]
        pub fn farey<'py>(py: Python<'py>, order: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::farey(order);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.
        #[pyfunction]
        #[pyo3(name = "grid", signature = (n))]
        pub fn grid<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::grid(n);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.
        #[pyfunction]
        #[pyo3(name = "new_nodes", signature = (n))]
        pub fn new_nodes<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::new_nodes(n);
            (out).into_bound_py_any(py)
        }

        /// Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.
        #[pyfunction]
        #[pyo3(name = "pi_estimate", signature = (n))]
        pub fn pi_estimate<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::pi_estimate(n);
            (out).into_bound_py_any(py)
        }

        /// Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one.
        #[pyfunction]
        #[pyo3(name = "recovered", signature = (n, dimension))]
        pub fn recovered<'py>(
            py: Python<'py>,
            n: usize,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::recovered(n, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The density the visible count of a window in the dimension walks to, one over zeta of the dimension.
        #[pyfunction]
        #[pyo3(name = "visible_density", signature = (dimension))]
        pub fn visible_density<'py>(
            py: Python<'py>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::visible_density(dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve.
        #[pyfunction]
        #[pyo3(name = "zeta_factor", signature = (dimension))]
        pub fn zeta_factor<'py>(py: Python<'py>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::zeta_factor(dimension);
            (out).into_bound_py_any(py)
        }

        /// The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one.
        #[pyfunction]
        #[pyo3(name = "zeta_whole", signature = (s))]
        pub fn zeta_whole<'py>(py: Python<'py>, s: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::zeta_whole(s);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.lattice")?;
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
            let names: Vec<&str> = vec![
                "coprime_pairs",
                "farey",
                "grid",
                "new_nodes",
                "pi_estimate",
                "recovered",
                "visible_density",
                "zeta_factor",
                "zeta_whole",
            ];
            m.add("__all__", names)?;
            parent.add("lattice", &m)?;
            sys.set_item("mrly.num._num.num.lattice", &m)?;
            Ok(())
        }
    }

    /// The Thue-Morse world: the digit rule, the substitution, the plane lifts, the runs and the period-doubling word.
    pub mod morse {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The four ways the word lifts from a line to the plane, one sign at every site.
        #[pyclass(name = "Lift", module = "mrly.num.morse", skip_from_py_object)]
        pub struct Lift;

        #[pymethods]
        impl Lift {
            /// Returns every Lift in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::morse::Lift::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the sign at a site, zero for plus one and one for minus one.
            #[staticmethod]
            #[pyo3(name = "at", signature = (lift, i, j))]
            pub fn at<'py>(
                py: Python<'py>,
                lift: PySerde<mrly_num::morse::Lift>,
                i: u64,
                j: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lift = lift.0;
                let out = mrly_num::morse::Lift::at(lift, i, j);
                (out).into_bound_py_any(py)
            }
            /// Returns the lift's formula, written the way the page prints it.
            #[staticmethod]
            #[pyo3(name = "formula", signature = (lift))]
            pub fn formula<'py>(
                py: Python<'py>,
                lift: PySerde<mrly_num::morse::Lift>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lift = lift.0;
                let out = mrly_num::morse::Lift::formula(lift);
                (out).into_bound_py_any(py)
            }
        }

        /// Returns the run-boundary word, one wherever a letter differs from the next.
        #[pyfunction]
        #[pyo3(name = "boundary", signature = (word))]
        pub fn boundary<'py>(py: Python<'py>, word: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::boundary(&word);
            (out).into_bound_py_any(py)
        }

        /// Exclusive-ors two grids of the same length, site by site.
        #[pyfunction]
        #[pyo3(name = "difference", signature = (a, b))]
        pub fn difference<'py>(
            py: Python<'py>,
            a: Vec<u8>,
            b: Vec<u8>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::difference(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Builds the first letters of the Thue-Morse word by the digit rule.
        #[pyfunction]
        #[pyo3(name = "digits", signature = (length))]
        pub fn digits<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::digits(length);
            (out).into_bound_py_any(py)
        }

        /// Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.
        #[pyfunction]
        #[pyo3(name = "doubling", signature = (length))]
        pub fn doubling<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::doubling(length);
            (out).into_bound_py_any(py)
        }

        /// Counts the sites where two grids of the same length differ.
        #[pyfunction]
        #[pyo3(name = "faults", signature = (a, b))]
        pub fn faults<'py>(py: Python<'py>, a: Vec<u8>, b: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::faults(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Tests a grid against the Kronecker power of its own corner tile.
        #[pyfunction]
        #[pyo3(name = "fold", signature = (grid, side, number))]
        pub fn fold<'py>(
            py: Python<'py>,
            grid: Vec<u8>,
            side: usize,
            number: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::fold(&grid, side, number);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
        #[pyfunction]
        #[pyo3(name = "letter", signature = (place))]
        pub fn letter<'py>(py: Python<'py>, place: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::letter(place);
            (out).into_bound_py_any(py)
        }

        /// Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.
        #[pyfunction]
        #[pyo3(name = "lift", signature = (kind, side))]
        pub fn lift<'py>(
            py: Python<'py>,
            kind: PySerde<mrly_num::morse::Lift>,
            side: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let kind = kind.0;
            let out = mrly_num::morse::lift(kind, side);
            (out).into_bound_py_any(py)
        }

        /// Keeps the sites lit in both grids of the same length, site by site.
        #[pyfunction]
        #[pyo3(name = "mask", signature = (a, b))]
        pub fn mask<'py>(py: Python<'py>, a: Vec<u8>, b: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::mask(&a, &b);
            (out).into_bound_py_any(py)
        }

        /// Folds a tile of the side into its Kronecker power at the level, one bit per site.
        #[pyfunction]
        #[pyo3(name = "power", signature = (tile, number, level))]
        pub fn power<'py>(
            py: Python<'py>,
            tile: Vec<u8>,
            number: usize,
            level: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::power(&tile, number, level);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Repeats a tile until it fills a grid of the side.
        #[pyfunction]
        #[pyo3(name = "repeat", signature = (tile, number, side))]
        pub fn repeat<'py>(
            py: Python<'py>,
            tile: Vec<u8>,
            number: usize,
            side: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::repeat(&tile, number, side);
            (out).into_bound_py_any(py)
        }

        /// Returns the lengths of the maximal blocks of one repeated letter, in order.
        #[pyfunction]
        #[pyo3(name = "runs", signature = (word))]
        pub fn runs<'py>(py: Python<'py>, word: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::runs(&word);
            (out).into_bound_py_any(py)
        }

        /// Reads a design plus-minus: a filled cell is plus, zero, and a void is minus, one.
        #[pyfunction]
        #[pyo3(name = "signs", signature = (types))]
        pub fn signs<'py>(py: Python<'py>, types: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::signs(&types);
            (out).into_bound_py_any(py)
        }

        /// Returns the substitution stage after the rounds, a word of length two to the rounds.
        #[pyfunction]
        #[pyo3(name = "stage", signature = (rounds))]
        pub fn stage<'py>(py: Python<'py>, rounds: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::stage(rounds);
            (out).into_bound_py_any(py)
        }

        /// Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.
        #[pyfunction]
        #[pyo3(name = "substitution", signature = (length))]
        pub fn substitution<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::substitution(length);
            (out).into_bound_py_any(py)
        }

        /// Blows a grid up by the scale, every site becoming a scale-by-scale block.
        #[pyfunction]
        #[pyo3(name = "upsample", signature = (grid, side, scale))]
        pub fn upsample<'py>(
            py: Python<'py>,
            grid: Vec<u8>,
            side: usize,
            scale: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::upsample(&grid, side, scale);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.morse")?;
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
            m.add_function(wrap_pyfunction!(mask, &m)?)?;
            m.add_function(wrap_pyfunction!(power, &m)?)?;
            m.add_function(wrap_pyfunction!(repeat, &m)?)?;
            m.add_function(wrap_pyfunction!(runs, &m)?)?;
            m.add_function(wrap_pyfunction!(signs, &m)?)?;
            m.add_function(wrap_pyfunction!(stage, &m)?)?;
            m.add_function(wrap_pyfunction!(substitution, &m)?)?;
            m.add_function(wrap_pyfunction!(upsample, &m)?)?;
            m.add(
                "LIFTS",
                (mrly_num::morse::LIFTS)
                    .into_iter()
                    .map(PySerde)
                    .collect::<Vec<_>>(),
            )?;
            let names: Vec<&str> = vec![
                "boundary",
                "difference",
                "digits",
                "doubling",
                "faults",
                "fold",
                "letter",
                "lift",
                "mask",
                "power",
                "repeat",
                "runs",
                "signs",
                "stage",
                "substitution",
                "upsample",
                "Lift",
                "LIFTS",
            ];
            m.add("__all__", names)?;
            parent.add("morse", &m)?;
            sys.set_item("mrly.num._num.num.morse", &m)?;
            Ok(())
        }
    }

    /// The prime objects: the sieve and its readings, values, ranks, gaps, the counts they make and the shape readings of a number.
    pub mod prime {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it.
        #[pyclass(name = "Sieve", module = "mrly.num.prime", from_py_object)]
        #[derive(Clone)]
        pub struct Sieve(pub mrly_num::prime::Sieve);

        #[pymethods]
        impl Sieve {
            /// Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.
            #[new]
            #[pyo3(signature = (limit))]
            pub fn __new__(limit: usize) -> PyResult<Self> {
                let out = mrly_num::prime::Sieve::new(limit);
                Ok(Self(out))
            }
            /// Returns the count of numbers marked prime so far.
            #[pyo3(name = "count", signature = ())]
            pub fn count<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::count(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns whether every number is settled.
            #[pyo3(name = "done", signature = ())]
            pub fn done<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::done(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Runs the sieve to the end.
            #[pyo3(name = "finish", signature = ())]
            pub fn finish<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                mrly_num::prime::Sieve::finish(&mut self.0);
                ().into_bound_py_any(py)
            }
            /// Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.
            #[staticmethod]
            #[pyo3(name = "new", signature = (limit))]
            pub fn new_<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::new(limit);
                (crate::doors::num::prime::Sieve(out)).into_bound_py_any(py)
            }
            /// Returns the count of primes used so far.
            #[pyo3(name = "rank", signature = ())]
            pub fn rank<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::rank(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done.
            #[pyo3(name = "step", signature = ())]
            pub fn step<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::step(&mut self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the count of numbers the last step struck.
            #[pyo3(name = "struck", signature = ())]
            pub fn struck<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::struck(&self.0);
                (out).into_bound_py_any(py)
            }
            /// Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it.
            #[pyo3(name = "types", signature = ())]
            pub fn types<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::types(&self.0);
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
            let out = mrly_num::prime::chart(top, bins);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag.
        #[pyfunction]
        #[pyo3(name = "flags", signature = (limit))]
        pub fn flags<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::flags(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of unordered pairs of primes summing to the number, zero below four.
        #[pyfunction]
        #[pyo3(name = "goldbach", signature = (number))]
        pub fn goldbach<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::goldbach(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the count of prime pairs at every even number from four up to the top, one entry per even number.
        #[pyfunction]
        #[pyo3(name = "goldbach_record", signature = (top))]
        pub fn goldbach_record<'py>(py: Python<'py>, top: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::goldbach_record(top);
            (out).into_bound_py_any(py)
        }

        /// Returns whether the number is prime, by trial division on the six-step wheel.
        #[pyfunction]
        #[pyo3(name = "is_prime", signature = (number))]
        pub fn is_prime<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::is_prime(number);
            (out).into_bound_py_any(py)
        }

        /// Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.
        #[pyfunction]
        #[pyo3(name = "pile", signature = (number))]
        pub fn pile<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::pile(number);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Returns the count of primes at or below n.
        #[pyfunction]
        #[pyo3(name = "prime_count", signature = (n))]
        pub fn prime_count<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::prime_count(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the smallest prime at or above the number.
        #[pyfunction]
        #[pyo3(name = "prime_from", signature = (number))]
        pub fn prime_from<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::prime_from(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the primes up to the limit, the finished sieve read as a list.
        #[pyfunction]
        #[pyo3(name = "primes", signature = (limit))]
        pub fn primes<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::primes(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root.
        #[pyfunction]
        #[pyo3(name = "rectangles", signature = (number))]
        pub fn rectangles<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::rectangles(number);
            (out).into_bound_py_any(py)
        }

        /// Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.
        #[pyfunction]
        #[pyo3(name = "splits", signature = (number))]
        pub fn splits<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::splits(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the smallest pair of positive sides whose squares sum to the number, when one exists.
        #[pyfunction]
        #[pyo3(name = "squares", signature = (number))]
        pub fn squares<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::squares(number);
            (out).into_bound_py_any(py)
        }

        /// Returns one prime object for every prime up to and including the limit.
        #[pyfunction]
        #[pyo3(name = "study", signature = (limit))]
        pub fn study<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::study(limit);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.prime")?;
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
            let names: Vec<&str> = vec![
                "chart",
                "flags",
                "goldbach",
                "goldbach_record",
                "is_prime",
                "pile",
                "prime_count",
                "prime_from",
                "primes",
                "rectangles",
                "splits",
                "squares",
                "study",
                "Sieve",
            ];
            m.add("__all__", names)?;
            parent.add("prime", &m)?;
            sys.set_item("mrly.num._num.num.prime", &m)?;
            Ok(())
        }
    }

    /// The readings: a digit-parity design read along a polynomial or the primes, the battery that tests the signs, the near-heads that fix some of its pairs, and the major-arc model of the bias along the squares.
    pub mod readings {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// A digit-parity design `b:D`, or Rudin-Shapiro: the sign `x(n)` is `-1` to the count of base-`b` digits of `n` in `D`.
        #[pyclass(name = "Design", module = "mrly.num.readings", skip_from_py_object)]
        pub struct Design;

        #[pymethods]
        impl Design {
            /// Builds the design `b:D` from its base and its digit mask.
            #[staticmethod]
            #[pyo3(name = "digits", signature = (base, mask))]
            pub fn digits<'py>(py: Python<'py>, base: u8, mask: u8) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::digits(base, mask);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns Rudin-Shapiro.
            #[staticmethod]
            #[pyo3(name = "rudin", signature = ())]
            pub fn rudin<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::rudin();
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the ternary parity, `3:2`, the parity of the ternary digits equal to 2.
            #[staticmethod]
            #[pyo3(name = "ternary", signature = ())]
            pub fn ternary<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::ternary();
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns Thue-Morse, `2:1`.
            #[staticmethod]
            #[pyo3(name = "thue", signature = ())]
            pub fn thue<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::thue();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// The sequences a design is read along: `y(n) = x(P(n))` for a polynomial `P`, or `x(p_n)` along the primes.
        #[pyclass(name = "Reading", module = "mrly.num.readings", skip_from_py_object)]
        pub struct Reading;

        #[pymethods]
        impl Reading {
            /// Returns every Reading in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Reading::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the reading's formula, written the way the page prints it.
            #[staticmethod]
            #[pyo3(name = "formula", signature = (reading))]
            pub fn formula<'py>(
                py: Python<'py>,
                reading: PySerde<mrly_num::readings::Reading>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let reading = reading.0;
                let out = mrly_num::readings::Reading::formula(reading);
                (out).into_bound_py_any(py)
            }
        }

        /// Lists a design's major arcs along the squares below the cut, best first, each with its amplitude.
        #[pyfunction]
        #[pyo3(name = "arcs", signature = (design, cut))]
        pub fn arcs<'py>(
            py: Python<'py>,
            design: PySerde<mrly_num::readings::Design>,
            cut: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrly_num::readings::arcs(design, cut);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Runs the battery on a stream of signs.
        #[pyfunction]
        #[pyo3(name = "battery", signature = (stream))]
        pub fn battery<'py>(py: Python<'py>, stream: Vec<i8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::readings::battery(&stream);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Measures the bias `S(N) = sum_(n<N) y(n)` of a stream at `N = 2^e` for each exponent; a checkpoint past the stream's end sums the whole stream.
        #[pyfunction]
        #[pyo3(name = "bias", signature = (stream, exponents))]
        pub fn bias<'py>(
            py: Python<'py>,
            stream: Vec<i8>,
            exponents: Vec<u32>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::readings::bias(&stream, &exponents);
            (out).into_bound_py_any(py)
        }

        /// Lists the 57 digit designs, base by base and mask by mask, then Rudin-Shapiro.
        #[pyfunction]
        #[pyo3(name = "designs", signature = ())]
        pub fn designs<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::readings::designs();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Predicts the bias `S(N) = sum_(n<N) x(n^2)` at `N = 2^e` for each exponent, from the arcs below the cut: `1 + Re sum_gamma A(gamma) U(gamma; N^2)`.
        #[pyfunction]
        #[pyo3(name = "model", signature = (design, cut, exponents))]
        pub fn model<'py>(
            py: Python<'py>,
            design: PySerde<mrly_num::readings::Design>,
            cut: f64,
            exponents: Vec<u32>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrly_num::readings::model(design, cut, &exponents);
            (out).into_bound_py_any(py)
        }

        /// Returns the design's name in the `b:D` form, the digits of `D` ascending, or `rudin-shapiro`.
        #[pyfunction]
        #[pyo3(name = "name", signature = (design))]
        pub fn name<'py>(
            py: Python<'py>,
            design: PySerde<mrly_num::readings::Design>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrly_num::readings::name(design);
            (out).into_bound_py_any(py)
        }

        /// Lists the battery pairs that are near-heads of a polynomial reading at the design's base, the 44 dilation pairs first and the two kernel pairs after; the primes have none.
        #[pyfunction]
        #[pyo3(name = "near_heads", signature = (design, reading))]
        pub fn near_heads<'py>(
            py: Python<'py>,
            design: PySerde<mrly_num::readings::Design>,
            reading: PySerde<mrly_num::readings::Reading>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let reading = reading.0;
            let out = mrly_num::readings::near_heads(design, reading);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        /// Returns the design's sign at a number, plus or minus one.
        #[pyfunction]
        #[pyo3(name = "sign", signature = (design, n))]
        pub fn sign<'py>(
            py: Python<'py>,
            design: PySerde<mrly_num::readings::Design>,
            n: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let out = mrly_num::readings::sign(design, n);
            (out).into_bound_py_any(py)
        }

        /// Builds a reading's first signs: `y(n) = x(P(n))` for `n < count`, or `x(p_n)` along the first `count` primes from a segmented sieve.
        #[pyfunction]
        #[pyo3(name = "stream", signature = (design, reading, count))]
        pub fn stream<'py>(
            py: Python<'py>,
            design: PySerde<mrly_num::readings::Design>,
            reading: PySerde<mrly_num::readings::Reading>,
            count: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let design = design.0;
            let reading = reading.0;
            let out = mrly_num::readings::stream(design, reading, count);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.readings")?;
            m.setattr("__doc__", "The readings: a digit-parity design read along a polynomial or the primes, the battery that tests the signs, the near-heads that fix some of its pairs, and the major-arc model of the bias along the squares.\n\nA design is `f_{b,D}(n)`, the parity of the number of base-`b` digits of `n` in `D`, with `2 <= b <= 6` and `D` a subset of the digits `1..b-1`, named `b:D`, so `2:1` is Thue-Morse and `3:2` the ternary parity; its sign is `x(n) = (-1)^f(n)`. Rudin-Shapiro, the parity of the `11` blocks of the binary digits, rides beside them.\n\nA reading is `y(n) = x(P(n))` along a polynomial `P`, or `x(p_n)` along the consecutive primes. The battery reads a stream of signs by Pearson correlations: the shift pairs `(n, n + h)`, the 44 dilation pairs `(n, B n + r)`, the two kernel pairs `(b n, b^2 n)` and the dilation square `y(n) y(2n) y(3n) y(6n)`.\n\nA battery pair is a near-head when the composed polynomials satisfy `P(psi(n)) = b^L (P(phi(n)) + q) + s` with `0 <= s < b^L`; when `q = 0` the design's digits concatenate and the pair reads exactly `x(s)`.\n\nAlong the squares the circle method's major arcs model `S(N) = sum_(n<N) x(n^2)` as `1 + Re sum_gamma A(gamma) U(gamma; N^2)`, the arcs `gamma = c/(b^2 - 1)` being the points fixed by `B = b^2`, each with `eta = 1 - log_B abs(sigma)`, the amplitude `A = (G(-c, q)/q) K` a quadratic Gauss sum times the `B`-adic limit `K` of the digit recursion, and `U(gamma; M) = sum_(1<=m<M) x(m) e(m gamma)/(2 sqrt(m))` summed exactly block by block through a Taylor-moment recursion.")?;
            m.add_class::<Design>()?;
            m.add_class::<Reading>()?;
            m.add_function(wrap_pyfunction!(arcs, &m)?)?;
            m.add_function(wrap_pyfunction!(battery, &m)?)?;
            m.add_function(wrap_pyfunction!(bias, &m)?)?;
            m.add_function(wrap_pyfunction!(designs, &m)?)?;
            m.add_function(wrap_pyfunction!(model, &m)?)?;
            m.add_function(wrap_pyfunction!(name, &m)?)?;
            m.add_function(wrap_pyfunction!(near_heads, &m)?)?;
            m.add_function(wrap_pyfunction!(sign, &m)?)?;
            m.add_function(wrap_pyfunction!(stream, &m)?)?;
            m.add(
                "READINGS",
                (mrly_num::readings::READINGS)
                    .into_iter()
                    .map(PySerde)
                    .collect::<Vec<_>>(),
            )?;
            let names: Vec<&str> = vec![
                "arcs",
                "battery",
                "bias",
                "designs",
                "model",
                "name",
                "near_heads",
                "sign",
                "stream",
                "Design",
                "Reading",
                "READINGS",
            ];
            m.add("__all__", names)?;
            parent.add("readings", &m)?;
            sys.set_item("mrly.num._num.num.readings", &m)?;
            Ok(())
        }
    }

    /// The infinite sums: the classic sequences, zeta and its Dirichlet cousins, the partials that walk to pi, e and gamma, the visible count and the Bernoulli fractions.
    pub mod series {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.
        #[pyfunction]
        #[pyo3(name = "basel", signature = (n))]
        pub fn basel<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::basel(n);
            (out).into_bound_py_any(py)
        }

        /// Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.
        #[pyfunction]
        #[pyo3(name = "bernoulli", signature = (count))]
        pub fn bernoulli<'py>(py: Python<'py>, count: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::bernoulli(count);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums.
        #[pyfunction]
        #[pyo3(name = "beta", signature = (s, terms))]
        pub fn beta<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::beta(s, terms);
            (out).into_bound_py_any(py)
        }

        /// Returns the powers of two up to the limit.
        #[pyfunction]
        #[pyo3(name = "binary", signature = (limit))]
        pub fn binary<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::binary(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the distinct Catalan numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "catalan", signature = (limit))]
        pub fn catalan<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::catalan(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the mod-three rhythm of the number: zero, one, minus one.
        #[pyfunction]
        #[pyo3(name = "chi3", signature = (number))]
        pub fn chi3<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::chi3(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the mod-four rhythm of the number: zero, one, zero, minus one.
        #[pyfunction]
        #[pyo3(name = "chi4", signature = (number))]
        pub fn chi4<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::chi4(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.
        #[pyfunction]
        #[pyo3(name = "chi8", signature = (number))]
        pub fn chi8<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::chi8(number);
            (out).into_bound_py_any(py)
        }

        /// Returns the L-series partial sum with a periodic rhythm painted on the terms.
        #[pyfunction]
        #[pyo3(name = "dirichlet", signature = (s, rhythm, terms))]
        pub fn dirichlet<'py>(
            py: Python<'py>,
            s: f64,
            rhythm: Vec<i8>,
            terms: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::dirichlet(s, &rhythm, terms);
            (out).into_bound_py_any(py)
        }

        /// Returns one plus one over n raised to the n, walking to the natural base.
        #[pyfunction]
        #[pyo3(name = "e_partial", signature = (n))]
        pub fn e_partial<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::e_partial(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.
        #[pyfunction]
        #[pyo3(name = "euler_gamma_partial", signature = (n))]
        pub fn euler_gamma_partial<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::euler_gamma_partial(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.
        #[pyfunction]
        #[pyo3(name = "euler_product", signature = (s, limit))]
        pub fn euler_product<'py>(
            py: Python<'py>,
            s: f64,
            limit: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::euler_product(s, limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the even numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "evens", signature = (limit))]
        pub fn evens<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::evens(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the distinct Fibonacci numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "fibonacci", signature = (limit))]
        pub fn fibonacci<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::fibonacci(limit);
            (out).into_bound_py_any(py)
        }

        /// Returns the partial harmonic sum, the reciprocals of one through the term count.
        #[pyfunction]
        #[pyo3(name = "harmonic", signature = (terms))]
        pub fn harmonic<'py>(py: Python<'py>, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::harmonic(terms);
            (out).into_bound_py_any(py)
        }

        /// Returns the Dirichlet lambda value, one minus two to the minus s times zeta.
        #[pyfunction]
        #[pyo3(name = "lambda_", signature = (s, terms))]
        pub fn lambda_<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::lambda(s, terms);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.
        #[pyfunction]
        #[pyo3(name = "leibniz", signature = (n))]
        pub fn leibniz<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::leibniz(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.
        #[pyfunction]
        #[pyo3(name = "li", signature = (x))]
        pub fn li<'py>(py: Python<'py>, x: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::li(x);
            (out).into_bound_py_any(py)
        }

        /// Returns the Mertens function at n, the Mobius values of one through n summed.
        #[pyfunction]
        #[pyo3(name = "mertens", signature = (n))]
        pub fn mertens<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::mertens(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the odd numbers up to the limit.
        #[pyfunction]
        #[pyo3(name = "odds", signature = (limit))]
        pub fn odds<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::odds(limit);
            (out).into_bound_py_any(py)
        }

        /// Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.
        #[pyfunction]
        #[pyo3(name = "visible", signature = (limit, dimension))]
        pub fn visible<'py>(
            py: Python<'py>,
            limit: usize,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::visible(limit, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.
        #[pyfunction]
        #[pyo3(name = "wallis_half_pi", signature = (n))]
        pub fn wallis_half_pi<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::wallis_half_pi(n);
            (out).into_bound_py_any(py)
        }

        /// Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.
        #[pyfunction]
        #[pyo3(name = "wallis_quarter_pi", signature = (factors))]
        pub fn wallis_quarter_pi<'py>(
            py: Python<'py>,
            factors: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::wallis_quarter_pi(factors);
            (out).into_bound_py_any(py)
        }

        /// Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.
        #[pyfunction]
        #[pyo3(name = "zeta", signature = (s, terms))]
        pub fn zeta<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::zeta(s, terms);
            (ok(out)?).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.series")?;
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
            m.add("APERY", mrly_num::series::APERY)?;
            m.add("BASEL", mrly_num::series::BASEL)?;
            m.add("CATALAN", mrly_num::series::CATALAN)?;
            m.add("EULER", mrly_num::series::EULER)?;
            m.add("VISIBLE", mrly_num::series::VISIBLE)?;
            let names: Vec<&str> = vec![
                "basel",
                "bernoulli",
                "beta",
                "binary",
                "catalan",
                "chi3",
                "chi4",
                "chi8",
                "dirichlet",
                "e_partial",
                "euler_gamma_partial",
                "euler_product",
                "evens",
                "fibonacci",
                "harmonic",
                "lambda_",
                "leibniz",
                "li",
                "mertens",
                "odds",
                "visible",
                "wallis_half_pi",
                "wallis_quarter_pi",
                "zeta",
                "APERY",
                "BASEL",
                "CATALAN",
                "EULER",
                "VISIBLE",
            ];
            m.add("__all__", names)?;
            parent.add("series", &m)?;
            sys.set_item("mrly.num._num.num.series", &m)?;
            Ok(())
        }
    }

    /// The punctured schedules: the Wallis sieve and its kin, their words, rasters, punctures and limits.
    pub mod sieve {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.
        #[pyfunction]
        #[pyo3(name = "cells", signature = (word, dimension))]
        pub fn cells<'py>(
            py: Python<'py>,
            word: Vec<u64>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::cells(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
        #[pyfunction]
        #[pyo3(name = "exponent", signature = (word, dimension))]
        pub fn exponent<'py>(
            py: Python<'py>,
            word: Vec<u64>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::exponent(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
        #[pyfunction]
        #[pyo3(name = "flat_word", signature = (side, levels))]
        pub fn flat_word<'py>(
            py: Python<'py>,
            side: u64,
            levels: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::flat_word(side, levels);
            (out).into_bound_py_any(py)
        }

        /// Returns the punctures the word makes, one per surviving cell at every level.
        #[pyfunction]
        #[pyo3(name = "holes", signature = (word, dimension))]
        pub fn holes<'py>(
            py: Python<'py>,
            word: Vec<u64>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::holes(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.
        #[pyfunction]
        #[pyo3(name = "limit", signature = (word, dimension))]
        pub fn limit<'py>(
            py: Python<'py>,
            word: Vec<u64>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::limit(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
        #[pyfunction]
        #[pyo3(name = "odd_word", signature = (levels))]
        pub fn odd_word<'py>(py: Python<'py>, levels: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::odd_word(levels);
            (out).into_bound_py_any(py)
        }

        /// Returns the Wallis sieve product of a parity design, `prod_(N odd >= 3) (1 - N^-dim)` for the corners with an odd count of odd coordinates and `prod (1 + N^-dim)` for the even count, at `dim >= 2`.
        #[pyfunction]
        #[pyo3(name = "parity_product", signature = (dimension, odd))]
        pub fn parity_product<'py>(
            py: Python<'py>,
            dimension: u32,
            odd: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::parity_product(dimension, odd);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
        #[pyfunction]
        #[pyo3(name = "punctures", signature = (word, dimension))]
        pub fn punctures<'py>(
            py: Python<'py>,
            word: Vec<u64>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::punctures(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
        #[pyfunction]
        #[pyo3(name = "raster", signature = (word))]
        pub fn raster<'py>(py: Python<'py>, word: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::raster(&word);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
        #[pyfunction]
        #[pyo3(name = "ratio", signature = (word, dimension))]
        pub fn ratio<'py>(
            py: Python<'py>,
            word: Vec<u64>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::ratio(&word, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the cells a design of this profile fills at one side, `sum_j a_j E^(dim-j) O^j` with `E` and `O` the even and the odd positions an axis holds: `P_F(n)` at side `2n - 1` and `w n^dim` at side `2n`.
        #[pyfunction]
        #[pyo3(name = "row_fill", signature = (profile, side))]
        pub fn row_fill<'py>(
            py: Python<'py>,
            profile: Vec<u64>,
            side: u64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::row_fill(&profile, side);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Reads the design of this profile along the row word: its drift, its first correction, the roots of its fill polynomial, its constant from the Gamma form, the constant spelled, and the constant again by reflection, against the mirror and as a Wallis sieve product wherever those apply.
        #[pyfunction]
        #[pyo3(name = "row_law", signature = (profile))]
        pub fn row_law<'py>(py: Python<'py>, profile: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::row_law(&profile);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        /// Counts a base-2 design's corners by how many odd coordinates each holds, the profile the row word reads: the code is a bitmask over the corners, corner `i` is the binary digits of `i` as `math::bang::code_to_corners` reads it, so its odd coordinates are the ones of `i`.
        #[pyfunction]
        #[pyo3(name = "row_profile", signature = (code, dimension))]
        pub fn row_profile<'py>(
            py: Python<'py>,
            code: u128,
            dimension: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::row_profile(code, dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Walks the renormalised fill `R_L (2^dim/w)^L / L^drift` of the design of this profile to every stop, `R_L` the product of the letters' fill ratios, on the odd sides `3, 5, ..., 2L+1` or on the even sides `2, 4, ..., 2L`.
        #[pyfunction]
        #[pyo3(name = "row_settle", signature = (profile, stops, even))]
        pub fn row_settle<'py>(
            py: Python<'py>,
            profile: Vec<u64>,
            stops: Vec<usize>,
            even: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::row_settle(&profile, &stops, even);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the side of the word, the product of its letters' sides.
        #[pyfunction]
        #[pyo3(name = "side", signature = (word))]
        pub fn side<'py>(py: Python<'py>, word: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::side(&word);
            (ok(out)?).into_bound_py_any(py)
        }

        /// Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.
        #[pyfunction]
        #[pyo3(name = "solid_limit", signature = ())]
        pub fn solid_limit<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::solid_limit();
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.sieve")?;
            m.setattr("__doc__", "The punctured schedules: the Wallis sieve and its kin, their words, rasters, punctures and limits.")?;
            m.add_function(wrap_pyfunction!(cells, &m)?)?;
            m.add_function(wrap_pyfunction!(exponent, &m)?)?;
            m.add_function(wrap_pyfunction!(flat_word, &m)?)?;
            m.add_function(wrap_pyfunction!(holes, &m)?)?;
            m.add_function(wrap_pyfunction!(limit, &m)?)?;
            m.add_function(wrap_pyfunction!(odd_word, &m)?)?;
            m.add_function(wrap_pyfunction!(parity_product, &m)?)?;
            m.add_function(wrap_pyfunction!(punctures, &m)?)?;
            m.add_function(wrap_pyfunction!(raster, &m)?)?;
            m.add_function(wrap_pyfunction!(ratio, &m)?)?;
            m.add_function(wrap_pyfunction!(row_fill, &m)?)?;
            m.add_function(wrap_pyfunction!(row_law, &m)?)?;
            m.add_function(wrap_pyfunction!(row_profile, &m)?)?;
            m.add_function(wrap_pyfunction!(row_settle, &m)?)?;
            m.add_function(wrap_pyfunction!(side, &m)?)?;
            m.add_function(wrap_pyfunction!(solid_limit, &m)?)?;
            m.add("PLANE_LIMIT", mrly_num::sieve::PLANE_LIMIT)?;
            let names: Vec<&str> = vec![
                "cells",
                "exponent",
                "flat_word",
                "holes",
                "limit",
                "odd_word",
                "parity_product",
                "punctures",
                "raster",
                "ratio",
                "row_fill",
                "row_law",
                "row_profile",
                "row_settle",
                "side",
                "solid_limit",
                "PLANE_LIMIT",
            ];
            m.add("__all__", names)?;
            parent.add("sieve", &m)?;
            sys.set_item("mrly.num._num.num.sieve", &m)?;
            Ok(())
        }
    }

    /// The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.
    pub mod spiral {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// Which cells of the square winding grow into a tile.
        #[pyclass(name = "Growth", module = "mrly.num.spiral", skip_from_py_object)]
        pub struct Growth;

        #[pymethods]
        impl Growth {
            /// Returns every Growth in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::spiral::Growth::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right.
        #[pyclass(name = "Lattice", module = "mrly.num.spiral", skip_from_py_object)]
        pub struct Lattice;

        #[pymethods]
        impl Lattice {
            /// Returns every Lattice in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::spiral::Lattice::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.
            #[staticmethod]
            #[pyo3(name = "count", signature = (lattice, side))]
            pub fn count<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_num::spiral::Lattice>,
                side: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_num::spiral::Lattice::count(lattice, side);
                (out).into_bound_py_any(py)
            }
            /// Returns the number at a cell, one at the origin.
            #[staticmethod]
            #[pyo3(name = "n", signature = (lattice, x, y))]
            pub fn n<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_num::spiral::Lattice>,
                x: i64,
                y: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_num::spiral::Lattice::n(lattice, x, y);
                (out).into_bound_py_any(py)
            }
            /// Returns the outermost ring of a sheet the odd side wide, half the side rounded down.
            #[staticmethod]
            #[pyo3(name = "radius", signature = (lattice, side))]
            pub fn radius<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_num::spiral::Lattice>,
                side: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_num::spiral::Lattice::radius(lattice, side);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring a number sits on, zero for one.
            #[staticmethod]
            #[pyo3(name = "ring", signature = (lattice, n))]
            pub fn ring<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_num::spiral::Lattice>,
                n: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_num::spiral::Lattice::ring(lattice, n);
                (out).into_bound_py_any(py)
            }
            /// Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.
            #[staticmethod]
            #[pyo3(name = "ring_of", signature = (lattice, x, y))]
            pub fn ring_of<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_num::spiral::Lattice>,
                x: i64,
                y: i64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_num::spiral::Lattice::ring_of(lattice, x, y);
                (out).into_bound_py_any(py)
            }
            /// Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.
            #[staticmethod]
            #[pyo3(name = "xy", signature = (lattice, n))]
            pub fn xy<'py>(
                py: Python<'py>,
                lattice: PySerde<mrly_num::spiral::Lattice>,
                n: u64,
            ) -> PyResult<Bound<'py, PyAny>> {
                let lattice = lattice.0;
                let out = mrly_num::spiral::Lattice::xy(lattice, n);
                (out).into_bound_py_any(py)
            }
        }

        /// What a cell is painted for.
        #[pyclass(name = "Mark", module = "mrly.num.spiral", skip_from_py_object)]
        pub struct Mark;

        #[pymethods]
        impl Mark {
            /// Returns every Mark in canonical order.
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::spiral::Mark::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        /// Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak.
        #[pyfunction]
        #[pyo3(name = "diagonal", signature = (lattice, side, a, b, c))]
        pub fn diagonal<'py>(
            py: Python<'py>,
            lattice: PySerde<mrly_num::spiral::Lattice>,
            side: usize,
            a: i64,
            b: i64,
            c: i64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let lattice = lattice.0;
            let out = mrly_num::spiral::diagonal(lattice, side, a, b, c);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself.
        #[pyfunction]
        #[pyo3(name = "level_of", signature = (n, base))]
        pub fn level_of<'py>(py: Python<'py>, n: u64, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::spiral::level_of(n, base);
            (out).into_bound_py_any(py)
        }

        /// Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.
        #[pyfunction]
        #[pyo3(name = "marks", signature = (mark, limit))]
        pub fn marks<'py>(
            py: Python<'py>,
            mark: PySerde<mrly_num::spiral::Mark>,
            limit: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mark = mark.0;
            let out = mrly_num::spiral::marks(mark, limit);
            (out).into_bound_py_any(py)
        }

        /// Winds one to the top on the square spiral and lays a square tile on every cell, the snail.
        #[pyfunction]
        #[pyo3(name = "snail", signature = (base, top, growth))]
        pub fn snail<'py>(
            py: Python<'py>,
            base: u64,
            top: u64,
            growth: PySerde<mrly_num::spiral::Growth>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let growth = growth.0;
            let out = mrly_num::spiral::snail(base, top, growth);
            (PySerde(out)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.spiral")?;
            m.setattr("__doc__", "The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.")?;
            m.add_class::<Growth>()?;
            m.add_class::<Lattice>()?;
            m.add_class::<Mark>()?;
            m.add_function(wrap_pyfunction!(diagonal, &m)?)?;
            m.add_function(wrap_pyfunction!(level_of, &m)?)?;
            m.add_function(wrap_pyfunction!(marks, &m)?)?;
            m.add_function(wrap_pyfunction!(snail, &m)?)?;
            let names: Vec<&str> = vec![
                "diagonal", "level_of", "marks", "snail", "Growth", "Lattice", "Mark",
            ];
            m.add("__all__", names)?;
            parent.add("spiral", &m)?;
            sys.set_item("mrly.num._num.num.spiral", &m)?;
            Ok(())
        }
    }

    /// The critical line: zeta at one half plus i t and off it, its zeros, the prime staircase they rebuild and the novelty meter their waves predict.
    pub mod zeta {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        /// The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once.
        #[pyclass(name = "Line", module = "mrly.num.zeta", skip_from_py_object)]
        pub struct Line(pub mrly_num::zeta::Line);

        #[pymethods]
        impl Line {
            /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
            #[new]
            #[pyo3(signature = ())]
            pub fn __new__() -> PyResult<Self> {
                let out = mrly_num::zeta::Line::new();
                Ok(Self(out))
            }
            /// Counts the zeros on the line below t: the sign changes of Z on ten steps a Gram interval, the Riemann-Siegel main sum turned from step to step and its remainder read only where it could flip the sign.
            #[pyo3(name = "count", signature = (t))]
            pub fn count<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::count(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns the default Line.
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::default();
                (crate::doors::num::zeta::Line(out)).into_bound_py_any(py)
            }
            /// Returns Z(t) from the Euler-Maclaurin value turned onto the real axis.
            #[pyo3(name = "exact", signature = (t))]
            pub fn exact<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::exact(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns the n-th Gram point, where theta is n pi, by Newton from the right.
            #[pyo3(name = "gram", signature = (n))]
            pub fn gram<'py>(&self, py: Python<'py>, n: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::gram(&self.0, n);
                (out).into_bound_py_any(py)
            }
            /// Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections.
            #[pyo3(name = "maclaurin", signature = (t))]
            pub fn maclaurin<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::maclaurin(&self.0, t);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
            #[staticmethod]
            #[pyo3(name = "new", signature = ())]
            pub fn new_<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::new();
                (crate::doors::num::zeta::Line(out)).into_bound_py_any(py)
            }
            /// Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump.
            #[pyo3(name = "novelty_coefficients", signature = (gammas))]
            pub fn novelty_coefficients<'py>(
                &self,
                py: Python<'py>,
                gammas: Vec<f64>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::novelty_coefficients(&self.0, &gammas);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
            /// Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s.
            #[pyo3(name = "pair", signature = (s))]
            pub fn pair<'py>(
                &self,
                py: Python<'py>,
                s: PySerde<mrly_num::zeta::Complex>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let s = s.0;
                let out = mrly_num::zeta::Line::pair(&self.0, s);
                ({
                    let t = out;
                    (PySerde(t.0), PySerde(t.1))
                })
                .into_bound_py_any(py)
            }
            /// Returns zeta on the line and Z(t) together, from the engine that serves the t.
            #[pyo3(name = "point", signature = (t))]
            pub fn point<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::point(&self.0, t);
                ({
                    let t = out;
                    (PySerde(t.0), t.1)
                })
                .into_bound_py_any(py)
            }
            /// Returns the largest gap between the two engines over the t range on a grid.
            #[pyo3(name = "seam", signature = (t0, t1, steps))]
            pub fn seam<'py>(
                &self,
                py: Python<'py>,
                t0: f64,
                t1: f64,
                steps: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::seam(&self.0, t0, t1, steps);
                (out).into_bound_py_any(py)
            }
            /// Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections.
            #[pyo3(name = "siegel", signature = (t))]
            pub fn siegel<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::siegel(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten.
            #[pyo3(name = "theta", signature = (t))]
            pub fn theta<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::theta(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above.
            #[pyo3(name = "z", signature = (t))]
            pub fn z<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::z(&self.0, t);
                (out).into_bound_py_any(py)
            }
            /// Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth.
            #[pyo3(name = "zeros", signature = (count))]
            pub fn zeros<'py>(&self, py: Python<'py>, count: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::zeros(&self.0, count);
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

        /// A complex number: a real and an imaginary part.
        #[pyclass(name = "Complex", module = "mrly.num.zeta", skip_from_py_object)]
        pub struct Complex;

        #[pymethods]
        impl Complex {
            /// Returns the modulus.
            #[staticmethod]
            #[pyo3(name = "abs", signature = (z))]
            pub fn abs<'py>(
                py: Python<'py>,
                z: PySerde<mrly_num::zeta::Complex>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let z = z.0;
                let out = mrly_num::zeta::Complex::abs(z);
                (out).into_bound_py_any(py)
            }
            /// Returns the principal argument.
            #[staticmethod]
            #[pyo3(name = "arg", signature = (z))]
            pub fn arg<'py>(
                py: Python<'py>,
                z: PySerde<mrly_num::zeta::Complex>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let z = z.0;
                let out = mrly_num::zeta::Complex::arg(z);
                (out).into_bound_py_any(py)
            }
            /// Returns the default Complex.
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Complex::default();
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the exponential.
            #[staticmethod]
            #[pyo3(name = "exp", signature = (z))]
            pub fn exp<'py>(
                py: Python<'py>,
                z: PySerde<mrly_num::zeta::Complex>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let z = z.0;
                let out = mrly_num::zeta::Complex::exp(z);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns the principal logarithm.
            #[staticmethod]
            #[pyo3(name = "ln", signature = (z))]
            pub fn ln<'py>(
                py: Python<'py>,
                z: PySerde<mrly_num::zeta::Complex>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let z = z.0;
                let out = mrly_num::zeta::Complex::ln(z);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Builds a complex number from its parts.
            #[staticmethod]
            #[pyo3(name = "new", signature = (re, im))]
            pub fn new_<'py>(py: Python<'py>, re: f64, im: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Complex::new(re, im);
                (PySerde(out)).into_bound_py_any(py)
            }
            /// Returns a unit complex number at the given angle.
            #[staticmethod]
            #[pyo3(name = "turn", signature = (angle))]
            pub fn turn<'py>(py: Python<'py>, angle: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Complex::turn(angle);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        /// The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2.
        #[pyfunction]
        #[pyo3(name = "bump", signature = (u))]
        pub fn bump<'py>(py: Python<'py>, u: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::bump(u);
            (out).into_bound_py_any(py)
        }

        /// Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.
        #[pyfunction]
        #[pyo3(name = "corrections", signature = (p))]
        pub fn corrections<'py>(py: Python<'py>, p: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::corrections(p);
            (out).into_bound_py_any(py)
        }

        /// Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.
        #[pyfunction]
        #[pyo3(name = "kernel", signature = (p))]
        pub fn kernel<'py>(py: Python<'py>, p: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::kernel(p);
            (out).into_bound_py_any(py)
        }

        /// Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule.
        #[pyfunction]
        #[pyo3(name = "mellin", signature = (s))]
        pub fn mellin<'py>(
            py: Python<'py>,
            s: PySerde<mrly_num::zeta::Complex>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let s = s.0;
            let out = mrly_num::zeta::mellin(s);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two.
        #[pyfunction]
        #[pyo3(name = "novelty_main", signature = ())]
        pub fn novelty_main<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::novelty_main();
            (out).into_bound_py_any(py)
        }

        /// Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict.
        #[pyfunction]
        #[pyo3(name = "novelty_wave", signature = (gammas, coef, log_y))]
        pub fn novelty_wave<'py>(
            py: Python<'py>,
            gammas: Vec<f64>,
            coef: Vec<PySerde<mrly_num::zeta::Complex>>,
            log_y: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let coef = coef.into_iter().map(|x| x.0).collect::<Vec<_>>();
            let out = mrly_num::zeta::novelty_wave(&gammas, &coef, log_y);
            (out).into_bound_py_any(py)
        }

        /// Returns the explicit formula over the zeros at the given ordinates at every sample x.
        #[pyfunction]
        #[pyo3(name = "psi_curve", signature = (gammas, samples))]
        pub fn psi_curve<'py>(
            py: Python<'py>,
            gammas: Vec<f64>,
            samples: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::psi_curve(&gammas, &samples);
            (out).into_bound_py_any(py)
        }

        /// Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.
        #[pyfunction]
        #[pyo3(name = "psi_formula", signature = (x, gammas))]
        pub fn psi_formula<'py>(
            py: Python<'py>,
            x: f64,
            gammas: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::psi_formula(x, &gammas);
            (out).into_bound_py_any(py)
        }

        /// Returns how far the explicit formula over the zeros at the given ordinates lands from the staircase at the whole number x.
        #[pyfunction]
        #[pyo3(name = "psi_gap", signature = (x, gammas))]
        pub fn psi_gap<'py>(
            py: Python<'py>,
            x: usize,
            gammas: Vec<f64>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::psi_gap(x, &gammas);
            (out).into_bound_py_any(py)
        }

        /// Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.
        #[pyfunction]
        #[pyo3(name = "psi_stair", signature = (x))]
        pub fn psi_stair<'py>(py: Python<'py>, x: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::psi_stair(x);
            (out).into_bound_py_any(py)
        }

        /// Returns a positive real base raised to a complex exponent.
        #[pyfunction]
        #[pyo3(name = "raise_", signature = (base, exponent))]
        pub fn raise_<'py>(
            py: Python<'py>,
            base: f64,
            exponent: PySerde<mrly_num::zeta::Complex>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let exponent = exponent.0;
            let out = mrly_num::zeta::raise(base, exponent);
            (PySerde(out)).into_bound_py_any(py)
        }

        /// Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y.
        #[pyfunction]
        #[pyo3(name = "sharp_novelty", signature = (prefix, y))]
        pub fn sharp_novelty<'py>(
            py: Python<'py>,
            prefix: Vec<u64>,
            y: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::sharp_novelty(&prefix, y);
            (out).into_bound_py_any(py)
        }

        /// Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y.
        #[pyfunction]
        #[pyo3(name = "smoothed_novelty", signature = (phi, y, main))]
        pub fn smoothed_novelty<'py>(
            py: Python<'py>,
            phi: Vec<u64>,
            y: f64,
            main: f64,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::smoothed_novelty(&phi, y, main);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.num.zeta")?;
            m.setattr("__doc__", "The critical line: zeta at one half plus i t and off it, its zeros, the prime staircase they rebuild and the novelty meter their waves predict.")?;
            m.add_class::<Line>()?;
            m.add_class::<Complex>()?;
            m.add_function(wrap_pyfunction!(bump, &m)?)?;
            m.add_function(wrap_pyfunction!(corrections, &m)?)?;
            m.add_function(wrap_pyfunction!(kernel, &m)?)?;
            m.add_function(wrap_pyfunction!(mellin, &m)?)?;
            m.add_function(wrap_pyfunction!(novelty_main, &m)?)?;
            m.add_function(wrap_pyfunction!(novelty_wave, &m)?)?;
            m.add_function(wrap_pyfunction!(psi_curve, &m)?)?;
            m.add_function(wrap_pyfunction!(psi_formula, &m)?)?;
            m.add_function(wrap_pyfunction!(psi_gap, &m)?)?;
            m.add_function(wrap_pyfunction!(psi_stair, &m)?)?;
            m.add_function(wrap_pyfunction!(raise_, &m)?)?;
            m.add_function(wrap_pyfunction!(sharp_novelty, &m)?)?;
            m.add_function(wrap_pyfunction!(smoothed_novelty, &m)?)?;
            m.add("JOIN", mrly_num::zeta::JOIN)?;
            let names: Vec<&str> = vec![
                "bump",
                "corrections",
                "kernel",
                "mellin",
                "novelty_main",
                "novelty_wave",
                "psi_curve",
                "psi_formula",
                "psi_gap",
                "psi_stair",
                "raise_",
                "sharp_novelty",
                "smoothed_novelty",
                "Line",
                "Complex",
                "JOIN",
            ];
            m.add("__all__", names)?;
            parent.add("zeta", &m)?;
            sys.set_item("mrly.num._num.num.zeta", &m)?;
            Ok(())
        }
    }

    pub fn init(
        py: Python<'_>,
        parent: &Bound<'_, PyModule>,
        sys: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let m = PyModule::new(py, "mrly.num")?;
        m.setattr("__doc__", "The integers: primes, divisors, series, sieves, lattices and spectra.\n\nPlain numbers and byte grids go in; counts, fractions, rates and measurements come out.\nEvery answer is exact where the integers allow and a stated approximation where they do not.\n\n# Files\n\n- `apollonian`: an integral circle packing, the Ford circles on its line, the Farey stack beneath.\n- `blend`: term ops on sequences, the exact recurrence behind one, its growth rate.\n- `boolean`: a truth table's Walsh spectrum, nonlinearity, balance and avalanche.\n- `chladni`: the figures of a ringing square plate, their ladder by pitch, the sand that runs to their lines and the lines themselves.\n- `factor`: factorizations, divisors, totients, radicals, Mobius values, gcd and lcm.\n- `fft`: the fast Fourier transform in one and two dimensions.\n- `gauss`: the Gaussian and the Eisenstein integers, their classes, windows and shells.\n- `kummer`: the Kummer sets `{k : p does not divide C(a k, b k)}` read by their carry automaton, their mass and the witness that they are no digit design.\n- `lattice`: coprime pairs, the Farey nodes of a window, the constant a dimension recovers.\n- `morse`: the Thue-Morse world: the digit rule, the substitution, the lifts and the runs.\n- `prime`: the sieve and its readings, ranks, gaps, counts and the shapes a number makes.\n- `readings`: a digit-parity design read along a polynomial or the primes, its battery, its near-heads and the major-arc model of its bias along the squares.\n- `series`: the classic sequences, zeta and its cousins, the partials walking to pi, e and gamma.\n- `sieve`: the Wallis sieve and its kin as schedule words, with their rasters and limits.\n- `spiral`: the whole numbers wound on the square and the hexagonal lattice, marked and read.\n- `zeta`: zeta on the critical line, its zeros, the prime staircase they rebuild.\n\n# Doors\n\n- The divisor arithmetic: `gcd`, `factorial`, `divisors`, `mobius`.\n- The primality test: `is_prime`.\n- The zeta value above one: `zeta`.")?;
        let names: Vec<&str> = vec![];
        m.add("__all__", names)?;
        apollonian::init(py, &m, sys)?;
        blend::init(py, &m, sys)?;
        boolean::init(py, &m, sys)?;
        chladni::init(py, &m, sys)?;
        factor::init(py, &m, sys)?;
        fft::init(py, &m, sys)?;
        gauss::init(py, &m, sys)?;
        kummer::init(py, &m, sys)?;
        lattice::init(py, &m, sys)?;
        morse::init(py, &m, sys)?;
        prime::init(py, &m, sys)?;
        readings::init(py, &m, sys)?;
        series::init(py, &m, sys)?;
        sieve::init(py, &m, sys)?;
        spiral::init(py, &m, sys)?;
        zeta::init(py, &m, sys)?;
        parent.add("num", &m)?;
        sys.set_item("mrly.num._num.num", &m)?;
        Ok(())
    }
}

pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = py
        .import("sys")?
        .getattr("modules")?
        .cast_into::<PyDict>()?;
    num::init(py, root, &sys)?;
    Ok(())
}
