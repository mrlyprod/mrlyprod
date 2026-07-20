use super::iden::Iden;
use super::manifest::Manifest;
use serde_json::{json, Value as Json};

#[derive(Clone, Debug, PartialEq)]
pub struct Verb {
    pub name: String,
    pub args: Json,
}

impl Verb {
    pub fn new(name: &str, args: Json) -> Verb {
        Verb {
            name: name.to_string(),
            args,
        }
    }
    pub fn to_json(&self) -> Json {
        json!({ "verb": self.name, "args": self.args })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub verb: String,
    pub args: Json,
    pub now: Option<i64>,
}

impl Call {
    pub fn new(verb: &str, args: Json) -> Call {
        Call {
            verb: verb.to_string(),
            args,
            now: None,
        }
    }
    pub fn at(mut self, now: i64) -> Call {
        self.now = Some(now);
        self
    }
    pub fn arg(&self, key: &str) -> &Json {
        &self.args[key]
    }
    pub fn to_json(&self) -> Json {
        let mut out = json!({ "verb": self.verb, "args": self.args });
        if let Some(now) = self.now {
            out["now"] = json!(now);
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    pub kind: String,
    pub data: Json,
    pub call: Option<Call>,
}

impl Effect {
    pub fn new(kind: &str, data: Json) -> Effect {
        Effect {
            kind: kind.to_string(),
            data,
            call: None,
        }
    }
    pub fn then(mut self, call: Call) -> Effect {
        self.call = Some(call);
        self
    }
    pub fn to_json(&self) -> Json {
        let mut out = json!({ "kind": self.kind, "data": self.data });
        if let Some(call) = &self.call {
            out["call"] = call.to_json();
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub ok: bool,
    pub data: Json,
    pub note: Option<String>,
    pub effects: Vec<Effect>,
}

impl Outcome {
    pub fn ok(data: Json) -> Outcome {
        Outcome {
            ok: true,
            data,
            note: None,
            effects: Vec::new(),
        }
    }
    pub fn fail(note: &str) -> Outcome {
        Outcome {
            ok: false,
            data: Json::Null,
            note: Some(note.to_string()),
            effects: Vec::new(),
        }
    }
    pub fn emit(mut self, effect: Effect) -> Outcome {
        self.effects.push(effect);
        self
    }
    pub fn to_json(&self) -> Json {
        json!({ "ok": self.ok, "data": self.data, "note": self.note })
    }
}

pub trait App {
    fn route(&self) -> &str;
    fn manifest(&self) -> Manifest {
        Manifest::new(self.route())
    }
    fn state(&self, _iden: &Iden) -> Json {
        self.save()
    }
    fn capture(&self, iden: &Iden) -> Json {
        self.state(iden)["frame"].clone()
    }
    fn geometry(&self) -> Option<Vec<f32>> {
        None
    }
    fn actions(&self, iden: &Iden) -> Vec<Verb>;
    fn act(&mut self, iden: &Iden, call: &Call) -> Outcome;
    fn beat(&self) -> Option<Call> {
        None
    }
    fn wear(&mut self, _world: &Json) {}
    fn share(&self) -> Option<Json> {
        None
    }
    fn save(&self) -> Json {
        Json::Null
    }
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
            fn act(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
                Outcome::ok(json!({}))
            }
        }
        let mut bare = Bare;
        assert_eq!(bare.save(), Json::Null);
        bare.load(&json!({ "ghost": true }));
        assert_eq!(bare.save(), Json::Null);
        assert_eq!(bare.state(&Iden::new("aria")), Json::Null);
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
            fn act(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
                Outcome::ok(json!({}))
            }
            fn save(&self) -> Json {
                json!({ "count": 3 })
            }
        }
        assert_eq!(Counter.state(&Iden::new("aria")), json!({ "count": 3 }));
    }
    #[test]
    fn outcome_json_stays_effect_free() {
        let out = Outcome::ok(json!({})).emit(Effect::new("notify", json!({})));
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.to_json()["effects"], Json::Null);
    }
}
