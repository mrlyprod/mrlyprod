use mrlyweb::drive::{Act, Driver};

fn menu() -> Driver {
    let mut driver = Driver::scripted(0);
    driver.open("menu");
    driver
}

fn taps(driver: &Driver) -> Vec<(usize, usize)> {
    driver
        .scene()
        .hits
        .iter()
        .filter(|hit| matches!(hit.act, Act::Tap { .. }))
        .map(|hit| (hit.x + hit.w / 2, hit.y + hit.h / 2))
        .collect()
}

#[test]
fn the_hovered_hit_decorates_and_nothing_else_does() {
    let mut driver = menu();
    let cold = driver.frame_fnv();
    let spots = taps(&driver);
    let first = *spots.first().expect("a menu tap");
    driver.hover(Some(first));
    assert!(driver.dirty());
    let lit = driver.frame_fnv();
    assert_ne!(cold, lit, "hover painted nothing");
    driver.hover(None);
    assert!(driver.dirty());
    assert_eq!(driver.frame_fnv(), cold, "hover left a mark behind");
}

#[test]
fn moving_inside_one_hit_never_redraws() {
    let mut driver = menu();
    let hit = driver
        .scene()
        .hits
        .iter()
        .find(|hit| matches!(hit.act, Act::Tap { .. }))
        .cloned()
        .expect("a menu tap");
    driver.hover(Some((hit.x + 1, hit.y + 1)));
    assert!(driver.dirty());
    let lit = driver.frame_fnv();
    driver.hover(Some((hit.x + hit.w - 1, hit.y + hit.h - 1)));
    assert!(!driver.dirty(), "the same hit redrew");
    assert_eq!(driver.frame_fnv(), lit);
    driver.hover(Some((hit.x + hit.w - 1, hit.y + hit.h - 1)));
    assert!(!driver.dirty());
}

#[test]
fn every_tap_lights_unless_it_already_reads_as_chosen() {
    let mut seen = 0;
    for app in ["menu", "snake", "twenty48", "mandelbrot"] {
        let mut driver = Driver::scripted(0);
        driver.open(app);
        let cold = driver.frame_fnv();
        let spots = taps(&driver);
        let mut cool = 0;
        for spot in &spots {
            driver.hover(Some(*spot));
            if driver.frame_fnv() == cold {
                cool += 1;
            }
        }
        assert!(cool <= 1, "{app} left {cool} taps cold");
        assert!(spots.is_empty() || cool < spots.len(), "{app} lit nothing");
        seen += spots.len();
        driver.hover(None);
        assert_eq!(driver.frame_fnv(), cold, "{app} kept a hover mark");
    }
    assert!(seen > 20, "only {seen} taps swept");
}
