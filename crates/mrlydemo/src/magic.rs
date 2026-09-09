use crate::{checked, Fault, Grid, Pixels};
use mrlycore::cell::mapping;
use mrlycore::tile::{Group, Source, Tile};
use mrlycore::{json, Json, Mode, Tensor};
use mrlylab::press;
use mrlymath::bang::{magic, word, MagicLayer};
use mrlymath::name::{Bang, Named, Word};
use mrlymath::six::{self, Cell6d};
use mrlymath::space::Pack;
use mrlymath::three::{quads, Cell3d};
use mrlymath::two::Cell2d;
use wasm_bindgen::prelude::*;

const PLANE_SIDE: usize = 243;
const SOLID_SIDE: usize = 128;
const HEX_SIDE: usize = 81;
const DRAWN_CELLS: u128 = 1 << 20;

fn letters(
    codes: Vec<String>,
    numbers: Vec<u32>,
    dimension: usize,
    bases: Vec<u32>,
) -> Result<Vec<MagicLayer>, Fault> {
    if codes.len() != numbers.len() || codes.len() != bases.len() {
        return Err(Fault::new("a word wants one side and one base per letter."));
    }
    if codes.len() < 2 {
        return Err(Fault::new("a word needs at least two letters."));
    }
    let mut out = Vec::with_capacity(codes.len());
    for ((code, number), base) in codes.iter().zip(&numbers).zip(&bases) {
        let base = *base as usize;
        let number = *number as usize;
        if number < 2 {
            return Err(Fault::new(format!("letter side {number} is below two.")));
        }
        let code = checked(code, dimension, base)?;
        out.push(MagicLayer::new(Bang::new(code, dimension, base), number));
    }
    Ok(out)
}

fn side_of(layers: &[MagicLayer]) -> Result<usize, Fault> {
    let side = word::side(layers)?;
    usize::try_from(side).map_err(|_| Fault::new(format!("side {side} is past what a page draws.")))
}

fn fits(layers: &[MagicLayer], budget: usize) -> Result<usize, Fault> {
    let side = side_of(layers)?;
    if side > budget {
        return Err(Fault::new(format!(
            "side {side} is more than the {budget} this page draws; drop a letter or use a prefix."
        )));
    }
    Ok(side)
}

fn drawn(layers: &[MagicLayer], budget: usize) -> Result<Tensor, Fault> {
    fits(layers, budget)?;
    Ok(magic(layers)?)
}

// PLANE

/// Builds the plane word as a byte grid, one byte per site.
#[wasm_bindgen]
pub fn magic_grid(codes: Vec<String>, numbers: Vec<u32>, bases: Vec<u32>) -> Result<Grid, Fault> {
    let tile = drawn(&letters(codes, numbers, 2, bases)?, PLANE_SIDE)?;
    Ok(Grid {
        width: tile.shape[1] as u32,
        height: tile.shape[0] as u32,
        types: tile.bytes().to_vec(),
    })
}

/// Paints the plane word: filled sites black, empty sites white.
#[wasm_bindgen]
pub fn magic_pixels(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
) -> Result<Pixels, Fault> {
    let tile = drawn(&letters(codes, numbers, 2, bases)?, PLANE_SIDE)?;
    let cell = Cell2d::new(tile).paint(&mapping(), Mode::Type);
    let (width, height) = (cell.width(), cell.height());
    Ok(Pixels::of(
        width,
        height,
        cell.cell.colors.unwrap_or_default(),
    ))
}

// SOLID

/// Packs the exposed faces of the solid word: two section lengths, then six floats per vertex,
/// position and normal, in the unit box.
#[wasm_bindgen]
pub fn magic_faces(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
) -> Result<Vec<f32>, Fault> {
    let tile = drawn(&letters(codes, numbers, 3, bases)?, SOLID_SIDE)?;
    let mut pack = Pack::new();
    for quad in quads(&Cell3d::new(tile)) {
        pack.quad(quad.verts, quad.normal);
    }
    Ok(pack.buffer())
}

/// Lists the filled sites of the solid word as x, y, z triples.
#[wasm_bindgen]
pub fn magic_cells(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
) -> Result<Vec<u32>, Fault> {
    let tile = drawn(&letters(codes, numbers, 3, bases)?, SOLID_SIDE)?;
    let (cols, deep) = (tile.shape[1], tile.shape[2]);
    let mut out = Vec::new();
    for (flat, &site) in tile.bytes().iter().enumerate() {
        if site != 0 {
            out.extend([
                (flat / (cols * deep)) as u32,
                (flat / deep % cols) as u32,
                (flat % deep) as u32,
            ]);
        }
    }
    Ok(out)
}

/// Counts the exposed faces of the solid word, as a decimal string.
#[wasm_bindgen]
pub fn magic_surface(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
) -> Result<String, Fault> {
    let tile = drawn(&letters(codes, numbers, 3, bases)?, SOLID_SIDE)?;
    let (rows, cols, deep) = (tile.shape[0], tile.shape[1], tile.shape[2]);
    let bytes = tile.bytes();
    let mut faces = 0u128;
    for i in 0..rows {
        for j in 0..cols {
            for k in 0..deep {
                if bytes[(i * cols + j) * deep + k] == 0 {
                    continue;
                }
                faces += 6;
                if i > 0 && bytes[((i - 1) * cols + j) * deep + k] != 0 {
                    faces -= 2;
                }
                if j > 0 && bytes[(i * cols + j - 1) * deep + k] != 0 {
                    faces -= 2;
                }
                if k > 0 && bytes[(i * cols + j) * deep + k - 1] != 0 {
                    faces -= 2;
                }
            }
        }
    }
    Ok(faces.to_string())
}

/// Counts the exposed edges of the plane word, the perimeter of its filled sites, as a decimal string.
#[wasm_bindgen]
pub fn magic_perimeter(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
) -> Result<String, Fault> {
    let tile = drawn(&letters(codes, numbers, 2, bases)?, PLANE_SIDE)?;
    Ok(mrlymath::two::census::perimeter(&Cell2d::new(tile)).to_string())
}

// HEXAGON

fn hexed(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
    projection: &str,
) -> Result<Cell6d, Fault> {
    let tile = drawn(&letters(codes, numbers, 3, bases)?, HEX_SIDE)?;
    let cell = Cell3d::new(tile);
    Ok(match projection {
        "pro" => six::pro(&cell)?,
        "cut" => six::cut(&cell)?,
        _ => six::iso(&cell)?,
    })
}

/// Renders the hexagonal projection of the solid word, iso, pro or cut, as SVG at the scale.
#[wasm_bindgen]
pub fn magic_hex(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
    projection: &str,
    scale: usize,
) -> Result<String, Fault> {
    Ok(six::svg(
        &hexed(codes, numbers, bases, projection)?,
        scale,
        None,
        0,
    )?)
}

/// Tallies the hexagonal projection of the solid word: its side, its mesh, its fill and the boundary edges of that fill, as JSON.
#[wasm_bindgen]
pub fn magic_hex_census(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
    projection: &str,
) -> Result<String, Fault> {
    let cell = six::skin(&hexed(codes, numbers, bases, projection)?);
    let tally = six::census(&cell, false);
    let rim = six::census::fills_only(&cell);
    Ok(json!({
        "projection": projection,
        "grid": [cell.width(), cell.height()],
        "triangles": tally.triangles,
        "fills": tally.fills,
        "voids": tally.voids,
        "boundary": tally.boundary_edges,
        "edges": tally.edges,
        "vertices": tally.vertices,
        "euler": tally.euler,
        "exposed": rim.boundary_edges,
        "ratio": tally.fills as f64 / tally.triangles.max(1) as f64,
    })
    .to_string())
}

// CENSUS

fn pieces(tile: &Tensor) -> u128 {
    let (rows, cols) = (tile.shape[0], tile.shape[1]);
    let bytes = tile.bytes();
    let mut seen = vec![false; rows * cols];
    let mut count = 0u128;
    let mut stack: Vec<usize> = Vec::new();
    for start in 0..rows * cols {
        if bytes[start] == 0 || seen[start] {
            continue;
        }
        count += 1;
        seen[start] = true;
        stack.push(start);
        while let Some(at) = stack.pop() {
            let (r, c) = (at / cols, at % cols);
            let mut steps: Vec<usize> = Vec::new();
            if r > 0 {
                steps.push(at - cols);
            }
            if r + 1 < rows {
                steps.push(at + cols);
            }
            if c > 0 {
                steps.push(at - 1);
            }
            if c + 1 < cols {
                steps.push(at + 1);
            }
            for next in steps {
                if bytes[next] != 0 && !seen[next] {
                    seen[next] = true;
                    stack.push(next);
                }
            }
        }
    }
    count
}

fn recipe(layers: &[MagicLayer]) -> Tile {
    let mut tile = Tile::new(Group::Magic);
    tile.sources = layers
        .iter()
        .map(|layer| Source::Code(layer.design.code))
        .collect();
    tile.numbers = layers.iter().map(|layer| layer.number).collect();
    tile.levels = vec![1; layers.len()];
    tile.rotations = vec![0; layers.len()];
    tile.anti = vec![false; layers.len()];
    tile.resize();
    tile
}

fn count_pieces(layers: &[MagicLayer], dimension: usize) -> (Option<u128>, &'static str) {
    if let Ok(closed) = word::components(layers) {
        return (Some(closed), "closed");
    }
    if dimension != 2 {
        return (None, "");
    }
    let cells = word::side(layers)
        .ok()
        .and_then(|side| side.checked_mul(side));
    match cells {
        Some(total) if total <= DRAWN_CELLS => match magic(layers) {
            Ok(tile) => (Some(pieces(&tile)), "drawn"),
            Err(_) => (None, ""),
        },
        _ => (None, ""),
    }
}

/// Tallies a word: side, cells, fill, voids, density, dimension, components, the letter list,
/// and the constant, periodic and composite flags, as JSON.
///
/// Every count but the component one is a product over the letters, so the census answers at any
/// length even where the raster is capped.
#[wasm_bindgen]
pub fn magic_census(
    codes: Vec<String>,
    numbers: Vec<u32>,
    dimension: usize,
    bases: Vec<u32>,
) -> Result<String, Fault> {
    let layers = letters(codes, numbers, dimension, bases)?;
    let side = word::side(&layers)?;
    let fill = word::fill(&layers)?;
    let fills = word::fills(&layers)?;
    let cells = side
        .checked_pow(dimension as u32)
        .ok_or_else(|| Fault::new("that word holds more cells than a u128 counts."))?;
    let period = word::period(&layers);
    let native = word::native(&layers);
    let uniform_base = layers
        .iter()
        .all(|l| l.design.base == layers[0].design.base);
    let (components, route) = count_pieces(&layers, dimension);
    let list: Vec<Json> = layers
        .iter()
        .zip(&fills)
        .map(|(layer, count)| {
            json!({
                "code": layer.design.code.to_string(),
                "number": layer.number,
                "base": layer.design.base,
                "name": Bang::new(layer.design.code, dimension, layer.design.base).to_str(),
                "fill": count.to_string(),
                "cells": (layer.number as u128).pow(dimension as u32).to_string(),
                "dimension": (*count as f64).ln() / (layer.number as f64).ln(),
                "native": layer.number == layer.design.base,
            })
        })
        .collect();
    Ok(json!({
        "length": layers.len(),
        "side": side.to_string(),
        "cells": cells.to_string(),
        "fill": fill.to_string(),
        "voids": (cells - fill).to_string(),
        "ratio": fill as f64 / cells as f64,
        "dimension": word::dimension(&layers)?,
        "period": period,
        "constant": recipe(&layers).degenerate() && uniform_base,
        "periodic": period < layers.len(),
        "native": native,
        "composite": period < layers.len() && native,
        "residue_base": if native {
            layers.iter().map(|l| l.design.base).product::<usize>().to_string()
        } else {
            String::new()
        },
        "components": components.map(|count| count.to_string()).unwrap_or_default(),
        "counted": route,
        "letters": list,
    })
    .to_string())
}

/// Returns the longest prefix of the sides whose product still fits the budget, at least one.
///
/// A prefix render is the box cover of the whole word at that scale, never a shallower word.
#[wasm_bindgen]
pub fn magic_cap(numbers: Vec<u32>, dimension: usize, budget: usize) -> Result<usize, Fault> {
    if !(2..=3).contains(&dimension) {
        return Err(Fault::new("a word draws in the plane or in the cube."));
    }
    let mut side = 1usize;
    let mut taken = 0usize;
    for number in numbers {
        match side.checked_mul(number as usize) {
            Some(next) if next <= budget => {
                side = next;
                taken += 1;
            }
            _ => break,
        }
    }
    Ok(taken.max(1))
}

// PRESS

/// Counts the members of a word's design from its letter fills, without enumeration.
#[wasm_bindgen]
pub fn word_count(
    codes: Vec<String>,
    numbers: Vec<u32>,
    dimension: usize,
    bases: Vec<u32>,
) -> Result<String, Fault> {
    Ok(press::word_count(&letters(codes, numbers, dimension, bases)?)?.to_string())
}

/// Lists every member of a word's design in ascending order, each as a decimal string.
#[wasm_bindgen]
pub fn word_members(
    codes: Vec<String>,
    numbers: Vec<u32>,
    dimension: usize,
    bases: Vec<u32>,
) -> Result<Vec<String>, Fault> {
    let layers = letters(codes, numbers, dimension, bases)?;
    let count = press::word_count(&layers)?;
    if count > 4096 {
        return Err(Fault::new(format!(
            "{count} members is more than this page lists; shorten the word."
        )));
    }
    Ok(press::word_members(&layers)?
        .iter()
        .map(|m| m.to_string())
        .collect())
}

/// Returns whether the number lies in the word's design, read in the word's mixed radix.
#[wasm_bindgen]
pub fn word_member(
    codes: Vec<String>,
    numbers: Vec<u32>,
    dimension: usize,
    bases: Vec<u32>,
    number: &str,
) -> Result<bool, Fault> {
    let layers = letters(codes, numbers, dimension, bases)?;
    let value = number
        .trim()
        .parse()
        .map_err(|_| Fault::new(format!("number {number:?} is not a whole number.")))?;
    Ok(press::word_member(&layers, value)?)
}

/// Returns the diagonal profile of a word by the substitution product, each count a decimal string.
#[wasm_bindgen]
pub fn word_profile(
    codes: Vec<String>,
    numbers: Vec<u32>,
    dimension: usize,
    bases: Vec<u32>,
) -> Result<Vec<String>, Fault> {
    let layers = letters(codes, numbers, dimension, bases)?;
    let side = word::side(&layers)?;
    let heights = (dimension as u128) * (side - 1) + 1;
    if heights > 100_000 {
        return Err(Fault::new(format!(
            "{heights} diagonal heights is more than this page reads; shorten the word."
        )));
    }
    Ok(press::word_profile(&layers)?
        .iter()
        .map(|count| count.to_string())
        .collect())
}

// NAMES

/// Prints the canonical name of a plane word at base two.
#[wasm_bindgen]
pub fn magic_name(codes: Vec<String>, numbers: Vec<u32>) -> Result<String, Fault> {
    let layers = letters(codes, numbers.clone(), 2, vec![2; numbers.len()])?;
    let spelt: Vec<(u128, usize)> = layers
        .iter()
        .map(|layer| (layer.design.code, layer.number))
        .collect();
    Ok(Word::new(&spelt)?.to_str())
}

/// Parses a plane word name back into its codes and sides, as JSON.
#[wasm_bindgen]
pub fn magic_parse(text: &str) -> Result<String, Fault> {
    let word = Word::from_str(text)?;
    Ok(json!({
        "codes": word.letters.iter().map(|(code, _)| code.to_string()).collect::<Vec<String>>(),
        "numbers": word.letters.iter().map(|(_, side)| *side).collect::<Vec<usize>>(),
    })
    .to_string())
}

// RATES

/// Charts the prefix rates of a schedule over the word's first two letters, in log two units.
///
/// It returns the component rate and the fill rate at every prefix length, the same pair along the
/// periodic control at the same letter frequencies, the constant-word functional the schedule
/// predicts, and the interior-frequency exponent the fill law gives.
#[wasm_bindgen]
pub fn magic_rates(
    codes: Vec<String>,
    numbers: Vec<u32>,
    bases: Vec<u32>,
    schedule: &str,
    length: usize,
) -> Result<String, Fault> {
    let layers = letters(codes, numbers, 2, bases)?;
    let kind = word::Schedule::parse(schedule)?;
    let pair = (layers[0], layers[1]);
    let spelt = word::spell(kind, pair, length.clamp(2, 120));
    let control = word::spell(word::Schedule::Periodic, pair, length.clamp(2, 120));
    let mut rows = word::rates(&spelt)?;
    let mut mirror = word::rates(&control)?;
    let take = rows.len().min(mirror.len());
    rows.truncate(take);
    mirror.truncate(take);
    let fills = word::fills(&[pair.0, pair.1])?;
    let (first, second) = kind.frequencies();
    let limit = first * (fills[0] as f64).log2() + second * (fills[1] as f64).log2();
    let alphabet = [pair.0, pair.1].iter().all(|letter| {
        letter.number == 2 && letter.design.base == 2 && (1..=15).contains(&letter.design.code)
    });
    Ok(json!({
        "schedule": schedule,
        "length": rows.len(),
        "letters": [
            Bang::new(pair.0.design.code, 2, pair.0.design.base).to_str(),
            Bang::new(pair.1.design.code, 2, pair.1.design.base).to_str(),
        ],
        "rows": rows.iter().map(|(a, b)| vec![*a, *b]).collect::<Vec<Vec<f64>>>(),
        "control": mirror.iter().map(|(a, _)| *a).collect::<Vec<f64>>(),
        "phi": word::constant_functional(&spelt)?,
        "limit": limit,
        "alphabet": alphabet,
    })
    .to_string())
}

/// Reads the carpet staircase to the depth: its letters, its length and its dimension at every
/// block, beside the flat dimension of the constant word its first letter spells.
#[wasm_bindgen]
pub fn magic_staircase(depth: usize) -> Result<String, Fault> {
    if !(1..=8).contains(&depth) {
        return Err(Fault::new("the staircase runs from one block to eight."));
    }
    let mut rows = Vec::new();
    for step in 1..=depth {
        let block = word::staircase(step)?;
        rows.push(json!({
            "blocks": step,
            "length": block.len(),
            "dimension": word::dimension(&block)?,
            "sides": block.iter().map(|layer| layer.number).collect::<Vec<usize>>(),
        }));
    }
    let one = word::staircase(1)?;
    Ok(json!({
        "rows": rows,
        "constant": word::dimension(&[one[0], one[0]])?,
    })
    .to_string())
}
