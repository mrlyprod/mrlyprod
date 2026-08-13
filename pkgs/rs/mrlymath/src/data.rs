use crate::bang::{self, factory};
use crate::crypto::hash::{digest, fingerprint_cell, Config as HashConfig, Rule};
use crate::formulas;
use crate::life;
use crate::name::{Bang, Named};
use crate::two::{self, Cell2d};
use mrlycore::data::{shuffle, Well};
use mrlycore::rng::Rng;
use mrlycore::state;
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};

const RETRIES: usize = 8;
const MAX_SIZE: usize = 9;
const MAX_DIM: usize = 3;
const BASE: usize = 2;
const TEXT_MAX: i64 = 32;
const SOUP_SPAN: (i64, i64) = (8, 16);
const DENSITY_SPAN: (i64, i64) = (20, 60);
const PRINT_SIDE: usize = 64;
const RULES: [Rule; 4] = [Rule::Life, Rule::Maze, Rule::Replicator, Rule::Anneal];
const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789 ";

/// Returns the wells this crate pours: tiles, designs, formulas, hashes and stories.
pub fn wells() -> Vec<Box<dyn Well>> {
    vec![
        Box::new(Tiles),
        Box::new(Designs),
        Box::new(Formulas),
        Box::new(Hashes),
        Box::new(Stories),
    ]
}

fn mix(seed: u64, i: u64) -> u64 {
    let mut x = seed.wrapping_add(i.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    x
}

fn clip(value: u64) -> Json {
    Json::Int((value & i64::MAX as u64) as i64)
}

fn big(value: u128) -> Json {
    Json::Int(value.min(i64::MAX as u128) as i64)
}

fn milli(value: f64) -> Json {
    Json::Int((value * 1000.0).round() as i64)
}

// TILES

struct Tiles;

impl Well for Tiles {
    fn name(&self) -> &str {
        "tiles"
    }
    fn about(&self) -> &str {
        "Seeded random tile recipes with their built grids and fill counts."
    }
    fn pour(&self, seed: u64, count: usize) -> Vec<Json> {
        let mut rows = Vec::new();
        for i in 0..count.saturating_mul(RETRIES) {
            if rows.len() >= count {
                break;
            }
            state::seed(mix(seed, i as u64));
            for _ in 0..RETRIES {
                let Ok(tile) = two::random_tile(MAX_SIZE) else {
                    continue;
                };
                let Ok(cell) = two::build(&tile) else {
                    continue;
                };
                rows.push(json!({
                    "recipe": tile.to_json(),
                    "group": tile.group.name(),
                    "width": cell.width(),
                    "height": cell.height(),
                    "grid": two::to_lists(&cell),
                    "fills": two::fills(&cell),
                }));
                break;
            }
        }
        rows
    }
}

// DESIGNS

struct Designs;

impl Well for Designs {
    fn name(&self) -> &str {
        "designs"
    }
    fn about(&self) -> &str {
        "Every canonical design code of dimensions 1 to 3 with its orbit facts."
    }
    fn pour(&self, seed: u64, count: usize) -> Vec<Json> {
        let mut rows = Vec::new();
        for dim in 1..=MAX_DIM {
            for design in bang::bang(dim).canonical() {
                rows.push(json!({
                    "dim": dim,
                    "code": big(design.i),
                    "name": Bang::new(design.i, dim, BASE).to_str(),
                    "orbit": design.orbit_size,
                    "degree": design.degree(),
                    "anf": design.anf(),
                    "corners": design.rule(),
                }));
            }
        }
        shuffle(&mut rows, seed);
        rows.truncate(count);
        rows
    }
}

// FORMULAS

struct Formulas;

impl Well for Formulas {
    fn name(&self) -> &str {
        "formulas"
    }
    fn about(&self) -> &str {
        "Closed-form fill, void, ratio, dimension and surface values of seeded design draws."
    }
    fn pour(&self, seed: u64, count: usize) -> Vec<Json> {
        let mut rows = Vec::new();
        for i in 0..count {
            let mut rng = Rng::new(mix(seed, i as u64));
            let dim = rng.range(1, MAX_DIM as i64) as usize;
            let code = rng.below(factory::total_codes(dim, BASE) as usize) as u128;
            let number = rng.range(2, 5) as usize;
            let level = rng.range(1, 4) as u32;
            if let Some(row) = formula_row(dim, code, number, level) {
                rows.push(row);
            }
        }
        rows
    }
}

fn formula_row(dim: usize, code: u128, number: usize, level: u32) -> Option<Json> {
    let fill = formulas::fill(code, number, dim, level, BASE).ok()?;
    let void = formulas::void(code, number, dim, level, BASE).ok()?;
    let ratio = formulas::ratio(code, number, dim, level, BASE).ok()?;
    let dimension = formulas::dimension(code, number, dim, BASE).ok()?;
    let mut row = json!({
        "dim": dim,
        "code": big(code),
        "number": number,
        "level": level,
        "grid": big(formulas::grid(number, dim, level)),
        "fill": big(fill),
        "void": big(void),
        "ratio_milli": milli(ratio),
        "dimension_milli": milli(dimension),
    });
    if dim == 3 {
        row["surface"] = big(formulas::surface(code, number, level, BASE).ok()?);
    }
    Some(row)
}

// HASHES

struct Hashes;

impl Well for Hashes {
    fn name(&self) -> &str {
        "hashes"
    }
    fn about(&self) -> &str {
        "Seeded texts with their automaton digests and a fingerprint sample."
    }
    fn pour(&self, seed: u64, count: usize) -> Vec<Json> {
        let mut rows = Vec::new();
        for i in 0..count {
            let mut rng = Rng::new(mix(seed, i as u64));
            let length = rng.range(1, TEXT_MAX) as usize;
            let text: String = (0..length)
                .map(|_| ALPHABET[rng.below(ALPHABET.len())] as char)
                .collect();
            let rule = *rng.choice(&RULES);
            let config = HashConfig {
                rule,
                ..HashConfig::default()
            };
            let Ok(d) = digest(text.as_bytes(), &config) else {
                continue;
            };
            let print = fingerprint_cell(&d, PRINT_SIDE);
            rows.push(json!({
                "text": text,
                "rule": rule.name(),
                "digest": d.hex(),
                "fingerprint": print[..PRINT_SIDE].to_vec(),
            }));
        }
        rows
    }
}

// STORIES

struct Stories;

impl Well for Stories {
    fn name(&self) -> &str {
        "stories"
    }
    fn about(&self) -> &str {
        "Seeded soups run under named life rules to their fates."
    }
    fn pour(&self, seed: u64, count: usize) -> Vec<Json> {
        let mut rows = Vec::new();
        for i in 0..count {
            let row_seed = mix(seed, i as u64);
            let mut rng = Rng::new(row_seed);
            let side = rng.range(SOUP_SPAN.0, SOUP_SPAN.1) as usize;
            let density = rng.range(DENSITY_SPAN.0, DENSITY_SPAN.1);
            let rule = *rng.choice(&RULES);
            let (birth, survive) = rule.counts();
            let soup = soup(&mut rng, side, density as f64 / 100.0);
            let config = life::Config::new(moore(), birth.clone(), survive.clone());
            let Ok(run) = life::animate(&soup, &config) else {
                continue;
            };
            rows.push(json!({
                "seed": clip(row_seed),
                "side": side,
                "density": density,
                "rule": rule.name(),
                "birth": birth,
                "survive": survive,
                "fate": run.fate.name(),
                "generations": run.count,
                "loop": run.loop_length,
                "entropy": run.last().map(life::entropy).unwrap_or(0),
            }));
        }
        rows
    }
}

fn moore() -> Cell2d {
    let mut mask = Tensor::of(vec![1u8; 9], vec![3, 3]);
    mask.set(&[1, 1], 0);
    Cell2d::new(mask)
}

fn soup(rng: &mut Rng, side: usize, density: f64) -> Cell2d {
    let mut grid = Tensor::new(vec![side, side]);
    for y in 0..side {
        for x in 0..side {
            if rng.chance(density) {
                grid.set(&[y, x], 1);
            }
        }
    }
    Cell2d::new(grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::guard;

    #[test]
    fn every_well_pours_deterministic_rows() {
        let _g = guard();
        for well in wells() {
            let a = well.pour(5, 3);
            let b = well.pour(5, 3);
            assert_eq!(a, b, "{} drifted between pours", well.name());
            assert!(!a.is_empty(), "{} poured nothing", well.name());
        }
    }

    #[test]
    fn a_short_pour_prefixes_a_long_one() {
        let _g = guard();
        for well in wells() {
            let short = well.pour(9, 2);
            let long = well.pour(9, 4);
            assert_eq!(short[..], long[..2], "{} broke the prefix law", well.name());
        }
    }

    #[test]
    fn tile_grids_match_their_recipes() {
        let _g = guard();
        let rows = Tiles.pour(11, 4);
        for row in rows {
            let width = row["width"].as_i64().unwrap() as usize;
            let height = row["height"].as_i64().unwrap() as usize;
            let grid = row["grid"].as_array().unwrap();
            assert_eq!(grid.len(), height);
            let mut fills = 0;
            for line in grid {
                let line = line.as_array().unwrap();
                assert_eq!(line.len(), width);
                fills += line.iter().filter(|v| v.as_i64() == Some(1)).count();
            }
            assert_eq!(row["fills"].as_i64().unwrap() as usize, fills);
            assert_eq!(row["recipe"]["v"], 1);
        }
    }

    #[test]
    fn designs_cover_three_dimensions() {
        let rows = Designs.pour(0, 999);
        for dim in 1..=MAX_DIM {
            let found = rows
                .iter()
                .filter(|r| r["dim"].as_i64() == Some(dim as i64));
            let expected = bang::bang(dim).distinct();
            assert_eq!(found.count(), expected, "dimension {dim} lost designs");
        }
    }

    #[test]
    fn formulas_agree_with_their_identities() {
        let rows = Formulas.pour(21, 8);
        for row in rows {
            let fill = row["fill"].as_i64().unwrap();
            let void = row["void"].as_i64().unwrap();
            assert_eq!(fill + void, row["grid"].as_i64().unwrap());
            let ratio = row["ratio_milli"].as_i64().unwrap();
            assert!((0..=1000).contains(&ratio));
        }
    }

    #[test]
    fn hashes_carry_hex_and_a_fingerprint_row() {
        let rows = Hashes.pour(2, 3);
        for row in rows {
            let hex = row["digest"].as_str().unwrap();
            assert_eq!(hex.len(), 64);
            assert!(hex.bytes().all(|b| b.is_ascii_hexdigit()));
            let print = row["fingerprint"].as_array().unwrap();
            assert_eq!(print.len(), PRINT_SIDE);
        }
    }

    #[test]
    fn stories_end_with_a_named_fate() {
        let rows = Stories.pour(4, 3);
        for row in rows {
            let fate = row["fate"].as_str().unwrap();
            assert!(["dead", "alive", "loop", "timeout"].contains(&fate));
            assert!(row["generations"].as_i64().unwrap() >= 1);
        }
    }
}
