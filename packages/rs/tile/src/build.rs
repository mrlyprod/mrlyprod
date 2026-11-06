use crate::draw::ConfigNd;
use crate::recipe::{Group, Tile};

pub type Config2d = ConfigNd<2>;

pub type Config3d = ConfigNd<3>;

pub use six::{HexTile, build as build_6d, create as create_6d, random_tile as random_tile_6d};
pub use three::{build as build_3d, create as create_3d, random_tile as random_tile_3d};
pub use two::{build as build_2d, create as create_2d, random_tile as random_tile_2d, tree_mask};

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

mod two {
    use super::Config2d as Config;
    use crate::draw as spec;
    use crate::recipe::{Group, Source, Tile};
    use mrly_error::{Result, value_error};
    use mrly_math::code::Code;
    use mrly_rand::Rng;
    use mrly_tensor::Tensor;
    use mrly_two::{Cell2d, designs, geometry};

    fn rotation(source: Source, rng: &mut Rng) -> usize {
        match source {
            Source::Classic(design) => spec::random_rotation(design, rng) as usize,
            Source::Code(_) => rng.below(4),
        }
    }

    pub fn create(config: &Config, rng: &mut Rng) -> Result<Tile> {
        spec::create(config, rotation, rng)
    }

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
        source_cell(tile.sources[i], tile.numbers[i], level, tile.rotations[i])
    }

    pub fn tree_mask(n: usize) -> Result<Tensor> {
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
}

mod three {
    use super::Config3d as Config;
    use crate::draw as spec;
    use crate::recipe::{Design, Group, Source, Tile};
    use mrly_error::{Result, value_error};
    use mrly_math::code::Code;
    use mrly_rand::Rng;
    use mrly_tensor::Tensor;
    use mrly_three::{Cell3d, designs, geometry};

    fn rotation(_: Source, rng: &mut Rng) -> usize {
        rng.below(24)
    }

    pub fn create(config: &Config, rng: &mut Rng) -> Result<Tile> {
        spec::create(config, rotation, rng)
    }

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
        source_cell(tile.sources[i], tile.numbers[i], level, tile.rotations[i])
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
}

mod six {
    use super::Config3d as Config;
    use super::three;
    use crate::recipe::Tile;
    use mrly_error::Result;
    use mrly_rand::Rng;
    use mrly_six::geometry::{cut, iso, pro};
    use mrly_six::{Cell6d, Projection};

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct HexTile {
        pub projection: Projection,
        pub tile: Tile,
    }

    fn projection(rng: &mut Rng) -> Result<Projection> {
        Ok(*rng.choice(&[Projection::Iso, Projection::Pro, Projection::Cut])?)
    }

    pub fn create(config: &Config, rng: &mut Rng) -> Result<HexTile> {
        Ok(HexTile {
            projection: projection(rng)?,
            tile: three::create(config, rng)?,
        })
    }

    pub fn random_tile(max_size: usize, rng: &mut Rng) -> Result<HexTile> {
        Ok(HexTile {
            projection: projection(rng)?,
            tile: three::random_tile(max_size, rng)?,
        })
    }

    pub fn build(hex: &HexTile) -> Result<Cell6d> {
        let cell = three::build(&hex.tile)?;
        match hex.projection {
            Projection::Iso => iso(&cell),
            Projection::Pro => pro(&cell),
            Projection::Cut => cut(&cell),
        }
    }
}
