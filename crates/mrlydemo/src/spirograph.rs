use crate::Fault;
use mrlycore::{json, Json};
use mrlynum::spirograph as roulette;
use mrlynum::spirograph::{Kind, Pencil, Track};
use wasm_bindgen::prelude::*;

#[allow(clippy::too_many_arguments)]
fn build(
    types: &[u8],
    width: usize,
    height: usize,
    pens: &str,
    track: &str,
    ring: usize,
    wheel: usize,
    sides: usize,
    laps: usize,
    reach: f64,
    jitter: f64,
    seed: u32,
) -> Result<(Track, Vec<Pencil>), Fault> {
    let path = roulette::track(track, ring, wheel, sides, laps)?;
    let pencils = roulette::pencils(types, width, height, pens, reach, jitter, seed)?;
    Ok((path, pencils))
}

/// Traces the roulette of a byte grid taken as the wheel: a pencil on every chosen site (`fill`, `void`, `both` or `corners`), the wheel of radius `wheel` rolling without slipping on the track (`line`, `in`, `out`, `polyin` or `polyout`) laid at radius `ring` with `sides` sides for `laps` laps, the tile scaled to `reach` wheel radii and every seat jittered by up to `jitter` cells under the seed. Pencil by pencil, `samples` points each along the whole track, x then y, in the track's units.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn spirograph(
    types: &[u8],
    width: usize,
    height: usize,
    pens: &str,
    track: &str,
    ring: usize,
    wheel: usize,
    sides: usize,
    laps: usize,
    reach: f64,
    jitter: f64,
    seed: u32,
    samples: usize,
) -> Result<Vec<f32>, Fault> {
    let (path, pencils) = build(
        types, width, height, pens, track, ring, wheel, sides, laps, reach, jitter, seed,
    )?;
    Ok(roulette::trace(&path, &pencils, samples)?)
}

/// Reads the roulette: the pencil count by kind, the classes under the coincidence law, which are the distinct curves on a circle and shapes up to a shift on a line, the generic node count on a circle inside the seat window `0 < |p| < min(1, A)` and null elsewhere, the ring over the wheel in lowest terms, the orbits the track closes after, the picture's rotation order, the wheel turns, the path length, the cell side on the wheel, every seat with its kind, the track outline and whether it closes, and the box the picture sits in, as JSON.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn spirograph_read(
    types: &[u8],
    width: usize,
    height: usize,
    pens: &str,
    track: &str,
    ring: usize,
    wheel: usize,
    sides: usize,
    laps: usize,
    reach: f64,
    jitter: f64,
    seed: u32,
) -> Result<String, Fault> {
    let (path, pencils) = build(
        types, width, height, pens, track, ring, wheel, sides, laps, reach, jitter, seed,
    )?;
    let counts = roulette::seats(&pencils);
    let seats: Vec<Json> = pencils
        .iter()
        .map(|p| {
            let kind = match p.kind {
                Kind::Fill => "fill",
                Kind::Void => "void",
                Kind::Corner => "corner",
            };
            json!([p.x, p.y, kind])
        })
        .collect();
    let outline: Vec<Json> = path.outline.iter().map(|&(x, y)| json!([x, y])).collect();
    Ok(json!({
        "pencils": pencils.len(),
        "fills": counts.fills,
        "voids": counts.voids,
        "corners": counts.corners,
        "distinct": roulette::distinct(&path, &pencils, jitter == 0.0),
        "nodes": roulette::nodes(&path, &pencils, jitter == 0.0),
        "a": path.ratio.0,
        "b": path.ratio.1,
        "orbits": path.orbits,
        "fold": path.fold,
        "turns": path.total / (std::f64::consts::TAU * path.wheel),
        "total": path.total,
        "wheel": path.wheel,
        "side": path.side,
        "cell": roulette::cell(width, height, reach),
        "seats": seats,
        "outline": outline,
        "closed": path.closed,
        "frame": roulette::frame(&path, &pencils),
    })
    .to_string())
}

/// The wheel's centre and its turn in radians a fraction `at` of the way along the track, as `[x, y, turn]`.
#[wasm_bindgen]
pub fn spirograph_pose(
    track: &str,
    ring: usize,
    wheel: usize,
    sides: usize,
    laps: usize,
    at: f64,
) -> Result<Vec<f64>, Fault> {
    let path = roulette::track(track, ring, wheel, sides, laps)?;
    let s = path.total * at.clamp(0.0, 1.0);
    let (x, y) = roulette::pose(&path, s);
    Ok(vec![x, y, roulette::turn(&path, s)])
}

/// The caps: the most pencils a wheel seats, the most points a trace returns, the largest radius, the most laps and the sides a polygon may have, as JSON.
#[wasm_bindgen]
pub fn spirograph_caps() -> String {
    json!({
        "pencils": roulette::PENCIL_CAP,
        "points": roulette::POINT_CAP,
        "radius": roulette::RADIUS_CAP,
        "laps": roulette::LAPS_CAP,
        "sides": [roulette::SIDES.0, roulette::SIDES.1],
    })
    .to_string()
}

/// The shape a roulette walls off inside its own disc: the raster, its numbers and the disc it sits in.
#[wasm_bindgen(getter_with_clone)]
pub struct Cover {
    /// The raster's side in pixels.
    pub side: u32,
    /// The raster row by row, the ordinate falling down the rows: zero outside the disc, one the flood poured from outside, two the flood poured at the centre, three the shape.
    pub mask: Vec<u8>,
    /// The shape's share of the disc's pixels, the walls counted in.
    pub covered: f64,
    /// The centre flood's share of the disc's pixels.
    pub hole: f64,
    /// The share of the disc's pixels the curves themselves mark, the boundary error `covered` carries and loses as the raster grows.
    pub wall: f64,
    /// The mean signed winding number of the disc's pixel centres, read by scanline off the polylines.
    pub winding: f64,
    /// The closed form `winding` converges to: the distinct curves' signed areas summed and divided by the disc's area.
    pub areas: f64,
    /// The disc in the track's units: its centre, the radius no curve leaves, and the radius no curve enters.
    pub disc: Vec<f64>,
}

/// Covers the roulette of a byte grid taken as the wheel, the arguments of `spirograph` and a raster side: every curve is a wall, one fluid is poured from outside the picture and one at the centre of the track, and the shape is everything between the two walls, the walls and their pockets included. The cover takes as many samples per curve as the raster needs. `winding` is read by scanline off the polylines and `areas` is the closed form it converges to, so the pair checks the raster against Green's theorem and never the floods. A circle track only, `in` or `out`: the wall of a line or a polygon roulette need not close.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn spirograph_cover(
    types: &[u8],
    width: usize,
    height: usize,
    pens: &str,
    track: &str,
    ring: usize,
    wheel: usize,
    sides: usize,
    laps: usize,
    reach: f64,
    jitter: f64,
    seed: u32,
    side: usize,
) -> Result<Cover, Fault> {
    let (path, pencils) = build(
        types, width, height, pens, track, ring, wheel, sides, laps, reach, jitter, seed,
    )?;
    let bounds = roulette::disc(&path, &pencils)?;
    let out = roulette::cover(&path, &pencils, jitter == 0.0, 2, side)?;
    Ok(Cover {
        side: out.side as u32,
        mask: out.mask,
        covered: out.covered,
        hole: out.hole,
        wall: out.wall,
        winding: out.winding,
        areas: out.areas,
        disc: vec![bounds.x, bounds.y, bounds.radius, bounds.hole],
    })
}
