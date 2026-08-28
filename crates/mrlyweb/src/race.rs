use crate::{code_of, Fault};
use mrlycore::Rng;
use mrlymath::two;
use wasm_bindgen::prelude::*;

/// A race: seeded walkers loose on a flat design, each stepping blind and losing the turn when a hole blocks it.
#[wasm_bindgen]
pub struct Race {
    side: usize,
    types: Vec<u8>,
    home: usize,
    at: Vec<usize>,
    trail: Vec<u16>,
    rng: Rng,
    steps: u32,
}

#[wasm_bindgen]
impl Race {
    /// Builds a race on the design the code names, every walker at the filled site nearest the centre.
    #[wasm_bindgen(constructor)]
    pub fn new(
        code: &str,
        number: usize,
        level: usize,
        base: usize,
        walkers: usize,
        seed: u32,
    ) -> Result<Race, Fault> {
        let cell = two::create(code_of(code)?, number, level, 0, base)?;
        let side = cell.width();
        let types = cell.types().bytes().to_vec();
        let centre = (side as i64 - 1) / 2;
        let home = (0..side * side)
            .filter(|&flat| types[flat] != 0)
            .min_by_key(|&flat| {
                let (r, c) = ((flat / side) as i64 - centre, (flat % side) as i64 - centre);
                r * r + c * c
            })
            .ok_or_else(|| Fault::new("the design is empty."))?;
        Ok(Race {
            side,
            types,
            home,
            at: vec![home; walkers],
            trail: vec![0; side * side],
            rng: Rng::new(seed as u64),
            steps: 0,
        })
    }
    /// Steps every walker the given number of ticks and returns the root mean square distance from home.
    pub fn step(&mut self, ticks: u32) -> f64 {
        let side = self.side;
        for _ in 0..ticks {
            for i in 0..self.at.len() {
                let (r, c) = (self.at[i] / side, self.at[i] % side);
                let (nr, nc) = match self.rng.below(4) {
                    0 => (r + 1, c),
                    1 => (r.wrapping_sub(1), c),
                    2 => (r, c + 1),
                    _ => (r, c.wrapping_sub(1)),
                };
                if nr < side && nc < side && self.types[nr * side + nc] != 0 {
                    self.at[i] = nr * side + nc;
                    self.trail[nr * side + nc] = self.trail[nr * side + nc].saturating_add(1);
                }
            }
            self.steps += 1;
        }
        self.distance()
    }
    /// Returns the root mean square distance of the walkers from home.
    pub fn distance(&self) -> f64 {
        if self.at.is_empty() {
            return 0.0;
        }
        let (hr, hc) = (
            (self.home / self.side) as f64,
            (self.home % self.side) as f64,
        );
        let total: f64 = self
            .at
            .iter()
            .map(|&flat| {
                let (r, c) = (
                    (flat / self.side) as f64 - hr,
                    (flat % self.side) as f64 - hc,
                );
                r * r + c * c
            })
            .sum();
        (total / self.at.len() as f64).sqrt()
    }
    /// Returns the side of the grid.
    pub fn side(&self) -> u32 {
        self.side as u32
    }
    /// Returns the flat index of home.
    pub fn home(&self) -> u32 {
        self.home as u32
    }
    /// Returns the count of ticks stepped so far.
    pub fn steps(&self) -> u32 {
        self.steps
    }
    /// Returns the grid types, one byte per site.
    pub fn types(&self) -> Vec<u8> {
        self.types.clone()
    }
    /// Returns the flat position of every walker.
    pub fn positions(&self) -> Vec<u32> {
        self.at.iter().map(|&flat| flat as u32).collect()
    }
    /// Returns the visit count of every site.
    pub fn trail(&self) -> Vec<u16> {
        self.trail.clone()
    }
}
