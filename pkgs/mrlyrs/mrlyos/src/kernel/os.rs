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
            "version": super::VERSION,
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
