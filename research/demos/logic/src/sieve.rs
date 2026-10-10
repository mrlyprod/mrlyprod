use crate::space::Pack;
use crate::{checked, Fault, Grid};
use mrlyrs::core::{json, Json, Tensor};
use mrlyrs::math::bang::factory::create;
use mrlyrs::math::bang::Code;
use mrlyrs::math::three::Vec3;
use mrlyrs::num::sieve;
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

fn fits(word: &[u64], dimension: u32) -> Result<bool, Fault> {
    if dimension == 2 {
        let side = sieve::side(word)?;
        return Ok(side * side <= PLANE_SITES as u128);
    }
    Ok(sieve::holes(word, dimension)? <= HOLE_BUDGET)
}

/// Returns the deepest level the schedule reaches before it outgrows the sites this page rasters in the plane or the punctures it draws in the cube.
#[wasm_bindgen]
pub fn wallis_cap(kind: &str, letter: u32, dimension: usize) -> Result<usize, Fault> {
    let axes = axes(dimension)?;
    let mut top = 1;
    for levels in 1..=READ_LEVELS {
        if !fits(&word(kind, letter, levels)?, axes)? {
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
    let mut rows: Vec<mrlyrs::core::Json> = Vec::with_capacity(word.len());
    for n in 1..=word.len() {
        let prefix = &word[..n];
        rows.push(json!({
            "level": n,
            "letter": prefix[n - 1],
            "side": sieve::side(prefix)?.to_string(),
            "cells": sieve::cells(prefix, axes)?.to_string(),
            "holes": sieve::holes(prefix, axes)?.to_string(),
            "ratio": sieve::ratio(prefix, axes)?,
            "exponent": sieve::exponent(prefix, axes)?,
        }));
    }
    let ratio = sieve::ratio(&word, axes)?;
    let limit = sieve::limit(&schedule, axes)?.unwrap_or(0.0);
    Ok(json!({
        "word": word.clone(),
        "dimension": dimension,
        "side": sieve::side(&word)?.to_string(),
        "cells": sieve::cells(&word, axes)?.to_string(),
        "holes": sieve::holes(&word, axes)?.to_string(),
        "ratio": ratio,
        "exponent": sieve::exponent(&word, axes)?,
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
    let mut out = Vec::with_capacity(stops);
    for n in 1..=stops {
        out.push(sieve::ratio(&word[..n], axes)?);
    }
    Ok(out)
}

/// Builds the plane sieve the schedule spells as a byte grid, one byte a site, one where the site survives and zero where a level punched it out.
#[wasm_bindgen]
pub fn wallis_grid(kind: &str, letter: u32, levels: usize) -> Result<Grid, Fault> {
    let word = word(kind, letter, levels)?;
    if !fits(&word, 2)? {
        return Err(Fault::new(format!(
            "a side of {} is more than this page rasters; lower the level.",
            sieve::side(&word)?
        )));
    }
    let (side, sites) = sieve::raster(&word)?;
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
    if !fits(&word, 3)? {
        return Err(Fault::new(format!(
            "{} punctures is more than this page draws; lower the level.",
            sieve::holes(&word, 3)?
        )));
    }
    let half = sieve::side(&word)? as f32 / 2.0;
    let at =
        |x: f32, y: f32, z: f32| Vec3::new((x - half) / half, (y - half) / half, (z - half) / half);
    let mut pack = Pack::new();
    for hole in sieve::punctures(&word, 3)?.chunks(4) {
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

// THE ROW WORD

const ROW_LEVELS: usize = 12;
const ROW_REACH: usize = 1_000_000;
const ROW_STOPS: usize = 400;
const ROW_DRAWN: u64 = 1000;

fn row_design(code: &str, dimension: usize) -> Result<(u128, Vec<u64>), Fault> {
    let code = checked(code, dimension, 2)?;
    if code == 0 {
        return Err(Fault::new("the empty design fills nothing; pick a corner."));
    }
    Ok((code, sieve::row_profile(code, dimension)?))
}

fn row_side(level: usize, even: bool) -> u64 {
    (if even { 2 * level } else { 2 * level + 1 }) as u64
}

fn row_mirror(code: u128, dimension: usize) -> u128 {
    let flip = (1usize << dimension) - 1;
    (0..1usize << dimension)
        .filter(|&i| code >> i & 1 == 1)
        .fold(0, |acc, i| acc | 1 << (i ^ flip))
}

fn row_floor(code: u128, dimension: usize) -> u128 {
    if dimension == 3 {
        code & 15
    } else {
        code
    }
}

fn row_letters(
    code: u128,
    dimension: usize,
    letters: usize,
    even: bool,
) -> Result<Vec<Tensor>, Fault> {
    let plane = dimension.min(2);
    let mut out = Vec::with_capacity(letters);
    let mut word: Option<Tensor> = None;
    for level in 1..=letters {
        let letter = create(
            Code::from(code),
            row_side(level, even) as usize,
            plane,
            2,
            1,
        )?;
        let next = match word {
            Some(tile) => tile.kron(&letter),
            None => letter,
        };
        out.push(next.clone());
        word = Some(next);
    }
    Ok(out)
}

/// Returns how many letters of the row word this page draws, the most whose product of sides stays inside the drawn side.
#[wasm_bindgen]
pub fn staircase_cap(even: bool) -> usize {
    let mut side = 1u64;
    let mut top = 0;
    for level in 1..=ROW_LEVELS {
        side *= row_side(level, even);
        if side > ROW_DRAWN {
            break;
        }
        top = level;
    }
    top
}

/// Reads a base-2 design along the row word to the count of letters, on odd or even sides: its profile, its drift and first correction, the roots of its fill polynomial, the constant from the Gamma form and spelled, by reflection, as a Wallis sieve product and against its mirror, and every letter's side, fill, share, running ratio and renormalised fill beside the law, as JSON.
#[wasm_bindgen]
pub fn staircase_read(
    code: &str,
    dimension: usize,
    levels: usize,
    even: bool,
) -> Result<String, Fault> {
    if !(1..=ROW_LEVELS).contains(&levels) {
        return Err(Fault::new(format!(
            "the letters must be between 1 and {ROW_LEVELS}."
        )));
    }
    let (code, profile) = row_design(code, dimension)?;
    let row = sieve::row_law(&profile)?;
    let mirror_code = row_mirror(code, dimension);
    let mirror = sieve::row_law(&sieve::row_profile(mirror_code, dimension)?)?;
    let stops: Vec<usize> = (1..=levels).collect();
    let walk = sieve::row_settle(&profile, &stops, even)?;
    let drift = row.drift.0 as f64 / row.drift.1 as f64;
    let correction = row.correction.0 as f64 / row.correction.1 as f64;
    let odd: u64 = profile.iter().enumerate().map(|(j, &a)| j as u64 * a).sum();
    let all: u64 = profile.iter().sum::<u64>() * dimension as u64;
    let mut ratio = 1.0f64;
    let mut word = Some((1u128, 1u128, 1u128));
    let mut rows: Vec<Json> = Vec::with_capacity(levels);
    for (level, &settle) in stops.iter().zip(&walk) {
        let side = row_side(*level, even);
        let fill = sieve::row_fill(&profile, side)?;
        let cells = u128::from(side).pow(dimension as u32);
        let share = fill as f64 / cells as f64;
        ratio *= share;
        word = word.and_then(|(span, filled, whole)| {
            Some((
                span.checked_mul(u128::from(side))?,
                filled.checked_mul(fill)?,
                whole.checked_mul(cells)?,
            ))
        });
        let law = if even {
            settle
        } else {
            row.constant * (1.0 + correction / *level as f64)
        };
        rows.push(json!({
            "level": level,
            "side": side,
            "fill": fill.to_string(),
            "cells": cells.to_string(),
            "share": share,
            "ratio": ratio,
            "settle": settle,
            "law": law,
            "word": word.map(|(span, filled, whole)| [span.to_string(), filled.to_string(), whole.to_string()]),
        }));
    }
    Ok(json!({
        "code": code.to_string(),
        "dimension": dimension,
        "corners": profile.iter().sum::<u64>(),
        "box": 1u64 << dimension,
        "even": even,
        "profile": profile,
        "coordinates": [all - odd, odd],
        "drift": [row.drift.0, row.drift.1],
        "correction": [row.correction.0, row.correction.1],
        "rate": drift,
        "c1": correction,
        "roots": row.roots.iter().map(|r| [r.re, r.im]).collect::<Vec<[f64; 2]>>(),
        "constant": row.constant,
        "closed": row.closed,
        "reflection": row.reflection,
        "parity": row.parity.map(|(odd, value)| json!({ "odd": odd, "value": value })),
        "mirror": {
            "code": mirror_code.to_string(),
            "constant": mirror.constant,
            "closed": mirror.closed,
            "both": row.constant * mirror.constant,
            "product": row.mirror,
        },
        "floor": row_floor(code, dimension).to_string(),
        "levels": rows,
    })
    .to_string())
}

/// Walks the renormalised fill of a base-2 design along the row word on a log grid of levels from 1 to the reach, four floats a stop: the level, the fill ratio times `(2^dim/w)^L / L^drift`, the law `C (1 + c_1/L)`, and `L` times the walk's relative gap to its constant, which tends to `c_1`; on even sides the constant is 1 and the law the walk itself.
#[wasm_bindgen]
pub fn staircase_walk(
    code: &str,
    dimension: usize,
    reach: usize,
    stops: usize,
    even: bool,
) -> Result<Vec<f64>, Fault> {
    if !(10..=ROW_REACH).contains(&reach) {
        return Err(Fault::new(format!(
            "the reach must be between 10 and {ROW_REACH}."
        )));
    }
    if !(2..=ROW_STOPS).contains(&stops) {
        return Err(Fault::new(format!(
            "the stops must be between 2 and {ROW_STOPS}."
        )));
    }
    let (_, profile) = row_design(code, dimension)?;
    let row = sieve::row_law(&profile)?;
    let correction = row.correction.0 as f64 / row.correction.1 as f64;
    let mut levels: Vec<usize> = (0..stops)
        .map(|i| (reach as f64).powf(i as f64 / (stops - 1) as f64).round() as usize)
        .collect();
    levels.dedup();
    let walk = sieve::row_settle(&profile, &levels, even)?;
    let mut out = Vec::with_capacity(4 * levels.len());
    for (&level, &value) in levels.iter().zip(&walk) {
        let (law, target) = if even {
            (value, walk[0])
        } else {
            (
                row.constant * (1.0 + correction / level as f64),
                row.constant,
            )
        };
        out.push(level as f64);
        out.push(value);
        out.push(law);
        out.push(level as f64 * (value / target - 1.0));
    }
    Ok(out)
}

/// Draws the first letters of the row word as a byte grid: on the line one band per prefix, each magnified to the full side; in the plane the word itself; in the cube its floor layer, the plane word of the corners whose first coordinate is even.
#[wasm_bindgen]
pub fn staircase_grid(
    code: &str,
    dimension: usize,
    letters: usize,
    even: bool,
) -> Result<Grid, Fault> {
    let (code, _) = row_design(code, dimension)?;
    if letters == 0 || letters > staircase_cap(even) {
        return Err(Fault::new(format!(
            "the picture holds 1 to {} letters.",
            staircase_cap(even)
        )));
    }
    let floor = row_floor(code, dimension);
    if floor == 0 {
        return Err(Fault::new(
            "every corner of this cube design has an odd first coordinate, so its floor layer is empty.",
        ));
    }
    let words = row_letters(floor, dimension, letters, even)?;
    let last = &words[letters - 1];
    let side = last.shape[0];
    if dimension >= 2 {
        return Ok(Grid {
            width: side as u32,
            height: side as u32,
            types: last.bytes()?.to_vec(),
        });
    }
    let band = (side / (3 * letters)).max(2);
    let mut types = Vec::with_capacity(side * (band + 1) * letters);
    for (k, word) in words.iter().enumerate() {
        let cells = word.bytes()?;
        let wide = side / cells.len();
        for _ in 0..band {
            for x in 0..side {
                types.push(cells[x / wide]);
            }
        }
        if k + 1 < letters {
            types.extend(std::iter::repeat_n(0u8, side));
        }
    }
    let height = types.len() / side;
    Ok(Grid {
        width: side as u32,
        height: height as u32,
        types,
    })
}
