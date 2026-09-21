use mrlydemo::radix::*;
use mrlyrs::core::error::parse;
use mrlyrs::core::Json;

fn preset(name: &str) -> Json {
    let menu = parse(&radix_menu()).unwrap();
    menu["presets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|card| card["name"] == name)
        .unwrap()
        .clone()
}

fn read(card: &Json, level: usize) -> Json {
    parse(
        &radix_read(
            card["ring"].as_str().unwrap(),
            card["a"].as_i64().unwrap() as i32,
            card["c"].as_i64().unwrap() as i32,
            card["digits"].as_str().unwrap(),
            card["twists"].as_str().unwrap(),
            level,
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn the_koch_preset_places_sixteen_points_at_level_two() {
    let koch = preset("koch");
    assert_eq!(koch["digits"], "0:0_1:0_2:1_2:0");
    assert_eq!(koch["twists"], "0_1_5_0");
    let read = read(&koch, 2);
    assert_eq!(read["ring"], "eisenstein");
    assert_eq!(read["q"].as_u64().unwrap(), 9);
    assert_eq!(read["size"].as_u64().unwrap(), 4);
    assert_eq!(read["fill"], "16");
    assert_eq!(read["distinct"].as_u64().unwrap(), 16);
    assert_eq!(read["glued"], false);
    assert_eq!(read["canonical"], false);
    assert_eq!(read["cap"].as_u64().unwrap(), 8);
    assert_eq!(
        format!("{:.6}", read["dimension"].as_f64().unwrap()),
        "1.261860"
    );
    let points = radix_points("eisenstein", 3, 0, "0:0_1:0_2:1_2:0", "0_1_5_0", 2).unwrap();
    assert_eq!(points.len(), 32);
    assert_eq!((points[0], points[1]), (0.0, 0.0));
}

#[test]
fn the_twisted_two_digits_glue_four_words_onto_three_points() {
    let read = parse(&radix_read("gaussian", 2, 0, "0:0_1:0", "0_2", 2).unwrap()).unwrap();
    assert_eq!(read["fill"], "4");
    assert_eq!(read["distinct"].as_u64().unwrap(), 3);
    assert_eq!(read["glued"], true);
    assert_eq!(read["code"], "3");
    assert_eq!(read["canonical"], true);
    let plain = parse(&radix_read("gaussian", 2, 0, "0:0_1:0", "0_0", 2).unwrap()).unwrap();
    assert_eq!(plain["distinct"].as_u64().unwrap(), 4);
    assert_eq!(plain["glued"], false);
}

#[test]
fn the_carpet_preset_fills_sixty_four_words_at_level_two() {
    let carpet = preset("carpet");
    assert_eq!(carpet["label"], "the carpet, box digits");
    let read = read(&carpet, 2);
    assert_eq!(read["ring"], "gaussian");
    assert_eq!(read["q"].as_u64().unwrap(), 9);
    assert_eq!(read["size"].as_u64().unwrap(), 8);
    assert_eq!(read["fill"], "64");
    assert_eq!(read["distinct"].as_u64().unwrap(), 64);
    assert_eq!(read["canonical"], false);
    assert_eq!(
        format!("{:.6}", read["dimension"].as_f64().unwrap()),
        "1.892789"
    );
    assert_eq!(radix_cap(carpet["digits"].as_str().unwrap()).unwrap(), 5);
}

#[test]
fn a_level_over_the_cap_and_a_broken_dial_are_refused() {
    assert!(radix_read("gaussian", 3, 0, "0:0", "0", 17).is_err());
    assert!(radix_read("gaussian", 1, 0, "0:0", "0", 1).is_err());
    assert!(radix_read("mrly", 2, 0, "0:0", "0", 1).is_err());
    assert!(radix_read("gaussian", 2, 0, "0:0_1:0", "0", 1).is_err());
    assert!(radix_read("gaussian", 2, 0, "", "", 1).is_err());
    assert!(radix_points("eisenstein", 2, 0, "0:0_1:0", "0_9", 1).is_err());
}

#[test]
fn a_congruent_digit_pair_is_refused_without_a_panic() {
    let fault = radix_read("eisenstein", 3, 0, "0:0_2:0_-1:0", "0_0_0", 1).unwrap_err();
    assert!(
        format!("{fault:?}").contains("congruent modulo the base"),
        "{fault:?}"
    );
    assert!(radix_points("eisenstein", 3, 0, "0:0_2:0_-1:0", "0_0_0", 1).is_err());
    assert!(radix_read("eisenstein", 3, 0, "0:0_2:0_2:1", "0_0_0", 1).is_ok());
}
