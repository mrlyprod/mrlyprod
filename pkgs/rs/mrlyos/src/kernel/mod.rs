/// The app contract: verbs, calls, effects and outcomes.
pub mod app;
/// The world frame: routes, notices, views and sync.
pub mod envelope;
/// The acting identity.
pub mod iden;
/// The app listing: route, emoji, title, category, flags and keys.
pub mod manifest;
/// The deterministic world of installed apps.
pub mod os;
/// The typed slot setters that echo what they set.
pub mod set;
/// The shape-guided pruning of state.
pub mod shape;

pub use app::{App, Call, Effect, Outcome, Verb};
pub use envelope::{Envelope, Notice, Route, Sync, View};
pub use iden::Iden;
pub use manifest::Manifest;
pub use os::Os;
pub use set::{drive, flag, int, pick};
pub use shape::prune;

/// The Mrly version baked in at build time.
pub const VERSION: &str = env!("MRLY_VERSION");

/// The stock identity and helpers tests drive apps with.
#[cfg(any(test, feature = "testkit"))]
pub mod testkit;
