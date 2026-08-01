use super::geometry::{cut, iso, pro};
use super::models::Cell6d;
use super::Projection;
use crate::three;
use mrlycore::errors::Result;
use mrlycore::state::choice;
use mrlycore::tile::Tile;

pub type Config = three::tile::Config;

#[derive(Clone, Debug)]
pub struct HexTile {
    pub projection: Projection,
    pub tile: Tile,
}

fn projection() -> Projection {
    choice(&[Projection::Iso, Projection::Pro, Projection::Cut])
}

pub fn create(config: &Config) -> Result<HexTile> {
    Ok(HexTile {
        projection: projection(),
        tile: three::tile::create(config)?,
    })
}

pub fn random_tile(max_size: usize) -> Result<HexTile> {
    Ok(HexTile {
        projection: projection(),
        tile: three::tile::random_tile(max_size)?,
    })
}

pub fn build(hex: &HexTile) -> Result<Cell6d> {
    let cell = three::tile::build(&hex.tile)?;
    match hex.projection {
        Projection::Iso => iso(&cell),
        Projection::Pro => pro(&cell),
        Projection::Cut => cut(&cell),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::{guard as rng_lock, seed};
    use mrlycore::tile::Group;
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
        let _guard = rng_lock();
        let config = config();
        for s in 0..40 {
            seed(s);
            let hex = create(&config).unwrap();
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
    fn magic_projects() {
        let _guard = rng_lock();
        let config = Config {
            min_size: 3,
            max_size: 15,
            groups: vec![Group::Magic],
            anti: Some(false),
            ..Config::default()
        };
        let mut built = 0;
        for s in 0..30 {
            seed(s);
            if let Ok(hex) = create(&config) {
                let cell = build(&hex).unwrap();
                assert!(cell.width() > 0);
                built += 1;
            }
        }
        assert!(built > 0, "expected magic tiles to project");
    }
    #[test]
    fn create_is_seeded() {
        let _guard = rng_lock();
        seed(555);
        let a = create(&config()).unwrap();
        seed(555);
        let b = create(&config()).unwrap();
        assert_eq!(a.projection, b.projection);
        assert_eq!(a.tile, b.tile);
    }
}
