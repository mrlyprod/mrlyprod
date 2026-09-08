/// The presets: the axes of seeds, tessellations, masks, rules and boundaries one census sweeps.
pub mod presets;
/// The readings of one frame: topology, symmetry, spectrum, entropy and the Kronecker block test.
pub mod readings;
/// The run record and the census loop with its dedupe of settled frames.
pub mod run;

pub use presets::{classes, maxi, medi, mini, Mask, Preset, Rule, Seed};
pub use readings::{
    block_split, canonical, cuts, factors, first_negative_lobe, pack, read, shifted_factors,
    translate_of, Reading, Symmetry,
};
pub use run::{census, run_one, Census, Run};
pub use run::{fate_table, mask_tensor};
