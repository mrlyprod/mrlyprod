pub mod build;
pub mod draw;
pub mod name;
pub mod paint;
pub mod recipe;
pub mod variation;

pub use build::tree_mask;
pub use draw::{random_design, random_rotation};
pub use name::{classic_code, classic_code_nd};
pub use recipe::{Group, Parity, Tile};
pub use variation::hex_key;

use mrly_error::{Result, value_error};
use mrly_image::Image;
use mrly_rand::Rng;

pub fn background(seed: u64, width: usize, height: usize) -> Result<Image> {
    if width == 0 || height == 0 {
        return value_error("a background wants a width and a height above zero.");
    }
    let mut rng = Rng::new(seed);
    let config = variation::Config {
        files: vec![(width, height)],
        ..variation::Config::default()
    };
    let drawn = variation::create(&config, &mut rng)?;
    let built = variation::generate(drawn, &config, &mut rng)?;
    let rendered = variation::render(built, 1, &mut rng)?;
    match rendered
        .files
        .into_iter()
        .next()
        .and_then(|file| file.image)
    {
        Some(image) => Ok(image),
        None => value_error("the background rendered no file."),
    }
}
