/// The odd multiplier ladder grids and canvases climb.
pub const LADDER: [usize; 6] = [1, 3, 5, 7, 9, 11];

/// The frames tempo in frames per second.
pub const FPS: usize = 8;

/// The heatmap tempo in frames per second.
pub const HEATMAP_FPS: usize = 32;

/// The freeze at the start and end of the timeline, in microseconds.
pub const FREEZE_US: u64 = 500_000;

/// The freeze on the held last frame between segments, in microseconds.
pub const INTER_FREEZE_US: u64 = 500_000;

/// The square side an assembler scales frames to, in pixels.
pub const SIZE: usize = 1080;

/// The editorial caps one quest is generated under.
#[derive(Clone, Debug)]
pub struct Config {
    /// The generation cap of one chapter.
    pub max_generations: usize,
    /// The chapter cap of one attempt.
    pub max_segments: usize,
    /// The canvas side cap in cells.
    pub max_canvas: usize,
    /// The smallest tile side drawn.
    pub min_tile: usize,
    /// The largest tile side drawn.
    pub max_tile: usize,
    /// The smallest mask side drawn.
    pub min_mask: usize,
    /// The largest mask side drawn.
    pub max_mask: usize,
    /// The attempt cap before the quest gives up.
    pub attempts: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            max_generations: 64,
            max_segments: 8,
            max_canvas: 256,
            min_tile: 3,
            max_tile: 128,
            min_mask: 3,
            max_mask: 64,
            attempts: 64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_ladder_is_odd_and_ascending() {
        assert!(LADDER.iter().all(|i| i % 2 == 1));
        assert!(LADDER.windows(2).all(|w| w[0] < w[1]));
    }
    #[test]
    fn the_defaults_carry_the_product_caps() {
        let config = Config::default();
        assert_eq!(config.max_generations, 64);
        assert_eq!(config.max_segments, 8);
        assert_eq!(config.max_canvas, 256);
        assert_eq!(config.max_tile, 128);
        assert_eq!(config.max_mask, 64);
    }
}
