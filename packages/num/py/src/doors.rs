#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub mod num {
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    pub mod apollonian {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Circle", module = "mrly.num.apollonian", from_py_object)]
        #[derive(Clone)]
        pub struct Circle(pub mrly_num::apollonian::Circle);

        #[pymethods]
        impl Circle {
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
            #[pyo3(name = "centre", signature = ())]
            pub fn centre<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::apollonian::Circle::centre(self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "is_line", signature = ())]
            pub fn is_line<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::apollonian::Circle::is_line(self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "radius", signature = ())]
            pub fn radius<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::apollonian::Circle::radius(self.0);
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
        #[pyo3(name = "flat", signature = (name, cap))]
        pub fn flat<'py>(py: Python<'py>, name: &str, cap: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::apollonian::flat(name, cap);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "form", signature = (u, v))]
        pub fn form<'py>(py: Python<'py>, u: [i64; 4], v: [i64; 4]) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::apollonian::form(u, v);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "grow", signature = (name, cap))]
        pub fn grow<'py>(py: Python<'py>, name: &str, cap: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::apollonian::grow(name, cap);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

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

    pub mod blend {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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

        #[pyfunction]
        #[pyo3(name = "characteristic", signature = (coefficients))]
        pub fn characteristic<'py>(
            py: Python<'py>,
            coefficients: Vec<(i128, i128)>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::characteristic(&coefficients);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "delta", signature = (a))]
        pub fn delta<'py>(py: Python<'py>, a: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::delta(&a);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "growth", signature = (coefficients))]
        pub fn growth<'py>(
            py: Python<'py>,
            coefficients: Vec<(i128, i128)>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::growth(&coefficients);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "recurrence", signature = (terms))]
        pub fn recurrence<'py>(py: Python<'py>, terms: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::recurrence(&terms);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "sigma", signature = (a))]
        pub fn sigma<'py>(py: Python<'py>, a: Vec<i128>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::blend::sigma(&a);
            (ok(out)?).into_bound_py_any(py)
        }

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

    pub mod boolean {
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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

        #[pyfunction]
        #[pyo3(name = "sac", signature = (code, n))]
        pub fn sac<'py>(py: Python<'py>, code: u128, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::boolean::sac(code, n);
            (out).into_bound_py_any(py)
        }

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

    pub mod chladni {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "ladder", signature = (cap))]
        pub fn ladder<'py>(py: Python<'py>, cap: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::chladni::ladder(cap);
            ((ok(out)?).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

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

    pub mod factor {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "aliquot", signature = (number))]
        pub fn aliquot<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::aliquot(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "coprime", signature = (a, b))]
        pub fn coprime<'py>(py: Python<'py>, a: usize, b: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::coprime(a, b);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "divisors", signature = (number))]
        pub fn divisors<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::divisors(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "factorial", signature = (number))]
        pub fn factorial<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::factorial(number);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "factorize", signature = (number))]
        pub fn factorize<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::factorize(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "factorize_wide", signature = (number))]
        pub fn factorize_wide<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::factorize_wide(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "gcd", signature = (a, b))]
        pub fn gcd<'py>(py: Python<'py>, a: u128, b: u128) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::gcd(a, b);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "lcm", signature = (a, b))]
        pub fn lcm<'py>(py: Python<'py>, a: usize, b: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::lcm(a, b);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mobius", signature = (number))]
        pub fn mobius<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::mobius(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mobius_sieve", signature = (limit))]
        pub fn mobius_sieve<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::mobius_sieve(limit);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "radical", signature = (number))]
        pub fn radical<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::radical(number);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "squarefree", signature = (number))]
        pub fn squarefree<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::squarefree(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "totient", signature = (number))]
        pub fn totient<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::totient(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "totients", signature = (n))]
        pub fn totients<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::factor::totients(n);
            (out).into_bound_py_any(py)
        }

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

    pub mod fft {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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

        #[pyfunction]
        #[pyo3(name = "peak_ring", signature = (profile))]
        pub fn peak_ring<'py>(py: Python<'py>, profile: Vec<f64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::fft::peak_ring(&profile);
            (out).into_bound_py_any(py)
        }

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

    pub mod gauss {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Window", module = "mrly.num.gauss", from_py_object)]
        #[derive(Clone)]
        pub struct Window(pub mrly_num::gauss::Window);

        #[pymethods]
        impl Window {
            #[new]
            #[pyo3(signature = (ring, radius))]
            pub fn __new__(ring: PySerde<mrly_num::gauss::Ring>, radius: u64) -> PyResult<Self> {
                let ring = ring.0;
                let out = mrly_num::gauss::Window::new(ring, radius);
                Ok(Self(out))
            }
            #[pyo3(name = "census", signature = ())]
            pub fn census<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::census(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
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
            #[pyo3(name = "points", signature = ())]
            pub fn points<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::points(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "radius", signature = ())]
            pub fn radius<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::radius(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "ring", signature = ())]
            pub fn ring<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Window::ring(&self.0);
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Class", module = "mrly.num.gauss", skip_from_py_object)]
        pub struct Class;

        #[pymethods]
        impl Class {
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

        #[pyclass(name = "Ring", module = "mrly.num.gauss", skip_from_py_object)]
        pub struct Ring;

        #[pymethods]
        impl Ring {
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
            #[staticmethod]
            #[pyo3(name = "named", signature = (name))]
            pub fn named<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::gauss::Ring::named(name);
                ((out).map(PySerde)).into_bound_py_any(py)
            }
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

    pub mod kummer {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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

    pub mod lattice {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "coprime_pairs", signature = (n))]
        pub fn coprime_pairs<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::coprime_pairs(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "farey", signature = (order))]
        pub fn farey<'py>(py: Python<'py>, order: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::farey(order);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "grid", signature = (n))]
        pub fn grid<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::grid(n);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "new_nodes", signature = (n))]
        pub fn new_nodes<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::new_nodes(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pi_estimate", signature = (n))]
        pub fn pi_estimate<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::pi_estimate(n);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "visible_density", signature = (dimension))]
        pub fn visible_density<'py>(
            py: Python<'py>,
            dimension: u32,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::visible_density(dimension);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "zeta_factor", signature = (dimension))]
        pub fn zeta_factor<'py>(py: Python<'py>, dimension: u32) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::lattice::zeta_factor(dimension);
            (out).into_bound_py_any(py)
        }

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

    pub mod morse {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Lift", module = "mrly.num.morse", skip_from_py_object)]
        pub struct Lift;

        #[pymethods]
        impl Lift {
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::morse::Lift::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
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

        #[pyfunction]
        #[pyo3(name = "boundary", signature = (word))]
        pub fn boundary<'py>(py: Python<'py>, word: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::boundary(&word);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "digits", signature = (length))]
        pub fn digits<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::digits(length);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "doubling", signature = (length))]
        pub fn doubling<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::doubling(length);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "faults", signature = (a, b))]
        pub fn faults<'py>(py: Python<'py>, a: Vec<u8>, b: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::faults(&a, &b);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "letter", signature = (place))]
        pub fn letter<'py>(py: Python<'py>, place: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::letter(place);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "mask", signature = (a, b))]
        pub fn mask<'py>(py: Python<'py>, a: Vec<u8>, b: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::mask(&a, &b);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "runs", signature = (word))]
        pub fn runs<'py>(py: Python<'py>, word: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::runs(&word);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "signs", signature = (types))]
        pub fn signs<'py>(py: Python<'py>, types: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::signs(&types);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "stage", signature = (rounds))]
        pub fn stage<'py>(py: Python<'py>, rounds: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::stage(rounds);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "substitution", signature = (length))]
        pub fn substitution<'py>(py: Python<'py>, length: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::morse::substitution(length);
            (out).into_bound_py_any(py)
        }

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

    pub mod prime {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Sieve", module = "mrly.num.prime", from_py_object)]
        #[derive(Clone)]
        pub struct Sieve(pub mrly_num::prime::Sieve);

        #[pymethods]
        impl Sieve {
            #[new]
            #[pyo3(signature = (limit))]
            pub fn __new__(limit: usize) -> PyResult<Self> {
                let out = mrly_num::prime::Sieve::new(limit);
                Ok(Self(out))
            }
            #[pyo3(name = "count", signature = ())]
            pub fn count<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::count(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "done", signature = ())]
            pub fn done<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::done(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "finish", signature = ())]
            pub fn finish<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                mrly_num::prime::Sieve::finish(&mut self.0);
                ().into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (limit))]
            pub fn new_<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::new(limit);
                (crate::doors::num::prime::Sieve(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "rank", signature = ())]
            pub fn rank<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::rank(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "step", signature = ())]
            pub fn step<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::step(&mut self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "struck", signature = ())]
            pub fn struck<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::struck(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "types", signature = ())]
            pub fn types<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::prime::Sieve::types(&self.0);
                ((out).to_vec()).into_bound_py_any(py)
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
        #[pyo3(name = "chart", signature = (top, bins))]
        pub fn chart<'py>(py: Python<'py>, top: usize, bins: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::chart(top, bins);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "flags", signature = (limit))]
        pub fn flags<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::flags(limit);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "goldbach", signature = (number))]
        pub fn goldbach<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::goldbach(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "goldbach_record", signature = (top))]
        pub fn goldbach_record<'py>(py: Python<'py>, top: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::goldbach_record(top);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "is_prime", signature = (number))]
        pub fn is_prime<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::is_prime(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "pile", signature = (number))]
        pub fn pile<'py>(py: Python<'py>, number: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::pile(number);
            (PySerde(out)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "prime_count", signature = (n))]
        pub fn prime_count<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::prime_count(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "prime_from", signature = (number))]
        pub fn prime_from<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::prime_from(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "primes", signature = (limit))]
        pub fn primes<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::primes(limit);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "rectangles", signature = (number))]
        pub fn rectangles<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::rectangles(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "splits", signature = (number))]
        pub fn splits<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::splits(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "squares", signature = (number))]
        pub fn squares<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::prime::squares(number);
            (out).into_bound_py_any(py)
        }

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

    pub mod readings {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Design", module = "mrly.num.readings", skip_from_py_object)]
        pub struct Design;

        #[pymethods]
        impl Design {
            #[staticmethod]
            #[pyo3(name = "digits", signature = (base, mask))]
            pub fn digits<'py>(py: Python<'py>, base: u8, mask: u8) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::digits(base, mask);
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "rudin", signature = ())]
            pub fn rudin<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::rudin();
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "ternary", signature = ())]
            pub fn ternary<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::ternary();
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "thue", signature = ())]
            pub fn thue<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Design::thue();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyclass(name = "Reading", module = "mrly.num.readings", skip_from_py_object)]
        pub struct Reading;

        #[pymethods]
        impl Reading {
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::readings::Reading::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
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

        #[pyfunction]
        #[pyo3(name = "battery", signature = (stream))]
        pub fn battery<'py>(py: Python<'py>, stream: Vec<i8>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::readings::battery(&stream);
            (PySerde(out)).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "designs", signature = ())]
        pub fn designs<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::readings::designs();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

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

    pub mod series {
        use crate::hand::ok;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyfunction]
        #[pyo3(name = "basel", signature = (n))]
        pub fn basel<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::basel(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "bernoulli", signature = (count))]
        pub fn bernoulli<'py>(py: Python<'py>, count: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::bernoulli(count);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "beta", signature = (s, terms))]
        pub fn beta<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::beta(s, terms);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "binary", signature = (limit))]
        pub fn binary<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::binary(limit);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "catalan", signature = (limit))]
        pub fn catalan<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::catalan(limit);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "chi3", signature = (number))]
        pub fn chi3<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::chi3(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "chi4", signature = (number))]
        pub fn chi4<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::chi4(number);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "chi8", signature = (number))]
        pub fn chi8<'py>(py: Python<'py>, number: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::chi8(number);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "e_partial", signature = (n))]
        pub fn e_partial<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::e_partial(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "euler_gamma_partial", signature = (n))]
        pub fn euler_gamma_partial<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::euler_gamma_partial(n);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "evens", signature = (limit))]
        pub fn evens<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::evens(limit);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "fibonacci", signature = (limit))]
        pub fn fibonacci<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::fibonacci(limit);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "harmonic", signature = (terms))]
        pub fn harmonic<'py>(py: Python<'py>, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::harmonic(terms);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "lambda_", signature = (s, terms))]
        pub fn lambda_<'py>(py: Python<'py>, s: f64, terms: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::lambda(s, terms);
            (ok(out)?).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "leibniz", signature = (n))]
        pub fn leibniz<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::leibniz(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "li", signature = (x))]
        pub fn li<'py>(py: Python<'py>, x: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::li(x);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "mertens", signature = (n))]
        pub fn mertens<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::mertens(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "odds", signature = (limit))]
        pub fn odds<'py>(py: Python<'py>, limit: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::odds(limit);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "wallis_half_pi", signature = (n))]
        pub fn wallis_half_pi<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::wallis_half_pi(n);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "wallis_quarter_pi", signature = (factors))]
        pub fn wallis_quarter_pi<'py>(
            py: Python<'py>,
            factors: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::series::wallis_quarter_pi(factors);
            (out).into_bound_py_any(py)
        }

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

    pub mod sieve {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

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

        #[pyfunction]
        #[pyo3(name = "odd_word", signature = (levels))]
        pub fn odd_word<'py>(py: Python<'py>, levels: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::odd_word(levels);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "raster", signature = (word))]
        pub fn raster<'py>(py: Python<'py>, word: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::raster(&word);
            (ok(out)?).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "row_law", signature = (profile))]
        pub fn row_law<'py>(py: Python<'py>, profile: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::row_law(&profile);
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "side", signature = (word))]
        pub fn side<'py>(py: Python<'py>, word: Vec<u64>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::sieve::side(&word);
            (ok(out)?).into_bound_py_any(py)
        }

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

    pub mod spiral {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Growth", module = "mrly.num.spiral", skip_from_py_object)]
        pub struct Growth;

        #[pymethods]
        impl Growth {
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::spiral::Growth::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        #[pyclass(name = "Lattice", module = "mrly.num.spiral", skip_from_py_object)]
        pub struct Lattice;

        #[pymethods]
        impl Lattice {
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::spiral::Lattice::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
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

        #[pyclass(name = "Mark", module = "mrly.num.spiral", skip_from_py_object)]
        pub struct Mark;

        #[pymethods]
        impl Mark {
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::spiral::Mark::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

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

        #[pyfunction]
        #[pyo3(name = "level_of", signature = (n, base))]
        pub fn level_of<'py>(py: Python<'py>, n: u64, base: u64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::spiral::level_of(n, base);
            (out).into_bound_py_any(py)
        }

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

    pub mod zeta {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Line", module = "mrly.num.zeta", skip_from_py_object)]
        pub struct Line(pub mrly_num::zeta::Line);

        #[pymethods]
        impl Line {
            #[new]
            #[pyo3(signature = ())]
            pub fn __new__() -> PyResult<Self> {
                let out = mrly_num::zeta::Line::new();
                Ok(Self(out))
            }
            #[pyo3(name = "count", signature = (t))]
            pub fn count<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::count(&self.0, t);
                (out).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::default();
                (crate::doors::num::zeta::Line(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "exact", signature = (t))]
            pub fn exact<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::exact(&self.0, t);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "gram", signature = (n))]
            pub fn gram<'py>(&self, py: Python<'py>, n: i64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::gram(&self.0, n);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "maclaurin", signature = (t))]
            pub fn maclaurin<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::maclaurin(&self.0, t);
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = ())]
            pub fn new_<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::new();
                (crate::doors::num::zeta::Line(out)).into_bound_py_any(py)
            }
            #[pyo3(name = "novelty_coefficients", signature = (gammas))]
            pub fn novelty_coefficients<'py>(
                &self,
                py: Python<'py>,
                gammas: Vec<f64>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::novelty_coefficients(&self.0, &gammas);
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
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
            #[pyo3(name = "point", signature = (t))]
            pub fn point<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::point(&self.0, t);
                ({
                    let t = out;
                    (PySerde(t.0), t.1)
                })
                .into_bound_py_any(py)
            }
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
            #[pyo3(name = "siegel", signature = (t))]
            pub fn siegel<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::siegel(&self.0, t);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "theta", signature = (t))]
            pub fn theta<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::theta(&self.0, t);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "z", signature = (t))]
            pub fn z<'py>(&self, py: Python<'py>, t: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::z(&self.0, t);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "zeros", signature = (count))]
            pub fn zeros<'py>(&self, py: Python<'py>, count: usize) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Line::zeros(&self.0, count);
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

        #[pyclass(name = "Complex", module = "mrly.num.zeta", skip_from_py_object)]
        pub struct Complex;

        #[pymethods]
        impl Complex {
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
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Complex::default();
                (PySerde(out)).into_bound_py_any(py)
            }
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
            #[staticmethod]
            #[pyo3(name = "new", signature = (re, im))]
            pub fn new_<'py>(py: Python<'py>, re: f64, im: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Complex::new(re, im);
                (PySerde(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "turn", signature = (angle))]
            pub fn turn<'py>(py: Python<'py>, angle: f64) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_num::zeta::Complex::turn(angle);
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "bump", signature = (u))]
        pub fn bump<'py>(py: Python<'py>, u: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::bump(u);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "corrections", signature = (p))]
        pub fn corrections<'py>(py: Python<'py>, p: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::corrections(p);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "kernel", signature = (p))]
        pub fn kernel<'py>(py: Python<'py>, p: f64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::kernel(p);
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "novelty_main", signature = ())]
        pub fn novelty_main<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::novelty_main();
            (out).into_bound_py_any(py)
        }

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

        #[pyfunction]
        #[pyo3(name = "psi_stair", signature = (x))]
        pub fn psi_stair<'py>(py: Python<'py>, x: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_num::zeta::psi_stair(x);
            (out).into_bound_py_any(py)
        }

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
