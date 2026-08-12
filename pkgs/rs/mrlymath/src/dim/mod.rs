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
/// The random tile drawing the dimensions share.
pub mod tile;
pub use designs::grow;
pub use painter::paint;
pub use renderer::push_glyph;
