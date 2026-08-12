use super::app::{App, Call, Outcome};
use super::iden::Iden;
use mrlycore::{json, Json};

/// Returns the stock test identity.
pub fn iden() -> Iden {
    Iden::new("aria")
}

/// Calls a verb on the app under the stock identity and returns the outcome.
pub fn send<A: App>(app: &mut A, verb: &str, args: Json) -> Outcome {
    app.call(&iden(), &Call::new(verb, args))
}

/// Seeds the app through one call of the verb and returns it.
pub fn seeded<A: App>(mut app: A, verb: &str, seed: u64) -> A {
    let _ = app.call(&iden(), &Call::new(verb, json!({ "seed": seed })));
    app
}
