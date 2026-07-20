use crate::music::cue;
use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Timer {
    stopwatch: bool,
    deadline: Option<i64>,
    paused: Option<i64>,
    rung: bool,
    origin: Option<i64>,
    banked: i64,
    laps: Vec<i64>,
    now: i64,
    glyphs: bool,
}

impl Default for Timer {
    fn default() -> Timer {
        Timer::new()
    }
}

fn minutes(arg: &Json) -> Option<f64> {
    let m = match arg {
        Json::String(s) => s.trim().parse::<f64>().ok()?,
        _ => arg.as_f64()?,
    };
    if m > 0.0 && m <= 1440.0 {
        Some(m)
    } else {
        None
    }
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            stopwatch: false,
            deadline: None,
            paused: None,
            rung: false,
            origin: None,
            banked: 0,
            laps: Vec::new(),
            now: 0,
            glyphs: false,
        }
    }
    fn remaining(&self) -> i64 {
        match (self.paused, self.deadline) {
            (Some(left), _) => left,
            (None, Some(deadline)) => (deadline - self.now).max(0),
            (None, None) => 0,
        }
    }
    fn elapsed(&self) -> i64 {
        self.banked + self.origin.map(|o| (self.now - o).max(0)).unwrap_or(0)
    }
    fn armed(&self) -> bool {
        if self.stopwatch {
            self.origin.is_some() || self.banked > 0 || !self.laps.is_empty()
        } else {
            self.deadline.is_some() || self.paused.is_some()
        }
    }
    fn running(&self) -> bool {
        if self.stopwatch {
            self.origin.is_some()
        } else {
            self.deadline.is_some() && !self.rung
        }
    }
    fn wipe(&mut self) {
        self.deadline = None;
        self.paused = None;
        self.rung = false;
        self.origin = None;
        self.banked = 0;
        self.laps.clear();
    }
    fn face(&self) -> String {
        if self.stopwatch {
            let secs = self.elapsed() / 1000;
            format!("{:02}:{:02}", secs / 60, secs % 60)
        } else if self.armed() {
            let secs = (self.remaining() + 999) / 1000;
            format!("{:02}:{:02}", secs / 60, secs % 60)
        } else {
            "--:--".to_string()
        }
    }
}

impl App for Timer {
    fn route(&self) -> &str {
        "timer"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("timer").emoji("⏱️").category("tools")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let mut out = json!({
            "mode": if self.stopwatch { "stopwatch" } else { "countdown" },
            "armed": self.armed(),
            "remaining": self.remaining(),
            "rung": self.rung,
            "running": self.running(),
            "elapsed": self.elapsed(),
            "laps": self.laps,
        });
        if self.glyphs {
            out["glyph"] = crate::ui::frame::glyph_fact(&self.face());
        }
        out
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let start = if self.stopwatch {
            json!({})
        } else {
            json!({ "minutes": "f64" })
        };
        let mut out = vec![Verb::new("timer.start", start)];
        out.push(Verb::new(
            "timer.set",
            json!({ "key": "duration", "value": "any" }),
        ));
        if self.stopwatch {
            out.push(Verb::new("timer.lap", json!({})));
        }
        out.push(Verb::new("timer.pause", json!({})));
        out.push(Verb::new("timer.resume", json!({})));
        out.push(Verb::new("timer.check", json!({})));
        out.push(Verb::new("timer.clear", json!({})));
        out.push(Verb::new(
            "timer.mode",
            json!({ "mode": "countdown | stopwatch" }),
        ));
        out
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        self.now = call.now.unwrap_or(self.now);
        match call.verb.as_str() {
            "timer.mode" => match call.arg("mode").as_str() {
                Some(mode @ ("countdown" | "stopwatch")) => {
                    self.stopwatch = mode == "stopwatch";
                    self.wipe();
                    Outcome::ok(json!({ "mode": mode }))
                }
                _ => Outcome::fail("mode must be countdown or stopwatch"),
            },
            "timer.start" => {
                if self.stopwatch {
                    self.wipe();
                    self.origin = Some(self.now);
                    Outcome::ok(json!({ "elapsed": 0 }))
                } else {
                    match minutes(call.arg("minutes")) {
                        Some(m) => {
                            let deadline = self.now + (m * 60000.0) as i64;
                            self.deadline = Some(deadline);
                            self.paused = None;
                            self.rung = false;
                            Outcome::ok(json!({ "deadline": deadline }))
                        }
                        None => Outcome::fail("minutes must be a positive number"),
                    }
                }
            }
            "timer.set" => {
                let key = call.arg("key").as_str().unwrap_or("");
                if key != "duration" {
                    Outcome::fail("no such key")
                } else {
                    let h = call.arg("value")["h"].as_u64().unwrap_or(0);
                    let m = call.arg("value")["m"].as_u64().unwrap_or(0);
                    let total = h * 60 + m;
                    if total == 0 || total > 1440 {
                        Outcome::fail("duration must be 1 to 1440 minutes")
                    } else {
                        self.stopwatch = false;
                        self.wipe();
                        self.deadline = Some(self.now + (total as i64) * 60000);
                        self.rung = false;
                        Outcome::ok(json!({ "deadline": self.deadline }))
                    }
                }
            }
            "timer.pause" => {
                if self.stopwatch {
                    match self.origin {
                        Some(o) => {
                            self.banked += (self.now - o).max(0);
                            self.origin = None;
                            Outcome::ok(json!({ "elapsed": self.elapsed() }))
                        }
                        None => Outcome::fail("not running"),
                    }
                } else if self.deadline.is_some() && !self.rung {
                    self.paused = Some(self.remaining());
                    self.deadline = None;
                    Outcome::ok(json!({ "remaining": self.remaining() }))
                } else {
                    Outcome::fail("not running")
                }
            }
            "timer.resume" => {
                if self.stopwatch {
                    if self.origin.is_some() {
                        Outcome::fail("already running")
                    } else if !self.armed() {
                        Outcome::fail("nothing to resume")
                    } else {
                        self.origin = Some(self.now);
                        Outcome::ok(json!({ "elapsed": self.elapsed() }))
                    }
                } else {
                    match self.paused.take() {
                        Some(left) => {
                            let deadline = self.now + left;
                            self.deadline = Some(deadline);
                            Outcome::ok(json!({ "deadline": deadline }))
                        }
                        None => Outcome::fail("not paused"),
                    }
                }
            }
            "timer.lap" => {
                if !self.stopwatch {
                    Outcome::fail("laps need stopwatch mode")
                } else if self.origin.is_some() {
                    let at = self.elapsed();
                    self.laps.push(at);
                    Outcome::ok(json!({ "lap": self.laps.len(), "elapsed": at }))
                        .emit(Effect::new("sound", cue::payload("blip")))
                } else {
                    Outcome::fail("not running")
                }
            }
            "timer.check" => {
                if self.stopwatch {
                    if self.armed() {
                        Outcome::ok(json!({ "elapsed": self.elapsed(), "running": self.running() }))
                    } else {
                        Outcome::fail("no timer set")
                    }
                } else if let Some(deadline) = self.deadline {
                    if self.now >= deadline && !self.rung {
                        self.rung = true;
                        Outcome::ok(json!({ "remaining": 0, "rung": true })).emit(Effect::new(
                            "notify",
                            json!({ "title": "timer", "body": "time is up" }),
                        ))
                    } else {
                        Outcome::ok(json!({ "remaining": self.remaining(), "rung": self.rung }))
                    }
                } else if let Some(left) = self.paused {
                    Outcome::ok(json!({ "remaining": left, "rung": false }))
                } else {
                    Outcome::fail("no timer set")
                }
            }
            "timer.clear" => {
                self.wipe();
                Outcome::ok(json!({}))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        if self.running() {
            Some(Call::new("timer.check", json!({})))
        } else {
            None
        }
    }
    fn wear(&mut self, world: &Json) {
        self.glyphs = world["shared"]["settings"]["font"] == "mrly";
    }
    fn save(&self) -> Json {
        json!({
            "stopwatch": self.stopwatch,
            "deadline": self.deadline,
            "paused": self.paused,
            "rung": self.rung,
            "origin": self.origin,
            "banked": self.banked,
            "laps": self.laps,
            "now": self.now,
        })
    }
    fn load(&mut self, state: &Json) {
        self.stopwatch = state["stopwatch"].as_bool().unwrap_or(false);
        self.deadline = state["deadline"].as_i64();
        self.paused = state["paused"].as_i64().filter(|&left| left >= 0);
        self.rung = state["rung"].as_bool().unwrap_or(false);
        self.origin = state["origin"].as_i64();
        self.banked = state["banked"].as_i64().unwrap_or(0).max(0);
        self.laps = state["laps"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_i64())
                    .filter(|&v| v >= 0)
                    .collect()
            })
            .unwrap_or_default();
        self.now = state["now"].as_i64().unwrap_or(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_arms_from_now() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        let out = t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(1000),
        );
        assert!(out.ok);
        assert_eq!(out.data["deadline"], json!(61000));
    }
    #[test]
    fn set_accepts_strings_and_fractions() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        let out = t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": "0.5" })).at(0),
        );
        assert_eq!(out.data["deadline"], json!(30000));
    }
    #[test]
    fn set_rejects_garbage() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        for bad in [json!(0), json!(-1), json!("x"), json!(null), json!(9000)] {
            assert!(
                !t.act(&iden, &Call::new("timer.start", json!({ "minutes": bad })))
                    .ok
            );
        }
    }
    #[test]
    fn duration_arms_a_countdown() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        let out = t.act(
            &iden,
            &Call::new(
                "timer.set",
                json!({ "key": "duration", "value": { "h": 0, "m": 1 } }),
            )
            .at(1000),
        );
        assert!(out.ok);
        assert_eq!(out.data["deadline"], json!(61000));
        assert_eq!(t.state(&iden)["mode"], json!("countdown"));
        assert_eq!(t.state(&iden)["armed"], json!(true));
    }
    #[test]
    fn duration_rejects_bad_input() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        for bad in [json!({ "h": 0, "m": 0 }), json!({ "h": 25, "m": 0 })] {
            assert!(
                !t.act(
                    &iden,
                    &Call::new("timer.set", json!({ "key": "duration", "value": bad }))
                )
                .ok
            );
        }
        assert!(
            !t.act(&iden, &Call::new("timer.set", json!({ "key": "nope" })))
                .ok
        );
    }
    #[test]
    fn duration_switches_from_stopwatch() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        t.act(
            &iden,
            &Call::new("timer.mode", json!({ "mode": "stopwatch" })),
        );
        t.act(&iden, &Call::new("timer.start", json!({})).at(0));
        t.act(
            &iden,
            &Call::new(
                "timer.set",
                json!({ "key": "duration", "value": { "h": 0, "m": 2 } }),
            )
            .at(1000),
        );
        let state = t.state(&iden);
        assert_eq!(state["mode"], json!("countdown"));
        assert_eq!(state["armed"], json!(true));
    }
    #[test]
    fn check_rings_once() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(1000),
        );
        let early = t.act(&iden, &Call::new("timer.check", json!({})).at(31000));
        assert!(early.ok);
        assert_eq!(early.data["remaining"], json!(30000));
        assert!(early.effects.is_empty());
        let ring = t.act(&iden, &Call::new("timer.check", json!({})).at(61000));
        assert_eq!(ring.effects.len(), 1);
        assert_eq!(ring.effects[0].kind, "notify");
        assert_eq!(ring.effects[0].data["title"], "timer");
        let after = t.act(&iden, &Call::new("timer.check", json!({})).at(62000));
        assert!(after.effects.is_empty());
        assert_eq!(after.data["rung"], json!(true));
    }
    #[test]
    fn check_without_timer_fails() {
        let iden = Iden::new("aria");
        assert!(
            !Timer::new()
                .act(&iden, &Call::new("timer.check", json!({})))
                .ok
        );
    }
    #[test]
    fn beat_only_while_armed() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        assert!(t.beat().is_none());
        t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(0),
        );
        assert_eq!(
            t.beat().unwrap().to_json(),
            json!({ "verb": "timer.check", "args": {} })
        );
        t.act(&iden, &Call::new("timer.check", json!({})).at(60000));
        assert!(t.beat().is_none());
        t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(60000),
        );
        assert!(t.beat().is_some());
        t.act(&iden, &Call::new("timer.clear", json!({})));
        assert!(t.beat().is_none());
    }
    #[test]
    fn state_publishes_the_numbers() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        assert_eq!(
            t.state(&iden),
            json!({
                "mode": "countdown",
                "armed": false,
                "remaining": 0,
                "rung": false,
                "running": false,
                "elapsed": 0,
                "laps": [],
            })
        );
        t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(1000),
        );
        t.act(&iden, &Call::new("timer.check", json!({})).at(31000));
        let state = t.state(&iden);
        assert_eq!(state["armed"], json!(true));
        assert_eq!(state["remaining"], json!(30000));
        assert_eq!(state["running"], json!(true));
        assert_eq!(state["rung"], json!(false));
        t.act(&iden, &Call::new("timer.check", json!({})).at(61000));
        let state = t.state(&iden);
        assert_eq!(state["remaining"], json!(0));
        assert_eq!(state["rung"], json!(true));
        assert_eq!(state["running"], json!(false));
    }
    #[test]
    fn mode_switches_and_wipes() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(0),
        );
        let out = t.act(
            &iden,
            &Call::new("timer.mode", json!({ "mode": "stopwatch" })),
        );
        assert!(out.ok);
        let state = t.state(&iden);
        assert_eq!(state["mode"], json!("stopwatch"));
        assert_eq!(state["armed"], json!(false));
        assert_eq!(state["remaining"], json!(0));
        assert!(
            t.act(
                &iden,
                &Call::new("timer.mode", json!({ "mode": "countdown" }))
            )
            .ok
        );
        assert_eq!(t.state(&iden)["mode"], json!("countdown"));
        assert!(
            !t.act(&iden, &Call::new("timer.mode", json!({ "mode": "egg" })))
                .ok
        );
        assert!(!t.act(&iden, &Call::new("timer.mode", json!({}))).ok);
    }
    #[test]
    fn countdown_pauses_and_resumes() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        assert!(!t.act(&iden, &Call::new("timer.pause", json!({}))).ok);
        assert!(!t.act(&iden, &Call::new("timer.resume", json!({}))).ok);
        t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(0),
        );
        let out = t.act(&iden, &Call::new("timer.pause", json!({})).at(30000));
        assert!(out.ok);
        assert_eq!(out.data["remaining"], json!(30000));
        assert!(t.beat().is_none());
        let held = t.act(&iden, &Call::new("timer.check", json!({})).at(50000));
        assert_eq!(held.data["remaining"], json!(30000));
        assert!(
            !t.act(&iden, &Call::new("timer.pause", json!({})).at(50000))
                .ok
        );
        let out = t.act(&iden, &Call::new("timer.resume", json!({})).at(60000));
        assert_eq!(out.data["deadline"], json!(90000));
        assert!(t.beat().is_some());
        let ring = t.act(&iden, &Call::new("timer.check", json!({})).at(90000));
        assert_eq!(ring.effects.len(), 1);
    }
    #[test]
    fn stopwatch_runs_pauses_and_resumes() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        t.act(
            &iden,
            &Call::new("timer.mode", json!({ "mode": "stopwatch" })),
        );
        assert!(t.beat().is_none());
        assert!(!t.act(&iden, &Call::new("timer.resume", json!({})).at(0)).ok);
        t.act(&iden, &Call::new("timer.start", json!({})).at(1000));
        assert!(t.beat().is_some());
        let out = t.act(&iden, &Call::new("timer.check", json!({})).at(6000));
        assert_eq!(out.data["elapsed"], json!(5000));
        let out = t.act(&iden, &Call::new("timer.pause", json!({})).at(9000));
        assert_eq!(out.data["elapsed"], json!(8000));
        assert!(t.beat().is_none());
        let held = t.act(&iden, &Call::new("timer.check", json!({})).at(20000));
        assert_eq!(held.data["elapsed"], json!(8000));
        let out = t.act(&iden, &Call::new("timer.resume", json!({})).at(21000));
        assert!(out.ok);
        let out = t.act(&iden, &Call::new("timer.check", json!({})).at(23000));
        assert_eq!(out.data["elapsed"], json!(10000));
        t.act(&iden, &Call::new("timer.start", json!({})).at(30000));
        assert_eq!(t.state(&iden)["elapsed"], json!(0));
    }
    #[test]
    fn laps_collect_and_blip() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        assert!(!t.act(&iden, &Call::new("timer.lap", json!({}))).ok);
        t.act(
            &iden,
            &Call::new("timer.mode", json!({ "mode": "stopwatch" })),
        );
        assert!(!t.act(&iden, &Call::new("timer.lap", json!({})).at(0)).ok);
        t.act(&iden, &Call::new("timer.start", json!({})).at(0));
        let out = t.act(&iden, &Call::new("timer.lap", json!({})).at(3000));
        assert!(out.ok);
        assert_eq!(out.data["lap"], json!(1));
        assert_eq!(out.effects.len(), 1);
        assert_eq!(out.effects[0].kind, "sound");
        t.act(&iden, &Call::new("timer.lap", json!({})).at(7000));
        assert_eq!(t.state(&iden)["laps"], json!([3000, 7000]));
        t.act(&iden, &Call::new("timer.pause", json!({})).at(8000));
        assert!(!t.act(&iden, &Call::new("timer.lap", json!({})).at(9000)).ok);
        t.act(&iden, &Call::new("timer.clear", json!({})));
        assert_eq!(t.state(&iden)["laps"], json!([]));
    }
    #[test]
    fn save_load_roundtrips() {
        let iden = Iden::new("aria");
        let mut a = Timer::new();
        a.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 2 })).at(1000),
        );
        a.act(&iden, &Call::new("timer.check", json!({})).at(31000));
        let mut b = Timer::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden), a.state(&iden));
        assert_eq!(b.beat(), a.beat());
        let mut c = Timer::new();
        c.act(
            &iden,
            &Call::new("timer.mode", json!({ "mode": "stopwatch" })),
        );
        c.act(&iden, &Call::new("timer.start", json!({})).at(0));
        c.act(&iden, &Call::new("timer.lap", json!({})).at(4000));
        c.act(&iden, &Call::new("timer.pause", json!({})).at(6000));
        let mut d = Timer::new();
        d.load(&c.save());
        assert_eq!(d.state(&iden), c.state(&iden));
        assert_eq!(d.beat(), c.beat());
    }
    #[test]
    fn load_survives_garbage() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        t.load(&json!({
            "stopwatch": "yes",
            "deadline": "nope",
            "paused": -5,
            "banked": "soup",
            "laps": [1000, "x", -3, 2000],
        }));
        let state = t.state(&iden);
        assert_eq!(state["mode"], json!("countdown"));
        assert_eq!(state["remaining"], json!(0));
        assert_eq!(state["elapsed"], json!(0));
        t.load(&json!("soup"));
        assert_eq!(t.state(&iden)["armed"], json!(false));
    }
    #[test]
    fn unknown_verb_fails() {
        let iden = Iden::new("aria");
        assert!(
            !Timer::new()
                .act(&iden, &Call::new("timer.fly", json!({})))
                .ok
        );
    }
    #[test]
    fn worn_timer_shows_the_glyph_face() {
        let iden = Iden::new("aria");
        let mut t = Timer::new();
        t.wear(&json!({ "shared": { "settings": { "font": "mrly" } } }));
        t.act(
            &iden,
            &Call::new("timer.start", json!({ "minutes": 1 })).at(1000),
        );
        t.act(&iden, &Call::new("timer.check", json!({})).at(31000));
        assert_eq!(t.state(&iden)["glyph"]["text"], json!("00:30"));
        t.act(
            &iden,
            &Call::new("timer.mode", json!({ "mode": "stopwatch" })),
        );
        assert_eq!(t.state(&iden)["glyph"]["text"], json!("00:00"));
        t.act(&iden, &Call::new("timer.start", json!({})).at(31000));
        t.act(&iden, &Call::new("timer.check", json!({})).at(96000));
        assert_eq!(t.state(&iden)["glyph"]["text"], json!("01:05"));
    }
}
