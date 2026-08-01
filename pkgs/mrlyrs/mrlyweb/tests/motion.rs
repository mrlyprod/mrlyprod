use mrlyui::tokens::{eased, FULL, PACE};
use mrlyweb::drive::Driver;

fn wall() -> Driver {
    let mut driver = Driver::wall();
    driver.open("menu");
    driver
}

#[test]
fn a_pace_of_zero_lands_at_once() {
    for out in [false, true] {
        assert_eq!(eased(0, 0, 0, out), FULL);
    }
}

#[test]
fn the_curve_starts_still_and_ends_whole() {
    assert_eq!(eased(0, 0, PACE, true), 0);
    assert_eq!(eased(0, PACE, PACE, true), FULL);
    assert_eq!(eased(0, PACE * 9, PACE, true), FULL);
    let half = eased(0, PACE / 2, PACE, true);
    assert!(half > FULL / 2, "ease-out is ahead at the halfway mark");
    assert!(eased(0, PACE / 2, PACE, false) < FULL / 2, "ease-in lags");
}

#[test]
fn a_scripted_driver_never_moves() {
    let mut driver = Driver::scripted(0);
    driver.open("menu");
    driver.focus("menu.search").unwrap();
    assert!(!driver.moving(), "a screenplay saw motion");
    assert!(driver
        .scene()
        .hits
        .iter()
        .any(|hit| matches!(hit.act, mrlyweb::drive::Act::Cap(_))));
}

#[test]
fn the_dock_rises_then_goes_quiet() {
    let mut driver = wall();
    driver.pace(PACE);
    driver.focus("menu.search").unwrap();
    assert!(driver.moving(), "the keyboard docked without moving");
    let mut frames = 0;
    while driver.animate() {
        frames += 1;
        assert!(frames < 10_000, "the dock never settled");
    }
    assert!(frames > 0);
    assert!(!driver.moving());
    let caps = driver
        .scene()
        .hits
        .iter()
        .filter(|hit| matches!(hit.act, mrlyweb::drive::Act::Cap(_)))
        .count();
    assert!(caps > 0, "the settled dock has no caps");
}

#[test]
fn leaving_keeps_the_board_until_it_is_gone() {
    let mut driver = wall();
    driver.pace(PACE);
    driver.focus("menu.search").unwrap();
    while driver.animate() {}
    driver.escape();
    assert!(driver.moving(), "the dock vanished instead of leaving");
    while driver.animate() {}
    assert!(!driver.moving());
    assert!(!driver
        .scene()
        .hits
        .iter()
        .any(|hit| matches!(hit.act, mrlyweb::drive::Act::Cap(_))));
}
