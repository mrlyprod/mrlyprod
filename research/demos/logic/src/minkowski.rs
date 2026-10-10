use crate::Fault;
use mrlyrs::core::json;
use mrlyrs::math::three::sponge;
use wasm_bindgen::prelude::*;

const WIDEST: usize = 729;
const PERIODS: usize = 12;
const STEPS: usize = 256;

/// Reads the distance from every pixel centre of the plane `z = height` to the Menger sponge, `side` by `side`, row-major.
///
/// Each distance is `mrlyrs::math::three::sponge::distance`, exact: the digits descend to the
/// plus a point sits in, then the twelve edges of its centre cube or the four carpets walling its
/// arm. At the midplane the centre pixel of an odd side reads `sqrt(2)/6`. The side is refused
/// past 729.
#[wasm_bindgen]
pub fn minkowski_slice(height: f64, side: usize) -> Result<Vec<f32>, Fault> {
    if side == 0 || side > WIDEST {
        return Err(Fault::new(format!(
            "the slice runs from 1 to {WIDEST} pixels a side, not {side}."
        )));
    }
    if !(0.0..=1.0).contains(&height) {
        return Err(Fault::new("the plane must cut the unit cube."));
    }
    let scale = side as f64;
    let mut out = Vec::with_capacity(side * side);
    for row in 0..side {
        for col in 0..side {
            let point = [
                (col as f64 + 0.5) / scale,
                (row as f64 + 0.5) / scale,
                height,
            ];
            out.push(sponge::distance(point) as f32);
        }
    }
    Ok(out)
}

/// Reads the sponge at one radius as JSON: the tube `T` inside the plus, the volume inside the cube, the Minkowski reading and the periodic profile `p`, each `null` where no closed form reaches.
#[wasm_bindgen]
pub fn minkowski_read(radius: f64) -> String {
    json!({
        "radius": radius,
        "tube": sponge::tube(radius),
        "volume": sponge::volume(radius),
        "reading": sponge::reading(radius),
        "profile": sponge::profile(radius),
    })
    .to_string()
}

/// Walks the Minkowski reading and the profile down `periods` factors of 3 from `sqrt(2)/6`, `steps` radii a period, as JSON.
///
/// The radii run evenly in `u = ln(1/r)`, each at the middle of its step, so none lands on the
/// edge of a window; `reading` and `profile` are `null` on the phases
/// `(1/6, sqrt(2)/6]` of every period, where the tube has no closed form. `low`, `high` and
/// `swing` are the extremes of `p` over the walked phases and their gap in percent of `low`,
/// and `dimension`, `edge` and `cover` are `log(20)/log(3)`, `1/6` and `sqrt(2)/6`.
#[wasm_bindgen]
pub fn minkowski_walk(periods: usize, steps: usize) -> Result<String, Fault> {
    if !(1..=PERIODS).contains(&periods) || !(2..=STEPS).contains(&steps) {
        return Err(Fault::new(format!(
            "the walk takes 1 to {PERIODS} periods of 2 to {STEPS} steps."
        )));
    }
    let start = (1.0 / sponge::COVER).ln();
    let period = 3f64.ln();
    let at = |i: usize| start + period * (i as f64 + 0.5) / steps as f64;
    let once: Vec<Option<f64>> = (0..steps)
        .map(|i| sponge::profile((-at(i)).exp()))
        .collect();
    let edge = sponge::profile(sponge::EDGE);
    let seen: Vec<f64> = once.iter().chain([&edge]).flatten().copied().collect();
    let low = seen.iter().copied().fold(f64::INFINITY, f64::min);
    let high = seen.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let count = periods * steps;
    let u: Vec<f64> = (0..count).map(at).collect();
    let radius: Vec<f64> = u.iter().map(|v| (-v).exp()).collect();
    let reading: Vec<Option<f64>> = radius.iter().map(|&r| sponge::reading(r)).collect();
    let profile: Vec<Option<f64>> = (0..count).map(|i| once[i % steps]).collect();
    Ok(json!({
        "dimension": sponge::dimension(),
        "edge": sponge::EDGE,
        "cover": sponge::COVER,
        "start": start,
        "period": period,
        "u": u,
        "radius": radius,
        "reading": reading,
        "profile": profile,
        "low": low,
        "high": high,
        "swing": 100.0 * (high - low) / low,
    })
    .to_string())
}
