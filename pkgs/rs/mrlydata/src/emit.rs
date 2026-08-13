use mrlycore::errors::{value_error, Result};
use mrlycore::io::{make, write};
use mrlycore::json::clip;
use mrlycore::tile::Tile;
use mrlycore::{json, state, Json};
use mrlymath::crypto::hash::quick_hexdigest;
use mrlymath::two::artwork::{self, Config, Variation};
use mrlymath::two::{self, carry as carry2d, tile as tile2d, Cell2d};
use std::path::Path;

/// The default count of artworks one batch emits.
pub const BATCH: usize = 8;

/// The pixel side one cell is drawn at.
pub const SCALE: usize = 10;

const KEY: usize = 8;

/// Emits a batch of artworks into the directory under the default settings.
pub fn emit(dir: &Path, seed: u64, count: usize) -> Result<Json> {
    emit_with(dir, seed, count, &Config::default())
}

/// Emits a batch of artworks into the directory, returning the manifest it wrote.
pub fn emit_with(dir: &Path, seed: u64, count: usize, config: &Config) -> Result<Json> {
    make(dir)?;
    state::seed(seed);
    let mut artworks = Vec::new();
    for _ in 0..count {
        artworks.push(one(dir, config)?);
    }
    let manifest = json!({
        "v": 1,
        "version": env!("CARGO_PKG_VERSION"),
        "seed": clip(seed),
        "scale": SCALE,
        "artworks": artworks,
    });
    write(
        &dir.join("index.json"),
        (manifest.pretty() + "\n").as_bytes(),
    )?;
    Ok(manifest)
}

fn one(dir: &Path, config: &Config) -> Result<Json> {
    let variation = artwork::create(config)?;
    state::seed(variation.seed);
    let variation = artwork::generate(variation, config)?;
    let variation = artwork::render(variation, SCALE)?;
    let home = dir.join(&variation.key);
    make(&home)?;
    let files = write_files(&home, &variation)?;
    write(
        &home.join("artwork.json"),
        (variation.to_json().pretty() + "\n").as_bytes(),
    )?;
    Ok(json!({
        "key": &variation.key,
        "seed": clip(variation.seed),
        "edition": variation.edition.name(),
        "unit": variation.tile.width,
        "files": files,
    }))
}

// TILES

/// Emits a batch of unpainted tile pngs into the directory under the default tile settings.
pub fn tiles(dir: &Path, seed: u64, count: usize) -> Result<Json> {
    tiles_with(dir, seed, count, &tile2d::Config::default())
}

/// Emits a batch of unpainted tile pngs into the directory, returning the manifest it wrote.
pub fn tiles_with(dir: &Path, seed: u64, count: usize, config: &tile2d::Config) -> Result<Json> {
    make(dir)?;
    state::seed(seed);
    let mut rows = Vec::new();
    for _ in 0..count {
        rows.push(one_tile(dir, config)?);
    }
    let manifest = json!({
        "v": 1,
        "version": env!("CARGO_PKG_VERSION"),
        "seed": clip(seed),
        "scale": SCALE,
        "tiles": rows,
    });
    write(
        &dir.join("index.json"),
        (manifest.pretty() + "\n").as_bytes(),
    )?;
    Ok(manifest)
}

fn one_tile(dir: &Path, config: &tile2d::Config) -> Result<Json> {
    let s = state::randint(0, i64::MAX) as u64;
    state::seed(s);
    let tile = tile2d::create(config)?;
    let cell = tile2d::build(&tile)?;
    let png = two::png(&cell, SCALE)?;
    let name = format!("{}_{}.png", tile.group.name(), key(KEY));
    write(&dir.join(&name), &png)?;
    Ok(json!({
        "name": name,
        "seed": clip(s),
        "group": tile.group.name(),
        "width": tile.width,
        "height": tile.height,
        "bytes": png.len(),
        "sha": quick_hexdigest(&png)?,
        "recipe": tile.to_json(),
    }))
}

fn key(length: usize) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    (0..length)
        .map(|_| DIGITS[state::randint(0, 15) as usize] as char)
        .collect()
}

// CARRY

/// The tile side a carried sheet lays each of its four tiles out at.
pub const SIDE: usize = 27;

const DRAWS: usize = 64;

/// Returns the tile settings a carried sheet draws its four tiles under.
pub fn sheets() -> tile2d::Config {
    tile2d::Config {
        min_size: SIDE,
        max_size: SIDE,
        ..tile2d::Config::default()
    }
}

/// Emits a batch of payload-carrying sheet pngs into the directory under the default sheet settings.
pub fn carry(dir: &Path, seed: u64, count: usize, payload: &[u8]) -> Result<Json> {
    carry_with(dir, seed, count, payload, &sheets())
}

/// Emits a batch of payload-carrying sheet pngs into the directory, returning the manifest it wrote.
pub fn carry_with(
    dir: &Path,
    seed: u64,
    count: usize,
    payload: &[u8],
    config: &tile2d::Config,
) -> Result<Json> {
    make(dir)?;
    state::seed(seed);
    let mut rows = Vec::new();
    for _ in 0..count {
        rows.push(one_sheet(dir, payload, config)?);
    }
    let manifest = json!({
        "v": 1,
        "version": env!("CARGO_PKG_VERSION"),
        "seed": clip(seed),
        "scale": SCALE,
        "payload": {
            "bytes": payload.len(),
            "sha": quick_hexdigest(payload)?,
        },
        "sheets": rows,
    });
    write(
        &dir.join("index.json"),
        (manifest.pretty() + "\n").as_bytes(),
    )?;
    Ok(manifest)
}

fn drawn(config: &tile2d::Config) -> Result<(Tile, Cell2d)> {
    let tile = tile2d::create(config)?;
    let cell = tile2d::build(&tile)?;
    Ok((tile, cell))
}

fn carrier(config: &tile2d::Config, payload: &[u8]) -> Result<(Tile, Cell2d)> {
    for _ in 0..DRAWS {
        let (tile, cell) = drawn(config)?;
        if carry2d::capacity(&cell) >= payload.len() {
            return Ok((tile, cell));
        }
    }
    value_error(format!(
        "no drawn tile holds sites enough for {} bytes.",
        payload.len()
    ))
}

fn one_sheet(dir: &Path, payload: &[u8], config: &tile2d::Config) -> Result<Json> {
    let s = state::randint(0, i64::MAX) as u64;
    state::seed(s);
    let (first, a) = drawn(config)?;
    let (second, b) = drawn(config)?;
    let (third, c) = drawn(config)?;
    let (fourth, d) = carrier(config, payload)?;
    let unit = d.width();
    let sheet = carry2d::sheet(&[a, b, c, d], payload)?;
    let png = two::png(&sheet, SCALE)?;
    let name = format!("{}_{}.png", fourth.group.name(), key(KEY));
    write(&dir.join(&name), &png)?;
    Ok(json!({
        "name": name,
        "seed": clip(s),
        "unit": unit,
        "width": sheet.width(),
        "height": sheet.height(),
        "bytes": png.len(),
        "sha": quick_hexdigest(&png)?,
        "tiles": [
            first.to_json(),
            second.to_json(),
            third.to_json(),
            fourth.to_json(),
        ],
    }))
}

// DISK

fn write_files(home: &Path, variation: &Variation) -> Result<Vec<Json>> {
    let mut files = Vec::new();
    for file in &variation.files {
        let name = format!("{}_{}x{}.png", variation.key, file.width, file.height);
        write(&home.join(&name), &file.png)?;
        files.push(json!({
            "name": name,
            "width": file.width,
            "height": file.height,
            "bytes": file.png.len(),
            "sha": quick_hexdigest(&file.png)?,
        }));
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::json::parse;
    use mrlycore::state::guard;
    use mrlymath::two::tile as tile2d;
    use std::fs;

    fn tiny() -> Config {
        Config {
            tile: tile2d::Config {
                min_size: 3,
                max_size: 5,
                ..tile2d::Config::default()
            },
            files: vec![(1, 1), (2, 3)],
            ..Config::default()
        }
    }

    fn small() -> tile2d::Config {
        tile2d::Config {
            min_size: 3,
            max_size: 9,
            anti: Some(false),
            ..tile2d::Config::default()
        }
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("mrlydata_art_{name}_{}", std::process::id()))
    }

    fn keys(manifest: &Json) -> Vec<String> {
        manifest["artworks"]
            .as_array()
            .unwrap()
            .iter()
            .map(|art| art["key"].as_str().unwrap().to_string())
            .collect()
    }

    #[test]
    fn the_batch_writes_a_png_per_file_and_a_manifest() {
        let _g = guard();
        let dir = scratch("batch");
        let manifest = emit_with(&dir, 7, 3, &tiny()).unwrap();
        assert_eq!(manifest["v"], 1);
        assert_eq!(
            manifest["version"].as_str(),
            Some(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(manifest["seed"], 7);
        assert_eq!(manifest["scale"], SCALE as i64);
        let written = fs::read_to_string(dir.join("index.json")).unwrap();
        assert_eq!(parse(&written).unwrap(), manifest);
        let artworks = manifest["artworks"].as_array().unwrap();
        assert_eq!(artworks.len(), 3);
        for art in artworks {
            let key = art["key"].as_str().unwrap();
            let home = dir.join(key);
            let record = parse(&fs::read_to_string(home.join("artwork.json")).unwrap()).unwrap();
            assert_eq!(record["key"].as_str(), Some(key));
            let files = art["files"].as_array().unwrap();
            assert_eq!(files.len(), 2);
            for file in files {
                let bytes = fs::read(home.join(file["name"].as_str().unwrap())).unwrap();
                assert_eq!(&bytes[1..4], b"PNG");
                assert_eq!(file["bytes"].as_i64().unwrap() as usize, bytes.len());
                assert_eq!(
                    file["sha"].as_str().unwrap(),
                    quick_hexdigest(&bytes).unwrap()
                );
            }
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn two_batches_press_identical_bytes() {
        let _g = guard();
        let (a, b) = (scratch("batcha"), scratch("batchb"));
        let first = emit_with(&a, 11, 2, &tiny()).unwrap();
        let second = emit_with(&b, 11, 2, &tiny()).unwrap();
        assert_eq!(first.pretty(), second.pretty());
        for key in keys(&first) {
            for name in fs::read_dir(a.join(&key)).unwrap() {
                let name = name.unwrap().file_name();
                let x = fs::read(a.join(&key).join(&name)).unwrap();
                let y = fs::read(b.join(&key).join(&name)).unwrap();
                assert_eq!(x, y, "{key} drifted between batches");
            }
        }
        fs::remove_dir_all(&a).ok();
        fs::remove_dir_all(&b).ok();
    }

    #[test]
    fn a_batch_of_one_replays_from_its_recorded_seed() {
        let _g = guard();
        let dir = scratch("replay");
        let manifest = emit_with(&dir, 3, 1, &tiny()).unwrap();
        let art = &manifest["artworks"].as_array().unwrap()[0];
        let key = art["key"].as_str().unwrap().to_string();
        let home = dir.join(&key);
        let record = parse(&fs::read_to_string(home.join("artwork.json")).unwrap()).unwrap();
        state::seed(record["seed"].as_u64().unwrap());
        let again = artwork::generate(Variation::from_json(&record).unwrap(), &tiny()).unwrap();
        let again = artwork::render(again, SCALE).unwrap();
        let first = fs::read(home.join(format!("{key}_1x1.png"))).unwrap();
        assert_eq!(again.files[0].png, first);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_tile_batch_writes_a_png_per_tile_and_a_manifest() {
        let _g = guard();
        let dir = scratch("tiles");
        let manifest = tiles_with(&dir, 5, 4, &small()).unwrap();
        assert_eq!(manifest["v"], 1);
        assert_eq!(
            manifest["version"].as_str(),
            Some(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(manifest["seed"], 5);
        assert_eq!(manifest["scale"], SCALE as i64);
        let written = fs::read_to_string(dir.join("index.json")).unwrap();
        assert_eq!(parse(&written).unwrap(), manifest);
        let rows = manifest["tiles"].as_array().unwrap();
        assert_eq!(rows.len(), 4);
        for row in rows {
            let name = row["name"].as_str().unwrap();
            assert!(name.starts_with(row["group"].as_str().unwrap()), "{name}");
            assert!(name.ends_with(".png"), "{name}");
            let bytes = fs::read(dir.join(name)).unwrap();
            assert_eq!(&bytes[1..4], b"PNG");
            assert_eq!(row["bytes"].as_i64().unwrap() as usize, bytes.len());
            assert_eq!(
                row["sha"].as_str().unwrap(),
                quick_hexdigest(&bytes).unwrap()
            );
            assert_eq!(row["width"], row["height"]);
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_tile_row_redraws_its_png_from_its_recipe() {
        let _g = guard();
        let dir = scratch("recipes");
        let manifest = tiles_with(&dir, 9, 3, &small()).unwrap();
        for row in manifest["tiles"].as_array().unwrap() {
            let tile = mrlycore::tile::Tile::from_json(&row["recipe"]).unwrap();
            assert_eq!(row["group"].as_str(), Some(tile.group.name()));
            assert_eq!(row["width"].as_i64().unwrap() as usize, tile.width);
            let cell = tile2d::build(&tile).unwrap();
            let png = two::png(&cell, SCALE).unwrap();
            let name = row["name"].as_str().unwrap();
            assert_eq!(png, fs::read(dir.join(name)).unwrap(), "{name} drifted");
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_carry_batch_writes_a_sheet_per_png_and_a_manifest() {
        let _g = guard();
        let dir = scratch("carry");
        let payload = b"mrlyprod";
        let manifest = carry_with(&dir, 5, 2, payload, &sheets()).unwrap();
        assert_eq!(manifest["v"], 1);
        assert_eq!(
            manifest["version"].as_str(),
            Some(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(manifest["seed"], 5);
        assert_eq!(manifest["scale"], SCALE as i64);
        assert_eq!(manifest["payload"]["bytes"].as_i64(), Some(8));
        assert_eq!(
            manifest["payload"]["sha"].as_str().unwrap(),
            quick_hexdigest(payload).unwrap()
        );
        let written = fs::read_to_string(dir.join("index.json")).unwrap();
        assert_eq!(parse(&written).unwrap(), manifest);
        let rows = manifest["sheets"].as_array().unwrap();
        assert_eq!(rows.len(), 2);
        for row in rows {
            let name = row["name"].as_str().unwrap();
            assert!(name.ends_with(".png"), "{name}");
            let bytes = fs::read(dir.join(name)).unwrap();
            assert_eq!(&bytes[1..4], b"PNG");
            assert_eq!(row["bytes"].as_i64().unwrap() as usize, bytes.len());
            assert_eq!(
                row["sha"].as_str().unwrap(),
                quick_hexdigest(&bytes).unwrap()
            );
            assert_eq!(row["unit"].as_i64(), Some(SIDE as i64));
            assert_eq!(row["width"].as_i64(), Some(SIDE as i64 * 5));
            assert_eq!(row["width"], row["height"]);
            assert_eq!(row["tiles"].as_array().unwrap().len(), 4);
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_sheet_row_redraws_its_png_and_gives_the_payload_back() {
        let _g = guard();
        let dir = scratch("carried");
        let payload = b"mrlyprod";
        let manifest = carry_with(&dir, 9, 1, payload, &sheets()).unwrap();
        for row in manifest["sheets"].as_array().unwrap() {
            let cells: Vec<Cell2d> = row["tiles"]
                .as_array()
                .unwrap()
                .iter()
                .map(|recipe| tile2d::build(&Tile::from_json(recipe).unwrap()).unwrap())
                .collect();
            let laid = [
                cells[0].clone(),
                cells[1].clone(),
                cells[2].clone(),
                cells[3].clone(),
            ];
            let sheet = carry2d::sheet(&laid, payload).unwrap();
            let name = row["name"].as_str().unwrap();
            assert_eq!(
                two::png(&sheet, SCALE).unwrap(),
                fs::read(dir.join(name)).unwrap(),
                "{name} drifted"
            );
            assert_eq!(carry2d::read(&sheet, &cells[3]).unwrap(), payload);
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_payload_past_the_sites_is_refused() {
        let _g = guard();
        let dir = scratch("carryover");
        let payload = vec![7u8; 4096];
        assert!(carry_with(&dir, 5, 1, &payload, &sheets()).is_err());
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn two_tile_batches_press_identical_bytes() {
        let _g = guard();
        let (a, b) = (scratch("tilesa"), scratch("tilesb"));
        let first = tiles_with(&a, 13, 3, &small()).unwrap();
        let second = tiles_with(&b, 13, 3, &small()).unwrap();
        assert_eq!(first.pretty(), second.pretty());
        for row in first["tiles"].as_array().unwrap() {
            let name = row["name"].as_str().unwrap();
            assert_eq!(
                fs::read(a.join(name)).unwrap(),
                fs::read(b.join(name)).unwrap()
            );
        }
        fs::remove_dir_all(&a).ok();
        fs::remove_dir_all(&b).ok();
    }
}
