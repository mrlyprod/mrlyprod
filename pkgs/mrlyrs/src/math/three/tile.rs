use crate::core::errors::{value_error, Result};
use crate::core::state::randint;
use crate::core::tensor::Tensor;
use crate::core::tile::{Design, Group, Source, Tile};

use super::designs;
use super::geometry;
use super::models::Cell3d;
use crate::math::dim::tile as spec;

pub type Config = spec::ConfigNd<3>;

fn rotation(_source: Source) -> usize {
    randint(0, 23) as usize
}

pub fn create(config: &Config) -> Result<Tile> {
    spec::create(config, rotation)
}

pub fn random_tile(max_size: usize) -> Result<Tile> {
    spec::random_tile::<3>(max_size, rotation)
}

fn design_cell(design: Design, number: usize, level: usize) -> Result<Cell3d> {
    match design {
        Design::Carpet => designs::carpet(number, level),
        Design::Net => designs::net(number, level),
        Design::Xtree => designs::xtree(number, level),
        Design::Ytree => designs::ytree(number, level),
        Design::Ztree => designs::ztree(number, level),
        Design::Void => designs::void(number, level),
        other => value_error(format!("design {} is not 3d.", other.name())),
    }
}

fn source_cell(source: Source, number: usize, level: usize, rotation: usize) -> Result<Cell3d> {
    let mut c = match source {
        Source::Classic(design) => design_cell(design, number, level)?,
        Source::Code(code) => designs::create(code, number, level, 2)?,
    };
    if rotation != 0 {
        c = c.orient(rotation)?;
    }
    Ok(c)
}

fn cell(tile: &Tile, i: usize, level: usize) -> Result<Cell3d> {
    let mut c = source_cell(tile.sources[i], tile.numbers[i], level, tile.rotations[i])?;
    if tile.anti.get(i).copied().unwrap_or(false) {
        c = c.anti();
    }
    Ok(c)
}

fn orient_mask(n: usize, fill: u8) -> Result<Tensor> {
    let line = designs::xtree(n, 1)?;
    let t = line.types();
    let data: Vec<u8> = t
        .bytes()
        .iter()
        .map(|&v| if v == 1 { fill } else { 0 })
        .collect();
    Ok(Tensor::of(data, t.shape.clone()))
}

fn index_mask(n: usize) -> Result<Tensor> {
    let x = designs::xtree(n, 1)?;
    let y = designs::ytree(n, 1)?;
    let z = designs::ztree(n, 1)?;
    let (xt, yt, zt) = (x.types(), y.types(), z.types());
    let mut data = vec![0u8; xt.size()];
    for (flat, item) in data.iter_mut().enumerate() {
        *item = if zt.bytes()[flat] == 1 {
            2
        } else if yt.bytes()[flat] == 1 {
            1
        } else {
            0
        };
    }
    Ok(Tensor::of(data, xt.shape.clone()))
}

fn build_general(tile: &Tile) -> Result<Cell3d> {
    cell(tile, 0, 1)
}

fn build_fractal(tile: &Tile) -> Result<Cell3d> {
    cell(tile, 0, tile.levels[0])
}

fn build_magic(tile: &Tile) -> Result<Cell3d> {
    let cells: Result<Vec<Cell3d>> = (0..tile.sources.len()).map(|i| cell(tile, i, 1)).collect();
    geometry::magic(&cells?)
}

fn build_special(tile: &Tile) -> Result<Cell3d> {
    let cell = designs::xtree(tile.numbers[0], 1)?;
    let fill = if tile.flip {
        0
    } else {
        tile.rotations[0].max(1) as u8
    };
    let mask = orient_mask(tile.factor, fill)?;
    geometry::special(&mask, &cell)
}

fn build_mosaic(tile: &Tile) -> Result<Cell3d> {
    let mask = index_mask(tile.factor)?;
    let cells: Result<Vec<Cell3d>> = (0..3).map(|i| cell(tile, i, 1)).collect();
    geometry::mosaic(&mask, &cells?)
}

fn builder(group: Group) -> fn(&Tile) -> Result<Cell3d> {
    match group {
        Group::General => build_general,
        Group::Fractal => build_fractal,
        Group::Magic => build_magic,
        Group::Special => build_special,
        Group::Mosaic => build_mosaic,
    }
}

pub fn build(tile: &Tile) -> Result<Cell3d> {
    let mut c = builder(tile.group)(tile)?;
    if tile.invert {
        c = c.invert();
    }
    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::{guard as rng_lock, seed};
    use crate::core::tile::Catalog;
    fn config() -> Config {
        Config {
            min_size: 3,
            max_size: 27,
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
                "width seed {} {:?}",
                s,
                tile.group
            );
            assert_eq!(
                cell.height(),
                tile.height,
                "height seed {} {:?}",
                s,
                tile.group
            );
            assert_eq!(cell.depth(), tile.width, "depth cubic seed {}", s);
        }
    }
    #[test]
    fn create_is_seeded() {
        let _guard = rng_lock();
        seed(321);
        let a = create(&config()).unwrap();
        seed(321);
        let b = create(&config()).unwrap();
        assert_eq!(a, b);
    }
    #[test]
    fn classics_use_named_designs() {
        let _guard = rng_lock();
        let config = config();
        for s in 0..50 {
            seed(s);
            let tile = create(&config).unwrap();
            for source in &tile.sources {
                assert!(matches!(source, Source::Classic(_)));
            }
        }
    }
    #[test]
    fn universe_builds_from_codes() {
        let _guard = rng_lock();
        let config = Config {
            catalog: Catalog::Universe,
            min_size: 3,
            max_size: 9,
            anti: Some(false),
            ..Config::default()
        };
        for s in 0..60 {
            seed(s);
            let tile = create(&config).unwrap();
            let cell = build(&tile).unwrap();
            assert_eq!(cell.width(), tile.width, "universe width seed {}", s);
        }
    }
}
