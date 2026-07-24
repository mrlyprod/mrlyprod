use mrlycore::time::civil;
use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

pub const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub const DAYS: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];

pub fn leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub fn weekday(year: i32, month: u32, day: u32) -> u32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    let w = (y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + day as i32) % 7;
    ((w + 6) % 7) as u32
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub day: u32,
    pub faded: bool,
}

pub struct Calendar {
    year: i32,
    month: u32,
    now: i64,
    picked_year: i32,
    picked_month: u32,
    picked_day: u32,
}

impl Default for Calendar {
    fn default() -> Calendar {
        Calendar::new()
    }
}

impl Calendar {
    pub fn new() -> Calendar {
        Calendar {
            year: 1970,
            month: 1,
            now: 0,
            picked_year: 1970,
            picked_month: 1,
            picked_day: 1,
        }
    }
    pub fn title(&self) -> String {
        format!("{} {}", MONTHS[(self.month - 1) as usize], self.year)
    }
    fn date(&self) -> (i64, u32, u32) {
        civil(self.now.div_euclid(86_400_000))
    }
    fn today(&self) -> Json {
        let (y, m, d) = self.date();
        if y == self.year as i64 && m == self.month {
            json!(d)
        } else {
            Json::Null
        }
    }
    pub fn grid(&self) -> Vec<Cell> {
        let start = weekday(self.year, self.month, 1) as i32;
        let count = days(self.year, self.month);
        let (py, pm) = if self.month == 1 {
            (self.year - 1, 12)
        } else {
            (self.year, self.month - 1)
        };
        let prev = days(py, pm) as i32;
        let mut cells = Vec::new();
        let mut i = start - 1;
        while i >= 0 {
            cells.push(Cell {
                day: (prev - i) as u32,
                faded: true,
            });
            i -= 1;
        }
        for d in 1..=count {
            cells.push(Cell {
                day: d,
                faded: false,
            });
        }
        let mut next = 1;
        while cells.len() % 7 != 0 {
            cells.push(Cell {
                day: next,
                faded: true,
            });
            next += 1;
        }
        cells
    }
    fn weeks(&self) -> Json {
        let rows: Vec<Json> = self
            .grid()
            .chunks(7)
            .map(|week| {
                Json::Array(
                    week.iter()
                        .map(|c| json!({ "day": c.day, "faded": c.faded }))
                        .collect(),
                )
            })
            .collect();
        Json::Array(rows)
    }
}

impl App for Calendar {
    fn route(&self) -> &str {
        "calendar"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("calendar").emoji("📅").category("tools")
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "year": self.year,
            "month": self.month,
            "title": self.title(),
            "days": DAYS,
            "weeks": self.weeks(),
            "today": self.today(),
            "picked": { "year": self.picked_year, "month": self.picked_month, "day": self.picked_day },
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("calendar.flip", json!({ "n": "int" })),
            Verb::new(
                "calendar.goto",
                json!({ "year": "int 1..9999", "month": "int 1..12" }),
            ),
            Verb::new("calendar.today", json!({})),
            Verb::new("calendar.pick", json!({ "day": "int" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        self.now = call.now.unwrap_or(self.now);
        match call.verb.as_str() {
            "calendar.flip" => {
                let n = if call.arg("n").is_null() {
                    1
                } else {
                    match call.arg("n").as_i64() {
                        Some(n) => n,
                        None => return Outcome::fail("n must be an integer"),
                    }
                };
                let page = self.year as i64 * 12 + self.month as i64 - 1;
                let Some(target) = page.checked_add(n) else {
                    return Outcome::fail("too far");
                };
                if !(12..=119_999).contains(&target) {
                    return Outcome::fail("too far");
                }
                self.year = (target / 12) as i32;
                self.month = (target % 12 + 1) as u32;
                Outcome::ok(json!({ "year": self.year, "month": self.month }))
            }
            "calendar.goto" => {
                let year = call.arg("year").as_i64().or_else(|| {
                    call.arg("year")
                        .as_str()
                        .and_then(|s| s.trim().parse::<i64>().ok())
                });
                let Some(year) = year else {
                    return Outcome::fail("year must be an integer");
                };
                if !(1..=9999).contains(&year) {
                    return Outcome::fail("year out of range");
                }
                let month = call.arg("month").as_u64().or_else(|| {
                    call.arg("month")
                        .as_str()
                        .and_then(|s| MONTHS.iter().position(|m| *m == s))
                        .map(|i| i as u64 + 1)
                });
                let Some(month) = month else {
                    return Outcome::fail("month must be an integer or name");
                };
                if !(1..=12).contains(&month) {
                    return Outcome::fail("month out of range");
                }
                self.year = year as i32;
                self.month = month as u32;
                Outcome::ok(json!({ "year": self.year, "month": self.month }))
            }
            "calendar.today" => {
                let (y, m, _) = self.date();
                self.year = y as i32;
                self.month = m;
                Outcome::ok(json!({ "year": self.year, "month": self.month }))
            }
            "calendar.pick" => {
                let Some(day) = call.arg("day").as_u64() else {
                    return Outcome::fail("day must be an integer");
                };
                if !(1..=days(self.year, self.month) as u64).contains(&day) {
                    return Outcome::fail("no such day");
                }
                self.picked_year = self.year;
                self.picked_month = self.month;
                self.picked_day = day as u32;
                Outcome::ok(json!({
                    "year": self.picked_year,
                    "month": self.picked_month,
                    "day": self.picked_day,
                }))
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({ "year": self.year, "month": self.month, "now": self.now })
    }
    fn load(&mut self, state: &Json) {
        if let Some(year) = state["year"].as_i64() {
            if (1..=9999).contains(&year) {
                self.year = year as i32;
            }
        }
        if let Some(month) = state["month"].as_u64() {
            if (1..=12).contains(&month) {
                self.month = month as u32;
            }
        }
        self.now = state["now"].as_i64().unwrap_or(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::testkit::{iden, send};

    const JULY: i64 = 1783600496000;

    #[test]
    fn leap_years() {
        assert!(leap(2000));
        assert!(leap(2024));
        assert!(!leap(1900));
        assert!(!leap(2023));
    }
    #[test]
    fn month_lengths() {
        assert_eq!(days(2024, 2), 29);
        assert_eq!(days(2023, 2), 28);
        assert_eq!(days(2024, 4), 30);
        assert_eq!(days(2024, 12), 31);
    }
    #[test]
    fn weekday_monday_first() {
        assert_eq!(weekday(2024, 1, 1), 0);
        assert_eq!(weekday(2025, 6, 21), 5);
    }
    #[test]
    fn grid_is_week_aligned() {
        let mut c = Calendar::new();
        send(&mut c, "calendar.goto", json!({ "year": 2025, "month": 6 }));
        let g = c.grid();
        assert_eq!(g.len() % 7, 0);
        let active: Vec<u32> = g.iter().filter(|c| !c.faded).map(|c| c.day).collect();
        assert_eq!(active.len(), 30);
        assert_eq!(active.first(), Some(&1));
        assert_eq!(active.last(), Some(&30));
    }
    #[test]
    fn grid_starts_on_correct_weekday() {
        let mut c = Calendar::new();
        send(&mut c, "calendar.goto", json!({ "year": 2024, "month": 1 }));
        assert_eq!(
            c.grid()[0],
            Cell {
                day: 1,
                faded: false
            }
        );
    }
    #[test]
    fn flip_wraps_years() {
        let mut c = Calendar::new();
        send(&mut c, "calendar.goto", json!({ "year": 2025, "month": 1 }));
        send(&mut c, "calendar.flip", json!({ "n": -1 }));
        assert_eq!(c.state(&iden())["title"], json!("December 2024"));
        send(&mut c, "calendar.flip", json!({ "n": 13 }));
        assert_eq!(c.state(&iden())["title"], json!("January 2026"));
    }
    #[test]
    fn flip_defaults_to_one_page() {
        let mut c = Calendar::new();
        let out = send(&mut c, "calendar.flip", json!({}));
        assert!(out.ok);
        assert_eq!(c.state(&iden())["title"], json!("February 1970"));
        assert!(!send(&mut c, "calendar.flip", json!({ "n": "soup" })).ok);
        assert!(!send(&mut c, "calendar.flip", json!({ "n": 999999 })).ok);
    }
    #[test]
    fn goto_validates_honestly() {
        let mut c = Calendar::new();
        assert!(!send(&mut c, "calendar.goto", json!({ "year": 0, "month": 1 })).ok);
        assert!(
            !send(
                &mut c,
                "calendar.goto",
                json!({ "year": 2026, "month": 13 })
            )
            .ok
        );
        assert!(!send(&mut c, "calendar.goto", json!({ "year": 2026 })).ok);
        assert_eq!(c.state(&iden())["title"], json!("January 1970"));
    }
    #[test]
    fn goto_takes_names_and_strings() {
        let mut c = Calendar::new();
        assert!(
            send(
                &mut c,
                "calendar.goto",
                json!({ "year": 2026, "month": "March" })
            )
            .ok
        );
        assert_eq!(c.state(&iden())["title"], json!("March 2026"));
        assert!(
            send(
                &mut c,
                "calendar.goto",
                json!({ "year": "2031", "month": 2 })
            )
            .ok
        );
        assert_eq!(c.state(&iden())["title"], json!("February 2031"));
        assert!(
            !send(
                &mut c,
                "calendar.goto",
                json!({ "year": 2026, "month": "Smarch" })
            )
            .ok
        );
        assert!(
            !send(
                &mut c,
                "calendar.goto",
                json!({ "year": "soon", "month": 1 })
            )
            .ok
        );
    }
    #[test]
    fn today_reads_the_stamp() {
        let mut c = Calendar::new();
        let out = c.act(&iden(), &Call::new("calendar.today", json!({})).at(JULY));
        assert!(out.ok);
        let state = c.state(&iden());
        assert_eq!(state["title"], json!("July 2026"));
        assert_eq!(state["today"], json!(9));
    }
    #[test]
    fn unstamped_today_is_honest() {
        let mut c = Calendar::new();
        send(&mut c, "calendar.flip", json!({ "n": 700 }));
        send(&mut c, "calendar.today", json!({}));
        assert_eq!(c.state(&iden())["title"], json!("January 1970"));
        assert_eq!(c.state(&iden())["today"], json!(1));
    }
    #[test]
    fn today_fact_clears_off_month() {
        let mut c = Calendar::new();
        c.act(&iden(), &Call::new("calendar.today", json!({})).at(JULY));
        send(&mut c, "calendar.flip", json!({}));
        assert_eq!(c.state(&iden())["today"], Json::Null);
    }
    #[test]
    fn state_carries_the_weeks() {
        let mut c = Calendar::new();
        c.act(&iden(), &Call::new("calendar.today", json!({})).at(JULY));
        let state = c.state(&iden());
        assert_eq!(state["days"].as_array().unwrap().len(), 7);
        let weeks = state["weeks"].as_array().unwrap();
        assert!(weeks.iter().all(|w| w.as_array().unwrap().len() == 7));
        assert_eq!(weeks[0][0], json!({ "day": 29, "faded": true }));
        assert_eq!(weeks[0][2], json!({ "day": 1, "faded": false }));
    }
    #[test]
    fn save_load_roundtrips() {
        let mut a = Calendar::new();
        a.act(&iden(), &Call::new("calendar.today", json!({})).at(JULY));
        send(&mut a, "calendar.flip", json!({ "n": -3 }));
        let mut b = Calendar::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        let mut c = Calendar::new();
        c.load(&json!({ "year": "soup", "month": 99 }));
        assert_eq!(c.state(&iden())["title"], json!("January 1970"));
    }
    #[test]
    fn picked_defaults_to_epoch() {
        let c = Calendar::new();
        assert_eq!(
            c.state(&iden())["picked"],
            json!({ "year": 1970, "month": 1, "day": 1 })
        );
    }
    #[test]
    fn pick_selects_a_day() {
        let mut c = Calendar::new();
        send(&mut c, "calendar.goto", json!({ "year": 2025, "month": 6 }));
        assert!(send(&mut c, "calendar.pick", json!({ "day": 15 })).ok);
        assert_eq!(
            c.state(&iden())["picked"],
            json!({ "year": 2025, "month": 6, "day": 15 })
        );
        assert!(!send(&mut c, "calendar.pick", json!({ "day": 31 })).ok);
        assert!(!send(&mut c, "calendar.pick", json!({ "day": 0 })).ok);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let c = Calendar::new();
        let names: Vec<String> = c.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "calendar.flip",
                "calendar.goto",
                "calendar.today",
                "calendar.pick",
            ]
        );
    }
    #[test]
    fn unknown_verb_fails() {
        assert!(!send(&mut Calendar::new(), "calendar.burn", json!({})).ok);
    }
}
