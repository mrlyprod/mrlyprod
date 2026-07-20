use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Identity;

impl Default for Identity {
    fn default() -> Identity {
        Identity::new()
    }
}

impl Identity {
    pub fn new() -> Identity {
        Identity
    }
}

impl App for Identity {
    fn route(&self) -> &str {
        "iden"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("iden")
            .emoji("👤")
            .title("identity")
            .category("system")
    }
    fn state(&self, iden: &Iden) -> Json {
        json!({
            "handle": iden.handle,
            "id": iden.id,
            "verified": iden.verified,
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        Vec::new()
    }
    fn act(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
        Outcome::fail("unknown verb")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_and_manifest_describe_the_app() {
        let app = Identity::new();
        assert_eq!(app.route(), "iden");
        let m = app.manifest();
        assert_eq!(m.route, "iden");
        assert_eq!(m.emoji, "👤");
        assert_eq!(m.title, "identity");
    }
    #[test]
    fn state_publishes_the_identity() {
        let app = Identity::new();
        let state = app.state(&Iden::new("guest"));
        assert_eq!(state["handle"], json!("@guest"));
        assert_eq!(state["id"], json!("guest"));
        assert_eq!(state["verified"], json!(false));
    }
    #[test]
    fn any_verb_fails() {
        let mut app = Identity::new();
        assert!(
            !app.act(&Iden::new("guest"), &Call::new("iden.edit", json!({})))
                .ok
        );
    }
}
