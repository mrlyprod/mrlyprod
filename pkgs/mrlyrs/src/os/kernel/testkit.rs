use super::app::{App, Call, Outcome};
use super::iden::Iden;
use serde_json::{json, Value as Json};

pub fn iden() -> Iden {
    Iden::new("aria")
}

pub fn send<A: App>(app: &mut A, verb: &str, args: Json) -> Outcome {
    app.act(&iden(), &Call::new(verb, args))
}

pub fn seeded<A: App>(mut app: A, verb: &str, seed: u64) -> A {
    let _ = app.act(&iden(), &Call::new(verb, json!({ "seed": seed })));
    app
}
