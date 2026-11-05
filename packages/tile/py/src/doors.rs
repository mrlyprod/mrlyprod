#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub mod tile {
    use crate::hand::{PyImage, PySerde, PyTensor, ok};
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    pub mod build {
        use crate::hand::{PyCell6d, PyCellNd, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Config2d", module = "mrly.tile.build", skip_from_py_object)]
        pub struct Config2d;

        #[pymethods]
        impl Config2d {
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::build::Config2d::default();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyclass(name = "Config3d", module = "mrly.tile.build", skip_from_py_object)]
        pub struct Config3d;

        #[pymethods]
        impl Config3d {
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::build::Config3d::default();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "build_2d", signature = (tile))]
        pub fn build_2d<'py>(
            py: Python<'py>,
            tile: PyRef<'_, crate::doors::tile::Tile>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::build::build_2d(&tile.0);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "build_3d", signature = (tile))]
        pub fn build_3d<'py>(
            py: Python<'py>,
            tile: PyRef<'_, crate::doors::tile::Tile>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::build::build_3d(&tile.0);
            (PyCellNd(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "build_6d", signature = (hex))]
        pub fn build_6d<'py>(
            py: Python<'py>,
            hex: PySerde<mrly_tile::build::HexTile>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let hex = hex.0;
            let out = mrly_tile::build::build_6d(&hex);
            (PyCell6d(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "create_2d", signature = (config, rng))]
        pub fn create_2d<'py>(
            py: Python<'py>,
            config: &Bound<'_, PyAny>,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let config = crate::hand::serde_from_py(config)?;
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::build::create_2d(&config, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (crate::doors::tile::Tile(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "create_3d", signature = (config, rng))]
        pub fn create_3d<'py>(
            py: Python<'py>,
            config: &Bound<'_, PyAny>,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let config = crate::hand::serde_from_py(config)?;
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::build::create_3d(&config, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (crate::doors::tile::Tile(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "create_6d", signature = (config, rng))]
        pub fn create_6d<'py>(
            py: Python<'py>,
            config: &Bound<'_, PyAny>,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let config = crate::hand::serde_from_py(config)?;
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::build::create_6d(&config, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "random_tile_2d", signature = (max_size, rng))]
        pub fn random_tile_2d<'py>(
            py: Python<'py>,
            max_size: usize,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::build::random_tile_2d(max_size, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (crate::doors::tile::Tile(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "random_tile_3d", signature = (max_size, rng))]
        pub fn random_tile_3d<'py>(
            py: Python<'py>,
            max_size: usize,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::build::random_tile_3d(max_size, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (crate::doors::tile::Tile(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "random_tile_6d", signature = (max_size, rng))]
        pub fn random_tile_6d<'py>(
            py: Python<'py>,
            max_size: usize,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::build::random_tile_6d(max_size, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (PySerde(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.tile.build")?;
            m.add_class::<Config2d>()?;
            m.add_class::<Config3d>()?;
            m.add_function(wrap_pyfunction!(build_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(build_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(build_6d, &m)?)?;
            m.add_function(wrap_pyfunction!(create_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(create_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(create_6d, &m)?)?;
            m.add_function(wrap_pyfunction!(random_tile_2d, &m)?)?;
            m.add_function(wrap_pyfunction!(random_tile_3d, &m)?)?;
            m.add_function(wrap_pyfunction!(random_tile_6d, &m)?)?;
            let names: Vec<&str> = vec![
                "build_2d",
                "build_3d",
                "build_6d",
                "create_2d",
                "create_3d",
                "create_6d",
                "random_tile_2d",
                "random_tile_3d",
                "random_tile_6d",
                "Config2d",
                "Config3d",
            ];
            m.add("__all__", names)?;
            parent.add("build", &m)?;
            sys.set_item("mrly.tile._tile.tile.build", &m)?;
            Ok(())
        }
    }

    pub mod name {
        use crate::hand::{PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Tile", module = "mrly.tile.name", from_py_object)]
        #[derive(Clone)]
        pub struct Tile(pub mrly_tile::name::Tile);

        #[pymethods]
        impl Tile {
            #[getter]
            #[pyo3(name = "code")]
            pub fn code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.code;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "code")]
            pub fn set_code(&mut self, value: Option<u128>) -> PyResult<()> {
                self.0.code = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "special")]
            pub fn special<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.special;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "special")]
            pub fn set_special(&mut self, value: Option<u128>) -> PyResult<()> {
                self.0.special = value;
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
            #[pyo3(name = "mosaic")]
            pub fn mosaic<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.mosaic.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "mosaic")]
            pub fn set_mosaic(&mut self, value: Vec<u128>) -> PyResult<()> {
                self.0.mosaic = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "factor")]
            pub fn factor<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.factor;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "factor")]
            pub fn set_factor(&mut self, value: Option<usize>) -> PyResult<()> {
                self.0.factor = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "side")]
            pub fn side<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.side.clone();
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "side")]
            pub fn set_side(&mut self, value: PySerde<mrly_tile::name::Slots>) -> PyResult<()> {
                let value = value.0;
                self.0.side = value;
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
            pub fn set_level(&mut self, value: Option<usize>) -> PyResult<()> {
                self.0.level = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "turn")]
            pub fn turn<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.turn.clone();
                (PySerde(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "turn")]
            pub fn set_turn(&mut self, value: PySerde<mrly_tile::name::Slots>) -> PyResult<()> {
                let value = value.0;
                self.0.turn = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "flip")]
            pub fn flip<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.flip;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "flip")]
            pub fn set_flip(&mut self, value: bool) -> PyResult<()> {
                self.0.flip = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "invert")]
            pub fn invert<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.invert;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "invert")]
            pub fn set_invert(&mut self, value: bool) -> PyResult<()> {
                self.0.invert = value;
                Ok(())
            }
            #[pyo3(name = "checked", signature = ())]
            pub fn checked<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out =
                    <mrly_tile::name::Tile as mrly_core::name::Named>::checked(self.0.clone());
                (crate::doors::tile::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_file", signature = (text))]
            pub fn from_file<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::from_file(text);
                (crate::doors::tile::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_json", signature = (text))]
            pub fn from_json<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::from_json(text);
                (crate::doors::tile::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "from_url", signature = (text))]
            pub fn from_url<'py>(py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::from_url(text);
                (crate::doors::tile::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            #[staticmethod]
            #[pyo3(name = "of", signature = (recipe))]
            pub fn of<'py>(
                py: Python<'py>,
                recipe: PyRef<'_, crate::doors::tile::Tile>,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::name::Tile::of(&recipe.0);
                (crate::doors::tile::name::Tile(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "recipe", signature = ())]
            pub fn recipe<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::name::Tile::recipe(&self.0);
                (crate::doors::tile::Tile(ok(out)?)).into_bound_py_any(py)
            }
            #[pyo3(name = "to_file", signature = ())]
            pub fn to_file<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::to_file(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_id", signature = ())]
            pub fn to_id<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::to_id(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_json", signature = ())]
            pub fn to_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::to_json(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "to_prose", signature = ())]
            pub fn to_prose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::to_prose(&self.0);
                (ok(out)?).into_bound_py_any(py)
            }
            #[pyo3(name = "to_url", signature = ())]
            pub fn to_url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = <mrly_tile::name::Tile as mrly_core::name::Named>::to_url(&self.0);
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

        #[pyclass(name = "Slots", module = "mrly.tile.name", skip_from_py_object)]
        pub struct Slots;

        #[pymethods]
        impl Slots {
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::name::Slots::default();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.tile.name")?;
            m.add_class::<Tile>()?;
            m.add_class::<Slots>()?;
            let names: Vec<&str> = vec!["Tile", "Slots"];
            m.add("__all__", names)?;
            parent.add("name", &m)?;
            sys.set_item("mrly.tile._tile.tile.name", &m)?;
            Ok(())
        }
    }

    pub mod recipe {
        use crate::hand::PySerde;
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "Design", module = "mrly.tile.recipe", skip_from_py_object)]
        pub struct Design;

        #[pymethods]
        impl Design {
            #[staticmethod]
            #[pyo3(name = "all", signature = ())]
            pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::recipe::Design::all();
                ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "classics", signature = (dimension))]
        pub fn classics<'py>(py: Python<'py>, dimension: usize) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::recipe::classics(dimension);
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "generals", signature = (min_size, max_size, parity))]
        pub fn generals<'py>(
            py: Python<'py>,
            min_size: usize,
            max_size: usize,
            parity: PySerde<mrly_tile::Parity>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrly_tile::recipe::generals(min_size, max_size, parity);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "nestings", signature = (min_size, max_size, parity))]
        pub fn nestings<'py>(
            py: Python<'py>,
            min_size: usize,
            max_size: usize,
            parity: PySerde<mrly_tile::Parity>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrly_tile::recipe::nestings(min_size, max_size, parity);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "powers", signature = (min_size, max_size, parity))]
        pub fn powers<'py>(
            py: Python<'py>,
            min_size: usize,
            max_size: usize,
            parity: PySerde<mrly_tile::Parity>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrly_tile::recipe::powers(min_size, max_size, parity);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "products", signature = (min_size, max_size, count, parity))]
        pub fn products<'py>(
            py: Python<'py>,
            min_size: usize,
            max_size: usize,
            count: usize,
            parity: PySerde<mrly_tile::Parity>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrly_tile::recipe::products(min_size, max_size, count, parity);
            (out).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "size", signature = (number, level))]
        pub fn size<'py>(py: Python<'py>, number: i64, level: i64) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::recipe::size(number, level);
            (out).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.tile.recipe")?;
            m.add_class::<Design>()?;
            m.add_function(wrap_pyfunction!(classics, &m)?)?;
            m.add_function(wrap_pyfunction!(generals, &m)?)?;
            m.add_function(wrap_pyfunction!(nestings, &m)?)?;
            m.add_function(wrap_pyfunction!(powers, &m)?)?;
            m.add_function(wrap_pyfunction!(products, &m)?)?;
            m.add_function(wrap_pyfunction!(size, &m)?)?;
            m.add(
                "CLASSICS_2D",
                (mrly_tile::recipe::CLASSICS_2D)
                    .into_iter()
                    .map(PySerde)
                    .collect::<Vec<_>>(),
            )?;
            m.add(
                "CLASSICS_3D",
                (mrly_tile::recipe::CLASSICS_3D)
                    .into_iter()
                    .map(PySerde)
                    .collect::<Vec<_>>(),
            )?;
            m.add("MAX_LEVEL", mrly_tile::recipe::MAX_LEVEL)?;
            m.add("MAX_SIDE", mrly_tile::recipe::MAX_SIDE)?;
            m.add("MAX_SLOTS", mrly_tile::recipe::MAX_SLOTS)?;
            m.add("MIN_SIDE", mrly_tile::recipe::MIN_SIDE)?;
            let names: Vec<&str> = vec![
                "classics",
                "generals",
                "nestings",
                "powers",
                "products",
                "size",
                "Design",
                "CLASSICS_2D",
                "CLASSICS_3D",
                "MAX_LEVEL",
                "MAX_SIDE",
                "MAX_SLOTS",
                "MIN_SIDE",
            ];
            m.add("__all__", names)?;
            parent.add("recipe", &m)?;
            sys.set_item("mrly.tile._tile.tile.recipe", &m)?;
            Ok(())
        }
    }

    pub mod variation {
        use crate::hand::{PyCell2d, PyCellNd, PyImage, PySerde, ok};
        use pyo3::IntoPyObjectExt;
        use pyo3::prelude::*;
        use pyo3::types::PyDict;

        #[pyclass(name = "File", module = "mrly.tile.variation", from_py_object)]
        #[derive(Clone)]
        pub struct File(pub mrly_tile::variation::File);

        #[pymethods]
        impl File {
            #[new]
            #[pyo3(signature = (width, height))]
            pub fn __new__(width: usize, height: usize) -> PyResult<Self> {
                let out = mrly_tile::variation::File::new(width, height);
                Ok(Self(out))
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
            #[pyo3(name = "height")]
            pub fn height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.height;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "height")]
            pub fn set_height(&mut self, value: usize) -> PyResult<()> {
                self.0.height = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "image")]
            pub fn image<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.image.clone();
                ((value).map(PyImage)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "image")]
            pub fn set_image(&mut self, value: Option<PyImage>) -> PyResult<()> {
                let value = value.map(|x| x.0);
                self.0.image = value;
                Ok(())
            }
            #[staticmethod]
            #[pyo3(name = "new", signature = (width, height))]
            pub fn new_<'py>(
                py: Python<'py>,
                width: usize,
                height: usize,
            ) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::variation::File::new(width, height);
                (crate::doors::tile::variation::File(out)).into_bound_py_any(py)
            }
            #[staticmethod]
            pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
                Ok(Self(crate::hand::serde_from_py(data)?))
            }
            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                crate::hand::serde_into_py(py, &self.0)
            }
        }

        #[pyclass(name = "Variation", module = "mrly.tile.variation", from_py_object)]
        #[derive(Clone)]
        pub struct Variation(pub mrly_tile::variation::Variation);

        #[pymethods]
        impl Variation {
            #[getter]
            #[pyo3(name = "key")]
            pub fn key<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.key.clone();
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "key")]
            pub fn set_key(&mut self, value: String) -> PyResult<()> {
                self.0.key = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "seed")]
            pub fn seed<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.seed;
                (value).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "seed")]
            pub fn set_seed(&mut self, value: u64) -> PyResult<()> {
                self.0.seed = value;
                Ok(())
            }
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
            #[getter]
            #[pyo3(name = "primaries")]
            pub fn primaries<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.primaries.clone();
                ((value).map(|x| (x).into_iter().map(PySerde).collect::<Vec<_>>()))
                    .into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "primaries")]
            pub fn set_primaries(
                &mut self,
                value: Option<Vec<PySerde<mrly_core::paint::Ink>>>,
            ) -> PyResult<()> {
                let value = value.map(|x| x.into_iter().map(|x| x.0).collect::<Vec<_>>());
                self.0.primaries = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "tile")]
            pub fn tile<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.tile.clone();
                (crate::doors::tile::Tile(value)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "tile")]
            pub fn set_tile(&mut self, value: crate::doors::tile::Tile) -> PyResult<()> {
                let value = value.0;
                self.0.tile = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "mask")]
            pub fn mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.mask.clone();
                ((value).map(crate::doors::tile::Tile)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "mask")]
            pub fn set_mask(&mut self, value: Option<crate::doors::tile::Tile>) -> PyResult<()> {
                let value = value.map(|x| x.0);
                self.0.mask = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "base")]
            pub fn base<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.base.clone();
                ((value).map(PyCellNd)).into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "base")]
            pub fn set_base(&mut self, value: Option<PyCell2d>) -> PyResult<()> {
                let value = value.map(|x| x.0);
                self.0.base = value;
                Ok(())
            }
            #[getter]
            #[pyo3(name = "files")]
            pub fn files<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let value = self.0.files.clone();
                ((value)
                    .into_iter()
                    .map(crate::doors::tile::variation::File)
                    .collect::<Vec<_>>())
                .into_bound_py_any(py)
            }
            #[setter]
            #[pyo3(name = "files")]
            pub fn set_files(
                &mut self,
                value: Vec<crate::doors::tile::variation::File>,
            ) -> PyResult<()> {
                let value = value.into_iter().map(|x| x.0).collect::<Vec<_>>();
                self.0.files = value;
                Ok(())
            }
            #[pyo3(name = "is_cover", signature = ())]
            pub fn is_cover<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::variation::Variation::is_cover(&self.0);
                (out).into_bound_py_any(py)
            }
            #[pyo3(name = "is_prime", signature = ())]
            pub fn is_prime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::variation::Variation::is_prime(&self.0);
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

        #[pyclass(name = "Config", module = "mrly.tile.variation", skip_from_py_object)]
        pub struct Config;

        #[pymethods]
        impl Config {
            #[staticmethod]
            #[pyo3(name = "default", signature = ())]
            pub fn default<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let out = mrly_tile::variation::Config::default();
                (PySerde(out)).into_bound_py_any(py)
            }
        }

        #[pyfunction]
        #[pyo3(name = "create", signature = (config, rng))]
        pub fn create<'py>(
            py: Python<'py>,
            config: PySerde<mrly_tile::variation::Config>,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let config = config.0;
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::variation::create(&config, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (crate::doors::tile::variation::Variation(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "generate", signature = (variation, config, rng))]
        pub fn generate<'py>(
            py: Python<'py>,
            variation: crate::doors::tile::variation::Variation,
            config: PySerde<mrly_tile::variation::Config>,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let variation = variation.0;
            let config = config.0;
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::variation::generate(variation, &config, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (crate::doors::tile::variation::Variation(ok(out)?)).into_bound_py_any(py)
        }

        #[pyfunction]
        #[pyo3(name = "render", signature = (variation, scale, rng))]
        pub fn render<'py>(
            py: Python<'py>,
            variation: crate::doors::tile::variation::Variation,
            scale: usize,
            rng: &Bound<'_, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let variation = variation.0;
            let mut rng_owned = crate::hand::rng_from_py(rng)?;
            let out = mrly_tile::variation::render(variation, scale, &mut rng_owned);
            crate::hand::rng_write_back(rng, &rng_owned)?;
            (crate::doors::tile::variation::Variation(ok(out)?)).into_bound_py_any(py)
        }

        pub fn init(
            py: Python<'_>,
            parent: &Bound<'_, PyModule>,
            sys: &Bound<'_, PyDict>,
        ) -> PyResult<()> {
            let m = PyModule::new(py, "mrly.tile.variation")?;
            m.add_class::<File>()?;
            m.add_class::<Variation>()?;
            m.add_class::<Config>()?;
            m.add_function(wrap_pyfunction!(create, &m)?)?;
            m.add_function(wrap_pyfunction!(generate, &m)?)?;
            m.add_function(wrap_pyfunction!(render, &m)?)?;
            let names: Vec<&str> = vec![
                "create",
                "generate",
                "render",
                "File",
                "Variation",
                "Config",
            ];
            m.add("__all__", names)?;
            parent.add("variation", &m)?;
            sys.set_item("mrly.tile._tile.tile.variation", &m)?;
            Ok(())
        }
    }

    #[pyclass(name = "Tile", module = "mrly.tile", from_py_object)]
    #[derive(Clone)]
    pub struct Tile(pub mrly_tile::Tile);

    #[pymethods]
    impl Tile {
        #[new]
        #[pyo3(signature = (group))]
        pub fn __new__(group: PySerde<mrly_tile::Group>) -> PyResult<Self> {
            let group = group.0;
            let out = mrly_tile::Tile::new(group);
            Ok(Self(out))
        }
        #[getter]
        #[pyo3(name = "group")]
        pub fn group<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.group;
            (PySerde(value)).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "group")]
        pub fn set_group(&mut self, value: PySerde<mrly_tile::Group>) -> PyResult<()> {
            let value = value.0;
            self.0.group = value;
            Ok(())
        }
        #[getter]
        #[pyo3(name = "factor")]
        pub fn factor<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.factor;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "factor")]
        pub fn set_factor(&mut self, value: usize) -> PyResult<()> {
            self.0.factor = value;
            Ok(())
        }
        #[getter]
        #[pyo3(name = "sources")]
        pub fn sources<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.sources.clone();
            ((value).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "sources")]
        pub fn set_sources(
            &mut self,
            value: Vec<PySerde<mrly_tile::recipe::Source>>,
        ) -> PyResult<()> {
            let value = value.into_iter().map(|x| x.0).collect::<Vec<_>>();
            self.0.sources = value;
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
        #[pyo3(name = "levels")]
        pub fn levels<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.levels.clone();
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "levels")]
        pub fn set_levels(&mut self, value: Vec<usize>) -> PyResult<()> {
            self.0.levels = value;
            Ok(())
        }
        #[getter]
        #[pyo3(name = "rotations")]
        pub fn rotations<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.rotations.clone();
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "rotations")]
        pub fn set_rotations(&mut self, value: Vec<usize>) -> PyResult<()> {
            self.0.rotations = value;
            Ok(())
        }
        #[getter]
        #[pyo3(name = "invert")]
        pub fn invert<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.invert;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "invert")]
        pub fn set_invert(&mut self, value: bool) -> PyResult<()> {
            self.0.invert = value;
            Ok(())
        }
        #[getter]
        #[pyo3(name = "flip")]
        pub fn flip<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.flip;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "flip")]
        pub fn set_flip(&mut self, value: bool) -> PyResult<()> {
            self.0.flip = value;
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
        #[pyo3(name = "height")]
        pub fn height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let value = self.0.height;
            (value).into_bound_py_any(py)
        }
        #[setter]
        #[pyo3(name = "height")]
        pub fn set_height(&mut self, value: usize) -> PyResult<()> {
            self.0.height = value;
            Ok(())
        }
        #[pyo3(name = "check", signature = ())]
        pub fn check<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::Tile::check(&self.0);
            (ok(out)?).into_bound_py_any(py)
        }
        #[pyo3(name = "degenerate", signature = ())]
        pub fn degenerate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::Tile::degenerate(&self.0);
            (out).into_bound_py_any(py)
        }
        #[pyo3(name = "max_size", signature = ())]
        pub fn max_size<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::Tile::max_size(&self.0);
            (out).into_bound_py_any(py)
        }
        #[staticmethod]
        #[pyo3(name = "new", signature = (group))]
        pub fn new_<'py>(
            py: Python<'py>,
            group: PySerde<mrly_tile::Group>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let group = group.0;
            let out = mrly_tile::Tile::new(group);
            (crate::doors::tile::Tile(out)).into_bound_py_any(py)
        }
        #[pyo3(name = "resize", signature = ())]
        pub fn resize<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            mrly_tile::Tile::resize(&mut self.0);
            ().into_bound_py_any(py)
        }
        #[pyo3(name = "size", signature = (width, height))]
        pub fn size<'py>(
            &self,
            py: Python<'py>,
            width: usize,
            height: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::Tile::size(self.0.clone(), width, height);
            (crate::doors::tile::Tile(out)).into_bound_py_any(py)
        }
        #[staticmethod]
        pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {
            Ok(Self(crate::hand::serde_from_py(data)?))
        }
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            crate::hand::serde_into_py(py, &self.0)
        }
    }

    #[pyclass(name = "Group", module = "mrly.tile", skip_from_py_object)]
    pub struct Group;

    #[pymethods]
    impl Group {
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::Group::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
    }

    #[pyclass(name = "Parity", module = "mrly.tile", skip_from_py_object)]
    pub struct Parity;

    #[pymethods]
    impl Parity {
        #[staticmethod]
        #[pyo3(name = "all", signature = ())]
        pub fn all<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            let out = mrly_tile::Parity::all();
            ((out).into_iter().map(PySerde).collect::<Vec<_>>()).into_bound_py_any(py)
        }
        #[staticmethod]
        #[pyo3(name = "keep", signature = (parity, n))]
        pub fn keep<'py>(
            py: Python<'py>,
            parity: PySerde<mrly_tile::Parity>,
            n: usize,
        ) -> PyResult<Bound<'py, PyAny>> {
            let parity = parity.0;
            let out = mrly_tile::Parity::keep(parity, n);
            (out).into_bound_py_any(py)
        }
    }

    #[pyfunction]
    #[pyo3(name = "background", signature = (seed, width, height))]
    pub fn background<'py>(
        py: Python<'py>,
        seed: u64,
        width: usize,
        height: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_tile::background(seed, width, height);
        (PyImage(ok(out)?)).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "classic_code", signature = (design))]
    pub fn classic_code<'py>(
        py: Python<'py>,
        design: PySerde<mrly_tile::recipe::Design>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let design = design.0;
        let out = mrly_tile::classic_code(design);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "classic_code_nd", signature = (design, dimension))]
    pub fn classic_code_nd<'py>(
        py: Python<'py>,
        design: PySerde<mrly_tile::recipe::Design>,
        dimension: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let design = design.0;
        let out = mrly_tile::classic_code_nd(design, dimension);
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "hex_key", signature = (length, rng))]
    pub fn hex_key<'py>(
        py: Python<'py>,
        length: usize,
        rng: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut rng_owned = crate::hand::rng_from_py(rng)?;
        let out = mrly_tile::hex_key(length, &mut rng_owned);
        crate::hand::rng_write_back(rng, &rng_owned)?;
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "random_design", signature = (rng))]
    pub fn random_design<'py>(
        py: Python<'py>,
        rng: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut rng_owned = crate::hand::rng_from_py(rng)?;
        let out = mrly_tile::random_design(&mut rng_owned);
        crate::hand::rng_write_back(rng, &rng_owned)?;
        (PySerde(out)).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "random_rotation", signature = (design, rng))]
    pub fn random_rotation<'py>(
        py: Python<'py>,
        design: PySerde<mrly_tile::recipe::Design>,
        rng: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let design = design.0;
        let mut rng_owned = crate::hand::rng_from_py(rng)?;
        let out = mrly_tile::random_rotation(design, &mut rng_owned);
        crate::hand::rng_write_back(rng, &rng_owned)?;
        (out).into_bound_py_any(py)
    }

    #[pyfunction]
    #[pyo3(name = "tree_mask", signature = (n))]
    pub fn tree_mask<'py>(py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        let out = mrly_tile::tree_mask(n);
        (PyTensor(ok(out)?)).into_bound_py_any(py)
    }

    pub fn init(
        py: Python<'_>,
        parent: &Bound<'_, PyModule>,
        sys: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let m = PyModule::new(py, "mrly.tile")?;
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
        let names: Vec<&str> = vec![
            "background",
            "classic_code",
            "classic_code_nd",
            "hex_key",
            "random_design",
            "random_rotation",
            "tree_mask",
            "Tile",
            "Group",
            "Parity",
        ];
        m.add("__all__", names)?;
        build::init(py, &m, sys)?;
        name::init(py, &m, sys)?;
        recipe::init(py, &m, sys)?;
        variation::init(py, &m, sys)?;
        parent.add("tile", &m)?;
        sys.set_item("mrly.tile._tile.tile", &m)?;
        Ok(())
    }
}

pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = py
        .import("sys")?
        .getattr("modules")?
        .cast_into::<PyDict>()?;
    tile::init(py, root, &sys)?;
    Ok(())
}
