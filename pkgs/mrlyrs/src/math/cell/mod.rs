//! The N-dimensional cell and the pipeline the fixed dimensions share.
//!
//! A seed pattern grows into a fractal cell here, and `two`, `three` and `six` are that one
//! pipeline pinned to a dimension: census, geometry, graphs, painting, text and JSON.

/// The fill, void and exposure counts of an N-dimensional cell.
pub mod census;
/// The growth of a seed pattern into a fractal cell.
pub mod designs;
/// The merges, magic folds, mosaics and perforations of N-dimensional cells.
pub mod geometry;
/// The core, edge and tunnel networks of an N-dimensional cell.
pub mod graph;
/// The N-dimensional cell and its fixed-dimension aliases.
pub mod models;
/// The coloring of an N-dimensional cell by mapping and mode.
pub mod painter;
/// The glyph pushing the text renderers share.
pub mod renderer;
/// The JSON reading the cell serializers share.
pub mod serializer;
pub use designs::grow;
pub use painter::paint;
pub use renderer::push_glyph;

#[cfg(test)]
mod tests {
    use crate::math::round_trip;
    use crate::math::{three, two};

    #[test]
    fn serde_round_trips() {
        round_trip(two::designs::carpet(3, 1).unwrap());
        round_trip(three::designs::carpet(3, 1).unwrap());
        round_trip(two::paint(two::designs::carpet(3, 1).unwrap(), None, None, None).unwrap());
    }
}
