//! The universe of design codes.
//!
//! A code is a bitmask over the residue corners of a hypercube; this folder packs corners into
//! codes, folds codes into symmetry classes, counts the classes and spells the magic words.

/// The base-q symmetry maps, the design counts raw and distinct, and the fill classes.
pub mod baseq;
/// The cached canonical codes and tile sources of a dimension.
pub mod catalog;
/// The design code: the bitmask of filled corners, printed and parsed as one number.
pub mod code;
/// The packing of residue corners into codes and back.
pub mod factory;
/// The corners, codes and symmetries that name designs.
pub mod universe;
/// The magic words: their products, their component counts and the schedules that spell them.
pub mod word;

pub use catalog::{sources, universe_codes};
pub use code::Code;
pub use factory::{code_to_corners, corners_to_code, levels_code, magic, magic_named, MagicLayer};
pub use universe::{
    bang, corners, symmetries, total_exposure, touches_every_corner, Design, Universe,
};

#[cfg(test)]
mod tests {
    use super::{Code, MagicLayer};
    use crate::math::bang::catalog::Catalog;
    use crate::math::bang::word::Schedule;
    use crate::math::name::Bang;
    use crate::math::round_trip;

    #[test]
    fn serde_round_trips() {
        round_trip(Code::from(402u64));
        round_trip(Catalog::Codes(vec![7, 14]));
        round_trip(Schedule::Periodic);
        round_trip(MagicLayer::new(Bang::new(7, 2, 2), 3));
    }
}
