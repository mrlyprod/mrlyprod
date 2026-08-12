use super::geometry::{cut, iso, pro};
use super::models::Cell6d;
use super::Projection;
use crate::three;
use mrlycore::errors::Result;
use mrlycore::state::choice;
use mrlycore::tile::Tile;

/// The tile configuration the hex pipeline shares with the 3d builder.
pub type Config = three::tile::Config;

/// A 3d tile paired with the projection that flattens it.
#[derive(Clone, Debug)]
pub struct HexTile {
    /// The projection that flattens the tile.
    pub projection: Projection,
    /// The 3d tile underneath.
    pub tile: Tile,
}

fn projection() -> Projection {
    choice(&[Projection::Iso, Projection::Pro, Projection::Cut])
}

/// Draws a 3d tile from the config under a random projection.
pub fn create(config: &Config) -> Result<HexTile> {
    Ok(HexTile {
        projection: projection(),
        tile: three::tile::create(config)?,
    })
}

/// Draws a random 3d tile up to the given size under a random projection.
pub fn random_tile(max_size: usize) -> Result<HexTile> {
    Ok(HexTile {
        projection: projection(),
        tile: three::tile::random_tile(max_size)?,
    })
}

/// Builds the tile's cube and flattens it through its projection.
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
