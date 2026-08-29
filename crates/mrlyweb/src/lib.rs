#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use mrlycore::MrlyError;
use wasm_bindgen::prelude::*;

/// The universes: codes, symmetries, counts, closed-form fills and names.
pub mod bang;
/// The laboratory: the sequence press and the moire presets.
pub mod lab;
/// The lattice: the Farey nodes and the totients.
pub mod lattice;
/// The life grids stepped, run and driven by sequences.
pub mod life;
/// The race: seeded walkers loose on a flat design.
pub mod race;
/// The hexagon projections of a cube as SVG.
pub mod six;
/// The Laplacian spectra of the designs: eigenvalues, degeneracy and the spectral exponent.
pub mod spectrum;
/// The cubes as packed faces, filled cells and censuses.
pub mod three;
/// The flat designs as byte grids, painted pixels and censuses.
pub mod two;

/// A byte grid: its width, its height and its row-major types.
#[wasm_bindgen(getter_with_clone)]
pub struct Grid {
    /// The count of columns.
    pub width: u32,
    /// The count of rows.
    pub height: u32,
    /// The type of every site, row by row.
    pub types: Vec<u8>,
}

/// A pixel sheet: its width, its height and its row-major RGBA bytes.
#[wasm_bindgen(getter_with_clone)]
pub struct Pixels {
    /// The count of columns.
    pub width: u32,
    /// The count of rows.
    pub height: u32,
    /// Four bytes per pixel, row by row.
    pub rgba: Vec<u8>,
}

/// The one error a call raises: a message the page catches as a thrown Error.
#[derive(Debug)]
pub struct Fault(String);

impl Fault {
    fn new(message: impl Into<String>) -> Fault {
        Fault(message.into())
    }
}

impl From<MrlyError> for Fault {
    fn from(error: MrlyError) -> Fault {
        Fault(error.to_string())
    }
}

impl From<Fault> for JsValue {
    fn from(fault: Fault) -> JsValue {
        JsError::new(&fault.0).into()
    }
}

impl Pixels {
    fn of(width: usize, height: usize, colors: Vec<[u8; 4]>) -> Pixels {
        Pixels {
            width: width as u32,
            height: height as u32,
            rgba: colors.concat(),
        }
    }
}

fn code_of(text: &str) -> Result<u128, Fault> {
    text.trim()
        .parse()
        .map_err(|_| Fault::new(format!("code {text:?} is not a whole number.")))
}

fn checked(code: &str, dimension: usize, base: usize) -> Result<u128, Fault> {
    let code = code_of(code)?;
    mrlymath::bang::code_to_corners(code, dimension, base)?;
    Ok(code)
}
