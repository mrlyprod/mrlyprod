use super::Os;
use crate::os::kernel::app::Outcome;
use crate::os::kernel::envelope::{Notice, Route};
use serde_json::{json, Value as Json};

impl Os {
    pub fn freeze(&self) -> Json {
        let mut apps = json!({});
        for app in &self.apps {
            apps[app.route()] = app.save();
        }
        json!({
            "route": self.focused().map(|r| r.app.clone()),
            "now": self.now,
            "tick": self.tick,
            "ring": self.ring,
            "notices": self.notices.iter().map(Notice::to_json).collect::<Vec<_>>(),
            "apps": apps,
        })
    }
    pub fn thaw(&mut self, state: &Json) -> Outcome {
        if !state.is_object() {
            return Outcome::fail("state must be an object");
        }
        let mut restored = Vec::new();
        for app in &mut self.apps {
            let saved = &state["apps"][app.route()];
            if !saved.is_null() {
                app.load(saved);
                restored.push(app.route().to_string());
            }
        }
        self.route = state["route"]
            .as_str()
            .filter(|app| self.find(app).is_some())
            .map(Route::new);
        if let Some(now) = state["now"].as_i64() {
            self.now = now;
        }
        if let Some(tick) = state["tick"].as_u64() {
            self.tick = tick;
        }
        self.ring = state["ring"].as_array().cloned().unwrap_or_default();
        self.notices = state["notices"]
            .as_array()
            .map(|notices| {
                notices
                    .iter()
                    .map(|n| {
                        Notice::new(
                            n["title"].as_str().unwrap_or(""),
                            n["body"].as_str().unwrap_or(""),
                            n["at"].as_i64().unwrap_or(0),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.dress();
        Outcome::ok(json!({ "route": self.focused().map(|r| r.app.clone()), "apps": restored }))
    }
}
