pub mod app;
pub mod envelope;
pub mod iden;
pub mod manifest;
pub mod os;
pub mod set;

pub use app::{App, Call, Effect, Outcome, Verb};
pub use envelope::{Envelope, Notice, Route, Sync, View};
pub use iden::Iden;
pub use manifest::Manifest;
pub use os::Os;
pub use set::{drive, flag, int, pick, real};

pub const VERSION: &str = env!("MRLY_VERSION");

#[cfg(any(test, feature = "testkit"))]
pub mod testkit;
