use mrlyweb::drive::{Act, Driver};

fn menu() -> Driver {
    let mut driver = Driver::scripted(0);
    driver.open("menu");
    driver
}

#[test]
fn the_ring_is_invisible_until_tab() {
    let mut driver = menu();
    let cold = driver.frame_fnv();
    assert!(!driver.ringed());
    driver.tab();
    assert!(driver.ringed());
    assert_ne!(driver.frame_fnv(), cold, "tab drew nothing");
    driver.escape();
    assert!(!driver.ringed());
    assert_eq!(driver.frame_fnv(), cold, "the ring left a mark");
}

#[test]
fn tab_walks_every_stop_and_comes_home() {
    let mut driver = menu();
    let stops = driver
        .scene()
        .hits
        .iter()
        .filter(|hit| {
            matches!(
                hit.act,
                Act::Tap { .. } | Act::Edit { .. } | Act::Menu { .. } | Act::Slide { .. }
            )
        })
        .count();
    assert!(stops > 3);
    driver.tab();
    let first = driver.frame_fnv();
    for _ in 1..stops {
        driver.tab();
        assert_ne!(driver.frame_fnv(), first, "tab stalled");
    }
    driver.tab();
    assert_eq!(driver.frame_fnv(), first, "tab never wrapped");
}

#[test]
fn arrows_walk_the_ring_and_enter_fires() {
    let mut driver = menu();
    assert!(!driver.walk("down"), "arrows moved without a ring");
    driver.tab();
    let start = driver.frame_fnv();
    assert!(driver.walk("down"));
    assert_ne!(driver.frame_fnv(), start, "down went nowhere");
    assert!(driver.walk("up"));
    assert_eq!(driver.frame_fnv(), start, "up did not come back");
    for _ in 0..200 {
        let opens = matches!(
            driver.ui().ring.as_ref(),
            Some(Act::Tap { call }) if call.verb == "nav.open"
        );
        if opens {
            break;
        }
        driver.tab();
    }
    assert!(driver.enter(), "enter found no ring");
    assert_ne!(driver.route(), "menu", "enter never opened the app");
    assert!(!driver.ringed(), "the ring survived a route change");
}
