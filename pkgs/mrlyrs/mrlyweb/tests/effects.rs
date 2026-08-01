use mrlycore::json;
use mrlyweb::drive::Driver;

#[test]
fn sound_effects_accumulate_until_drained() {
    let mut driver = Driver::scripted(0);
    driver.open("piano");
    driver.act("piano.press", json!({ "midi": 43 }));
    let effects = driver.drain_effects();
    assert!(effects
        .iter()
        .any(|e| e.kind == "sound" && e.data["op"].as_str() == Some("start")));
    assert!(driver.drain_effects().is_empty());
    driver.act("piano.lift", json!({ "midi": 43 }));
    let effects = driver.drain_effects();
    assert!(effects
        .iter()
        .any(|e| e.kind == "sound" && e.data["op"].as_str() == Some("stop")));
}
