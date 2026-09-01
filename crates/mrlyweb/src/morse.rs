use crate::{code_of, Fault, Grid};
use mrlycore::json;
use mrlymath::two;
use mrlynum::morse::{self, Lift, LIFTS};
use wasm_bindgen::prelude::*;

const WORD_MAX: usize = 4096;
const ROUNDS_MAX: usize = 12;
const SIDE_MAX: usize = 512;

fn word_of(length: usize) -> Result<Vec<u8>, Fault> {
    if !(1..=WORD_MAX).contains(&length) {
        return Err(Fault::new(format!(
            "the word runs from one letter to {WORD_MAX}."
        )));
    }
    Ok(morse::digits(length))
}

fn tile_of(code: &str, number: usize, base: usize) -> Result<Vec<u8>, Fault> {
    if number < 2 {
        return Err(Fault::new(format!("side {number} is below two.")));
    }
    let cell = two::create(code_of(code)?, number, 1, 0, base)?;
    Ok(cell.types().bytes().to_vec())
}

fn signs_of(tile: &[u8]) -> Vec<u8> {
    tile.iter().map(|&byte| 1 - (byte != 0) as u8).collect()
}

fn side_of(number: usize, level: usize) -> Result<usize, Fault> {
    let side = number
        .checked_pow(level as u32)
        .filter(|&side| side <= SIDE_MAX)
        .ok_or_else(|| {
            Fault::new(format!(
                "side {number} to the {level} is more than the {SIDE_MAX} this page draws."
            ))
        })?;
    Ok(side)
}

fn design_of(tile: &[u8]) -> Option<String> {
    for code in 0..16u128 {
        let cell = two::create(code, 2, 1, 0, 2).ok()?;
        if signs_of(cell.types().bytes()) == tile {
            return Some(code.to_string());
        }
    }
    None
}

// THE WORD

/// Reads the Thue-Morse word to the length: the two constructions, the runs, and the boundary word.
///
/// The digit rule is the parity of the binary digit sum of the place; the substitution grows
/// `0 -> 01`, `1 -> 10` from a single 0. They agree letter for letter, which is the page's first
/// claim, checked here rather than asserted.
#[wasm_bindgen]
pub fn morse_word(length: usize) -> Result<String, Fault> {
    let digits = word_of(length)?;
    let substitution = morse::substitution(length);
    let runs = morse::runs(&digits);
    let longest = runs.iter().copied().max().unwrap_or(0);
    let boundary = morse::boundary(&digits);
    let doubling = morse::doubling(boundary.len());
    Ok(json!({
        "length": length,
        "digits": digits,
        "substitution": substitution.clone(),
        "agree": digits == substitution,
        "ones": digits.iter().map(|&bit| bit as usize).sum::<usize>(),
        "runs": runs.clone(),
        "longest": longest,
        "cube_free": longest <= 2,
        "singles": runs.iter().filter(|&&run| run == 1).count(),
        "doubles": runs.iter().filter(|&&run| run == 2).count(),
        "boundary": boundary.clone(),
        "doubling": doubling.clone(),
        "doubling_agree": boundary == doubling,
    })
    .to_string())
}

/// Returns the substitution stage after the rounds, a word of length two to the rounds.
#[wasm_bindgen]
pub fn morse_stage(rounds: usize) -> Result<Vec<u8>, Fault> {
    if rounds > ROUNDS_MAX {
        return Err(Fault::new(format!(
            "the substitution animates to {ROUNDS_MAX} rounds."
        )));
    }
    Ok(morse::stage(rounds))
}

// THE LIFTS

/// Builds one plane lift of the word as a sign grid, zero for plus one and one for minus one.
#[wasm_bindgen]
pub fn morse_lift(kind: &str, level: usize) -> Result<Grid, Fault> {
    let side = side_of(2, level)?;
    let types = morse::lift(Lift::parse(kind)?, side);
    Ok(Grid {
        width: side as u32,
        height: side as u32,
        types,
    })
}

/// Tests every lift at the level against the Kronecker power of its own corner tile, as JSON.
///
/// Each row carries the lift's formula, the verdict, the corner tile, the count of sites where
/// the fold fails, the first such site, the earlier lift it is identical to when there is one,
/// and the plane design whose plus-minus render it is when the fold succeeds.
#[wasm_bindgen]
pub fn morse_gallery(level: usize) -> Result<String, Fault> {
    let side = side_of(2, level)?;
    let mut rows = Vec::new();
    let mut drawn: Vec<(&'static str, Vec<u8>)> = Vec::new();
    for kind in LIFTS {
        let grid = morse::lift(kind, side);
        let read = morse::fold(&grid, side, 2)?;
        let twin = drawn
            .iter()
            .find(|(_, seen)| *seen == grid)
            .map(|(name, _)| *name);
        rows.push(json!({
            "name": kind.name(),
            "formula": kind.formula(),
            "side": side,
            "level": read.level,
            "folds": read.folds,
            "tile": read.tile.clone(),
            "faults": read.faults,
            "first": read.first.map(|(r, c)| vec![r, c]),
            "twin": twin,
            "design": if read.folds { design_of(&read.tile) } else { None },
        }));
        drawn.push((kind.name(), grid));
    }
    Ok(json!(rows).to_string())
}

// THE DESIGNS

/// Builds a plane design read plus-minus at the level: plus one where it fills, minus one where
/// it does not, folded by the exclusive or rather than the and.
#[wasm_bindgen]
pub fn morse_signs(code: &str, number: usize, base: usize, level: usize) -> Result<Grid, Fault> {
    let side = side_of(number, level)?;
    let tile = signs_of(&tile_of(code, number, base)?);
    Ok(Grid {
        width: side as u32,
        height: side as u32,
        types: morse::power(&tile, number, level)?,
    })
}

// THE FILTER

struct Pair {
    grown: Vec<u8>,
    fine: Vec<u8>,
    tile: Vec<u8>,
    wide: usize,
}

fn levels(code: &str, number: usize, base: usize, level: usize, fold: &str) -> Result<Pair, Fault> {
    let wide = side_of(number, level + 1)?;
    let side = wide / number;
    let tile = tile_of(code, number, base)?;
    let (coarse, fine) = match fold {
        "design" => (
            two::create(code_of(code)?, number, level, 0, base)?
                .types()
                .bytes()
                .to_vec(),
            two::create(code_of(code)?, number, level + 1, 0, base)?
                .types()
                .bytes()
                .to_vec(),
        ),
        "sign" => {
            let signs = signs_of(&tile);
            (
                morse::power(&signs, number, level)?,
                morse::power(&signs, number, level + 1)?,
            )
        }
        other => return Err(Fault::new(format!("unknown fold {other:?}."))),
    };
    Ok(Pair {
        grown: morse::upsample(&coarse, side, number),
        fine,
        tile,
        wide,
    })
}

/// Builds the difference filter: a design's level blown up to the next side and exclusive-ored
/// against the next level, one where the two disagree.
#[wasm_bindgen]
pub fn morse_difference(
    code: &str,
    number: usize,
    base: usize,
    level: usize,
    fold: &str,
) -> Result<Grid, Fault> {
    let pair = levels(code, number, base, level, fold)?;
    Ok(Grid {
        width: pair.wide as u32,
        height: pair.wide as u32,
        types: morse::difference(&pair.grown, &pair.fine),
    })
}

/// Judges the difference filter against its closed form and against the Thue-Morse grid, as JSON.
///
/// The closed form is exact in both folds and needs no search. Under the and fold the next level
/// is the blown-up level masked by the tile, so the difference is the blown-up level masked by
/// the tile's complement. Under the exclusive-or fold the next level is the blown-up level
/// exclusive-ored with the repeated tile, so the difference is the repeated tile alone. Either
/// way the filter keeps only the last digit, so its output repeats with period `number` while the
/// Thue-Morse grid does not, and the two differ at every side past `number`. Under the
/// exclusive-or fold at side two the disagreement is exactly half the sites at every side four and
/// beyond, for every tile: the low digits fix a residue class and the high digits of the two
/// coordinates carry opposite Thue-Morse letters on exactly half of each class.
#[wasm_bindgen]
pub fn morse_filter(
    code: &str,
    number: usize,
    base: usize,
    level: usize,
    fold: &str,
) -> Result<String, Fault> {
    let pair = levels(code, number, base, level, fold)?;
    let wide = pair.wide;
    let difference = morse::difference(&pair.grown, &pair.fine);
    let signs = signs_of(&pair.tile);
    let (form, closed) = if fold == "sign" {
        (
            "the base tile repeated",
            morse::repeat(&signs, number, wide),
        )
    } else {
        let mask = morse::repeat(&pair.tile, number, wide);
        (
            "the level below, punched by the tile's complement",
            pair.grown
                .iter()
                .zip(&mask)
                .map(|(&bit, &keep)| bit & (1 - keep))
                .collect::<Vec<u8>>(),
        )
    };
    let closed_faults = morse::faults(&difference, &closed);
    let grid = (number == 2).then(|| morse::lift(Lift::Parity, wide));
    let morse_faults = grid.as_ref().map(|grid| morse::faults(&difference, grid));
    Ok(json!({
        "fold": fold,
        "number": number,
        "level": level,
        "side": wide,
        "tile": pair.tile.clone(),
        "signs": signs.clone(),
        "morse_tile": signs == vec![0, 1, 1, 0],
        "form": form,
        "closed": closed.clone(),
        "closed_faults": closed_faults,
        "closed_exact": closed_faults == 0,
        "morse_faults": morse_faults,
        "morse_exact": morse_faults == Some(0),
        "lit": difference.iter().map(|&bit| bit as usize).sum::<usize>(),
        "cells": difference.len(),
    })
    .to_string())
}
