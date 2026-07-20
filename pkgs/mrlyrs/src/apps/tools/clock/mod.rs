use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub fn new(hour: u8, minute: u8, second: u8) -> Time {
        Time {
            hour: hour % 24,
            minute: minute % 60,
            second: second % 60,
        }
    }
    pub fn from(seconds: u32) -> Time {
        let s = seconds % 86400;
        Time {
            hour: (s / 3600) as u8,
            minute: ((s % 3600) / 60) as u8,
            second: (s % 60) as u8,
        }
    }
    pub fn seconds(&self) -> u32 {
        self.hour as u32 * 3600 + self.minute as u32 * 60 + self.second as u32
    }
    pub fn text(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

pub use crate::core::time::civil;

pub struct Clock {
    now: i64,
    glyphs: bool,
    stage_h: u8,
    stage_m: u8,
}

impl Default for Clock {
    fn default() -> Clock {
        Clock::new()
    }
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            now: 0,
            glyphs: false,
            stage_h: 0,
            stage_m: 0,
        }
    }
}

impl App for Clock {
    fn route(&self) -> &str {
        "clock"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("clock").emoji("🕐").category("tools")
    }
    fn state(&self, _iden: &Iden) -> Json {
        let mut out = json!({
            "now": self.now,
            "work": { "h": self.stage_h, "m": self.stage_m },
            "stage": { "h": self.stage_h, "m": self.stage_m },
        });
        if self.glyphs {
            let face = if self.now == 0 {
                "--:--:--".to_string()
            } else {
                Time::from(((self.now / 1000).rem_euclid(86400)) as u32).text()
            };
            out["glyph"] = crate::ui::frame::glyph_fact(&face);
        }
        out
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("clock.tick", json!({})),
            Verb::new(
                "clock.set",
                json!({ "key": "hour | minute | work", "value": "any" }),
            ),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "clock.tick" => {
                self.now = call.now.unwrap_or(self.now);
                Outcome::ok(json!({ "now": self.now }))
            }
            "clock.set" => {
                let key = call.arg("key").as_str().unwrap_or("");
                let value = call.arg("value");
                match key {
                    "hour" => {
                        let v = value
                            .as_u64()
                            .or_else(|| value.as_str().and_then(|s| s.parse().ok()));
                        match v {
                            Some(v) if (0..=23).contains(&v) => {
                                self.stage_h = v as u8;
                                Outcome::ok(json!({ "key": key, "value": v }))
                            }
                            _ => Outcome::fail("hour is 0 to 23"),
                        }
                    }
                    "minute" => {
                        let v = value
                            .as_u64()
                            .or_else(|| value.as_str().and_then(|s| s.parse().ok()));
                        match v {
                            Some(v) if (0..=59).contains(&v) => {
                                self.stage_m = v as u8;
                                Outcome::ok(json!({ "key": key, "value": v }))
                            }
                            _ => Outcome::fail("minute is 0 to 59"),
                        }
                    }
                    "work" => {
                        if let Some(h) = value["h"].as_u64() {
                            self.stage_h = h.min(23) as u8;
                        }
                        if let Some(m) = value["m"].as_u64() {
                            self.stage_m = m.min(59) as u8;
                        }
                        Outcome::ok(json!({ "h": self.stage_h, "m": self.stage_m }))
                    }
                    _ => Outcome::fail("no such key"),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        Some(Call::new("clock.tick", json!({})))
    }
    fn wear(&mut self, world: &Json) {
        self.glyphs = world["shared"]["settings"]["font"] == "mrly";
    }
    fn save(&self) -> Json {
        json!({ "now": self.now, "stage_h": self.stage_h, "stage_m": self.stage_m })
    }
    fn load(&mut self, state: &Json) {
        self.now = state["now"].as_i64().unwrap_or(0);
        self.stage_h = state["stage_h"].as_u64().unwrap_or(0).min(23) as u8;
        self.stage_m = state["stage_m"].as_u64().unwrap_or(0).min(59) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn formats_padded() {
        assert_eq!(Time::new(9, 5, 3).text(), "09:05:03");
        assert_eq!(Time::new(23, 59, 59).text(), "23:59:59");
    }
    #[test]
    fn from_seconds_splits() {
        let t = Time::from(3661);
        assert_eq!((t.hour, t.minute, t.second), (1, 1, 1));
    }
    #[test]
    fn wraps_a_day() {
        assert_eq!(Time::from(86400).text(), "00:00:00");
        assert_eq!(Time::from(86461).text(), "00:01:01");
    }
    #[test]
    fn roundtrips_seconds() {
        for s in [0u32, 1, 3599, 3600, 43200, 86399] {
            assert_eq!(Time::from(s).seconds(), s);
        }
    }
    #[test]
    fn tick_reads_now_from_the_call() {
        let iden = Iden::new("aria");
        let mut clock = Clock::new();
        let out = clock.act(&iden, &Call::new("clock.tick", json!({})).at(1783600496000));
        assert!(out.ok);
        assert_eq!(out.data["now"], json!(1783600496000i64));
        assert_eq!(clock.state(&iden)["now"], json!(1783600496000i64));
    }
    #[test]
    fn unset_clock_is_honest() {
        let iden = Iden::new("aria");
        assert_eq!(Clock::new().state(&iden)["now"], json!(0));
    }
    #[test]
    fn beat_is_the_tick() {
        assert_eq!(
            Clock::new().beat().unwrap().to_json(),
            json!({ "verb": "clock.tick", "args": {} })
        );
    }
    #[test]
    fn save_load_roundtrips() {
        let iden = Iden::new("aria");
        let mut a = Clock::new();
        a.act(&iden, &Call::new("clock.tick", json!({})).at(1783600496000));
        let mut b = Clock::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden), a.state(&iden));
        let mut c = Clock::new();
        c.load(&json!({}));
        assert_eq!(c.state(&iden)["now"], json!(0));
    }
    #[test]
    fn unknown_verb_fails() {
        let iden = Iden::new("aria");
        assert!(
            !Clock::new()
                .act(&iden, &Call::new("clock.fly", json!({})))
                .ok
        );
    }
    #[test]
    fn worn_clock_shows_the_glyph_face() {
        let iden = Iden::new("aria");
        let mut clock = Clock::new();
        clock.wear(&json!({ "shared": { "settings": { "font": "mrly" } } }));
        assert_eq!(clock.state(&iden)["glyph"]["text"], json!("--:--:--"));
        clock.act(&iden, &Call::new("clock.tick", json!({})).at(45296000));
        assert_eq!(clock.state(&iden)["glyph"]["text"], json!("12:34:56"));
    }
    #[test]
    fn unworn_clock_has_no_glyph() {
        let iden = Iden::new("aria");
        assert!(Clock::new().state(&iden)["glyph"].is_null());
    }
    #[test]
    fn stage_defaults_to_zero() {
        let iden = Iden::new("aria");
        let state = Clock::new().state(&iden);
        assert_eq!(state["work"], json!({ "h": 0, "m": 0 }));
        assert_eq!(state["stage"], json!({ "h": 0, "m": 0 }));
    }
    #[test]
    fn set_stages_hour_and_minute() {
        let iden = Iden::new("aria");
        let mut clock = Clock::new();
        let out = clock.act(
            &iden,
            &Call::new("clock.set", json!({ "key": "hour", "value": 5 })),
        );
        assert!(out.ok);
        assert_eq!(clock.state(&iden)["work"]["h"], json!(5));
        let out = clock.act(
            &iden,
            &Call::new("clock.set", json!({ "key": "minute", "value": 30 })),
        );
        assert!(out.ok);
        assert_eq!(clock.state(&iden)["work"]["m"], json!(30));
        assert!(
            !clock
                .act(
                    &iden,
                    &Call::new("clock.set", json!({ "key": "hour", "value": 24 }))
                )
                .ok
        );
        assert!(
            !clock
                .act(
                    &iden,
                    &Call::new("clock.set", json!({ "key": "minute", "value": 60 }))
                )
                .ok
        );
        assert!(
            !clock
                .act(
                    &iden,
                    &Call::new("clock.set", json!({ "key": "nope", "value": 1 }))
                )
                .ok
        );
    }
    #[test]
    fn set_work_seeds_the_duration() {
        let iden = Iden::new("aria");
        let mut clock = Clock::new();
        let out = clock.act(
            &iden,
            &Call::new(
                "clock.set",
                json!({ "key": "work", "value": { "h": 2, "m": 15 } }),
            ),
        );
        assert!(out.ok);
        assert_eq!(clock.state(&iden)["work"], json!({ "h": 2, "m": 15 }));
    }
    #[test]
    fn stage_survives_save_load() {
        let iden = Iden::new("aria");
        let mut a = Clock::new();
        a.act(
            &iden,
            &Call::new("clock.set", json!({ "key": "hour", "value": 3 })),
        );
        a.act(
            &iden,
            &Call::new("clock.set", json!({ "key": "minute", "value": 45 })),
        );
        let mut b = Clock::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden)["work"], a.state(&iden)["work"]);
    }
}
