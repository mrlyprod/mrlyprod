use super::app::{App, Call, Effect, Outcome, Verb};
use super::envelope::{Envelope, Notice, Route, Sync, View};
use super::iden::Iden;
use super::manifest::Manifest;
use serde_json::{json, Value as Json};

mod capture;
mod persist;

const RING: usize = 100;

pub struct Os {
    iden: Iden,
    apps: Vec<Box<dyn App>>,
    route: Option<Route>,
    tick: u64,
    now: i64,
    last: Option<Outcome>,
    effects: Vec<Effect>,
    notices: Vec<Notice>,
    ring: Vec<Json>,
}

impl Os {
    pub fn new(iden: Iden) -> Os {
        Os {
            iden,
            apps: Vec::new(),
            route: Some(Route::new("menu")),
            tick: 0,
            now: 0,
            last: None,
            effects: Vec::new(),
            notices: Vec::new(),
            ring: Vec::new(),
        }
    }
    pub fn install(mut self, app: Box<dyn App>) -> Os {
        if self.apps.is_empty() {
            self.route = Some(Route::new(app.route()));
        }
        self.apps.push(app);
        self.dress();
        self
    }
    fn focused(&self) -> Option<&Route> {
        self.route.as_ref()
    }
    pub fn open(&mut self, app: &str) -> Result<(), &'static str> {
        if self.find(app).is_none() {
            return Err("no such app");
        }
        self.route = Some(Route::new(app));
        Ok(())
    }
    pub fn act(&mut self, call: Call) -> Outcome {
        self.tick += 1;
        let mut call = call;
        match call.now {
            Some(now) => self.now = now,
            None => call.now = Some(self.now),
        }
        let now = self.now;
        let tick = self.tick;
        let joined = match (self.ring.last_mut(), Self::runs(&call.args)) {
            (Some(tail), Some(add))
                if !call.verb.starts_with("sys.")
                    && tail["verb"].as_str() == Some(call.verb.as_str()) =>
            {
                match Self::runs(&tail["args"]) {
                    Some(held) => {
                        tail["args"] = json!({ "n": held + add });
                        tail["now"] = json!(now);
                        tail["tick"] = json!(tick);
                        true
                    }
                    None => false,
                }
            }
            _ => false,
        };
        if !joined {
            self.ring.push(json!({
                "verb": call.verb,
                "args": call.args,
                "now": now,
                "tick": tick,
            }));
            if self.ring.len() > RING {
                self.ring.remove(0);
            }
        }
        let mut target = self.focused().map(|r| r.app.clone()).unwrap_or_default();
        let mut outcome = match call.verb.as_str() {
            "nav.open" => {
                let app = call.arg("app").as_str().unwrap_or("");
                match self.open(app) {
                    Ok(()) => Outcome::ok(json!({ "app": app })),
                    Err(note) => Outcome::fail(note),
                }
            }
            "sys.freeze" => Outcome::ok(self.freeze()),
            "sys.thaw" => self.thaw(call.arg("state")),
            "sys.shot" => self.shot(),
            "sys.dismiss" => {
                let count = self.notices.len();
                self.notices.clear();
                Outcome::ok(json!({ "dismissed": count }))
            }
            _ => {
                let prefix = call.verb.split('.').next().unwrap_or("");
                if self.find(prefix).is_some() {
                    target = prefix.to_string();
                }
                match self.find(&target.clone()) {
                    Some(i) => {
                        let iden = self.iden.clone();
                        self.apps[i].act(&iden, &call)
                    }
                    None => Outcome::fail("no current app"),
                }
            }
        };
        self.effects = std::mem::take(&mut outcome.effects);
        if self.effects.iter().any(|e| e.kind == "fetch") {
            let online = self
                .find(&target)
                .map(|i| self.apps[i].manifest().internet)
                .unwrap_or(false);
            if !online {
                self.effects.retain(|e| e.kind != "fetch");
                self.notices.push(Notice::new(
                    "refused",
                    &format!("{} has no internet", target),
                    self.now,
                ));
            }
        }
        for effect in &self.effects {
            if effect.kind == "notify" {
                self.notices.push(Notice::new(
                    effect.data["title"].as_str().unwrap_or(""),
                    effect.data["body"].as_str().unwrap_or(""),
                    self.now,
                ));
            }
        }
        let files: Vec<Effect> = self
            .effects
            .iter()
            .filter(|e| e.kind == "file")
            .cloned()
            .collect();
        self.effects.retain(|e| e.kind != "file");
        if let Some(fi) = self.find("files") {
            let iden = self.iden.clone();
            for e in &files {
                let call = Call::new("files.keep", e.data.clone()).at(self.now);
                let kept = self.apps[fi].act(&iden, &call);
                if kept.ok {
                    let name = e.data["name"].as_str().unwrap_or("file");
                    self.notices
                        .push(Notice::new("saved", &format!("{name} → files"), self.now));
                }
            }
        }
        self.last = Some(outcome.clone());
        self.dress();
        outcome
    }
    fn runs(args: &Json) -> Option<i64> {
        let map = args.as_object()?;
        match map.len() {
            0 => Some(1),
            1 => map.get("n").and_then(|n| n.as_i64()),
            _ => None,
        }
    }
    fn manifests(&self) -> Vec<Manifest> {
        self.apps.iter().map(|a| a.manifest()).collect()
    }
    fn world(&self) -> Json {
        let mut shared = json!({});
        for app in &self.apps {
            if let Some(facts) = app.share() {
                shared[app.route()] = facts;
            }
        }
        json!({
            "apps": self.manifests().iter().map(Manifest::to_json).collect::<Vec<_>>(),
            "shared": shared,
            "ring": self.ring,
        })
    }
    fn dress(&mut self) {
        let world = self.world();
        for app in &mut self.apps {
            app.wear(&world);
        }
    }
    fn nav_verbs() -> Vec<Verb> {
        vec![Verb::new("nav.open", json!({ "app": "string" }))]
    }
    pub fn frame(&self) -> Envelope {
        let view = self.route.as_ref().map(|route| {
            let (state, actions, beat) = match self.find(&route.app) {
                Some(i) => (
                    self.apps[i].state(&self.iden),
                    self.apps[i].actions(&self.iden),
                    self.apps[i].beat(),
                ),
                None => (Json::Null, Vec::new(), None),
            };
            View {
                app: route.app.clone(),
                params: route.params.clone(),
                state,
                actions,
                beat,
            }
        });
        Envelope {
            tick: self.tick,
            route: self.route.clone(),
            view,
            last: self.last.clone(),
            sync: Sync::Synced,
            effects: self.effects.clone(),
            notices: self.notices.clone(),
        }
    }
    pub fn geometry(&self, app: &str) -> Option<Vec<f32>> {
        let i = self.find(app)?;
        self.apps[i].geometry()
    }
    pub fn peek(&self, app: &str) -> Option<View> {
        let i = self.find(app)?;
        Some(View {
            app: app.to_string(),
            params: json!({}),
            state: self.apps[i].state(&self.iden),
            actions: self.apps[i].actions(&self.iden),
            beat: None,
        })
    }
    pub fn describe(&self) -> Json {
        let verbs: Vec<Json> = self
            .apps
            .iter()
            .map(|a| {
                json!({
                    "app": a.route(),
                    "verbs": a.actions(&self.iden).iter().map(Verb::to_json).collect::<Vec<_>>(),
                })
            })
            .collect();
        json!({
            "version": env!("MRLY_VERSION"),
            "apps": self.manifests().iter().map(Manifest::to_json).collect::<Vec<_>>(),
            "verbs": verbs,
            "nav": Os::nav_verbs().iter().map(Verb::to_json).collect::<Vec<_>>(),
        })
    }
    pub fn catalogue(&self) -> Vec<String> {
        self.apps.iter().map(|a| a.route().to_string()).collect()
    }
    fn find(&self, app: &str) -> Option<usize> {
        self.apps.iter().position(|a| a.route() == app)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::Notes;
    struct Home;
    impl App for Home {
        fn route(&self) -> &str {
            "home"
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            Vec::new()
        }
        fn act(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
            Outcome::ok(json!({}))
        }
    }
    struct Pulse {
        now: i64,
    }
    impl App for Pulse {
        fn route(&self) -> &str {
            "pulse"
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            vec![Verb::new("pulse.beep", json!({}))]
        }
        fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
            self.now = call.now.unwrap_or(0);
            Outcome::ok(json!({ "now": self.now })).emit(
                Effect::new("notify", json!({ "title": "beep" }))
                    .then(Call::new("pulse.beep", json!({}))),
            )
        }
        fn beat(&self) -> Option<Call> {
            Some(Call::new("pulse.beep", json!({})))
        }
    }
    struct Wire {
        online: bool,
    }
    impl App for Wire {
        fn route(&self) -> &str {
            "wire"
        }
        fn manifest(&self) -> Manifest {
            if self.online {
                Manifest::new("wire").internet()
            } else {
                Manifest::new("wire")
            }
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            vec![Verb::new("wire.pull", json!({}))]
        }
        fn act(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
            Outcome::ok(json!({})).emit(
                Effect::new(
                    "fetch",
                    json!({ "url": "https://example.com", "as": "bytes" }),
                )
                .then(Call::new("wire.land", json!({}))),
            )
        }
    }
    fn boot() -> Os {
        Os::new(Iden::new("aria"))
            .install(Box::new(Notes::new()))
            .install(Box::new(Home))
    }
    fn focused_state(os: &Os) -> Json {
        os.frame().view.unwrap().state
    }
    #[test]
    fn routes_to_first_app() {
        assert_eq!(boot().frame().route.unwrap().app, "notes");
    }
    #[test]
    fn act_flows_to_app() {
        let mut os = boot();
        let out = os.act(Call::new("notes.add", json!({ "text": "milk" })));
        assert!(out.ok);
        let frame = os.frame();
        assert_eq!(frame.view.unwrap().state["found"][0]["text"], "milk");
        assert_eq!(frame.tick, 1);
    }
    #[test]
    fn frame_carries_the_open_routes_state() {
        let mut os = boot();
        os.act(Call::new("notes.add", json!({ "text": "milk" })));
        let frame = os.frame().to_json();
        assert_eq!(frame["view"]["app"], "notes");
        assert_eq!(frame["view"]["state"]["query"], "");
        assert_eq!(frame["view"]["state"]["found"][0]["text"], "milk");
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(Notes::new()))
            .install(Box::new(crate::apps::Menu::new()));
        os.act(Call::new("nav.open", json!({ "app": "menu" })));
        let state = focused_state(&os);
        assert_eq!(state["apps"][0]["route"], "notes");
        assert_eq!(state["apps"].as_array().unwrap().len(), 1);
    }
    #[test]
    fn frame_exposes_the_view_verbs() {
        let frame = boot().frame();
        let names: Vec<String> = frame
            .view
            .as_ref()
            .unwrap()
            .actions
            .iter()
            .map(|v| v.name.clone())
            .collect();
        assert!(names.contains(&"notes.add".to_string()));
        assert!(!names.contains(&"nav.open".to_string()));
    }
    #[test]
    fn peek_reads_a_background_app() {
        let mut os = boot();
        os.act(Call::new("notes.add", json!({ "text": "milk" })));
        os.act(Call::new("nav.open", json!({ "app": "home" })));
        let view = os.peek("notes").unwrap();
        assert_eq!(view.app, "notes");
        assert_eq!(view.state["found"][0]["text"], "milk");
        assert!(view.beat.is_none());
        assert!(view.actions.iter().any(|v| v.name == "notes.add"));
        assert!(os.peek("ghost").is_none());
    }
    #[test]
    fn verb_prefix_reaches_the_unfocused_app() {
        let mut os = boot();
        os.act(Call::new("nav.open", json!({ "app": "home" })));
        assert!(os.act(Call::new("notes.add", json!({ "text": "milk" }))).ok);
        let frame = os.frame();
        assert_eq!(frame.route.as_ref().unwrap().app, "home");
        assert_eq!(os.peek("notes").unwrap().state["found"][0]["text"], "milk");
    }
    #[test]
    fn unknown_prefix_falls_to_the_focused_app() {
        let mut os = boot();
        assert!(!os.act(Call::new("ghost.fly", json!({}))).ok);
        assert!(!os.act(Call::new("dotless", json!({}))).ok);
    }
    #[test]
    fn fetch_gate_follows_the_verb_prefix() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(Home))
            .install(Box::new(Wire { online: true }));
        assert!(os.act(Call::new("wire.pull", json!({}))).ok);
        let frame = os.frame().to_json();
        assert_eq!(frame["route"]["app"], "home");
        assert_eq!(frame["effects"][0]["kind"], "fetch");
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(Home))
            .install(Box::new(Wire { online: false }));
        os.act(Call::new("wire.pull", json!({})).at(5000));
        let frame = os.frame().to_json();
        assert_eq!(frame["effects"], Json::Null);
        assert_eq!(frame["notices"][0]["body"], "wire has no internet");
    }
    #[test]
    fn unknown_verb_fails_cleanly() {
        let mut os = boot();
        assert!(!os.act(Call::new("notes.fly", json!({}))).ok);
    }
    #[test]
    fn nav_open_replaces_the_route() {
        let mut os = boot();
        assert!(os.act(Call::new("nav.open", json!({ "app": "home" }))).ok);
        assert_eq!(os.frame().route.unwrap().app, "home");
        assert!(os.act(Call::new("nav.open", json!({ "app": "notes" }))).ok);
        let frame = os.frame();
        assert_eq!(frame.route.unwrap().app, "notes");
        assert_eq!(frame.view.unwrap().app, "notes");
    }
    #[test]
    fn nav_open_missing_fails() {
        let mut os = boot();
        assert!(!os.act(Call::new("nav.open", json!({ "app": "ghost" }))).ok);
        assert_eq!(os.frame().route.unwrap().app, "notes");
    }
    #[test]
    fn envelope_serializes() {
        let j = boot().frame().to_json();
        assert_eq!(j["sync"], "synced");
        assert_eq!(j["views"], Json::Null);
        assert_eq!(j["focus"], Json::Null);
        assert_eq!(j["view"]["app"], "notes");
        assert_eq!(j["view"]["params"], json!({}));
        assert_eq!(j["view"]["state"]["query"], "");
        assert!(j["view"]["actions"].is_array());
        assert_eq!(j["nav"], Json::Null);
        assert_eq!(j["apps"], Json::Null);
    }
    #[test]
    fn kernel_remembers_now_and_stamps_the_unstamped() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(Pulse { now: -1 }));
        os.act(Call::new("pulse.beep", json!({})).at(5000));
        os.act(Call::new("pulse.beep", json!({})));
        assert_eq!(os.frame().last.unwrap().data["now"], json!(5000));
    }
    #[test]
    fn effects_move_to_the_envelope() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(Pulse { now: 0 }));
        let out = os.act(Call::new("pulse.beep", json!({})));
        assert!(out.effects.is_empty());
        let frame = os.frame().to_json();
        assert_eq!(frame["effects"][0]["kind"], "notify");
        assert_eq!(frame["effects"][0]["data"]["title"], "beep");
        assert_eq!(frame["effects"][0]["call"]["verb"], "pulse.beep");
        assert_eq!(frame["last"]["effects"], Json::Null);
        os.act(Call::new("nav.open", json!({ "app": "pulse" })));
        assert_eq!(os.frame().to_json()["effects"], Json::Null);
    }
    #[test]
    fn notify_effects_land_in_the_notice_log() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(Pulse { now: 0 }));
        os.act(Call::new("pulse.beep", json!({})).at(5000));
        os.act(Call::new("pulse.beep", json!({})).at(6000));
        let frame = os.frame().to_json();
        assert_eq!(frame["notices"].as_array().unwrap().len(), 2);
        assert_eq!(frame["notices"][0]["title"], "beep");
        assert_eq!(frame["notices"][0]["at"], json!(5000));
        assert_eq!(frame["notices"][1]["at"], json!(6000));
    }
    #[test]
    fn declared_internet_lets_fetch_through() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(Wire { online: true }));
        let out = os.act(Call::new("wire.pull", json!({})));
        assert!(out.ok);
        let frame = os.frame().to_json();
        assert_eq!(frame["effects"][0]["kind"], "fetch");
        assert_eq!(frame["notices"], Json::Null);
    }
    #[test]
    fn undeclared_fetch_is_refused() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(Wire { online: false }));
        let out = os.act(Call::new("wire.pull", json!({})).at(5000));
        assert!(out.ok);
        let frame = os.frame().to_json();
        assert_eq!(frame["effects"], Json::Null);
        assert_eq!(frame["notices"][0]["title"], "refused");
        assert_eq!(frame["notices"][0]["body"], "wire has no internet");
        assert_eq!(frame["notices"][0]["at"], json!(5000));
    }
    #[test]
    fn quiet_envelope_omits_notices() {
        assert_eq!(boot().frame().to_json()["notices"], Json::Null);
    }
    #[test]
    fn dismiss_clears_the_notice_log() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(Pulse { now: 0 }));
        os.act(Call::new("pulse.beep", json!({})));
        let out = os.act(Call::new("sys.dismiss", json!({})));
        assert!(out.ok);
        assert_eq!(out.data["dismissed"], json!(1));
        assert_eq!(os.frame().to_json()["notices"], Json::Null);
    }
    #[test]
    fn freeze_thaw_carries_notices() {
        let mut a = Os::new(Iden::new("aria")).install(Box::new(Pulse { now: 0 }));
        a.act(Call::new("pulse.beep", json!({})).at(5000));
        let state = a.act(Call::new("sys.freeze", json!({}))).data;
        assert_eq!(state["notices"][0]["title"], "beep");
        let mut b = Os::new(Iden::new("aria")).install(Box::new(Pulse { now: 0 }));
        b.act(Call::new("sys.thaw", json!({ "state": state })));
        let frame = b.frame().to_json();
        assert_eq!(frame["notices"][0]["title"], "beep");
        assert_eq!(frame["notices"][0]["at"], json!(5000));
        let mut c = boot();
        c.act(Call::new(
            "sys.thaw",
            json!({ "state": { "route": "notes", "history": [], "apps": {} } }),
        ));
        assert_eq!(c.frame().to_json()["notices"], Json::Null);
    }
    #[test]
    fn beat_rides_the_focused_view() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(Pulse { now: 0 }))
            .install(Box::new(Home));
        assert_eq!(
            os.frame().to_json()["view"]["beat"],
            json!({ "verb": "pulse.beep", "args": {} })
        );
        os.act(Call::new("nav.open", json!({ "app": "home" })));
        assert_eq!(os.frame().to_json()["view"]["beat"], Json::Null);
    }
    #[test]
    fn freeze_captures_the_kernel() {
        let mut os = boot();
        os.act(Call::new("notes.add", json!({ "text": "milk" })).at(5000));
        os.act(Call::new("nav.open", json!({ "app": "home" })));
        let out = os.act(Call::new("sys.freeze", json!({})));
        assert!(out.ok);
        assert_eq!(out.data["route"], "home");
        assert_eq!(out.data["dock"], Json::Null);
        assert_eq!(out.data["history"], Json::Null);
        assert_eq!(out.data["now"], json!(5000));
        assert_eq!(out.data["apps"]["notes"]["items"][0]["text"], "milk");
        assert_eq!(out.data["apps"]["home"], Json::Null);
    }
    #[test]
    fn thaw_restores_a_fresh_boot() {
        let mut a = boot();
        a.act(Call::new("notes.add", json!({ "text": "milk" })).at(5000));
        a.act(Call::new("nav.open", json!({ "app": "home" })));
        let state = a.act(Call::new("sys.freeze", json!({}))).data;
        let mut b = boot();
        let out = b.act(Call::new("sys.thaw", json!({ "state": state })));
        assert!(out.ok);
        assert_eq!(out.data["route"], "home");
        assert_eq!(out.data["apps"], json!(["notes"]));
        let frame = b.frame();
        assert_eq!(frame.route.as_ref().unwrap().app, "home");
        assert!(b.act(Call::new("nav.open", json!({ "app": "notes" }))).ok);
        assert_eq!(b.frame().route.unwrap().app, "notes");
        assert_eq!(focused_state(&b)["found"][0]["text"], "milk");
    }
    #[test]
    fn thaw_restores_the_remembered_now() {
        let mut a = boot();
        a.act(Call::new("notes.add", json!({ "text": "milk" })).at(5000));
        let state = a.act(Call::new("sys.freeze", json!({}))).data;
        let mut b = Os::new(Iden::new("aria")).install(Box::new(Pulse { now: -1 }));
        b.act(Call::new("sys.thaw", json!({ "state": state })).at(9000));
        b.act(Call::new("pulse.beep", json!({})));
        assert_eq!(b.frame().last.unwrap().data["now"], json!(5000));
    }
    #[test]
    fn thaw_rejects_garbage() {
        let mut os = boot();
        assert!(!os.act(Call::new("sys.thaw", json!({}))).ok);
        assert!(!os.act(Call::new("sys.thaw", json!({ "state": 7 }))).ok);
        let out = os.act(Call::new(
            "sys.thaw",
            json!({ "state": { "route": "ghost", "history": ["ghost"], "apps": {} } }),
        ));
        assert!(out.ok);
        assert_eq!(os.frame().route, None);
        assert!(os.frame().view.is_none());
        assert!(os.act(Call::new("nav.open", json!({ "app": "notes" }))).ok);
        assert_eq!(os.frame().route.unwrap().app, "notes");
    }
    #[test]
    fn thaw_lands_on_the_route() {
        let mut os = boot();
        os.act(Call::new(
            "sys.thaw",
            json!({ "state": { "route": "home", "apps": {} } }),
        ));
        let frame = os.frame();
        assert_eq!(frame.route.unwrap().app, "home");
        assert_eq!(frame.view.unwrap().app, "home");
    }
    #[test]
    fn describe_covers_the_surface() {
        let d = boot().describe();
        assert_eq!(d["version"], json!(env!("MRLY_VERSION")));
        assert_eq!(d["apps"][0]["route"], "notes");
        assert_eq!(d["apps"][1]["route"], "home");
        assert_eq!(d["verbs"][0]["app"], "notes");
        assert_eq!(d["nav"][0]["verb"], "nav.open");
        assert_eq!(d["kinds"], Json::Null);
    }
    #[test]
    fn settings_set_validates_in_app() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(Home))
            .install(Box::new(crate::apps::Settings::new()));
        os.act(Call::new("nav.open", json!({ "app": "settings" })));
        let out = os.act(Call::new(
            "settings.set",
            json!({ "key": "scale", "value": 99 }),
        ));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("out of range"));
        let out = os.act(Call::new(
            "settings.set",
            json!({ "key": "color", "value": "mint" }),
        ));
        assert!(out.ok);
        assert_eq!(focused_state(&os)["color"], json!("mint"));
    }
    #[test]
    fn shot_lands_in_the_roll() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(crate::apps::Two::new()))
            .install(Box::new(crate::apps::Photos::new()));
        os.act(Call::new("nav.open", json!({ "app": "two" })));
        let out = os.act(Call::new("sys.shot", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["shot"], json!("two"));
        let frame = os.frame().to_json();
        assert_eq!(frame["notices"][0]["title"], "saved");
        assert_eq!(frame["notices"][0]["body"], "screenshot → photos");
        assert_eq!(frame["notices"][0]["at"], json!(5000));
        os.act(Call::new("nav.open", json!({ "app": "photos" })));
        let photos = focused_state(&os)["photos"].clone();
        let photos = photos.as_array().unwrap();
        assert_eq!(photos.len(), 1);
        assert!(photos[0]
            .as_str()
            .unwrap()
            .starts_with("data:image/png;base64,"));
    }
    #[test]
    fn shot_fails_without_a_frame() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(crate::apps::Colors::new()))
            .install(Box::new(crate::apps::Photos::new()));
        let out = os.act(Call::new("sys.shot", json!({})));
        assert!(!out.ok);
        os.act(Call::new("nav.open", json!({ "app": "photos" })));
        assert_eq!(focused_state(&os)["photos"].as_array().unwrap().len(), 0);
    }
    #[test]
    fn shot_on_photos_fails() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(crate::apps::Photos::new()));
        let out = os.act(Call::new("sys.shot", json!({})));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("nothing to shoot here"));
    }
    #[test]
    fn file_effect_lands_in_files() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(crate::apps::Colors::new()))
            .install(Box::new(crate::apps::Files::new()));
        os.act(Call::new("nav.open", json!({ "app": "colors" })));
        let out = os.act(Call::new("colors.export", json!({})).at(5000));
        assert!(out.ok);
        let frame = os.frame().to_json();
        assert_eq!(frame["effects"], Json::Null);
        assert_eq!(frame["notices"][0]["title"], "saved");
        assert_eq!(frame["notices"][0]["body"], "palette.json → files");
        assert_eq!(frame["notices"][0]["at"], json!(5000));
        os.act(Call::new("nav.open", json!({ "app": "files" })));
        let state = focused_state(&os);
        let files = state["files"].as_array().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0]["name"], json!("palette.json"));
        assert_eq!(files[0]["tick"], json!(5000));
        assert!(files[0]["uri"]
            .as_str()
            .unwrap()
            .starts_with("data:application/json;base64,"));
    }
    #[test]
    fn file_effect_without_files_app_is_dropped() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(crate::apps::Colors::new()));
        os.act(Call::new("nav.open", json!({ "app": "colors" })));
        let out = os.act(Call::new("colors.export", json!({})));
        assert!(out.ok);
        assert_eq!(os.frame().to_json()["effects"], Json::Null);
    }
    #[test]
    fn the_ring_records_every_call() {
        let mut os = boot();
        os.act(Call::new("notes.add", json!({ "text": "milk" })).at(5000));
        os.act(Call::new("notes.fly", json!({})));
        let state = os.act(Call::new("sys.freeze", json!({}))).data;
        let ring = state["ring"].as_array().unwrap();
        assert_eq!(ring.len(), 3);
        assert_eq!(ring[0]["verb"], "notes.add");
        assert_eq!(ring[0]["args"]["text"], "milk");
        assert_eq!(ring[0]["now"], json!(5000));
        assert_eq!(ring[0]["tick"], json!(1));
        assert_eq!(ring[1]["verb"], "notes.fly");
        assert_eq!(ring[2]["verb"], "sys.freeze");
        assert_eq!(ring[2]["tick"], json!(3));
    }
    #[test]
    fn the_ring_coalesces_step_runs() {
        let mut os = boot();
        os.act(Call::new("nav.open", json!({ "app": "solids" })));
        os.act(Call::new("solids.step", json!({})));
        os.act(Call::new("solids.step", json!({})));
        os.act(Call::new("solids.step", json!({ "n": 4 })));
        os.act(Call::new("nav.open", json!({ "app": "solids" })));
        os.act(Call::new("solids.step", json!({})));
        let state = os.act(Call::new("sys.freeze", json!({}))).data;
        let ring = state["ring"].as_array().unwrap();
        assert_eq!(ring.len(), 5);
        assert_eq!(ring[1]["verb"], "solids.step");
        assert_eq!(ring[1]["args"]["n"], json!(6));
        assert_eq!(ring[1]["tick"], json!(4));
        assert_eq!(ring[3]["verb"], "solids.step");
        assert_eq!(ring[3]["args"], json!({}));
    }
    #[test]
    fn the_ring_caps_at_a_hundred() {
        let mut os = boot();
        for i in 0..150 {
            os.act(Call::new("notes.search", json!({ "q": i.to_string() })));
        }
        let state = os.act(Call::new("sys.freeze", json!({}))).data;
        let ring = state["ring"].as_array().unwrap();
        assert_eq!(ring.len(), 100);
        assert_eq!(ring[0]["tick"], json!(52));
        assert_eq!(ring[99]["verb"], "sys.freeze");
        assert_eq!(ring[99]["tick"], json!(151));
    }
    #[test]
    fn freeze_thaw_carries_ring_and_tick() {
        let mut a = boot();
        a.act(Call::new("notes.add", json!({ "text": "milk" })).at(5000));
        let state = a.act(Call::new("sys.freeze", json!({}))).data;
        assert_eq!(state["tick"], json!(2));
        let mut b = boot();
        b.act(Call::new("sys.thaw", json!({ "state": state })));
        assert_eq!(b.frame().tick, 2);
        let frozen = b.act(Call::new("sys.freeze", json!({}))).data;
        assert_eq!(frozen["tick"], json!(3));
        let ring = frozen["ring"].as_array().unwrap();
        assert_eq!(ring.len(), 3);
        assert_eq!(ring[0]["verb"], "notes.add");
        assert_eq!(ring[1]["verb"], "sys.freeze");
        assert_eq!(ring[2]["verb"], "sys.freeze");
        assert_eq!(ring[2]["tick"], json!(3));
        let mut c = boot();
        c.act(Call::new(
            "sys.thaw",
            json!({ "state": { "route": "notes", "history": [], "apps": {} } }),
        ));
        let bare = c.act(Call::new("sys.freeze", json!({}))).data;
        assert_eq!(bare["ring"].as_array().unwrap().len(), 1);
    }
    #[test]
    fn the_world_wears_the_ring() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(crate::apps::Log::new()))
            .install(Box::new(Home));
        os.act(Call::new("nav.open", json!({ "app": "home" })));
        os.act(Call::new("nav.open", json!({ "app": "log" })));
        let state = focused_state(&os);
        let entries = state["entries"].as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0]["verb"], "nav.open");
        assert_eq!(entries[1]["verb"], "nav.open");
    }
    #[test]
    fn shared_facts_reach_the_worn_app() {
        let mut os = Os::new(Iden::new("aria"))
            .install(Box::new(crate::apps::Settings::new()))
            .install(Box::new(crate::apps::Clock::new()));
        os.act(Call::new(
            "settings.set",
            json!({ "key": "font", "value": "mrly" }),
        ));
        os.act(Call::new("nav.open", json!({ "app": "clock" })));
        os.act(Call::new("clock.tick", json!({})).at(45296000));
        let state = focused_state(&os);
        assert_eq!(state["glyph"]["text"], json!("12:34:56"));
        assert_eq!(state["glyph"]["height"], json!(5));
    }
}
