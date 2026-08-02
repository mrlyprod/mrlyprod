use mrlyweb::drive::Driver;
use std::time::Instant;

const ROUNDS: usize = 40;
const HOVER_MS: f64 = 40.0;
const WHEEL_MS: f64 = 40.0;

fn per(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0 / ROUNDS as f64
}

fn hover_cost(app: &str, spots: &[(usize, usize)]) -> f64 {
    let mut driver = Driver::wall();
    driver.open(app);
    driver.hover(Some(spots[0]));
    let start = Instant::now();
    for i in 0..ROUNDS {
        driver.hover(Some(spots[i % spots.len()]));
    }
    per(start)
}

#[test]
fn hover_stays_inside_its_budget() {
    for (app, spots) in [
        ("menu", [(160, 200), (60, 200), (260, 260), (160, 320)]),
        ("settings", [(160, 120), (160, 200), (60, 260), (260, 320)]),
        ("twenty48", [(160, 400), (60, 400), (260, 400), (160, 380)]),
    ] {
        let each = hover_cost(app, &spots);
        assert!(
            each < HOVER_MS,
            "{app} hover costs {each:.2}ms, over the {HOVER_MS}ms budget"
        );
    }
}

#[test]
fn a_wheel_tick_stays_inside_its_budget() {
    let mut driver = Driver::wall();
    driver.open("menu");
    let start = Instant::now();
    for i in 0..ROUNDS {
        driver.wheel(if i % 2 == 0 { -20.0 } else { 20.0 }, None);
    }
    let each = per(start);
    assert!(
        each < WHEEL_MS,
        "a wheel tick costs {each:.2}ms, over the {WHEEL_MS}ms budget"
    );
}

#[test]
fn a_still_screen_renders_nothing() {
    let mut driver = Driver::wall();
    driver.open("menu");
    assert!(driver.dirty(), "opening an app must draw");
    assert!(!driver.dirty(), "a settled screen asked for a second frame");
    driver.hover(None);
    assert!(!driver.dirty(), "hovering nothing redrew");
}
