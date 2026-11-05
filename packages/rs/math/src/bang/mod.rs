pub mod baseq;
pub mod catalog;
pub mod code;
pub mod factory;
pub mod universe;
pub mod word;

pub use catalog::{sources, universe_codes};
pub use code::Code;
pub use factory::{
    MagicLayer, code_to_corners, corners_to_code, level_cap, levels_code, magic, magic_named,
    random_code,
};
pub use universe::{
    Design, Universe, bang, corners, symmetries, total_exposure, touches_every_corner,
};
