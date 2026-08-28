use mrlycore::errors::{value_error, Result};
use mrlycore::state::{boolean, choice, sample, shuffle};
use mrlycore::tile::{
    generals, nestings, powers, products, uniform, Catalog, Group, Parity, Source, Tile,
};

/// The constraints a random tile is drawn under.
#[derive(Clone, Debug)]
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
    /// The forced anti flag for every source, or None to flip coins.
    pub anti: Option<bool>,
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
            anti: None,
        }
    }
}

impl<const N: usize> ConfigNd<N> {
    fn sources(&self) -> Vec<Source> {
        crate::bang::sources(&self.catalog, N)
    }
    fn source(&self) -> Source {
        choice(&self.sources())
    }
}

type Rotation = fn(Source) -> usize;

fn general<const N: usize>(config: &ConfigNd<N>, rotation: Rotation) -> Option<Tile> {
    let numbers = generals(config.min_size, config.max_size, config.parity);
    if numbers.is_empty() {
        return None;
    }
    let n = choice(&numbers);
    let source = config.source();
    let mut tile = Tile::new(Group::General).size(n, n);
    tile.sources = vec![source];
    tile.numbers = vec![n];
    tile.levels = vec![1];
    tile.rotations = vec![rotation(source)];
    tile.factor = n;
    Some(tile)
}

fn fractal<const N: usize>(config: &ConfigNd<N>, rotation: Rotation) -> Option<Tile> {
    let options = powers(config.min_size, config.max_size, config.parity);
    if options.is_empty() {
        return None;
    }
    let (n, level) = choice(&options);
    let source = config.source();
    let size = n.pow(level as u32);
    let mut tile = Tile::new(Group::Fractal).size(size, size);
    tile.sources = vec![source];
    tile.numbers = vec![n];
    tile.levels = vec![level];
    tile.rotations = vec![rotation(source)];
    tile.factor = n;
    Some(tile)
}

fn mixed(numbers: Vec<usize>, sources: &[Source], options: &[Vec<usize>]) -> Vec<usize> {
    if !uniform(sources) || !uniform(&numbers) {
        return numbers;
    }
    let fresh: Vec<Vec<usize>> = options
        .iter()
        .filter(|option| option.len() == numbers.len() && !uniform(option))
        .cloned()
        .collect();
    match fresh.is_empty() {
        true => numbers,
        false => choice(&fresh),
    }
}

fn magic<const N: usize>(config: &ConfigNd<N>, rotation: Rotation) -> Option<Tile> {
    let options = nestings(config.min_size, config.max_size, config.parity);
    if options.is_empty() {
        return None;
    }
    let drawn = choice(&options);
    let sources: Vec<Source> = drawn.iter().map(|_| config.source()).collect();
    let numbers = mixed(drawn, &sources, &options);
    let count = numbers.len();
    let size: usize = numbers.iter().product();
    let mut tile = Tile::new(Group::Magic).size(size, size);
    tile.sources = sources.clone();
    tile.numbers = numbers.clone();
    tile.levels = vec![1; count];
    tile.rotations = sources.iter().map(|&s| rotation(s)).collect();
    tile.factor = numbers[0];
    Some(tile)
}

fn special<const N: usize>(config: &ConfigNd<N>, rotation: Rotation) -> Option<Tile> {
    let options = products(config.min_size, config.max_size, 2, config.parity);
    if options.is_empty() {
        return None;
    }
    let pair = choice(&options);
    let (factor, n) = (pair[0], pair[1]);
    let source = config.source();
    let size = factor * n;
    let mut tile = Tile::new(Group::Special).size(size, size);
    tile.sources = vec![source];
    tile.numbers = vec![n];
    tile.levels = vec![1];
    tile.rotations = vec![rotation(source)];
    tile.factor = factor;
    tile.flip = boolean();
    Some(tile)
}

fn mosaic<const N: usize>(config: &ConfigNd<N>, rotation: Rotation) -> Option<Tile> {
    let palette = config.sources();
    if palette.len() < 3 {
        return None;
    }
    let options = products(config.min_size, config.max_size, 2, config.parity);
    if options.is_empty() {
        return None;
    }
    let pair = choice(&options);
    let (factor, n) = (pair[0], pair[1]);
    let sources = sample(&palette, 3);
    let size = factor * n;
    let mut tile = Tile::new(Group::Mosaic).size(size, size);
    tile.sources = sources.clone();
    tile.numbers = vec![n, n, n];
    tile.levels = vec![1, 1, 1];
    tile.rotations = sources.iter().map(|&s| rotation(s)).collect();
    tile.factor = factor;
    Some(tile)
}

fn creator<const N: usize>(group: Group) -> fn(&ConfigNd<N>, Rotation) -> Option<Tile> {
    match group {
        Group::General => general,
        Group::Fractal => fractal,
        Group::Magic => magic,
        Group::Special => special,
        Group::Mosaic => mosaic,
    }
}

/// Draws a random tile satisfying the config, or an error when no group fits the size constraints.
pub fn create<const N: usize>(config: &ConfigNd<N>, rotation: Rotation) -> Result<Tile> {
    let mut groups = config.groups.clone();
    shuffle(&mut groups);
    let mut tile = None;
    for group in groups {
        if let Some(candidate) = creator::<N>(group)(config, rotation) {
            tile = Some(candidate);
            break;
        }
    }
    let mut tile = match tile {
        Some(tile) => tile,
        None => return value_error("could not generate a tile within the size constraints."),
    };
    let count = tile.sources.len();
    tile.anti = match config.anti {
        Some(flag) => vec![flag; count],
        None => (0..count).map(|_| boolean()).collect(),
    };
    tile.invert = config.invert.unwrap_or_else(boolean);
    Ok(tile)
}

/// Draws a random tile up to the given size under the default config.
pub fn random_tile<const N: usize>(max_size: usize, rotation: Rotation) -> Result<Tile> {
    let config: ConfigNd<N> = ConfigNd {
        max_size,
        ..Default::default()
    };
    create(&config, rotation)
}
