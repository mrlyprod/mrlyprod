/// The band gaps a transmission curve carries.
pub mod gaps;
/// The two-speed corridor a design mask stamps.
pub mod medium;
/// The transmission spectrum against a free-space reference.
pub mod spectrum;
/// The leapfrog pressure stepper and its fixed source.
pub mod stepper;

/// The wave speed of empty sites.
pub const C_FAST: f64 = 1.0;
/// The wave speed of filled sites.
pub const C_SLOW: f64 = 0.5;
/// The Courant factor keeping the stepper stable.
pub const COURANT: f64 = 0.35;
/// The source frequency in cycles per step.
pub const F_SRC: f64 = 0.06;

pub use gaps::{gaps, Gap, GAP_THRESH};
pub use medium::{medium, Medium};
pub use spectrum::{transmission, Spectrum, BAND_HI, BAND_LO};
pub use stepper::run;
