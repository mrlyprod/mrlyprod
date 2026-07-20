pub use crate::math::dim::paint;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::colors::{BLACK, WHITE};
    use crate::math::two::designs;
    #[test]
    fn default_paint_is_black_on_white() {
        let c = paint(designs::carpet(3, 1).unwrap(), None, None);
        let colors = c.cell.colors.as_ref().unwrap();
        assert_eq!(colors[0], [BLACK.r, BLACK.g, BLACK.b, 255]);
        assert_eq!(colors[4], [WHITE.r, WHITE.g, WHITE.b, 255]);
        let blacks = colors
            .iter()
            .filter(|c| **c == [BLACK.r, BLACK.g, BLACK.b, 255])
            .count();
        assert_eq!(blacks as u64, designs::carpet(3, 1).unwrap().types().sum());
    }
}
