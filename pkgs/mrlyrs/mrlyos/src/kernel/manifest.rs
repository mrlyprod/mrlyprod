use serde_json::{json, Value as Json};

#[derive(Clone, Debug, PartialEq)]
pub struct Manifest {
    pub route: String,
    pub emoji: String,
    pub title: String,
    pub category: String,
    pub hidden: bool,
    pub internet: bool,
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
    pub fn to_json(&self) -> Json {
        json!({
            "route": self.route,
            "emoji": self.emoji,
            "title": self.title,
            "category": self.category,
            "hidden": self.hidden,
            "internet": self.internet,
        })
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
}
