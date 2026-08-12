use super::iden::Iden;
use super::manifest::Manifest;
use mrlycore::{json, Json};

/// An action an app offers, named and ready with its arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct Verb {
    /// The verb name.
    pub name: String,
    /// The arguments the verb carries.
    pub args: Json,
}

impl Verb {
    /// Builds a verb from a name and its arguments.
    pub fn new(name: &str, args: Json) -> Verb {
        Verb {
            name: name.to_string(),
            args,
        }
    }
    /// Returns the verb as plain JSON.
    pub fn to_json(&self) -> Json {
        json!({ "verb": &self.name, "args": &self.args })
    }
}

/// One invocation of a verb, optionally stamped with a moment.
#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    /// The verb being called.
    pub verb: String,
    /// The arguments of the call.
    pub args: Json,
    /// The moment stamped on the call, or None to inherit the kernel's.
    pub now: Option<i64>,
}

impl Call {
    /// Builds an unstamped call from a verb name and arguments.
    pub fn new(verb: &str, args: Json) -> Call {
        Call {
            verb: verb.to_string(),
            args,
            now: None,
        }
    }
    /// Stamps the call with a moment and returns it.
    ///
    /// ```
    /// use mrlyos::Call;
    /// use mrlycore::json;
    /// let stamped = Call::new("clock.tick", json!({})).at(5000);
    /// assert_eq!(stamped.to_json(), json!({ "verb": "clock.tick", "args": {}, "now": 5000 }));
    /// ```
    pub fn at(mut self, now: i64) -> Call {
        self.now = Some(now);
        self
    }
    /// Returns the named argument.
    pub fn arg(&self, key: &str) -> &Json {
        &self.args[key]
    }
    /// Returns the call as plain JSON, carrying the stamp only when present.
    pub fn to_json(&self) -> Json {
        let mut out = json!({ "verb": &self.verb, "args": &self.args });
        if let Some(now) = self.now {
            out["now"] = json!(now);
        }
        out
    }
}

/// A side effect an outcome hands to the outside, with an optional return call.
#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    /// The kind of effect.
    pub kind: String,
    /// The payload of the effect.
    pub data: Json,
    /// The call to make once the effect lands, if any.
    pub call: Option<Call>,
}

impl Effect {
    /// Builds an effect from a kind and its payload.
    pub fn new(kind: &str, data: Json) -> Effect {
        Effect {
            kind: kind.to_string(),
            data,
            call: None,
        }
    }
    /// Attaches the call to make once the effect lands and returns the effect.
    pub fn then(mut self, call: Call) -> Effect {
        self.call = Some(call);
        self
    }
    /// Returns the effect as plain JSON, carrying the call only when present.
    pub fn to_json(&self) -> Json {
        let mut out = json!({ "kind": &self.kind, "data": &self.data });
        if let Some(call) = &self.call {
            out["call"] = call.to_json();
        }
        out
    }
}

/// What a call came to: success or failure, data, and any emitted effects.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    /// Whether the call succeeded.
    pub ok: bool,
    /// The data the call returned.
    pub data: Json,
    /// The note explaining a failure, if any.
    pub note: Option<String>,
    /// The effects the call emitted.
    pub effects: Vec<Effect>,
}

impl Outcome {
    /// Builds a successful outcome carrying data.
    pub fn ok(data: Json) -> Outcome {
        Outcome {
            ok: true,
            data,
            note: None,
            effects: Vec::new(),
        }
    }
    /// Builds a failed outcome carrying a note.
    pub fn fail(note: &str) -> Outcome {
        Outcome {
            ok: false,
            data: Json::Null,
            note: Some(note.to_string()),
            effects: Vec::new(),
        }
    }
    /// Appends an effect and returns the outcome.
    pub fn emit(mut self, effect: Effect) -> Outcome {
        self.effects.push(effect);
        self
    }
    /// Returns the outcome as plain JSON, effects left out.
    pub fn to_json(&self) -> Json {
        json!({ "ok": self.ok, "data": &self.data, "note": self.note.clone() })
    }
}

/// The contract an app signs to live in the kernel.
pub trait App {
    /// Returns the app's route name.
    fn route(&self) -> &str;
    /// Returns the app's manifest, a bare one by default.
    fn manifest(&self) -> Manifest {
        Manifest::new(self.route())
    }
    /// Returns the state to show an identity under a shape, the saved state by default.
    fn state(&self, _iden: &Iden, _shape: Option<&Json>) -> Json {
        self.save()
    }
    /// Returns the app's geometry floats, or None when it draws no scene.
    fn geometry(&self) -> Option<Vec<f32>> {
        None
    }
    /// Returns the app's uniform floats, or None when it has none.
    fn uniforms(&self) -> Option<Vec<f32>> {
        None
    }
    /// Returns the app's triangle floats, or None when it has none.
    fn tris(&self) -> Option<Vec<f32>> {
        None
    }
    /// Returns the verbs an identity may take right now.
    fn actions(&self, iden: &Iden) -> Vec<Verb>;
    /// Handles one call for an identity and returns its outcome.
    fn call(&mut self, iden: &Iden, call: &Call) -> Outcome;
    /// Returns the call to repeat every beat, or None to sit still.
    fn beat(&self) -> Option<Call> {
        None
    }
    /// Absorbs the world other apps share, a no-op by default.
    fn wear(&mut self, _world: &Json) {}
    /// Returns the state this app shares with the world, or None to stay private.
    fn share(&self) -> Option<Json> {
        None
    }
    /// Returns the state to persist, Null by default.
    fn save(&self) -> Json {
        Json::Null
    }
    /// Restores the app from persisted state, a no-op by default.
    fn load(&mut self, _state: &Json) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unstamped_call_omits_now() {
        assert_eq!(
            Call::new("life.step", json!({})).to_json(),
            json!({ "verb": "life.step", "args": {} })
        );
    }
    #[test]
    fn stamped_call_carries_now() {
        assert_eq!(
            Call::new("clock.tick", json!({})).at(5000).to_json(),
            json!({ "verb": "clock.tick", "args": {}, "now": 5000 })
        );
    }
    #[test]
    fn effect_omits_absent_call() {
        let plain = Effect::new("notify", json!({ "title": "hi" }));
        assert_eq!(
            plain.to_json(),
            json!({ "kind": "notify", "data": { "title": "hi" } })
        );
        let returning = plain.then(Call::new("timer.check", json!({})));
        assert_eq!(
            returning.to_json()["call"],
            json!({ "verb": "timer.check", "args": {} })
        );
    }
    #[test]
    fn save_defaults_to_null() {
        struct Bare;
        impl App for Bare {
            fn route(&self) -> &str {
                "bare"
            }
            fn actions(&self, _iden: &Iden) -> Vec<Verb> {
                Vec::new()
            }
            fn call(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
                Outcome::ok(json!({}))
            }
        }
        let mut bare = Bare;
        assert_eq!(bare.save(), Json::Null);
        bare.load(&json!({ "ghost": true }));
        assert_eq!(bare.save(), Json::Null);
        assert_eq!(bare.state(&Iden::new("aria"), None), Json::Null);
    }
    #[test]
    fn state_defaults_to_save() {
        struct Counter;
        impl App for Counter {
            fn route(&self) -> &str {
                "counter"
            }
            fn actions(&self, _iden: &Iden) -> Vec<Verb> {
                Vec::new()
            }
            fn call(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
                Outcome::ok(json!({}))
            }
            fn save(&self) -> Json {
                json!({ "count": 3 })
            }
        }
        assert_eq!(
            Counter.state(&Iden::new("aria"), None),
            json!({ "count": 3 })
        );
    }
    #[test]
    fn outcome_json_stays_effect_free() {
        let out = Outcome::ok(json!({})).emit(Effect::new("notify", json!({})));
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.to_json()["effects"], Json::Null);
    }
}
