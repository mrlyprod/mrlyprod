use mrlycore::errors::{value_error, Result};
use mrlycore::state::choice;
use mrlycore::tensor::Tensor;
use mrlycore::tile::{Design, Group, Source, Tile};

use super::designs;
use super::geometry;
use super::models::Cell2d;
use crate::dim::tile as spec;

/// The constraints a random 2d tile is drawn under.
pub type Config = spec::ConfigNd<2>;

fn rotation(_source: Source) -> usize {
    choice(&[0, 1, 2, 3])
}

/// Draws a random tile satisfying the config, rotations drawn from the four quarter-turns.
pub fn create(config: &Config) -> Result<Tile> {
    spec::create(config, rotation)
}

/// Draws a random tile up to the given size under the default config.
pub fn random_tile(max_size: usize) -> Result<Tile> {
    spec::random_tile::<2>(max_size, rotation)
}

fn design_cell(design: Design, number: usize, level: usize) -> Result<Cell2d> {
    match design {
        Design::Carpet => designs::carpet(number, level),
        Design::Net => designs::net(number, level),
        Design::Htree => designs::htree(number, level),
        Design::Vtree => designs::vtree(number, level),
        Design::Void => designs::void(number, level),
        other => value_error(format!("design {} is not 2d.", other.name())),
    }
}

fn source_cell(source: Source, number: usize, level: usize, rotation: usize) -> Result<Cell2d> {
    let mut c = match source {
        Source::Classic(design) => design_cell(design, number, level)?,
        Source::Code(code) => designs::create(code, number, level, 0, 2)?,
    };
    if rotation != 0 {
        c = c.rotate(rotation);
    }
    Ok(c)
}

fn cell(tile: &Tile, i: usize, level: usize) -> Result<Cell2d> {
    let mut c = source_cell(tile.sources[i], tile.numbers[i], level, tile.rotations[i])?;
    if tile.anti.get(i).copied().unwrap_or(false) {
        c = c.anti();
    }
    Ok(c)
}

fn tree_mask(n: usize) -> Result<Tensor> {
    let vertical = designs::vtree(n, 1)?;
    let horizontal = vertical.clone().rotate(1);
    let v = vertical.types();
    let h = horizontal.types();
    let mut data = vec![0u8; v.size()];
    for (flat, item) in data.iter_mut().enumerate() {
        let a = v.bytes()[flat];
        let b = h.bytes()[flat];
        *item = match (a, b) {
            (1, 1) => 2,
            (1, _) | (_, 1) => 1,
            _ => 0,
        };
    }
    Ok(Tensor::of(data, v.shape.clone()))
}

fn build_general(tile: &Tile) -> Result<Cell2d> {
    cell(tile, 0, 1)
}

fn build_fractal(tile: &Tile) -> Result<Cell2d> {
    cell(tile, 0, tile.levels[0])
}

fn build_magic(tile: &Tile) -> Result<Cell2d> {
    let cells: Result<Vec<Cell2d>> = (0..tile.sources.len()).map(|i| cell(tile, i, 1)).collect();
    geometry::magic(&cells?)
}

fn build_special(tile: &Tile) -> Result<Cell2d> {
    let cell = designs::vtree(tile.numbers[0], 1)?;
    let mut mask = source_cell(tile.sources[0], tile.factor, 1, tile.rotations[0])?;
    if tile.flip {
        mask = mask.invert();
    }
    geometry::special(mask.types(), &cell)
}

fn build_mosaic(tile: &Tile) -> Result<Cell2d> {
    let mask = tree_mask(tile.factor)?;
    let cells: Result<Vec<Cell2d>> = (0..3).map(|i| cell(tile, i, 1)).collect();
    geometry::mosaic(&mask, &cells?)
}

fn builder(group: Group) -> fn(&Tile) -> Result<Cell2d> {
    match group {
        Group::General => build_general,
        Group::Fractal => build_fractal,
        Group::Magic => build_magic,
        Group::Special => build_special,
        Group::Mosaic => build_mosaic,
    }
}

fn ragged(tile: &Tile) -> bool {
    let slots = tile.sources.len();
    let wanted = match tile.group {
        Group::Mosaic => 3,
        _ => 1,
    };
    slots < wanted
        || tile.numbers.len() < slots
        || tile.levels.len() < slots
        || tile.rotations.len() < slots
}

/// Builds the cell the tile describes, or an error when the tile is ragged or will not render.
pub fn build(tile: &Tile) -> Result<Cell2d> {
    if ragged(tile) {
        return value_error("tile slots are ragged.");
    }
    let mut c = builder(tile.group)(tile)?;
    if tile.invert {
        c = c.invert();
    }
    Ok(c)
}

/// Returns whether the tile passes its check and builds to its declared size.
pub fn probe(tile: &Tile) -> bool {
    tile.check().is_ok()
        && build(tile)
            .map(|c| c.width() == tile.width && c.height() == tile.height)
            .unwrap_or(false)
}

/// Returns a k by k tensor of types sampled evenly across the cell.
pub fn sample_types(cell: &Cell2d, k: usize) -> Tensor {
    let (w, h) = (cell.width(), cell.height());
    let mut out = Tensor::new(vec![k, k]);
    for y in 0..k {
        for x in 0..k {
            out.set(&[y, x], cell.types().get(&[y * h / k, x * w / k]));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::{guard as rng_lock, seed};
    use mrlycore::tile::{Catalog, Parity};
    fn config() -> Config {
        Config {
            min_size: 3,
            max_size: 64,
            anti: Some(false),
            ..Config::default()
        }
    }
    #[test]
    fn built_size_matches_unit_size() {
        let _guard = rng_lock();
        let config = config();
        for s in 0..200 {
            seed(s);
            let tile = create(&config).unwrap();
            let cell = build(&tile).unwrap();
            assert_eq!(
                cell.width(),
                tile.width,
                "width {:?} seed {}",
                tile.group,
                s
            );
            assert_eq!(
                cell.height(),
                tile.height,
                "height {:?} seed {}",
                tile.group,
                s
            );
        }
    }
    #[test]
    fn create_is_seeded() {
        let _guard = rng_lock();
        seed(123);
        let a = create(&config()).unwrap();
        seed(123);
        let b = create(&config()).unwrap();
        assert_eq!(a, b);
    }
    #[test]
    fn random_tile_respects_max() {
        let _guard = rng_lock();
        for s in 0..50 {
            seed(s);
            let tile = random_tile(30).unwrap();
            assert!(tile.max_size() <= 30);
        }
    }
    #[test]
    fn classics_use_named_designs() {
        let _guard = rng_lock();
        let config = config();
        for s in 0..50 {
            seed(s);
            let tile = create(&config).unwrap();
            for source in &tile.sources {
                assert!(
                    matches!(source, Source::Classic(_)),
                    "classics catalog must yield named designs, got {:?}",
                    source
                );
            }
        }
    }
    #[test]
    fn universe_builds_from_codes() {
        let _guard = rng_lock();
        let config = Config {
            catalog: Catalog::Universe,
            min_size: 3,
            max_size: 27,
            anti: Some(false),
            ..Config::default()
        };
        let mut saw_code = false;
        for s in 0..100 {
            seed(s);
            let tile = create(&config).unwrap();
            if tile.sources.iter().any(|s| matches!(s, Source::Code(_))) {
                saw_code = true;
            }
            let cell = build(&tile).unwrap();
            assert_eq!(cell.width(), tile.width, "universe width seed {}", s);
            assert_eq!(cell.height(), tile.height, "universe height seed {}", s);
        }
        assert!(
            saw_code,
            "universe catalog should produce code-based sources"
        );
    }
    #[test]
    fn magic_can_nest_deeper_than_two() {
        let _guard = rng_lock();
        let config = Config {
            min_size: 3,
            max_size: 300,
            groups: vec![Group::Magic],
            anti: Some(false),
            ..Config::default()
        };
        let mut deep = false;
        for s in 0..200 {
            seed(s);
            if let Ok(tile) = create(&config) {
                if tile.sources.len() >= 3 {
                    deep = true;
                    let cell = build(&tile).unwrap();
                    assert_eq!(cell.width(), tile.width);
                }
            }
        }
        assert!(deep, "expected at least one magic tile nested 3+ deep");
    }
    #[test]
    fn build_errors_on_a_ragged_tile() {
        use mrlycore::json;
        let parsed = Tile::from_json(&json!({
            "v": 1, "group": "General", "factor": 0,
            "sources": [{ "design": "Carpet" }],
            "numbers": [], "levels": [], "rotations": [], "anti": [],
            "invert": false, "flip": false, "base": 2, "width": 0, "height": 0,
        }))
        .unwrap();
        assert!(build(&parsed).is_err());
        assert!(!probe(&parsed));
        let mut bare = Tile::new(Group::Mosaic);
        bare.sources = vec![Source::Classic(Design::Carpet)];
        assert!(build(&bare).is_err());
    }
    #[test]
    fn probe_delegates_to_the_check_law() {
        let mut tile = Tile::new(Group::General);
        tile.sources = vec![Source::Classic(Design::Carpet)];
        tile.numbers = vec![3];
        tile.levels = vec![1];
        tile.rotations = vec![0];
        tile.anti = vec![false];
        tile.resize();
        assert!(probe(&tile));
        tile.anti = Vec::new();
        assert!(!probe(&tile));
    }
    #[test]
    fn evens_parity_builds() {
        let _guard = rng_lock();
        let config = Config {
            min_size: 4,
            max_size: 64,
            parity: Parity::Evens,
            groups: vec![Group::General],
            anti: Some(false),
            ..Config::default()
        };
        for s in 0..50 {
            seed(s);
            let tile = create(&config).unwrap();
            assert_eq!(tile.numbers[0] % 2, 0);
            let cell = build(&tile).unwrap();
            assert_eq!(cell.width(), tile.width);
        }
    }
}
