use crate::core::error::{value_error, Result};
use crate::core::rng::Rng;
use crate::gen::recipe::{
    generals, nestings, powers, products, uniform, Catalog, Design, Group, Parity, Source, Tile,
};
use serde::{Deserialize, Serialize};

/// The constraints a random tile is drawn under.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigNd<const N: usize> {
    /// The tile groups allowed.
    pub groups: Vec<Group>,
    /// The catalog the sources are drawn from.
    pub catalog: Catalog,
    /// The smallest allowed side.
    pub min_size: usize,
    /// The largest allowed side.
    pub max_size: usize,
    /// The parity the sizes must keep.
    pub parity: Parity,
    /// The forced inversion flag, or None to flip a coin.
    pub invert: Option<bool>,
}

impl<const N: usize> Default for ConfigNd<N> {
    fn default() -> ConfigNd<N> {
        ConfigNd {
            groups: Group::all().to_vec(),
            catalog: Catalog::Classics,
            min_size: 3,
            max_size: 9,
            parity: Parity::Odds,
            invert: None,
        }
    }
}

impl<const N: usize> ConfigNd<N> {
    fn sources(&self) -> Vec<Source> {
        crate::math::bang::sources(&self.catalog, N).unwrap_or_default()
    }
    fn source(&self, rng: &mut Rng) -> Result<Source> {
        Ok(*rng.choice(&self.sources())?)
    }
}

/// Draws one named design from the stream.
pub fn random_design(rng: &mut Rng) -> Design {
    let designs = Design::all();
    designs[rng.below(designs.len())]
}

/// Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3.
pub fn random_rotation(design: Design, rng: &mut Rng) -> u8 {
    let turns = match design {
        Design::Htree | Design::Vtree | Design::Xtree | Design::Ytree | Design::Ztree => 2,
        _ => 4,
    };
    rng.below(turns) as u8
}

type Rotation = fn(Source, &mut Rng) -> usize;

fn general<const N: usize>(
    config: &ConfigNd<N>,
    rotation: Rotation,
    rng: &mut Rng,
) -> Result<Option<Tile>> {
    let numbers = generals(config.min_size, config.max_size, config.parity);
    if numbers.is_empty() {
        return Ok(None);
    }
    let n = *rng.choice(&numbers)?;
    let source = config.source(rng)?;
    let mut tile = Tile::new(Group::General).size(n, n);
    tile.sources = vec![source];
    tile.numbers = vec![n];
    tile.levels = vec![1];
    tile.rotations = vec![rotation(source, rng)];
    tile.factor = n;
    Ok(Some(tile))
}

fn fractal<const N: usize>(
    config: &ConfigNd<N>,
    rotation: Rotation,
    rng: &mut Rng,
) -> Result<Option<Tile>> {
    let options = powers(config.min_size, config.max_size, config.parity);
    if options.is_empty() {
        return Ok(None);
    }
    let (n, level) = *rng.choice(&options)?;
    let source = config.source(rng)?;
    let size = n.pow(level as u32);
    let mut tile = Tile::new(Group::Fractal).size(size, size);
    tile.sources = vec![source];
    tile.numbers = vec![n];
    tile.levels = vec![level];
    tile.rotations = vec![rotation(source, rng)];
    tile.factor = n;
    Ok(Some(tile))
}

fn mixed(
    numbers: Vec<usize>,
    sources: &[Source],
    options: &[Vec<usize>],
    rng: &mut Rng,
) -> Result<Vec<usize>> {
    if !uniform(sources) || !uniform(&numbers) {
        return Ok(numbers);
    }
    let fresh: Vec<Vec<usize>> = options
        .iter()
        .filter(|option| option.len() == numbers.len() && !uniform(option))
        .cloned()
        .collect();
    match fresh.is_empty() {
        true => Ok(numbers),
        false => Ok(rng.choice(&fresh)?.clone()),
    }
}

fn magic<const N: usize>(
    config: &ConfigNd<N>,
    rotation: Rotation,
    rng: &mut Rng,
) -> Result<Option<Tile>> {
    let options = nestings(config.min_size, config.max_size, config.parity);
    if options.is_empty() {
        return Ok(None);
    }
    let drawn = rng.choice(&options)?.clone();
    let sources: Vec<Source> = drawn
        .iter()
        .map(|_| config.source(rng))
        .collect::<Result<Vec<Source>>>()?;
    let numbers = mixed(drawn, &sources, &options, rng)?;
    let count = numbers.len();
    let size: usize = numbers.iter().product();
    let mut tile = Tile::new(Group::Magic).size(size, size);
    tile.sources = sources.clone();
    tile.numbers = numbers.clone();
    tile.levels = vec![1; count];
    tile.rotations = sources.iter().map(|&s| rotation(s, rng)).collect();
    tile.factor = numbers[0];
    Ok(Some(tile))
}

fn special<const N: usize>(
    config: &ConfigNd<N>,
    rotation: Rotation,
    rng: &mut Rng,
) -> Result<Option<Tile>> {
    let options = products(config.min_size, config.max_size, 2, config.parity);
    if options.is_empty() {
        return Ok(None);
    }
    let pair = rng.choice(&options)?;
    let (factor, n) = (pair[0], pair[1]);
    let source = config.source(rng)?;
    let size = factor * n;
    let mut tile = Tile::new(Group::Special).size(size, size);
    tile.sources = vec![source];
    tile.numbers = vec![n];
    tile.levels = vec![1];
    tile.rotations = vec![rotation(source, rng)];
    tile.factor = factor;
    tile.flip = rng.boolean();
    Ok(Some(tile))
}

fn mosaic<const N: usize>(
    config: &ConfigNd<N>,
    rotation: Rotation,
    rng: &mut Rng,
) -> Result<Option<Tile>> {
    let palette = config.sources();
    if palette.len() < 3 {
        return Ok(None);
    }
    let options = products(config.min_size, config.max_size, 2, config.parity);
    if options.is_empty() {
        return Ok(None);
    }
    let pair = rng.choice(&options)?;
    let (factor, n) = (pair[0], pair[1]);
    let sources: Vec<Source> = rng
        .sample_indices(palette.len(), 3)
        .into_iter()
        .map(|i| palette[i])
        .collect();
    let size = factor * n;
    let mut tile = Tile::new(Group::Mosaic).size(size, size);
    tile.sources = sources.clone();
    tile.numbers = vec![n, n, n];
    tile.levels = vec![1, 1, 1];
    tile.rotations = sources.iter().map(|&s| rotation(s, rng)).collect();
    tile.factor = factor;
    Ok(Some(tile))
}

type Creator<const N: usize> = fn(&ConfigNd<N>, Rotation, &mut Rng) -> Result<Option<Tile>>;

fn creator<const N: usize>(group: Group) -> Creator<N> {
    match group {
        Group::General => general,
        Group::Fractal => fractal,
        Group::Magic => magic,
        Group::Special => special,
        Group::Mosaic => mosaic,
    }
}

/// Draws a random tile satisfying the config from the stream.
///
/// ```
/// use mrlyrs::core::rng::Rng;
/// use mrlyrs::gen::draw::{create, ConfigNd};
/// let mut rng = Rng::new(1);
/// let tile = create(&ConfigNd::<2>::default(), |_, rng| rng.below(4), &mut rng)?;
/// assert!((3..=9).contains(&tile.max_size()));
/// # Ok::<(), mrlyrs::Error>(())
/// ```
///
/// # Errors
///
/// Errs when no allowed group fits the size constraints, or when the catalog holds no source.
pub fn create<const N: usize>(
    config: &ConfigNd<N>,
    rotation: Rotation,
    rng: &mut Rng,
) -> Result<Tile> {
    let mut groups = config.groups.clone();
    rng.shuffle(&mut groups);
    let mut tile = None;
    for group in groups {
        if let Some(candidate) = creator::<N>(group)(config, rotation, rng)? {
            tile = Some(candidate);
            break;
        }
    }
    let mut tile = match tile {
        Some(tile) => tile,
        None => return value_error("could not generate a tile within the size constraints."),
    };
    tile.invert = config.invert.unwrap_or_else(|| rng.boolean());
    Ok(tile)
}

/// Draws a random tile up to the given size under the default config.
///
/// # Errors
///
/// Errs when no group fits a tile inside the size, or when the catalog holds no source.
pub fn random_tile<const N: usize>(
    max_size: usize,
    rotation: Rotation,
    rng: &mut Rng,
) -> Result<Tile> {
    let config: ConfigNd<N> = ConfigNd {
        max_size,
        ..Default::default()
    };
    create(&config, rotation, rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn turn(_: Source, rng: &mut Rng) -> usize {
        rng.below(4)
    }
    #[test]
    fn refuses_a_config_that_draws_no_tile() {
        let mut rng = Rng::new(1);
        let narrow: ConfigNd<2> = ConfigNd {
            min_size: 9,
            max_size: 3,
            ..Default::default()
        };
        assert!(create(&narrow, turn, &mut rng).is_err());
        let groupless: ConfigNd<2> = ConfigNd {
            groups: Vec::new(),
            ..Default::default()
        };
        assert!(create(&groupless, turn, &mut rng).is_err());
        let sourceless: ConfigNd<2> = ConfigNd {
            catalog: Catalog::Codes(Vec::new()),
            ..Default::default()
        };
        assert!(create(&sourceless, turn, &mut rng).is_err());
        assert!(random_tile::<2>(1, turn, &mut rng).is_err());
    }
    #[test]
    fn a_named_catalog_draws_only_its_designs() {
        let config: ConfigNd<2> = ConfigNd {
            catalog: Catalog::Designs(vec![Design::Carpet, Design::Net]),
            ..Default::default()
        };
        for s in 0..64 {
            let mut rng = Rng::new(s);
            let tile = crate::gen::build::create_2d(&config, &mut rng).unwrap();
            for source in &tile.sources {
                assert!(
                    matches!(source, Source::Classic(Design::Carpet | Design::Net)),
                    "seed {s}"
                );
            }
        }
    }
    #[test]
    fn a_tree_turns_a_half() {
        let config: ConfigNd<2> = ConfigNd {
            catalog: Catalog::Designs(vec![Design::Vtree]),
            ..Default::default()
        };
        for s in 0..64 {
            let mut rng = Rng::new(s);
            let tile = crate::gen::build::create_2d(&config, &mut rng).unwrap();
            assert!(tile.rotations.iter().all(|&turn| turn < 2), "seed {s}");
        }
    }
}
