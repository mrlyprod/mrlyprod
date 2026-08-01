use super::app::Call;
use mrlycore::{json, Json};

#[derive(Clone, Debug, PartialEq)]
pub struct Manifest {
    pub route: String,
    pub emoji: String,
    pub title: String,
    pub category: String,
    pub hidden: bool,
    pub internet: bool,
    pub keys: Vec<(String, Call)>,
}

impl Manifest {
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
    pub fn emoji(mut self, emoji: &str) -> Manifest {
        self.emoji = emoji.to_string();
        self
    }
    pub fn title(mut self, title: &str) -> Manifest {
        self.title = title.to_string();
        self
    }
    pub fn category(mut self, category: &str) -> Manifest {
        self.category = category.to_string();
        self
    }
    pub fn hidden(mut self) -> Manifest {
        self.hidden = true;
        self
    }
    pub fn internet(mut self) -> Manifest {
        self.internet = true;
        self
    }
    pub fn key(mut self, dir: &str, call: Call) -> Manifest {
        self.keys.push((dir.to_string(), call));
        self
    }
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
