use super::app::{Call, Effect, Outcome, Verb};
use serde_json::{json, Value as Json};

#[derive(Clone, Debug, PartialEq)]
pub struct Route {
    pub app: String,
    pub view: String,
    pub params: Json,
}

impl Route {
    pub fn new(app: &str) -> Route {
        Route {
            app: app.to_string(),
            view: "main".to_string(),
            params: json!({}),
        }
    }
    pub fn to_json(&self) -> Json {
        json!({ "app": self.app, "view": self.view, "params": self.params })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Notice {
    pub title: String,
    pub body: String,
    pub at: i64,
}

impl Notice {
    pub fn new(title: &str, body: &str, at: i64) -> Notice {
        Notice {
            title: title.to_string(),
            body: body.to_string(),
            at,
        }
    }
    pub fn to_json(&self) -> Json {
        json!({ "title": self.title, "body": self.body, "at": self.at })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sync {
    Pending,
    Synced,
    Failed,
}

impl Sync {
    pub fn name(&self) -> &'static str {
        match self {
            Sync::Pending => "pending",
            Sync::Synced => "synced",
            Sync::Failed => "failed",
        }
    }
}

pub struct View {
    pub app: String,
    pub params: Json,
    pub state: Json,
    pub actions: Vec<Verb>,
    pub beat: Option<Call>,
}

impl View {
    pub fn to_json(&self) -> Json {
        let mut out = json!({
            "app": self.app,
            "params": self.params,
            "state": self.state,
            "actions": self.actions.iter().map(Verb::to_json).collect::<Vec<_>>(),
        });
        if let Some(beat) = &self.beat {
            out["beat"] = beat.to_json();
        }
        out
    }
}

pub struct Envelope {
    pub tick: u64,
    pub route: Option<Route>,
    pub view: Option<View>,
    pub last: Option<Outcome>,
    pub sync: Sync,
    pub effects: Vec<Effect>,
    pub notices: Vec<Notice>,
}

impl Envelope {
    pub fn to_json(&self) -> Json {
        let mut out = json!({
            "tick": self.tick,
            "route": self.route.as_ref().map(Route::to_json),
            "view": self.view.as_ref().map(View::to_json),
            "last": self.last.as_ref().map(Outcome::to_json),
            "sync": self.sync.name(),
        });
        if !self.effects.is_empty() {
            out["effects"] = self.effects.iter().map(Effect::to_json).collect();
        }
        if !self.notices.is_empty() {
            out["notices"] = self.notices.iter().map(Notice::to_json).collect();
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn notice_serializes() {
        assert_eq!(
            Notice::new("timer", "time is up", 5000).to_json(),
            json!({ "title": "timer", "body": "time is up", "at": 5000 })
        );
    }
}
