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

#[test]
fn enter_steps_a_ringed_slider_instead_of_jumping_to_its_middle() {
    let mut driver = Driver::scripted(0);
    driver.open("settings");
    let (min, max, step) = driver
        .scene()
        .hits
        .iter()
        .find_map(|hit| match &hit.act {
            Act::Slide {
                call,
                value,
                min,
                max,
                step,
                ..
            } if call.args["key"] == mrlycore::json!("scale") => {
                assert_eq!(*value, 5, "the shipped default is scale 5");
                Some((*min, *max, *step))
            }
            _ => None,
        })
        .expect("settings shows a scale slider");
    for _ in 0..200 {
        let on_scale = matches!(
            driver.ui().ring.as_ref(),
            Some(Act::Slide { call, .. }) if call.args["key"] == mrlycore::json!("scale")
        );
        if on_scale {
            break;
        }
        driver.tab();
    }
    assert!(driver.enter(), "enter found no ring");
    let now = driver.os().peek("settings", None).expect("settings").state["scale"]
        .as_i64()
        .expect("an integer scale");
    assert_ne!(now, min + (max - min) / 2, "enter jumped to the middle");
    assert_eq!(now, 5 + step, "enter did not step the slider");
}

#[test]
fn the_ring_never_paints_through_an_overlay() {
    let shade = |ring: bool| {
        let mut driver = Driver::scripted(0);
        driver.open("settings");
        if ring {
            driver.tab();
        }
        let spot = driver
            .scene()
            .hits
            .iter()
            .find(|hit| matches!(&hit.act, Act::Menu { .. }))
            .map(|hit| (hit.x + hit.w / 2, hit.y + hit.h / 2))
            .expect("settings shows a select");
        driver.tap_at(spot.0, spot.1);
        assert_eq!(ring, driver.ui().ring.is_some(), "the ring state moved");
        driver.frame_fnv()
    };
    assert_eq!(
        shade(true),
        shade(false),
        "the ring painted a rim through the veil"
    );
}
