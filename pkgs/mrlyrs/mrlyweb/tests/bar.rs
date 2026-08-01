use mrlyweb::drive::{Act, Driver};

fn menu() -> Driver {
    let mut driver = Driver::scripted(0);
    driver.open("menu");
    driver
}

fn line(driver: &Driver) -> String {
    driver
        .ui()
        .bar
        .as_ref()
        .map(|bar| bar.line.clone())
        .unwrap_or_default()
}

#[test]
fn the_bar_is_closed_until_asked() {
    let mut driver = menu();
    let cold = driver.frame_fnv();
    assert!(!driver.asking());
    driver.ask();
    assert!(driver.asking());
    assert_ne!(driver.frame_fnv(), cold, "the bar drew nothing");
    driver.escape();
    assert!(!driver.asking());
    assert_eq!(driver.frame_fnv(), cold, "the bar left a mark");
}

#[test]
fn hints_narrow_as_the_verb_is_typed() {
    let mut driver = menu();
    driver.ask();
    let wide = driver.ui().bar.as_ref().unwrap().hints.clone();
    assert!(!wide.is_empty());
    for c in "snake.t".chars() {
        driver.ask_type(c);
    }
    let hints = driver.ui().bar.as_ref().unwrap().hints.clone();
    assert!(!hints.is_empty(), "typing a real verb killed every hint");
    assert!(hints.iter().all(|h| h.starts_with("snake.t")), "{hints:?}");
    assert!(hints.len() < wide.len());
}

#[test]
fn a_hint_fills_the_line_by_tap() {
    let mut driver = menu();
    driver.ask();
    for c in "nav.".chars() {
        driver.ask_type(c);
    }
    let fill = driver
        .scene()
        .hits
        .iter()
        .find_map(|hit| match &hit.act {
            Act::Fill { text } => Some((hit.x + 1, hit.y + 1, text.clone())),
            _ => None,
        })
        .expect("a hint to tap");
    driver.tap_at(fill.0, fill.1);
    assert_eq!(line(&driver), format!("{} ", fill.2));
}

#[test]
fn a_typed_command_is_a_verb_call() {
    let mut driver = menu();
    driver.ask();
    for c in "nav.open {\"app\":\"snake\"}".chars() {
        driver.ask_type(c);
    }
    driver.ask_run().unwrap();
    assert_eq!(driver.route(), "snake");
    assert!(!driver.asking(), "the bar stayed open");
}

#[test]
fn a_bad_command_says_so_and_fires_nothing() {
    let mut driver = menu();
    driver.ask();
    for c in "nope.wat".chars() {
        driver.ask_type(c);
    }
    assert!(driver.ask_run().is_err());
    assert_eq!(driver.route(), "menu");
    driver.ask();
    for c in "nav.open {oops".chars() {
        driver.ask_type(c);
    }
    assert!(driver.ask_run().is_err());
    assert_eq!(driver.route(), "menu");
}
