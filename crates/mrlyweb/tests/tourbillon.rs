use mrlycore::json::parse;
use mrlyweb::tourbillon::*;

fn read(
    top: usize,
    size: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    blend: &str,
) -> mrlycore::Json {
    let field = tourbillon(
        top, size, schedule, increment, set, weights, "cells", blend, 1,
    )
    .unwrap();
    parse(
        &tourbillon_stats(
            &field, size, top, schedule, increment, set, weights, blend, 1,
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn the_unspun_centre_is_the_layers_three_mod_four() {
    let stack = read(55, 512, "unspun", 0.0, "odd", "plain", "mean");
    assert_eq!(stack["layers"], 28);
    assert_eq!(stack["centre"].as_f64().unwrap(), 14.0 / 28.0);
    let counted = read(55, 512, "unspun", 0.0, "odd", "plain", "sum");
    assert_eq!(counted["centre"].as_f64().unwrap(), 14.0);
}

#[test]
fn the_unspun_stack_tops_out_at_eighteen_of_the_twenty_eight() {
    let counted = read(55, 512, "unspun", 0.0, "odd", "plain", "sum");
    assert_eq!(counted["high"].as_f64().unwrap(), 18.0);
    assert_eq!(counted["low"].as_f64().unwrap(), 0.0);
    let shared = read(55, 512, "unspun", 0.0, "odd", "plain", "mean");
    assert_eq!(
        format!("{:.6}", shared["high"].as_f64().unwrap()),
        format!("{:.6}", 18.0 / 28.0)
    );
}

#[test]
fn no_turn_of_the_layers_moves_the_centre() {
    for step in 0..8 {
        let spun = read(55, 64, "degrees", f64::from(step), "odd", "plain", "mean");
        assert_eq!(spun["centre"].as_f64().unwrap(), 14.0 / 28.0);
    }
    for schedule in ["golden", "primes", "random", "gaussian"] {
        let spun = read(55, 64, schedule, 0.0, "odd", "plain", "mean");
        assert_eq!(spun["centre"].as_f64().unwrap(), 14.0 / 28.0);
    }
}

#[test]
fn the_parity_blend_folds_the_centre_count_to_its_parity() {
    let folded = read(55, 64, "unspun", 0.0, "odd", "plain", "parity");
    assert_eq!(folded["centre"].as_f64().unwrap(), 0.0);
    assert_eq!(folded["weighted"], false);
    let odd = read(51, 64, "unspun", 0.0, "odd", "plain", "sum");
    assert_eq!(odd["centre"].as_f64().unwrap(), 13.0);
    let flipped = read(51, 64, "unspun", 0.0, "odd", "plain", "parity");
    assert_eq!(flipped["centre"].as_f64().unwrap(), 1.0);
}

#[test]
fn the_primes_only_stack_drops_the_even_prime() {
    let stack = read(199, 64, "unspun", 0.0, "primes", "plain", "mean");
    assert_eq!(stack["layers"], 45);
    let sets: Vec<String> = ["odd", "primes", "squarefree", "prime powers"]
        .iter()
        .map(|set| read(55, 64, "unspun", 0.0, set, "plain", "mean")["layers"].to_string())
        .collect();
    assert_eq!(sets.join(","), "28,15,23,19");
}

#[test]
fn the_prime_schedule_turns_layer_k_by_the_k_th_prime() {
    let stack = read(55, 64, "primes", 0.0, "odd", "plain", "mean");
    let angles: Vec<String> = stack["angles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| format!("{:.1}", value.as_f64().unwrap()))
        .collect();
    assert_eq!(angles.join(" "), "0.0 2.0 3.0 5.0 7.0 11.0 13.0 17.0");
    let scales: Vec<String> = stack["scales"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.to_string())
        .collect();
    assert_eq!(scales.join(" "), "1 3 5 7 9 11 13 15");
}

#[test]
fn the_caps_and_the_names_are_held() {
    assert!(tourbillon(54, 64, "unspun", 0.0, "odd", "plain", "cells", "mean", 1).is_err());
    assert!(tourbillon(201, 64, "unspun", 0.0, "odd", "plain", "cells", "mean", 1).is_err());
    assert!(tourbillon(55, 4096, "unspun", 0.0, "odd", "plain", "cells", "mean", 1).is_err());
    assert!(tourbillon(55, 64, "spiral", 0.0, "odd", "plain", "cells", "mean", 1).is_err());
    assert!(tourbillon(55, 64, "unspun", 0.0, "even", "plain", "cells", "mean", 1).is_err());
    assert!(tourbillon(55, 64, "unspun", 0.0, "odd", "zeta", "cells", "mean", 1).is_err());
    assert!(tourbillon(55, 64, "unspun", 0.0, "odd", "plain", "dots", "mean", 1).is_err());
    assert!(tourbillon(55, 64, "unspun", 0.0, "odd", "plain", "cells", "blur", 1).is_err());
}

#[test]
fn the_three_modes_read_the_same_layers_apart() {
    let shares: Vec<String> = ["cells", "edges", "corners"]
        .iter()
        .map(|mode| {
            let field =
                tourbillon(55, 256, "golden", 0.0, "odd", "plain", mode, "mean", 1).unwrap();
            let read = parse(
                &tourbillon_stats(&field, 256, 55, "golden", 0.0, "odd", "plain", "mean", 1)
                    .unwrap(),
            )
            .unwrap();
            format!("{:.6}", read["mean"].as_f64().unwrap())
        })
        .collect();
    assert_eq!(shares.join(" "), "0.231722 0.111924 0.141211");
}

#[test]
fn the_quarter_turn_lattice_lists_its_angles_smallest_first() {
    let list = parse(&tourbillon_eyes(12).unwrap()).unwrap();
    let rows = list.as_array().unwrap();
    assert_eq!(rows.len(), 185);
    let first: Vec<String> = rows
        .iter()
        .take(8)
        .map(|row| format!("{:.6}", row[0].as_f64().unwrap()))
        .collect();
    assert_eq!(
        first.join(" "),
        "0.000000 7.500000 8.181818 9.000000 10.000000 11.250000 12.857143 15.000000"
    );
    assert_eq!(
        format!("{} {}", rows[1][1], rows[1][2]),
        format!("{} {}", 90, 12)
    );
    assert!(tourbillon_eyes(0).is_err());
    assert!(tourbillon_eyes(61).is_err());
}

#[test]
fn a_ninetieth_increment_folds_the_layers_into_angle_classes() {
    let stack = read(55, 64, "degrees", 18.0, "odd", "plain", "mean");
    assert_eq!(stack["period"], 5);
    assert_eq!(stack["classes"], 5);
    assert_eq!(stack["pairs"], 65);
    let fine = read(55, 64, "degrees", 7.5, "odd", "plain", "mean");
    assert_eq!(fine["period"], 12);
    assert_eq!(fine["classes"], 12);
    let loose = read(55, 64, "golden", 0.0, "odd", "plain", "mean");
    assert_eq!(loose["classes"], 28);
    assert_eq!(loose["pairs"], 0);
}
