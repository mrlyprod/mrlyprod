use crate::gen::draw::ConfigNd;
use crate::gen::recipe::{Group, Tile};

/// The constraints a random flat tile is drawn under.
pub type Config2d = ConfigNd<2>;

/// The constraints a random cube tile is drawn under, shared by the hex pipeline.
pub type Config3d = ConfigNd<3>;

pub use six::{build as build_6d, create as create_6d, random_tile as random_tile_6d, HexTile};
pub use three::{build as build_3d, create as create_3d, random_tile as random_tile_3d};
pub use two::{build as build_2d, create as create_2d, random_tile as random_tile_2d};

// SLOTS

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

// TWO

mod two {
    use super::Config2d as Config;
    use crate::core::error::{value_error, Result};
    use crate::core::rng::Rng;
    use crate::core::tensor::Tensor;
    use crate::gen::draw as spec;
    use crate::gen::recipe::{Group, Source, Tile};
    use crate::math::bang::Code;
    use crate::math::two::{designs, geometry, Cell2d};

    fn rotation(rng: &mut Rng) -> usize {
        rng.below(4)
    }

    /// Draws a random flat tile from the stream, rotations from the four quarter-turns.
    ///
    /// # Errors
    ///
    /// Errs when no allowed group fits the size constraints, or the catalog holds no source.
    pub fn create(config: &Config, rng: &mut Rng) -> Result<Tile> {
        spec::create(config, rotation, rng)
    }

    /// Draws a random flat tile up to the given size under the default config.
    ///
    /// # Errors
    ///
    /// Errs when no group fits a tile inside the size, or the catalog holds no source.
    pub fn random_tile(max_size: usize, rng: &mut Rng) -> Result<Tile> {
        spec::random_tile::<2>(max_size, rotation, rng)
    }

    fn source_cell(source: Source, number: usize, level: usize, rotation: usize) -> Result<Cell2d> {
        match source {
            Source::Classic(design) => designs::named(design, number, level, rotation),
            Source::Code(code) => designs::create(Code::from(code), number, level, rotation, 2),
        }
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
        let horizontal = vertical.clone().rotate(1)?;
        let v = vertical.types();
        let h = horizontal.types();
        let mut data = vec![0u8; v.size()];
        for (flat, item) in data.iter_mut().enumerate() {
            let a = v.at(flat);
            let b = h.at(flat);
            *item = match (a, b) {
                (1, 1) => 2,
                (1, _) | (_, 1) => 1,
                _ => 0,
            };
        }
        Tensor::of(data, v.shape.clone())
    }

    fn build_general(tile: &Tile) -> Result<Cell2d> {
        cell(tile, 0, 1)
    }

    fn build_fractal(tile: &Tile) -> Result<Cell2d> {
        cell(tile, 0, tile.levels[0])
    }

    fn build_magic(tile: &Tile) -> Result<Cell2d> {
        let cells: Result<Vec<Cell2d>> =
            (0..tile.sources.len()).map(|i| cell(tile, i, 1)).collect();
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

    /// Builds the flat cell the tile describes.
    ///
    /// ```
    /// use mrlyrs::core::rng::Rng;
    /// use mrlyrs::gen::build::{build_2d, random_tile_2d};
    /// let mut rng = Rng::new(1);
    /// let tile = random_tile_2d(9, &mut rng)?;
    /// assert_eq!(build_2d(&tile)?.width(), tile.width);
    /// # Ok::<(), mrlyrs::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Errs when the tile's slots are ragged, or when a source will not render at its size.
    pub fn build(tile: &Tile) -> Result<Cell2d> {
        if super::ragged(tile) {
            return value_error("tile slots are ragged.");
        }
        let mut c = builder(tile.group)(tile)?;
        if tile.invert {
            c = c.invert();
        }
        Ok(c)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::gen::recipe::{Catalog, Design, Parity};
        #[test]
        fn random_tile_respects_max() {
            for s in 0..50 {
                let mut rng = Rng::new(s);
                let tile = random_tile(30, &mut rng).unwrap();
                assert!(tile.max_size() <= 30);
            }
        }
        #[test]
        fn magic_can_nest_deeper_than_two() {
            let config = Config {
                min_size: 3,
                max_size: 300,
                groups: vec![Group::Magic],
                anti: Some(false),
                ..Config::default()
            };
            let mut deep = false;
            for s in 0..200 {
                let mut rng = Rng::new(s);
                if let Ok(tile) = create(&config, &mut rng) {
                    if tile.sources.len() >= 3 {
                        deep = true;
                        build(&tile).unwrap();
                    }
                }
            }
            assert!(deep, "expected at least one magic tile nested 3+ deep");
        }
        #[test]
        fn magic_rolls_never_repeat_a_fractal() {
            let config = Config {
                catalog: Catalog::Codes(vec![7]),
                min_size: 3,
                max_size: 64,
                groups: vec![Group::Magic],
                anti: Some(false),
                ..Config::default()
            };
            for s in 0..200 {
                let mut rng = Rng::new(s);
                let tile = create(&config, &mut rng).unwrap();
                assert!(tile.sources.len() >= 2, "seed {s} rolled one slot");
                assert!(!tile.degenerate(), "seed {s} rolled a fractal twin");
                let cell = build(&tile).unwrap();
                assert_eq!(cell.width(), tile.width, "seed {s}");
            }
        }
        #[test]
        fn a_magic_roll_keeps_its_twin_when_nothing_else_fits() {
            let config = Config {
                catalog: Catalog::Codes(vec![7]),
                min_size: 9,
                max_size: 9,
                groups: vec![Group::Magic],
                anti: Some(false),
                ..Config::default()
            };
            for s in 0..20 {
                let mut rng = Rng::new(s);
                let tile = create(&config, &mut rng).unwrap();
                assert_eq!(tile.numbers, vec![3, 3], "seed {s}");
                assert!(tile.degenerate(), "seed {s}");
                assert_eq!(build(&tile).unwrap().width(), 9, "seed {s}");
            }
        }
        #[test]
        fn refuses_a_flat_tile_it_cannot_build() {
            use crate::core::json;
            let parsed: Tile = serde_json::from_value(json!({
                "group": "General", "factor": 0,
                "sources": [{ "design": "Carpet" }],
                "numbers": [], "levels": [], "rotations": [], "anti": [],
                "invert": false, "flip": false, "width": 0, "height": 0,
            }))
            .unwrap();
            assert!(build(&parsed).is_err());
            let mut bare = Tile::new(Group::Mosaic);
            bare.sources = vec![Source::Classic(Design::Carpet)];
            assert!(build(&bare).is_err());
            assert!(build(&Tile::new(Group::General)).is_err());
            let mut cubic = Tile::new(Group::General);
            cubic.sources = vec![Source::Classic(Design::Xtree)];
            cubic.numbers = vec![3];
            cubic.levels = vec![1];
            cubic.rotations = vec![0];
            cubic.anti = vec![false];
            cubic.resize();
            assert!(build(&cubic).is_err());
        }
        #[test]
        fn evens_parity_builds() {
            let config = Config {
                min_size: 4,
                max_size: 64,
                parity: Parity::Evens,
                groups: vec![Group::General],
                anti: Some(false),
                ..Config::default()
            };
            for s in 0..50 {
                let mut rng = Rng::new(s);
                let tile = create(&config, &mut rng).unwrap();
                assert_eq!(tile.numbers[0] % 2, 0);
                let cell = build(&tile).unwrap();
                assert_eq!(cell.width(), tile.width);
            }
        }
    }
}

// THREE

mod three {
    use super::Config3d as Config;
    use crate::core::error::{value_error, Result};
    use crate::core::rng::Rng;
    use crate::core::tensor::Tensor;
    use crate::gen::draw as spec;
    use crate::gen::recipe::{Design, Group, Source, Tile};
    use crate::math::bang::Code;
    use crate::math::three::{designs, geometry, Cell3d};

    fn rotation(rng: &mut Rng) -> usize {
        rng.below(24)
    }

    /// Draws a cube tile from the config with cube orientations drawn from the stream.
    ///
    /// # Errors
    ///
    /// Errs when no allowed group fits the size constraints, or the catalog holds no source.
    pub fn create(config: &Config, rng: &mut Rng) -> Result<Tile> {
        spec::create(config, rotation, rng)
    }

    /// Draws a random cube tile up to the given size.
    ///
    /// # Errors
    ///
    /// Errs when no group fits a tile inside the size, or the catalog holds no source.
    pub fn random_tile(max_size: usize, rng: &mut Rng) -> Result<Tile> {
        spec::random_tile::<3>(max_size, rotation, rng)
    }

    fn design_cell(design: Design, number: usize, level: usize) -> Result<Cell3d> {
        match design {
            Design::Carpet => designs::carpet(number, level),
            Design::Net => designs::net(number, level),
            Design::Xtree => designs::xtree(number, level),
            Design::Ytree => designs::ytree(number, level),
            Design::Ztree => designs::ztree(number, level),
            Design::Void => designs::void(number, level),
            Design::Point => designs::point(number, level),
            Design::Dust => designs::dust(number, level),
            Design::Xline => designs::xline(number, level),
            Design::Yline => designs::yline(number, level),
            Design::Zline => designs::zline(number, level),
            Design::Star => designs::star(number, level),
            other => value_error(format!("design {} is not 3d.", other.name())),
        }
    }

    fn source_cell(source: Source, number: usize, level: usize, rotation: usize) -> Result<Cell3d> {
        let mut c = match source {
            Source::Classic(design) => design_cell(design, number, level)?,
            Source::Code(code) => designs::create(Code::from(code), number, level, 2)?,
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
            .bytes()?
            .iter()
            .map(|&v| if v == 1 { fill } else { 0 })
            .collect();
        Tensor::of(data, t.shape.clone())
    }

    fn index_mask(n: usize) -> Result<Tensor> {
        let x = designs::xtree(n, 1)?;
        let y = designs::ytree(n, 1)?;
        let z = designs::ztree(n, 1)?;
        let (xt, yt, zt) = (x.types(), y.types(), z.types());
        let mut data = vec![0u8; xt.size()];
        for (flat, item) in data.iter_mut().enumerate() {
            *item = if zt.at(flat) == 1 {
                2
            } else if yt.at(flat) == 1 {
                1
            } else {
                0
            };
        }
        Tensor::of(data, xt.shape.clone())
    }

    fn build_general(tile: &Tile) -> Result<Cell3d> {
        cell(tile, 0, 1)
    }

    fn build_fractal(tile: &Tile) -> Result<Cell3d> {
        cell(tile, 0, tile.levels[0])
    }

    fn build_magic(tile: &Tile) -> Result<Cell3d> {
        let cells: Result<Vec<Cell3d>> =
            (0..tile.sources.len()).map(|i| cell(tile, i, 1)).collect();
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

    /// Builds the cube the tile describes.
    ///
    /// # Errors
    ///
    /// Errs when the tile's slots are ragged, or when a source is flat or will not render.
    pub fn build(tile: &Tile) -> Result<Cell3d> {
        if super::ragged(tile) {
            return value_error("tile slots are ragged.");
        }
        let mut c = builder(tile.group)(tile)?;
        if tile.invert {
            c = c.invert();
        }
        Ok(c)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::gen::recipe::Catalog;
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
            let config = config();
            for s in 0..200 {
                let mut rng = Rng::new(s);
                let tile = create(&config, &mut rng).unwrap();
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
        fn create_replays_its_seed() {
            let a = create(&config(), &mut Rng::new(321)).unwrap();
            let b = create(&config(), &mut Rng::new(321)).unwrap();
            assert_eq!(a, b);
            assert_ne!(a, create(&config(), &mut Rng::new(322)).unwrap());
        }
        #[test]
        fn classics_use_named_designs() {
            let config = config();
            for s in 0..50 {
                let mut rng = Rng::new(s);
                let tile = create(&config, &mut rng).unwrap();
                for source in &tile.sources {
                    assert!(matches!(source, Source::Classic(_)));
                }
            }
        }
        #[test]
        fn refuses_a_cube_tile_it_cannot_build() {
            assert!(build(&Tile::new(Group::General)).is_err());
            let mut bare = Tile::new(Group::Mosaic);
            bare.sources = vec![Source::Classic(Design::Carpet)];
            assert!(build(&bare).is_err());
            let mut flat = Tile::new(Group::General);
            flat.sources = vec![Source::Classic(Design::Htree)];
            flat.numbers = vec![3];
            flat.levels = vec![1];
            flat.rotations = vec![0];
            flat.anti = vec![false];
            flat.resize();
            assert!(build(&flat).is_err());
        }
        #[test]
        fn universe_builds_from_codes() {
            let config = Config {
                catalog: Catalog::Universe,
                min_size: 3,
                max_size: 9,
                anti: Some(false),
                ..Config::default()
            };
            for s in 0..60 {
                let mut rng = Rng::new(s);
                let tile = create(&config, &mut rng).unwrap();
                let cell = build(&tile).unwrap();
                assert_eq!(cell.width(), tile.width, "universe width seed {}", s);
                for source in &tile.sources {
                    assert!(matches!(source, Source::Code(_)));
                }
            }
        }
    }
}

// SIX

mod six {
    use super::three;
    use super::Config3d as Config;
    use crate::core::error::Result;
    use crate::core::rng::Rng;
    use crate::gen::recipe::Tile;
    use crate::math::six::geometry::{cut, iso, pro};
    use crate::math::six::{Cell6d, Projection};

    /// A cube tile paired with the projection that flattens it.
    #[derive(Clone, Debug)]
    pub struct HexTile {
        /// The projection that flattens the tile.
        pub projection: Projection,
        /// The cube tile underneath.
        pub tile: Tile,
    }

    fn projection(rng: &mut Rng) -> Result<Projection> {
        Ok(*rng.choice(&[Projection::Iso, Projection::Pro, Projection::Cut])?)
    }

    /// Draws a cube tile from the config under a projection drawn from the stream.
    ///
    /// # Errors
    ///
    /// Errs when no allowed group fits the size constraints, or the catalog holds no source.
    pub fn create(config: &Config, rng: &mut Rng) -> Result<HexTile> {
        Ok(HexTile {
            projection: projection(rng)?,
            tile: three::create(config, rng)?,
        })
    }

    /// Draws a random cube tile up to the given size under a random projection.
    ///
    /// # Errors
    ///
    /// Errs when no group fits a tile inside the size, or the catalog holds no source.
    pub fn random_tile(max_size: usize, rng: &mut Rng) -> Result<HexTile> {
        Ok(HexTile {
            projection: projection(rng)?,
            tile: three::random_tile(max_size, rng)?,
        })
    }

    /// Builds the tile's cube and flattens it through its projection.
    ///
    /// # Errors
    ///
    /// Errs when the cube will not build, or when the projection will not flatten it.
    pub fn build(hex: &HexTile) -> Result<Cell6d> {
        let cell = three::build(&hex.tile)?;
        match hex.projection {
            Projection::Iso => iso(&cell),
            Projection::Pro => pro(&cell),
            Projection::Cut => cut(&cell),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::gen::recipe::Group;
        fn config() -> Config {
            Config {
                min_size: 3,
                max_size: 9,
                anti: Some(false),
                ..Config::default()
            }
        }
        #[test]
        fn projects_every_group_in_every_projection() {
            let config = config();
            for s in 0..40 {
                let mut rng = Rng::new(s);
                let hex = create(&config, &mut rng).unwrap();
                let cell = build(&hex).unwrap();
                assert!(
                    cell.width() > 0,
                    "empty width seed {} {:?}",
                    s,
                    hex.tile.group
                );
                assert!(cell.height() > 0, "empty height seed {}", s);
            }
        }
        #[test]
        fn refuses_a_hex_tile_it_cannot_build() {
            let bare = HexTile {
                projection: Projection::Iso,
                tile: Tile::new(Group::General),
            };
            assert!(build(&bare).is_err());
        }
        #[test]
        fn magic_projects() {
            let config = Config {
                min_size: 3,
                max_size: 15,
                groups: vec![Group::Magic],
                anti: Some(false),
                ..Config::default()
            };
            let mut built = 0;
            for s in 0..30 {
                let mut rng = Rng::new(s);
                if let Ok(hex) = create(&config, &mut rng) {
                    let cell = build(&hex).unwrap();
                    assert!(cell.width() > 0);
                    built += 1;
                }
            }
            assert!(built > 0, "expected magic tiles to project");
        }
    }
}
