use crate::{checked, code_of, Fault};
use mrlycore::{json, Json};
use mrlymath::formulas::six as formulas;
use mrlymath::six::{self, Cell6d};
use mrlymath::three;
use mrlynum::boolean;
use wasm_bindgen::prelude::*;

/// Projects the cube the code names to a hexagon, iso, pro or cut, and renders it as SVG at the scale.
#[wasm_bindgen]
pub fn hex_svg(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    projection: &str,
    scale: usize,
) -> Result<String, Fault> {
    let code = code_of(code)?;
    let cell = match projection {
        "pro" => six::pro_design(code, number, level, base)?,
        "cut" => six::cut_design(code, number, level, base)?,
        _ => six::iso_design(code, number, level, base)?,
    };
    Ok(six::svg(&cell, scale, None, 0)?)
}

// SLICE

fn slice(code: &str, number: usize, level: usize, base: usize) -> Result<Cell6d, Fault> {
    Ok(six::cut(&three::create(
        code_of(code)?,
        number,
        level,
        base,
    )?)?)
}

/// Tallies the diagonal section of the cube the code names: the mesh, the fill, its pieces and holes, and the solid closed forms at that side, as JSON.
#[wasm_bindgen]
pub fn slice_census(code: &str, number: usize, level: usize, base: usize) -> Result<String, Fault> {
    let cell = slice(code, number, level, base)?;
    let tally = six::census(&cell, false);
    let side = number.pow(level as u32);
    Ok(json!({
        "side": side,
        "triangles": tally.triangles,
        "boundary": tally.boundary_edges,
        "edges": tally.edges,
        "interior": tally.interior_edges,
        "vertices": tally.vertices,
        "euler": tally.euler,
        "fills": tally.fills,
        "voids": tally.voids,
        "components": six::components(&cell)?,
        "holes": six::holes(&cell)?,
        "giant": six::giant(&cell)?,
        "closed": {
            "triangles": formulas::solid_slice_triangles(side)?.to_string(),
            "boundary": formulas::solid_slice_boundary(side)?.to_string(),
            "edges": formulas::solid_slice_edges(side)?.to_string(),
            "vertices": formulas::solid_slice_vertices(side)?.to_string(),
        },
    })
    .to_string())
}

/// Walks the level-one slice of the code at odd side `2k-1`, one row per `k`, as JSON.
#[wasm_bindgen]
pub fn slice_series(code: &str, max_k: usize) -> Result<String, Fault> {
    if !(1..=16).contains(&max_k) {
        return Err(Fault::new("max_k must be between 1 and 16."));
    }
    let code = code_of(code)?;
    let mut rows = Vec::new();
    for k in 1..=max_k {
        let number = 2 * k - 1;
        let cell = six::cut(&three::create(code, number, 1, 2)?)?;
        rows.push(json!({
            "k": k,
            "n": number,
            "fills": six::census(&cell, false).fills,
            "components": six::components(&cell)?,
            "holes": six::holes(&cell)?,
        }));
    }
    Ok(json!(rows).to_string())
}

/// Splits the level-one hexagon of the side between the carpet and the net: their filled triangles, the two together, the hexagon's triangles and whether the partition is exact, as JSON.
#[wasm_bindgen]
pub fn slice_partition(number: usize) -> Result<String, Fault> {
    let carpet = six::census(&slice("23", number, 1, 2)?, false);
    let net = six::census(&slice("232", number, 1, 2)?, false);
    let together = carpet.fills + net.fills;
    Ok(json!({
        "carpet": carpet.fills,
        "net": net.fills,
        "together": together,
        "hexagon": carpet.triangles,
        "exact": together == carpet.triangles,
    })
    .to_string())
}

// SPECTROMETER

fn sixteenths(walsh: &[i64]) -> Vec<i64> {
    walsh
        .iter()
        .enumerate()
        .map(|(mask, &value)| if mask == 0 { 8 - value } else { -value })
        .collect()
}

fn level_sums(parts: &[i64]) -> [i64; 4] {
    let mut sums = [0i64; 4];
    for (mask, &part) in parts.iter().enumerate() {
        sums[mask.count_ones() as usize] += part;
    }
    sums
}

fn ink_numerator(sums: &[i64; 4], number: i64) -> i64 {
    let sign = if number % 4 == 1 { -1 } else { 1 };
    (6 * sums[0] - 3 * sums[3] * sign) * number * number
        + (4 * sums[1] - 2 * sums[2] * sign) * number
        + 4 * sums[2]
        - (2 * sums[1] + 3 * sums[3]) * sign
}

/// Reads the Walsh spectrum of the cube design the code names, its four level sums, and the exact diagonal-slice ink those sums set at every odd side `2k-1`, as JSON.
#[wasm_bindgen]
pub fn walsh_spectrum(code: &str, max_k: usize) -> Result<String, Fault> {
    if !(1..=16).contains(&max_k) {
        return Err(Fault::new("max_k must be between 1 and 16."));
    }
    let code = checked(code, 3, 2)?;
    let walsh = boolean::walsh_spectrum(code, 3);
    let parts = sixteenths(&walsh);
    let sums = level_sums(&parts);
    let mut weights = [0u32; 4];
    for corner in 0..8usize {
        if (code >> corner) & 1 == 1 {
            weights[corner.count_ones() as usize] += 1;
        }
    }
    let coefficients: Vec<Json> = (0..8usize)
        .map(|mask| {
            json!({
                "mask": mask,
                "level": mask.count_ones(),
                "walsh": walsh[mask],
                "sixteenths": parts[mask],
                "value": parts[mask] as f64 / 16.0,
            })
        })
        .collect();
    let levels: Vec<Json> = (0..4usize)
        .map(|level| {
            json!({
                "level": level,
                "sixteenths": sums[level],
                "eighths": sums[level] / 2,
                "sigma": sums[level] as f64 / 16.0,
            })
        })
        .collect();
    let law: Vec<Json> = (1..=max_k)
        .map(|k| {
            let number = 2 * k as i64 - 1;
            let numerator = ink_numerator(&sums, number);
            json!({
                "k": k,
                "n": number,
                "s": if number % 4 == 1 { -1 } else { 1 },
                "numerator": numerator,
                "denominator": 96 * number * number,
                "ink": numerator as f64 / (96 * number * number) as f64,
                "fills": numerator / 16,
                "triangles": 6 * number * number,
            })
        })
        .collect();
    Ok(json!({
        "code": code.to_string(),
        "corners": code.count_ones(),
        "background": sums[0] as f64 / 16.0,
        "blink": -(sums[3] as f64) / 32.0,
        "spectrum": walsh,
        "coefficients": coefficients,
        "levels": levels,
        "weights": weights.to_vec(),
        "law": law,
    })
    .to_string())
}
