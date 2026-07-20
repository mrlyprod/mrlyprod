use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Menu {
    apps: Vec<Json>,
    query: String,
    mode: String,
}

impl Default for Menu {
    fn default() -> Menu {
        Menu::new()
    }
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            apps: Vec::new(),
            query: String::new(),
            mode: "grid".to_string(),
        }
    }
    pub fn search(&mut self, q: &str) {
        self.query = q.to_string();
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn found(&self) -> Vec<&Json> {
        self.apps.iter().filter(|m| self.matches(m)).collect()
    }
    fn matches(&self, m: &Json) -> bool {
        let query = self.query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }
        ["route", "title", "category"].iter().any(|key| {
            m[key]
                .as_str()
                .map(|s| s.to_lowercase().contains(&query))
                .unwrap_or(false)
        })
    }
}

impl App for Menu {
    fn route(&self) -> &str {
        "menu"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("menu").category("system").hidden()
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({ "apps": self.found(), "query": self.query, "mode": self.mode })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new("menu.search", json!({ "q": "string" }))]
    }
    fn wear(&mut self, world: &Json) {
        self.apps = world["apps"]
            .as_array()
            .map(|apps| {
                apps.iter()
                    .filter(|m| m["hidden"] != json!(true))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        self.mode = match world["shared"]["settings"]["launchpad"].as_str() {
            Some("list") => "list".to_string(),
            _ => "grid".to_string(),
        };
    }
    fn save(&self) -> Json {
        json!({ "query": self.query })
    }
    fn load(&mut self, state: &Json) {
        self.query = state["query"].as_str().unwrap_or("").to_string();
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "menu.search" => {
                self.search(call.arg("q").as_str().unwrap_or(""));
                Outcome::ok(json!({ "q": self.query(), "found": self.found().len() }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> Json {
        json!({
            "apps": [
                Manifest::new("menu").hidden().to_json(),
                Manifest::new("notes").emoji("📝").to_json(),
                Manifest::new("clock").emoji("🕐").to_json(),
            ],
        })
    }

    #[test]
    fn wear_keeps_the_visible_manifests() {
        let mut menu = Menu::new();
        menu.wear(&world());
        assert_eq!(menu.apps.len(), 2);
        assert_eq!(menu.apps[0]["route"], "notes");
        assert_eq!(menu.apps[1]["route"], "clock");
    }
    #[test]
    fn state_is_the_manifest_list() {
        let mut menu = Menu::new();
        menu.wear(&world());
        let state = menu.state(&Iden::new("aria"));
        assert_eq!(state["apps"][0]["route"], "notes");
        assert_eq!(state["apps"][0]["emoji"], "📝");
        assert_eq!(state["apps"].as_array().unwrap().len(), 2);
    }
    #[test]
    fn bare_wear_empties_the_grid() {
        let mut menu = Menu::new();
        menu.wear(&world());
        menu.wear(&json!({}));
        assert!(menu.apps.is_empty());
    }
    #[test]
    fn wear_defaults_mode_to_grid_when_nothing_is_shared() {
        let mut menu = Menu::new();
        menu.wear(&world());
        let state = menu.state(&Iden::new("aria"));
        assert_eq!(state["mode"], "grid");
    }
    #[test]
    fn wear_picks_up_list_from_the_shared_world() {
        let mut menu = Menu::new();
        let mut shared = world();
        shared["shared"] = json!({ "settings": { "launchpad": "list" } });
        menu.wear(&shared);
        let state = menu.state(&Iden::new("aria"));
        assert_eq!(state["mode"], "list");
    }
    #[test]
    fn garbage_shared_values_fall_back_to_grid() {
        let mut menu = Menu::new();
        let mut shared = world();
        shared["shared"] = json!({ "settings": { "launchpad": "carousel" } });
        menu.wear(&shared);
        assert_eq!(menu.state(&Iden::new("aria"))["mode"], "grid");
    }
    #[test]
    fn search_filters_found() {
        let mut menu = Menu::new();
        menu.wear(&world());
        menu.search("Notes");
        assert_eq!(menu.found().len(), 1);
        assert_eq!(menu.found()[0]["route"], "notes");
        menu.search("");
        assert_eq!(menu.found().len(), 2);
    }
    #[test]
    fn any_verb_fails() {
        let mut menu = Menu::new();
        assert!(
            !menu
                .act(&Iden::new("aria"), &Call::new("menu.fly", json!({})))
                .ok
        );
    }
}
