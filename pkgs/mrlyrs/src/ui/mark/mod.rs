pub mod animation;
pub mod frames;
pub mod letters;
pub mod render;

pub const ROWS: usize = 7;
pub const COLS: usize = 49;
pub const FPS: usize = 25;
pub const HOLD: usize = 25;

pub use animation::animation;
pub use frames::{merging, writing};
pub use letters::SEQUENCE;
pub use render::{cell, grid, logo, wordmark};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board_is_seven_by_forty_nine() {
        let w = wordmark();
        assert_eq!((w.height(), w.width()), (ROWS, COLS));
    }
    #[test]
    fn writing_starts_empty_and_grows() {
        let frames = writing();
        assert!(frames[0].is_empty());
        for pair in frames.windows(2) {
            assert_eq!(pair[1].len(), pair[0].len() + 1);
        }
        assert_eq!(
            frames.last().unwrap().len(),
            crate::math::two::fills(&wordmark())
        );
    }
    #[test]
    fn frames_are_sorted_and_in_bounds() {
        for frame in animation() {
            assert!(frame.windows(2).all(|w| w[0] < w[1]));
            assert!(frame.iter().all(|&i| i < ROWS * COLS));
        }
    }
    #[test]
    fn animation_loops_through_both_halves() {
        let (w, m) = (writing().len(), merging().len());
        assert_eq!(animation().len(), 2 * w + 2 * m + 4 * HOLD);
    }
    #[test]
    fn cell_lights_exactly_the_frame() {
        let frame = wordmark();
        let rebuilt = cell(writing().last().unwrap());
        assert_eq!(rebuilt, frame);
    }
}
