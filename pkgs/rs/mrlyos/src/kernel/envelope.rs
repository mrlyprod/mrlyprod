use super::app::{Call, Effect, Outcome, Verb};
use mrlycore::{json, Json};

/// Where the world is looking: an app, a view within it, and its params.
#[derive(Clone, Debug, PartialEq)]
pub struct Route {
    /// The app being viewed.
    pub app: String,
    /// The view within the app.
    pub view: String,
    /// The parameters of the view.
    pub params: Json,
}

impl Route {
    /// Builds a route to an app's main view with empty params.
    pub fn new(app: &str) -> Route {
        Route {
            app: app.to_string(),
            view: "main".to_string(),
            params: json!({}),
        }
    }
    /// Returns the route as plain JSON.
    pub fn to_json(&self) -> Json {
        json!({ "app": &self.app, "view": &self.view, "params": &self.params })
    }
}

/// A message raised for the user, stamped with its moment.
#[derive(Clone, Debug, PartialEq)]
pub struct Notice {
    /// The notice title.
    pub title: String,
    /// The notice body.
    pub body: String,
    /// The moment the notice fired.
    pub at: i64,
}

impl Notice {
    /// Builds a notice from a title, a body and a moment.
    pub fn new(title: &str, body: &str, at: i64) -> Notice {
        Notice {
            title: title.to_string(),
            body: body.to_string(),
            at,
        }
    }
    /// Returns the notice as plain JSON.
    pub fn to_json(&self) -> Json {
        json!({ "title": &self.title, "body": &self.body, "at": self.at })
    }
}

/// The envelope's standing against its remote.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sync {
    /// The sync is still owed.
    Pending,
    /// The sync is complete.
    Synced,
    /// The sync did not land.
    Failed,
}

impl Sync {
    /// Returns the lowercase name of the standing.
    pub fn name(&self) -> &'static str {
        match self {
            Sync::Pending => "pending",
            Sync::Synced => "synced",
            Sync::Failed => "failed",
        }
    }
}

/// The focused app's face: its state, its actions, and an optional beat.
pub struct View {
    /// The app being shown.
    pub app: String,
    /// The parameters of the view.
    pub params: Json,
    /// The app's visible state.
    pub state: Json,
    /// The verbs on offer.
    pub actions: Vec<Verb>,
    /// The call the app wants repeated every beat, if any.
    pub beat: Option<Call>,
}

impl View {
    /// Returns the view as plain JSON, carrying the beat only when present.
    pub fn to_json(&self) -> Json {
        let mut out = json!({
            "app": &self.app,
            "params": &self.params,
            "state": &self.state,
            "actions": self.actions.iter().map(Verb::to_json).collect::<Vec<_>>(),
        });
        if let Some(beat) = &self.beat {
            out["beat"] = beat.to_json();
        }
        out
    }
}

/// One frame of the whole world, ready to hand across the wire.
pub struct Envelope {
    /// The tick the frame was cut at.
    pub tick: u64,
    /// Where the world is looking, if anywhere.
    pub route: Option<Route>,
    /// The focused app's view, if any.
    pub view: Option<View>,
    /// The outcome of the latest call, if any.
    pub last: Option<Outcome>,
    /// The frame's standing against its remote.
    pub sync: Sync,
    /// The effects awaiting the outside.
    pub effects: Vec<Effect>,
    /// The notices raised for the user.
    pub notices: Vec<Notice>,
}

impl Envelope {
    /// Returns the envelope as plain JSON, carrying effects and notices only when present.
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
