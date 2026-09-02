#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use mrlycore::MrlyError;
use wasm_bindgen::prelude::*;

/// The elementary automata: their rows stepped, their space-time diagrams and the card of one rule.
pub mod automata;
/// The universes: codes, symmetries, counts, closed-form fills and names.
pub mod bang;
/// The blends: registry sequences drawn as terms, ratios, differences and the recurrence they satisfy, and the term operations that mix two of them.
pub mod blend;
/// The slice carry automaton: its digit polynomial, its even block, its ladder, its sign law and its spectral ratio.
pub mod carry;
/// The census: which integers the whole registry writes inside a pinned window, how often, and which rows write one.
pub mod census;
/// The exact crops: the designs trimmed to rational shapes, tallied, swept, drawn and masked.
pub mod crop;
/// The alphabet: text laid out as a grid, written in stroke order, cycled, and read glyph by glyph.
pub mod font;
/// The primes of the plane: the Gaussian and the Eisenstein windows painted, counted, clicked and weighed by norm.
pub mod gauss;
/// The networks of the designs: nodes, branches, roles and censuses, and the force layout that relaxes them.
pub mod graph;
/// The laboratory: the sequence press and the moire presets.
pub mod lab;
/// The lattice: the Farey nodes and the totients.
pub mod lattice;
/// The ledger: every measure of every design as a sequence, searched, identified and read against the curated records.
pub mod ledger;
/// The life grids stepped, run and driven by sequences.
pub mod life;
/// The magic words: the folded design, its census, its press readings and its prefix rates.
pub mod magic;
/// The Thue-Morse word: its two constructions, its plane lifts, its runs and the difference filter.
pub mod morse;
/// The primes: the sieve stepped, the stone pile, the count chart and the carpet witness.
pub mod prime;
/// The race: seeded walkers loose on a flat design.
pub mod race;
/// The hexagon projections of a cube as SVG.
pub mod six;
/// The Laplacian spectra of the designs: eigenvalues, degeneracy and the spectral exponent.
pub mod spectrum;
/// The turntable: designs, moire fields and slices spun about their centre into ring profiles and wheels.
pub mod spin;
/// The spirals: the whole numbers wound on a square or hexagonal sheet, painted, clicked and read along a quadratic.
pub mod spiral;
/// The cubes as packed faces, filled cells and censuses.
pub mod three;
/// The tessellations: a design repeated across the plane, the cube and the hexagonal mesh, drawn and counted.
pub mod tile;
/// The flat designs as byte grids, painted pixels and censuses.
pub mod two;
/// The cube designs stacked into a moire volume: its faces at a level, and the planes that cut it.
pub mod volume;
/// The critical line: zeta walked at one half plus i t, its zeros counted and listed, and the prime staircase against the explicit formula.
pub mod zeta;

pub(crate) mod ink {
    pub const DEEP: [u8; 4] = [7, 9, 11, 255];
    pub const FAINT: [u8; 4] = [31, 38, 46, 255];
    pub const GOLD: [u8; 4] = [255, 209, 102, 255];
    pub const BLUE: [u8; 4] = [92, 200, 255, 255];
    pub const ORANGE: [u8; 4] = [255, 138, 92, 255];
    pub const PINK: [u8; 4] = [255, 122, 182, 255];
    pub const GREEN: [u8; 4] = [110, 231, 168, 255];
}

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
