use super::app::{App, Call, Effect, Outcome, Verb};
use super::envelope::{Envelope, Notice, Route, Sync, View};
use super::iden::Iden;
use super::manifest::Manifest;
use super::shape::prune;
use mrlycore::codec::base64;
use mrlycore::json::Map;
use mrlycore::{json, Json};

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
    pub fn call(&mut self, call: Call) -> Outcome {
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
                "verb": &call.verb,
                "args": &call.args,
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
            "sys.shot" => self.shot(call.arg("image")),
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
                        self.apps[i].call(&iden, &call)
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
                let kept = self.apps[fi].call(&iden, &call);
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
            "ring": self.ring.clone(),
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
    fn view(&self, app: &str, shape: Option<&Json>) -> Option<View> {
        let i = self.find(app)?;
        let state = self.apps[i].state(&self.iden, shape);
        let state = match shape {
            Some(s) => prune(&state, s),
            None => state,
        };
        let params = match self.focused() {
            Some(route) if route.app == app => route.params.clone(),
            _ => json!({}),
        };
        Some(View {
            app: app.to_string(),
            params,
            state,
            actions: self.apps[i].actions(&self.iden),
            beat: self.apps[i].beat(),
        })
    }
    pub fn envelope(&self, shape: Option<&Json>) -> Envelope {
        let view = self.route.as_ref().map(|route| {
            self.view(&route.app, shape).unwrap_or(View {
                app: route.app.clone(),
                params: route.params.clone(),
                state: Json::Null,
                actions: Vec::new(),
                beat: None,
            })
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
    pub fn read(&self, path: &str, shape: Option<&Json>) -> Option<Json> {
        if path.is_empty() {
            return Some(self.envelope(shape).to_json());
        }
        let (app, path) = split(path);
        let i = self.find(&app)?;
        if let Some(head) = path.first() {
            if head == "geometry" || head == "uniforms" || head == "tris" {
                if path.len() > 1 {
                    return None;
                }
                let values = match head.as_str() {
                    "geometry" => self.apps[i].geometry(),
                    "uniforms" => self.apps[i].uniforms(),
                    _ => self.apps[i].tris(),
                };
                return Some(buffer(&values?));
            }
        }
        if path.is_empty() {
            return Some(self.view(&app, shape)?.to_json());
        }
        let state = self.apps[i].state(&self.iden, Some(&nest(&path)));
        let found = drill(&state, &path)?;
        Some(match shape {
            Some(s) => prune(found, s),
            None => found.clone(),
        })
    }
    pub fn list(&self, shape: Option<&Json>) -> Json {
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
        let out = json!({
            "version": super::VERSION,
            "apps": self.manifests().iter().map(Manifest::to_json).collect::<Vec<_>>(),
            "verbs": verbs,
            "nav": Os::nav_verbs().iter().map(Verb::to_json).collect::<Vec<_>>(),
        });
        match shape {
            Some(s) => prune(&out, s),
            None => out,
        }
    }
    pub fn catalogue(&self) -> Vec<String> {
        self.apps.iter().map(|a| a.route().to_string()).collect()
    }
    fn find(&self, app: &str) -> Option<usize> {
        self.apps.iter().position(|a| a.route() == app)
    }
    #[cfg(any(test, feature = "testkit"))]
    pub(crate) fn shoot(&self, app: &str) -> Option<Json> {
        let i = self.find(app)?;
        Some(self.apps[i].capture(&self.iden))
    }
}

fn split(path: &str) -> (String, Vec<String>) {
    let mut parts = path.split('/');
    let app = parts.next().unwrap_or("").to_string();
    (app, parts.map(str::to_string).collect())
}

fn drill<'a>(value: &'a Json, path: &[String]) -> Option<&'a Json> {
    let mut found = value;
    for step in path {
        found = match found {
            Json::Obj(map) => map.get(step)?,
            Json::Arr(items) => items.get(step.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(found)
}

fn nest(path: &[String]) -> Json {
    let mut shape = json!(1);
    for step in path.iter().rev() {
        let mut map = Map::new();
        map.insert(step.clone(), shape);
        shape = Json::Obj(map);
    }
    shape
}

fn buffer(values: &[f32]) -> Json {
    let mut bytes = Vec::with_capacity(values.len() * 4);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    json!({ "dtype": "f32", "data": base64(&bytes) })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Toy;

    impl App for Toy {
        fn route(&self) -> &str {
            "toy"
        }
        fn state(&self, _iden: &Iden, shape: Option<&Json>) -> Json {
            json!({
                "hint": shape.cloned().unwrap_or(Json::Null),
                "nest": { "leaf": 7, "twig": 8 },
                "items": [{ "id": 1 }, { "id": 2 }],
            })
        }
        fn geometry(&self) -> Option<Vec<f32>> {
            Some(vec![1.0, -2.0])
        }
        fn tris(&self) -> Option<Vec<f32>> {
            Some(vec![0.5])
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            vec![Verb::new("toy.poke", json!({}))]
        }
        fn call(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
            Outcome::ok(json!({}))
        }
        fn beat(&self) -> Option<Call> {
            Some(Call::new("toy.poke", json!({})))
        }
    }

    struct Bare;

    impl App for Bare {
        fn route(&self) -> &str {
            "bare"
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            Vec::new()
        }
        fn call(&mut self, _iden: &Iden, _call: &Call) -> Outcome {
            Outcome::ok(json!({}))
        }
    }

    fn boot() -> Os {
        Os::new(Iden::new("aria"))
            .install(Box::new(Toy))
            .install(Box::new(Bare))
    }

    #[test]
    fn empty_path_reads_the_envelope() {
        let os = boot();
        let out = os.read("", None).unwrap();
        assert_eq!(out["route"]["app"], json!("toy"));
        assert_eq!(out["view"]["app"], json!("toy"));
        assert_eq!(out["tick"], json!(0));
    }

    #[test]
    fn app_path_reads_the_view_with_an_honest_beat() {
        let out = boot().read("toy", None).unwrap();
        assert_eq!(out["state"]["nest"]["leaf"], json!(7));
        assert_eq!(out["beat"], json!({ "verb": "toy.poke", "args": {} }));
        assert_eq!(out["actions"][0]["verb"], json!("toy.poke"));
    }

    #[test]
    fn params_ride_along_only_when_focused() {
        let mut os = boot();
        os.route = Some(Route {
            app: "toy".to_string(),
            view: "main".to_string(),
            params: json!({ "id": 4 }),
        });
        assert_eq!(os.read("toy", None).unwrap()["params"], json!({ "id": 4 }));
        os.open("bare").unwrap();
        assert_eq!(os.read("toy", None).unwrap()["params"], json!({}));
    }

    #[test]
    fn deeper_paths_drill_objects_and_arrays() {
        let os = boot();
        assert_eq!(os.read("toy/nest/leaf", None), Some(json!(7)));
        assert_eq!(os.read("toy/items/1/id", None), Some(json!(2)));
    }

    #[test]
    fn missing_apps_and_paths_read_nothing() {
        let os = boot();
        assert_eq!(os.read("ghost", None), None);
        assert_eq!(os.read("ghost/nest", None), None);
        assert_eq!(os.read("toy/ghost", None), None);
        assert_eq!(os.read("toy/items/9", None), None);
        assert_eq!(os.read("toy/nest/leaf/deeper", None), None);
    }

    #[test]
    fn shape_prunes_the_drilled_subtree() {
        let os = boot();
        assert_eq!(
            os.read("toy/nest", Some(&json!({ "leaf": 1 }))),
            Some(json!({ "leaf": 7 }))
        );
    }

    #[test]
    fn the_path_becomes_the_state_hint() {
        let os = boot();
        assert_eq!(os.read("toy/hint", None), Some(json!({ "hint": 1 })));
        let out = os.read("", Some(&json!({ "hint": 1 }))).unwrap();
        assert_eq!(out["view"]["state"], json!({ "hint": { "hint": 1 } }));
    }

    #[test]
    fn reserved_heads_read_little_endian_f32() {
        let os = boot();
        assert_eq!(
            os.read("toy/geometry", None),
            Some(json!({ "dtype": "f32", "data": "AACAPwAAAMA=" }))
        );
        assert_eq!(
            os.read("toy/tris", None),
            Some(json!({ "dtype": "f32", "data": "AAAAPw==" }))
        );
        assert_eq!(os.read("toy/uniforms", None), None);
        assert_eq!(os.read("toy/geometry/0", None), None);
        assert_eq!(os.read("bare/geometry", None), None);
        assert_eq!(os.read("bare/tris", None), None);
    }

    #[test]
    fn list_names_the_version_and_the_verbs() {
        let out = boot().list(None);
        assert_eq!(out["version"], json!(super::super::VERSION));
        assert_eq!(out["verbs"][0]["app"], json!("toy"));
        assert_eq!(out["nav"][0]["verb"], json!("nav.open"));
    }
}
