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
/// The magic words: their products, their component counts and the schedules that spell them.
pub mod word;

pub use catalog::{sources, universe_codes};
pub use factory::{code_to_corners, corners_to_code, levels_code, magic, magic_named, MagicLayer};
pub use universe::{
    bang, corners, symmetries, total_exposure, touches_every_corner, Code, Design, Universe,
};
