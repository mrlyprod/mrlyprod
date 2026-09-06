use crate::{Fault, Grid};
use mrlycore::json;
use mrlymath::space::{Pack, Vec3};
use mrlynum::sieve;
use wasm_bindgen::prelude::*;

const PLANE_SITES: usize = 4_000_000;
const HOLE_BUDGET: u128 = 20_000;
const READ_LEVELS: usize = 16;
const WALK_STOPS: usize = 400;

fn word(kind: &str, letter: u32, levels: usize) -> Result<Vec<u64>, Fault> {
    let side = u64::from(letter);
    if levels > READ_LEVELS {
        return Err(Fault::new(format!(
            "the levels must be between 1 and {READ_LEVELS}."
        )));
    }
    match kind {
        "odd" => Ok(sieve::odd_word(levels)),
        "flat" => {
            if side < 3 || side % 2 == 0 || side > 15 {
                return Err(Fault::new(format!(
                    "the letter {letter} is not an odd side between three and fifteen."
                )));
            }
            Ok(sieve::flat_word(side, levels))
        }
        _ => Err(Fault::new(format!("no schedule is named {kind:?}."))),
    }
}

fn axes(dimension: usize) -> Result<u32, Fault> {
    match dimension {
        2 | 3 => Ok(dimension as u32),
        _ => Err(Fault::new("the sieve draws in two or three dimensions.")),
    }
}

fn fits(word: &[u64], dimension: u32) -> bool {
    if dimension == 2 {
        let side = sieve::side(word);
        return side * side <= PLANE_SITES as u128;
    }
    sieve::holes(word, dimension) <= HOLE_BUDGET
}

/// Returns the deepest level the schedule reaches before it outgrows the sites this page rasters in the plane or the punctures it draws in the cube.
#[wasm_bindgen]
pub fn wallis_cap(kind: &str, letter: u32, dimension: usize) -> Result<usize, Fault> {
    let axes = axes(dimension)?;
    let mut top = 1;
    for levels in 1..=READ_LEVELS {
        if !fits(&word(kind, letter, levels)?, axes) {
            break;
        }
        top = levels;
    }
    Ok(top)
}

/// Reads the schedule at every level up to the one asked: its letter, its side, its surviving cells, its punctures, the share of the whole it leaves and the box exponent it reads, then the word's own reading beside the limit its schedule walks to and the gap left, as JSON.
#[wasm_bindgen]
pub fn wallis_read(
    kind: &str,
    letter: u32,
    levels: usize,
    dimension: usize,
) -> Result<String, Fault> {
    let axes = axes(dimension)?;
    let schedule = word(kind, letter, levels.max(2))?;
    let word = word(kind, letter, levels)?;
    let rows: Vec<mrlycore::Json> = (1..=word.len())
        .map(|n| {
            let prefix = &word[..n];
            json!({
                "level": n,
                "letter": prefix[n - 1],
                "side": sieve::side(prefix).to_string(),
                "cells": sieve::cells(prefix, axes).to_string(),
                "holes": sieve::holes(prefix, axes).to_string(),
                "ratio": sieve::ratio(prefix, axes),
                "exponent": sieve::exponent(prefix, axes),
            })
        })
        .collect();
    let ratio = sieve::ratio(&word, axes);
    let limit = sieve::limit(&schedule, axes).unwrap_or(0.0);
    Ok(json!({
        "word": word.clone(),
        "dimension": dimension,
        "side": sieve::side(&word).to_string(),
        "cells": sieve::cells(&word, axes).to_string(),
        "holes": sieve::holes(&word, axes).to_string(),
        "ratio": ratio,
        "exponent": sieve::exponent(&word, axes),
        "limit": limit,
        "gap": ratio - limit,
        "closed": limit > 0.0,
        "levels": rows,
    })
    .to_string())
}

/// Walks the share of the whole the schedule leaves at level one through the count of stops, one number a level, so the approach to the limit can be drawn past the level the page rasters.
#[wasm_bindgen]
pub fn wallis_walk(
    kind: &str,
    letter: u32,
    dimension: usize,
    stops: usize,
) -> Result<Vec<f64>, Fault> {
    let axes = axes(dimension)?;
    if !(1..=WALK_STOPS).contains(&stops) {
        return Err(Fault::new(format!(
            "the stops must be between 1 and {WALK_STOPS}."
        )));
    }
    let word = match kind {
        "odd" => sieve::odd_word(stops),
        _ => word(kind, letter, 1).map(|_| sieve::flat_word(u64::from(letter), stops))?,
    };
    Ok((1..=stops)
        .map(|n| sieve::ratio(&word[..n], axes))
        .collect())
}

/// Builds the plane sieve the schedule spells as a byte grid, one byte a site, one where the site survives and zero where a level punched it out.
#[wasm_bindgen]
pub fn wallis_grid(kind: &str, letter: u32, levels: usize) -> Result<Grid, Fault> {
    let word = word(kind, letter, levels)?;
    if !fits(&word, 2) {
        return Err(Fault::new(format!(
            "a side of {} is more than this page rasters; lower the level.",
            sieve::side(&word)
        )));
    }
    let (side, sites) = sieve::raster(&word);
    Ok(Grid {
        width: side as u32,
        height: side as u32,
        types: sites,
    })
}

/// Packs the punctures of the solid sieve the schedule spells as boxes: two section lengths, then six floats per vertex, position and normal, in the unit box, one box a hole at the size the level that punched it left.
#[wasm_bindgen]
pub fn wallis_faces(kind: &str, letter: u32, levels: usize) -> Result<Vec<f32>, Fault> {
    let word = word(kind, letter, levels)?;
    if !fits(&word, 3) {
        return Err(Fault::new(format!(
            "{} punctures is more than this page draws; lower the level.",
            sieve::holes(&word, 3)
        )));
    }
    let half = sieve::side(&word) as f32 / 2.0;
    let at =
        |x: f32, y: f32, z: f32| Vec3::new((x - half) / half, (y - half) / half, (z - half) / half);
    let mut pack = Pack::new();
    for hole in sieve::punctures(&word, 3).chunks(4) {
        let (x, y, z) = (hole[0] as f32, hole[1] as f32, hole[2] as f32);
        let s = hole[3] as f32;
        let (a, b, c) = (x + s, y + s, z + s);
        pack.quad(
            [at(x, y, z), at(x, y, c), at(x, b, c), at(x, b, z)],
            Vec3::new(-1.0, 0.0, 0.0),
        );
        pack.quad(
            [at(a, y, z), at(a, b, z), at(a, b, c), at(a, y, c)],
            Vec3::new(1.0, 0.0, 0.0),
        );
        pack.quad(
            [at(x, y, z), at(a, y, z), at(a, y, c), at(x, y, c)],
            Vec3::new(0.0, -1.0, 0.0),
        );
        pack.quad(
            [at(x, b, z), at(x, b, c), at(a, b, c), at(a, b, z)],
            Vec3::new(0.0, 1.0, 0.0),
        );
        pack.quad(
            [at(x, y, z), at(x, b, z), at(a, b, z), at(a, y, z)],
            Vec3::new(0.0, 0.0, -1.0),
        );
        pack.quad(
            [at(x, y, c), at(a, y, c), at(a, b, c), at(x, b, c)],
            Vec3::new(0.0, 0.0, 1.0),
        );
    }
    Ok(pack.buffer())
}
