use super::app::Call;
use mrlycore::{json, Json};

/// An app's listing: route, emoji, title, category, flags and key bindings.
#[derive(Clone, Debug, PartialEq)]
pub struct Manifest {
    /// The route the manifest describes.
    pub route: String,
    /// The emoji the app wears.
    pub emoji: String,
    /// The title the app shows.
    pub title: String,
    /// The category the app files under.
    pub category: String,
    /// Whether the app stays off the menu.
    pub hidden: bool,
    /// Whether the app declares internet use.
    pub internet: bool,
    /// The key directions bound to calls.
    pub keys: Vec<(String, Call)>,
}

impl Manifest {
    /// Builds a default manifest for a route, titled after it.
    ///
    /// ```
    /// use mrlyos::kernel::Manifest;
    /// let m = Manifest::new("clock");
    /// assert_eq!(m.title, "clock");
    /// assert_eq!(m.emoji, "✨");
    /// ```
    pub fn new(route: &str) -> Manifest {
        Manifest {
            route: route.to_string(),
            emoji: "✨".to_string(),
            title: route.to_string(),
            category: "other".to_string(),
            hidden: false,
            internet: false,
            keys: Vec::new(),
        }
    }
    /// Sets the emoji and returns the manifest.
    pub fn emoji(mut self, emoji: &str) -> Manifest {
        self.emoji = emoji.to_string();
        self
    }
    /// Sets the title and returns the manifest.
    pub fn title(mut self, title: &str) -> Manifest {
        self.title = title.to_string();
        self
    }
    /// Sets the category and returns the manifest.
    pub fn category(mut self, category: &str) -> Manifest {
        self.category = category.to_string();
        self
    }
    /// Marks the app hidden and returns the manifest.
    pub fn hidden(mut self) -> Manifest {
        self.hidden = true;
        self
    }
    /// Declares internet use and returns the manifest.
    pub fn internet(mut self) -> Manifest {
        self.internet = true;
        self
    }
    /// Binds a key direction to a call and returns the manifest.
    pub fn key(mut self, dir: &str, call: Call) -> Manifest {
        self.keys.push((dir.to_string(), call));
        self
    }
    /// Returns the manifest as plain JSON, carrying keys only when bound.
    pub fn to_json(&self) -> Json {
        let mut out = json!({
            "route": &self.route,
            "emoji": &self.emoji,
            "title": &self.title,
            "category": &self.category,
            "hidden": self.hidden,
            "internet": self.internet,
        });
        if !self.keys.is_empty() {
            let mut keys = json!({});
            for (dir, call) in &self.keys {
                keys[dir.as_str()] = call.to_json();
            }
            out["keys"] = keys;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_derives_from_route() {
        let m = Manifest::new("calculator");
        assert_eq!(m.route, "calculator");
        assert_eq!(m.title, "calculator");
        assert_eq!(m.emoji, "✨");
        assert_eq!(m.category, "other");
        assert!(!m.hidden);
        assert!(!m.internet);
    }
    #[test]
    fn builders_dress_the_default() {
        let m = Manifest::new("clock").emoji("🕐").category("tools");
        assert_eq!(
            m.to_json(),
            json!({
                "route": "clock",
                "emoji": "🕐",
                "title": "clock",
                "category": "tools",
                "hidden": false,
                "internet": false,
            })
        );
    }
    #[test]
    fn internet_is_declared() {
        let m = Manifest::new("photos").internet();
        assert!(m.internet);
        assert_eq!(m.to_json()["internet"], json!(true));
    }
    #[test]
    fn keys_ride_the_json_only_when_bound() {
        let bare = Manifest::new("clock");
        assert_eq!(bare.to_json()["keys"], Json::Null);
        let m = Manifest::new("snake")
            .key("up", Call::new("snake.turn", json!({"dir": "up"})))
            .key("down", Call::new("snake.turn", json!({"dir": "down"})));
        assert_eq!(m.keys.len(), 2);
        assert_eq!(
            m.to_json()["keys"]["up"],
            json!({"verb": "snake.turn", "args": {"dir": "up"}})
        );
    }
}
