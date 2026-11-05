use crate::build::{Config2d, build_2d, create_2d};
use crate::recipe::Tile;
use mrly_core::error::{Error, Result, value_error};
use mrly_core::image::Image;
use mrly_core::paint::{self as engine, Config as PaintConfig, Edition, Ink, Paint};
use mrly_core::rng::Rng;
use mrly_math::two;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    pub width: usize,
    pub height: usize,
    #[serde(skip)]
    pub image: Option<Image>,
}

impl File {
    pub fn new(width: usize, height: usize) -> File {
        File {
            width,
            height,
            image: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub tile: Config2d,
    pub paint: PaintConfig,
    pub files: Vec<(usize, usize)>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tile: Config2d::default(),
            paint: PaintConfig::default(),
            files: vec![(1, 1), (3, 3), (5, 5)],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variation {
    pub key: String,
    pub seed: u64,
    pub edition: Edition,
    pub primaries: Option<Vec<Ink>>,
    pub tile: Tile,
    pub mask: Option<Tile>,
    pub paint: Option<Paint>,
    #[serde(skip)]
    pub base: Option<two::Cell2d>,
    pub files: Vec<File>,
}

impl Variation {
    pub fn is_cover(&self) -> bool {
        matches!(
            self.edition,
            Edition::Rows | Edition::Columns | Edition::Random
        )
    }
    pub fn is_prime(&self) -> bool {
        matches!(self.edition, Edition::Layers | Edition::Neighbors)
    }
}

pub fn hex_key(length: usize, rng: &mut Rng) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    (0..length).map(|_| DIGITS[rng.below(16)] as char).collect()
}

fn pop_center(tile: &Tile, cell: &mut two::Cell2d) {
    let _ = tile;
    let center = cell.width() / 2;
    let mut types = cell.cell.types.clone();
    types.put(types.index(&[center, center]), 0);
    cell.cell.types = types;
}

pub fn create(config: &Config, rng: &mut Rng) -> Result<Variation> {
    let s = rng.range(0, i64::MAX) as u64;
    let mut rng = Rng::new(s);
    let edition = engine::random_edition(config.paint.editions.as_deref(), &mut rng);
    let tile = create_2d(&config.tile, &mut rng)?;
    let mask = if edition == Edition::Neighbors {
        let mask_config = Config2d {
            min_size: 3,
            max_size: 3,
            ..Config2d::default()
        };
        Some(create_2d(&mask_config, &mut rng)?)
    } else {
        None
    };
    Ok(Variation {
        key: hex_key(8, &mut rng),
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

pub fn generate(mut variation: Variation, config: &Config, rng: &mut Rng) -> Result<Variation> {
    let _ = config;
    let mut base = build_2d(&variation.tile)?;
    let paint_config = PaintConfig {
        editions: Some(vec![variation.edition]),
        primaries: variation.primaries.clone(),
        target: None,
    };
    if variation.is_cover() {
        let p = engine::setup(Paint::new(variation.edition), &paint_config, rng);
        variation.paint = Some(p);
    } else {
        let mask_tensor = match &variation.mask {
            Some(mask_tile) => {
                let mut mask_cell = build_2d(mask_tile)?;
                pop_center(mask_tile, &mut mask_cell);
                Some(mask_cell.cell.types.clone())
            }
            None => None,
        };
        let mut cell = base.cell.clone();
        let p = engine::paint(&mut cell, &paint_config, mask_tensor.as_ref(), rng)?;
        base.cell = cell;
        variation.paint = Some(p);
    }
    variation.base = Some(base);
    Ok(variation)
}

pub fn render(mut variation: Variation, scale: usize, rng: &mut Rng) -> Result<Variation> {
    let base = match &variation.base {
        Some(base) => base.clone(),
        None => return value_error("call generate before render."),
    };
    let paint = variation
        .paint
        .clone()
        .ok_or_else(|| Error::Value("call generate before render.".into()))?;
    let cover = variation.is_cover();
    let mut files = std::mem::take(&mut variation.files);
    for file in files.iter_mut() {
        let mut canvas = base.clone().tile(file.width, file.height)?;
        if cover {
            let mut cell = canvas.cell.clone();
            engine::apply(&paint, &mut cell, rng)?;
            canvas.cell = cell;
        }
        file.image = Some(two::image(&canvas, scale, None, 1, two::Shape::Square)?);
    }
    variation.files = files;
    Ok(variation)
}
