use crate::two::{self, tile as tile2d};
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::paint::{self as engine, Config as PaintConfig, Edition, Ink, Paint};
use mrlycore::state::{randint, seed};
use mrlycore::tile::Tile;
use mrlycore::{json, Json};

/// One rendering of an artwork, sized in tile repetitions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    /// The count of tile repetitions across.
    pub width: usize,
    /// The count of tile repetitions down.
    pub height: usize,
    /// The encoded PNG bytes, empty until rendered.
    pub png: Vec<u8>,
}

impl File {
    /// Builds a file of the given repetition counts with no PNG bytes.
    pub fn new(width: usize, height: usize) -> File {
        File {
            width,
            height,
            png: Vec::new(),
        }
    }
    /// Returns the file's width and height as JSON.
    pub fn to_json(&self) -> Json {
        json!({ "width": self.width, "height": self.height })
    }
}

/// The settings an artwork is drawn under.
#[derive(Clone, Debug)]
pub struct Config {
    /// The constraints the tile is drawn under.
    pub tile: tile2d::Config,
    /// The constraints the paint is drawn under.
    pub paint: PaintConfig,
    /// The width and height repetition pairs to render.
    pub files: Vec<(usize, usize)>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tile: tile2d::Config::default(),
            paint: PaintConfig::default(),
            files: vec![(1, 1), (3, 3), (5, 5)],
        }
    }
}

/// One seeded artwork, from tile recipe to rendered files.
#[derive(Clone, Debug)]
pub struct Variation {
    /// The random hex identifier.
    pub key: String,
    /// The seed the variation is drawn under.
    pub seed: u64,
    /// The paint edition.
    pub edition: Edition,
    /// The primary inks, when the config fixes them.
    pub primaries: Option<Vec<Ink>>,
    /// The tile recipe.
    pub tile: Tile,
    /// The mask tile, present only under the Neighbors edition.
    pub mask: Option<Tile>,
    /// The paint, set by generate.
    pub paint: Option<Paint>,
    /// The built base cell, set by generate.
    pub base: Option<two::Cell2d>,
    /// The renderings, filled by render.
    pub files: Vec<File>,
}

impl Variation {
    /// Returns whether the edition paints the whole tiled canvas.
    pub fn is_cover(&self) -> bool {
        matches!(
            self.edition,
            Edition::Rows | Edition::Columns | Edition::Random
        )
    }
    /// Returns whether the edition paints the base cell before tiling.
    pub fn is_prime(&self) -> bool {
        matches!(self.edition, Edition::Layers | Edition::Neighbors)
    }
    /// Returns the variation's key, seed, tile, mask and files as JSON.
    pub fn to_json(&self) -> Json {
        json!({
            "v": 1,
            "key": &self.key,
            "seed": self.seed,
            "tile": self.tile.to_json(),
            "mask": self.mask.as_ref().map(|m| m.to_json()),
            "files": self.files.iter().map(|f| f.to_json()).collect::<Vec<_>>(),
        })
    }
}

fn hex_key(length: usize) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    (0..length)
        .map(|_| DIGITS[randint(0, 15) as usize] as char)
        .collect()
}

fn pop_center(tile: &Tile, cell: &mut two::Cell2d) {
    let _ = tile;
    let center = cell.width() / 2;
    let mut types = cell.cell.types.clone();
    types.set(&[center, center], 0);
    cell.cell.types = types;
}

/// Draws a fresh seeded variation under the config, with a mask when the edition is Neighbors.
pub fn create(config: &Config) -> Result<Variation> {
    let s = randint(0, i64::MAX) as u64;
    seed(s);
    let edition = engine::random_edition(config.paint.editions.as_deref());
    let tile = tile2d::create(&config.tile)?;
    let mask = if edition == Edition::Neighbors {
        let mask_config = tile2d::Config {
            min_size: 3,
            max_size: 3,
            ..config.tile.clone()
        };
        Some(tile2d::create(&mask_config)?)
    } else {
        None
    };
    Ok(Variation {
        key: hex_key(8),
        seed: s,
        edition,
        primaries: config.paint.primaries.clone(),
        tile,
        mask,
        paint: None,
        base: None,
        files: config.files.iter().map(|&(w, h)| File::new(w, h)).collect(),
    })
}

/// Builds the variation's base cell and paint, painting the base under a prime edition.
pub fn generate(mut variation: Variation, config: &Config) -> Result<Variation> {
    let _ = config;
    let mut base = tile2d::build(&variation.tile)?;
    let paint_config = PaintConfig {
        editions: Some(vec![variation.edition]),
        primaries: variation.primaries.clone(),
        target: None,
    };
    if variation.is_cover() {
        let p = engine::setup(Paint::new(variation.edition), &paint_config);
        variation.paint = Some(p);
    } else {
        let mask_tensor = match &variation.mask {
            Some(mask_tile) => {
                let mut mask_cell = tile2d::build(mask_tile)?;
                pop_center(mask_tile, &mut mask_cell);
                Some(mask_cell.cell.types.clone())
            }
            None => None,
        };
        let mut cell = base.cell.clone();
        let p = engine::paint(&mut cell, &paint_config, mask_tensor.as_ref())?;
        base.cell = cell;
        variation.paint = Some(p);
    }
    variation.base = Some(base);
    Ok(variation)
}

/// Renders every file of the variation to PNG at the given scale, or an error before generate.
pub fn render(mut variation: Variation, scale: usize) -> Result<Variation> {
    let base = match &variation.base {
        Some(base) => base.clone(),
        None => return value_error("call generate before render."),
    };
    let paint = variation
        .paint
        .clone()
        .ok_or_else(|| MrlyError::Value("call generate before render.".into()))?;
    let cover = variation.is_cover();
    let mut files = std::mem::take(&mut variation.files);
    for file in files.iter_mut() {
        let mut canvas = base.clone().tile(file.width, file.height);
        if cover {
            let mut cell = canvas.cell.clone();
            engine::apply(&paint, &mut cell)?;
            canvas.cell = cell;
        }
        file.png = two::png(&canvas, scale)?;
    }
    variation.files = files;
    Ok(variation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::guard;
    fn config() -> Config {
        Config {
            tile: tile2d::Config {
                min_size: 3,
                max_size: 27,
                anti: Some(false),
                ..tile2d::Config::default()
            },
            paint: PaintConfig::default(),
            files: vec![(1, 1), (3, 3)],
        }
    }
    #[test]
    fn full_pipeline_produces_png_bytes() {
        let _g = guard();
        for _ in 0..20 {
            let v = create(&config()).unwrap();
            let v = generate(v, &config()).unwrap();
            let v = render(v, 4).unwrap();
            assert_eq!(v.files.len(), 2);
            for file in &v.files {
                assert!(
                    !file.png.is_empty(),
                    "empty png for {}x{}",
                    file.width,
                    file.height
                );
                assert_eq!(&file.png[1..4], b"PNG", "not a png header");
            }
        }
    }
    #[test]
    fn variation_is_seeded() {
        let _g = guard();
        seed(42);
        let a = create(&config()).unwrap();
        seed(42);
        let b = create(&config()).unwrap();
        assert_eq!(a.seed, b.seed);
        assert_eq!(a.key, b.key);
        assert_eq!(a.tile, b.tile);
        assert_eq!(a.edition, b.edition);
    }
    #[test]
    fn neighbors_edition_gets_a_mask() {
        let _g = guard();
        let config = Config {
            paint: PaintConfig {
                editions: Some(vec![Edition::Neighbors]),
                ..PaintConfig::default()
            },
            ..config()
        };
        let v = create(&config).unwrap();
        assert_eq!(v.edition, Edition::Neighbors);
        assert!(v.mask.is_some());
        let v = generate(v, &config).unwrap();
        let v = render(v, 4).unwrap();
        assert!(v.files.iter().all(|f| !f.png.is_empty()));
    }
    #[test]
    fn json_round_trips_tile() {
        let _g = guard();
        seed(5);
        let v = create(&config()).unwrap();
        let json = v.to_json();
        let tile_json = &json["tile"];
        let back = Tile::from_json(tile_json).unwrap();
        assert_eq!(back, v.tile);
    }
}
