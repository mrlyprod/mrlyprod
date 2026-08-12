#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The deterministic core: apps, calls, envelopes and the world that runs them.
pub mod kernel;

pub use kernel::{App, Call, Effect, Envelope, Iden, Os, Outcome, Route, Sync, Verb};
