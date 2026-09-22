use crate::core::error::{value_error, Error, Result};
use crate::core::paint::{self as engine, Config as PaintConfig, Edition, Ink, Paint};
use crate::core::rng::Rng;
use crate::gen::build::{build_2d, create_2d, Config2d};
use crate::gen::recipe::Tile;
use crate::math::two;
use serde::{Deserialize, Serialize};

/// One rendering of an artwork, sized in tile repetitions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    /// The count of tile repetitions across.
    pub width: usize,
    /// The count of tile repetitions down.
    pub height: usize,
    /// The encoded PNG bytes, empty until rendered and left out of the json.
    #[serde(skip)]
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
}

/// The settings an artwork is drawn under.
#[derive(Clone, Debug)]
pub struct Config {
    /// The constraints the tile is drawn under.
    pub tile: Config2d,
    /// The constraints the paint is drawn under.
    pub paint: PaintConfig,
    /// The width and height repetition pairs to render.
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

/// One seeded artwork, from tile recipe to rendered files.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// The built base cell, set by generate and left out of the json.
    #[serde(skip)]
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
}

fn hex_key(length: usize, rng: &mut Rng) -> String {
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

/// Draws a variation's seed from the stream, then the variation itself on that seed, with a mask when the edition is Neighbors.
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

/// Builds the variation's base cell and draws its paint from the stream, painting the base under a prime edition.
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

/// Renders every file of the variation to PNG at the given scale, scattering a Random edition from the stream, or an error before generate.
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
        file.png = two::png(&canvas, scale)?;
    }
    variation.files = files;
    Ok(variation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::json;
    use crate::gen::recipe::{Catalog, Parity};
    fn round_trip(variation: &Variation) -> Variation {
        serde_json::from_value(serde_json::to_value(variation).unwrap()).unwrap()
    }
    fn config() -> Config {
        Config {
            tile: Config2d {
                min_size: 3,
                max_size: 27,
                anti: Some(false),
                ..Config2d::default()
            },
            paint: PaintConfig::default(),
            files: vec![(1, 1), (3, 3)],
        }
    }
    fn edition_config(edition: Edition) -> Config {
        Config {
            paint: PaintConfig {
                editions: Some(vec![edition]),
                ..PaintConfig::default()
            },
            ..config()
        }
    }
    fn png_size(png: &[u8]) -> (usize, usize) {
        let w = u32::from_be_bytes(png[16..20].try_into().unwrap()) as usize;
        let h = u32::from_be_bytes(png[20..24].try_into().unwrap()) as usize;
        (w, h)
    }
    fn bare_png(variation: &Variation, file: &File, scale: usize) -> Vec<u8> {
        let base = variation.base.as_ref().unwrap();
        let bare = two::Cell2d::new(base.types().clone())
            .unwrap()
            .tile(file.width, file.height)
            .unwrap();
        two::png(&bare, scale).unwrap()
    }
    fn run(config: &Config, seed: u64, scale: usize) -> Variation {
        let mut rng = Rng::new(seed);
        let v = create(config, &mut rng).unwrap();
        let v = generate(v, config, &mut rng).unwrap();
        render(v, scale, &mut rng).unwrap()
    }
    #[test]
    fn full_pipeline_produces_png_bytes() {
        for s in 0..20 {
            let v = run(&config(), s, 4);
            assert_eq!(v.files.len(), 2);
            for file in &v.files {
                assert!(
                    !file.png.is_empty(),
                    "empty png for {}x{}",
                    file.width,
                    file.height
                );
                assert_eq!(&file.png[1..4], b"PNG", "not a png header");
                let expected = (
                    v.tile.width * file.width * 4,
                    v.tile.height * file.height * 4,
                );
                assert_eq!(png_size(&file.png), expected, "png size seed {s}");
            }
        }
    }
    #[test]
    fn variation_replays_its_seed() {
        let a = create(&config(), &mut Rng::new(42)).unwrap();
        let b = create(&config(), &mut Rng::new(42)).unwrap();
        assert_eq!(a.seed, b.seed);
        assert_eq!(a.key, b.key);
        assert_eq!(a.tile, b.tile);
        assert_eq!(a.edition, b.edition);
        assert_ne!(a.seed, create(&config(), &mut Rng::new(43)).unwrap().seed);
    }
    #[test]
    fn editions_keep_their_palette() {
        for (i, edition) in Edition::all().into_iter().enumerate() {
            let config = edition_config(edition);
            let mut differed = false;
            for s in 0..8 {
                let v = run(&config, 1000 * (i as u64 + 1) + s, 2);
                if v.files.iter().all(|f| f.png != bare_png(&v, f, 2)) {
                    differed = true;
                    break;
                }
            }
            assert!(differed, "edition {edition:?} never rendered its palette");
        }
    }
    #[test]
    fn layers_edition_keeps_its_palette() {
        let config = edition_config(Edition::Layers);
        let mut rng = Rng::new(7);
        let v = create(&config, &mut rng).unwrap();
        assert_eq!(v.edition, Edition::Layers);
        let v = generate(v, &config, &mut rng).unwrap();
        assert!(v.base.as_ref().unwrap().cell.colors.is_some());
        let v = render(v, 2, &mut rng).unwrap();
        for file in &v.files {
            assert_ne!(file.png, bare_png(&v, file, 2), "default mapping leaked");
        }
    }
    #[test]
    fn neighbors_edition_gets_a_mask() {
        let config = edition_config(Edition::Neighbors);
        let mut rng = Rng::new(3);
        let v = create(&config, &mut rng).unwrap();
        assert_eq!(v.edition, Edition::Neighbors);
        assert!(v.mask.is_some());
        let v = generate(v, &config, &mut rng).unwrap();
        assert!(v.base.as_ref().unwrap().cell.colors.is_some());
        let v = render(v, 2, &mut rng).unwrap();
        for file in &v.files {
            assert_ne!(file.png, bare_png(&v, file, 2), "default mapping leaked");
        }
    }
    #[test]
    fn neighbors_mask_builds_under_evens_parity() {
        let config = Config {
            tile: Config2d {
                min_size: 4,
                max_size: 16,
                parity: Parity::Evens,
                anti: Some(false),
                ..Config2d::default()
            },
            paint: PaintConfig {
                editions: Some(vec![Edition::Neighbors]),
                ..PaintConfig::default()
            },
            files: vec![(1, 1)],
        };
        for s in 0..10 {
            let mut rng = Rng::new(s);
            let v = create(&config, &mut rng).unwrap();
            let mask = v.mask.as_ref().unwrap();
            assert_eq!((mask.width, mask.height), (3, 3));
            let v = generate(v, &config, &mut rng).unwrap();
            let v = render(v, 2, &mut rng).unwrap();
            assert!(!v.files[0].png.is_empty());
        }
    }
    #[test]
    fn json_round_trips_the_record() {
        for (i, edition) in Edition::all().into_iter().enumerate() {
            let config = edition_config(edition);
            let a = create(&config, &mut Rng::new(500 + i as u64)).unwrap();
            let b = round_trip(&a);
            assert_eq!(b.key, a.key);
            assert_eq!(b.seed, a.seed);
            assert_eq!(b.edition, a.edition);
            assert_eq!(b.primaries, a.primaries);
            assert_eq!(b.tile, a.tile);
            assert_eq!(b.mask, a.mask);
            assert_eq!(b.paint, a.paint);
            let mut ra = Rng::new(a.seed);
            let a = generate(a, &config, &mut ra).unwrap();
            let a = render(a, 2, &mut ra).unwrap();
            let mut rb = Rng::new(b.seed);
            let b = generate(b, &config, &mut rb).unwrap();
            let b = render(b, 2, &mut rb).unwrap();
            assert_eq!(a.paint, b.paint);
            for (fa, fb) in a.files.iter().zip(&b.files) {
                assert_eq!((fa.width, fa.height), (fb.width, fb.height));
                assert!(!fa.png.is_empty());
                assert_eq!(fa.png, fb.png, "edition {edition:?}");
            }
            let c = round_trip(&a);
            assert_eq!(c.paint, a.paint);
            assert!(c.base.is_none());
            assert!(c.files.iter().all(|f| f.png.is_empty()));
        }
    }
    #[test]
    fn refuses_a_config_or_a_record_it_cannot_draw() {
        let mut rng = Rng::new(5);
        let narrow = Config {
            tile: Config2d {
                min_size: 9,
                max_size: 3,
                ..Config2d::default()
            },
            ..config()
        };
        assert!(create(&narrow, &mut rng).is_err());
        let sourceless = Config {
            tile: Config2d {
                catalog: Catalog::Codes(Vec::new()),
                ..Config2d::default()
            },
            ..config()
        };
        assert!(create(&sourceless, &mut rng).is_err());
        let drawn = create(&config(), &mut rng).unwrap();
        assert!(render(drawn, 1, &mut rng).is_err());
        assert!(serde_json::from_value::<Variation>(json!({})).is_err());
        assert!(serde_json::from_value::<File>(json!({ "width": 2 })).is_err());
    }
    #[test]
    fn json_round_trips_tile() {
        let v = create(&config(), &mut Rng::new(5)).unwrap();
        let json = serde_json::to_value(&v).unwrap();
        assert!(json.get("base").is_none());
        let back: Tile = serde_json::from_value(json["tile"].clone()).unwrap();
        assert_eq!(back, v.tile);
    }
}
