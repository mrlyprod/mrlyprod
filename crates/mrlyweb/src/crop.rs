#![allow(clippy::too_many_arguments)]

use crate::{code_of, Fault, Grid};
use mrlycore::json;
use mrlycore::tensor::Tensor;
use mrlymath::bang::factory;
use mrlymath::shape::{self, Frac, Shape};
use mrlymath::space::Pack;
use mrlymath::three::{self, Cell3d};
use wasm_bindgen::prelude::*;

fn radius_of(rnum: u32, rden: u32) -> Result<Frac, Fault> {
    if rden == 0 {
        return Err(Fault::new("the radius denominator must be at least 1."));
    }
    Ok(Frac::new(rnum as i64, rden as i64))
}

fn shape_of(
    name: &str,
    dimension: usize,
    rnum: u32,
    rden: u32,
    anti: bool,
) -> Result<Shape, Fault> {
    let core = shape::named(name, dimension, radius_of(rnum, rden)?)?;
    Ok(if anti {
        Shape::Anti(Box::new(core))
    } else {
        core
    })
}

fn keep_of(policy: &str) -> Result<bool, Fault> {
    match policy {
        "inside" => Ok(false),
        "touching" => Ok(true),
        _ => Err(Fault::new(format!(
            "policy {policy:?} is not inside or touching."
        ))),
    }
}

fn design(
    code: &str,
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor, Fault> {
    Ok(factory::create(
        code_of(code)?,
        number,
        dimension,
        base,
        level,
    )?)
}

/// Lists the named shapes a crop can take in the dimension, as a JSON array.
#[wasm_bindgen]
pub fn crop_shapes(dimension: usize) -> String {
    json!(shape::shapes(dimension)).to_string()
}

/// Crops the flat design to the shape as a byte grid: inside and touching keep coarse cells, refined1 and refined2 rebuild the rim on a finer lattice.
#[wasm_bindgen]
pub fn crop_grid(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
    policy: &str,
) -> Result<Grid, Fault> {
    let types = design(code, number, 2, base, level)?;
    let shape = shape_of(shape, 2, rnum, rden, anti)?;
    let kept = match policy {
        "refined1" => shape::refine(&types, &shape, number, 1, false)?,
        "refined2" => shape::refine(&types, &shape, number, 2, false)?,
        _ => shape::crop(&types, &shape, keep_of(policy)?),
    };
    Ok(Grid {
        width: kept.shape[1] as u32,
        height: kept.shape[0] as u32,
        types: kept.bytes().to_vec(),
    })
}

/// Lists the filled cells of the cube design kept by the shape as x, y, z triples.
#[wasm_bindgen]
pub fn crop_cells(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
    policy: &str,
) -> Result<Vec<u32>, Fault> {
    let kept = cropped_cube(code, number, level, base, shape, rnum, rden, anti, policy)?;
    let grid = kept.types();
    let mut out = Vec::new();
    for (flat, &site) in grid.bytes().iter().enumerate() {
        if site != 0 {
            let (i, rest) = (
                flat / (grid.shape[1] * grid.shape[2]),
                flat % (grid.shape[1] * grid.shape[2]),
            );
            out.extend([
                i as u32,
                (rest / grid.shape[2]) as u32,
                (rest % grid.shape[2]) as u32,
            ]);
        }
    }
    Ok(out)
}

/// Packs the exposed faces of the cropped cube, capping the cut with correct normals: two section lengths, then six floats per vertex, position and normal.
#[wasm_bindgen]
pub fn crop_faces(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
    policy: &str,
) -> Result<Vec<f32>, Fault> {
    let kept = cropped_cube(code, number, level, base, shape, rnum, rden, anti, policy)?;
    let mut pack = Pack::new();
    for quad in three::quads(&kept) {
        pack.quad(quad.verts, quad.normal);
    }
    Ok(pack.buffer())
}

fn cropped_cube(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
    policy: &str,
) -> Result<Cell3d, Fault> {
    let types = design(code, number, 3, base, level)?;
    let shape = shape_of(shape, 3, rnum, rden, anti)?;
    Ok(Cell3d::new(shape::crop(&types, &shape, keep_of(policy)?)))
}

/// Tallies the design against the shape: cells and fills per region, and the exposed measure before and after the touching crop, as JSON.
#[wasm_bindgen]
pub fn crop_census(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    dimension: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
) -> Result<String, Fault> {
    let types = design(code, number, dimension, base, level)?;
    let shape = shape_of(shape, dimension, rnum, rden, anti)?;
    let tally = shape::census(&shape, &types);
    let after = shape::crop(&types, &shape, true);
    Ok(json!({
        "cells_out": tally.cells[0],
        "cells_cut": tally.cells[1],
        "cells_in": tally.cells[2],
        "filled_out": tally.filled[0],
        "filled_cut": tally.filled[1],
        "filled_in": tally.filled[2],
        "exposed_before": mrlynum::census::exposed(&types).to_string(),
        "exposed_after": mrlynum::census::exposed(&after).to_string(),
    })
    .to_string())
}

const SERIES_CELLS: usize = 1_000_000;

fn series_guard(number: usize, dimension: usize, level: usize, steps: usize) -> Result<(), Fault> {
    let stop = || Fault::new(format!("the series would build over {SERIES_CELLS} cells."));
    if !(1..=64).contains(&steps) {
        return Err(Fault::new("steps must be between 1 and 64."));
    }
    let side = number.checked_pow(level as u32).ok_or_else(stop)?;
    let cells = side.checked_pow(dimension as u32).ok_or_else(stop)?;
    if cells > SERIES_CELLS {
        return Err(stop());
    }
    Ok(())
}

/// Sweeps the crop along one axis for charts: level walks the depth at the radius, radius walks the fraction 1 over steps to 1 at the level; each entry carries x, filled_in, filled_cut and exposed_after, as JSON.
#[wasm_bindgen]
pub fn crop_series(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    dimension: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
    axis: &str,
    steps: usize,
) -> Result<String, Fault> {
    let entry = |x: f64, types: &Tensor, s: &Shape| {
        let tally = shape::census(s, types);
        let after = shape::crop(types, s, true);
        json!({
            "x": x,
            "filled_in": tally.filled[2],
            "filled_cut": tally.filled[1],
            "exposed_after": mrlynum::census::exposed(&after).to_string(),
        })
    };
    let mut rows = Vec::new();
    match axis {
        "level" => {
            series_guard(number, dimension, steps, steps)?;
            let s = shape_of(shape, dimension, rnum, rden, anti)?;
            for depth in 0..=steps {
                let types = if depth == 0 {
                    Tensor::full(vec![1; dimension], 1)
                } else {
                    design(code, number, dimension, base, depth)?
                };
                rows.push(entry(depth as f64, &types, &s));
            }
        }
        "radius" => {
            series_guard(number, dimension, level, steps)?;
            let types = design(code, number, dimension, base, level)?;
            for num in 1..=steps {
                let s = shape_of(shape, dimension, num as u32, steps as u32, anti)?;
                rows.push(entry(num as f64 / steps as f64, &types, &s));
            }
        }
        _ => return Err(Fault::new(format!("axis {axis:?} is not level or radius."))),
    }
    Ok(json!(rows).to_string())
}

fn outline(name: &str, r: Frac) -> Option<Vec<[Frac; 2]>> {
    let h = Frac::new(1, 2);
    let q = r * h;
    let m = Frac::whole(0) - r;
    let mq = Frac::whole(0) - q;
    let shifted = |points: Vec<[Frac; 2]>| points.iter().map(|[a, b]| [h + *a, h + *b]).collect();
    match name {
        "box" => Some(shifted(vec![[m, m], [m, r], [r, r], [r, m]])),
        "diamond" => Some(shifted(vec![
            [m, Frac::whole(0)],
            [Frac::whole(0), r],
            [r, Frac::whole(0)],
            [Frac::whole(0), m],
        ])),
        "triangle" => Some(shifted(vec![[m, Frac::whole(0)], [r, m], [r, r]])),
        "octagon" => Some(shifted(vec![
            [m, mq],
            [m, q],
            [mq, r],
            [q, r],
            [r, q],
            [r, mq],
            [q, m],
            [mq, m],
        ])),
        _ => None,
    }
}

/// Draws the flat crop as SVG: the touched cells as rects, trimmed to the exact shape by a clip path, or by a mask when the crop is an anti-crop.
#[wasm_bindgen]
pub fn crop_svg(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
    scale: usize,
) -> Result<String, Fault> {
    let types = design(code, number, 2, base, level)?;
    let cropper = shape_of(shape, 2, rnum, rden, anti)?;
    let kept = shape::crop(&types, &cropper, true);
    let r = radius_of(rnum, rden)?;
    let side = kept.shape[0];
    let span = side * scale;
    let px = |f: Frac| f.num as f64 * span as f64 / f.den as f64;
    let element = match outline(shape, r) {
        Some(points) => {
            let listed: Vec<String> = points
                .iter()
                .map(|[a0, a1]| format!("{},{}", px(*a1), px(*a0)))
                .collect();
            format!("<polygon points=\"{}\"", listed.join(" "))
        }
        None => {
            let centre = px(Frac::new(1, 2));
            format!("<circle cx=\"{centre}\" cy=\"{centre}\" r=\"{}\"", px(r))
        }
    };
    let mut out = vec![format!(
        "<svg width=\"{span}\" height=\"{span}\" xmlns=\"http://www.w3.org/2000/svg\">"
    )];
    if anti {
        out.push(format!(
            "<mask id=\"crop\"><rect width=\"{span}\" height=\"{span}\" fill=\"white\"/>{element} fill=\"black\"/></mask>"
        ));
        out.push("<g mask=\"url(#crop)\">".to_string());
    } else {
        out.push(format!("<clipPath id=\"crop\">{element}/></clipPath>"));
        out.push("<g clip-path=\"url(#crop)\">".to_string());
    }
    for a0 in 0..side {
        for a1 in 0..side {
            if kept.get(&[a0, a1]) == 0 {
                continue;
            }
            let (x, y) = (a1 * scale, a0 * scale);
            out.push(format!(
                "<rect x=\"{x}\" y=\"{y}\" width=\"{scale}\" height=\"{scale}\" fill=\"#000\"/>"
            ));
        }
    }
    out.push("</g>".to_string());
    out.push("</svg>".to_string());
    Ok(out.join("\n"))
}

fn holds(shape: &Shape, side: usize, index: &[usize]) -> bool {
    match shape {
        Shape::Ball { center, radius } => {
            if radius.num < 0 {
                return false;
            }
            let l = center.iter().fold(radius.den, |acc, c| {
                acc / mrlynum::classics::gcd(acc as u128, c.den as u128) as i64 * c.den
            });
            let scale = 2 * side as i128 * l as i128;
            let rr = radius.num as i128 * (scale / radius.den as i128);
            let mut gap: i128 = 0;
            for (axis, c) in center.iter().enumerate() {
                let p = (2 * index[axis] as i128 + 1) * l as i128;
                let cc = c.num as i128 * (scale / c.den as i128);
                let d = p - cc;
                gap += d * d;
            }
            gap <= rr * rr
        }
        Shape::Polytope { walls } => walls.iter().all(|wall| {
            let form: i128 = wall
                .normal
                .iter()
                .enumerate()
                .map(|(axis, &n)| n as i128 * (2 * index[axis] as i128 + 1))
                .sum();
            form * wall.offset.den as i128 <= wall.offset.num as i128 * 2 * side as i128
        }),
        Shape::Anti(inner) => !holds(inner, side, index),
    }
}

/// Masks a float field for display: a copy with NaN wherever the cell centre falls outside the kept region.
#[wasm_bindgen]
pub fn field_crop(
    data: &[f32],
    size: usize,
    dimension: usize,
    shape: &str,
    rnum: u32,
    rden: u32,
    anti: bool,
) -> Result<Vec<f32>, Fault> {
    if !(2..=3).contains(&dimension) {
        return Err(Fault::new("the dimension must be 2 or 3."));
    }
    let want = size.checked_pow(dimension as u32).ok_or_else(|| {
        Fault::new("the field must hold size to the dimension samples with size at least 1.")
    })?;
    if size == 0 || data.len() != want {
        return Err(Fault::new(
            "the field must hold size to the dimension samples with size at least 1.",
        ));
    }
    let cropper = shape_of(shape, dimension, rnum, rden, anti)?;
    let mut index = vec![0usize; dimension];
    Ok(data
        .iter()
        .enumerate()
        .map(|(flat, &value)| {
            let mut rem = flat;
            for axis in (0..dimension).rev() {
                index[axis] = rem % size;
                rem /= size;
            }
            if holds(&cropper, size, &index) {
                value
            } else {
                f32::NAN
            }
        })
        .collect())
}
