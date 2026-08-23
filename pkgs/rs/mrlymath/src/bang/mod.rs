/// The base-q symmetry maps and distinct-design counts.
pub mod baseq;
/// The cached canonical codes and tile sources of a dimension.
pub mod catalog;
/// The design counts, raw and distinct under symmetry.
pub mod counting;
/// The packing of residue corners into codes and back.
pub mod factory;
/// The corners, codes and symmetries that name designs.
pub mod universe;

pub use catalog::{sources, universe_codes};
pub use factory::{code_to_corners, corners_to_code, levels_code, magic, magic_named, MagicLayer};
pub use universe::{bang, corners, symmetries, Code, Design, Universe};
