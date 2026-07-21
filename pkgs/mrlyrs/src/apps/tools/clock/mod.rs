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
        let mut out = json!({ "now": self.now });
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
        vec![Verb::new("clock.tick", json!({}))]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "clock.tick" => {
                self.now = call.now.unwrap_or(self.now);
                Outcome::ok(json!({ "now": self.now }))
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
        json!({ "now": self.now })
    }
    fn load(&mut self, state: &Json) {
        self.now = state["now"].as_i64().unwrap_or(0);
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
}
