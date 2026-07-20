pub mod billiards;
pub mod field;
pub mod lasers;
pub mod mask;
pub mod rng;
pub mod waves;
mod waves_luts;

pub use crate::core::trig;
pub use billiards::{Billiards, BilliardsConfig, Particle};
pub use field::Field;
pub use lasers::{Emitter, Lasers, LasersConfig};
pub use mask::Mask;
pub use rng::Rng;
pub use waves::{Source, Waves, WavesConfig};
