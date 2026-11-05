use crate::recipe::{
    Catalog, Design, Group, Parity, Source, Tile, generals, nestings, powers, products, uniform,
};
use mrly_core::error::{Result, value_error};
use mrly_core::rng::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigNd<const N: usize> {
    pub groups: Vec<Group>,
    pub catalog: Catalog,
    pub min_size: usize,
    pub max_size: usize,
    pub parity: Parity,
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
        mrly_math::bang::sources(&self.catalog, N).unwrap_or_default()
    }
    fn source(&self, rng: &mut Rng) -> Result<Source> {
        Ok(*rng.choice(&self.sources())?)
    }
}

pub fn random_design(rng: &mut Rng) -> Design {
    const DRAWN: [Design; 4] = [Design::Carpet, Design::Net, Design::Vtree, Design::Void];
    DRAWN[rng.below(DRAWN.len())]
}

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
