use crate::Fault;
use mrlyrs::core::json;
use mrlyrs::num::sumset::{pairs, Sumset};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

const LOWEST: u32 = 6;
const FIRST: u32 = 6;
const HIGHEST: u32 = 16;
const CELLS: u32 = 4096;

thread_local! {
    static HELD: RefCell<Option<Sumset>> = const { RefCell::new(None) };
}

fn held<T>(level: u32, read: impl FnOnce(&Sumset) -> Result<T, Fault>) -> Result<T, Fault> {
    if !(LOWEST..=HIGHEST).contains(&level) {
        return Err(Fault::new(format!(
            "the height runs from 3^{LOWEST} to 3^{HIGHEST}, not 3^{level}."
        )));
    }
    HELD.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.as_ref().map(Sumset::level) != Some(level) {
            *slot = None;
            *slot = Some(Sumset::new(level)?);
        }
        match slot.as_ref() {
            Some(sumset) => read(sumset),
            None => Err(Fault::new("the sumset was not built.")),
        }
    })
}

/// Reads `S = A + B` at one integer as JSON: `top = 3^level`, `x`, `count = card(S meet [1, x])`, `density = D(x)` and whether `x` is a member.
///
/// `A` holds the integers whose base-3 digits are all `0` or `1`, `B` those whose base-4 digits
/// are; one bit array of `S meet [0, 3^level]` from `mrlyrs::num::sumset::Sumset` is held between
/// calls and rebuilt when the level changes. The level runs from 6 to 16, `x` from 1 to `3^level`.
#[wasm_bindgen]
pub fn sumset_read(level: u32, x: u32) -> Result<String, Fault> {
    held(level, |sumset| {
        let at = u64::from(x);
        let (Some(count), Some(density), Some(member)) =
            (sumset.count(at), sumset.density(at), sumset.contains(at))
        else {
            return Err(Fault::new(format!(
                "x runs from 1 to {}, not {x}.",
                sumset.top()
            )));
        };
        Ok(json!({
            "level": level,
            "top": sumset.top(),
            "x": x,
            "count": count,
            "density": density,
            "member": member,
        })
        .to_string())
    })
}

/// Reads the strip of `S` over the integers `[low, high)`: the share of members in each of `cells` equal runs, one run an integer when `cells = high - low`.
#[wasm_bindgen]
pub fn sumset_strip(level: u32, low: u32, high: u32, cells: u32) -> Result<Vec<f32>, Fault> {
    held(level, |sumset| {
        Ok(sumset
            .fills(u64::from(low), u64::from(high), cells as usize)?
            .into_iter()
            .map(|v| v as f32)
            .collect())
    })
}

/// Reads the least and the greatest `D(x)` over `cells` windows of `[1, 3^level]` spread evenly in `log x`, flat as `low, high` pairs, `NaN` for a window that holds no integer.
///
/// Window `i` runs from `round(3^(level i/cells))` to the next edge, the first from `1` and the
/// last to `3^level` itself, so every dip of `D` shows at its true depth whatever the cell width.
#[wasm_bindgen]
pub fn sumset_envelope(level: u32, cells: u32) -> Result<Vec<f32>, Fault> {
    if !(1..=CELLS).contains(&cells) {
        return Err(Fault::new(format!(
            "the envelope takes 1 to {CELLS} cells, not {cells}."
        )));
    }
    held(level, |sumset| {
        let top = sumset.top();
        let edges: Vec<u64> = (0..=cells)
            .map(|i| match i {
                0 => 1,
                i if i == cells => top + 1,
                i => ((top as f64).powf(f64::from(i) / f64::from(cells)).round() as u64)
                    .clamp(1, top),
            })
            .collect();
        Ok(sumset
            .extremes(&edges)?
            .into_iter()
            .flat_map(|w| match w {
                Some((low, high)) => [low as f32, high as f32],
                None => [f32::NAN, f32::NAN],
            })
            .collect())
    })
}

/// Lists the census pairs `(k, m)` with `k >= 6`, `4^m` within a factor `3` of `3^k` and `d(k, m) <= 3^level`, as JSON rows by `d`.
///
/// Each row holds `three = k`, `four = m`, the scaling `scale = 4^m/3^k`, the largest element
/// `largest = d(k, m)` of `A_k + B_m`, the `gap` `[first, last]` of integers in
/// `(d, min(3^k, 4^m))` that `S` misses or `null`, whether the pair is `clean` and whether it is a
/// gap `copy`, the additive `energy` `E(k, m)` as a decimal string, the energy ratio
/// `ratio = Q(k, m)`, the Cauchy-Schwarz `bound = 1/Q` on the fill, and the `fill`
/// `card(S meet [0, d])/(d + 1)` read off the held bit array.
#[wasm_bindgen]
pub fn sumset_pairs(level: u32) -> Result<String, Fault> {
    held(level, |sumset| {
        let mut rows = Vec::new();
        for pair in pairs(level)?.into_iter().filter(|p| p.three >= FIRST) {
            let d = pair.largest();
            let energy = pair.energy();
            let ratio = pair.ratio(energy);
            let fill = sumset.count(d).map(|c| (c + 1) as f64 / (d + 1) as f64);
            rows.push(json!({
                "three": pair.three,
                "four": pair.four,
                "scale": pair.scale(),
                "largest": d,
                "gap": pair.gap().map(|(a, b)| [a, b]),
                "clean": pair.clean(),
                "copy": pair.copy(),
                "energy": energy.to_string(),
                "ratio": ratio,
                "bound": 1.0 / ratio,
                "fill": fill,
            }));
        }
        Ok(json!(rows).to_string())
    })
}
