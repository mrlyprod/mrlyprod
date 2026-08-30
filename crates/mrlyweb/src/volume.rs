use crate::spin::ramp_of;
use crate::{code_of, Fault, Pixels};
use mrlycore::json;
use mrlylab::moire::{self, Combine, Spec, Volume};
use mrlymath::space::Pack;
use mrlymath::three::{self, Cell3d};
use wasm_bindgen::prelude::*;

fn combine_of(name: &str) -> Result<Combine, Fault> {
    match name {
        "sum" => Ok(Combine::Sum),
        "xor" => Ok(Combine::Xor),
        "and" => Ok(Combine::And),
        _ => Err(Fault::new(format!(
            "combine {name:?} is not sum, xor or and."
        ))),
    }
}

fn volume_of(data: &[f32], size: usize) -> Result<Volume, Fault> {
    Ok(Volume::from_data(data.to_vec(), size)?)
}

/// Stacks the cube design the code names at the odd side numbers up to the limit into a volume of the side: the samples, x-major.
#[wasm_bindgen]
pub fn volume(
    code: &str,
    base: usize,
    limit: usize,
    combine: &str,
    level: usize,
    size: usize,
) -> Result<Vec<f32>, Fault> {
    let numbers: Vec<usize> = (1..=limit.max(1)).step_by(2).collect();
    let spec = Spec::new(code_of(code)?, base, 3);
    Ok(moire::volume(spec, &numbers, combine_of(combine)?, level, size)?.data)
}

/// Reads a volume: its smallest, largest and mean sample, as JSON.
#[wasm_bindgen]
pub fn volume_stats(data: &[f32], size: usize) -> Result<String, Fault> {
    let v = volume_of(data, size)?;
    let mean = v.data.iter().map(|&x| x as f64).sum::<f64>() / v.data.len().max(1) as f64;
    Ok(json!({ "min": v.min() as f64, "max": v.max() as f64, "mean": mean }).to_string())
}

/// Counts the voxels at or above the level.
#[wasm_bindgen]
pub fn volume_count(data: &[f32], size: usize, level: f32) -> Result<usize, Fault> {
    Ok(volume_of(data, size)?.count(level))
}

/// Packs the exposed faces of the voxels at or above the level: two section lengths, then six floats per vertex, position and normal, in the unit box.
#[wasm_bindgen]
pub fn volume_faces(data: &[f32], size: usize, level: f32) -> Result<Vec<f32>, Fault> {
    let cell = Cell3d::new(volume_of(data, size)?.solid(level));
    let mut pack = Pack::new();
    for quad in three::quads(&cell) {
        pack.quad(quad.verts, quad.normal);
    }
    Ok(pack.buffer())
}

fn normal_of(normal: &[f64]) -> Result<[f64; 3], Fault> {
    match normal {
        [x, y, z] => Ok([*x, *y, *z]),
        _ => Err(Fault::new("the normal needs three components.")),
    }
}

/// Frames the plane normal to the direction at the offset across the box: its centre, its two axes, its normal and its window width in the unit box, as JSON.
#[wasm_bindgen]
pub fn plane_frame(normal: &[f64], offset: f64) -> Result<String, Fault> {
    let f = moire::frame(normal_of(normal)?, offset)?;
    Ok(json!({
        "centre": f.centre.to_vec(),
        "u": f.u.to_vec(),
        "v": f.v.to_vec(),
        "normal": f.normal.to_vec(),
        "width": f.width,
    })
    .to_string())
}

/// Samples the section of the volume on the plane normal to the direction at the offset: an out by out field, row by row, NaN outside the cube.
#[wasm_bindgen]
pub fn plane_field(
    data: &[f32],
    size: usize,
    normal: &[f64],
    offset: f64,
    out: usize,
) -> Result<Vec<f32>, Fault> {
    if out == 0 {
        return Err(Fault::new("out must be at least 1."));
    }
    let v = volume_of(data, size)?;
    let frame = moire::frame(normal_of(normal)?, offset)?;
    let (values, inside) = v.plane(&frame, out);
    Ok(values
        .iter()
        .zip(inside.iter())
        .map(|(&value, &hit)| if hit == 0 { f32::NAN } else { value })
        .collect())
}

/// Paints a square field through the ramp with the values scaled from low to high into the levels, NaN samples transparent.
#[wasm_bindgen]
pub fn paint_span(
    field: &[f32],
    size: usize,
    low: f32,
    high: f32,
    ramp: &str,
    levels: usize,
    invert: bool,
) -> Result<Pixels, Fault> {
    if size == 0 || field.len() != size * size {
        return Err(Fault::new(
            "the field must be size by size with size at least 1.",
        ));
    }
    let colorizer = ramp_of(ramp);
    let levels = levels.max(2);
    let span = (high - low).max(f32::EPSILON);
    let colors = field
        .iter()
        .map(|&value| {
            if value.is_nan() {
                return [0, 0, 0, 0];
            }
            let t = ((value - low) / span).clamp(0.0, 1.0);
            let t = if invert { 1.0 - t } else { t };
            let bucket = ((t * (levels - 1) as f32).round() as usize).min(levels - 1);
            let c = colorizer.color(bucket + 1, levels);
            [c.r, c.g, c.b, 255]
        })
        .collect();
    Ok(Pixels::of(size, size, colors))
}
