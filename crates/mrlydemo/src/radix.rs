use crate::Fault;
use mrlycore::json;
use mrlynum::gauss::Ring;
use mrlynum::radix::{self, Base, Radix};
use wasm_bindgen::prelude::*;

/// The drawing budget: the most points one level may carry.
pub const POINTS: usize = 1 << 16;

/// The deepest level offered, whatever the digit count.
pub const DEEPEST: usize = 16;

/// The carpet code: the nine box cells of the side-three tile with the centre out.
pub const CARPET: u128 = 0b111101111;

fn named(name: &str) -> Result<Ring, Fault> {
    Ring::named(name)
        .ok_or_else(|| Fault::new(format!("ring {name:?} is not a ring of this plane.")))
}

fn whole(text: &str) -> Result<i64, Fault> {
    text.trim()
        .parse()
        .map_err(|_| Fault::new(format!("{text:?} is not a whole number.")))
}

fn pairs(text: &str) -> Result<Vec<(i64, i64)>, Fault> {
    text.split('_')
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            let (a, c) = part
                .split_once(':')
                .ok_or_else(|| Fault::new(format!("digit {part:?} is not a pair a:c.")))?;
            Ok((whole(a)?, whole(c)?))
        })
        .collect()
}

fn indices(text: &str) -> Result<Vec<usize>, Fault> {
    text.split('_')
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            part.trim()
                .parse()
                .map_err(|_| Fault::new(format!("twist {part:?} is not a unit index.")))
        })
        .collect()
}

fn ceiling(size: usize) -> usize {
    let mut level = 1;
    while level < DEEPEST
        && size
            .checked_pow(level as u32 + 1)
            .is_some_and(|count| count <= POINTS)
    {
        level += 1;
    }
    level
}

fn design(ring: &str, a: i32, c: i32, digits: &str, twists: &str) -> Result<Radix, Fault> {
    let ring = named(ring)?;
    let (a, c) = (i64::from(a), i64::from(c));
    if ring.norm(a, c) < 2 {
        return Err(Fault::new(format!(
            "the base {a}, {c} has norm {}, below two.",
            ring.norm(a, c)
        )));
    }
    let digits = pairs(digits)?;
    if digits.is_empty() {
        return Err(Fault::new("a design needs a digit."));
    }
    let units = ring.associates(1, 0);
    let picked = indices(twists)?;
    if picked.len() != digits.len() {
        return Err(Fault::new(format!(
            "{} digits carry {} twists.",
            digits.len(),
            picked.len()
        )));
    }
    if let Some(&over) = picked.iter().find(|&&i| i >= units.len()) {
        return Err(Fault::new(format!(
            "unit {over} is past the {} units of the ring.",
            units.len()
        )));
    }
    let turns = picked.into_iter().map(|i| units[i]).collect();
    Ok(Radix::new(Base::new(ring, (a, c)), digits, turns))
}

fn levelled(radix: &Radix, level: usize) -> Result<usize, Fault> {
    let cap = ceiling(radix.size());
    if level < 1 || level > cap {
        return Err(Fault::new(format!(
            "level {level} is past the cap of {cap} for {} digits at the budget of {POINTS} points.",
            radix.size()
        )));
    }
    Ok(cap)
}

fn spelling(radix: &Radix) -> (String, String) {
    let units = radix.ring().associates(1, 0);
    let digits = radix
        .digits()
        .iter()
        .map(|(a, c)| format!("{a}:{c}"))
        .collect::<Vec<String>>()
        .join("_");
    let twists = radix
        .twists()
        .iter()
        .map(|u| {
            units
                .iter()
                .position(|v| v == u)
                .expect("a twist is a unit")
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("_");
    (digits, twists)
}

fn word(ring: Ring) -> &'static str {
    match ring {
        Ring::Gaussian => "gaussian",
        Ring::Eisenstein => "eisenstein",
    }
}

fn card(name: &str, label: &str, radix: &Radix, level: usize, line: bool) -> mrlycore::Json {
    let (digits, twists) = spelling(radix);
    let base = radix.base().value();
    json!({
        "name": name,
        "label": label,
        "ring": word(radix.ring()),
        "a": base.0,
        "c": base.1,
        "digits": digits,
        "twists": twists,
        "level": level,
        "line": line,
    })
}

/// Returns the dial itself: the rings, the bases offered on each, the units of each ring and the presets, as JSON.
///
/// Every preset is a quintuple built by `mrlynum::radix`, spelled back as the digit list and the unit indices the page carries in its query.
#[wasm_bindgen]
pub fn radix_menu() -> String {
    let places = |ring: Ring, list: &[(i64, i64)]| {
        list.iter()
            .map(|&(a, c)| json!({"a": a, "c": c, "norm": ring.norm(a, c)}))
            .collect::<Vec<mrlycore::Json>>()
    };
    let bases = |ring: Ring, list: Vec<(i64, i64)>| {
        json!({
            "name": word(ring),
            "units": ring.associates(1, 0).iter().map(|&(a, c)| json!({"a": a, "c": c})).collect::<Vec<mrlycore::Json>>(),
            "bases": places(ring, &list),
        })
    };
    json!({
        "rings": [
            bases(Ring::Gaussian, vec![(2, 0), (1, 1), (2, 1), (3, 0)]),
            bases(Ring::Eisenstein, vec![(2, 0), (2, 1), (3, 0), (3, 1)]),
        ],
        "presets": [
            card("koch", "the Koch curve", &radix::koch(), 5, true),
            card("gasket", "the gasket", &radix::gasket(), 7, false),
            card("twindragon", "the twindragon", &radix::twindragon(), 14, false),
            card("tile7", "the norm-7 tile", &radix::flowsnake(), 5, false),
            card("carpet", "the carpet, box digits", &radix::tile(3, CARPET), 5, false),
        ],
        "points": POINTS,
    })
    .to_string()
}

/// Returns the deepest level a digit list is drawn at: the largest `L` with `(card F)^L` inside the budget.
#[wasm_bindgen]
pub fn radix_cap(digits: &str) -> Result<usize, Fault> {
    let digits = pairs(digits)?;
    if digits.is_empty() {
        return Err(Fault::new("a design needs a digit."));
    }
    Ok(ceiling(digits.len()))
}

/// Reads a radix design: its base, the canonical residues, the digits and their classes, the code of those classes, the fill `(card F)^L`, the distinct points, the similarity dimension and whether every digit is canonical, as JSON.
///
/// The digits are spelled `a:c` and joined by `_`, the twists are the indices of the ring's units in turning order from one, one per digit.
#[wasm_bindgen]
pub fn radix_read(
    ring: &str,
    a: i32,
    c: i32,
    digits: &str,
    twists: &str,
    level: usize,
) -> Result<String, Fault> {
    let radix = design(ring, a, c, digits, twists)?;
    let cap = levelled(&radix, level)?;
    let base = radix.base();
    let residues = base.residues();
    let units = radix.ring().associates(1, 0);
    let fill = radix.fill(level);
    let distinct = radix.distinct(level);
    Ok(json!({
        "ring": word(radix.ring()),
        "a": base.value().0,
        "c": base.value().1,
        "q": base.norm(),
        "level": level,
        "cap": cap,
        "size": radix.size(),
        "fill": fill.to_string(),
        "distinct": distinct,
        "glued": (distinct as u128) < fill,
        "dimension": radix.dimension(),
        "canonical": radix.canonical(),
        "code": radix.code().to_string(),
        "residues": residues.iter().map(|&(x, y)| json!({"a": x, "c": y})).collect::<Vec<mrlycore::Json>>(),
        "digits": radix.digits().iter().map(|&(x, y)| json!({"a": x, "c": y, "class": base.class((x, y))})).collect::<Vec<mrlycore::Json>>(),
        "twists": radix.twists().iter().map(|u| units.iter().position(|v| v == u).expect("a twist is a unit")).collect::<Vec<usize>>(),
        "units": units.iter().map(|&(x, y)| json!({"a": x, "c": y})).collect::<Vec<mrlycore::Json>>(),
    })
    .to_string())
}

/// Returns the level-`L` points of a radix design in the plane as `x, y` pairs of floats, one pair a word, in digit-lexicographic order.
#[wasm_bindgen]
pub fn radix_points(
    ring: &str,
    a: i32,
    c: i32,
    digits: &str,
    twists: &str,
    level: usize,
) -> Result<Vec<f32>, Fault> {
    let radix = design(ring, a, c, digits, twists)?;
    levelled(&radix, level)?;
    let mut out = Vec::with_capacity(2 * radix.fill(level) as usize);
    for (x, y) in radix.plane(level) {
        out.push(x as f32);
        out.push(y as f32);
    }
    Ok(out)
}
