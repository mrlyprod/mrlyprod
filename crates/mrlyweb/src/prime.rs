use crate::{Fault, Grid};
use mrlycore::json;
use mrlylab::moire::pairs;
use mrlynum::prime;
use wasm_bindgen::prelude::*;

const SHEET: usize = 400;
const STONES: u64 = 1_000_000_000_000;
const TOP: usize = 1_000_000;
const SCALE: usize = 999;

/// The sieve of Eratosthenes taken one prime at a time: each byte says who struck the number.
#[wasm_bindgen]
pub struct Sieve(prime::Sieve);

#[wasm_bindgen]
impl Sieve {
    /// Starts a sieve over zero through the limit, at most four hundred.
    #[wasm_bindgen(constructor)]
    pub fn new(limit: usize) -> Result<Sieve, Fault> {
        if limit > SHEET {
            return Err(Fault::new(format!("the sheet holds {SHEET} numbers.")));
        }
        Ok(Sieve(prime::Sieve::new(limit)))
    }
    /// Uses the next prime and returns it, zero once the sieve is done.
    pub fn step(&mut self) -> u32 {
        self.0.step() as u32
    }
    /// Runs the sieve to the end.
    pub fn finish(&mut self) {
        self.0.finish();
    }
    /// Returns whether every number is settled.
    pub fn done(&self) -> bool {
        self.0.done()
    }
    /// Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it.
    pub fn types(&self) -> Vec<u8> {
        self.0.types().to_vec()
    }
    /// Returns the count of numbers marked prime so far.
    pub fn count(&self) -> u32 {
        self.0.count() as u32
    }
    /// Returns the count of numbers the last step struck.
    pub fn struck(&self) -> u32 {
        self.0.struck() as u32
    }
    /// Returns the count of primes used so far.
    pub fn rank(&self) -> u32 {
        self.0.rank() as u32
    }
    /// Lays the numbers from one out in rows of the given width as a grid, one on the primes.
    pub fn grid(&self, columns: usize) -> Result<Grid, Fault> {
        if columns == 0 {
            return Err(Fault::new("a row needs a width."));
        }
        let types = &self.0.types()[1..];
        let height = types.len().div_ceil(columns);
        let mut cells = vec![0u8; columns * height];
        for (cell, &t) in cells.iter_mut().zip(types) {
            *cell = u8::from(t == 1);
        }
        Ok(Grid {
            width: columns as u32,
            height: height as u32,
            types: cells,
        })
    }
}

/// Reads a number of stones up to a million million: its prime factors as pairs, whether it is prime, and every rectangle as a pair of sides, as JSON.
#[wasm_bindgen]
pub fn factor(number: &str) -> Result<String, Fault> {
    let number: u64 = number
        .trim()
        .parse()
        .map_err(|_| Fault::new(format!("{number:?} is not a whole number.")))?;
    if number > STONES {
        return Err(Fault::new(format!("the pile holds {STONES} stones.")));
    }
    let pile = prime::pile(number);
    Ok(json!({
        "n": pile.number,
        "factors": pile.factors,
        "prime": pile.prime,
        "rectangles": pile.rectangles,
    })
    .to_string())
}

/// Reads the prime count against x over ln x and li at evenly spaced x up to the top, a million at most, in at most the given count of bins, as JSON columns.
#[wasm_bindgen]
pub fn prime_chart(top: usize, bins: usize) -> Result<String, Fault> {
    if top > TOP {
        return Err(Fault::new(format!("the chart counts to {TOP}.")));
    }
    let readings = prime::chart(top, bins.min(1000));
    let column = |pick: fn(&prime::Reading) -> f64| readings.iter().map(pick).collect::<Vec<f64>>();
    Ok(json!({
        "x": readings.iter().map(|r| r.x).collect::<Vec<usize>>(),
        "pi": readings.iter().map(|r| r.pi).collect::<Vec<usize>>(),
        "ratio": column(|r| r.ratio),
        "li": column(|r| r.li),
    })
    .to_string())
}

/// Returns the smallest prime at or above the number.
#[wasm_bindgen]
pub fn prime_from(number: u32) -> u32 {
    prime::prime_from(number as usize) as u32
}

/// Puts an odd scale on trial against every earlier odd scale of the flat carpet stack: the scales, the exact correlation with each, the largest and where, and whether the row is clear, as JSON.
#[wasm_bindgen]
pub fn carpet_witness(scale: usize) -> Result<String, Fault> {
    if scale > SCALE {
        return Err(Fault::new(format!("the stack reaches scale {SCALE}.")));
    }
    let trial = pairs::witness(scale)?;
    Ok(json!({
        "n": trial.scale,
        "scales": trial.scales,
        "row": trial.row,
        "max": trial.max,
        "at": trial.at,
        "prime": trial.prime,
    })
    .to_string())
}
